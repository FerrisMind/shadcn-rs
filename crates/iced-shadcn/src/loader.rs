use iced::{Color, Element};

use crate::spinner::{Spinner, SpinnerSize, SpinnerVariant, spinner};
use crate::theme::Theme;

#[derive(Clone, Copy, Debug)]
pub struct LoaderProps {
    pub size: f32,
    pub color: Option<Color>,
    pub duration_ms: u32,
}

impl Default for LoaderProps {
    fn default() -> Self {
        Self {
            size: 16.0,
            color: None,
            duration_ms: 1000,
        }
    }
}

impl LoaderProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size.max(1.0);
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    pub fn duration_ms(mut self, duration_ms: u32) -> Self {
        self.duration_ms = duration_ms.max(1);
        self
    }
}

#[derive(Clone, Copy, Debug)]
pub struct LoaderIconProps {
    pub size: f32,
    pub color: Option<Color>,
}

impl Default for LoaderIconProps {
    fn default() -> Self {
        Self {
            size: 16.0,
            color: None,
        }
    }
}

impl LoaderIconProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size.max(1.0);
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum PromptLoaderVariant {
    #[default]
    Circular,
    Classic,
    Pulse,
    PulseDot,
    Dots,
    Typing,
    Wave,
    Bars,
    Terminal,
    TextBlink,
    TextShimmer,
    LoadingDots,
}

impl PromptLoaderVariant {
    pub const ALL: [Self; 12] = [
        Self::Circular,
        Self::Classic,
        Self::Pulse,
        Self::PulseDot,
        Self::Dots,
        Self::Typing,
        Self::Wave,
        Self::Bars,
        Self::Terminal,
        Self::TextBlink,
        Self::TextShimmer,
        Self::LoadingDots,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::Circular => "circular",
            Self::Classic => "classic",
            Self::Pulse => "pulse",
            Self::PulseDot => "pulse-dot",
            Self::Dots => "dots",
            Self::Typing => "typing",
            Self::Wave => "wave",
            Self::Bars => "bars",
            Self::Terminal => "terminal",
            Self::TextBlink => "text-blink",
            Self::TextShimmer => "text-shimmer",
            Self::LoadingDots => "loading-dots",
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub enum PromptLoaderSize {
    Sm,
    #[allow(dead_code)]
    #[default]
    Md,
    Lg,
    /// Explicit pixel size — use when Sm/Md/Lg don't fit your layout.
    Custom(f32),
}

impl PartialEq for PromptLoaderSize {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Self::Sm, Self::Sm) | (Self::Md, Self::Md) | (Self::Lg, Self::Lg)
        ) || matches!((self, other), (Self::Custom(a), Self::Custom(b)) if (a - b).abs() < f32::EPSILON)
    }
}

impl Eq for PromptLoaderSize {}

impl PromptLoaderSize {
    fn pixels(self) -> f32 {
        match self {
            Self::Sm => 16.0,
            Self::Md => 20.0,
            Self::Lg => 24.0,
            Self::Custom(px) => px.max(1.0),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct PromptLoaderProps {
    pub variant: PromptLoaderVariant,
    pub size: PromptLoaderSize,
    pub color: Option<Color>,
    pub duration_ms: u32,
    /// Per-bar amplitude values for Wave/Bars variants (0.0–1.0 each).
    /// When `Some`, drives the bars from real audio rather than animation.
    pub amplitudes: Option<[f32; 5]>,
}

impl Default for PromptLoaderProps {
    fn default() -> Self {
        Self {
            variant: PromptLoaderVariant::Circular,
            size: PromptLoaderSize::Md,
            color: None,
            duration_ms: 1200,
            amplitudes: None,
        }
    }
}

impl PromptLoaderProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn variant(mut self, variant: PromptLoaderVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: PromptLoaderSize) -> Self {
        self.size = size;
        self
    }

    /// Shorthand for `.size(PromptLoaderSize::Custom(px))`.
    pub fn custom_size(mut self, px: f32) -> Self {
        self.size = PromptLoaderSize::Custom(px.max(1.0));
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    pub fn duration_ms(mut self, duration_ms: u32) -> Self {
        self.duration_ms = duration_ms.max(1);
        self
    }

    /// Set per-bar amplitude values for Wave/Bars variants.
    /// Values are clamped to `[0.0, 1.0]`. Drives bar heights from real audio.
    pub fn amplitudes(mut self, amps: [f32; 5]) -> Self {
        self.amplitudes = Some(amps);
        self
    }
}

pub fn loader<'a, Message: 'a>(props: LoaderProps, theme: &Theme) -> Element<'a, Message> {
    let mut model = Spinner::new(theme)
        .size(SpinnerSize::Custom(props.size))
        .variant(SpinnerVariant::AiLoaderIcon)
        .animated(true)
        .duration_ms(props.duration_ms);

    model = model.color(props.color.unwrap_or(theme.palette.foreground));

    spinner(model).into()
}

pub fn loader_icon<'a, Message: 'a>(props: LoaderIconProps, theme: &Theme) -> Element<'a, Message> {
    let mut model = Spinner::new(theme)
        .size(SpinnerSize::Custom(props.size))
        .variant(SpinnerVariant::AiLoaderIcon)
        .animated(false)
        .progress(0.0)
        .loading(true);

    model = model.color(props.color.unwrap_or(theme.palette.foreground));

    spinner(model).into()
}

pub fn prompt_loader<'a, Message: 'a>(
    props: PromptLoaderProps,
    theme: &Theme,
) -> Element<'a, Message> {
    let variant = match props.variant {
        PromptLoaderVariant::Circular => SpinnerVariant::PromptCircular,
        PromptLoaderVariant::Classic => SpinnerVariant::PromptClassic,
        PromptLoaderVariant::Pulse => SpinnerVariant::PromptPulse,
        PromptLoaderVariant::PulseDot => SpinnerVariant::PromptPulseDot,
        PromptLoaderVariant::Dots => SpinnerVariant::PromptDots,
        PromptLoaderVariant::Typing => SpinnerVariant::PromptTyping,
        PromptLoaderVariant::Wave => SpinnerVariant::PromptWave,
        PromptLoaderVariant::Bars => SpinnerVariant::PromptBars,
        PromptLoaderVariant::Terminal => SpinnerVariant::PromptTerminal,
        PromptLoaderVariant::TextBlink => SpinnerVariant::PromptTextBlink,
        PromptLoaderVariant::TextShimmer => SpinnerVariant::PromptTextShimmer,
        PromptLoaderVariant::LoadingDots => SpinnerVariant::PromptLoadingDots,
    };

    let mut model = Spinner::new(theme)
        .size(SpinnerSize::Custom(props.size.pixels()))
        .variant(variant)
        .animated(true)
        .duration_ms(props.duration_ms);

    model = model.color(props.color.unwrap_or(theme.palette.primary));

    if let Some(amps) = props.amplitudes {
        model = model.amplitudes(amps);
    }

    spinner(model).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loader_props_defaults_match_reference() {
        let props = LoaderProps::new();
        assert!((props.size - 16.0).abs() < f32::EPSILON);
        assert_eq!(props.color, None);
        assert_eq!(props.duration_ms, 1000);
    }

    #[test]
    fn loader_icon_props_defaults_match_reference() {
        let props = LoaderIconProps::new();
        assert!((props.size - 16.0).abs() < f32::EPSILON);
        assert_eq!(props.color, None);
    }

    #[test]
    fn prompt_loader_defaults_match_reference() {
        let props = PromptLoaderProps::new();
        assert_eq!(props.variant, PromptLoaderVariant::Circular);
        assert_eq!(props.size, PromptLoaderSize::Md);
        assert_eq!(props.color, None);
        assert_eq!(props.duration_ms, 1200);
    }
}
