//! Backend-agnostic meter range math shared by iced and egui.
//!
//! Mirrors bits-ui `Meter` (`value` / `min` / `max`) plus the threshold bands
//! used by the shadcn-svelte-extras Tokens demo.

use crate::recipes::meter::WARNING_RATIO;
use crate::value_mapping::fraction;

/// Inclusive measurement range for a meter.
///
/// ```rust
/// use shadcn_common::{MeterConfig, meter_ratio};
///
/// let config = MeterConfig::new().value(50.0);
/// assert_eq!(meter_ratio(config), 0.5);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
#[must_use]
pub struct MeterConfig {
    /// Current measurement.
    pub value: f32,
    /// Lower bound (`aria-valuemin`, default `0`).
    pub min: f32,
    /// Upper bound (`aria-valuemax`, default `100`).
    pub max: f32,
}

impl Default for MeterConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl MeterConfig {
    /// Bits-ui defaults: `value = 0`, `min = 0`, `max = 100`.
    pub const fn new() -> Self {
        Self {
            value: 0.0,
            min: 0.0,
            max: 100.0,
        }
    }

    /// Sets the current measurement (non-finite values become `0`).
    pub fn value(mut self, value: f32) -> Self {
        self.value = sanitize_scalar(value);
        self
    }

    /// Sets the lower bound.
    pub fn min(mut self, min: f32) -> Self {
        self.min = sanitize_scalar(min);
        self
    }

    /// Sets the upper bound.
    pub fn max(mut self, max: f32) -> Self {
        self.max = sanitize_scalar(max);
        self
    }

    /// Returns `(min, max)` with a positive finite span.
    #[must_use]
    pub fn ordered_bounds(self) -> (f32, f32) {
        sanitize_bounds(self.min, self.max)
    }

    /// Clamps [`Self::value`] into the sanitized bounds.
    #[must_use]
    pub fn clamped_value(self) -> f32 {
        let (min, max) = self.ordered_bounds();
        clamp_meter_value(self.value, min, max)
    }

    /// Normalized fill ratio in `0.0..=1.0`.
    #[must_use]
    pub fn ratio(self) -> f32 {
        meter_ratio(self)
    }
}

/// Semantic fill band for threshold-styled meters (extras Tokens demo).
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum MeterFillTone {
    /// Below the warning threshold — theme primary / custom default.
    #[default]
    Default,
    /// Above the warning ratio but below the maximum.
    Warning,
    /// At (or above) the maximum.
    Danger,
}

/// Sanitizes a meter scalar: non-finite values become `0.0`.
#[must_use]
pub fn sanitize_scalar(value: f32) -> f32 {
    if value.is_finite() { value } else { 0.0 }
}

/// Returns ordered bounds with a positive finite span.
///
/// When `max <= min` or either bound is non-finite, falls back to `(0.0, 1.0)`
/// so callers never divide by zero.
#[must_use]
pub fn sanitize_bounds(min: f32, max: f32) -> (f32, f32) {
    let min = sanitize_scalar(min);
    let max = sanitize_scalar(max);
    if max > min { (min, max) } else { (0.0, 1.0) }
}

/// Clamps `value` into `[min, max]` after sanitizing non-finite input.
#[must_use]
pub fn clamp_meter_value(value: f32, min: f32, max: f32) -> f32 {
    let (min, max) = sanitize_bounds(min, max);
    sanitize_scalar(value).clamp(min, max)
}

/// Maps a meter config to a fill ratio using `(value - min) / (max - min)`.
#[must_use]
pub fn meter_ratio(config: MeterConfig) -> f32 {
    let (min, max) = config.ordered_bounds();
    fraction(config.value, min, max)
}

/// Picks a fill tone using the extras Tokens thresholds.
///
/// - [`MeterFillTone::Danger`] when `value >= max`
/// - [`MeterFillTone::Warning`] when the ratio is strictly above `warning_ratio`
/// - [`MeterFillTone::Default`] otherwise
///
/// A non-finite or out-of-range `warning_ratio` falls back to
/// [`WARNING_RATIO`] (`0.75`).
#[must_use]
pub fn meter_fill_tone(config: MeterConfig, warning_ratio: f32) -> MeterFillTone {
    let (min, max) = config.ordered_bounds();
    let value = clamp_meter_value(config.value, min, max);
    if value >= max {
        return MeterFillTone::Danger;
    }

    let warning = if warning_ratio.is_finite() && (0.0..1.0).contains(&warning_ratio) {
        warning_ratio
    } else {
        WARNING_RATIO
    };

    if meter_ratio(config) > warning {
        MeterFillTone::Warning
    } else {
        MeterFillTone::Default
    }
}

/// Formats `"{value}/{max}"` with no fractional digits (extras Tokens label).
#[must_use]
pub fn meter_value_label(config: MeterConfig) -> String {
    let value = config.clamped_value();
    let (_, max) = config.ordered_bounds();
    format!("{:.0}/{:.0}", value, max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_match_bits_ui() {
        let config = MeterConfig::new();
        assert_eq!(config.value, 0.0);
        assert_eq!(config.min, 0.0);
        assert_eq!(config.max, 100.0);
        assert_eq!(meter_ratio(config), 0.0);
    }

    #[test]
    fn ratio_respects_min_max() {
        let config = MeterConfig::new().min(10.0).max(50.0).value(30.0);
        assert!((meter_ratio(config) - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn invalid_bounds_are_safe() {
        let config = MeterConfig::new().min(10.0).max(10.0).value(5.0);
        assert_eq!(config.ordered_bounds(), (0.0, 1.0));
        // Value 5.0 maps onto the repaired `[0, 1]` span as full.
        assert_eq!(meter_ratio(config), 1.0);

        let nan = MeterConfig::new().value(f32::NAN).max(f32::NAN);
        assert_eq!(nan.clamped_value(), 0.0);
    }

    #[test]
    fn fill_tone_matches_extras_demo() {
        let base = MeterConfig::new().max(100.0);
        assert_eq!(
            meter_fill_tone(base.value(50.0), WARNING_RATIO),
            MeterFillTone::Default
        );
        assert_eq!(
            meter_fill_tone(base.value(76.0), WARNING_RATIO),
            MeterFillTone::Warning
        );
        assert_eq!(
            meter_fill_tone(base.value(100.0), WARNING_RATIO),
            MeterFillTone::Danger
        );
        assert_eq!(
            meter_fill_tone(base.value(100.0), f32::NAN),
            MeterFillTone::Danger
        );
    }

    #[test]
    fn value_label_is_integer_pair() {
        assert_eq!(
            meter_value_label(MeterConfig::new().value(42.4).max(100.0)),
            "42/100"
        );
    }
}
