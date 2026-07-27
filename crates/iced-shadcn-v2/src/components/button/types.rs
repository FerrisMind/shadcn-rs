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
/// ```rust
/// use iced_shadcn_v2::ButtonSize;
///
/// assert!(ButtonSize::Size0 < ButtonSize::Size4);
/// assert_eq!(ButtonSize::default(), ButtonSize::Size2);
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub enum ButtonSize {
    /// Extra-small control size.
    Size0,
    /// Small control size.
    Size1,
    /// Medium control size.
    #[default]
    Size2,
    /// Large control size.
    Size3,
    /// Extra-large control size.
    Size4,
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
