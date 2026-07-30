//! Configuration types for the empty-state component.

/// Media treatment for [`super::EmptyMedia`].
///
/// The `Icon` variant applies the style-pack muted surface, fixed media size,
/// and style-pack corner radius used by shadcn-svelte. `Default` leaves the
/// media unboxed so it can contain an avatar, illustration, or any other
/// arbitrary iced element.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum EmptyMediaVariant {
    /// Render media without a background or fixed footprint.
    #[default]
    Default,
    /// Render media in the style-pack icon tile.
    Icon,
}

/// Root border treatment for [`super::Empty`].
///
/// shadcn-svelte sets the border style to dashed, but does not make a border
/// visible until a caller supplies a border width. The default therefore
/// remains [`Self::None`]; [`super::Empty::outline`] is the convenient way to
/// opt into the source component's visible dashed outline.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum EmptyBorderStyle {
    /// Do not draw a root border.
    #[default]
    None,
    /// Draw a solid root border.
    Solid,
    /// Draw a dashed root border.
    Dashed,
}

/// Corner-radius intent for [`super::Empty`].
///
/// [`Self::Theme`] follows the active shadcn style pack, including the sharp
/// Lyra and Sera packs and the larger Rhea radius. Explicit variants resolve
/// against the active theme's radius scale.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum EmptyRadius {
    /// Use the active style-pack empty-state radius.
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
    /// The `xxl` radius token.
    Xxl,
    /// A fully rounded radius, capped by the rendered bounds.
    Full,
    /// A custom radius in pixels. Invalid values resolve to zero.
    Custom(f32),
}
