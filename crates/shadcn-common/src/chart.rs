//! Backend-agnostic chart layout engine.
//!
//! Ports the slice of layerchart/d3 behaviour that the shadcn chart relies
//! on: nice linear ticks (`yNice`), band scales with padding
//! (`scaleBand().padding(0.25)`), grouped-bar sub-bands
//! (`x1Scale.paddingInner(0.2)`), diverging value stacking
//! (`seriesLayout="stack"` with negative support), natural cubic splines
//! (`curveNatural`), pie slice angles, and cursor-to-sample hit testing.
//! The module works in logical pixels and data units only, so a GUI backend
//! owns rendering, animation frames, and input plumbing.

/// Default aspect ratio of a chart container (`aspect-video`).
pub const CHART_ASPECT_RATIO: f32 = 16.0 / 9.0;

/// Band padding of the shadcn bar charts (`scaleBand().padding(0.25)`).
pub const CHART_BAND_PADDING_FRACTION: f32 = 0.25;

/// Inner padding between grouped bars (`x1Scale.paddingInner(0.2)`).
pub const CHART_GROUP_PADDING_FRACTION: f32 = 0.2;

/// Fill opacity of area marks (`fillOpacity: 0.4`).
pub const CHART_AREA_FILL_OPACITY: f32 = 0.4;

/// Duration of the entrance motion tween (`motion: "tween"`, 500 ms).
pub const CHART_MOTION_MS: f32 = 500.0;

/// Target number of value-axis ticks (`yNice={4}`).
pub const CHART_TICK_COUNT: usize = 4;

/// Minimum tooltip width (`min-w-[9rem]`).
pub const CHART_TOOLTIP_MIN_WIDTH_PX: f32 = 144.0;

/// Radius of the hover highlight point on line/area charts
/// (`highlight: { points: { r: 4 } }`).
pub const CHART_HIGHLIGHT_POINT_RADIUS_PX: f32 = 4.0;

/// Nice tick values covering `min..=max` (d3 `ticks`).
///
/// The step is the largest of `1`, `2`, or `5` times a power of ten that
/// produces at most about `target` intervals, and returned ticks are the
/// multiples of that step inside the domain. Degenerate or non-finite
/// domains yield an empty list.
///
/// ```rust
/// use shadcn_common::chart_nice_ticks;
///
/// assert_eq!(chart_nice_ticks(0.0, 305.0, 4), [0.0, 100.0, 200.0, 300.0]);
/// ```
#[must_use]
pub fn chart_nice_ticks(min: f64, max: f64, target: usize) -> Vec<f64> {
    if !min.is_finite() || !max.is_finite() || target == 0 {
        return Vec::new();
    }

    let (lo, hi) = if min <= max { (min, max) } else { (max, min) };

    if hi - lo <= f64::EPSILON {
        return vec![lo];
    }

    let step = nice_step(hi - lo, target);
    let first = (lo / step).ceil();
    let last = (hi / step).floor();
    let count = (last - first) as usize + 1;
    let mut ticks = Vec::with_capacity(count);

    for i in 0..count {
        ticks.push((first + i as f64) * step);
    }

    ticks
}

/// Domain expanded outward to the nice tick grid (d3 `nice`).
///
/// ```rust
/// use shadcn_common::chart_nice_domain;
///
/// assert_eq!(chart_nice_domain(3.0, 97.0, 4), (0.0, 100.0));
/// ```
#[must_use]
pub fn chart_nice_domain(min: f64, max: f64, target: usize) -> (f64, f64) {
    if !min.is_finite() || !max.is_finite() || target == 0 {
        return (min, max);
    }

    let (lo, hi) = if min <= max { (min, max) } else { (max, min) };

    if hi - lo <= f64::EPSILON {
        return (lo, hi);
    }

    let step = nice_step(hi - lo, target);

    ((lo / step).floor() * step, (hi / step).ceil() * step)
}

/// The 1/2/5 × 10ⁿ step that splits `span` into about `target` intervals
/// (d3 `tickIncrement` rounding thresholds: √2, √10, √50).
fn nice_step(span: f64, target: usize) -> f64 {
    let raw = span / target.max(1) as f64;
    let power = raw.log10().floor();
    let magnitude = 10.0_f64.powf(power);
    let error = raw / magnitude;

    let nice = if error >= 50.0_f64.sqrt() {
        10.0
    } else if error >= 10.0_f64.sqrt() {
        5.0
    } else if error >= 2.0_f64.sqrt() {
        2.0
    } else {
        1.0
    };

    nice * magnitude
}

/// Value extent of `series`, optionally stacked, always spanning zero.
///
/// Stacking sums positives and negatives per sample separately, matching
/// diverging d3 stacks. Non-finite samples are ignored. Returns `None` when
/// no finite sample exists.
///
/// ```rust
/// use shadcn_common::chart_value_extent;
///
/// let desktop = [186.0, 305.0];
/// let mobile = [80.0, 200.0];
///
/// assert_eq!(
///     chart_value_extent(&[&desktop, &mobile], true),
///     Some((0.0, 505.0)),
/// );
/// ```
#[must_use]
pub fn chart_value_extent(series: &[&[f64]], stacked: bool) -> Option<(f64, f64)> {
    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;

    if stacked {
        let samples = series.iter().map(|values| values.len()).max().unwrap_or(0);

        for index in 0..samples {
            let mut positive = 0.0_f64;
            let mut negative = 0.0_f64;

            for values in series {
                match values.get(index) {
                    Some(value) if value.is_finite() => {
                        if *value >= 0.0 {
                            positive += value;
                        } else {
                            negative += value;
                        }
                    }
                    _ => {}
                }
            }

            min = min.min(negative);
            max = max.max(positive);
        }
    } else {
        for values in series {
            for value in *values {
                if value.is_finite() {
                    min = min.min(*value);
                    max = max.max(*value);
                }
            }
        }
    }

    if min > max {
        return None;
    }

    Some((min.min(0.0), max.max(0.0)))
}

/// Diverging stack spans per series and sample (d3 `stack` with a
/// diverging offset): positives pile up from zero, negatives pile down.
///
/// `spans[series][sample] = (start, end)` in data units, with
/// `start <= end`. Missing or non-finite samples produce `(0.0, 0.0)`.
///
/// ```rust
/// use shadcn_common::chart_stack_spans;
///
/// let spans = chart_stack_spans(&[&[10.0, -5.0], &[20.0, -15.0]]);
///
/// assert_eq!(spans[0][0], (0.0, 10.0));
/// assert_eq!(spans[1][0], (10.0, 30.0));
/// assert_eq!(spans[1][1], (-20.0, -5.0));
/// ```
#[must_use]
pub fn chart_stack_spans(series: &[&[f64]]) -> Vec<Vec<(f64, f64)>> {
    let samples = series.iter().map(|values| values.len()).max().unwrap_or(0);
    let mut spans = vec![vec![(0.0, 0.0); samples]; series.len()];
    let mut positive = vec![0.0_f64; samples];
    let mut negative = vec![0.0_f64; samples];

    for (series_index, values) in series.iter().enumerate() {
        for sample in 0..samples {
            let value = values
                .get(sample)
                .copied()
                .filter(|value| value.is_finite())
                .unwrap_or(0.0);

            spans[series_index][sample] = if value >= 0.0 {
                let start = positive[sample];
                positive[sample] += value;
                (start, positive[sample])
            } else {
                let end = negative[sample];
                negative[sample] += value;
                (negative[sample], end)
            };
        }
    }

    spans
}

/// Band starts and bandwidth of a d3 `scaleBand` with equal inner/outer
/// padding (`padding(fraction)`).
///
/// ```rust
/// use shadcn_common::chart_band_slots;
///
/// let (starts, width) = chart_band_slots(625.0, 6, 0.25);
///
/// assert_eq!(starts.len(), 6);
/// assert!((width - 75.0).abs() < 0.01);
/// assert!((starts[0] - 25.0).abs() < 0.01);
/// ```
#[must_use]
pub fn chart_band_slots(range_px: f32, count: usize, padding_fraction: f32) -> (Vec<f32>, f32) {
    if count == 0 || !range_px.is_finite() || range_px <= 0.0 {
        return (Vec::new(), 0.0);
    }

    let padding = padding_fraction.clamp(0.0, 1.0);
    let divisions = (count as f32 - padding + 2.0 * padding).max(f32::EPSILON);
    let step = range_px / divisions;
    let bandwidth = step * (1.0 - padding);
    let offset = (range_px - step * (count as f32 - padding)) * 0.5;

    let starts = (0..count).map(|i| offset + i as f32 * step).collect();

    (starts, bandwidth.max(0.0))
}

/// Sub-band offsets and width for grouped bars inside one band
/// (`x1Scale = scaleBand().paddingInner(fraction)`).
///
/// ```rust
/// use shadcn_common::chart_group_slots;
///
/// let (offsets, width) = chart_group_slots(90.0, 2, 0.2);
///
/// assert_eq!(offsets.len(), 2);
/// assert!((width - 40.0).abs() < 0.01);
/// ```
#[must_use]
pub fn chart_group_slots(
    bandwidth_px: f32,
    series_count: usize,
    inner_padding_fraction: f32,
) -> (Vec<f32>, f32) {
    if series_count == 0 || !bandwidth_px.is_finite() || bandwidth_px <= 0.0 {
        return (Vec::new(), 0.0);
    }

    let padding = inner_padding_fraction.clamp(0.0, 1.0);
    let divisions = (series_count as f32 - padding).max(f32::EPSILON);
    let step = bandwidth_px / divisions;
    let width = step * (1.0 - padding);

    let offsets = (0..series_count).map(|i| i as f32 * step).collect();

    (offsets, width.max(0.0))
}

/// Fraction of `value` inside `min..=max` (a linear scale without range).
///
/// The result is not clamped so out-of-domain marks keep their geometry;
/// degenerate or non-finite domains map everything to `0.0`.
///
/// ```rust
/// use shadcn_common::chart_linear_fraction;
///
/// assert_eq!(chart_linear_fraction(150.0, 0.0, 300.0), 0.5);
/// ```
#[must_use]
pub fn chart_linear_fraction(value: f64, min: f64, max: f64) -> f32 {
    if !value.is_finite() || !min.is_finite() || !max.is_finite() {
        return 0.0;
    }

    let span = max - min;

    if span.abs() <= f64::EPSILON {
        return 0.0;
    }

    ((value - min) / span) as f32
}

/// One cubic Bézier segment of an interpolated curve.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ChartCubicSegment {
    /// First control point.
    pub control_1: (f32, f32),
    /// Second control point.
    pub control_2: (f32, f32),
    /// Segment end point.
    pub to: (f32, f32),
}

/// Natural cubic spline through `points` (d3 `curveNatural`) as cubic
/// Bézier segments. Fewer than two points yield no segments; exactly two
/// yield one straight segment.
///
/// ```rust
/// use shadcn_common::chart_natural_curve;
///
/// let segments = chart_natural_curve(&[(0.0, 0.0), (1.0, 1.0), (2.0, 0.0)]);
///
/// assert_eq!(segments.len(), 2);
/// assert_eq!(segments[1].to, (2.0, 0.0));
/// ```
#[must_use]
pub fn chart_natural_curve(points: &[(f32, f32)]) -> Vec<ChartCubicSegment> {
    if points.len() < 2 {
        return Vec::new();
    }

    let xs: Vec<f32> = points.iter().map(|point| point.0).collect();
    let ys: Vec<f32> = points.iter().map(|point| point.1).collect();
    let (x1, x2) = natural_control_points(&xs);
    let (y1, y2) = natural_control_points(&ys);

    (0..points.len() - 1)
        .map(|i| ChartCubicSegment {
            control_1: (x1[i], y1[i]),
            control_2: (x2[i], y2[i]),
            to: points[i + 1],
        })
        .collect()
}

/// First/second Bézier control values of a 1-D natural spline through `k`.
///
/// Solves the standard tridiagonal system with the Thomas algorithm.
fn natural_control_points(k: &[f32]) -> (Vec<f32>, Vec<f32>) {
    let n = k.len() - 1;

    if n == 1 {
        let p1 = (2.0 * k[0] + k[1]) / 3.0;
        return (vec![p1], vec![2.0 * p1 - k[0]]);
    }

    let mut a = vec![0.0_f32; n];
    let mut b = vec![0.0_f32; n];
    let mut c = vec![0.0_f32; n];
    let mut r = vec![0.0_f32; n];

    b[0] = 2.0;
    c[0] = 1.0;
    r[0] = k[0] + 2.0 * k[1];

    for i in 1..n - 1 {
        a[i] = 1.0;
        b[i] = 4.0;
        c[i] = 1.0;
        r[i] = 4.0 * k[i] + 2.0 * k[i + 1];
    }

    a[n - 1] = 2.0;
    b[n - 1] = 7.0;
    r[n - 1] = 8.0 * k[n - 1] + k[n];

    for i in 1..n {
        let m = a[i] / b[i - 1];
        b[i] -= m * c[i - 1];
        r[i] -= m * r[i - 1];
    }

    let mut p1 = vec![0.0_f32; n];
    p1[n - 1] = r[n - 1] / b[n - 1];

    for i in (0..n - 1).rev() {
        p1[i] = (r[i] - c[i] * p1[i + 1]) / b[i];
    }

    let mut p2 = vec![0.0_f32; n];

    for i in 0..n - 1 {
        p2[i] = 2.0 * k[i + 1] - p1[i + 1];
    }
    p2[n - 1] = (k[n] + p1[n - 1]) / 2.0;

    (p1, p2)
}

/// One pie slice as fractions of a full turn, clockwise from 12 o'clock.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ChartPieSlice {
    /// Start of the slice in `0.0..=1.0` turns.
    pub start_fraction: f32,
    /// End of the slice in `0.0..=1.0` turns.
    pub end_fraction: f32,
}

impl ChartPieSlice {
    /// Angular size of the slice in turns.
    #[must_use]
    pub fn sweep_fraction(&self) -> f32 {
        (self.end_fraction - self.start_fraction).max(0.0)
    }
}

/// Slice angles of a pie chart (d3 `pie` without sorting).
///
/// Non-finite or non-positive values produce empty slices that keep their
/// index, so slice colors and labels stay aligned with the input.
///
/// ```rust
/// use shadcn_common::chart_pie_slices;
///
/// let slices = chart_pie_slices(&[1.0, 1.0, 2.0]);
///
/// assert_eq!(slices.len(), 3);
/// assert!((slices[2].sweep_fraction() - 0.5).abs() < 1e-6);
/// ```
#[must_use]
pub fn chart_pie_slices(values: &[f64]) -> Vec<ChartPieSlice> {
    let total: f64 = values
        .iter()
        .filter(|value| value.is_finite() && **value > 0.0)
        .sum();

    if total <= 0.0 {
        return values
            .iter()
            .map(|_| ChartPieSlice {
                start_fraction: 0.0,
                end_fraction: 0.0,
            })
            .collect();
    }

    let mut cursor = 0.0_f64;

    values
        .iter()
        .map(|value| {
            let share = if value.is_finite() && *value > 0.0 {
                value / total
            } else {
                0.0
            };
            let start = cursor;
            cursor += share;

            ChartPieSlice {
                start_fraction: start as f32,
                end_fraction: cursor as f32,
            }
        })
        .collect()
}

/// Index of the center in `centers` closest to `position_px`, if any.
///
/// ```rust
/// use shadcn_common::chart_nearest_center;
///
/// assert_eq!(chart_nearest_center(&[10.0, 50.0, 90.0], 60.0), Some(1));
/// ```
#[must_use]
pub fn chart_nearest_center(centers: &[f32], position_px: f32) -> Option<usize> {
    if !position_px.is_finite() {
        return None;
    }

    centers
        .iter()
        .enumerate()
        .filter(|(_, center)| center.is_finite())
        .min_by(|(_, a), (_, b)| {
            (**a - position_px)
                .abs()
                .total_cmp(&(**b - position_px).abs())
        })
        .map(|(index, _)| index)
}

/// Slice hit by the cursor at offset `(dx, dy)` from the pie center.
///
/// The cursor must sit inside the `inner_radius..=outer_radius` ring; the
/// angle is measured clockwise from 12 o'clock like [`chart_pie_slices`].
///
/// ```rust
/// use shadcn_common::{chart_pie_hit, chart_pie_slices};
///
/// let slices = chart_pie_slices(&[1.0, 1.0]);
///
/// // Right of center ⇒ first (clockwise) half.
/// assert_eq!(chart_pie_hit(20.0, 0.0, &slices, 50.0, 0.0), Some(0));
/// assert_eq!(chart_pie_hit(-20.0, 0.0, &slices, 50.0, 0.0), Some(1));
/// ```
#[must_use]
pub fn chart_pie_hit(
    dx: f32,
    dy: f32,
    slices: &[ChartPieSlice],
    outer_radius: f32,
    inner_radius: f32,
) -> Option<usize> {
    if !dx.is_finite() || !dy.is_finite() {
        return None;
    }

    let distance = (dx * dx + dy * dy).sqrt();

    if distance > outer_radius || distance < inner_radius.max(0.0) {
        return None;
    }

    // atan2 with x/y swapped measures clockwise from 12 o'clock.
    let angle = dx.atan2(-dy);
    let fraction = (angle / std::f32::consts::TAU).rem_euclid(1.0);

    slices.iter().position(|slice| {
        fraction >= slice.start_fraction && fraction < slice.end_fraction.max(slice.start_fraction)
    })
}

/// Formats a value like JavaScript `toLocaleString("en-US")`: thousands
/// separated by commas, at most three fraction digits, no trailing zeros.
///
/// ```rust
/// use shadcn_common::chart_format_value;
///
/// assert_eq!(chart_format_value(1234.0), "1,234");
/// assert_eq!(chart_format_value(-0.5), "-0.5");
/// ```
#[must_use]
pub fn chart_format_value(value: f64) -> String {
    if !value.is_finite() {
        return String::from("–");
    }

    let negative = value < 0.0;
    let rounded = (value.abs() * 1000.0).round() / 1000.0;
    let integer = rounded.trunc() as u64;
    let fraction = rounded.fract();

    let digits = integer.to_string();
    let mut grouped = String::with_capacity(digits.len() + digits.len() / 3 + 1);

    for (index, digit) in digits.chars().enumerate() {
        if index > 0 && (digits.len() - index).is_multiple_of(3) {
            grouped.push(',');
        }
        grouped.push(digit);
    }

    let mut result = String::new();

    if negative && (integer > 0 || fraction > 0.0) {
        result.push('-');
    }
    result.push_str(&grouped);

    if fraction > 0.0 {
        let fraction_digits = format!("{fraction:.3}");
        let trimmed = fraction_digits
            .trim_start_matches("0.")
            .trim_end_matches('0');

        if !trimmed.is_empty() {
            result.push('.');
            result.push_str(trimmed);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nice_ticks_cover_domain() {
        let ticks = chart_nice_ticks(-209.0, 214.0, 4);

        assert_eq!(ticks, [-200.0, -100.0, 0.0, 100.0, 200.0]);
    }

    #[test]
    fn nice_ticks_reject_bad_input() {
        assert!(chart_nice_ticks(f64::NAN, 1.0, 4).is_empty());
        assert!(chart_nice_ticks(0.0, 1.0, 0).is_empty());
        assert_eq!(chart_nice_ticks(5.0, 5.0, 4), [5.0]);
    }

    #[test]
    fn extent_spans_zero() {
        assert_eq!(
            chart_value_extent(&[&[186.0, 305.0]], false),
            Some((0.0, 305.0)),
        );
        assert_eq!(
            chart_value_extent(&[&[-207.0, 214.0]], false),
            Some((-207.0, 214.0)),
        );
        assert_eq!(chart_value_extent(&[&[]], false), None);
    }

    #[test]
    fn stack_handles_negatives() {
        let spans = chart_stack_spans(&[&[5.0, -5.0], &[-3.0, 10.0]]);

        assert_eq!(spans[0][0], (0.0, 5.0));
        assert_eq!(spans[1][0], (-3.0, 0.0));
        assert_eq!(spans[0][1], (-5.0, 0.0));
        assert_eq!(spans[1][1], (0.0, 10.0));
    }

    #[test]
    fn band_slots_match_d3() {
        let (starts, width) = chart_band_slots(625.0, 6, 0.25);
        let step = 625.0 / 6.25;

        assert!((width - step * 0.75).abs() < 0.001);
        assert!((starts[0] - step * 0.25).abs() < 0.001);
        assert!((starts[5] - (step * 0.25 + 5.0 * step)).abs() < 0.001);
    }

    #[test]
    fn band_slots_handle_degenerate_input() {
        assert_eq!(chart_band_slots(0.0, 6, 0.25).0.len(), 0);
        assert_eq!(chart_band_slots(100.0, 0, 0.25).0.len(), 0);
    }

    #[test]
    fn group_slots_split_band() {
        let (offsets, width) = chart_group_slots(90.0, 2, 0.2);
        let step = 90.0 / 1.8;

        assert!((width - step * 0.8).abs() < 0.001);
        assert_eq!(offsets[0], 0.0);
        assert!((offsets[1] - step).abs() < 0.001);
    }

    #[test]
    fn natural_curve_hits_end_points() {
        let points = [(0.0, 0.0), (10.0, 20.0), (20.0, 5.0), (30.0, 15.0)];
        let segments = chart_natural_curve(&points);

        assert_eq!(segments.len(), 3);
        for (segment, point) in segments.iter().zip(&points[1..]) {
            assert_eq!(segment.to, *point);
        }
    }

    #[test]
    fn natural_curve_two_points_is_straight() {
        let segments = chart_natural_curve(&[(0.0, 0.0), (3.0, 3.0)]);

        assert_eq!(segments.len(), 1);
        assert!((segments[0].control_1.0 - 1.0).abs() < 0.001);
        assert!((segments[0].control_2.0 - 2.0).abs() < 0.001);
    }

    #[test]
    fn pie_slices_are_contiguous() {
        let slices = chart_pie_slices(&[275.0, 200.0, 187.0, 173.0, 90.0]);

        assert_eq!(slices[0].start_fraction, 0.0);
        for pair in slices.windows(2) {
            assert!((pair[0].end_fraction - pair[1].start_fraction).abs() < 1e-6);
        }
        assert!((slices[4].end_fraction - 1.0).abs() < 1e-6);
    }

    #[test]
    fn pie_slices_skip_invalid_values() {
        let slices = chart_pie_slices(&[1.0, f64::NAN, -2.0, 1.0]);

        assert_eq!(slices.len(), 4);
        assert_eq!(slices[1].sweep_fraction(), 0.0);
        assert_eq!(slices[2].sweep_fraction(), 0.0);
        assert!((slices[3].end_fraction - 1.0).abs() < 1e-6);
    }

    #[test]
    fn pie_hit_respects_donut_hole() {
        let slices = chart_pie_slices(&[1.0]);

        assert_eq!(chart_pie_hit(0.0, -30.0, &slices, 50.0, 40.0), None);
        assert_eq!(chart_pie_hit(0.0, -45.0, &slices, 50.0, 40.0), Some(0));
        assert_eq!(chart_pie_hit(0.0, -60.0, &slices, 50.0, 40.0), None);
    }

    #[test]
    fn nearest_center_ignores_non_finite() {
        assert_eq!(chart_nearest_center(&[f32::NAN, 10.0], 0.0), Some(1));
        assert_eq!(chart_nearest_center(&[], 0.0), None);
        assert_eq!(chart_nearest_center(&[1.0], f32::NAN), None);
    }

    #[test]
    fn format_value_groups_thousands() {
        assert_eq!(chart_format_value(0.0), "0");
        assert_eq!(chart_format_value(1234567.0), "1,234,567");
        assert_eq!(chart_format_value(-207.0), "-207");
        assert_eq!(chart_format_value(12.25), "12.25");
        assert_eq!(chart_format_value(f64::NAN), "–");
    }
}
