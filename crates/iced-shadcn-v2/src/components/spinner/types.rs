//! Public configuration and state types for the spinner component.

use iced::widget::canvas;
use iced::{Color, Length, Size};

use crate::theme::Theme;

/// Animation style of a [`Spinner`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SpinnerVariant {
    /// Eight-spoke legacy Lucide spinner.
    #[default]
    LegacyLucide,
    /// Ten-segment AI loader icon.
    AiLoaderIcon,
    /// Dot orbiting a circle.
    PromptCircular,
    /// Classic twelve-spoke fading spinner.
    PromptClassic,
    /// Pulsing ring.
    PromptPulse,
    /// Pulsing dot.
    PromptPulseDot,
    /// Three bouncing dots.
    PromptDots,
    /// Typing-indicator dots.
    PromptTyping,
    /// Five-bar audio wave.
    PromptWave,
    /// Three-bar audio wave.
    PromptBars,
    /// Terminal prompt with a blinking cursor.
    PromptTerminal,
    /// Blinking "Thinking" text.
    PromptTextBlink,
    /// Shimmering "Thinking" text.
    PromptTextShimmer,
    /// "Loading" text with animated dots.
    PromptLoadingDots,
}

/// Preset (or custom pixel) size of a [`Spinner`].
///
/// ```rust
/// use iced_shadcn_v2::SpinnerSize;
///
/// let size = SpinnerSize::Custom(24.0);
/// assert_ne!(size, SpinnerSize::Size2);
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SpinnerSize {
    /// 12 px.
    Size1,
    /// 16 px.
    Size2,
    /// 20 px.
    Size3,
    /// Custom size in pixels (clamped to at least 1 px).
    Custom(f32),
}

impl SpinnerSize {
    pub(super) fn pixels(self) -> f32 {
        match self {
            SpinnerSize::Size1 => 12.0,
            SpinnerSize::Size2 => 16.0,
            SpinnerSize::Size3 => 20.0,
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
/// let indicator = Spinner::new(&theme).size(SpinnerSize::Size3).animated(true);
/// ```
#[derive(Clone, Copy, Debug)]
pub struct Spinner {
    pub(super) progress: f32,
    pub(super) color: Color,
    pub(super) size: SpinnerSize,
    pub(super) loading: bool,
    pub(super) animated: bool,
    pub(super) duration_ms: u32,
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
            size: SpinnerSize::Size2,
            loading: true,
            animated: false,
            duration_ms: 1000,
            variant: SpinnerVariant::AiLoaderIcon,
            amplitudes: None,
        }
    }

    /// Sets the externally-driven progress (used when not animated).
    pub fn progress(mut self, progress: f32) -> Self {
        self.progress = progress;
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
    pub fn duration_ms(mut self, duration_ms: u32) -> Self {
        self.duration_ms = duration_ms.max(1);
        self
    }

    /// Sets the animation style.
    pub fn variant(mut self, variant: SpinnerVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set per-bar amplitude values for Wave/Bars variants.
    ///
    /// Each value in `[f32; 5]` is clamped to `[0.0, 1.0]` and maps to one bar.
    /// When set, real audio amplitudes are used instead of the time-driven sine wave.
    /// For the `PromptBars` variant (3 bars) only the first 3 values are used.
    pub fn amplitudes(mut self, amps: [f32; 5]) -> Self {
        self.amplitudes = Some(amps.map(|a| a.clamp(0.0, 1.0)));
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
            SpinnerVariant::PromptTerminal => Size::new(size * 2.4, size),
            SpinnerVariant::PromptTextBlink
            | SpinnerVariant::PromptTextShimmer
            | SpinnerVariant::PromptLoadingDots => Size::new(size * 4.8, size * 1.2),
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

/// Internal animation state of a [`Spinner`] canvas program.
#[derive(Debug, Default)]
pub struct SpinnerState {
    pub(super) start_time: Option<iced::time::Instant>,
    pub(super) phase: f32,
}
