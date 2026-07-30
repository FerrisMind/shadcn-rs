//! Public configuration and builder types for the skeleton component.

use std::time::Duration;

use crate::iced_compat::widget::canvas;
use crate::iced_compat::{Color, Element, Length};
use twill_core::prelude::theme::SemanticColor;

use crate::theme::Theme;

fn default_radius_for_theme(theme: &Theme) -> SkeletonRadius {
    match theme.style.skeleton_default_radius() {
        shadcn_common::ComponentRadius::None => SkeletonRadius::None,
        shadcn_common::ComponentRadius::Sm => SkeletonRadius::Small,
        shadcn_common::ComponentRadius::Md => SkeletonRadius::Medium,
        shadcn_common::ComponentRadius::Lg
        | shadcn_common::ComponentRadius::Xl
        | shadcn_common::ComponentRadius::S2xl
        | shadcn_common::ComponentRadius::S3xl
        | shadcn_common::ComponentRadius::S4xl => SkeletonRadius::Large,
        shadcn_common::ComponentRadius::Full => SkeletonRadius::Full,
        _ => SkeletonRadius::Medium,
    }
}

/// Animation used by a [`Skeleton`].
///
/// `Pulse` matches shadcn-svelte's `animate-pulse` default. `Static` is an
/// optional non-animated mode for callers that need a deterministic
/// placeholder.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SkeletonAnimation {
    /// Opacity pulse matching Tailwind's two-second pulse timing.
    #[default]
    Pulse,
    /// A non-animated placeholder.
    Static,
}

/// Corner-radius preset for a [`Skeleton`].
///
/// `Custom` is interpreted in logical pixels and clamped to the shape's
/// maximum usable radius at render time.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SkeletonRadius {
    /// Square corners.
    None,
    /// Small shadcn radius.
    Small,
    /// The theme's medium control radius (`rounded-md` by default).
    #[default]
    Medium,
    /// Large shadcn radius.
    Large,
    /// Fully rounded corners, capped to half the smallest dimension.
    Full,
    /// An explicit radius in logical pixels.
    Custom(f32),
}

/// Shape of a [`Skeleton`] placeholder.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SkeletonShape {
    /// A rectangle with the selected radius.
    Rounded(SkeletonRadius),
    /// A circle or capsule, capped by the smallest dimension.
    Circle,
}

impl Default for SkeletonShape {
    fn default() -> Self {
        // Vega default; concrete themes override via [`Skeleton::new`].
        Self::Rounded(SkeletonRadius::Medium)
    }
}

/// Fill source for a [`Skeleton`].
///
/// Semantic colors are preferred because they follow the active [`Theme`].
/// [`SkeletonFill::Custom`] is provided for intentionally bespoke placeholders.
///
/// Unlike the other configuration enums, this one does not derive serde
/// support under the `serde` feature: its variants wrap foreign types
/// (`iced` [`Color`], `twill-core` [`SemanticColor`]) that are not
/// serializable.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SkeletonFill {
    /// Resolve a semantic shadcn color from the active theme.
    Semantic(SemanticColor),
    /// Use an explicit iced color.
    Custom(Color),
}

impl Default for SkeletonFill {
    fn default() -> Self {
        Self::Semantic(SemanticColor::Muted)
    }
}

/// Theme-aware loading placeholder based on shadcn-svelte's `Skeleton`.
///
/// The default is a `Fill × 16 px` muted block with `rounded-md` corners and
/// the Tailwind-compatible pulse animation. Width, height, shape, fill, and
/// animation can be configured independently, so the component covers common
/// usages such as avatar, text-line, card, and media placeholders.
///
/// ```rust,no_run
/// use iced::{Element, Length};
/// use iced_shadcn_v2::{Skeleton, SkeletonShape, Theme};
///
/// #[derive(Debug, Clone)]
/// enum Message {}
///
/// fn view(theme: &Theme) -> Element<'_, Message> {
///     Skeleton::new(theme)
///         .size(Length::Fixed(48.0))
///         .shape(SkeletonShape::Circle)
///         .into()
/// }
/// ```
#[derive(Debug, Clone, Copy)]
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct Skeleton<'a> {
    pub(super) theme: &'a Theme,
    pub(super) width: Length,
    pub(super) height: Length,
    pub(super) shape: SkeletonShape,
    pub(super) fill: SkeletonFill,
    pub(super) animation: SkeletonAnimation,
    pub(super) duration: Duration,
}

impl<'a> Skeleton<'a> {
    /// Creates a skeleton using the theme's muted semantic color.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            width: Length::Fill,
            height: Length::Fixed(16.0),
            shape: SkeletonShape::Rounded(default_radius_for_theme(theme)),
            fill: SkeletonFill::default(),
            animation: SkeletonAnimation::default(),
            duration: Duration::from_secs(2),
        }
    }

    /// Sets the placeholder width.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the placeholder height.
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Sets equal width and height, useful for avatar placeholders.
    pub fn size(mut self, size: impl Into<Length>) -> Self {
        let size = size.into();
        self.width = size;
        self.height = size;
        self
    }

    /// Sets the placeholder shape.
    pub fn shape(mut self, shape: SkeletonShape) -> Self {
        self.shape = shape;
        self
    }

    /// Sets rounded corners using a preset or custom radius.
    pub fn radius(mut self, radius: SkeletonRadius) -> Self {
        self.shape = SkeletonShape::Rounded(radius);
        self
    }

    /// Makes the placeholder circular or capsule-shaped.
    pub fn circle(mut self) -> Self {
        self.shape = SkeletonShape::Circle;
        self
    }

    /// Selects the animation treatment.
    pub fn animation(mut self, animation: SkeletonAnimation) -> Self {
        self.animation = animation;
        self
    }

    /// Sets the animation cycle duration.
    ///
    /// A zero duration is normalized to one millisecond so the renderer never
    /// divides by zero.
    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = if duration.is_zero() {
            Duration::from_millis(1)
        } else {
            duration
        };
        self
    }

    /// Sets the animation cycle duration in milliseconds.
    pub fn duration_ms(self, duration_ms: u32) -> Self {
        self.duration(Duration::from_millis(u64::from(duration_ms)))
    }

    /// Uses a semantic color resolved from the active theme.
    pub fn color(mut self, color: SemanticColor) -> Self {
        self.fill = SkeletonFill::Semantic(color);
        self
    }

    /// Uses an explicit iced color as a low-level styling escape hatch.
    pub fn custom_color(mut self, color: Color) -> Self {
        self.fill = SkeletonFill::Custom(color);
        self
    }

    /// Converts the builder into the underlying iced canvas widget.
    pub fn into_canvas<Message>(self) -> canvas::Canvas<Self, Message> {
        let width = self.width;
        let height = self.height;
        canvas::Canvas::new(self).width(width).height(height)
    }
}

impl<'a, Message: 'a> From<Skeleton<'a>> for Element<'a, Message> {
    fn from(skeleton: Skeleton<'a>) -> Self {
        skeleton.into_canvas().into()
    }
}

/// Per-instance animation state for the skeleton canvas program.
#[derive(Debug, Default)]
#[doc(hidden)]
pub struct SkeletonState {
    pub(super) start_time: Option<crate::iced_compat::time::Instant>,
    pub(super) phase: f32,
}
