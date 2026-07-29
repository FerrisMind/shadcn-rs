//! Configuration types used by the badge component.

/// Visual treatment of a [`super::Badge`].
///
/// Names match shadcn-svelte `badgeVariants` (`default` / `secondary` /
/// `destructive` / `outline` / `ghost` / `link`).
///
/// ```rust
/// use iced_shadcn_v2::BadgeVariant;
///
/// assert_eq!(BadgeVariant::default(), BadgeVariant::Default);
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum BadgeVariant {
    /// Filled badge using the theme primary color.
    #[default]
    Default,
    /// Soft destructive badge using the theme destructive color.
    Destructive,
    /// Transparent badge with a visible border.
    Outline,
    /// Filled badge using the theme secondary surface.
    Secondary,
    /// Transparent badge without a border.
    Ghost,
    /// Text-only badge with a hover underline when interactive.
    Link,
}

/// Border radius preset for a [`super::Badge`].
///
/// When unset on the builder, badges default to a pill (`Full`) — matching
/// shadcn’s `rounded-4xl` — except for style packs that lock radius to none
/// (Lyra / Sera).
///
/// ```rust
/// use iced_shadcn_v2::BadgeRadius;
///
/// assert!(BadgeRadius::None < BadgeRadius::Full);
/// assert_eq!(BadgeRadius::default(), BadgeRadius::Full);
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum BadgeRadius {
    /// No corner radius.
    None,
    /// Small corner radius.
    Small,
    /// Medium corner radius.
    Medium,
    /// Large corner radius.
    Large,
    /// Fully rounded (pill) corners.
    #[default]
    Full,
}
