use std::time::{Duration, Instant};
use sysinfo::{System, Process, Pid};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SortOrder {
    Cpu,
    Memory,
    Pid,
    Runtime,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub enum ToastLevel {
    Info,
    Success,
    Warning,
    Error,
}

#[derive(Debug, Clone)]
pub struct Toast {
    pub message: String,
    pub level: ToastLevel,
    pub expires_at: Instant,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProcessAction {
    Kill,
    Suspend,
    Resume,
}

#[derive(Debug, Clone)]
pub struct ActionConfirmation {
    pub action: ProcessAction,
    pub pid: Pid,
    pub process_name: String,
}

#[cfg(feature = "gpu-nvidia")]
use nvml_wrapper::Nvml;

#[cfg(feature = "gpu-nvidia")]
pub struct GpuInfo {
    pub name: String,
    pub usage: f32,
    pub memory_used: u64,
    pub memory_total: u64,
    pub temperature: u32,
}

/// Application state
pub struct App {
    /// System information
    pub system: System,
    /// Selected process index in the list
    pub selected_process: Option<usize>,
    /// Scroll offset for process list
    pub scroll_offset: usize,
    /// Whether to show detailed view
    pub show_details: bool,
    /// Last update time
    pub last_update: Instant,
    /// CPU usage history for graphs (per core)
    pub cpu_history: Vec<Vec<f32>>,
    /// Overall CPU usage history
    pub overall_cpu_history: Vec<f32>,
    /// Memory usage history
    pub memory_history: Vec<f32>,
    /// Should the app quit
    pub should_quit: bool,
    /// Whether to show help overlay
    pub show_help: bool,
    /// Refresh rate in milliseconds
    pub refresh_rate_ms: u64,
    /// Current sort order for processes
    pub sort_order: SortOrder,
    /// Whether in search mode
    pub search_mode: bool,
    /// Current search query
    pub search_query: String,
    /// Filtered process indices
    pub filtered_processes: Option<Vec<usize>>,
    /// Current toast notification
    pub toast: Option<Toast>,
    /// Pending action confirmation
    pub pending_action: Option<ActionConfirmation>,
    /// History buffer size
    pub history_size: usize,
    /// GPU information (if available)
    #[cfg(feature = "gpu-nvidia")]
    pub gpu_info: Option<GpuInfo>,
    #[cfg(feature = "gpu-nvidia")]
    nvml: Option<Nvml>,
    /// GPU usage history
    #[cfg(feature = "gpu-nvidia")]
    pub gpu_usage_history: Vec<f32>,
}

impl App {
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all();

        let cpu_count = system.cpus().len();
        let history_size = 60; // Keep 60 data points

        #[cfg(feature = "gpu-nvidia")]
        let (nvml, gpu_info) = match Nvml::init() {
            Ok(nvml) => {
                match nvml.device_by_index(0) {
                    Ok(device) => {
                        let name = device.name().unwrap_or_else(|_| "Unknown GPU".to_string());
                        let info = GpuInfo {
                            name,
                            usage: 0.0,
                            memory_used: 0,
                            memory_total: device.memory_info().map(|m| m.total / 1024 / 1024).unwrap_or(0),
                            temperature: 0,
                        };
                        (Some(nvml), Some(info))
                    }
                    Err(_) => (None, None),
                }
            }
            Err(_) => (None, None),
        };

        Self {
            system,
            selected_process: None,
            scroll_offset: 0,
            show_details: false,
            last_update: Instant::now(),
            cpu_history: vec![Vec::new(); cpu_count],
            overall_cpu_history: Vec::new(),
            memory_history: Vec::new(),
            should_quit: false,
            show_help: false,
            refresh_rate_ms: 500,
            sort_order: SortOrder::Cpu,
            search_mode: false,
            search_query: String::new(),
            filtered_processes: None,
            toast: None,
            pending_action: None,
            history_size,
            #[cfg(feature = "gpu-nvidia")]
            gpu_info,
            #[cfg(feature = "gpu-nvidia")]
            nvml,
            #[cfg(feature = "gpu-nvidia")]
            gpu_usage_history: Vec::new(),
        }
    }

    /// Update system information
    pub fn update(&mut self) {
        self.system.refresh_all();
        self.last_update = Instant::now();

        // Update toast expiration
        self.update_toast();

        // Update CPU history
        for (i, cpu) in self.system.cpus().iter().enumerate() {
            if i < self.cpu_history.len() {
                self.cpu_history[i].push(cpu.cpu_usage());
                if self.cpu_history[i].len() > self.history_size {
                    self.cpu_history[i].remove(0);
                }
            }
        }

        // Update overall CPU usage
        let overall_cpu = self.system.global_cpu_usage();
        self.overall_cpu_history.push(overall_cpu);
        if self.overall_cpu_history.len() > self.history_size {
            self.overall_cpu_history.remove(0);
        }

        // Update memory history
        let mem_usage = (self.system.used_memory() as f32 / self.system.total_memory() as f32) * 100.0;
        self.memory_history.push(mem_usage);
        if self.memory_history.len() > self.history_size {
            self.memory_history.remove(0);
        }

        // Update GPU information
        #[cfg(feature = "gpu-nvidia")]
        if let Some(ref nvml) = self.nvml {
            if let Ok(device) = nvml.device_by_index(0) {
                if let Some(ref mut gpu_info) = self.gpu_info {
                    gpu_info.usage = device
                        .utilization_rates()
                        .map(|u| u.gpu as f32)
                        .unwrap_or(0.0);

                    if let Ok(mem_info) = device.memory_info() {
                        gpu_info.memory_used = mem_info.used / 1024 / 1024;
                    }

                    gpu_info.temperature = device
                        .temperature(nvml_wrapper::enum_wrappers::device::TemperatureSensor::Gpu)
                        .unwrap_or(0);

                    // Update GPU usage history
                    self.gpu_usage_history.push(gpu_info.usage);
                    if self.gpu_usage_history.len() > self.history_size {
                        self.gpu_usage_history.remove(0);
                    }
                }
            }
        }
    }

    /// Get sorted processes based on current sort order
    pub fn get_sorted_processes(&self) -> Vec<(&Pid, &Process)> {
        let mut processes: Vec<_> = self.system.processes().iter().collect();

        match self.sort_order {
            SortOrder::Cpu => {
                processes.sort_by(|a, b| {
                    b.1.cpu_usage().partial_cmp(&a.1.cpu_usage()).unwrap_or(std::cmp::Ordering::Equal)
                });
            }
            SortOrder::Memory => {
                processes.sort_by(|a, b| {
                    b.1.memory().cmp(&a.1.memory())
                });
            }
            SortOrder::Pid => {
                processes.sort_by(|a, b| {
                    a.0.as_u32().cmp(&b.0.as_u32())
                });
            }
            SortOrder::Runtime => {
                processes.sort_by(|a, b| {
                    b.1.run_time().cmp(&a.1.run_time())
                });
            }
        }

        processes
    }

    /// Navigate process list down
    pub fn next_process(&mut self) {
        let process_count = self.system.processes().len();
        if process_count == 0 {
            return;
        }

        if let Some(selected) = self.selected_process {
            self.selected_process = Some((selected + 1).min(process_count - 1));
        } else {
            self.selected_process = Some(0);
        }
    }

    /// Navigate process list up
    pub fn previous_process(&mut self) {
        if let Some(selected) = self.selected_process {
            if selected > 0 {
                self.selected_process = Some(selected - 1);
            }
        } else {
            self.selected_process = Some(0);
        }
    }

    /// Toggle details view
    pub fn toggle_details(&mut self) {
        self.show_details = !self.show_details;
    }

    /// Get selected process
    pub fn get_selected_process(&self) -> Option<(&Pid, &Process)> {
        if let Some(idx) = self.selected_process {
            self.get_sorted_processes().get(idx).copied()
        } else {
            None
        }
    }

    /// Toggle help overlay
    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
    }

    /// Increase refresh rate (faster updates)
    pub fn increase_refresh_rate(&mut self) {
        self.refresh_rate_ms = match self.refresh_rate_ms {
            5000 => 2000,
            2000 => 1000,
            1000 => 500,
            500 => 250,
            _ => 250,
        };
    }

    /// Decrease refresh rate (slower updates)
    pub fn decrease_refresh_rate(&mut self) {
        self.refresh_rate_ms = match self.refresh_rate_ms {
            250 => 500,
            500 => 1000,
            1000 => 2000,
            2000 => 5000,
            _ => 5000,
        };
    }

    /// Get refresh duration
    pub fn get_refresh_duration(&self) -> Duration {
        Duration::from_millis(self.refresh_rate_ms)
    }

    /// Set sort order
    pub fn set_sort_order(&mut self, order: SortOrder) {
        self.sort_order = order;
    }

    /// Start search mode
    pub fn start_search(&mut self) {
        self.search_mode = true;
        self.search_query.clear();
        self.filtered_processes = None;
    }

    /// Exit search mode
    pub fn exit_search(&mut self) {
        self.search_mode = false;
        self.search_query.clear();
        self.filtered_processes = None;
    }

    /// Add character to search query
    pub fn search_input(&mut self, c: char) {
        self.search_query.push(c);
        self.update_filter();
    }

    /// Remove last character from search query
    pub fn search_backspace(&mut self) {
        self.search_query.pop();
        self.update_filter();
    }

    /// Update process filter based on search query
    pub fn update_filter(&mut self) {
        if self.search_query.is_empty() {
            self.filtered_processes = None;
            return;
        }

        let query = self.search_query.to_lowercase();
        let all_processes = self.get_sorted_processes();

        let mut filtered_indices = Vec::new();
        for (idx, (_pid, process)) in all_processes.iter().enumerate() {
            let name = process.name().to_string_lossy().to_lowercase();
            let pid_str = _pid.to_string();

            if name.contains(&query) || pid_str.contains(&query) {
                filtered_indices.push(idx);
            }
        }

        self.filtered_processes = Some(filtered_indices);
    }

    /// Get display processes (filtered or all)
    pub fn get_display_processes(&self) -> Vec<(&Pid, &Process)> {
        let all_processes = self.get_sorted_processes();

        if let Some(ref filtered_indices) = self.filtered_processes {
            filtered_indices.iter()
                .filter_map(|&idx| all_processes.get(idx).copied())
                .collect()
        } else {
            all_processes
        }
    }

    /// Show toast notification
    pub fn show_toast(&mut self, message: String, level: ToastLevel) {
        let expires_at = Instant::now() + Duration::from_secs(3);
        self.toast = Some(Toast {
            message,
            level,
            expires_at,
        });
    }

    /// Update toast expiration
    pub fn update_toast(&mut self) {
        if let Some(ref toast) = self.toast {
            if Instant::now() >= toast.expires_at {
                self.toast = None;
            }
        }
    }

    /// Request process action with confirmation
    pub fn request_action(&mut self, action: ProcessAction) {
        if let Some((_pid, process)) = self.get_selected_process() {
            let pid = *_pid;
            let process_name = process.name().to_string_lossy().to_string();

            self.pending_action = Some(ActionConfirmation {
                action,
                pid,
                process_name,
            });
        }
    }

    /// Cancel pending action
    pub fn cancel_action(&mut self) {
        self.pending_action = None;
    }

    /// Execute pending action
    pub fn execute_action(&mut self) {
        if let Some(ref confirmation) = self.pending_action.clone() {
            let result = match confirmation.action {
                ProcessAction::Kill => self.kill_process(confirmation.pid),
                ProcessAction::Suspend => self.suspend_process(confirmation.pid),
                ProcessAction::Resume => self.resume_process(confirmation.pid),
            };

            match result {
                Ok(msg) => {
                    self.show_toast(msg, ToastLevel::Success);
                }
                Err(err) => {
                    self.show_toast(err, ToastLevel::Error);
                }
            }

            self.pending_action = None;
        }
    }

    /// Kill a process
    fn kill_process(&mut self, pid: Pid) -> Result<String, String> {
        if let Some(process) = self.system.process(pid) {
            if process.kill() {
                Ok(format!("Process {} killed successfully", pid))
            } else {
                Err(format!("Failed to kill process {}", pid))
            }
        } else {
            Err(format!("Process {} not found", pid))
        }
    }

    /// Suspend a process (Unix only)
    #[cfg(target_family = "unix")]
    fn suspend_process(&mut self, pid: Pid) -> Result<String, String> {
        use sysinfo::Signal;

        if let Some(process) = self.system.process(pid) {
            if process.kill_with(Signal::Stop).is_some() {
                Ok(format!("Process {} suspended", pid))
            } else {
                Err(format!("Failed to suspend process {}", pid))
            }
        } else {
            Err(format!("Process {} not found", pid))
        }
    }

    /// Suspend a process (Windows - not supported)
    #[cfg(not(target_family = "unix"))]
    fn suspend_process(&mut self, _pid: Pid) -> Result<String, String> {
        Err("Process suspend is not supported on Windows".to_string())
    }

    /// Resume a process (Unix only)
    #[cfg(target_family = "unix")]
    fn resume_process(&mut self, pid: Pid) -> Result<String, String> {
        use sysinfo::Signal;

        if let Some(process) = self.system.process(pid) {
            if process.kill_with(Signal::Continue).is_some() {
                Ok(format!("Process {} resumed", pid))
            } else {
                Err(format!("Failed to resume process {}", pid))
            }
        } else {
            Err(format!("Process {} not found", pid))
        }
    }

    /// Resume a process (Windows - not supported)
    #[cfg(not(target_family = "unix"))]
    fn resume_process(&mut self, _pid: Pid) -> Result<String, String> {
        Err("Process resume is not supported on Windows".to_string())
    }

    /// Quit application
    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}
