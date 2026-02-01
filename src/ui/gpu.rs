use crate::app::App;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders},
    Frame,
};

#[cfg(feature = "gpu-nvidia")]
use ratatui::{
    layout::{Constraint, Direction, Layout},
    symbols,
    widgets::{Axis, Chart, Dataset, Gauge, GraphType},
};

/// Render GPU information
#[cfg(feature = "gpu-nvidia")]
pub fn render(f: &mut Frame, app: &App, area: Rect) {
    if let Some(ref gpu_info) = app.gpu_info {
        render_nvidia_gpu(f, app, area, gpu_info);
    } else {
        render_no_gpu(f, area);
    }
}

/// Render GPU information (no GPU feature)
#[cfg(not(feature = "gpu-nvidia"))]
pub fn render(f: &mut Frame, _app: &App, area: Rect) {
    render_no_gpu(f, area);
}

/// Render when no GPU monitoring available
fn render_no_gpu(f: &mut Frame, area: Rect) {
    let text = vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            "GPU Monitoring Not Available",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Compile with --features gpu-nvidia for NVIDIA GPU support",
            Style::default().fg(Color::Gray),
        )]),
    ];

    let paragraph = ratatui::widgets::Paragraph::new(text)
        .block(
            Block::default()
                .title(" GPU Monitor ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow)),
        )
        .alignment(ratatui::layout::Alignment::Center);

    f.render_widget(paragraph, area);
}

#[cfg(feature = "gpu-nvidia")]
fn render_nvidia_gpu(f: &mut Frame, app: &App, area: Rect, gpu_info: &crate::app::GpuInfo) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // GPU usage gauge
            Constraint::Length(3), // Memory gauge
            Constraint::Length(3), // Temperature gauge
            Constraint::Length(8), // Usage graph
        ])
        .split(area);

    // GPU usage gauge
    let gpu_usage = gpu_info.usage as f64 / 100.0;
    let usage_color = get_gpu_usage_color(gpu_info.usage);

    let gauge = Gauge::default()
        .block(
            Block::default()
                .title(format!(" {} - GPU Usage ", gpu_info.name))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Magenta)),
        )
        .gauge_style(
            Style::default()
                .fg(usage_color)
                .bg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .ratio(gpu_usage)
        .label(format!("{:.1}%", gpu_info.usage));

    f.render_widget(gauge, chunks[0]);

    // Memory usage gauge
    let mem_ratio = gpu_info.memory_used as f64 / gpu_info.memory_total as f64;
    let mem_gauge = Gauge::default()
        .block(
            Block::default()
                .title(" GPU Memory ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Magenta)),
        )
        .gauge_style(
            Style::default()
                .fg(Color::Blue)
                .bg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .ratio(mem_ratio)
        .label(format!(
            "{} MB / {} MB ({:.1}%)",
            gpu_info.memory_used,
            gpu_info.memory_total,
            mem_ratio * 100.0
        ));

    f.render_widget(mem_gauge, chunks[1]);

    // Temperature gauge
    let temp_ratio = (gpu_info.temperature as f64 / 100.0).min(1.0);
    let temp_color = get_temp_color(gpu_info.temperature);

    let temp_gauge = Gauge::default()
        .block(
            Block::default()
                .title(" GPU Temperature ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Magenta)),
        )
        .gauge_style(
            Style::default()
                .fg(temp_color)
                .bg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .ratio(temp_ratio)
        .label(format!("{}°C", gpu_info.temperature));

    f.render_widget(temp_gauge, chunks[2]);

    // GPU usage history graph
    let data: Vec<(f64, f64)> = app
        .gpu_usage_history
        .iter()
        .enumerate()
        .map(|(i, &v)| (i as f64, v as f64))
        .collect();

    if !data.is_empty() {
        let datasets = vec![Dataset::default()
            .name("GPU Usage %")
            .marker(symbols::Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default().fg(Color::Magenta))
            .data(&data)];

        let x_max = app.history_size as f64;
        let chart = Chart::new(datasets)
            .block(
                Block::default()
                    .title(" GPU Usage History ")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Magenta)),
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
                    .labels(vec![Span::raw("0"), Span::raw("50"), Span::raw("100")]),
            );

        f.render_widget(chart, chunks[3]);
    }
}

#[cfg(feature = "gpu-nvidia")]
fn get_gpu_usage_color(usage: f32) -> Color {
    if usage < 60.0 {
        Color::Green
    } else if usage < 85.0 {
        Color::Yellow
    } else {
        Color::Red
    }
}

#[cfg(feature = "gpu-nvidia")]
fn get_temp_color(temp: u32) -> Color {
    if temp < 60 {
        Color::Green
    } else if temp < 80 {
        Color::Yellow
    } else {
        Color::Red
    }
}
