use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

/// Render help overlay
pub fn render(f: &mut Frame) {
    let area = centered_rect(60, 80, f.size());

    // Clear the area first
    f.render_widget(Clear, area);

    // Create the help content
    let help_text = vec![
        Line::from(vec![Span::styled(
            "ResMan TUI - Keyboard Shortcuts",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "General",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
        )]),
        Line::from(vec![
            Span::styled("  ?          ", Style::default().fg(Color::Green)),
            Span::raw("Toggle this help screen"),
        ]),
        Line::from(vec![
            Span::styled("  q / Q      ", Style::default().fg(Color::Green)),
            Span::raw("Quit application (normal mode only)"),
        ]),
        Line::from(vec![
            Span::styled("  Esc        ", Style::default().fg(Color::Green)),
            Span::raw("Close overlay/exit mode"),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Navigation",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
        )]),
        Line::from(vec![
            Span::styled("  ↑ / k      ", Style::default().fg(Color::Green)),
            Span::raw("Navigate up in process list"),
        ]),
        Line::from(vec![
            Span::styled("  ↓ / j      ", Style::default().fg(Color::Green)),
            Span::raw("Navigate down in process list"),
        ]),
        Line::from(vec![
            Span::styled("  Enter      ", Style::default().fg(Color::Green)),
            Span::raw("Show process details"),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "View Options",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
        )]),
        Line::from(vec![
            Span::styled("  + / =      ", Style::default().fg(Color::Green)),
            Span::raw("Increase refresh rate (faster)"),
        ]),
        Line::from(vec![
            Span::styled("  -          ", Style::default().fg(Color::Green)),
            Span::raw("Decrease refresh rate (slower)"),
        ]),
        Line::from(vec![
            Span::styled("  t          ", Style::default().fg(Color::Green)),
            Span::raw("Toggle process tree view / Sort by runtime"),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Search & Sort",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
        )]),
        Line::from(vec![
            Span::styled("  /          ", Style::default().fg(Color::Green)),
            Span::raw("Enter search mode"),
        ]),
        Line::from(vec![
            Span::styled("  c          ", Style::default().fg(Color::Green)),
            Span::raw("Sort by CPU usage"),
        ]),
        Line::from(vec![
            Span::styled("  m          ", Style::default().fg(Color::Green)),
            Span::raw("Sort by Memory usage"),
        ]),
        Line::from(vec![
            Span::styled("  p          ", Style::default().fg(Color::Green)),
            Span::raw("Sort by PID"),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Process Actions",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
        )]),
        Line::from(vec![
            Span::styled("  k          ", Style::default().fg(Color::Red)),
            Span::raw("Kill selected process"),
        ]),
        Line::from(vec![
            Span::styled("  s          ", Style::default().fg(Color::Magenta)),
            Span::raw("Suspend selected process (Unix only)"),
        ]),
        Line::from(vec![
            Span::styled("  r          ", Style::default().fg(Color::Magenta)),
            Span::raw("Resume selected process (Unix only)"),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Press ", Style::default().fg(Color::Gray)),
            Span::styled("?", Style::default().fg(Color::Green)),
            Span::styled(" or ", Style::default().fg(Color::Gray)),
            Span::styled("Esc", Style::default().fg(Color::Green)),
            Span::styled(" to close this help", Style::default().fg(Color::Gray)),
        ]),
    ];

    let paragraph = Paragraph::new(help_text)
        .block(
            Block::default()
                .title(" Help ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .alignment(Alignment::Left);

    f.render_widget(paragraph, area);
}

/// Helper function to create centered rect
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
