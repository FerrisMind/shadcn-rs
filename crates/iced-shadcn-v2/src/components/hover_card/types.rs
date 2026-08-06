//! Configuration types used by the hover-card component.

use shadcn_common::TransitionValue;
use shadcn_common::{FloatingAlign, FloatingSide};

use crate::iced_compat::time::Instant;

/// Side of the trigger on which a [`super::HoverCard`] opens.
///
/// Matches the `side` prop of the shadcn-svelte hover-card content.
///
/// ```rust
/// use iced_shadcn_v2::HoverCardSide;
///
/// assert_eq!(HoverCardSide::default(), HoverCardSide::Bottom);
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum HoverCardSide {
    /// Above the trigger.
    Top,
    /// To the right of the trigger.
    Right,
    /// Below the trigger.
    #[default]
    Bottom,
    /// To the left of the trigger.
    Left,
}

impl HoverCardSide {
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

impl From<HoverCardSide> for FloatingSide {
    fn from(side: HoverCardSide) -> Self {
        side.to_floating()
    }
}

/// Alignment of the hover card along the trigger edge.
///
/// Matches the `align` prop of the shadcn-svelte hover-card content.
///
/// ```rust
/// use iced_shadcn_v2::HoverCardAlign;
///
/// assert_eq!(HoverCardAlign::default(), HoverCardAlign::Center);
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum HoverCardAlign {
    /// Aligned with the start of the trigger edge.
    Start,
    /// Centered on the trigger edge.
    #[default]
    Center,
    /// Aligned with the end of the trigger edge.
    End,
}

impl HoverCardAlign {
    /// The equivalent backend-agnostic alignment from `shadcn-common`.
    pub const fn to_floating(self) -> FloatingAlign {
        match self {
            Self::Start => FloatingAlign::Start,
            Self::Center => FloatingAlign::Center,
            Self::End => FloatingAlign::End,
        }
    }
}

impl From<HoverCardAlign> for FloatingAlign {
    fn from(align: HoverCardAlign) -> Self {
        align.to_floating()
    }
}

/// Hover / delay / transition state stored in the widget tree.
#[derive(Debug, Clone, Copy)]
pub(super) struct HoverCardState {
    /// Uncontrolled open intent, driven by hover and the delay timers.
    pub(super) requested_open: bool,
    /// Current effective open target (after the controlled override).
    pub(super) open: bool,
    /// Backend-agnostic open/close transition state.
    pub(super) transition: TransitionValue,
    /// Instant the cursor entered the trigger, for `openDelay`.
    pub(super) hover_started: Option<Instant>,
    /// Instant the cursor left both trigger and content, for `closeDelay`.
    pub(super) leave_started: Option<Instant>,
    /// Whether the cursor is currently over the floating surface. Written
    /// by the overlay, read by the widget when evaluating `closeDelay`.
    pub(super) content_hovered: bool,
}

impl HoverCardState {
    /// Creates the initial state honoring `defaultOpen`.
    pub(super) fn new(default_open: bool) -> Self {
        Self {
            requested_open: default_open,
            open: false,
            transition: TransitionValue::new(),
            hover_started: None,
            leave_started: None,
            content_hovered: false,
        }
    }

    /// Whether the overlay should currently be mounted.
    pub(super) fn is_visible(&self) -> bool {
        self.open || self.transition.current() > 0.0 || self.transition.is_running()
    }

    /// Progress currently painted by the overlay.
    pub(super) fn progress(&self) -> f32 {
        self.transition
            .displayed(f32::from(u8::from(self.open)))
            .clamp(0.0, 1.0)
    }
}
