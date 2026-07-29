//! Configuration types used by the button component.

/// Visual treatment of a [`super::Button`].
///
/// ```rust
/// use iced_shadcn_v2::ButtonVariant;
///
/// assert_eq!(ButtonVariant::default(), ButtonVariant::Default);
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ButtonVariant {
    /// Filled button using the theme primary color.
    #[default]
    Default,
    /// Soft destructive button using the theme destructive color.
    Destructive,
    /// Transparent button with a visible border.
    Outline,
    /// Filled button using the theme secondary surface.
    Secondary,
    /// Transparent button without a border.
    Ghost,
    /// Text-only button with a hover underline.
    Link,
    /// Filled button using the accent's soft surface.
    Soft,
    /// Elevated button using the background surface and a shadow.
    Surface,
}

/// Preset control size for a [`super::Button`].
///
/// Names and footprints match shadcn-svelte `buttonVariants` sizes
/// (`xs` / `sm` / `default` / `lg` / `icon*`), expressed as a Rust enum
/// instead of Tailwind classes.
///
/// ```rust
/// use iced_shadcn_v2::ButtonSize;
///
/// assert_eq!(ButtonSize::default(), ButtonSize::Default);
/// assert!(ButtonSize::Icon.is_icon());
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ButtonSize {
    /// `h-6` text button (`xs`).
    Xs,
    /// `h-8` text button (`sm`).
    Sm,
    /// `h-9` text button (`default`).
    #[default]
    Default,
    /// `h-10` text button (`lg`).
    Lg,
    /// Square `size-6` icon button (`icon-xs`).
    IconXs,
    /// Square `size-8` icon button (`icon-sm`).
    IconSm,
    /// Square `size-9` icon button (`icon`).
    Icon,
    /// Square `size-10` icon button (`icon-lg`).
    IconLg,
}

impl ButtonSize {
    /// Whether this size is an icon-only (square) footprint.
    pub const fn is_icon(self) -> bool {
        matches!(
            self,
            Self::IconXs | Self::IconSm | Self::Icon | Self::IconLg
        )
    }
}

/// Crate-internal mask of corners flattened to a zero radius after style
/// resolution. Group containers (e.g. the button-group component) use it to
/// merge adjacent controls into a single visual unit.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct CornerFlatten {
    pub(crate) top_left: bool,
    pub(crate) top_right: bool,
    pub(crate) bottom_right: bool,
    pub(crate) bottom_left: bool,
}

impl CornerFlatten {
    /// Whether at least one corner is flattened.
    pub(crate) const fn is_any(self) -> bool {
        self.top_left || self.top_right || self.bottom_right || self.bottom_left
    }
}

/// Border radius preset for a [`super::Button`].
///
/// ```rust
/// use iced_shadcn_v2::ButtonRadius;
///
/// assert!(ButtonRadius::None < ButtonRadius::Full);
/// assert_eq!(ButtonRadius::default(), ButtonRadius::Medium);
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ButtonRadius {
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
