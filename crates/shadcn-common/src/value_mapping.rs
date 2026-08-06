//! Backend-agnostic value, fraction, and step-grid helpers.
//!
//! Extra numeric helpers mirror Zag `@zag-js/utils` `number.ts` so slider,
//! number-input, and color-channel math stay consistent across backends.

/// Maps a value from `[min, max]` to a clamped fraction in `0.0..=1.0`.
#[must_use]
pub fn fraction(value: f32, min: f32, max: f32) -> f32 {
    let (min, max) = ordered_bounds(min, max);
    let span = max - min;

    if !value.is_finite() || !span.is_finite() || span.abs() <= f32::EPSILON {
        return 0.0;
    }

    ((value - min) / span).clamp(0.0, 1.0)
}

/// Snaps a value to a positive finite step grid anchored at `min`.
///
/// A non-positive or non-finite step leaves the clamped value continuous.
#[must_use]
pub fn snap(value: f32, min: f32, max: f32, step: f32) -> f32 {
    let (min, max) = ordered_bounds(min, max);
    let clamped = if value.is_finite() {
        value.clamp(min, max)
    } else {
        min
    };

    if !step.is_finite() || step <= 0.0 {
        return clamped;
    }

    let steps = ((clamped - min) / step).round();
    (min + steps * step).clamp(min, max)
}

/// Maps a value to a fraction after applying the step grid.
#[must_use]
pub fn snapped_fraction(value: f32, min: f32, max: f32, step: f32) -> f32 {
    fraction(snap(value, min, max, step), min, max)
}

/// Maps a normalized fraction back into the range and applies the step grid.
#[must_use]
pub fn value_at_fraction(value: f32, min: f32, max: f32, step: f32) -> f32 {
    let (min, max) = ordered_bounds(min, max);
    let fraction = if value.is_finite() {
        value.clamp(0.0, 1.0)
    } else {
        0.0
    };

    snap(min + (max - min) * fraction, min, max, step)
}

/// Finds the closest finite value, resolving exact distance ties towards the
/// cursor direction so stacked slider thumbs can still be selected.
#[must_use]
pub fn closest_index(values: &[f32], target: f32) -> Option<usize> {
    let target = if target.is_finite() { target } else { 0.0 };
    let mut best = None;
    let mut best_distance = f32::INFINITY;

    for (index, value) in values.iter().copied().enumerate() {
        if !value.is_finite() {
            continue;
        }

        let distance = (value - target).abs();
        let Some(best_index) = best else {
            best = Some(index);
            best_distance = distance;
            continue;
        };

        let closer = distance < best_distance - f32::EPSILON;
        let tie_breaks_towards_cursor = (distance - best_distance).abs() <= f32::EPSILON
            && ((target > value && index > best_index) || (target < value && index < best_index));

        if closer || tie_breaks_towards_cursor {
            best = Some(index);
            best_distance = distance;
        }
    }

    best
}

/// Replaces non-finite values with `0.0` (Zag `nan`).
#[must_use]
pub fn finite_or_zero(value: f32) -> f32 {
    if value.is_finite() { value } else { 0.0 }
}

/// Euclidean modulo that always returns a non-negative remainder.
#[must_use]
pub fn modulo(value: f32, modulus: f32) -> f32 {
    if !modulus.is_finite() || modulus == 0.0 {
        return finite_or_zero(value);
    }
    let value = finite_or_zero(value);
    ((value % modulus) + modulus) % modulus
}

/// Wraps `value` into `[0, max)`.
#[must_use]
pub fn wrap(value: f32, max: f32) -> f32 {
    modulo(value, max)
}

/// Rounds `value` to the decimal precision implied by `step`.
#[must_use]
pub fn round_to_step_precision(value: f32, step: f32) -> f32 {
    if !value.is_finite() {
        return 0.0;
    }
    let decimals = step_decimal_places(step);
    if decimals == 0 {
        return value.round();
    }
    let scale = 10f32.powi(decimals as i32);
    (value * scale).round() / scale
}

/// Zag-style snap that clamps to the largest valid step at the upper bound.
#[must_use]
pub fn snap_value_to_step(value: f32, min: f32, max: f32, step: f32) -> f32 {
    let (min, max) = ordered_bounds(min, max);
    let value = finite_or_zero(value);

    if !step.is_finite() || step <= 0.0 {
        return value.clamp(min, max);
    }

    let remainder = (value - min) % step;
    let snapped = if remainder.abs() * 2.0 >= step {
        value + remainder.signum() * (step - remainder.abs())
    } else {
        value - remainder
    };
    let snapped = round_to_step_precision(snapped, step);

    if snapped < min {
        return min;
    }
    if snapped > max {
        let steps_in_range = ((max - min) / step).floor();
        let largest_valid = min + steps_in_range * step;
        let snapped = if steps_in_range <= 0.0 || largest_valid < min {
            max
        } else {
            largest_valid
        };
        return round_to_step_precision(snapped, step);
    }

    snapped
}

/// Inclusive lower bound for a multi-thumb value at `index`.
#[must_use]
pub fn min_value_at_index(index: usize, values: &[f32], min: f32) -> f32 {
    if index == 0 {
        min
    } else {
        values.get(index - 1).copied().unwrap_or(min)
    }
}

/// Inclusive upper bound for a multi-thumb value at `index`.
#[must_use]
pub fn max_value_at_index(index: usize, values: &[f32], max: f32) -> f32 {
    if index + 1 >= values.len() {
        max
    } else {
        values.get(index + 1).copied().unwrap_or(max)
    }
}

/// Per-thumb range for ordered multi-value controls (Zag `getValueRanges`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ValueRange {
    /// Lower bound for this thumb.
    pub min: f32,
    /// Upper bound for this thumb.
    pub max: f32,
    /// Current thumb value.
    pub value: f32,
}

/// Builds per-index ranges enforcing a minimum `gap` between thumbs.
#[must_use]
pub fn value_ranges(values: &[f32], min: f32, max: f32, gap: f32) -> Vec<ValueRange> {
    let (min, max) = ordered_bounds(min, max);
    let gap = if gap.is_finite() && gap > 0.0 {
        gap
    } else {
        0.0
    };
    let len = values.len();

    values
        .iter()
        .copied()
        .enumerate()
        .map(|(index, value)| ValueRange {
            min: if index == 0 {
                min
            } else {
                values[index - 1] + gap
            },
            max: if index + 1 >= len {
                max
            } else {
                values[index + 1] - gap
            },
            value: finite_or_zero(value),
        })
        .collect()
}

/// Sets `values[index]` after snapping into the thumb's allowed range.
#[must_use]
pub fn set_value_at_index(
    values: &[f32],
    index: usize,
    value: f32,
    min: f32,
    max: f32,
    step: f32,
) -> Vec<f32> {
    let mut next = values.to_vec();
    if index >= next.len() {
        return next;
    }
    let lower = min_value_at_index(index, &next, min);
    let upper = max_value_at_index(index, &next, max);
    next[index] = snap_value_to_step(value, lower, upper, step);
    next
}

/// Linearly maps `value` from domain `from` onto range `to`.
#[must_use]
pub fn transform_value(value: f32, from: [f32; 2], to: [f32; 2]) -> f32 {
    let [a, b] = from;
    let [c, d] = to;
    if (a - b).abs() <= f32::EPSILON || (c - d).abs() <= f32::EPSILON {
        return c;
    }
    c + ((d - c) / (b - a)) * (finite_or_zero(value) - a)
}

/// Adds `step` with decimal-safe arithmetic (Zag `incrementValue`).
#[must_use]
pub fn increment(value: f32, step: f32) -> f32 {
    decimal_op(finite_or_zero(value), step, true)
}

/// Subtracts `step` with decimal-safe arithmetic (Zag `decrementValue`).
#[must_use]
pub fn decrement(value: f32, step: f32) -> f32 {
    decimal_op(finite_or_zero(value), step, false)
}

fn ordered_bounds(min: f32, max: f32) -> (f32, f32) {
    let min = if min.is_finite() { min } else { 0.0 };
    let max = if max.is_finite() { max } else { min };

    if min <= max { (min, max) } else { (max, min) }
}

fn step_decimal_places(step: f32) -> u32 {
    if !step.is_finite() || step <= 0.0 {
        return 0;
    }
    let text = format!("{step}");
    text.find('.')
        .map(|dot| (text.len() - dot - 1) as u32)
        .unwrap_or(0)
}

fn count_decimals(value: f32) -> u32 {
    if !value.is_finite() {
        return 0;
    }
    let mut scale = 1.0_f32;
    let mut places = 0_u32;
    while places < 12 && (value * scale).round() / scale != value {
        scale *= 10.0;
        places += 1;
    }
    places
}

fn decimal_op(left: f32, right: f32, add: bool) -> f32 {
    let right = finite_or_zero(right);
    if left.fract() == 0.0 && right.fract() == 0.0 {
        return if add { left + right } else { left - right };
    }
    let scale = 10f32.powi(count_decimals(left).max(count_decimals(right)) as i32);
    let left_i = (left * scale).round();
    let right_i = (right * scale).round();
    let result = if add {
        left_i + right_i
    } else {
        left_i - right_i
    };
    result / scale
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_and_clamps_values() {
        assert_eq!(fraction(25.0, 0.0, 100.0), 0.25);
        assert_eq!(fraction(-5.0, 0.0, 100.0), 0.0);
        assert_eq!(fraction(120.0, 0.0, 100.0), 1.0);
        assert_eq!(fraction(5.0, 5.0, 5.0), 0.0);
    }

    #[test]
    fn snaps_to_a_grid_anchored_at_the_lower_bound() {
        assert_eq!(snap(12.0, 0.0, 100.0, 5.0), 10.0);
        assert_eq!(snap(13.0, 0.0, 100.0, 5.0), 15.0);
        assert_eq!(snap(-10.0, 0.0, 100.0, 5.0), 0.0);
        assert_eq!(snap(12.3, 0.0, 100.0, 0.0), 12.3);
    }

    #[test]
    fn maps_fractions_back_to_values() {
        assert_eq!(value_at_fraction(0.5, 0.0, 100.0, 5.0), 50.0);
        assert_eq!(value_at_fraction(f32::NAN, 0.0, 100.0, 5.0), 0.0);
    }

    #[test]
    fn chooses_the_closest_finite_value() {
        assert_eq!(closest_index(&[10.0, 60.0], 51.0), Some(1));
        assert_eq!(closest_index(&[10.0, f32::NAN], 50.0), Some(0));
        assert_eq!(closest_index(&[f32::NAN], 50.0), None);
    }

    #[test]
    fn wraps_and_modulos_like_zag() {
        assert!((modulo(-1.0, 360.0) - 359.0).abs() < f32::EPSILON);
        assert!((wrap(370.0, 360.0) - 10.0).abs() < f32::EPSILON);
    }

    #[test]
    fn snap_value_to_step_respects_upper_step_grid() {
        assert_eq!(snap_value_to_step(99.0, 0.0, 100.0, 3.0), 99.0);
        assert_eq!(snap_value_to_step(100.0, 0.0, 100.0, 3.0), 99.0);
    }

    #[test]
    fn multi_thumb_ranges_and_setters() {
        let values = [10.0, 40.0, 80.0];
        let ranges = value_ranges(&values, 0.0, 100.0, 5.0);
        assert_eq!(ranges[1].min, 15.0);
        assert_eq!(ranges[1].max, 75.0);

        let next = set_value_at_index(&values, 1, 70.0, 0.0, 100.0, 5.0);
        assert_eq!(next[1], 70.0);
        assert_eq!(next[0], 10.0);
    }

    #[test]
    fn decimal_increment_avoids_float_drift() {
        assert_eq!(increment(0.1, 0.1), 0.2);
        assert_eq!(decrement(1.0, 0.1), 0.9);
    }
}
