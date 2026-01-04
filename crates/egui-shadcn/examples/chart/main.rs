#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

#[cfg(feature = "plot")]
#[path = "../_shared/icon.rs"]
mod icon;
#[cfg(feature = "plot")]
#[path = "../_shared/screenshot.rs"]
mod screenshot;

#[cfg(not(feature = "plot"))]
fn main() {
    eprintln!("Enable the `plot` feature: cargo run --example chart --features plot");
}

#[cfg(feature = "plot")]
mod app {
    use eframe::{App, Frame, egui};
    use egui::scroll_area::ScrollSource;
    use egui::{CentralPanel, FontData, FontDefinitions, FontFamily, FontId, RichText, vec2};
    use egui_plot::Bar;
    use egui_shadcn::chart::AxisFormatter;
    use egui_shadcn::{
        BarChart, CardProps, CardVariant, ChartIndicator, ChartLegend, ChartLegendItem, ChartProps,
        ChartTooltip, ChartTooltipItem, LineChart, ShadcnChart, Theme, card,
    };
    use lucide_icons::{Icon, LUCIDE_FONT_BYTES};
    use std::sync::Arc;

    pub struct ChartExample {
        theme: Theme,
    }

    impl ChartExample {
        pub fn new() -> Self {
            Self {
                theme: Theme::default(),
            }
        }
    }

    impl App for ChartExample {
        fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
            super::screenshot::apply_screenshot_scale(ctx);
            ensure_lucide_font(ctx);
            CentralPanel::default().show(ctx, |ui| {
                egui::ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .scroll_source(ScrollSource {
                        drag: false,
                        ..ScrollSource::ALL
                    })
                    .show(ui, |ui| {
                        ui.set_width(ui.available_width());
                        ui.heading("Chart");
                        ui.label("Charts based on egui_plot.");
                        ui.add_space(16.0);
                        ui.spacing_mut().item_spacing.y = 24.0;

                        render_line_demo(ui, &self.theme);
                        render_bar_demo(ui, &self.theme);
                        render_tooltip_demo(ui, &self.theme);
                        render_legend_demo(ui, &self.theme);
                    });
            });
        }
    }

    const MONTH_LABELS: [&str; 6] = ["Jan", "Feb", "Mar", "Apr", "May", "Jun"];
    const WEEKDAY_LABELS: [&str; 6] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
    const LINE_VALUES: [f64; 6] = [186.0, 305.0, 237.0, 73.0, 209.0, 214.0];
    const MOBILE_VALUES: [f64; 6] = [80.0, 200.0, 120.0, 190.0, 130.0, 140.0];
    const RUNNING_VALUES: [f64; 6] = [450.0, 380.0, 520.0, 140.0, 600.0, 480.0];
    const SWIMMING_VALUES: [f64; 6] = [300.0, 420.0, 120.0, 550.0, 350.0, 400.0];

    fn axis_formatter(labels: &'static [&'static str]) -> AxisFormatter {
        Arc::new(move |mark, _range| {
            let index = mark.value.round() as isize;
            if index >= 0 && (index as usize) < labels.len() {
                labels[index as usize].to_string()
            } else {
                String::new()
            }
        })
    }

    fn ensure_lucide_font(ctx: &egui::Context) {
        let font_loaded_id = egui::Id::new("lucide_font_loaded_chart");
        let already_set = ctx.data(|d| d.get_temp::<bool>(font_loaded_id).unwrap_or(false));
        if already_set {
            return;
        }

        let mut fonts = FontDefinitions::default();
        fonts.font_data.insert(
            "lucide".into(),
            FontData::from_static(LUCIDE_FONT_BYTES).into(),
        );
        fonts
            .families
            .entry(FontFamily::Proportional)
            .or_default()
            .insert(0, "lucide".into());
        ctx.set_fonts(fonts);
        ctx.data_mut(|d| d.insert_temp(font_loaded_id, true));
    }

    fn render_card_header(ui: &mut egui::Ui, theme: &Theme, title: &str, description: &str) {
        ui.spacing_mut().item_spacing.y = 4.0;
        ui.label(
            RichText::new(title)
                .size(14.0)
                .strong()
                .color(theme.palette.foreground),
        );
        ui.label(
            RichText::new(description)
                .size(12.0)
                .color(theme.palette.muted_foreground),
        );
    }

    fn render_trending_footer(ui: &mut egui::Ui, theme: &Theme, subtitle: &str) {
        ui.spacing_mut().item_spacing.y = 4.0;
        ui.horizontal(|row| {
            row.spacing_mut().item_spacing.x = 6.0;
            row.label(
                RichText::new("Trending up by 5.2% this month")
                    .size(12.0)
                    .strong()
                    .color(theme.palette.foreground),
            );
            row.label(
                RichText::new(Icon::TrendingUp.unicode())
                    .font(FontId::proportional(14.0))
                    .color(theme.palette.foreground),
            );
        });
        ui.label(
            RichText::new(subtitle)
                .size(11.0)
                .color(theme.palette.muted_foreground),
        );
    }

    fn chart_height(ui: &egui::Ui) -> f32 {
        let height = ui.available_width() * 9.0 / 16.0;
        height.clamp(160.0, 260.0)
    }

    fn render_line_demo(ui: &mut egui::Ui, theme: &Theme) {
        card(
            ui,
            theme,
            CardProps::default()
                .with_variant(CardVariant::Outline)
                .with_shadow(false)
                .with_padding(vec2(24.0, 24.0)),
            |card_ui| {
                card_ui.spacing_mut().item_spacing.y = 12.0;
                render_card_header(card_ui, theme, "Line Chart", "January - June 2024");

                card_ui.vertical(|content| {
                    let height = chart_height(content);
                    let points = series_points(&LINE_VALUES);
                    let line = LineChart::new(points)
                        .label("Desktop")
                        .color(theme.palette.chart_1);
                    let chart = ShadcnChart::new(
                        ChartProps::new(egui::Id::new("chart-line-default"))
                            .height(height)
                            .margin(vec2(12.0, 12.0))
                            .show_legend(false)
                            .x_axis_formatter(axis_formatter(&MONTH_LABELS)),
                    );
                    let response = chart.show(content, theme, |plot_ui| {
                        line.show(plot_ui);
                    });

                    if let (Some(point), Some(pos)) = (response.hovered_point, response.pointer_pos)
                        && let Some((_, label, value)) =
                            lookup_value(&MONTH_LABELS, &LINE_VALUES, point.x)
                    {
                            let items = [ChartTooltipItem {
                                label: "Desktop".to_string(),
                                value,
                                color: theme.palette.chart_1,
                            }];
                            ChartTooltip::new(&items)
                                .title(label)
                                .hide_label(true)
                                .show(
                                    content.ctx(),
                                    theme,
                                    egui::Id::new("chart-line-tooltip"),
                                    pos,
                                );
                        }
                    }
                });

                render_trending_footer(
                    card_ui,
                    theme,
                    "Showing total visitors for the last 6 months",
                );
            },
        );
    }

    fn render_bar_demo(ui: &mut egui::Ui, theme: &Theme) {
        card(
            ui,
            theme,
            CardProps::default()
                .with_variant(CardVariant::Outline)
                .with_shadow(false)
                .with_padding(vec2(24.0, 24.0)),
            |card_ui| {
                card_ui.spacing_mut().item_spacing.y = 12.0;
                render_card_header(card_ui, theme, "Bar Chart", "January - June 2024");

                card_ui.vertical(|content| {
                    let height = chart_height(content);
                    let values = series_values(&LINE_VALUES);
                    let bars = BarChart::new(values)
                        .label("Desktop")
                        .color(theme.palette.chart_1)
                        .bar_width(0.7);
                    let chart = ShadcnChart::new(
                        ChartProps::new(egui::Id::new("chart-bar-default"))
                            .height(height)
                            .margin(vec2(12.0, 12.0))
                            .show_legend(false)
                            .x_axis_formatter(axis_formatter(&MONTH_LABELS)),
                    );
                    let response = chart.show(content, theme, |plot_ui| {
                        bars.show(plot_ui);
                    });

                    if let (Some(point), Some(pos)) = (response.hovered_point, response.pointer_pos)
                        && let Some((_, label, value)) =
                            lookup_value(&MONTH_LABELS, &LINE_VALUES, point.x)
                    {
                        let items = [ChartTooltipItem {
                            label: "Desktop".to_string(),
                            value,
                            color: theme.palette.chart_1,
                        }];
                        ChartTooltip::new(&items)
                            .title(label)
                            .hide_label(true)
                            .show(
                                content.ctx(),
                                theme,
                                egui::Id::new("chart-bar-tooltip"),
                                pos,
                            );
                    }
                });

                render_trending_footer(
                    card_ui,
                    theme,
                    "Showing total visitors for the last 6 months",
                );
            },
        );
    }

    fn render_tooltip_demo(ui: &mut egui::Ui, theme: &Theme) {
        card(
            ui,
            theme,
            CardProps::default()
                .with_variant(CardVariant::Outline)
                .with_shadow(false)
                .with_padding(vec2(24.0, 24.0)),
            |card_ui| {
                card_ui.spacing_mut().item_spacing.y = 12.0;
                render_card_header(
                    card_ui,
                    theme,
                    "Tooltip - Default",
                    "Default tooltip with ChartTooltipContent.",
                );

                card_ui.vertical(|content| {
                    let height = chart_height(content);
                    let running =
                        series_bars_with_base(&RUNNING_VALUES, None, theme.palette.chart_1);
                    let swimming = series_bars_with_base(
                        &SWIMMING_VALUES,
                        Some(&RUNNING_VALUES),
                        theme.palette.chart_2,
                    );
                    let chart = ShadcnChart::new(
                        ChartProps::new(egui::Id::new("chart-tooltip-default"))
                            .height(height)
                            .margin(vec2(12.0, 12.0))
                            .show_legend(false)
                            .x_axis_formatter(axis_formatter(&WEEKDAY_LABELS)),
                    );
                    let response = chart.show(content, theme, |plot_ui| {
                        BarChart::from_bars(running).show(plot_ui);
                        BarChart::from_bars(swimming).show(plot_ui);
                    });

                    if let (Some(point), Some(pos)) = (response.hovered_point, response.pointer_pos)
                        && let Some((index, label, running_value)) =
                            lookup_value(&WEEKDAY_LABELS, &RUNNING_VALUES, point.x)
                    {
                            let swimming_value = SWIMMING_VALUES[index];
                            let items = [
                                ChartTooltipItem {
                                    label: "Running".to_string(),
                                    value: running_value,
                                    color: theme.palette.chart_1,
                                },
                                ChartTooltipItem {
                                    label: "Swimming".to_string(),
                                    value: format!("{:.0}", swimming_value),
                                    color: theme.palette.chart_2,
                                },
                            ];
                            ChartTooltip::new(&items).title(label).show(
                                content.ctx(),
                                theme,
                                egui::Id::new("chart-tooltip-default-ui"),
                                pos,
                            );
                        }
                    }
                });
            },
        );
    }

    fn render_legend_demo(ui: &mut egui::Ui, theme: &Theme) {
        card(
            ui,
            theme,
            CardProps::default()
                .with_variant(CardVariant::Outline)
                .with_shadow(false)
                .with_padding(vec2(24.0, 24.0)),
            |card_ui| {
                card_ui.spacing_mut().item_spacing.y = 12.0;
                render_card_header(
                    card_ui,
                    theme,
                    "Area Chart - Legend",
                    "Showing total visitors for the last 6 months",
                );

                card_ui.vertical(|content| {
                    let height = chart_height(content);
                    let desktop = LineChart::new(series_points(&LINE_VALUES))
                        .label("Desktop")
                        .color(theme.palette.chart_1)
                        .fill_alpha(0.4);
                    let mobile = LineChart::new(series_points(&MOBILE_VALUES))
                        .label("Mobile")
                        .color(theme.palette.chart_2)
                        .fill_alpha(0.4);
                    let chart = ShadcnChart::new(
                        ChartProps::new(egui::Id::new("chart-legend-demo"))
                            .height(height)
                            .margin(vec2(12.0, 12.0))
                            .show_legend(false)
                            .x_axis_formatter(axis_formatter(&MONTH_LABELS)),
                    );
                    let response = chart.show(content, theme, |plot_ui| {
                        mobile.show(plot_ui);
                        desktop.show(plot_ui);
                    });

                    if let (Some(point), Some(pos)) = (response.hovered_point, response.pointer_pos)
                        && let Some((index, label, desktop_value)) =
                            lookup_value(&MONTH_LABELS, &LINE_VALUES, point.x)
                    {
                        let mobile_value = MOBILE_VALUES[index];
                        let items = [
                            ChartTooltipItem {
                                label: "Mobile".to_string(),
                                value: format!("{:.0}", mobile_value),
                                color: theme.palette.chart_2,
                            },
                            ChartTooltipItem {
                                label: "Desktop".to_string(),
                                value: desktop_value,
                                color: theme.palette.chart_1,
                            },
                        ];
                        ChartTooltip::new(&items)
                            .title(label)
                            .indicator(ChartIndicator::Line)
                            .show(
                                content.ctx(),
                                theme,
                                egui::Id::new("chart-legend-tooltip"),
                                pos,
                            );
                    }

                    content.add_space(8.0);
                    let legend_items = [
                        ChartLegendItem {
                            label: "Desktop".to_string(),
                            color: theme.palette.chart_1,
                        },
                        ChartLegendItem {
                            label: "Mobile".to_string(),
                            color: theme.palette.chart_2,
                        },
                    ];
                    let _ = ChartLegend::new(&legend_items).show(content, theme);
                });

                render_trending_footer(card_ui, theme, "January - June 2024");
            },
        );
    }

    fn series_points(values: &[f64]) -> Vec<[f64; 2]> {
        values
            .iter()
            .enumerate()
            .map(|(idx, value)| [idx as f64, *value])
            .collect()
    }

    fn series_values(values: &[f64]) -> Vec<(f64, f64)> {
        values
            .iter()
            .enumerate()
            .map(|(idx, value)| (idx as f64, *value))
            .collect()
    }

    fn series_bars_with_base(
        values: &[f64],
        base: Option<&[f64]>,
        color: egui::Color32,
    ) -> Vec<Bar> {
        values
            .iter()
            .enumerate()
            .map(|(idx, value)| {
                let mut bar = Bar::new(idx as f64, *value)
                    .width(0.7)
                    .fill(color)
                    .stroke(egui::Stroke::new(1.0, egui::Color32::TRANSPARENT));
                if let Some(base_values) = base
                    && let Some(offset) = base_values.get(idx)
                {
                    bar = bar.base_offset(*offset);
                }
                bar
            })
            .collect()
    }

    fn lookup_value<'a>(
        labels: &'a [&'a str],
        values: &[f64],
        x: f64,
    ) -> Option<(usize, &'a str, String)> {
        let index = x.round() as isize;
        if index < 0 || (index as usize) >= values.len() {
            return None;
        }
        let idx = index as usize;
        let label = labels.get(idx).copied()?;
        Some((idx, label, format!("{:.0}", values[idx])))
    }
}

#[cfg(feature = "plot")]
fn main() -> eframe::Result<()> {
    env_logger::init();
    let options = icon::native_options();
    eframe::run_native(
        "Chart example",
        options,
        Box::new(|_cc| Ok(Box::new(app::ChartExample::new()))),
    )
}
