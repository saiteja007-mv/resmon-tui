pub mod cpu;
pub mod gpu;
pub mod processes;
pub mod details;
pub mod help;
pub mod status_bar;
pub mod toast;
pub mod confirmation;

use crate::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

/// Render the main UI
pub fn render(f: &mut Frame, app: &App) {
    let area = f.size();

    // Split into main content area and status bar
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(3), Constraint::Length(1)])
        .split(area);

    let main_area = chunks[0];
    let status_area = chunks[1];

    if app.show_details {
        // Show split view: left side = overview, right side = details
        let content_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(main_area);

        render_overview(f, app, content_chunks[0]);
        details::render(f, app, content_chunks[1]);
    } else {
        // Show full overview
        render_overview(f, app, main_area);
    }

    // Render status bar
    status_bar::render(f, app, status_area);

    // Render toast notification if active
    if let Some(ref toast) = app.toast {
        toast::render(f, toast);
    }

    // Render confirmation dialog if active
    if let Some(ref pending) = app.pending_action {
        confirmation::render(f, pending);
    }

    // Render help overlay on top if active
    if app.show_help {
        help::render(f);
    }
}

/// Render the overview (CPU, GPU, processes)
fn render_overview(f: &mut Frame, app: &App, area: Rect) {
    // Split into left (CPU overview + GPU) and right (per-core CPUs + processes)
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    // Left side: CPU overview and GPU
    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(55), Constraint::Percentage(45)])
        .split(main_chunks[0]);

    cpu::render_overview(f, app, left_chunks[0]);
    gpu::render(f, app, left_chunks[1]);

    // Right side: Per-core CPUs and Process list
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(65), Constraint::Percentage(35)])
        .split(main_chunks[1]);

    cpu::render_cores(f, app, right_chunks[0]);
    processes::render(f, app, right_chunks[1]);
}
