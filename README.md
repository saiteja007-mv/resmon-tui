# ResMon TUI - Resource Monitor Terminal UI

<p align="center">
  <img src="Resmon-tui-logo.png" alt="ResMan TUI Logo" width="400"/>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="License"/>
  <img src="https://img.shields.io/badge/platform-Windows%20%7C%20Linux%20%7C%20macOS-lightgrey" alt="Platform"/>
  <img src="https://img.shields.io/badge/rust-1.70%2B-orange.svg" alt="Rust"/>
</p>

<p align="center">
  <strong>A powerful, feature-rich terminal UI for real-time system monitoring</strong>
</p>

---

## ✨ Features

### 📊 CPU Monitoring

- **Stylized ASCII Art Logo** - Beautiful RESMON branding with gradient colors
- **Per-core CPU Usage** - Individual gauges with sparkline graphs for each core
- **CPU Frequency Display** - Real-time frequency monitoring per core (GHz)
- **Overall CPU Usage** - Gauge showing total system CPU usage
- **Historical Data** - Line charts showing CPU usage over time
- **Smart Grid Layout** - 4-column responsive grid for optimal core visualization

### 💾 Memory Monitoring

- Real-time memory usage tracking
- Historical memory usage graphs
- Memory percentage with visual indicators

### 🎮 GPU Monitoring (Optional - NVIDIA only)

- GPU usage percentage with gauge
- Memory usage (used/total)
- Temperature monitoring with color-coded warnings
- GPU usage history graph
- Automatic feature detection

### ⚙️ Enhanced Process Management

- **Live Process List** - Real-time updates with color-coded CPU usage
- **Multiple Sort Options:**
  - `c` - Sort by CPU usage (default)
  - `m` - Sort by Memory usage
  - `p` - Sort by Process ID
  - `t` - Sort by Runtime
  - Visual sort indicator (▼) in column headers
- **Search & Filter** - Press `/` to filter processes by name or PID
- **Process Actions:**
  - `k` - Kill selected process (with confirmation)
  - `s` - Suspend process (Unix/Linux only)
  - `r` - Resume process (Unix/Linux only)
  - Confirmation dialogs with color-coded warnings
  - Success/error toast notifications
- **Detailed Process View:**
  - CPU and memory usage gauges
  - Process information (PID, parent, status, executable)
  - Disk I/O statistics
  - Virtual memory usage
  - Working directory

### 🎨 Beautiful Interface

- **Enhanced Color Thresholds:**
  - 🟢 Green: 0-60% (optimal)
  - 🟡 Yellow: 60-85% (warning)
  - 🔴 Red: 85-100% (critical)
- **Context-Aware Status Bar** - Dynamic keyboard hints based on current mode
- **Toast Notifications** - Auto-dismissing feedback messages (3 seconds)
- **Help System** - Press `?` for comprehensive keyboard shortcut reference
- **Optimized Layout** - Reorganized for maximum information density
- **Smooth Updates** - Configurable refresh rate (250ms - 5000ms)

### ⌨️ Comprehensive Keyboard Controls

#### Navigation
- `↑` / `k` - Navigate up in process list
- `↓` / `j` - Navigate down in process list
- `Enter` - View detailed info for selected process
- `Esc` - Close overlay/detail view/exit mode

#### View Controls
- `?` - Toggle help overlay
- `/` - Enter search mode
- `+` / `=` - Increase refresh rate (faster updates)
- `-` - Decrease refresh rate (slower updates)

#### Sorting
- `c` - Sort by CPU usage
- `m` - Sort by Memory usage
- `p` - Sort by Process ID
- `t` - Sort by Runtime

#### Process Actions
- `k` - Kill selected process
- `s` - Suspend process (Unix/Linux)
- `r` - Resume process (Unix/Linux)
- `y` / `Enter` - Confirm action
- `n` / `Esc` - Cancel action

#### General
- `q` / `Q` - Quit application (normal mode only)

---

## 🎨 Screenshots & Demo

### 🎬 Live Demo

> **Note:** The live GPU stats shown in this demo are from running Ollama with the `gpt-oss:20b` model locally.

<video
  src="https://github.com/user-attachments/assets/6b1b9ad9-a2d0-4d8e-894e-d37034e09499"
  controls
  style="max-width: 100%; height: auto;">
</video>

*Resource Monitor in action on Windows - showing real-time CPU, Memory, GPU monitoring and process details*

### 📸 Screenshots

#### Main View - Windows

![Resource Monitor Main View](Outputs/Resource%20Monitor%20output.png)

*Main dashboard with RESMON ASCII logo, CPU per-core usage with frequency display, overall CPU history, GPU monitoring, and live process list with search and sort capabilities*

#### Process Details View

![Resource Monitor with Process Details](Outputs/Resource%20Monitor%20with%20each%20process.png)

*Split view with detailed process information including CPU/Memory gauges, executable path, and I/O statistics*

#### Linux Support

![Linux Output](Outputs/Linux%20Output.png)

*Running on Linux with full cross-platform support and process control features*

---

## 🚀 Quick Installation

### ⚡ One-Click Installer (Recommended)

**The easiest way to install - handles everything automatically!**

#### Windows

1. Download or clone the repository
2. **Double-click `install.bat`**
3. Follow the prompts
4. Done! Run `rtui` from any terminal

```powershell
# Or from PowerShell/CMD:
git clone https://github.com/saiteja007-mv/resmon-tui.git
cd resmon-tui
install.bat
```

#### Linux / macOS

```bash
git clone https://github.com/saiteja007-mv/resmon-tui.git
cd resmon-tui
chmod +x install.sh
./install.sh
```

**The installer automatically:**

- ✅ Checks if Rust is installed (installs if needed)
- ✅ Asks if you want GPU support (NVIDIA only)
- ✅ Builds the optimized release version
- ✅ Installs globally so you can run `rtui` from anywhere
- ✅ Verifies the installation

---

## 📖 Usage

After installation, simply run:

```bash
rtui
```

### Getting Started

1. **Explore the Interface** - Let it run for a few seconds to see graphs populate
2. **Navigate Processes** - Use `↑/↓` or `j/k` to select processes
3. **View Details** - Press `Enter` on any process for detailed stats
4. **Search** - Press `/` to filter processes by name or PID
5. **Sort** - Use `c`, `m`, `p`, or `t` to sort by different criteria
6. **Adjust Speed** - Press `+`/`-` to change refresh rate
7. **Get Help** - Press `?` anytime to see all keyboard shortcuts

### Keyboard Reference

| Key          | Action                                    | Context         |
| ------------ | ----------------------------------------- | --------------- |
| `?`          | Toggle help screen                        | Always          |
| `/`          | Enter search mode                         | Normal mode     |
| `c`          | Sort by CPU usage                         | Normal mode     |
| `m`          | Sort by Memory usage                      | Normal mode     |
| `p`          | Sort by Process ID                        | Normal mode     |
| `t`          | Sort by Runtime                           | Normal mode     |
| `+` / `=`    | Increase refresh rate (faster)            | Normal mode     |
| `-`          | Decrease refresh rate (slower)            | Normal mode     |
| `k`          | Kill selected process                     | Process selected|
| `s`          | Suspend selected process                  | Unix/Linux only |
| `r`          | Resume selected process                   | Unix/Linux only |
| `↑` / `k`    | Navigate up                               | Process list    |
| `↓` / `j`    | Navigate down                             | Process list    |
| `Enter`      | View process details / Confirm action     | Context-aware   |
| `Esc`        | Close overlay / Exit mode / Cancel action | Context-aware   |
| `q` / `Q`    | Quit application                          | Normal mode     |

### Tips & Tricks

- 💡 **Vim Users:** Use `j`/`k` for navigation just like in vim
- 🔍 **Quick Search:** Type `/` followed by process name or PID
- 📊 **Performance Mode:** Use `+` to set 250ms refresh for real-time monitoring
- 🎯 **Battery Saver:** Use `-` to set 5000ms refresh when idle
- 🎨 **Color Guide:** Green (healthy), Yellow (elevated), Red (critical)
- ⚡ **Process Control:** Select a process and press `k` to kill it (requires confirmation)

---

## 🎮 GPU Support (Optional)

GPU monitoring is **optional** and currently supports **NVIDIA GPUs only**.

### Requirements

- NVIDIA GPU (any model)
- NVIDIA drivers installed
- CUDA Toolkit ([Download here](https://developer.nvidia.com/cuda-downloads))

### Installing with GPU Support

The installer will ask if you want GPU support. Choose `y` if:

- You have an NVIDIA GPU
- You've installed CUDA Toolkit
- You want to monitor GPU usage, memory, and temperature

Choose `n` if:

- You don't have an NVIDIA GPU
- You don't need GPU monitoring
- You want a lighter build

**Note:** The app works perfectly without GPU support!

---

## 🛠️ Manual Installation

If you prefer to install manually or already have Rust:

### Prerequisites

- Rust 1.70 or later ([Install Rust](https://rustup.rs/))

### Install (Basic - without GPU monitoring)

```bash
git clone https://github.com/saiteja007-mv/resmon-tui.git
cd resmon-tui
cargo install --path .
```

### Install with NVIDIA GPU Support

```bash
cargo install --path . --features gpu-nvidia
```

### Build from Source

```bash
# Clone repository
git clone https://github.com/saiteja007-mv/resmon-tui.git
cd resmon-tui

# Build without GPU
cargo build --release

# Build with NVIDIA GPU support
cargo build --release --features gpu-nvidia

# Run
./target/release/rtui
```

The binary will be installed to `~/.cargo/bin/rtui` (or `%USERPROFILE%\.cargo\bin\rtui.exe` on Windows).

---

## 🔧 Troubleshooting

### "Command not found: rtui"

**Windows:**

1. Close and reopen your terminal
2. If still not working, add to PATH manually:
   - Press `Win + X` → System → Advanced system settings
   - Click "Environment Variables"
   - Under "User variables", select "Path" and click "Edit"
   - Click "New" and add: `%USERPROFILE%\.cargo\bin`
   - Click OK on all windows
   - Restart terminal

**Linux/macOS:**

```bash
# Add to your shell config
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc

# For macOS with zsh:
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

### Build fails with GPU support

Make sure you have:

- NVIDIA drivers installed (`nvidia-smi` command works)
- CUDA Toolkit installed

The installer will automatically fall back to non-GPU build if it fails.

**Linux users:** Install development libraries:

```bash
# Ubuntu/Debian
sudo apt-get install libnvidia-ml-dev

# Fedora/RHEL
sudo dnf install nvidia-driver-devel
```

### "Rust is not installed"

The installer will automatically download and install Rust for you.
Just follow the prompts and restart the installer when done.

---

## 🔧 Development

For developers who want to contribute:

```bash
# Run in development mode
cargo run

# Run with GPU support
cargo run --features gpu-nvidia

# Run tests
cargo test

# Check code
cargo check

# Format code
cargo fmt

# Lint code
cargo clippy
```

---

## 🎯 Roadmap

### Implemented ✅

- [X] CPU monitoring (per-core with sparkline graphs)
- [X] CPU frequency display per core
- [X] Overall CPU history (line chart)
- [X] Memory monitoring
- [X] Process list with multiple sort options
- [X] Process detail view
- [X] GPU monitoring (NVIDIA support via feature flag)
- [X] Search and filter processes
- [X] Configurable refresh rate
- [X] Help system with keyboard shortcuts
- [X] Process management (kill/suspend/resume)
- [X] Toast notifications
- [X] Context-aware status bar
- [X] Enhanced color thresholds
- [X] RESMON ASCII logo branding

### Planned 🚧

- [ ] GPU monitoring for AMD/Intel
- [ ] Network I/O monitoring
- [ ] Disk I/O monitoring with graphs
- [ ] Process tree view
- [ ] Custom color themes
- [ ] Configuration file support
- [ ] Export data to CSV/JSON
- [ ] Mouse support
- [ ] Responsive layouts for different terminal sizes
- [ ] Process history tracking

---

## 🌐 Cross-Platform Support

ResMan TUI works on:

- ✅ **Windows** 10/11
- ✅ **Linux** (Ubuntu, Debian, Fedora, Arch, etc.)
- ✅ **macOS** 10.15+ (Intel and Apple Silicon)

**Platform-Specific Features:**

- **Windows:** Full CPU/Memory/GPU monitoring, process kill
- **Linux/Unix:** All features + process suspend/resume
- **macOS:** Full CPU/Memory monitoring (GPU if NVIDIA eGPU)

---

## 📦 Uninstall

To remove ResMan TUI:

```bash
cargo uninstall resmon-tui
```

Or manually delete:

- Windows: `%USERPROFILE%\.cargo\bin\rtui.exe`
- Linux/macOS: `~/.cargo/bin/rtui`

---

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

### Development Guidelines

- Follow existing code style
- Add tests for new features
- Update documentation
- Ensure `cargo clippy` passes with no warnings
- Format code with `cargo fmt`

---

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## 🙏 Acknowledgments

- Built with [ratatui](https://github.com/ratatui-org/ratatui) - Modern terminal UI framework
- [sysinfo](https://github.com/GuillaumeGomez/sysinfo) - Cross-platform system information
- [crossterm](https://github.com/crossterm-rs/crossterm) - Cross-platform terminal manipulation
- [nvml-wrapper](https://github.com/Cldfire/nvml-wrapper) - NVIDIA GPU monitoring
- ASCII logo inspired by [oh-my-logo](https://github.com/shinshin86/oh-my-logo)

---

## 📞 Support

If you encounter any issues or have questions:

- 🐛 [Report a Bug](https://github.com/saiteja007-mv/resmon-tui/issues)
- 💡 [Request a Feature](https://github.com/saiteja007-mv/resmon-tui/issues)
- 💬 [Start a Discussion](https://github.com/saiteja007-mv/resmon-tui/discussions)

---

## 🚀 Quick Reference

**Installation:**

```bash
# Windows: Double-click install.bat
# Linux/macOS: ./install.sh
```

**Running:**

```bash
rtui
```

**Quick Help:**

```bash
# Inside the app, press:
?  - Show all keyboard shortcuts
/  - Search processes
c  - Sort by CPU
+  - Faster updates
-  - Slower updates
q  - Quit
```

**Uninstall:**

```bash
cargo uninstall resmon-tui
```

---

<p align="center">
  Made with ❤️ using Rust
</p>

<p align="center">
  <strong>Star ⭐ this repo if you find it useful!</strong>
</p>
