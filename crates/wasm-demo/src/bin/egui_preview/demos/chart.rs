use super::super::app::EguiPreviewApp;
use eframe::egui::{Id, Ui, vec2};
use egui_shadcn::{BarChart, ChartProps, LineChart, ShadcnChart};

const MONTHS: [&str; 6] = ["Jan", "Feb", "Mar", "Apr", "May", "Jun"];
const VALUES: [f64; 6] = [186.0, 305.0, 237.0, 73.0, 209.0, 214.0];

pub fn render(app: &mut EguiPreviewApp, ui: &mut Ui, compact: bool) {
    let chart_height = if compact { 120.0 } else { 220.0 };
    let points: Vec<[f64; 2]> = VALUES
        .iter()
        .enumerate()
        .map(|(i, v)| [i as f64, *v])
        .collect();
    let bars: Vec<(f64, f64)> = VALUES
        .iter()
        .enumerate()
        .map(|(i, v)| (i as f64, *v))
        .collect();

    let line = LineChart::new(points)
        .label("Desktop")
        .color(app.theme.palette.chart_1);
    let bar = BarChart::new(bars)
        .label("Visitors")
        .color(app.theme.palette.chart_2)
        .bar_width(0.6);

    let formatter: egui_shadcn::chart::AxisFormatter = std::sync::Arc::new(|mark, _range| {
        let idx = mark.value.round() as isize;
        if idx >= 0 && (idx as usize) < MONTHS.len() {
            MONTHS[idx as usize].to_string()
        } else {
            String::new()
        }
    });

    let chart = ShadcnChart::new(
        ChartProps::new(Id::new("preview-chart"))
            .height(chart_height)
            .margin(vec2(12.0, 12.0))
            .x_axis_formatter(formatter)
            .show_legend(!compact),
    );

    let _ = chart.show(ui, &app.theme, |plot_ui| {
        line.show(plot_ui);
        if !compact {
            bar.show(plot_ui);
        }
    });
}
