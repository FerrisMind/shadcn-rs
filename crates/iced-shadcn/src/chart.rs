use iced::border::Border;
use iced::widget::canvas::{Path, Stroke};
use iced::widget::{canvas, column, container, row, text};
use iced::{Alignment, Background, Color, Element, Length, Point, Rectangle, Size};
use std::sync::Arc;

use crate::theme::Theme;

pub type AxisFormatter = Arc<dyn Fn(f64, (f64, f64)) -> String + Send + Sync>;

#[derive(Clone, Copy, Debug)]
pub struct ChartGrid {
    pub x: bool,
    pub y: bool,
}

impl ChartGrid {
    pub fn new(x: bool, y: bool) -> Self {
        Self { x, y }
    }
}

#[derive(Clone)]
pub struct ChartProps {
    pub title: Option<String>,
    pub show_legend: bool,
    pub show_tooltip: bool,
    pub show_grid: ChartGrid,
    pub show_x: bool,
    pub show_y: bool,
    pub height: f32,
    pub margin: [f32; 2],
    pub x_axis_formatter: Option<AxisFormatter>,
    pub y_axis_formatter: Option<AxisFormatter>,
}

impl std::fmt::Debug for ChartProps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ChartProps")
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
    pub fn new() -> Self {
        Self {
            title: None,
            show_legend: true,
            show_tooltip: false,
            show_grid: ChartGrid::new(false, true),
            show_x: true,
            show_y: false,
            height: 220.0,
            margin: [12.0, 12.0],
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

    pub fn show_grid(mut self, show: ChartGrid) -> Self {
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

    pub fn margin(mut self, margin: [f32; 2]) -> Self {
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

impl Default for ChartProps {
    fn default() -> Self {
        Self::new()
    }
}

pub struct ChartResponse {
    pub hovered_point: Option<(f64, f64)>,
    pub pointer_pos: Option<Point>,
}

#[derive(Debug)]
pub struct ChartPlot {
    series: Vec<ChartSeries>,
}

impl ChartPlot {
    pub fn new() -> Self {
        Self { series: Vec::new() }
    }

    pub fn line(&mut self, line: LineChart) {
        self.series.push(ChartSeries::Line(line));
    }

    pub fn bar(&mut self, bar: BarChart) {
        self.series.push(ChartSeries::Bar(bar));
    }
}

impl Default for ChartPlot {
    fn default() -> Self {
        Self::new()
    }
}

pub fn chart<'a, Message: Clone + 'a>(
    props: ChartProps,
    theme: &'a Theme,
    add_plot: impl FnOnce(&mut ChartPlot),
) -> Element<'a, Message> {
    let mut plot = ChartPlot::new();
    add_plot(&mut plot);

    let mut content = column![];
    if let Some(title) = props.title.clone() {
        content = content.push(text(title).size(16));
    }

    if props.show_legend {
        let legend = chart_legend(&plot, theme);
        if let Some(legend) = legend {
            content = content.push(legend);
        }
    }

    let program = ChartProgram {
        props: props.clone(),
        plot,
        theme: theme.clone(),
    };

    let canvas = canvas::Canvas::new(program)
        .width(Length::Fill)
        .height(Length::Fixed(props.height));

    content.push(canvas).spacing(12).into()
}

#[derive(Clone, Debug)]
pub struct LineChart {
    points: Vec<[f64; 2]>,
    label: Option<String>,
    color: Option<Color>,
    stroke_width: f32,
}

impl LineChart {
    pub fn new(points: Vec<[f64; 2]>) -> Self {
        Self {
            points,
            label: None,
            color: None,
            stroke_width: 2.0,
        }
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    pub fn stroke_width(mut self, width: f32) -> Self {
        self.stroke_width = width;
        self
    }

    pub fn show(&self, plot: &mut ChartPlot) {
        plot.line(self.clone());
    }
}

#[derive(Clone, Debug)]
pub struct BarChart {
    bars: Vec<(f64, f64)>,
    label: Option<String>,
    color: Option<Color>,
    bar_width: Option<f64>,
}

impl BarChart {
    pub fn new(values: Vec<(f64, f64)>) -> Self {
        Self {
            bars: values,
            label: None,
            color: None,
            bar_width: Some(0.6),
        }
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    pub fn bar_width(mut self, width: f64) -> Self {
        self.bar_width = Some(width);
        self
    }

    pub fn show(&self, plot: &mut ChartPlot) {
        plot.bar(self.clone());
    }
}

#[derive(Clone, Debug)]
enum ChartSeries {
    Line(LineChart),
    Bar(BarChart),
}

#[derive(Debug)]
struct ChartProgram {
    props: ChartProps,
    plot: ChartPlot,
    theme: Theme,
}

impl<Message> canvas::Program<Message> for ChartProgram {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &iced::Theme,
        bounds: Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());
        let background = self.theme.palette.card;
        let border = self.theme.palette.border;

        frame.fill_rectangle(Point::ORIGIN, bounds.size(), background);
        frame.stroke_rectangle(
            Point::ORIGIN,
            bounds.size(),
            Stroke::default().with_color(border).with_width(1.0),
        );

        let margin_x = self.props.margin[0].max(0.0);
        let margin_y = self.props.margin[1].max(0.0);
        let plot_bounds = Rectangle::new(
            Point::new(margin_x, margin_y),
            Size::new(
                (bounds.width - margin_x * 2.0).max(1.0),
                (bounds.height - margin_y * 2.0).max(1.0),
            ),
        );

        let (min_x, max_x, min_y, max_y) = data_bounds(&self.plot.series);
        let scale = ChartScale {
            min_x,
            range_x: (max_x - min_x).max(1.0),
            min_y,
            range_y: (max_y - min_y).max(1.0),
        };

        if self.props.show_grid.x || self.props.show_grid.y {
            let grid_color = self.theme.palette.muted;
            let steps = 4;
            for step in 0..=steps {
                let t = step as f32 / steps as f32;
                if self.props.show_grid.x {
                    let x = plot_bounds.x + plot_bounds.width * t;
                    let path = Path::line(
                        Point::new(x, plot_bounds.y),
                        Point::new(x, plot_bounds.y + plot_bounds.height),
                    );
                    frame.stroke(&path, Stroke::default().with_color(grid_color));
                }
                if self.props.show_grid.y {
                    let y = plot_bounds.y + plot_bounds.height * t;
                    let path = Path::line(
                        Point::new(plot_bounds.x, y),
                        Point::new(plot_bounds.x + plot_bounds.width, y),
                    );
                    frame.stroke(&path, Stroke::default().with_color(grid_color));
                }
            }
        }

        for series in &self.plot.series {
            match series {
                ChartSeries::Line(line) => {
                    draw_line(
                        &mut frame,
                        plot_bounds,
                        self.theme.palette.chart_1,
                        line,
                        scale,
                    );
                }
                ChartSeries::Bar(bar) => {
                    draw_bars(
                        &mut frame,
                        plot_bounds,
                        self.theme.palette.chart_2,
                        bar,
                        scale,
                    );
                }
            }
        }

        vec![frame.into_geometry()]
    }
}

fn chart_legend<'a, Message: Clone + 'a>(
    plot: &ChartPlot,
    theme: &'a Theme,
) -> Option<Element<'a, Message>> {
    let mut entries: Vec<(String, Color)> = Vec::new();
    for series in &plot.series {
        match series {
            ChartSeries::Line(line) => {
                if let Some(label) = &line.label {
                    entries.push((label.clone(), line.color.unwrap_or(theme.palette.chart_1)));
                }
            }
            ChartSeries::Bar(bar) => {
                if let Some(label) = &bar.label {
                    entries.push((label.clone(), bar.color.unwrap_or(theme.palette.chart_2)));
                }
            }
        }
    }

    if entries.is_empty() {
        None
    } else {
        let items = entries
            .into_iter()
            .map(|(label, color)| legend_item(label, color, theme))
            .collect::<Vec<_>>();
        Some(row(items).spacing(8).align_y(Alignment::Center).into())
    }
}

fn legend_item<'a, Message: Clone + 'a>(
    label: String,
    color: Color,
    theme: &'a Theme,
) -> Element<'a, Message> {
    let swatch = container(text(""))
        .width(Length::Fixed(12.0))
        .height(Length::Fixed(12.0))
        .style(move |_t| iced::widget::container::Style {
            background: Some(Background::Color(color)),
            border: Border {
                radius: theme.radius.sm.into(),
                width: 1.0,
                color,
            },
            ..Default::default()
        });
    row![swatch, text(label).size(12)]
        .spacing(6)
        .align_y(Alignment::Center)
        .into()
}

fn data_bounds(series: &[ChartSeries]) -> (f64, f64, f64, f64) {
    let mut min_x = f64::MAX;
    let mut max_x = f64::MIN;
    let mut min_y = f64::MAX;
    let mut max_y = f64::MIN;

    for item in series {
        match item {
            ChartSeries::Line(line) => {
                for [x, y] in &line.points {
                    min_x = min_x.min(*x);
                    max_x = max_x.max(*x);
                    min_y = min_y.min(*y);
                    max_y = max_y.max(*y);
                }
            }
            ChartSeries::Bar(bar) => {
                for (x, y) in &bar.bars {
                    min_x = min_x.min(*x);
                    max_x = max_x.max(*x);
                    min_y = min_y.min(*y);
                    max_y = max_y.max(*y);
                }
            }
        }
    }

    if min_x == f64::MAX {
        (0.0, 1.0, 0.0, 1.0)
    } else {
        (min_x, max_x, min_y, max_y)
    }
}

fn map_point(x: f64, y: f64, bounds: Rectangle, scale: ChartScale) -> Point {
    let x_norm = ((x - scale.min_x) / scale.range_x).clamp(0.0, 1.0) as f32;
    let y_norm = ((y - scale.min_y) / scale.range_y).clamp(0.0, 1.0) as f32;
    Point::new(
        bounds.x + bounds.width * x_norm,
        bounds.y + bounds.height * (1.0 - y_norm),
    )
}

#[derive(Clone, Copy, Debug)]
struct ChartScale {
    min_x: f64,
    range_x: f64,
    min_y: f64,
    range_y: f64,
}

fn draw_line(
    frame: &mut canvas::Frame,
    bounds: Rectangle,
    fallback_color: Color,
    line: &LineChart,
    scale: ChartScale,
) {
    if line.points.len() < 2 {
        return;
    }
    let path = Path::new(|builder| {
        for (index, [x, y]) in line.points.iter().enumerate() {
            let point = map_point(*x, *y, bounds, scale);
            if index == 0 {
                builder.move_to(point);
            } else {
                builder.line_to(point);
            }
        }
    });
    frame.stroke(
        &path,
        Stroke::default()
            .with_color(line.color.unwrap_or(fallback_color))
            .with_width(line.stroke_width),
    );
}

fn draw_bars(
    frame: &mut canvas::Frame,
    bounds: Rectangle,
    fallback_color: Color,
    bar: &BarChart,
    scale: ChartScale,
) {
    let count = bar.bars.len().max(1) as f32;
    let default_width = bounds.width / count * 0.6;
    let width_px = bar
        .bar_width
        .map(|w| (w / scale.range_x) as f32 * bounds.width)
        .unwrap_or(default_width)
        .max(2.0);

    for (x, y) in &bar.bars {
        let base = map_point(*x, scale.min_y, bounds, scale);
        let top = map_point(*x, *y, bounds, scale);
        let height = (base.y - top.y).max(0.0);
        let rect = Rectangle::new(
            Point::new(top.x - width_px / 2.0, top.y),
            Size::new(width_px, height),
        );
        frame.fill_rectangle(
            rect.position(),
            rect.size(),
            bar.color.unwrap_or(fallback_color),
        );
    }
}
