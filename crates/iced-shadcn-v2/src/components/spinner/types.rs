//! Public configuration and state types for the spinner component.

use std::time::Duration;

use crate::iced_compat::widget::canvas;
use crate::iced_compat::{Color, Element, Length, Size};

use crate::theme::Theme;

/// Animation style of a [`Spinner`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum SpinnerVariant {
    /// Eight-spoke legacy Lucide spinner.
    LegacyLucide,
    /// Ten-segment AI loader icon.
    #[default]
    AiLoaderIcon,
    /// Circular stroke with a gap (rotating arc).
    Circular,
    /// Classic twelve-spoke fading spinner.
    Classic,
    /// Pulsing ring.
    Pulse,
    /// Pulsing dot.
    PulseDot,
    /// Three bouncing dots.
    Dots,
    /// Typing-indicator dots.
    Typing,
    /// Five-bar audio wave.
    Wave,
    /// Three-bar audio wave.
    Bars,
    /// Terminal prompt with a blinking cursor.
    Terminal,
    /// Blinking "Thinking" text.
    TextBlink,
    /// Shimmering "Thinking" text.
    TextShimmer,
    /// "Loading" text with animated dots.
    LoadingDots,
}

/// Preset (or custom pixel) size of a [`Spinner`].
///
/// Presets mirror common Tailwind `size-*` classes used with shadcn-svelte’s
/// Spinner (`size-3` … `size-8`); there is no size prop in svelte — only `class`.
///
/// ```rust
/// use iced_shadcn_v2::SpinnerSize;
///
/// let size = SpinnerSize::Custom(24.0);
/// assert_ne!(size, SpinnerSize::Default);
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum SpinnerSize {
    /// 12 px (`size-3`).
    Xs,
    /// 16 px (`size-4`).
    Sm,
    /// 16 px — same footprint as shadcn’s default `size-4` class.
    #[default]
    Default,
    /// 24 px (`size-6`).
    Lg,
    /// 32 px (`size-8`).
    Xl,
    /// Custom size in pixels (clamped to at least 1 px).
    Custom(f32),
}

impl SpinnerSize {
    pub(super) fn pixels(self) -> f32 {
        match self {
            SpinnerSize::Xs => 12.0,
            SpinnerSize::Sm | SpinnerSize::Default => 16.0,
            SpinnerSize::Lg => 24.0,
            SpinnerSize::Xl => 32.0,
            SpinnerSize::Custom(value) => value.max(1.0),
        }
    }
}

/// Canvas-based loading indicator.
///
/// ```rust
/// use iced_shadcn_v2::{Spinner, SpinnerSize, Theme};
///
/// let theme = Theme::light();
/// let indicator = Spinner::new(&theme).size(SpinnerSize::Lg).animated(true);
/// ```
#[derive(Clone, Copy, Debug)]
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct Spinner {
    pub(super) progress: f32,
    pub(super) color: Color,
    pub(super) size: SpinnerSize,
    pub(super) loading: bool,
    pub(super) animated: bool,
    pub(super) duration: Duration,
    pub(super) variant: SpinnerVariant,
    /// Per-bar amplitude values for Wave/Bars variants (0.0–1.0 each).
    /// When `Some`, overrides the sine-wave animation with real audio levels.
    /// When `None`, falls back to phase-driven sine animation.
    pub(super) amplitudes: Option<[f32; 5]>,
}

impl Spinner {
    /// Spinner colored with the theme primary.
    pub fn new(theme: &Theme) -> Self {
        Self::from_color(theme.palette.primary)
    }

    /// Spinner with an explicit color.
    pub fn from_color(color: Color) -> Self {
        Self {
            progress: 0.0,
            color,
            size: SpinnerSize::Default,
            loading: true,
            animated: false,
            duration: Duration::from_millis(1000),
            variant: SpinnerVariant::default(),
            amplitudes: None,
        }
    }

    /// Sets the externally-driven progress (used when not animated).
    ///
    /// Non-finite values (`NaN`, `±inf`) are normalized to `0.0` so a bad
    /// division upstream (e.g. `done / 0`) cannot poison the geometry.
    pub fn progress(mut self, progress: f32) -> Self {
        self.progress = if progress.is_finite() { progress } else { 0.0 };
        self
    }

    /// Sets the spinner color.
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Sets the spinner size.
    pub fn size(mut self, size: SpinnerSize) -> Self {
        self.size = size;
        self
    }

    /// Shows or hides the spinner.
    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    /// Enables the internal time-driven animation.
    pub fn animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }

    /// Sets the duration of one animation cycle (clamped to at least 1 ms).
    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = duration.max(Duration::from_millis(1));
        self
    }

    /// Sets the animation style.
    pub fn variant(mut self, variant: SpinnerVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set per-bar amplitude values for Wave/Bars variants.
    ///
    /// Each value in `[f32; 5]` is clamped to `[0.0, 1.0]` and maps to one bar;
    /// non-finite values (`NaN`, `±inf`) are treated as silence (`0.0`).
    /// When set, real audio amplitudes are used instead of the time-driven sine wave.
    /// For the `Bars` variant (3 bars) only the first 3 values are used.
    pub fn amplitudes(mut self, amps: [f32; 5]) -> Self {
        self.amplitudes = Some(amps.map(|a| {
            if a.is_finite() {
                a.clamp(0.0, 1.0)
            } else {
                0.0
            }
        }));
        self
    }

    pub(super) fn resolved_progress(self, state: &SpinnerState) -> f32 {
        if self.animated {
            state.phase
        } else {
            self.progress
        }
    }

    pub(super) fn dimensions(self) -> Size {
        let size = self.size.pixels();
        match self.variant {
            SpinnerVariant::Terminal => Size::new(size * 2.4, size),
            SpinnerVariant::TextBlink
            | SpinnerVariant::TextShimmer
            | SpinnerVariant::LoadingDots => Size::new(size * 4.8, size * 1.2),
            _ => Size::new(size, size),
        }
    }
}

/// Wraps a [`Spinner`] program into a fixed-size canvas widget.
pub fn spinner<Message>(spinner: Spinner) -> canvas::Canvas<Spinner, Message> {
    let size = spinner.dimensions();
    canvas::Canvas::new(spinner)
        .width(Length::Fixed(size.width))
        .height(Length::Fixed(size.height))
}

impl<'a, Message: 'a> From<Spinner> for Element<'a, Message> {
    fn from(config: Spinner) -> Self {
        spinner(config).into()
    }
}

/// Internal animation state of a [`Spinner`] canvas program.
#[derive(Debug, Default)]
pub struct SpinnerState {
    pub(super) start_time: Option<crate::iced_compat::time::Instant>,
    pub(super) phase: f32,
}
