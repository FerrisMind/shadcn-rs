//! Public configuration types for the card component.

/// Card density, matching shadcn-svelte `Card.Root`'s `size` prop.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CardSize {
    /// The style-pack default card spacing.
    #[default]
    Default,
    /// The compact card spacing (`data-size="sm"`).
    Sm,
}

/// Corner-radius intent for a card.
///
/// [`CardRadius::Theme`] keeps the active style-pack geometry, including the
/// sharp Lyra and Sera cards and the capped Rhea radius. Explicit presets are
/// resolved against the active theme's radius scale.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CardRadius {
    /// Use the active style-pack card radius.
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

/// Border mode for a card header or footer.
///
/// `Theme` follows the source style pack. Vega-like packs have no section
/// border by default, while Nova and Lyra give the footer a top border.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CardBorder {
    /// Use the active style-pack default.
    #[default]
    Theme,
    /// Force the section border off.
    None,
    /// Force the section border on.
    Present,
}

/// Footer layout direction.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CardFooterDirection {
    /// Place children in a horizontal row.
    #[default]
    Row,
    /// Place children in a vertical column.
    Column,
}

/// Horizontal footer child alignment, matching common footer utility classes.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CardFooterAlignment {
    /// Pack children from the leading edge.
    #[default]
    Start,
    /// Center children in the footer.
    Center,
    /// Pack children from the trailing edge.
    End,
    /// Distribute row children across the available width.
    SpaceBetween,
}
