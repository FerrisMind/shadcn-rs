//! Public configuration types for the aspect-ratio component.

use std::fmt;

use crate::iced_compat::{Color, Element};

use crate::theme::Theme;

/// Minimum positive aspect ratio accepted by [`super::AspectRatio`].
///
/// Values at or below zero are clamped to this constant so layout never
/// divides by zero.
pub const MIN_ASPECT_RATIO: f32 = 0.000_1;

/// Layout wrapper that keeps child content within a fixed width-to-height ratio.
///
/// Port of shadcn-svelte `AspectRatio` (bits-ui `AspectRatio.Root`): the outer
/// box preserves `ratio` while arbitrary child content fills the interior.
/// Tailwind `class` overrides such as `bg-muted` or `rounded-lg` map to
/// [`Self::background`], [`Self::muted`], [`Self::radius`], and
/// [`Self::style_override`].
///
/// ```rust
/// use iced::widget::text;
/// use iced_shadcn_v2::{AspectRatio, Theme};
///
/// let theme = Theme::light();
/// let frame: AspectRatio<'_, ()> = AspectRatio::new(text("16:9"))
///     .ratio(16.0 / 9.0)
///     .muted(&theme);
/// ```
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct AspectRatio<'a, Message> {
    pub(super) content: Element<'a, Message>,
    pub(super) ratio: f32,
    pub(super) background: Option<Color>,
    pub(super) radius: f32,
    pub(super) clip: bool,
    pub(super) style_override: Option<
        Box<
            dyn Fn(
                    crate::iced_compat::widget::container::Style,
                ) -> crate::iced_compat::widget::container::Style
                + 'a,
        >,
    >,
}

impl<Message> fmt::Debug for AspectRatio<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AspectRatio")
            .field("ratio", &self.ratio)
            .field("background", &self.background)
            .field("radius", &self.radius)
            .field("clip", &self.clip)
            .field("style_override", &self.style_override.is_some())
            .finish_non_exhaustive()
    }
}

impl<'a, Message> AspectRatio<'a, Message> {
    /// Creates an aspect-ratio wrapper around arbitrary content.
    ///
    /// Defaults to a square box (`ratio = 1.0`) with no background fill.
    pub fn new(content: impl Into<Element<'a, Message>>) -> Self {
        Self {
            content: content.into(),
            ratio: 1.0,
            background: None,
            radius: 0.0,
            clip: false,
            style_override: None,
        }
    }

    /// Sets the desired width-to-height ratio.
    ///
    /// Mirrors the `ratio` prop of bits-ui `AspectRatio.Root` (default `1`).
    /// Non-positive values are clamped to [`MIN_ASPECT_RATIO`].
    pub fn ratio(mut self, ratio: f32) -> Self {
        self.ratio = Self::clamp_ratio(ratio);
        self
    }

    /// Sets an explicit background color for the outer wrapper.
    ///
    /// Equivalent to a `bg-*` Tailwind class on the web component.
    pub fn background(mut self, color: Color) -> Self {
        self.background = Some(color);
        self
    }

    /// Paints the wrapper with the theme `muted` surface (`bg-muted`).
    pub fn muted(mut self, theme: &Theme) -> Self {
        self.background = Some(theme.palette.muted);
        self
    }

    /// Sets the corner radius in pixels (clamped to at least 0 px).
    ///
    /// Rounded corners only clip child content when [`Self::clip`] is `true`.
    pub fn radius(mut self, radius: f32) -> Self {
        self.radius = radius.max(0.0);
        self
    }

    /// Clips child content that overflows the wrapper bounds.
    ///
    /// Mirrors `overflow-hidden` on the web primitive's outer container.
    /// Clipping is rectangular (the wrapper's layout bounds); rounded-corner
    /// clipping is not supported by iced's container yet, so [`Self::radius`]
    /// only rounds the painted background.
    pub fn clip(mut self, clip: bool) -> Self {
        self.clip = clip;
        self
    }

    /// Applies a narrow iced-style escape hatch after internal style resolution.
    ///
    /// The equivalent of passing a custom `class` / `style` to
    /// `AspectRatio.Root`.
    pub fn style_override(
        mut self,
        style_override: impl Fn(
            crate::iced_compat::widget::container::Style,
        ) -> crate::iced_compat::widget::container::Style
        + 'a,
    ) -> Self {
        self.style_override = Some(Box::new(style_override));
        self
    }

    /// Builds the aspect-ratio wrapper as an iced [`Element`](iced_core::Element).
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        super::render::aspect_ratio(self)
    }

    /// Returns the validated ratio used during layout.
    pub fn resolved_ratio(&self) -> f32 {
        Self::clamp_ratio(self.ratio)
    }

    fn clamp_ratio(ratio: f32) -> f32 {
        if ratio.is_finite() && ratio > MIN_ASPECT_RATIO {
            ratio
        } else {
            MIN_ASPECT_RATIO
        }
    }
}

impl<'a, Message> From<AspectRatio<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(config: AspectRatio<'a, Message>) -> Self {
        config.into_element()
    }
}
