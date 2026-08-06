//! Public configuration, style, and state types for the slider component.

use crate::iced_compat::Color;

/// Axis a [`super::Slider`] runs along.
///
/// ```rust
/// use iced_shadcn_v2::SliderOrientation;
///
/// assert_eq!(SliderOrientation::default(), SliderOrientation::Horizontal);
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SliderOrientation {
    /// Values grow from the leading to the trailing edge.
    #[default]
    Horizontal,
    /// Values grow from the bottom to the top edge.
    Vertical,
}

impl SliderOrientation {
    /// Whether the slider runs top to bottom.
    ///
    /// ```rust
    /// use iced_shadcn_v2::SliderOrientation;
    ///
    /// assert!(SliderOrientation::Vertical.is_vertical());
    /// assert!(!SliderOrientation::Horizontal.is_vertical());
    /// ```
    pub const fn is_vertical(self) -> bool {
        matches!(self, Self::Vertical)
    }
}

/// Corner-radius preset for a [`super::Slider`] track and thumbs.
///
/// Radii are capped to half of the smallest dimension, so [`SliderRadius::Full`]
/// always produces a pill instead of overlapping itself.
///
/// ```rust
/// use iced_shadcn_v2::SliderRadius;
///
/// assert_eq!(SliderRadius::default(), SliderRadius::Full);
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SliderRadius {
    /// Square corners.
    None,
    /// The theme's small radius slot.
    Small,
    /// The theme's medium radius slot.
    Medium,
    /// The theme's large radius slot.
    Large,
    /// A pill radius capped to half the smallest dimension.
    #[default]
    Full,
    /// An explicit radius in logical pixels.
    Custom(f32),
}

/// Interaction state a [`super::Slider`] is styled for.
///
/// ```rust
/// use iced_shadcn_v2::SliderStatus;
///
/// let status = SliderStatus::default();
/// assert!(!status.dragging && !status.disabled);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct SliderStatus {
    /// Whether the cursor is over the slider.
    pub hovered: bool,
    /// Whether a thumb is being dragged.
    pub dragging: bool,
    /// Whether interaction is suppressed.
    pub disabled: bool,
    /// Whether the application marked the slider as focused.
    pub focused: bool,
}

/// Resolved colors and geometry a [`super::Slider`] paints for one status.
///
/// The ring is painted around the active thumb only — the hovered one, the one
/// being dragged, or every thumb while [`super::Slider::focused`] is set.
///
/// ```rust
/// use iced::Color;
/// use iced_shadcn_v2::{Slider, SliderStyle, Theme};
///
/// # #[derive(Debug, Clone)]
/// # enum Message {}
/// let theme = Theme::light();
/// let slider = Slider::<Message>::new(&theme).style_override(|style, _status| SliderStyle {
///     thumb: Color::WHITE,
///     ..style
/// });
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SliderStyle {
    /// Track fill.
    pub track: Color,
    /// Selected-range fill.
    pub range: Color,
    /// Track and range corner radius in logical pixels.
    pub track_radius: f32,
    /// Thumb fill.
    pub thumb: Color,
    /// Thumb border color.
    pub thumb_border: Color,
    /// Thumb border width in logical pixels.
    pub thumb_border_width: f32,
    /// Thumb corner radius in logical pixels.
    pub thumb_radius: f32,
    /// Ring color painted around the active thumb.
    pub ring: Color,
    /// Ring width in logical pixels.
    pub ring_width: f32,
}

/// Per-instance interaction state of the slider canvas program.
///
/// The runtime owns this value; applications keep only the numbers they pass to
/// [`super::Slider::value`] or [`super::Slider::values`].
#[derive(Debug, Default)]
#[doc(hidden)]
pub struct SliderState {
    pub(super) dragging: Option<usize>,
    pub(super) hovered: Option<usize>,
    pub(super) active_finger: Option<crate::iced_compat::touch::Finger>,
}
