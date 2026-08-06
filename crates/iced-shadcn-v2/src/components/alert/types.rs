//! Public configuration types for the alert component.

/// Visual treatment of an [`super::Alert`].
///
/// The variants mirror shadcn-svelte's `Alert.Root` variants. The root stays
/// a card-like surface in both cases; `Destructive` changes the foreground
/// and description colors to the theme's semantic destructive token.
///
/// ```rust
/// use iced_shadcn_v2::AlertVariant;
///
/// assert_eq!(AlertVariant::default(), AlertVariant::Default);
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AlertVariant {
    /// A regular informational or success callout.
    #[default]
    Default,
    /// A destructive or error callout.
    Destructive,
}

/// Corner-radius intent for an [`super::Alert`].
///
/// `Theme` follows the active shadcn style pack. Explicit values are resolved
/// against that theme's radius scale, while `Custom` is useful when an app
/// needs to match a surrounding surface exactly.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AlertRadius {
    /// Use the active style-pack geometry.
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
