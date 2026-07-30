//! Configuration types used by the tooltip component.

use shadcn_common::{FloatingAlign, FloatingSide};

use crate::iced_compat::time::Instant;

/// Side of the trigger on which a [`super::Tooltip`] opens.
///
/// Matches the `side` prop of the shadcn-svelte tooltip content.
///
/// ```rust
/// use iced_shadcn_v2::TooltipSide;
///
/// assert_eq!(TooltipSide::default(), TooltipSide::Top);
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TooltipSide {
    /// Above the trigger.
    #[default]
    Top,
    /// To the right of the trigger.
    Right,
    /// Below the trigger.
    Bottom,
    /// To the left of the trigger.
    Left,
}

impl TooltipSide {
    /// The equivalent backend-agnostic side from `shadcn-common`.
    pub const fn to_floating(self) -> FloatingSide {
        match self {
            Self::Top => FloatingSide::Top,
            Self::Right => FloatingSide::Right,
            Self::Bottom => FloatingSide::Bottom,
            Self::Left => FloatingSide::Left,
        }
    }
}

impl From<TooltipSide> for FloatingSide {
    fn from(side: TooltipSide) -> Self {
        side.to_floating()
    }
}

/// Alignment of the tooltip along the trigger edge.
///
/// Matches the `align` prop of the shadcn-svelte tooltip content.
///
/// ```rust
/// use iced_shadcn_v2::TooltipAlign;
///
/// assert_eq!(TooltipAlign::default(), TooltipAlign::Center);
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TooltipAlign {
    /// Aligned with the start of the trigger edge.
    Start,
    /// Centered on the trigger edge.
    #[default]
    Center,
    /// Aligned with the end of the trigger edge.
    End,
}

impl TooltipAlign {
    /// The equivalent backend-agnostic alignment from `shadcn-common`.
    pub const fn to_floating(self) -> FloatingAlign {
        match self {
            Self::Start => FloatingAlign::Start,
            Self::Center => FloatingAlign::Center,
            Self::End => FloatingAlign::End,
        }
    }
}

impl From<TooltipAlign> for FloatingAlign {
    fn from(align: TooltipAlign) -> Self {
        align.to_floating()
    }
}

/// Hover / open-transition state stored in the widget tree.
#[derive(Debug, Clone, Copy, Default)]
pub(super) struct TooltipState {
    /// Whether the state has seen its first frame.
    pub(super) initialized: bool,
    /// Current logical open target (after delay and overrides).
    pub(super) open: bool,
    /// Progress currently painted, in `0.0..=1.0`.
    pub(super) displayed: f32,
    /// Progress the running transition started from.
    pub(super) transition_from: f32,
    /// Start instant of the running open/close transition.
    pub(super) transition_start: Option<Instant>,
    /// Instant the cursor entered the trigger, for the open delay.
    pub(super) hover_started: Option<Instant>,
}

impl TooltipState {
    /// Whether the overlay should currently be mounted.
    pub(super) fn is_visible(&self) -> bool {
        self.open || self.displayed > 0.0 || self.transition_start.is_some()
    }
}
