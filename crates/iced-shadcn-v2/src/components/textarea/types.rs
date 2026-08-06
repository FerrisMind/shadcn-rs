//! Configuration types used by the textarea component.

/// Preset control size for a [`super::Textarea`].
///
/// The web component ships a single `min-h-16` height per style pack; the
/// extra slots scale the minimum height and text size so a textarea can line
/// up with [`crate::Input`] / [`crate::Button`] rows of any size.
///
/// ```rust
/// use iced_shadcn_v2::TextareaSize;
///
/// assert_eq!(TextareaSize::default(), TextareaSize::Default);
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TextareaSize {
    /// Compact minimum height (pack `control_height_sm`).
    Sm,
    /// Default minimum height matching `.cn-textarea` (`min-h-16` → 64).
    #[default]
    Default,
    /// Tall minimum height (`min-h-24` → 96).
    Lg,
}

/// Border radius preset for a [`super::Textarea`].
///
/// When no radius is set, the default follows the active style pack's
/// `.cn-textarea` corner treatment (`rounded-md` on Vega, `rounded-lg` on
/// Nova, square on Lyra/Sera, …).
///
/// ```rust
/// use iced_shadcn_v2::TextareaRadius;
///
/// assert!(TextareaRadius::None < TextareaRadius::Full);
/// assert_eq!(TextareaRadius::default(), TextareaRadius::Medium);
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TextareaRadius {
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

/// Resize policy for a [`super::Textarea`].
///
/// Iced's `text_editor` widget has no browser-style pointer resize handle.
/// [`Self::None`] keeps the control at its minimum height; the other values
/// leave the height unconstrained while preserving the source component's
/// intent for applications that provide their own layout policy.
///
/// ```rust
/// use iced_shadcn_v2::TextareaResize;
///
/// assert_eq!(TextareaResize::default(), TextareaResize::None);
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TextareaResize {
    /// Keep the textarea at its minimum height.
    #[default]
    None,
    /// Allow vertical resizing.
    Vertical,
    /// Allow horizontal resizing.
    Horizontal,
    /// Allow resizing in both directions.
    Both,
}
