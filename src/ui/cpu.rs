use crate::app::App;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Line, Span},
    widgets::{Axis, Block, Borders, Chart, Dataset, Gauge, GraphType, Paragraph, Sparkline},
    Frame,
};

/// Render CPU overview (logo, overall CPU gauge and graph)
pub fn render_overview(f: &mut Frame, app: &App, area: Rect) {
    // Split into logo, overall CPU gauge, and overall CPU graph
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(9),  // Logo/Title (ASCII art)
            Constraint::Length(3),  // Overall CPU gauge
            Constraint::Min(8),     // Overall CPU graph
        ])
        .split(area);

    // Logo/Title
    render_logo(f, chunks[0]);

    // Overall CPU usage with gauge
    render_overall_cpu(f, app, chunks[1]);

    // Overall CPU graph
    render_overall_cpu_graph(f, app, chunks[2]);
}

/// Render per-core CPU information
pub fn render_cores(f: &mut Frame, app: &App, area: Rect) {
    render_per_core_cpu(f, app, area);
}

/// Render RESMON logo/title
fn render_logo(f: &mut Frame, area: Rect) {
    let logo = vec![
        Line::from(vec![
            Span::styled(
                " ██████╗ ███████╗███████╗███╗   ███╗ ██████╗ ███╗   ██╗",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                " ██╔══██╗██╔════╝██╔════╝████╗ ████║██╔═══██╗████╗  ██║",
                Style::default()
                    .fg(Color::LightCyan)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                " ██████╔╝█████╗  ███████╗██╔████╔██║██║   ██║██╔██╗ ██║",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                " ██╔══██╗██╔══╝  ╚════██║██║╚██╔╝██║██║   ██║██║╚██╗██║",
                Style::default()
                    .fg(Color::LightCyan)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                " ██║  ██║███████╗███████║██║ ╚═╝ ██║╚██████╔╝██║ ╚████║",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                " ╚═╝  ╚═╝╚══════╝╚══════╝╚═╝     ╚═╝ ╚═════╝ ╚═╝  ╚═══╝",
                Style::default()
                    .fg(Color::LightCyan),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "              Resource Monitor TUI",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::DIM),
            ),
        ]),
    ];

    let paragraph = Paragraph::new(logo)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .alignment(Alignment::Left);

    f.render_widget(paragraph, area);
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

    // Calculate how many cores per row (4 columns for better fit)
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

/// Render individual core (compact version)
fn render_core(f: &mut Frame, app: &App, area: Rect, core_idx: usize) {
    let cpu = &app.system.cpus()[core_idx];
    let usage = cpu.cpu_usage();
    let frequency = cpu.frequency(); // MHz
    let color = get_usage_color(usage);

    // Get history data for sparkline
    let empty_vec: Vec<f32> = Vec::new();
    let history = if core_idx < app.cpu_history.len() {
        &app.cpu_history[core_idx]
    } else {
        &empty_vec
    };

    // Compact display: gauge with sparkline below
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Gauge with percentage and freq
            Constraint::Min(2),    // Compact sparkline
        ])
        .split(area);

    // Build gauge label with frequency
    let freq_ghz = frequency as f64 / 1000.0;
    let label = if freq_ghz > 0.0 {
        format!("{:.1}% | {:.2}GHz", usage, freq_ghz)
    } else {
        format!("{:.1}%", usage)
    };

    let gauge = Gauge::default()
        .block(
            Block::default()
                .title(format!(" Core {} ", core_idx))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(color)),
        )
        .gauge_style(
            Style::default()
                .fg(color)
                .bg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .ratio(usage as f64 / 100.0)
        .label(label);

    f.render_widget(gauge, chunks[0]);

    // Compact sparkline without border
    if history.len() >= 2 && chunks[1].height >= 1 {
        let sparkline_data: Vec<u64> = history.iter().map(|&v| v as u64).collect();

        let sparkline = Sparkline::default()
            .data(&sparkline_data)
            .style(Style::default().fg(color))
            .max(100);

        f.render_widget(sparkline, chunks[1]);
    }
}

/// Get color based on usage percentage
fn get_usage_color(usage: f32) -> Color {
    if usage < 60.0 {
        Color::Green
    } else if usage < 85.0 {
        Color::Yellow
    } else {
        Color::Red
    }
}
