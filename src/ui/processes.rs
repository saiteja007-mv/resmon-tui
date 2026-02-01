use crate::app::App;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

/// Render process list
pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let processes = app.get_display_processes();

    // Calculate how many processes can fit in the view
    let visible_count = (area.height.saturating_sub(2)) as usize;

    // Adjust scroll offset if needed
    let scroll_offset = if let Some(selected) = app.selected_process {
        if selected < app.scroll_offset {
            selected
        } else if selected >= app.scroll_offset + visible_count {
            selected - visible_count + 1
        } else {
            app.scroll_offset
        }
    } else {
        0
    };

    // Create list items
    let items: Vec<ListItem> = processes
        .iter()
        .enumerate()
        .skip(scroll_offset)
        .take(visible_count)
        .map(|(idx, (pid, process))| {
            let name = process.name().to_string_lossy();
            let cpu = process.cpu_usage();
            let mem = process.memory() / 1024 / 1024; // Convert to MB

            // Highlight selected process
            let style = if Some(idx) == app.selected_process {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };

            let cpu_color = if cpu >= 85.0 {
                Color::Red
            } else if cpu >= 60.0 {
                Color::Yellow
            } else {
                Color::Green
            };

            let content = vec![Line::from(vec![
                Span::styled(format!("{:<8}", pid), style),
                Span::raw(" "),
                Span::styled(format!("{:<30}", truncate_string(&name, 30)), style),
                Span::raw(" "),
                Span::styled(
                    format!("{:>6.1}%", cpu),
                    if Some(idx) == app.selected_process {
                        style
                    } else {
                        Style::default().fg(cpu_color)
                    },
                ),
                Span::raw(" "),
                Span::styled(format!("{:>8} MB", mem), style),
            ])];

            ListItem::new(content)
        })
        .collect();

    // Header with sort indicators
    let pid_header = match app.sort_order {
        crate::app::SortOrder::Pid => "PID ▼   ",
        _ => "PID     ",
    };
    let cpu_header = match app.sort_order {
        crate::app::SortOrder::Cpu => "CPU ▼ ",
        _ => "CPU   ",
    };
    let mem_header = match app.sort_order {
        crate::app::SortOrder::Memory => "Memory ▼",
        _ => "Memory  ",
    };

    let header = vec![Line::from(vec![
        Span::styled(pid_header, Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::raw(" "),
        Span::styled(
            format!("{:<30}", "Process Name"),
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        ),
        Span::raw(" "),
        Span::styled(cpu_header, Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::raw(" "),
        Span::styled(mem_header, Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
    ])];

    let title = if app.search_mode {
        format!(" Search: {} ", app.search_query)
    } else if app.show_details {
        " Processes (↑/↓: Navigate, Enter: Details, Esc: Close Details) ".to_string()
    } else {
        " Processes (↑/↓: Navigate, Enter: View Details, q: Quit) ".to_string()
    };

    let border_color = if app.search_mode {
        Color::Yellow
    } else {
        Color::Cyan
    };

    let list = List::new(items)
        .block(
            Block::default()
                .title(title)
                .borders(Borders::ALL)
                .border_style(Style::default().fg(border_color)),
        )
        .style(Style::default().fg(Color::White));

    // Render header separately
    if area.height > 2 {
        let header_area = Rect {
            x: area.x + 1,
            y: area.y + 1,
            width: area.width.saturating_sub(2),
            height: 1,
        };

        let header_paragraph = ratatui::widgets::Paragraph::new(header);
        f.render_widget(header_paragraph, header_area);
    }

    // Render process list
    let list_area = Rect {
        x: area.x,
        y: area.y,
        width: area.width,
        height: area.height,
    };

    f.render_widget(list, list_area);
}

/// Truncate string to max length
fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len - 3])
    }
}
