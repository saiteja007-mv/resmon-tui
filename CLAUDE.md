# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

ResMan TUI (`rtui`) is a cross-platform terminal UI resource monitor written in Rust. It displays real-time CPU, memory, GPU (NVIDIA only), and process information with historical graphs and interactive process details.

## Build and Run Commands

### Development
```bash
# Run in development mode (no GPU support)
cargo run

# Run with NVIDIA GPU support
cargo run --features gpu-nvidia

# Build release version
cargo build --release

# Build with GPU support
cargo build --release --features gpu-nvidia
```

### Testing and Quality
```bash
# Run tests
cargo test

# Check code without building
cargo check

# Format code
cargo fmt

# Run linter
cargo clippy
```

### Installation
```bash
# Install globally without GPU support
cargo install --path .

# Install with NVIDIA GPU support
cargo install --path . --features gpu-nvidia

# Uninstall
cargo uninstall resmon-tui
```

The binary name is `rtui` (not `resmon`), as defined in Cargo.toml.

## Architecture

### Module Structure

**src/main.rs**: Entry point that sets up the terminal, creates the event loop, and handles keyboard input. The main loop refreshes every 500ms and renders the UI using ratatui.

**src/app.rs**: Contains the `App` struct which is the central state manager:
- Maintains `System` from sysinfo for reading system metrics
- Stores historical data for CPU (per-core and overall), memory, and GPU usage (60 data points)
- Manages process selection and detail view state
- Handles GPU initialization via NVML when `gpu-nvidia` feature is enabled
- Provides methods for updating metrics and navigating the process list

**src/ui/mod.rs**: Orchestrates UI rendering:
- `render()` decides between full overview or split view (when showing process details)
- `render_overview()` creates the main layout: left side (CPU + GPU), right side (process list)

**src/ui/cpu.rs**: Renders CPU metrics including per-core sparkline graphs and overall CPU history chart

**src/ui/gpu.rs**: Conditionally compiled with `gpu-nvidia` feature. Displays GPU usage, memory, temperature, and usage history graph

**src/ui/processes.rs**: Renders the scrollable process list sorted by CPU usage

**src/ui/details.rs**: Renders detailed view for a selected process (CPU/memory gauges, disk I/O, virtual memory)

### Key Design Patterns

- **Feature flags**: GPU support is optional via `gpu-nvidia` feature to avoid requiring NVIDIA drivers/CUDA
- **Data history**: Fixed-size circular buffers (60 elements) for historical graphs, implemented by pushing new values and removing old ones
- **Process sorting**: Processes are sorted by CPU usage on each render via `get_sorted_processes()`
- **Split view**: When viewing process details, UI switches to horizontal split (overview | details)

### Keyboard Controls

Implemented in `src/main.rs:68-93`:
- `q/Q`: Quit (only when not in detail view)
- `↓/j`: Navigate down in process list
- `↑/k`: Navigate up in process list
- `Enter`: Show process details (split view)
- `Esc`: Close detail view

## Dependencies

- **ratatui**: TUI framework for rendering widgets
- **crossterm**: Cross-platform terminal manipulation (raw mode, events)
- **sysinfo**: Cross-platform system metrics (CPU, memory, processes)
- **tokio**: Async runtime (currently included but not actively used)
- **nvml-wrapper**: NVIDIA GPU metrics (optional, feature-gated)

## Platform Support

Supports Windows, Linux, and macOS. GPU monitoring requires NVIDIA GPU, drivers, and CUDA Toolkit.
