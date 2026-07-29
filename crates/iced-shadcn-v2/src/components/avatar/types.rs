//! Public configuration types for the avatar component.

/// Preset or custom footprint for an [`super::Avatar`].
///
/// The presets mirror shadcn-svelte's `data-size` contract: `sm` is 24px,
/// `default` is 32px, and `lg` is 40px. `Custom` covers the additional
/// `size-*` classes that can be applied to the web component.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AvatarSize {
    /// 24px (`size-6`).
    Sm,
    /// 32px (`size-8`).
    #[default]
    Default,
    /// 40px (`size-10`).
    Lg,
    /// A custom square footprint in pixels. Invalid values resolve to 1px.
    Custom(f32),
}

impl AvatarSize {
    pub(super) fn pixels(self) -> f32 {
        match self {
            Self::Sm => 24.0,
            Self::Default => 32.0,
            Self::Lg => 40.0,
            Self::Custom(value) if value.is_finite() => value.max(1.0),
            Self::Custom(_) => 1.0,
        }
    }
}

/// Corner-radius intent for an [`super::Avatar`].
///
/// Avatar roots are fully rounded by default, independently of the active
/// style pack. Explicit presets are resolved against the theme radius scale,
/// while `Custom` is useful for matching a surrounding surface exactly.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AvatarRadius {
    /// The source component's default `rounded-full` shape.
    #[default]
    Theme,
    /// No corner radius.
    None,
    /// The `sm` radius token.
    Small,
    /// The `md` radius token.
    Medium,
    /// The `lg` radius token.
    Large,
    /// The `xl` radius token.
    Xl,
    /// Fully rounded, capped by the rendered bounds.
    Full,
    /// A custom radius in pixels. Invalid values resolve to zero.
    Custom(f32),
}
