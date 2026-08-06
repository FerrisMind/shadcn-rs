//! Canvas-rendered chart component.
//!
//! Port of the shadcn-svelte chart (layerchart `BarChart` / `AreaChart` /
//! `LineChart` / `PieChart` inside `Chart.Container` with `Chart.Tooltip`).
//! There is no layerchart in Rust, so marks, axes, grid, legend, hover
//! highlight, and the tooltip are drawn manually on an iced canvas; the
//! backend-agnostic layout math (scales, ticks, stacking, splines, slice
//! angles, hit testing) lives in [`shadcn_common::chart`].
//!
//! The public builder and configuration types live in `types` and this
//! module; canvas drawing and frame scheduling are isolated in `render`.

mod render;
mod types;

#[cfg(test)]
mod tests;

pub use types::{
    ChartAxis, ChartColor, ChartCurve, ChartIndicator, ChartKind, ChartSeries, ChartState,
};

use std::fmt;

use crate::iced_compat::widget::canvas;
use crate::iced_compat::{Element, Length};

use shadcn_common::{CHART_BAND_PADDING_FRACTION, CHART_TICK_COUNT};

use crate::theme::Theme;

/// Default fixed height of a chart in logical pixels.
///
/// Mirrors the `aspect-video` footprint of `Chart.Container` at the typical
/// shadcn card width; override with [`Chart::height`].
pub const CHART_DEFAULT_HEIGHT_PX: f32 = 300.0;

type LabelFormatter<'a> = Box<dyn Fn(&str) -> String + 'a>;
type ValueFormatter<'a> = Box<dyn Fn(f64) -> String + 'a>;

/// Builder-first chart styled directly with iced types.
///
/// Theme tokens (including `--chart-1` … `--chart-5`) come from
/// `shadcn-common` via [`Theme`]. Categories label the band/point axis and
/// every [`ChartSeries`] contributes one mark per category.
///
/// ```rust,no_run
/// use iced::Element;
/// use iced_shadcn_v2::{Chart, ChartAxis, ChartColor, ChartSeries, Theme};
///
/// #[derive(Debug, Clone)]
/// enum Message {}
///
/// fn view(theme: &Theme) -> Element<'_, Message> {
///     Chart::bar(theme)
///         .categories(["January", "February", "March"])
///         .series(ChartSeries::new("Desktop", [186.0, 305.0, 237.0]))
///         .axis(ChartAxis::X)
///         .category_format(|month| month.chars().take(3).collect())
///         .into()
/// }
/// ```
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct Chart<'a> {
    theme: &'a Theme,
    kind: ChartKind,
    categories: Vec<String>,
    series: Vec<ChartSeries>,
    stacked: bool,
    horizontal: bool,
    curve: ChartCurve,
    axis: ChartAxis,
    grid: bool,
    legend: bool,
    tooltip: bool,
    tooltip_indicator: ChartIndicator,
    tooltip_hide_label: bool,
    tooltip_hide_indicator: bool,
    highlight: bool,
    band_padding: f32,
    bar_radius: Option<f32>,
    donut_fraction: f32,
    tick_count: usize,
    animated: bool,
    width: Length,
    height: Length,
    category_format: Option<LabelFormatter<'a>>,
    tooltip_label_format: Option<LabelFormatter<'a>>,
    value_format: Option<ValueFormatter<'a>>,
}

impl<'a> Chart<'a> {
    /// Creates a chart of the given [`ChartKind`].
    ///
    /// `theme` is required because styling is derived from `shadcn-common`
    /// theme tokens instead of `iced::Theme`.
    pub fn new(kind: ChartKind, theme: &'a Theme) -> Self {
        Self {
            theme,
            kind,
            categories: Vec::new(),
            series: Vec::new(),
            stacked: false,
            horizontal: false,
            curve: ChartCurve::Linear,
            axis: ChartAxis::default(),
            grid: true,
            legend: false,
            tooltip: true,
            tooltip_indicator: ChartIndicator::default(),
            tooltip_hide_label: false,
            tooltip_hide_indicator: false,
            highlight: true,
            band_padding: CHART_BAND_PADDING_FRACTION,
            bar_radius: None,
            donut_fraction: 0.0,
            tick_count: CHART_TICK_COUNT,
            animated: true,
            width: Length::Fill,
            height: Length::Fixed(CHART_DEFAULT_HEIGHT_PX),
            category_format: None,
            tooltip_label_format: None,
            value_format: None,
        }
    }

    /// Creates a bar chart (layerchart `BarChart`).
    pub fn bar(theme: &'a Theme) -> Self {
        Self::new(ChartKind::Bar, theme)
    }

    /// Creates an area chart (layerchart `AreaChart`).
    pub fn area(theme: &'a Theme) -> Self {
        Self::new(ChartKind::Area, theme)
    }

    /// Creates a line chart (layerchart `LineChart`).
    pub fn line(theme: &'a Theme) -> Self {
        Self::new(ChartKind::Line, theme)
    }

    /// Creates a pie chart (layerchart `PieChart`) from one series.
    ///
    /// Categories name the slices; the first series supplies slice values,
    /// colored by its point colors (default: `Chart1` … `Chart5` cycle).
    pub fn pie(theme: &'a Theme) -> Self {
        Self::new(ChartKind::Pie, theme)
    }

    /// Sets the category labels of the band/point axis (pie slice names).
    pub fn categories(mut self, categories: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.categories = categories.into_iter().map(Into::into).collect();
        self
    }

    /// Appends one data series.
    pub fn series(mut self, series: ChartSeries) -> Self {
        self.series.push(series);
        self
    }

    /// Appends several data series at once.
    pub fn series_list(mut self, series: impl IntoIterator<Item = ChartSeries>) -> Self {
        self.series.extend(series);
        self
    }

    /// Stacks multiple series on top of each other
    /// (`seriesLayout="stack"`); unset series render grouped side by side.
    pub fn stacked(mut self, stacked: bool) -> Self {
        self.stacked = stacked;
        self
    }

    /// Lays bars out horizontally (`orientation="horizontal"`): categories
    /// run down the left edge and values grow to the right. Ignored by
    /// non-bar charts.
    pub fn horizontal(mut self, horizontal: bool) -> Self {
        self.horizontal = horizontal;
        self
    }

    /// Sets the interpolation of line/area charts.
    pub fn curve(mut self, curve: ChartCurve) -> Self {
        self.curve = curve;
        self
    }

    /// Selects which axis tick labels are shown.
    pub fn axis(mut self, axis: ChartAxis) -> Self {
        self.axis = axis;
        self
    }

    /// Shows or hides the value grid lines.
    pub fn grid(mut self, grid: bool) -> Self {
        self.grid = grid;
        self
    }

    /// Shows or hides the legend below the plot.
    pub fn legend(mut self, legend: bool) -> Self {
        self.legend = legend;
        self
    }

    /// Enables or disables the hover tooltip.
    pub fn tooltip(mut self, tooltip: bool) -> Self {
        self.tooltip = tooltip;
        self
    }

    /// Sets the per-series marker shape inside the tooltip.
    pub fn tooltip_indicator(mut self, indicator: ChartIndicator) -> Self {
        self.tooltip_indicator = indicator;
        self
    }

    /// Hides the category label row of the tooltip (shadcn `hideLabel`).
    pub fn tooltip_hide_label(mut self, hide: bool) -> Self {
        self.tooltip_hide_label = hide;
        self
    }

    /// Hides the per-series markers of the tooltip (shadcn `hideIndicator`).
    pub fn tooltip_hide_indicator(mut self, hide: bool) -> Self {
        self.tooltip_hide_indicator = hide;
        self
    }

    /// Enables or disables the hover highlight (band fill behind bars,
    /// sample points on line/area charts).
    pub fn highlight(mut self, highlight: bool) -> Self {
        self.highlight = highlight;
        self
    }

    /// Sets the band padding fraction of bar charts
    /// (`scaleBand().padding(…)`); clamped to `0.0..=0.9`.
    pub fn band_padding(mut self, fraction: f32) -> Self {
        self.band_padding = if fraction.is_finite() {
            fraction.clamp(0.0, 0.9)
        } else {
            CHART_BAND_PADDING_FRACTION
        };
        self
    }

    /// Sets the bar corner radius in pixels (clamped to at least 0).
    ///
    /// When unset, the radius comes from the active style pack
    /// ([`shadcn_common::chart_recipe`]): square packs (Lyra, Sera) draw
    /// square bars, the others round them per their radius scale.
    pub fn bar_radius(mut self, radius: f32) -> Self {
        self.bar_radius = Some(if radius.is_finite() {
            radius.max(0.0)
        } else {
            0.0
        });
        self
    }

    /// Turns a pie into a donut: inner radius as a fraction of the outer
    /// one, clamped to `0.0..=0.95`. Ignored by non-pie charts.
    pub fn donut(mut self, fraction: f32) -> Self {
        self.donut_fraction = if fraction.is_finite() {
            fraction.clamp(0.0, 0.95)
        } else {
            0.0
        };
        self
    }

    /// Sets the target number of value-axis ticks (clamped to at least 1).
    pub fn tick_count(mut self, count: usize) -> Self {
        self.tick_count = count.max(1);
        self
    }

    /// Enables the 500 ms entrance tween (`motion: "tween"`); enabled by
    /// default.
    pub fn animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }

    /// Sets a custom chart width.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets a custom chart height.
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Formats category tick labels (layerchart `xAxis.format`), e.g.
    /// shorten `"January"` to `"Jan"`. The tooltip keeps the full category.
    pub fn category_format(mut self, format: impl Fn(&str) -> String + 'a) -> Self {
        self.category_format = Some(Box::new(format));
        self
    }

    /// Formats the tooltip label row (shadcn `labelFormatter`).
    pub fn tooltip_label_format(mut self, format: impl Fn(&str) -> String + 'a) -> Self {
        self.tooltip_label_format = Some(Box::new(format));
        self
    }

    /// Formats tick and tooltip values; defaults to
    /// [`shadcn_common::chart_format_value`] (`1,234` style grouping).
    pub fn value_format(mut self, format: impl Fn(f64) -> String + 'a) -> Self {
        self.value_format = Some(Box::new(format));
        self
    }

    /// Kind of mark this chart draws.
    #[must_use]
    pub fn kind(&self) -> ChartKind {
        self.kind
    }

    /// Number of samples: the longest series length and category count.
    fn sample_count(&self) -> usize {
        self.series
            .iter()
            .map(|series| series.values.len())
            .chain(std::iter::once(self.categories.len()))
            .max()
            .unwrap_or(0)
    }

    /// Category label at `index`, or an empty string.
    fn category(&self, index: usize) -> &str {
        self.categories.get(index).map_or("", String::as_str)
    }
}

impl fmt::Debug for Chart<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Chart")
            .field("kind", &self.kind)
            .field("categories", &self.categories)
            .field("series", &self.series)
            .field("stacked", &self.stacked)
            .field("horizontal", &self.horizontal)
            .field("curve", &self.curve)
            .field("axis", &self.axis)
            .field("grid", &self.grid)
            .field("legend", &self.legend)
            .field("tooltip", &self.tooltip)
            .field("animated", &self.animated)
            .finish_non_exhaustive()
    }
}

/// Wraps a [`Chart`] program into a canvas widget with its configured size.
pub fn chart<'a, Message>(chart: Chart<'a>) -> canvas::Canvas<Chart<'a>, Message> {
    let width = chart.width;
    let height = chart.height;

    canvas::Canvas::new(chart).width(width).height(height)
}

impl<'a, Message: 'a> From<Chart<'a>> for Element<'a, Message> {
    fn from(config: Chart<'a>) -> Self {
        chart(config).into()
    }
}
