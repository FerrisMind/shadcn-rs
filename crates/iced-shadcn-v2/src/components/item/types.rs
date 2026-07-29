//! Public configuration types for the item component.

/// Visual treatment of an [`Item`](super::Item) row.
///
/// Mirrors the `variant` prop of shadcn-svelte `Item`.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ItemVariant {
    /// Transparent surface without a visible border.
    #[default]
    Default,
    /// Transparent surface outlined with the theme `border` token.
    Outline,
    /// Half-opacity `muted` surface without a visible border.
    Muted,
}

/// Item density, matching shadcn-svelte `Item`'s `size` prop.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ItemSize {
    /// The style-pack default item spacing.
    #[default]
    Default,
    /// The compact item spacing (`data-size="sm"`).
    Sm,
    /// The extra-compact item spacing (`data-size="xs"`).
    Xs,
}

/// Corner-radius intent for an item row.
///
/// [`ItemRadius::Theme`] keeps the active style-pack geometry, including the
/// sharp Lyra and Sera rows. Explicit presets are resolved against the active
/// theme's radius scale.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ItemRadius {
    /// Use the active style-pack item radius.
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
    /// A fully rounded radius, capped by the rendered bounds.
    Full,
    /// A custom radius in pixels. Invalid values resolve to zero.
    Custom(f32),
}

/// Visual treatment of an [`ItemMedia`](super::ItemMedia) slot.
///
/// Mirrors the `variant` prop of shadcn-svelte `ItemMedia`.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ItemMediaVariant {
    /// Unstyled media content (e.g. an avatar).
    #[default]
    Default,
    /// A leading glyph sized to the style-pack icon metrics.
    Icon,
    /// A clipped thumbnail sized by the item density.
    Image,
}
