use crate::app::{ActionConfirmation, ProcessAction};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

/// Render confirmation dialog
pub fn render(f: &mut Frame, confirmation: &ActionConfirmation) {
    let area = centered_rect(50, 30, f.size());

    // Clear the area first
    f.render_widget(Clear, area);

    let (action_name, action_color, warning) = match confirmation.action {
        ProcessAction::Kill => (
            "Kill Process",
            Color::Red,
            "This will forcefully terminate the process!",
        ),
        ProcessAction::Suspend => (
            "Suspend Process",
            Color::Magenta,
            "This will pause the process execution.",
        ),
        ProcessAction::Resume => (
            "Resume Process",
            Color::Green,
            "This will continue the process execution.",
        ),
    };

    let text = vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            action_name,
            Style::default()
                .fg(action_color)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(vec![Span::styled(
            format!("Process: {}", confirmation.process_name),
            Style::default().fg(Color::White),
        )]),
        Line::from(vec![Span::styled(
            format!("PID: {}", confirmation.pid),
            Style::default().fg(Color::White),
        )]),
        Line::from(""),
        Line::from(vec![Span::styled(
            warning,
            Style::default().fg(Color::Yellow),
        )]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Press ", Style::default().fg(Color::Gray)),
            Span::styled("y", Style::default().fg(Color::Green)),
            Span::styled(" to confirm or ", Style::default().fg(Color::Gray)),
            Span::styled("n/Esc", Style::default().fg(Color::Red)),
            Span::styled(" to cancel", Style::default().fg(Color::Gray)),
        ]),
    ];

    let paragraph = Paragraph::new(text)
        .block(
            Block::default()
                .title(" Confirmation ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(action_color)),
        )
        .alignment(Alignment::Center);

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
