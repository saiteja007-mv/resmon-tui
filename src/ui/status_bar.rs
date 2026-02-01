use crate::app::App;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

/// Render status bar at the bottom of the screen
pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let process_count = app.system.processes().len();
    let refresh_rate_ms = app.refresh_rate_ms;

    // Context-aware key hints
    let hints = if app.show_help {
        "? or Esc: Close Help"
    } else if app.search_mode {
        "Type to search | Enter/Esc: Exit search"
    } else if app.show_details {
        "↑/↓: Navigate | Esc: Close Details | ?: Help"
    } else {
        "↑/↓: Navigate | Enter: Details | /: Search | c/m/p/t: Sort | ?: Help"
    };

    let right_content = format!(
        "Processes: {} | Refresh: {}ms",
        process_count, refresh_rate_ms
    );

    // Calculate spacing to push right content to the right
    let left_text = hints;
    let available_width = area.width as usize;
    let right_text_len = right_content.len();
    let left_text_len = left_text.len();

    let spacing = if left_text_len + right_text_len + 3 < available_width {
        available_width - left_text_len - right_text_len
    } else {
        3
    };

    let line = Line::from(vec![
        Span::styled(left_text, Style::default().fg(Color::Gray)),
        Span::raw(" ".repeat(spacing)),
        Span::styled(
            right_content,
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::DIM),
        ),
    ]);

    let paragraph = Paragraph::new(line);
    f.render_widget(paragraph, area);
}
