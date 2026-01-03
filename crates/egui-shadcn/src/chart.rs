use crate::theme::Theme;
use egui::{
    Align, Color32, FontId, Frame, Id, InnerResponse, Layout, Margin, Pos2, RichText, Stroke, Ui,
    Vec2, Vec2b, pos2, vec2,
};
use egui_plot::{
    Bar, BarChart as PlotBarChart, Corner, GridMark, Legend, Line, Plot, PlotPoint, PlotUi,
};
use std::ops::RangeInclusive;
use std::sync::Arc;

pub type AxisFormatter = Arc<dyn Fn(GridMark, &RangeInclusive<f64>) -> String + Send + Sync>;

#[derive(Clone)]
pub struct ChartProps {
    pub id_source: Id,
    pub title: Option<String>,
    pub show_legend: bool,
    pub show_tooltip: bool,
    pub show_grid: Vec2b,
    pub show_x: bool,
    pub show_y: bool,
    pub height: f32,
    pub margin: Vec2,
    pub x_axis_formatter: Option<AxisFormatter>,
    pub y_axis_formatter: Option<AxisFormatter>,
}

impl std::fmt::Debug for ChartProps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ChartProps")
            .field("id_source", &self.id_source)
            .field("title", &self.title)
            .field("show_legend", &self.show_legend)
            .field("show_tooltip", &self.show_tooltip)
            .field("show_grid", &self.show_grid)
            .field("show_x", &self.show_x)
            .field("show_y", &self.show_y)
            .field("height", &self.height)
            .field("margin", &self.margin)
            .field("x_axis_formatter", &self.x_axis_formatter.is_some())
            .field("y_axis_formatter", &self.y_axis_formatter.is_some())
            .finish()
    }
}

impl ChartProps {
    pub fn new(id_source: Id) -> Self {
        Self {
            id_source,
            title: None,
            show_legend: true,
            show_tooltip: true,
            show_grid: Vec2b::new(false, true),
            show_x: true,
            show_y: false,
            height: 220.0,
            margin: vec2(8.0, 8.0),
            x_axis_formatter: None,
            y_axis_formatter: None,
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn show_legend(mut self, show: bool) -> Self {
        self.show_legend = show;
        self
    }

    pub fn show_tooltip(mut self, show: bool) -> Self {
        self.show_tooltip = show;
        self
    }

    pub fn show_grid(mut self, show: Vec2b) -> Self {
        self.show_grid = show;
        self
    }

    pub fn show_x(mut self, show: bool) -> Self {
        self.show_x = show;
        self
    }

    pub fn show_y(mut self, show: bool) -> Self {
        self.show_y = show;
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    pub fn margin(mut self, margin: Vec2) -> Self {
        self.margin = margin;
        self
    }

    pub fn x_axis_formatter(mut self, formatter: AxisFormatter) -> Self {
        self.x_axis_formatter = Some(formatter);
        self
    }

    pub fn y_axis_formatter(mut self, formatter: AxisFormatter) -> Self {
        self.y_axis_formatter = Some(formatter);
        self
    }
}

pub struct ChartResponse<R> {
    pub inner: R,
    pub response: egui::Response,
    pub hovered_point: Option<PlotPoint>,
    pub pointer_pos: Option<Pos2>,
}

pub struct ShadcnChart {
    props: ChartProps,
}

impl ShadcnChart {
    pub fn new(props: ChartProps) -> Self {
        Self { props }
    }

    pub fn show<R>(
        &self,
        ui: &mut Ui,
        theme: &Theme,
        add_plot: impl FnOnce(&mut PlotUi) -> R,
    ) -> ChartResponse<R> {
        if let Some(title) = &self.props.title {
            ui.label(RichText::new(title).strong());
            ui.add_space(8.0);
        }

        let frame = Frame::NONE
            .fill(theme.palette.card)
            .stroke(Stroke::new(1.0, theme.palette.border))
            .corner_radius(theme.radius.r4)
            .inner_margin(Margin::from(self.props.margin));

        let mut hovered_point: Option<PlotPoint> = None;
        let inner = frame.show(ui, |frame_ui| {
            let mut plot = Plot::new(self.props.id_source)
                .height(self.props.height)
                .show_background(false)
                .show_grid(self.props.show_grid)
                .show_x(self.props.show_x)
                .show_y(self.props.show_y);

            if let Some(formatter) = &self.props.x_axis_formatter {
                let formatter = Arc::clone(formatter);
                plot = plot.x_axis_formatter(move |mark, range| (formatter)(mark, range));
            }

            if let Some(formatter) = &self.props.y_axis_formatter {
                let formatter = Arc::clone(formatter);
                plot = plot.y_axis_formatter(move |mark, range| (formatter)(mark, range));
            }

            if self.props.show_legend {
                plot = plot.legend(Legend::default().position(Corner::RightTop));
            }

            plot.show(frame_ui, |plot_ui| {
                if self.props.show_tooltip {
                    hovered_point = plot_ui.pointer_coordinate();
                }
                add_plot(plot_ui)
            })
        });

        let plot_response = inner.inner;
        let pointer_pos = if self.props.show_tooltip {
            ui.ctx().input(|i| i.pointer.hover_pos())
        } else {
            None
        };

        ChartResponse {
            inner: plot_response.inner,
            response: plot_response.response,
            hovered_point,
            pointer_pos,
        }
    }
}

pub fn chart<R>(
    ui: &mut Ui,
    theme: &Theme,
    props: ChartProps,
    add_plot: impl FnOnce(&mut PlotUi) -> R,
) -> ChartResponse<R> {
    ShadcnChart::new(props).show(ui, theme, add_plot)
}

#[derive(Clone, Debug)]
pub struct LineChart {
    points: Vec<[f64; 2]>,
    label: Option<String>,
    color: Color32,
    fill_alpha: Option<f32>,
    fill_base: Option<f32>,
    stroke_width: f32,
}

impl LineChart {
    pub fn new(points: Vec<[f64; 2]>) -> Self {
        Self {
            points,
            label: None,
            color: Color32::WHITE,
            fill_alpha: None,
            fill_base: None,
            stroke_width: 2.0,
        }
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn color(mut self, color: Color32) -> Self {
        self.color = color;
        self
    }

    pub fn fill_alpha(mut self, alpha: f32) -> Self {
        self.fill_alpha = Some(alpha);
        self
    }

    pub fn fill_base(mut self, base: f32) -> Self {
        self.fill_base = Some(base);
        self
    }

    pub fn stroke_width(mut self, width: f32) -> Self {
        self.stroke_width = width;
        self
    }

    pub fn show(&self, plot_ui: &mut PlotUi) {
        let name = self.label.clone().unwrap_or_else(|| "Series".to_string());
        let mut line = Line::new(name, self.points.clone())
            .color(self.color)
            .width(self.stroke_width);
        if let Some(alpha) = self.fill_alpha {
            let base = self.fill_base.unwrap_or(0.0);
            line = line.fill(base).fill_alpha(alpha);
        }
        plot_ui.line(line);
    }
}

#[derive(Clone, Debug)]
pub struct BarChart {
    bars: Vec<Bar>,
    label: Option<String>,
    color: Color32,
    bar_width: Option<f64>,
}

impl BarChart {
    pub fn new(values: Vec<(f64, f64)>) -> Self {
        let bars = values.into_iter().map(|(x, y)| Bar::new(x, y)).collect();
        Self {
            bars,
            label: None,
            color: Color32::WHITE,
            bar_width: Some(0.6),
        }
    }

    pub fn from_bars(bars: Vec<Bar>) -> Self {
        Self {
            bars,
            label: None,
            color: Color32::WHITE,
            bar_width: None,
        }
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn color(mut self, color: Color32) -> Self {
        self.color = color;
        self
    }

    pub fn bar_width(mut self, width: f64) -> Self {
        self.bar_width = Some(width);
        self
    }

    pub fn show(&self, plot_ui: &mut PlotUi) {
        let name = self.label.clone().unwrap_or_else(|| "Series".to_string());
        let mut chart = PlotBarChart::new(name, self.bars.clone()).color(self.color);
        if let Some(width) = self.bar_width {
            chart = chart.width(width);
        }
        plot_ui.bar_chart(chart);
    }
}

#[derive(Clone, Debug)]
pub struct ChartLegendItem {
    pub label: String,
    pub color: Color32,
}

pub struct ChartLegend<'a> {
    items: &'a [ChartLegendItem],
}

impl<'a> ChartLegend<'a> {
    pub fn new(items: &'a [ChartLegendItem]) -> Self {
        Self { items }
    }

    pub fn show(self, ui: &mut Ui, theme: &Theme) -> InnerResponse<()> {
        ui.with_layout(Layout::left_to_right(Align::Center), |legend_ui| {
            legend_ui.spacing_mut().item_spacing.x = 16.0;
            for item in self.items {
                legend_ui.horizontal(|row| {
                    row.spacing_mut().item_spacing.x = 6.0;
                    let (rect, _) = row.allocate_exact_size(vec2(8.0, 8.0), egui::Sense::hover());
                    row.painter().rect_filled(rect, theme.radius.r2, item.color);
                    row.label(
                        RichText::new(&item.label)
                            .size(12.0)
                            .color(theme.palette.muted_foreground),
                    );
                });
            }
        })
    }
}

#[derive(Clone, Debug)]
pub struct ChartTooltipItem {
    pub label: String,
    pub value: String,
    pub color: Color32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChartIndicator {
    Dot,
    Line,
    Dashed,
}

pub struct ChartTooltip<'a> {
    title: Option<&'a str>,
    items: &'a [ChartTooltipItem],
    indicator: ChartIndicator,
    hide_label: bool,
    hide_indicator: bool,
}

impl<'a> ChartTooltip<'a> {
    pub fn new(items: &'a [ChartTooltipItem]) -> Self {
        Self {
            title: None,
            items,
            indicator: ChartIndicator::Dot,
            hide_label: false,
            hide_indicator: false,
        }
    }

    pub fn title(mut self, title: &'a str) -> Self {
        self.title = Some(title);
        self
    }

    pub fn indicator(mut self, indicator: ChartIndicator) -> Self {
        self.indicator = indicator;
        self
    }

    pub fn hide_label(mut self, hide: bool) -> Self {
        self.hide_label = hide;
        self
    }

    pub fn hide_indicator(mut self, hide: bool) -> Self {
        self.hide_indicator = hide;
        self
    }

    pub fn show(self, ctx: &egui::Context, theme: &Theme, id: Id, pos: Pos2) {
        let frame = Frame::NONE
            .fill(theme.palette.popover)
            .stroke(Stroke::new(1.0, theme.palette.border))
            .corner_radius(theme.radius.r3)
            .inner_margin(Margin::symmetric(10, 8));

        egui::Area::new(id)
            .fixed_pos(pos + vec2(12.0, 12.0))
            .order(egui::Order::Tooltip)
            .show(ctx, |ui| {
                frame.show(ui, |tooltip_ui| {
                    tooltip_ui.spacing_mut().item_spacing.y = 6.0;
                    if let Some(title) = self.title
                        && !self.hide_label
                    {
                        tooltip_ui.label(
                            RichText::new(title)
                                .size(12.0)
                                .strong()
                                .color(theme.palette.foreground),
                        );
                    }

                    for item in self.items {
                        tooltip_ui.horizontal(|row| {
                            row.spacing_mut().item_spacing.x = 8.0;
                            if !self.hide_indicator {
                                let indicator_size = match self.indicator {
                                    ChartIndicator::Dot => vec2(8.0, 8.0),
                                    ChartIndicator::Line | ChartIndicator::Dashed => {
                                        vec2(6.0, 12.0)
                                    }
                                };
                                let (rect, _) =
                                    row.allocate_exact_size(indicator_size, egui::Sense::hover());
                                let center_x = rect.center().x;
                                let painter = row.painter();
                                match self.indicator {
                                    ChartIndicator::Dot => {
                                        painter.rect_filled(rect, theme.radius.r2, item.color);
                                    }
                                    ChartIndicator::Line => {
                                        let line_rect = rect.shrink2(vec2(2.0, 0.0));
                                        painter.rect_filled(line_rect, theme.radius.r1, item.color);
                                    }
                                    ChartIndicator::Dashed => {
                                        let top = rect.top() + 1.0;
                                        let bottom = rect.bottom() - 1.0;
                                        let mid = rect.center().y;
                                        let stroke = Stroke::new(1.5, item.color);
                                        painter.line_segment(
                                            [pos2(center_x, top), pos2(center_x, mid - 1.0)],
                                            stroke,
                                        );
                                        painter.line_segment(
                                            [pos2(center_x, mid + 1.0), pos2(center_x, bottom)],
                                            stroke,
                                        );
                                    }
                                }
                            }
                            row.label(
                                RichText::new(&item.label)
                                    .size(11.0)
                                    .color(theme.palette.muted_foreground),
                            );
                            row.allocate_ui_with_layout(
                                row.available_size(),
                                Layout::right_to_left(Align::Center),
                                |right| {
                                    right.label(
                                        RichText::new(&item.value)
                                            .size(11.0)
                                            .font(FontId::monospace(11.0))
                                            .strong()
                                            .color(theme.palette.foreground),
                                    );
                                },
                            );
                        });
                    }
                });
            });
    }
}
