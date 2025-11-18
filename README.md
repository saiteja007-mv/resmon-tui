# ResMan TUI - Resource Monitor Terminal UI

![License](https://img.shields.io/badge/license-MIT-blue.svg)

![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20Linux%20%7C%20macOS-lightgrey)

---

## ✨ Features

- **📊 CPU Monitoring**

  - Per-core CPU usage with real-time sparkline graphs
  - Overall CPU usage gauge
  - Historical data visualization with line charts
- **💾 Memory Monitoring**

  - Real-time memory usage tracking
  - Historical memory usage graphs
- **🎮 GPU Monitoring** (Optional - NVIDIA only)

  - GPU usage percentage with gauge
  - Memory usage (used/total)
  - Temperature monitoring with color-coded warnings
  - GPU usage history graph
- **⚙️ Process Management**

  - Live process list sorted by CPU usage
  - Process details view with:
    - CPU and memory usage gauges
    - Process information (PID, parent, status, executable)
    - Disk I/O statistics
    - Virtual memory usage
- **🎨 Beautiful Interface**

  - Color-coded usage indicators (🟢 green < 50%, 🟡 yellow < 75%, 🔴 red ≥ 75%)
  - Responsive layout that adapts to terminal size
  - Smooth real-time updates (500ms refresh)
- **⌨️ Keyboard Controls**

  - `↑/k` - Navigate up in process list
  - `↓/j` - Navigate down in process list
  - `Enter` - View detailed info for selected process
  - `Esc` - Close detail view
  - `q` - Quit application

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

*Main dashboard showing CPU per-core usage with sparklines, overall CPU history, memory usage, GPU monitoring, and live process list*

#### Process Details View

![Resource Monitor with Process Details](Outputs/Resource%20Monitor%20with%20each%20process.png)

*Split view with detailed process information including CPU/Memory gauges, executable path, and I/O statistics*

#### Linux Support

![Linux Output](Outputs/Linux%20Output.png)

*Running on Linux with full cross-platform support*

---

## 🚀 Quick Installation

### ⚡ One-Click Installer (Recommended)

**The easiest way to install - handles everything automatically!**

#### Windows

1. Download or clone the repository
2. **Double-click `install.bat`**
3. Follow the prompts
4. Done! Run `resmon` from any terminal

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
- ✅ Installs globally so you can run `resmon` from anywhere
- ✅ Verifies the installation

---

## 📖 Usage

After installation, simply run:

```bash
resmon
```

### Keyboard Controls

| Key             | Action                            |
| --------------- | --------------------------------- |
| `↑` or `k` | Move selection up                 |
| `↓` or `j` | Move selection down               |
| `Enter`       | View process details (split view) |
| `Esc`         | Close detail view                 |
| `q`           | Quit (when not in detail view)    |

### Tips

- Let it run for a few seconds to see the graphs populate with data
- Press `Enter` on any process to see detailed statistics
- Use `j`/`k` for vim-style navigation

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
./target/release/resmon
```

The binary will be installed to `~/.cargo/bin/resmon` (or `%USERPROFILE%\.cargo\bin\resmon.exe` on Windows).

---

## 🔧 Troubleshooting

### "Command not found: resmon"

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

- [X] CPU monitoring (per-core with sparkline graphs)
- [X] Overall CPU history (line chart)
- [X] Memory monitoring
- [X] Process list with sorting
- [X] Process detail view
- [X] GPU monitoring (NVIDIA support via feature flag)
- [ ] GPU monitoring for AMD/Intel
- [ ] Network I/O monitoring
- [ ] Disk I/O monitoring with graphs
- [ ] Custom color themes
- [ ] Configuration file support
- [ ] Process filtering and search
- [ ] Export data to CSV/JSON
- [ ] Mouse support

---

## 🌐 Cross-Platform Support

ResMan TUI works on:

- ✅ **Windows** 10/11
- ✅ **Linux** (Ubuntu, Debian, Fedora, Arch, etc.)
- ✅ **macOS** 10.15+ (Intel and Apple Silicon)

---

## 📦 Uninstall

To remove ResMan TUI:

```bash
cargo uninstall resmon-tui
```

Or manually delete:

- Windows: `%USERPROFILE%\.cargo\bin\resmon.exe`
- Linux/macOS: `~/.cargo/bin/resmon`

---

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

---

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## 🙏 Acknowledgments

- Built with [ratatui](https://github.com/ratatui-org/ratatui) - Modern terminal UI framework
- [sysinfo](https://github.com/GuillaumeGomez/sysinfo) - Cross-platform system information
- [crossterm](https://github.com/crossterm-rs/crossterm) - Cross-platform terminal manipulation
- [nvml-wrapper](https://github.com/Cldfire/nvml-wrapper) - NVIDIA GPU monitoring

---

## 📞 Support

If you encounter any issues or have questions:

- 🐛 [Report a Bug](https://github.com/yourusername/resmon-tui/issues)
- 💡 [Request a Feature](https://github.com/yourusername/resmon-tui/issues)
- 💬 [Start a Discussion](https://github.com/yourusername/resmon-tui/discussions)

---

## 🚀 Quick Reference

**Installation:**

```bash
# Windows: Double-click install.bat
# Linux/macOS: ./install.sh
```

**Running:**

```bash
resmon
```

**Uninstall:**

```bash
cargo uninstall resmon-tui
```

---

Made with ❤️ using Rust
