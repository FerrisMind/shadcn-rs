//! Configuration types used by the toggle component.

/// Visual treatment of a [`super::Toggle`].
///
/// Mirrors shadcn-svelte `toggleVariants` (`default` / `outline`).
///
/// ```rust
/// use iced_shadcn_v2::ToggleVariant;
///
/// assert_eq!(ToggleVariant::default(), ToggleVariant::Default);
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ToggleVariant {
    /// Transparent control that fills with `muted` on hover / when pressed on.
    #[default]
    Default,
    /// Transparent control with a visible `input` border.
    Outline,
}

/// Preset control size for a [`super::Toggle`].
///
/// Names and footprints match shadcn-svelte `toggleVariants` sizes
/// (`sm` / `default` / `lg`), expressed as a Rust enum instead of Tailwind
/// classes. Icon-only toggles keep the same heights and become square, like
/// the `min-w-*` utilities of the web component.
///
/// ```rust
/// use iced_shadcn_v2::ToggleSize;
///
/// assert_eq!(ToggleSize::default(), ToggleSize::Default);
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ToggleSize {
    /// The pack's `sm` footprint.
    Sm,
    /// The pack's `default` footprint.
    #[default]
    Default,
    /// The pack's `lg` footprint.
    Lg,
}

/// Border radius preset for a [`super::Toggle`].
///
/// ```rust
/// use iced_shadcn_v2::ToggleRadius;
///
/// assert!(ToggleRadius::None < ToggleRadius::Full);
/// assert_eq!(ToggleRadius::default(), ToggleRadius::Medium);
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ToggleRadius {
    /// No corner radius.
    None,
    /// Small corner radius.
    Small,
    /// Medium corner radius.
    #[default]
    Medium,
    /// Large corner radius.
    Large,
    /// Fully rounded corners.
    Full,
}
