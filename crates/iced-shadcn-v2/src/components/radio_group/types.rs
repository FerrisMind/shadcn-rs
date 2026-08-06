//! Public configuration, status, and style types for the radio-group component.

use crate::iced_compat::Color;

/// Axis a [`super::RadioGroup`] lays its items out on.
///
/// The web component renders `.cn-radio-group` as a `grid gap-*`, i.e. a
/// vertical stack, which is why [`Self::Vertical`] is the default. The
/// orientation also decides which arrow keys [`super::RadioGroup::next_value`]
/// and [`super::RadioGroup::previous_value`] are meant to answer.
///
/// ```rust
/// use iced_shadcn_v2::RadioGroupOrientation;
///
/// assert_eq!(
///     RadioGroupOrientation::default(),
///     RadioGroupOrientation::Vertical,
/// );
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RadioGroupOrientation {
    /// Stack items from top to bottom (`grid gap-*`).
    #[default]
    Vertical,
    /// Lay items out from left to right.
    Horizontal,
}

impl RadioGroupOrientation {
    /// Returns `true` when items are laid out from left to right.
    #[must_use]
    pub const fn is_horizontal(self) -> bool {
        matches!(self, Self::Horizontal)
    }
}

/// Preset footprint of a [`super::RadioGroup`] indicator.
///
/// shadcn-svelte ships a single radio footprint per style pack, so
/// [`Self::Default`] is the pack value (`size-4`, Sera `size-4.5`) and the
/// other presets scale it: [`Self::Sm`] by `0.875` (`size-3.5` next to
/// `size-4`) and [`Self::Lg`] by `1.25` (`size-5`). Scaling keeps the dot and
/// the border in proportion, so a radio never loses its shape.
///
/// ```rust
/// use iced_shadcn_v2::RadioGroupSize;
///
/// assert_eq!(RadioGroupSize::default(), RadioGroupSize::Default);
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RadioGroupSize {
    /// `0.875` of the pack footprint.
    Sm,
    /// The pack footprint.
    #[default]
    Default,
    /// `1.25` of the pack footprint.
    Lg,
    /// Indicator diameter in logical pixels (clamped to at least 1 px).
    Custom(f32),
}

/// Corner-radius preset for a [`super::RadioGroup`] indicator.
///
/// Radii are capped to half the indicator diameter, so [`Self::Full`] always
/// produces a circle and never overlaps itself. Every shadcn style pack draws
/// radios as circles; the other presets exist for apps that deliberately want
/// a squarer control.
///
/// ```rust
/// use iced_shadcn_v2::RadioGroupRadius;
///
/// assert_eq!(RadioGroupRadius::default(), RadioGroupRadius::Full);
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RadioGroupRadius {
    /// Square corners.
    None,
    /// The theme's small radius slot.
    Small,
    /// The theme's medium radius slot.
    Medium,
    /// The theme's large radius slot.
    Large,
    /// A circle radius capped to half the indicator diameter.
    #[default]
    Full,
    /// An explicit radius in logical pixels.
    Custom(f32),
}

/// Interaction state one radio indicator is styled for.
///
/// Passed to [`super::RadioGroup::item_style_override`] and
/// [`super::RadioGroupItem::style_override`] together with the resolved
/// [`RadioGroupStyle`].
///
/// ```rust
/// use iced_shadcn_v2::RadioGroupStatus;
///
/// let status = RadioGroupStatus::default();
/// assert!(!status.checked && !status.disabled);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct RadioGroupStatus {
    /// Whether this item carries the group's selected value.
    pub checked: bool,
    /// Whether interaction is suppressed and the item is dimmed.
    pub disabled: bool,
    /// Whether the item is inert but keeps its normal colors.
    pub readonly: bool,
    /// Whether the application marked this item as focused.
    pub focused: bool,
    /// Whether the application marked the value as invalid.
    pub invalid: bool,
}

/// Resolved colors and geometry one radio indicator paints for a status.
///
/// ```rust
/// use iced::Color;
/// use iced_shadcn_v2::{RadioGroup, RadioGroupStyle, Theme};
///
/// # #[derive(Debug, Clone)]
/// # enum Message {}
/// let theme = Theme::light();
/// let group = RadioGroup::<Message>::new(&theme).item_style_override(|style, _status| {
///     RadioGroupStyle {
///         dot: Color::WHITE,
///         ..style
///     }
/// });
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RadioGroupStyle {
    /// Indicator fill.
    pub indicator: Color,
    /// Indicator border color.
    pub border: Color,
    /// Indicator border width in logical pixels.
    pub border_width: f32,
    /// Indicator corner radius in logical pixels.
    pub radius: f32,
    /// Indicator diameter in logical pixels.
    pub indicator_size: f32,
    /// Selected-dot fill.
    pub dot: Color,
    /// Selected-dot diameter in logical pixels.
    pub dot_size: f32,
    /// Focus / invalid ring color, or `None` when no ring is painted.
    pub ring: Option<Color>,
    /// Ring width in logical pixels.
    pub ring_width: f32,
    /// Label text color.
    pub label: Color,
    /// Description text color.
    pub description: Color,
}
