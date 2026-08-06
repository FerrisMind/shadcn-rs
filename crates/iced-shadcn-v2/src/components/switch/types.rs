//! Public configuration, style, and state types for the switch component.

use crate::iced_compat::Color;
use shadcn_common::TransitionValue;

/// Preset footprint of a [`super::Switch`].
///
/// The presets mirror the `data-size` values of shadcn-svelte's switch
/// (`sm` / `default`) and resolve against the active style pack.
/// [`SwitchSize::Custom`] scales the pack's default footprint — track, thumb,
/// and travel distance — so a switch keeps its proportions at any height.
///
/// ```rust
/// use iced_shadcn_v2::SwitchSize;
///
/// assert_eq!(SwitchSize::default(), SwitchSize::Default);
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SwitchSize {
    /// The pack's `sm` footprint.
    Sm,
    /// The pack's `default` footprint.
    #[default]
    Default,
    /// Track height in logical pixels (clamped to at least 1 px).
    Custom(f32),
}

/// Corner-radius preset for a [`super::Switch`] track and thumb.
///
/// Radii are capped to half of the smallest dimension, so [`SwitchRadius::Full`]
/// always produces a pill and never overlaps itself.
///
/// ```rust
/// use iced_shadcn_v2::SwitchRadius;
///
/// assert_eq!(SwitchRadius::default(), SwitchRadius::Full);
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SwitchRadius {
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

/// Interaction state a [`super::Switch`] is styled for.
///
/// Passed to [`super::Switch::style_override`] together with the resolved
/// [`SwitchStyle`].
///
/// ```rust
/// use iced_shadcn_v2::SwitchStatus;
///
/// let status = SwitchStatus::default();
/// assert!(!status.checked && !status.disabled);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct SwitchStatus {
    /// Whether the switch is on.
    pub checked: bool,
    /// Whether the cursor is over the switch.
    pub hovered: bool,
    /// Whether interaction is suppressed.
    pub disabled: bool,
    /// Whether the application marked the switch as focused.
    pub focused: bool,
    /// Whether the application marked the value as invalid.
    pub invalid: bool,
}

/// Resolved colors and geometry a [`super::Switch`] paints for one status.
///
/// ```rust
/// use iced::Color;
/// use iced_shadcn_v2::{Switch, SwitchStyle, Theme};
///
/// # #[derive(Debug, Clone)]
/// # enum Message {}
/// let theme = Theme::light();
/// let switch = Switch::<Message>::new(&theme).style_override(|style, _status| SwitchStyle {
///     thumb: Color::WHITE,
///     ..style
/// });
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SwitchStyle {
    /// Track fill.
    pub track: Color,
    /// Track border color.
    pub border: Color,
    /// Track border width in logical pixels.
    pub border_width: f32,
    /// Track corner radius in logical pixels.
    pub track_radius: f32,
    /// Thumb fill.
    pub thumb: Color,
    /// Thumb corner radius in logical pixels.
    pub thumb_radius: f32,
    /// Focus / invalid ring color, or `None` when no ring is painted.
    pub ring: Option<Color>,
    /// Ring width in logical pixels.
    pub ring_width: f32,
}

/// Per-instance animation state of the switch canvas program.
///
/// The runtime owns this value; applications keep only the boolean `checked`
/// state they pass to [`super::Switch::checked`].
#[derive(Debug, Default)]
#[doc(hidden)]
pub struct SwitchState {
    pub(super) transition: TransitionValue,
}
