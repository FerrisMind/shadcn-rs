//! Public configuration and builder types for the progress component.

use std::time::Duration;

use crate::iced_compat::widget::canvas;
use crate::iced_compat::{Color, Element, Length};
use shadcn_common::{AccentColor, TransitionValue};

use crate::theme::Theme;

/// Preset thickness for a [`Progress`] bar.
///
/// The `Default` preset follows the active shadcn-svelte style pack. Use
/// [`ProgressSize::Custom`] or [`Progress::height`] when a layout needs an
/// exact CSS-like height.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ProgressSize {
    /// 2 px (`h-0.5`).
    Xs,
    /// 4 px (`h-1`).
    Sm,
    /// Style-pack default (`h-0.5` to `h-3`).
    #[default]
    Default,
    /// 8 px (`h-2`).
    Lg,
    /// 12 px (`h-3`).
    Xl,
    /// Custom thickness in logical pixels (clamped to at least 1 px).
    Custom(f32),
}

/// Visual treatment of a [`Progress`] bar.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ProgressVariant {
    /// The default shadcn-svelte `bg-muted` track and `bg-primary` indicator.
    Default,
    /// Flat muted track with a theme-primary indicator.
    Classic,
    /// Style-pack surface treatment. This is the default for the builder and
    /// preserves the shadcn-svelte progress colors.
    #[default]
    Surface,
    /// A low-contrast accent-tinted track with a theme-primary indicator.
    Soft,
}

/// Axis along which a [`Progress`] bar fills.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ProgressOrientation {
    /// Fill from left to right.
    #[default]
    Horizontal,
    /// Fill from bottom to top.
    Vertical,
}

/// Corner-radius preset for a [`Progress`] track and indicator.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ProgressRadius {
    /// Square corners.
    None,
    /// The theme's small radius slot.
    Small,
    /// The theme's medium radius slot.
    Medium,
    /// The theme's large radius slot.
    Large,
    /// A pill radius capped to half the smallest dimension.
    #[default]
    Full,
    /// An explicit radius in logical pixels.
    Custom(f32),
}

/// Theme-aware progress indicator based on shadcn-svelte's `Progress`.
///
/// The builder supports determinate values, an indeterminate animated state,
/// custom dimensions and colors, style-pack geometry, horizontal/vertical
/// orientation, and an optional smooth transition when the value changes.
/// Values are normalized against `max` and clamped to the visible range at
/// render time.
///
/// An animated determinate bar only self-schedules redraws while a value
/// transition is in flight; once settled, the next frame is driven by the
/// application. Do not rely on the bar as a continuous redraw "heartbeat" —
/// only the indeterminate state animates continuously.
///
/// ```rust,no_run
/// use iced::{Element, Length};
/// use iced_shadcn_v2::{Progress, ProgressSize, Theme};
///
/// #[derive(Debug, Clone)]
/// enum Message {}
///
/// fn view(theme: &Theme) -> Element<'_, Message> {
///     Progress::new(theme)
///         .value(66.0)
///         .size(ProgressSize::Lg)
///         .width(Length::Fixed(320.0))
///         .into()
/// }
/// ```
#[derive(Clone, Copy, Debug)]
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct Progress<'a> {
    pub(super) theme: &'a Theme,
    pub(super) value: Option<f32>,
    pub(super) max: f32,
    pub(super) size: ProgressSize,
    pub(super) variant: ProgressVariant,
    pub(super) orientation: ProgressOrientation,
    pub(super) color: Option<AccentColor>,
    pub(super) custom_indicator_color: Option<Color>,
    pub(super) track_color: Option<Color>,
    pub(super) radius: Option<ProgressRadius>,
    pub(super) high_contrast: bool,
    pub(super) animated: bool,
    pub(super) width: Option<Length>,
    pub(super) height: Option<Length>,
    pub(super) transition_duration: Duration,
    pub(super) indeterminate_duration: Duration,
}

impl<'a> Progress<'a> {
    /// Creates an indeterminate progress bar using the active theme.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            value: None,
            max: 100.0,
            size: ProgressSize::Default,
            variant: ProgressVariant::default(),
            orientation: ProgressOrientation::default(),
            color: None,
            custom_indicator_color: None,
            track_color: None,
            radius: None,
            high_contrast: false,
            animated: true,
            width: None,
            height: None,
            transition_duration: Duration::from_millis(150),
            indeterminate_duration: Duration::from_millis(1200),
        }
    }

    /// Sets a determinate value. Non-finite values are treated as zero.
    pub fn value(mut self, value: f32) -> Self {
        self.value = Some(sanitize_value(value));
        self
    }

    /// Sets a determinate value or clears it for an indeterminate bar.
    pub fn value_maybe(mut self, value: Option<f32>) -> Self {
        self.value = value.map(sanitize_value);
        self
    }

    /// Switches the bar to the animated indeterminate state.
    pub fn indeterminate(mut self) -> Self {
        self.value = None;
        self
    }

    /// Sets the upper bound used to normalize the value.
    ///
    /// Non-finite or non-positive bounds are normalized to `1.0`, keeping the
    /// component renderable instead of allowing a division-by-zero path.
    pub fn max(mut self, max: f32) -> Self {
        self.max = sanitize_max(max);
        self
    }

    /// Sets the preset thickness.
    pub fn size(mut self, size: ProgressSize) -> Self {
        self.size = size;
        self
    }

    /// Sets the visual treatment.
    pub fn variant(mut self, variant: ProgressVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Sets the fill direction.
    pub fn orientation(mut self, orientation: ProgressOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Uses a theme accent's primary color for the indicator.
    pub fn color(mut self, color: AccentColor) -> Self {
        self.color = Some(color);
        self.custom_indicator_color = None;
        self
    }

    /// Uses the theme primary color for the indicator.
    pub fn theme_primary(mut self) -> Self {
        self.color = None;
        self.custom_indicator_color = None;
        self
    }

    /// Uses an explicit iced color for the indicator.
    pub fn custom_color(mut self, color: Color) -> Self {
        self.custom_indicator_color = Some(color);
        self.color = None;
        self
    }

    /// Uses an explicit iced color for the track.
    pub fn track_color(mut self, color: Color) -> Self {
        self.track_color = Some(color);
        self
    }

    /// Forces the indicator color to full opacity.
    pub fn high_contrast(mut self, high_contrast: bool) -> Self {
        self.high_contrast = high_contrast;
        self
    }

    /// Sets the corner radius.
    pub fn radius(mut self, radius: ProgressRadius) -> Self {
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

    /// Enables or disables internal animation.
    pub fn animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }

    /// Sets both the determinate transition and indeterminate cycle duration.
    pub fn duration(mut self, duration: Duration) -> Self {
        let duration = normalize_duration(duration);
        self.transition_duration = duration;
        self.indeterminate_duration = duration;
        self
    }

    /// Sets both animation durations in milliseconds.
    pub fn duration_ms(self, duration_ms: u32) -> Self {
        self.duration(Duration::from_millis(u64::from(duration_ms)))
    }

    /// Sets the determinate value transition duration.
    pub fn transition_duration(mut self, duration: Duration) -> Self {
        self.transition_duration = normalize_duration(duration);
        self
    }

    /// Sets the indeterminate animation cycle duration.
    pub fn indeterminate_duration(mut self, duration: Duration) -> Self {
        self.indeterminate_duration = normalize_duration(duration);
        self
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

/// Wraps a [`Progress`] program into an iced canvas widget.
pub fn progress<Message>(progress: Progress<'_>) -> canvas::Canvas<Progress<'_>, Message> {
    progress.into_canvas()
}

impl<'a, Message: 'a> From<Progress<'a>> for Element<'a, Message> {
    fn from(progress: Progress<'a>) -> Self {
        progress.into_canvas().into()
    }
}

/// Per-instance animation state for the progress canvas program.
#[derive(Debug, Default)]
#[doc(hidden)]
pub struct ProgressState {
    pub(super) start_time: Option<crate::iced_compat::time::Instant>,
    pub(super) phase: f32,
    pub(super) initialized: bool,
    pub(super) determinate: bool,
    pub(super) target_ratio: f32,
    pub(super) transition: TransitionValue,
}

fn sanitize_value(value: f32) -> f32 {
    if value.is_finite() { value } else { 0.0 }
}

fn sanitize_max(max: f32) -> f32 {
    if max.is_finite() && max > 0.0 {
        max
    } else {
        1.0
    }
}

fn normalize_duration(duration: Duration) -> Duration {
    if duration.is_zero() {
        Duration::from_millis(1)
    } else {
        duration
    }
}
