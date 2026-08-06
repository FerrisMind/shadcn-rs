//! Canvas rendering, hover handling, and frame scheduling for [`super::Chart`].

use std::f32::consts::{PI, TAU};
use std::time::Duration;

use crate::iced_compat::alignment::Vertical;
use crate::iced_compat::widget::canvas;
use crate::iced_compat::widget::canvas::{LineCap, Path, Stroke, Text};
use crate::iced_compat::{
    Color, Point, Radians, Rectangle, Renderer, Size, border, font, mouse, window,
};

use shadcn_common::{
    CHART_AREA_FILL_OPACITY, CHART_GROUP_PADDING_FRACTION, CHART_HIGHLIGHT_POINT_RADIUS_PX,
    CHART_MOTION_MS, CHART_TOOLTIP_MIN_WIDTH_PX, ChartRecipe, chart_band_slots, chart_format_value,
    chart_group_slots, chart_linear_fraction, chart_natural_curve, chart_nearest_center,
    chart_nice_domain, chart_nice_ticks, chart_pie_hit, chart_pie_slices, chart_recipe,
    chart_stack_spans, chart_value_extent,
};

use super::Chart;
use super::types::{ChartCurve, ChartIndicator, ChartKind, ChartState};
use crate::fonts::iced_font;
use crate::recipes::component_radius_px;

const CHART_FRAME_INTERVAL: Duration = Duration::from_millis(16);
/// Gap between the plot edge and axis tick labels.
const AXIS_GAP_PX: f32 = 8.0;
const PLOT_PADDING_PX: f32 = 4.0;
const LEGEND_HEIGHT_PX: f32 = 28.0;
/// Legend swatch footprint (`size-2.5 rounded-[2px]`).
const LEGEND_SWATCH_PX: f32 = 10.0;
const LEGEND_SWATCH_GAP_PX: f32 = 6.0;
const LEGEND_ITEM_GAP_PX: f32 = 16.0;
/// Row gap inside the tooltip (`gap-1.5`).
const TOOLTIP_GAP: f32 = 6.0;
const TOOLTIP_OFFSET: f32 = 12.0;
/// Tooltip indicator footprint (`size-2.5 rounded-[2px]`).
const INDICATOR_PX: f32 = 10.0;

impl<Message> canvas::Program<Message> for Chart<'_> {
    type State = ChartState;

    fn update(
        &self,
        state: &mut Self::State,
        event: &canvas::Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Option<canvas::Action<Message>> {
        match event {
            canvas::Event::Window(window::Event::RedrawRequested(now)) => {
                if !self.animated || state.progress >= 1.0 {
                    return None;
                }

                if state.start_time.is_none() {
                    state.start_time = Some(*now);
                }

                if let Some(start) = state.start_time {
                    let elapsed = now.saturating_duration_since(start);
                    state.progress = (elapsed.as_secs_f32() * 1000.0 / CHART_MOTION_MS).min(1.0);
                }

                if state.progress < 1.0 {
                    return Some(canvas::Action::request_redraw_at(
                        *now + CHART_FRAME_INTERVAL,
                    ));
                }

                Some(canvas::Action::request_redraw())
            }
            canvas::Event::Mouse(mouse::Event::CursorMoved { .. } | mouse::Event::CursorLeft) => {
                if !self.tooltip && !self.highlight {
                    return None;
                }

                let over = cursor.is_over(bounds);

                if over || state.was_over {
                    state.was_over = over;
                    return Some(canvas::Action::request_redraw());
                }

                None
            }
            _ => None,
        }
    }

    fn draw(
        &self,
        state: &Self::State,
        renderer: &Renderer,
        _theme: &crate::iced_compat::Theme,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());
        let progress = if self.animated {
            ease_in_out_cubic(state.progress)
        } else {
            1.0
        };
        let cursor_position = cursor.position_in(bounds);

        match self.kind {
            ChartKind::Pie => draw_pie(self, &mut frame, bounds.size(), cursor_position, progress),
            _ => draw_cartesian(self, &mut frame, bounds.size(), cursor_position, progress),
        }

        vec![frame.into_geometry()]
    }
}

/// `cubicInOut` easing from the svelte demos.
fn ease_in_out_cubic(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);

    if t < 0.5 {
        4.0 * t * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
    }
}

// ─── Cartesian charts ────────────────────────────────────────────────────────

/// Resolved plot geometry of a cartesian chart.
struct PlotLayout {
    plot: Rectangle,
    domain: (f64, f64),
    ticks: Vec<f64>,
    /// Category centers along the band/point axis, in widget coordinates.
    centers: Vec<f32>,
    band_starts: Vec<f32>,
    bandwidth: f32,
    count: usize,
}

impl PlotLayout {
    /// Pixel coordinate of `value` along the value axis.
    fn value_px(&self, chart: &Chart<'_>, value: f64) -> f32 {
        let fraction = chart_linear_fraction(value, self.domain.0, self.domain.1);

        if chart.is_horizontal() {
            self.plot.x + self.plot.width * fraction
        } else {
            self.plot.y + self.plot.height * (1.0 - fraction)
        }
    }
}

impl Chart<'_> {
    fn is_horizontal(&self) -> bool {
        self.kind == ChartKind::Bar && self.horizontal
    }

    pub(super) fn format_value(&self, value: f64) -> String {
        match &self.value_format {
            Some(format) => format(value),
            None => chart_format_value(value),
        }
    }

    pub(super) fn format_category(&self, label: &str) -> String {
        match &self.category_format {
            Some(format) => format(label),
            None => label.to_owned(),
        }
    }

    pub(super) fn format_tooltip_label(&self, label: &str) -> String {
        match &self.tooltip_label_format {
            Some(format) => format(label),
            None => label.to_owned(),
        }
    }

    /// Style-pack tokens for this chart.
    fn recipe(&self) -> ChartRecipe {
        chart_recipe(self.theme.style_id())
    }

    /// Tick/legend/tooltip text size from the pack (`text-xs`).
    fn text_size(&self) -> f32 {
        self.recipe().typography.size_px
    }

    /// Line height paired with [`Self::text_size`].
    fn text_line(&self) -> f32 {
        self.recipe().typography.line_height_px
    }

    /// Bar corner radius: builder override or the pack default.
    pub(super) fn resolved_bar_radius(&self) -> f32 {
        match self.bar_radius {
            Some(radius) => radius,
            None => component_radius_px(self.theme, self.recipe().bar_radius),
        }
    }
}

fn compute_layout(chart: &Chart<'_>, size: Size) -> Option<PlotLayout> {
    let count = chart.sample_count();

    if count == 0 || chart.series.is_empty() {
        return None;
    }

    let slices: Vec<&[f64]> = chart.series.iter().map(|series| series.values()).collect();
    let stacked = chart.stacked && chart.series.len() > 1;
    let (min, max) = chart_value_extent(&slices, stacked).unwrap_or((0.0, 1.0));
    let (min, max) = if (max - min).abs() <= f64::EPSILON {
        (min, min + 1.0)
    } else {
        (min, max)
    };
    let domain = chart_nice_domain(min, max, chart.tick_count);
    let ticks = chart_nice_ticks(domain.0, domain.1, chart.tick_count);

    let horizontal = chart.is_horizontal();
    let show_bottom = chart.axis.shows_bottom();
    let show_left = chart.axis.shows_left();
    let text_size = chart.text_size();

    let left_labels_width = if show_left {
        let width: f32 = if horizontal {
            (0..count)
                .map(|index| {
                    estimate_text_width(
                        &chart.format_category(chart.category(index)),
                        text_size,
                        false,
                    )
                })
                .fold(0.0, f32::max)
        } else {
            ticks
                .iter()
                .map(|tick| estimate_text_width(&chart.format_value(*tick), text_size, false))
                .fold(0.0, f32::max)
        };

        width + AXIS_GAP_PX
    } else {
        0.0
    };

    let left = PLOT_PADDING_PX + left_labels_width;
    let right = PLOT_PADDING_PX;
    let top = PLOT_PADDING_PX + 2.0;
    let legend = if chart.legend { LEGEND_HEIGHT_PX } else { 0.0 };
    let bottom = legend
        + if show_bottom {
            chart.text_line() + AXIS_GAP_PX
        } else {
            PLOT_PADDING_PX
        };

    let plot = Rectangle {
        x: left,
        y: top,
        width: (size.width - left - right).max(0.0),
        height: (size.height - top - bottom).max(0.0),
    };

    if plot.width <= 0.0 || plot.height <= 0.0 {
        return None;
    }

    let band_range = if horizontal { plot.height } else { plot.width };
    let band_origin = if horizontal { plot.y } else { plot.x };

    let (band_starts, bandwidth, centers) = if chart.kind == ChartKind::Bar {
        let (starts, width) = chart_band_slots(band_range, count, chart.band_padding);
        let starts: Vec<f32> = starts.iter().map(|start| band_origin + start).collect();
        let centers = starts.iter().map(|start| start + width / 2.0).collect();

        (starts, width, centers)
    } else {
        let centers: Vec<f32> = (0..count)
            .map(|index| {
                let fraction = if count > 1 {
                    index as f32 / (count - 1) as f32
                } else {
                    0.5
                };

                band_origin + band_range * fraction
            })
            .collect();

        (Vec::new(), 0.0, centers)
    };

    Some(PlotLayout {
        plot,
        domain,
        ticks,
        centers,
        band_starts,
        bandwidth,
        count,
    })
}

fn draw_cartesian(
    chart: &Chart<'_>,
    frame: &mut canvas::Frame<Renderer>,
    size: Size,
    cursor: Option<Point>,
    progress: f32,
) {
    let Some(layout) = compute_layout(chart, size) else {
        return;
    };

    let hovered = cursor
        .filter(|position| layout.plot.contains(*position))
        .and_then(|position| {
            let along = if chart.is_horizontal() {
                position.y
            } else {
                position.x
            };

            chart_nearest_center(&layout.centers, along)
        });

    if chart.grid {
        draw_grid(chart, frame, &layout);
    }

    draw_axis_labels(chart, frame, &layout);

    if chart.kind == ChartKind::Bar
        && chart.highlight
        && let Some(index) = hovered
    {
        draw_band_highlight(chart, frame, &layout, index);
    }

    match chart.kind {
        ChartKind::Bar => draw_bars(chart, frame, &layout, progress),
        ChartKind::Area | ChartKind::Line => {
            draw_series_curves(chart, frame, &layout, progress, hovered)
        }
        ChartKind::Pie => {}
    }

    if chart.legend {
        draw_legend(chart, frame, size, false);
    }

    if chart.tooltip
        && let (Some(index), Some(position)) = (hovered, cursor)
    {
        draw_cartesian_tooltip(chart, frame, size, position, index);
    }
}

fn draw_grid(chart: &Chart<'_>, frame: &mut canvas::Frame<Renderer>, layout: &PlotLayout) {
    let color = scale_alpha(chart.theme.palette.border, 0.5);
    let stroke = Stroke::default().with_width(1.0).with_color(color);

    for tick in &layout.ticks {
        let position = layout.value_px(chart, *tick);

        let line = if chart.is_horizontal() {
            Path::line(
                Point::new(position, layout.plot.y),
                Point::new(position, layout.plot.y + layout.plot.height),
            )
        } else {
            Path::line(
                Point::new(layout.plot.x, position),
                Point::new(layout.plot.x + layout.plot.width, position),
            )
        };

        frame.stroke(&line, stroke);
    }
}

fn draw_axis_labels(chart: &Chart<'_>, frame: &mut canvas::Frame<Renderer>, layout: &PlotLayout) {
    let color = chart.theme.palette.muted_foreground;
    let sans = iced_font(chart.theme.font_pack().sans);
    let horizontal = chart.is_horizontal();

    if chart.axis.shows_bottom() {
        let baseline = layout.plot.y + layout.plot.height + AXIS_GAP_PX + chart.text_line() / 2.0;

        if horizontal {
            for tick in &layout.ticks {
                fill_label(
                    frame,
                    &chart.format_value(*tick),
                    Point::new(layout.value_px(chart, *tick), baseline),
                    color,
                    sans,
                    chart.text_size(),
                );
            }
        } else {
            for (index, center) in layout.centers.iter().enumerate() {
                fill_label(
                    frame,
                    &chart.format_category(chart.category(index)),
                    Point::new(*center, baseline),
                    color,
                    sans,
                    chart.text_size(),
                );
            }
        }
    }

    if chart.axis.shows_left() {
        let right_edge = layout.plot.x - AXIS_GAP_PX;

        if horizontal {
            for (index, center) in layout.centers.iter().enumerate() {
                fill_label_right(
                    frame,
                    &chart.format_category(chart.category(index)),
                    Point::new(right_edge, *center),
                    color,
                    sans,
                    chart.text_size(),
                );
            }
        } else {
            for tick in &layout.ticks {
                fill_label_right(
                    frame,
                    &chart.format_value(*tick),
                    Point::new(right_edge, layout.value_px(chart, *tick)),
                    color,
                    sans,
                    chart.text_size(),
                );
            }
        }
    }
}

fn draw_band_highlight(
    chart: &Chart<'_>,
    frame: &mut canvas::Frame<Renderer>,
    layout: &PlotLayout,
    index: usize,
) {
    let Some(start) = layout.band_starts.get(index) else {
        return;
    };
    let step = if layout.band_starts.len() > 1 {
        layout.band_starts[1] - layout.band_starts[0]
    } else if chart.is_horizontal() {
        layout.plot.height
    } else {
        layout.plot.width
    };
    let inset = (step - layout.bandwidth) / 2.0;
    let color = chart.theme.palette.muted;

    if chart.is_horizontal() {
        frame.fill_rectangle(
            Point::new(layout.plot.x, start - inset),
            Size::new(layout.plot.width, step),
            color,
        );
    } else {
        frame.fill_rectangle(
            Point::new(start - inset, layout.plot.y),
            Size::new(step, layout.plot.height),
            color,
        );
    }
}

/// Value spans per series and sample: stacked or independent from zero.
fn value_spans(chart: &Chart<'_>) -> Vec<Vec<(f64, f64)>> {
    let slices: Vec<&[f64]> = chart.series.iter().map(|series| series.values()).collect();

    if chart.stacked && chart.series.len() > 1 {
        chart_stack_spans(&slices)
    } else {
        let count = chart.sample_count();

        slices
            .iter()
            .map(|values| {
                (0..count)
                    .map(|index| {
                        let value = values
                            .get(index)
                            .copied()
                            .filter(|value| value.is_finite())
                            .unwrap_or(0.0);

                        (value.min(0.0), value.max(0.0))
                    })
                    .collect()
            })
            .collect()
    }
}

fn draw_bars(
    chart: &Chart<'_>,
    frame: &mut canvas::Frame<Renderer>,
    layout: &PlotLayout,
    progress: f32,
) {
    let spans = value_spans(chart);
    let stacked = chart.stacked && chart.series.len() > 1;
    let grouped = !stacked && chart.series.len() > 1;
    let (offsets, width) = if grouped {
        chart_group_slots(
            layout.bandwidth,
            chart.series.len(),
            CHART_GROUP_PADDING_FRACTION,
        )
    } else {
        (vec![0.0], layout.bandwidth)
    };
    let horizontal = chart.is_horizontal();

    // Outermost stack ends, per sample, for corner rounding.
    let mut positive_top = vec![0.0_f64; layout.count];
    let mut negative_bottom = vec![0.0_f64; layout.count];

    for series_spans in &spans {
        for (sample, (start, end)) in series_spans.iter().enumerate() {
            positive_top[sample] = positive_top[sample].max(*end);
            negative_bottom[sample] = negative_bottom[sample].min(*start);
        }
    }

    for (series_index, series) in chart.series.iter().enumerate() {
        let offset = if grouped {
            offsets.get(series_index).copied().unwrap_or(0.0)
        } else {
            0.0
        };

        for sample in 0..layout.count {
            let (start, end) = spans[series_index][sample];

            if end - start <= 0.0 {
                continue;
            }

            let (start, end) = (start * f64::from(progress), end * f64::from(progress));
            let color = series
                .resolved_point_color(sample, series_index)
                .resolve(chart.theme);
            let band_start = layout.band_starts[sample] + offset;

            let start_px = layout.value_px(chart, start);
            let end_px = layout.value_px(chart, end);

            let rect = if horizontal {
                Rectangle {
                    x: start_px.min(end_px),
                    y: band_start,
                    width: (end_px - start_px).abs(),
                    height: width,
                }
            } else {
                Rectangle {
                    x: band_start,
                    y: start_px.min(end_px),
                    width,
                    height: (end_px - start_px).abs(),
                }
            };

            if rect.width <= 0.0 || rect.height <= 0.0 {
                continue;
            }

            let radius = chart
                .resolved_bar_radius()
                .min(rect.width / 2.0)
                .min(rect.height / 2.0);
            let radius = bar_corner_radius(
                chart,
                stacked,
                radius,
                (spans[series_index][sample].0, spans[series_index][sample].1),
                positive_top[sample],
                negative_bottom[sample],
            );

            let path = Path::rounded_rectangle(
                Point::new(rect.x, rect.y),
                Size::new(rect.width, rect.height),
                radius,
            );

            frame.fill(&path, color);
        }
    }
}

/// Corner radii of one bar segment. Non-stacked bars round every corner
/// (`rounded: "all"`); stacked segments round only the outer stack ends.
fn bar_corner_radius(
    chart: &Chart<'_>,
    stacked: bool,
    radius: f32,
    span: (f64, f64),
    positive_top: f64,
    negative_bottom: f64,
) -> border::Radius {
    if !stacked {
        return radius.into();
    }

    let epsilon = f64::EPSILON;
    let at_positive_top = span.1 > span.0 && (span.1 - positive_top).abs() <= epsilon;
    let at_negative_bottom = span.1 > span.0 && (span.0 - negative_bottom).abs() <= epsilon;
    let horizontal = chart.is_horizontal();

    let mut corners = border::Radius::from(0.0);

    if at_positive_top && span.1 > 0.0 {
        if horizontal {
            corners.top_right = radius;
            corners.bottom_right = radius;
        } else {
            corners.top_left = radius;
            corners.top_right = radius;
        }
    }

    if at_negative_bottom && span.0 < 0.0 {
        if horizontal {
            corners.top_left = radius;
            corners.bottom_left = radius;
        } else {
            corners.bottom_left = radius;
            corners.bottom_right = radius;
        }
    }

    corners
}

fn draw_series_curves(
    chart: &Chart<'_>,
    frame: &mut canvas::Frame<Renderer>,
    layout: &PlotLayout,
    progress: f32,
    hovered: Option<usize>,
) {
    let spans = value_spans(chart);
    let alpha = progress.clamp(0.0, 1.0);

    for (series_index, series) in chart.series.iter().enumerate() {
        let color = series.resolved_color(series_index).resolve(chart.theme);
        let upper: Vec<(f32, f32)> = (0..layout.count)
            .map(|sample| {
                (
                    layout.centers[sample],
                    layout.value_px(
                        chart,
                        spans[series_index][sample]
                            .1
                            .max(spans[series_index][sample].0),
                    ),
                )
            })
            .collect();

        if upper.len() < 2 {
            continue;
        }

        if chart.kind == ChartKind::Area {
            let lower: Vec<(f32, f32)> = (0..layout.count)
                .map(|sample| {
                    let base = if chart.stacked && chart.series.len() > 1 {
                        spans[series_index][sample].0
                    } else {
                        0.0
                    };

                    (layout.centers[sample], layout.value_px(chart, base))
                })
                .collect();

            let area = area_path(&upper, &lower, chart.curve);
            frame.fill(&area, scale_alpha(color, CHART_AREA_FILL_OPACITY * alpha));

            let line = curve_path(&upper, chart.curve);
            frame.stroke(
                &line,
                Stroke::default()
                    .with_width(1.0)
                    .with_color(scale_alpha(color, alpha))
                    .with_line_cap(LineCap::Round),
            );
        } else {
            let line = curve_path(&upper, chart.curve);
            frame.stroke(
                &line,
                Stroke::default()
                    .with_width(2.0)
                    .with_color(scale_alpha(color, alpha))
                    .with_line_cap(LineCap::Round),
            );
        }

        if chart.highlight
            && let Some(index) = hovered
            && let Some((x, y)) = upper.get(index)
        {
            let point = Path::circle(Point::new(*x, *y), CHART_HIGHLIGHT_POINT_RADIUS_PX);
            frame.fill(&point, scale_alpha(color, alpha));
        }
    }
}

/// Open path through `points` with the configured interpolation.
fn curve_path(points: &[(f32, f32)], curve: ChartCurve) -> Path {
    Path::new(|builder| {
        add_curve(builder, points, curve);
    })
}

fn add_curve(builder: &mut canvas::path::Builder, points: &[(f32, f32)], curve: ChartCurve) {
    let Some(first) = points.first() else {
        return;
    };

    builder.move_to(Point::new(first.0, first.1));
    continue_curve(builder, points, curve);
}

/// Adds `points[1..]` to a path whose pen already sits at `points[0]`.
fn continue_curve(builder: &mut canvas::path::Builder, points: &[(f32, f32)], curve: ChartCurve) {
    match curve {
        ChartCurve::Linear => {
            for point in &points[1..] {
                builder.line_to(Point::new(point.0, point.1));
            }
        }
        ChartCurve::Natural => {
            for segment in chart_natural_curve(points) {
                builder.bezier_curve_to(
                    Point::new(segment.control_1.0, segment.control_1.1),
                    Point::new(segment.control_2.0, segment.control_2.1),
                    Point::new(segment.to.0, segment.to.1),
                );
            }
        }
        ChartCurve::Step => {
            for pair in points.windows(2) {
                let mid = (pair[0].0 + pair[1].0) / 2.0;

                builder.line_to(Point::new(mid, pair[0].1));
                builder.line_to(Point::new(mid, pair[1].1));
                builder.line_to(Point::new(pair[1].0, pair[1].1));
            }
        }
    }
}

/// Closed path between an upper and a lower boundary (an area band).
fn area_path(upper: &[(f32, f32)], lower: &[(f32, f32)], curve: ChartCurve) -> Path {
    Path::new(|builder| {
        add_curve(builder, upper, curve);

        let reversed: Vec<(f32, f32)> = lower.iter().rev().copied().collect();

        if let Some(first) = reversed.first() {
            builder.line_to(Point::new(first.0, first.1));
            continue_curve(builder, &reversed, curve);
        }

        builder.close();
    })
}

// ─── Pie charts ──────────────────────────────────────────────────────────────

fn draw_pie(
    chart: &Chart<'_>,
    frame: &mut canvas::Frame<Renderer>,
    size: Size,
    cursor: Option<Point>,
    progress: f32,
) {
    let Some(series) = chart.series.first() else {
        return;
    };

    let values = series.values();

    if values.is_empty() {
        return;
    }

    let slices = chart_pie_slices(values);
    let legend = if chart.legend { LEGEND_HEIGHT_PX } else { 0.0 };
    let center = Point::new(size.width / 2.0, (size.height - legend) / 2.0);
    let outer = ((size.width.min(size.height - legend)) / 2.0 - 8.0).max(0.0);
    let inner = outer * chart.donut_fraction;

    if outer <= 0.0 {
        return;
    }

    for (index, slice) in slices.iter().enumerate() {
        let sweep = slice.sweep_fraction() * progress;

        if sweep <= 0.0 {
            continue;
        }

        let start = Radians(-PI / 2.0 + slice.start_fraction * progress * TAU);
        let end = Radians(start.0 + sweep * TAU);
        let color = series
            .resolved_point_color(index, index)
            .resolve(chart.theme);
        let path = pie_slice_path(center, outer, inner, start, end);

        frame.fill(&path, color);
    }

    if chart.legend {
        draw_legend(chart, frame, size, true);
    }

    if chart.tooltip
        && let Some(position) = cursor
        && let Some(index) = chart_pie_hit(
            position.x - center.x,
            position.y - center.y,
            &slices,
            outer,
            inner,
        )
    {
        let color = series
            .resolved_point_color(index, index)
            .resolve(chart.theme);
        let value = values.get(index).copied().unwrap_or(f64::NAN);
        let rows = vec![TooltipRow {
            color,
            name: chart.category(index).to_owned(),
            value: chart.format_value(value),
        }];

        draw_tooltip(chart, frame, size, position, None, &rows);
    }
}

fn pie_slice_path(center: Point, outer: f32, inner: f32, start: Radians, end: Radians) -> Path {
    Path::new(|builder| {
        let outer_start = Point::new(
            center.x + outer * start.0.cos(),
            center.y + outer * start.0.sin(),
        );

        if inner > 0.0 {
            let inner_end = Point::new(
                center.x + inner * end.0.cos(),
                center.y + inner * end.0.sin(),
            );

            builder.move_to(outer_start);
            builder.arc(canvas::path::Arc {
                center,
                radius: outer,
                start_angle: start,
                end_angle: end,
            });
            builder.line_to(inner_end);
            builder.arc(canvas::path::Arc {
                center,
                radius: inner,
                start_angle: end,
                end_angle: start,
            });
        } else {
            builder.move_to(center);
            builder.line_to(outer_start);
            builder.arc(canvas::path::Arc {
                center,
                radius: outer,
                start_angle: start,
                end_angle: end,
            });
        }

        builder.close();
    })
}

// ─── Legend ──────────────────────────────────────────────────────────────────

fn legend_entries(chart: &Chart<'_>, pie: bool) -> Vec<(Color, String)> {
    if pie {
        let Some(series) = chart.series.first() else {
            return Vec::new();
        };

        (0..series.values().len())
            .map(|index| {
                (
                    series
                        .resolved_point_color(index, index)
                        .resolve(chart.theme),
                    chart.category(index).to_owned(),
                )
            })
            .collect()
    } else {
        chart
            .series
            .iter()
            .enumerate()
            .map(|(index, series)| {
                (
                    series.resolved_color(index).resolve(chart.theme),
                    series.label().to_owned(),
                )
            })
            .collect()
    }
}

fn draw_legend(chart: &Chart<'_>, frame: &mut canvas::Frame<Renderer>, size: Size, pie: bool) {
    let entries = legend_entries(chart, pie);

    if entries.is_empty() {
        return;
    }

    let sans = iced_font(chart.theme.font_pack().sans);
    let text_size = chart.text_size();
    let total: f32 = entries
        .iter()
        .map(|(_, label)| {
            LEGEND_SWATCH_PX + LEGEND_SWATCH_GAP_PX + estimate_text_width(label, text_size, false)
        })
        .sum::<f32>()
        + LEGEND_ITEM_GAP_PX * (entries.len() as f32 - 1.0);

    let mut x = (size.width - total) / 2.0;
    let center_y = size.height - LEGEND_HEIGHT_PX / 2.0;

    for (color, label) in entries {
        let swatch = Path::rounded_rectangle(
            Point::new(x, center_y - LEGEND_SWATCH_PX / 2.0),
            Size::new(LEGEND_SWATCH_PX, LEGEND_SWATCH_PX),
            2.0.into(),
        );
        frame.fill(&swatch, color);
        x += LEGEND_SWATCH_PX + LEGEND_SWATCH_GAP_PX;

        frame.fill_text(Text {
            content: label.clone(),
            position: Point::new(x, center_y),
            color: chart.theme.palette.foreground,
            size: text_size.into(),
            font: sans,
            align_x: crate::iced_compat::widget::text::Alignment::Left,
            align_y: Vertical::Center,
            ..Text::default()
        });
        x += estimate_text_width(&label, text_size, false) + LEGEND_ITEM_GAP_PX;
    }
}

// ─── Tooltip ─────────────────────────────────────────────────────────────────

struct TooltipRow {
    color: Color,
    name: String,
    value: String,
}

fn draw_cartesian_tooltip(
    chart: &Chart<'_>,
    frame: &mut canvas::Frame<Renderer>,
    size: Size,
    position: Point,
    index: usize,
) {
    let label = if chart.tooltip_hide_label {
        None
    } else {
        Some(chart.format_tooltip_label(chart.category(index)))
    };

    let rows: Vec<TooltipRow> = chart
        .series
        .iter()
        .enumerate()
        .filter_map(|(series_index, series)| {
            let value = series.values().get(index).copied()?;

            if !value.is_finite() {
                return None;
            }

            Some(TooltipRow {
                color: series
                    .resolved_point_color(index, series_index)
                    .resolve(chart.theme),
                name: series.label().to_owned(),
                value: chart.format_value(value),
            })
        })
        .collect();

    if rows.is_empty() {
        return;
    }

    draw_tooltip(chart, frame, size, position, label, &rows);
}

fn draw_tooltip(
    chart: &Chart<'_>,
    frame: &mut canvas::Frame<Renderer>,
    size: Size,
    position: Point,
    label: Option<String>,
    rows: &[TooltipRow],
) {
    let palette = &chart.theme.palette;
    let recipe = chart.recipe();
    let text_size = recipe.typography.size_px;
    let text_line = recipe.typography.line_height_px;
    let pad_x = recipe.tooltip_pad_x_px;
    let pad_y = recipe.tooltip_pad_y_px;
    let corner_radius = component_radius_px(chart.theme, recipe.tooltip_radius);
    let sans = iced_font(chart.theme.font_pack().sans);
    let mono = iced_font(chart.theme.font_pack().mono);
    let mut medium = sans;
    medium.weight = font::Weight::Medium;
    let mut mono_medium = mono;
    mono_medium.weight = font::Weight::Medium;

    let indicator_width = if chart.tooltip_hide_indicator {
        0.0
    } else {
        let footprint = match chart.tooltip_indicator {
            ChartIndicator::Dot => INDICATOR_PX,
            ChartIndicator::Line | ChartIndicator::Dashed => 4.0,
        };

        footprint + 8.0
    };

    let label_width = label
        .as_deref()
        .map_or(0.0, |label| estimate_text_width(label, text_size, false));
    let rows_width = rows
        .iter()
        .map(|row| {
            indicator_width
                + estimate_text_width(&row.name, text_size, false)
                + 16.0
                + estimate_text_width(&row.value, text_size, true)
        })
        .fold(0.0, f32::max);

    let width = (label_width.max(rows_width) + pad_x * 2.0).max(CHART_TOOLTIP_MIN_WIDTH_PX);
    let label_height = if label.is_some() {
        text_line + TOOLTIP_GAP
    } else {
        0.0
    };
    let height = pad_y * 2.0
        + label_height
        + rows.len() as f32 * text_line
        + (rows.len() as f32 - 1.0).max(0.0) * TOOLTIP_GAP;

    let mut x = position.x + TOOLTIP_OFFSET;
    let mut y = position.y + TOOLTIP_OFFSET;

    if x + width > size.width {
        x = (position.x - TOOLTIP_OFFSET - width).max(0.0);
    }
    if y + height > size.height {
        y = (position.y - TOOLTIP_OFFSET - height).max(0.0);
    }

    let top_left = Point::new(x, y);
    let box_size = Size::new(width, height);

    // Soft drop shadow stand-in (canvas frames cannot blur).
    let shadow = Path::rounded_rectangle(Point::new(x, y + 2.0), box_size, corner_radius.into());
    frame.fill(&shadow, Color::from_rgba(0.0, 0.0, 0.0, 0.08));

    let body = Path::rounded_rectangle(top_left, box_size, corner_radius.into());
    frame.fill(&body, palette.background);
    frame.stroke(
        &body,
        Stroke::default()
            .with_width(1.0)
            .with_color(scale_alpha(palette.border, 0.5)),
    );

    let mut row_y = y + pad_y + text_line / 2.0;

    if let Some(label) = &label {
        frame.fill_text(Text {
            content: label.clone(),
            position: Point::new(x + pad_x, row_y),
            color: palette.foreground,
            size: text_size.into(),
            font: medium,
            align_x: crate::iced_compat::widget::text::Alignment::Left,
            align_y: Vertical::Center,
            ..Text::default()
        });
        row_y += text_line + TOOLTIP_GAP;
    }

    for row in rows {
        let mut cursor_x = x + pad_x;

        if !chart.tooltip_hide_indicator {
            draw_tooltip_indicator(chart, frame, Point::new(cursor_x, row_y), row.color);
            cursor_x += indicator_width;
        }

        frame.fill_text(Text {
            content: row.name.clone(),
            position: Point::new(cursor_x, row_y),
            color: palette.muted_foreground,
            size: text_size.into(),
            font: sans,
            align_x: crate::iced_compat::widget::text::Alignment::Left,
            align_y: Vertical::Center,
            ..Text::default()
        });

        frame.fill_text(Text {
            content: row.value.clone(),
            position: Point::new(x + width - pad_x, row_y),
            color: palette.foreground,
            size: text_size.into(),
            font: mono_medium,
            align_x: crate::iced_compat::widget::text::Alignment::Right,
            align_y: Vertical::Center,
            ..Text::default()
        });

        row_y += text_line + TOOLTIP_GAP;
    }
}

fn draw_tooltip_indicator(
    chart: &Chart<'_>,
    frame: &mut canvas::Frame<Renderer>,
    row_start: Point,
    color: Color,
) {
    let text_line = chart.text_line();

    match chart.tooltip_indicator {
        ChartIndicator::Dot => {
            let dot = Path::rounded_rectangle(
                Point::new(row_start.x, row_start.y - INDICATOR_PX / 2.0),
                Size::new(INDICATOR_PX, INDICATOR_PX),
                2.0.into(),
            );
            frame.fill(&dot, color);
        }
        ChartIndicator::Line => {
            frame.fill_rectangle(
                Point::new(row_start.x, row_start.y - text_line / 2.0 + 1.0),
                Size::new(4.0, text_line - 2.0),
                color,
            );
        }
        ChartIndicator::Dashed => {
            let segment = (text_line - 2.0) / 3.0;

            for step in [0.0_f32, 2.0] {
                frame.fill_rectangle(
                    Point::new(
                        row_start.x + 1.0,
                        row_start.y - text_line / 2.0 + 1.0 + step * segment,
                    ),
                    Size::new(1.5, segment),
                    color,
                );
            }
        }
    }
}

// ─── Shared helpers ──────────────────────────────────────────────────────────

fn scale_alpha(color: Color, factor: f32) -> Color {
    Color {
        a: color.a * factor,
        ..color
    }
}

/// Rough width of `text` at `size` px; canvas text cannot be measured.
fn estimate_text_width(text: &str, size: f32, mono: bool) -> f32 {
    let factor: f32 = if mono { 0.62 } else { 0.58 };

    text.chars()
        .map(|character| match character {
            'i' | 'l' | 'j' | 't' | 'f' | 'r' | '.' | ',' | '\'' | ' ' | ':' | '|' => {
                if mono {
                    factor
                } else {
                    0.30
                }
            }
            'm' | 'w' | 'M' | 'W' => {
                if mono {
                    factor
                } else {
                    0.88
                }
            }
            _ => factor,
        })
        .sum::<f32>()
        * size
}

/// Bottom-centered axis label.
fn fill_label(
    frame: &mut canvas::Frame<Renderer>,
    content: &str,
    position: Point,
    color: Color,
    font: crate::iced_compat::Font,
    size: f32,
) {
    frame.fill_text(Text {
        content: content.to_owned(),
        position,
        color,
        size: size.into(),
        font,
        align_x: crate::iced_compat::widget::text::Alignment::Center,
        align_y: Vertical::Center,
        ..Text::default()
    });
}

/// Right-aligned axis label (left axis).
fn fill_label_right(
    frame: &mut canvas::Frame<Renderer>,
    content: &str,
    position: Point,
    color: Color,
    font: crate::iced_compat::Font,
    size: f32,
) {
    frame.fill_text(Text {
        content: content.to_owned(),
        position,
        color,
        size: size.into(),
        font,
        align_x: crate::iced_compat::widget::text::Alignment::Right,
        align_y: Vertical::Center,
        ..Text::default()
    });
}
