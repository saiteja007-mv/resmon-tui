pub mod cpu;
pub mod gpu;
pub mod processes;
pub mod details;

use crate::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

/// Render the main UI
pub fn render(f: &mut Frame, app: &App) {
    let area = f.size();
    if app.show_details {
        // Show split view: left side = overview, right side = details
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        render_overview(f, app, chunks[0]);
        details::render(f, app, chunks[1]);
    } else {
        // Show full overview
        render_overview(f, app, area);
    }
}

/// Render the overview (CPU, GPU, processes)
fn render_overview(f: &mut Frame, app: &App, area: Rect) {
    // Split into left (CPU/GPU) and right (processes)
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    // Left side: CPU and GPU
    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(55), Constraint::Percentage(45)])
        .split(main_chunks[0]);

    cpu::render(f, app, left_chunks[0]);
    gpu::render(f, app, left_chunks[1]);

    // Right side: Process list
    processes::render(f, app, main_chunks[1]);
}
