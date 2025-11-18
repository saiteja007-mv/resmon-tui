use crate::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, Paragraph, Wrap},
    Frame,
};

/// Render process details
pub fn render(f: &mut Frame, app: &App, area: Rect) {
    if let Some((_pid, process)) = app.get_selected_process() {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),  // CPU gauge
                Constraint::Length(3),  // Memory gauge
                Constraint::Min(5),     // Details text
            ])
            .split(area);

        // CPU usage gauge
        let cpu_usage = process.cpu_usage();
        let cpu_color = if cpu_usage > 75.0 {
            Color::Red
        } else if cpu_usage > 50.0 {
            Color::Yellow
        } else {
            Color::Green
        };

        let cpu_gauge = Gauge::default()
            .block(
                Block::default()
                    .title(" CPU Usage ")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Cyan)),
            )
            .gauge_style(Style::default().fg(cpu_color).add_modifier(Modifier::BOLD))
            .ratio((cpu_usage as f64 / 100.0).min(1.0))
            .label(format!("{:.1}%", cpu_usage));

        f.render_widget(cpu_gauge, chunks[0]);

        // Memory usage gauge
        let mem_usage = process.memory() / 1024 / 1024; // MB
        let total_mem = app.system.total_memory() / 1024 / 1024; // MB
        let mem_ratio = (mem_usage as f64 / total_mem as f64).min(1.0);

        let mem_gauge = Gauge::default()
            .block(
                Block::default()
                    .title(" Memory Usage ")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Cyan)),
            )
            .gauge_style(Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD))
            .ratio(mem_ratio)
            .label(format!("{} MB", mem_usage));

        f.render_widget(mem_gauge, chunks[1]);

        // Process details
        let mut details = vec![];

        details.push(Line::from(vec![
            Span::styled("Name: ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw(process.name().to_string_lossy().to_string()),
        ]));

        details.push(Line::from(vec![
            Span::styled("PID: ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw(format!("{}", process.pid())),
        ]));

        if let Some(parent) = process.parent() {
            details.push(Line::from(vec![
                Span::styled("Parent PID: ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::raw(format!("{}", parent)),
            ]));
        }

        details.push(Line::from(vec![
            Span::styled("Status: ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw(format!("{:?}", process.status())),
        ]));

        if let Some(exe) = process.exe() {
            details.push(Line::from(vec![
                Span::styled("Executable: ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::raw(exe.to_string_lossy().to_string()),
            ]));
        }

        if let Some(cwd) = process.cwd() {
            details.push(Line::from(vec![
                Span::styled("Working Dir: ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::raw(cwd.to_string_lossy().to_string()),
            ]));
        }

        details.push(Line::from(""));
        details.push(Line::from(vec![
            Span::styled("Disk Usage: ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        ]));

        let disk_usage = process.disk_usage();
        details.push(Line::from(vec![
            Span::raw("  Read: "),
            Span::styled(
                format!("{} bytes", disk_usage.total_read_bytes),
                Style::default().fg(Color::Cyan),
            ),
        ]));
        details.push(Line::from(vec![
            Span::raw("  Write: "),
            Span::styled(
                format!("{} bytes", disk_usage.total_written_bytes),
                Style::default().fg(Color::Cyan),
            ),
        ]));

        details.push(Line::from(""));
        details.push(Line::from(vec![
            Span::styled("Virtual Memory: ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw(format!("{} MB", process.virtual_memory() / 1024 / 1024)),
        ]));

        let paragraph = Paragraph::new(details)
            .block(
                Block::default()
                    .title(" Process Details ")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Cyan)),
            )
            .wrap(Wrap { trim: true });

        f.render_widget(paragraph, chunks[2]);
    } else {
        // No process selected
        let text = vec![Line::from(vec![Span::styled(
            "No process selected",
            Style::default().fg(Color::Gray).add_modifier(Modifier::ITALIC),
        )])];

        let paragraph = Paragraph::new(text)
            .block(
                Block::default()
                    .title(" Process Details ")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Cyan)),
            )
            .alignment(ratatui::layout::Alignment::Center);

        f.render_widget(paragraph, area);
    }
}
