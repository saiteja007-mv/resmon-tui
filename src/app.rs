use std::time::Instant;
use sysinfo::{System, Process, Pid};

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
    /// History buffer size
    pub history_size: usize,
    /// GPU information (if available)
    #[cfg(feature = "gpu-nvidia")]
    pub gpu_info: Option<GpuInfo>,
    #[cfg(feature = "gpu-nvidia")]
    nvml: Option<Nvml>,
    /// GPU usage history
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
            history_size,
            #[cfg(feature = "gpu-nvidia")]
            gpu_info,
            #[cfg(feature = "gpu-nvidia")]
            nvml,
            gpu_usage_history: Vec::new(),
        }
    }

    /// Update system information
    pub fn update(&mut self) {
        self.system.refresh_all();
        self.last_update = Instant::now();

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

    /// Get sorted processes by CPU usage
    pub fn get_sorted_processes(&self) -> Vec<(&Pid, &Process)> {
        let mut processes: Vec<_> = self.system.processes().iter().collect();
        processes.sort_by(|a, b| {
            b.1.cpu_usage().partial_cmp(&a.1.cpu_usage()).unwrap()
        });
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

    /// Quit application
    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}
