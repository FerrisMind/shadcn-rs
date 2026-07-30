//! Backend-agnostic value, fraction, and step-grid helpers.

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

fn ordered_bounds(min: f32, max: f32) -> (f32, f32) {
    let min = if min.is_finite() { min } else { 0.0 };
    let max = if max.is_finite() { max } else { min };

    if min <= max { (min, max) } else { (max, min) }
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
}
