//! Configuration types used by the input component.

/// Preset control size for an [`super::Input`].
///
/// The web component ships a single `h-*` height per style pack; the extra
/// slots reuse the pack's `sm` / `md` / `lg` control-height ladder so an input
/// can line up with [`crate::Button`] rows of any size.
///
/// ```rust
/// use iced_shadcn_v2::InputSize;
///
/// assert_eq!(InputSize::default(), InputSize::Default);
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InputSize {
    /// Compact control height (pack `control_height_sm`).
    Sm,
    /// Default control height matching `.cn-input` (`h-9` on Vega).
    #[default]
    Default,
    /// Tall control height (pack `control_height_lg`).
    Lg,
}

/// Border radius preset for an [`super::Input`].
///
/// When no radius is set, the default follows the active style pack's
/// `.cn-input` corner treatment (`rounded-md` on Vega, pill on Maia/Luma,
/// square on Lyra/Sera, …).
///
/// ```rust
/// use iced_shadcn_v2::InputRadius;
///
/// assert!(InputRadius::None < InputRadius::Full);
/// assert_eq!(InputRadius::default(), InputRadius::Medium);
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InputRadius {
    /// No corner radius.
    None,
    /// Small corner radius.
    Small,
    /// Medium corner radius.
    #[default]
    Medium,
    /// Large corner radius.
    Large,
    /// Fully rounded (pill) corners.
    Full,
}
