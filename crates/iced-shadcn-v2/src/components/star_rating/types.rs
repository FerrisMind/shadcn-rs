//! Public configuration, style, and state types for the star-rating component.

use crate::iced_compat::Color;

/// Axis a [`super::StarRating`] lays out along.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StarRatingOrientation {
    /// Stars run left-to-right (or right-to-left when direction is RTL).
    #[default]
    Horizontal,
    /// Stars run top-to-bottom.
    Vertical,
}

impl StarRatingOrientation {
    /// Whether the rating runs top to bottom.
    #[must_use]
    pub const fn is_vertical(self) -> bool {
        matches!(self, Self::Vertical)
    }
}

/// Per-star footprint presets mirroring common Tailwind `size-*` classes.
///
/// The extras demos use `size-5` (default) and `size-10` (custom size).
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StarRatingSize {
    /// 16 px (`size-4`).
    Sm,
    /// 20 px (`size-5`) — default in shadcn-svelte-extras.
    #[default]
    Default,
    /// 24 px (`size-6`).
    Md,
    /// 32 px (`size-8`).
    Lg,
    /// 40 px (`size-10`) — custom-size demo.
    Xl,
    /// Explicit edge length in logical pixels (clamped to at least 1 px).
    Custom(f32),
}

impl StarRatingSize {
    /// Edge length in logical pixels.
    #[must_use]
    pub fn pixels(self) -> f32 {
        match self {
            Self::Sm => 16.0,
            Self::Default => 20.0,
            Self::Md => 24.0,
            Self::Lg => 32.0,
            Self::Xl => 40.0,
            Self::Custom(value) => value.max(1.0),
        }
    }
}

/// Interaction state a [`super::StarRating`] is styled for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct StarRatingStatus {
    /// Whether the cursor is over the control.
    pub hovered: bool,
    /// Whether interaction is suppressed.
    pub disabled: bool,
    /// Whether the control is read-only.
    pub readonly: bool,
    /// Whether the application marked the control as focused.
    pub focused: bool,
}

/// Resolved colors a [`super::StarRating`] paints for one status.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StarRatingStyle {
    /// Fill / stroke color of the stars (`text-primary` by default).
    pub foreground: Color,
    /// Focus ring color (`ring-ring`).
    pub ring: Color,
    /// Opacity applied to the whole group when disabled.
    pub opacity: f32,
}

/// Internal canvas state for hover preview.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct StarRatingState {
    /// Hover preview rating, or `None` when not previewing.
    pub(super) hover_value: Option<f32>,
    /// Index of the star under the cursor, if any.
    pub(super) hovered_index: Option<usize>,
}
