//! Public configuration and builder types for the meter component.

use std::time::Duration;

use crate::iced_compat::widget::canvas;
use crate::iced_compat::{Color, Element, Length};
use shadcn_common::{
    AccentColor, MeterConfig, MeterFillTone, TransitionValue, clamp_meter_value, meter_fill_tone,
    meter_ratio, sanitize_meter_bounds, sanitize_meter_scalar,
};

use crate::theme::Theme;

/// Preset thickness for a [`Meter`] bar.
///
/// The `Default` preset follows the extras `h-2` (8 px) recipe. Use
/// [`MeterSize::Custom`] or [`Meter::height`] when a layout needs an exact size.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MeterSize {
    /// 2 px (`h-0.5`).
    Xs,
    /// 4 px (`h-1`).
    Sm,
    /// Extras default (`h-2` → 8 px).
    #[default]
    Default,
    /// 12 px (`h-3`).
    Lg,
    /// 16 px (`h-4`).
    Xl,
    /// Custom thickness in logical pixels (clamped to at least 1 px).
    Custom(f32),
}

/// Axis along which a [`Meter`] fills.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MeterOrientation {
    /// Fill from left to right.
    #[default]
    Horizontal,
    /// Fill from bottom to top.
    Vertical,
}

/// Corner-radius preset for a [`Meter`] track and indicator.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MeterRadius {
    /// Square corners.
    None,
    /// The theme's small radius slot.
    Small,
    /// The theme's medium radius slot.
    Medium,
    /// The theme's large radius slot.
    Large,
    /// A pill radius capped to half the smallest dimension (`rounded-full`).
    #[default]
    Full,
    /// An explicit radius in logical pixels.
    Custom(f32),
}

/// Theme-aware meter based on shadcn-svelte-extras `Meter` / bits-ui `Meter`.
///
/// Displays a **static measurement** within `[min, max]` (CPU, tokens, battery),
/// not task completion — use [`crate::Progress`] for that. The track is the
/// indicator color at 20% opacity (`bg-(--meter-background)/20`); the fill uses
/// the full indicator color. Optional [`MeterFillTone`] mirrors the extras
/// Tokens demo thresholds (warning above 75%, danger at max).
///
/// Extras ships no Meter style-pack variants. Pick Rhea / Nova / … on the shared
/// [`Theme`] — Meter paints with that pack's primary / destructive / accents,
/// the same way Form defers look to Label / Input / Button recipes.
///
/// ```rust,no_run
/// use iced::{Element, Length};
/// use iced_shadcn_v2::{Meter, StyleId, Theme};
///
/// #[derive(Debug, Clone)]
/// enum Message {}
///
/// fn view(theme: &Theme) -> Element<'_, Message> {
///     // `theme.with_style(StyleId::Rhea)` → Rhea primary/destructive on this bar.
///     Meter::new(theme)
///         .value(50.0)
///         .max(100.0)
///         .width(Length::Fixed(200.0))
///         .into()
/// }
/// ```
#[derive(Clone, Copy, Debug)]
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct Meter<'a> {
    pub(super) theme: &'a Theme,
    pub(super) value: f32,
    pub(super) min: f32,
    pub(super) max: f32,
    pub(super) size: MeterSize,
    pub(super) orientation: MeterOrientation,
    pub(super) color: Option<AccentColor>,
    pub(super) custom_indicator_color: Option<Color>,
    pub(super) track_color: Option<Color>,
    pub(super) tone: Option<MeterFillTone>,
    pub(super) auto_tone: bool,
    pub(super) warning_ratio: f32,
    pub(super) radius: Option<MeterRadius>,
    pub(super) high_contrast: bool,
    pub(super) animated: bool,
    pub(super) width: Option<Length>,
    pub(super) height: Option<Length>,
    pub(super) transition_duration: Duration,
}

impl<'a> Meter<'a> {
    /// Creates a meter at `0` within `[0, 100]` using the active theme.
    pub fn new(theme: &'a Theme) -> Self {
        let recipe = theme.style.meter();
        Self {
            theme,
            value: 0.0,
            min: 0.0,
            max: 100.0,
            size: MeterSize::Default,
            orientation: MeterOrientation::default(),
            color: None,
            custom_indicator_color: None,
            track_color: None,
            tone: None,
            auto_tone: false,
            warning_ratio: recipe.warning_ratio,
            radius: None,
            high_contrast: false,
            animated: true,
            width: None,
            height: None,
            transition_duration: Duration::from_millis(u64::from(recipe.transition_ms)),
        }
    }

    /// Sets the current measurement. Non-finite values become `0`.
    pub fn value(mut self, value: f32) -> Self {
        self.value = sanitize_meter_scalar(value);
        self
    }

    /// Sets the lower bound (`aria-valuemin`).
    pub fn min(mut self, min: f32) -> Self {
        self.min = sanitize_meter_scalar(min);
        self
    }

    /// Sets the upper bound (`aria-valuemax`).
    ///
    /// Non-finite or non-positive spans are repaired at render time via
    /// [`sanitize_meter_bounds`].
    pub fn max(mut self, max: f32) -> Self {
        self.max = sanitize_meter_scalar(max);
        self
    }

    /// Sets the preset thickness.
    pub fn size(mut self, size: MeterSize) -> Self {
        self.size = size;
        self
    }

    /// Sets the fill direction.
    pub fn orientation(mut self, orientation: MeterOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Uses a theme accent's primary color for the indicator.
    pub fn color(mut self, color: AccentColor) -> Self {
        self.color = Some(color);
        self.custom_indicator_color = None;
        self.tone = None;
        self
    }

    /// Uses the theme primary color for the indicator.
    pub fn theme_primary(mut self) -> Self {
        self.color = None;
        self.custom_indicator_color = None;
        self.tone = None;
        self
    }

    /// Uses an explicit iced color for the indicator (`--meter-background`).
    pub fn custom_color(mut self, color: Color) -> Self {
        self.custom_indicator_color = Some(color);
        self.color = None;
        self.tone = None;
        self
    }

    /// Uses an explicit iced color for the track (overrides `/20` derivation).
    pub fn track_color(mut self, color: Color) -> Self {
        self.track_color = Some(color);
        self
    }

    /// Forces a fill tone (extras Tokens demo: Default / Warning / Danger).
    pub fn tone(mut self, tone: MeterFillTone) -> Self {
        self.tone = Some(tone);
        self.auto_tone = false;
        self.color = None;
        self.custom_indicator_color = None;
        self
    }

    /// Derives the fill tone from the current value using `warning_ratio`.
    pub fn auto_tone(mut self, enabled: bool) -> Self {
        self.auto_tone = enabled;
        if enabled {
            self.tone = None;
        }
        self
    }

    /// Sets the warning threshold fraction used by [`Self::auto_tone`].
    pub fn warning_ratio(mut self, ratio: f32) -> Self {
        self.warning_ratio = if ratio.is_finite() {
            ratio.clamp(0.0, 1.0)
        } else {
            self.theme.style.meter().warning_ratio
        };
        self
    }

    /// Forces the indicator color to full opacity.
    pub fn high_contrast(mut self, high_contrast: bool) -> Self {
        self.high_contrast = high_contrast;
        self
    }

    /// Sets the corner radius.
    pub fn radius(mut self, radius: MeterRadius) -> Self {
        self.radius = Some(radius);
        self
    }

    /// Sets the preferred widget width.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = Some(width.into());
        self
    }

    /// Sets the preferred widget height.
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = Some(height.into());
        self
    }

    /// Enables or disables the value transition animation.
    pub fn animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }

    /// Sets the determinate value transition duration.
    pub fn transition_duration(mut self, duration: Duration) -> Self {
        self.transition_duration = normalize_duration(duration);
        self
    }

    /// Sets the transition duration in milliseconds.
    pub fn transition_duration_ms(self, duration_ms: u32) -> Self {
        self.transition_duration(Duration::from_millis(u64::from(duration_ms)))
    }

    /// Backend-agnostic range snapshot for shared helpers / egui parity.
    pub fn config(&self) -> MeterConfig {
        MeterConfig {
            value: self.value,
            min: self.min,
            max: self.max,
        }
    }

    /// Normalized fill ratio in `0.0..=1.0`.
    #[must_use]
    pub fn ratio(&self) -> f32 {
        meter_ratio(self.config())
    }

    /// Resolved fill tone after `tone` / `auto_tone` knobs.
    #[must_use]
    pub fn resolved_tone(&self) -> MeterFillTone {
        if let Some(tone) = self.tone {
            return tone;
        }
        if self.auto_tone {
            return meter_fill_tone(self.config(), self.warning_ratio);
        }
        MeterFillTone::Default
    }

    /// Converts the builder into an iced canvas widget.
    pub fn into_canvas<Message>(self) -> canvas::Canvas<Self, Message> {
        let (width, height) = super::geometry::resolved_dimensions(
            self.theme,
            self.size,
            self.orientation,
            self.width,
            self.height,
        );

        canvas::Canvas::new(self).width(width).height(height)
    }
}

/// Wraps a [`Meter`] program into an iced canvas widget.
pub fn meter<Message>(meter: Meter<'_>) -> canvas::Canvas<Meter<'_>, Message> {
    meter.into_canvas()
}

impl<'a, Message: 'a> From<Meter<'a>> for Element<'a, Message> {
    fn from(meter: Meter<'a>) -> Self {
        meter.into_canvas().into()
    }
}

/// Per-instance animation state for the meter canvas program.
#[derive(Debug, Default)]
#[doc(hidden)]
pub struct MeterState {
    pub(super) initialized: bool,
    pub(super) target_ratio: f32,
    pub(super) transition: TransitionValue,
}

#[allow(dead_code)]
pub(super) fn clamped_value(meter: &Meter<'_>) -> f32 {
    let (min, max) = sanitize_meter_bounds(meter.min, meter.max);
    clamp_meter_value(meter.value, min, max)
}

fn normalize_duration(duration: Duration) -> Duration {
    if duration.is_zero() {
        Duration::from_millis(1)
    } else {
        duration
    }
}
