use crate::app::{Toast, ToastLevel};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

/// Render toast notification
pub fn render(f: &mut Frame, toast: &Toast) {
    let area = toast_rect(f.size());

    // Clear the area first
    f.render_widget(Clear, area);

    let (border_color, text_color) = match toast.level {
        ToastLevel::Info => (Color::Blue, Color::White),
        ToastLevel::Success => (Color::Green, Color::White),
        ToastLevel::Warning => (Color::Yellow, Color::Black),
        ToastLevel::Error => (Color::Red, Color::White),
    };

    let text = vec![Line::from(vec![Span::styled(
        &toast.message,
        Style::default()
            .fg(text_color)
            .add_modifier(Modifier::BOLD),
    )])];

    let paragraph = Paragraph::new(text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(border_color)),
        )
        .alignment(Alignment::Center);

    f.render_widget(paragraph, area);
}

/// Helper function to create toast rect at top center
fn toast_rect(r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(50),
            Constraint::Percentage(25),
        ])
        .split(popup_layout[1])[1]
}
