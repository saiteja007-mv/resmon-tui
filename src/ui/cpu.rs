use crate::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Line, Span},
    widgets::{Axis, Block, Borders, Chart, Dataset, Gauge, GraphType, Sparkline},
    Frame,
};

/// Render CPU information
pub fn render(f: &mut Frame, app: &App, area: Rect) {
    // Split into overall CPU, graph, and per-core stats
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Overall CPU gauge
            Constraint::Length(8),  // Overall CPU graph
            Constraint::Min(8),     // Per-core stats
        ])
        .split(area);

    // Overall CPU usage with gauge
    render_overall_cpu(f, app, chunks[0]);

    // Overall CPU graph
    render_overall_cpu_graph(f, app, chunks[1]);

    // Per-core CPU usage with sparklines
    render_per_core_cpu(f, app, chunks[2]);
}

/// Render overall CPU usage
fn render_overall_cpu(f: &mut Frame, app: &App, area: Rect) {
    let cpu_usage = app.system.global_cpu_usage();
    let color = get_usage_color(cpu_usage);

    let gauge = Gauge::default()
        .block(
            Block::default()
                .title(" Overall CPU Usage ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .gauge_style(Style::default().fg(color).bg(Color::Black).add_modifier(Modifier::BOLD))
        .ratio(cpu_usage as f64 / 100.0)
        .label(format!("{:.1}%", cpu_usage));

    f.render_widget(gauge, area);
}

/// Render overall CPU graph
fn render_overall_cpu_graph(f: &mut Frame, app: &App, area: Rect) {
    let data: Vec<(f64, f64)> = app
        .overall_cpu_history
        .iter()
        .enumerate()
        .map(|(i, &v)| (i as f64, v as f64))
        .collect();

    let datasets = vec![Dataset::default()
        .name("CPU %")
        .marker(symbols::Marker::Braille)
        .graph_type(GraphType::Line)
        .style(Style::default().fg(Color::Cyan))
        .data(&data)];

    let x_max = app.history_size as f64;
    let chart = Chart::new(datasets)
        .block(
            Block::default()
                .title(" Overall CPU History ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .x_axis(
            Axis::default()
                .style(Style::default().fg(Color::Gray))
                .bounds([0.0, x_max]),
        )
        .y_axis(
            Axis::default()
                .style(Style::default().fg(Color::Gray))
                .bounds([0.0, 100.0])
                .labels(vec![
                    Span::raw("0"),
                    Span::raw("50"),
                    Span::raw("100"),
                ]),
        );

    f.render_widget(chart, area);
}

/// Render per-core CPU usage
fn render_per_core_cpu(f: &mut Frame, app: &App, area: Rect) {
    let cpus = app.system.cpus();
    let core_count = cpus.len();

    // Calculate how many cores per row (max 4 columns)
    let cols = 4.min(core_count);
    let rows = (core_count + cols - 1) / cols;

    // Create layout
    let mut constraints = vec![];
    for _ in 0..rows {
        constraints.push(Constraint::Ratio(1, rows as u32));
    }

    let row_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(area);

    for row in 0..rows {
        // Split each row into columns
        let mut col_constraints = vec![];
        let cores_in_row = cols.min(core_count - row * cols);
        for _ in 0..cores_in_row {
            col_constraints.push(Constraint::Ratio(1, cores_in_row as u32));
        }

        let col_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(col_constraints)
            .split(row_chunks[row]);

        for col in 0..cores_in_row {
            let core_idx = row * cols + col;
            if core_idx < core_count {
                render_core(f, app, col_chunks[col], core_idx);
            }
        }
    }
}

/// Render individual core
fn render_core(f: &mut Frame, app: &App, area: Rect, core_idx: usize) {
    let cpu = &app.system.cpus()[core_idx];
    let usage = cpu.cpu_usage();
    let color = get_usage_color(usage);

    // Get history data for sparkline
    let empty_vec: Vec<f32> = Vec::new();
    let history = if core_idx < app.cpu_history.len() {
        &app.cpu_history[core_idx]
    } else {
        &empty_vec
    };

    // Need at least 2 data points for sparkline
    if history.len() < 2 {
        // Just show percentage without graph
        let text = vec![
            Line::from(""),
            Line::from(vec![Span::styled(
                format!("{:.0}%", usage),
                Style::default().fg(color).add_modifier(Modifier::BOLD),
            )]),
        ];

        let paragraph = ratatui::widgets::Paragraph::new(text)
            .block(
                Block::default()
                    .title(format!(" Core {} ", core_idx))
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(color)),
            )
            .alignment(ratatui::layout::Alignment::Center);

        f.render_widget(paragraph, area);
        return;
    }

    let sparkline_data: Vec<u64> = history.iter().map(|&v| v as u64).collect();

    // Split area into graph and percentage
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(3), Constraint::Length(1)])
        .split(area);

    let sparkline = Sparkline::default()
        .block(
            Block::default()
                .title(format!(" Core {} ", core_idx))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(color)),
        )
        .data(&sparkline_data)
        .style(Style::default().fg(color))
        .max(100);

    f.render_widget(sparkline, chunks[0]);

    // Render usage percentage at the bottom
    if chunks[1].height > 0 && chunks[1].width > 5 {
        let text = vec![Line::from(vec![Span::styled(
            format!("{:>5.1}%", usage),
            Style::default().fg(color).add_modifier(Modifier::BOLD),
        )])];

        let paragraph = ratatui::widgets::Paragraph::new(text)
            .alignment(ratatui::layout::Alignment::Center);

        let text_area = Rect {
            x: chunks[1].x,
            y: chunks[0].y + chunks[0].height - 1,
            width: chunks[1].width,
            height: 1,
        };

        f.render_widget(paragraph, text_area);
    }
}

/// Get color based on usage percentage
fn get_usage_color(usage: f32) -> Color {
    if usage < 50.0 {
        Color::Green
    } else if usage < 75.0 {
        Color::Yellow
    } else {
        Color::Red
    }
}
