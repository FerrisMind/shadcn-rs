//! Public configuration types for the chart component.

use crate::iced_compat::Color;

use crate::theme::Theme;

/// Kind of mark a [`super::Chart`] draws.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ChartKind {
    /// Vertical (or horizontal) bars on a band scale.
    #[default]
    Bar,
    /// Filled area below a value line.
    Area,
    /// Value line without a fill.
    Line,
    /// Pie / donut of one series.
    Pie,
}

/// Series color slot of a [`super::Chart`].
///
/// The five slots map to the shadcn `--chart-1` … `--chart-5` tokens of the
/// active theme (and accent overlay); [`ChartColor::Custom`] escapes to a raw
/// iced color.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum ChartColor {
    /// First chart series color (`--chart-1`).
    #[default]
    Chart1,
    /// Second chart series color (`--chart-2`).
    Chart2,
    /// Third chart series color (`--chart-3`).
    Chart3,
    /// Fourth chart series color (`--chart-4`).
    Chart4,
    /// Fifth chart series color (`--chart-5`).
    Chart5,
    /// Explicit iced color.
    Custom(Color),
}

impl ChartColor {
    /// Palette slot for a zero-based series index, cycling every five.
    ///
    /// ```rust
    /// use iced_shadcn_v2::ChartColor;
    ///
    /// assert_eq!(ChartColor::from_index(0), ChartColor::Chart1);
    /// assert_eq!(ChartColor::from_index(6), ChartColor::Chart2);
    /// ```
    #[must_use]
    pub const fn from_index(index: usize) -> Self {
        match index % 5 {
            0 => Self::Chart1,
            1 => Self::Chart2,
            2 => Self::Chart3,
            3 => Self::Chart4,
            _ => Self::Chart5,
        }
    }

    /// Resolves the slot to an iced color of `theme`.
    #[must_use]
    pub fn resolve(self, theme: &Theme) -> Color {
        match self {
            Self::Chart1 => theme.palette.chart_1,
            Self::Chart2 => theme.palette.chart_2,
            Self::Chart3 => theme.palette.chart_3,
            Self::Chart4 => theme.palette.chart_4,
            Self::Chart5 => theme.palette.chart_5,
            Self::Custom(color) => color,
        }
    }
}

/// Which axis tick labels a [`super::Chart`] shows (layerchart `axis`).
///
/// The sides are positional: [`ChartAxis::X`] labels the bottom edge and
/// [`ChartAxis::Y`] the left edge, exactly like `axis="x"` / `axis="y"` in
/// shadcn-svelte (horizontal bar charts label categories with `y`).
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ChartAxis {
    /// No tick labels.
    None,
    /// Bottom tick labels only (`axis="x"`).
    X,
    /// Left tick labels only (`axis="y"`).
    Y,
    /// Both bottom and left tick labels (layerchart default).
    #[default]
    Both,
}

impl ChartAxis {
    pub(super) const fn shows_bottom(self) -> bool {
        matches!(self, Self::X | Self::Both)
    }

    pub(super) const fn shows_left(self) -> bool {
        matches!(self, Self::Y | Self::Both)
    }
}

/// Interpolation between samples of line/area charts (d3 `curve*`).
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ChartCurve {
    /// Straight segments between samples.
    #[default]
    Linear,
    /// Natural cubic spline (`curveNatural`).
    Natural,
    /// Midpoint step interpolation (`curveStep`).
    Step,
}

/// Shape of the per-series marker inside the chart tooltip
/// (shadcn `indicator`).
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ChartIndicator {
    /// Small rounded square in the series color.
    #[default]
    Dot,
    /// Full-height bar in the series color.
    Line,
    /// Dashed vertical line in the series color.
    Dashed,
}

/// One named data series of a [`super::Chart`].
///
/// ```rust
/// use iced_shadcn_v2::{ChartColor, ChartSeries};
///
/// let desktop = ChartSeries::new("Desktop", [186.0, 305.0, 237.0])
///     .color(ChartColor::Chart1);
/// assert_eq!(desktop.label(), "Desktop");
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct ChartSeries {
    pub(super) label: String,
    pub(super) values: Vec<f64>,
    pub(super) color: Option<ChartColor>,
    pub(super) point_colors: Vec<Option<ChartColor>>,
}

impl ChartSeries {
    /// Creates a series from a legend label and its sample values.
    ///
    /// Non-finite samples (`NaN`, `±inf`) are kept and skipped by the
    /// renderer, so a bad upstream division cannot poison the geometry.
    pub fn new(label: impl Into<String>, values: impl IntoIterator<Item = f64>) -> Self {
        Self {
            label: label.into(),
            values: values.into_iter().collect(),
            color: None,
            point_colors: Vec::new(),
        }
    }

    /// Sets the series color; unset series cycle `Chart1` … `Chart5` by
    /// position.
    #[must_use]
    pub fn color(mut self, color: ChartColor) -> Self {
        self.color = Some(color);
        self
    }

    /// Per-sample color overrides (`None` falls back to the series color).
    ///
    /// Mirrors the shadcn-svelte `c` accessor used by the negative and mixed
    /// bar demos, and colors individual pie slices.
    #[must_use]
    pub fn point_colors(mut self, colors: impl IntoIterator<Item = Option<ChartColor>>) -> Self {
        self.point_colors = colors.into_iter().collect();
        self
    }

    /// Legend label of the series.
    #[must_use]
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Sample values of the series.
    #[must_use]
    pub fn values(&self) -> &[f64] {
        &self.values
    }

    /// Color of sample `index`: point override, series color, or the cycled
    /// default for `series_index`.
    pub(super) fn resolved_point_color(&self, index: usize, series_index: usize) -> ChartColor {
        self.point_colors
            .get(index)
            .copied()
            .flatten()
            .unwrap_or_else(|| self.resolved_color(series_index))
    }

    /// Series color or the cycled default for `series_index`.
    pub(super) fn resolved_color(&self, series_index: usize) -> ChartColor {
        self.color.unwrap_or(ChartColor::from_index(series_index))
    }
}

/// Internal animation and hover state of a [`super::Chart`] canvas program.
#[derive(Debug, Default)]
pub struct ChartState {
    pub(super) start_time: Option<crate::iced_compat::time::Instant>,
    pub(super) progress: f32,
    pub(super) was_over: bool,
}
