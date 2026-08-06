//! Configuration types used by the popover component.

use shadcn_common::{FloatingAlign, FloatingSide, TransitionValue};

/// Side of the trigger on which a [`super::Popover`] opens.
///
/// Matches the `side` prop of the shadcn-svelte popover content.
///
/// ```rust
/// use iced_shadcn_v2::PopoverSide;
///
/// assert_eq!(PopoverSide::default(), PopoverSide::Bottom);
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PopoverSide {
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

impl PopoverSide {
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

impl From<PopoverSide> for FloatingSide {
    fn from(side: PopoverSide) -> Self {
        side.to_floating()
    }
}

/// Alignment of the popover along the trigger edge.
///
/// Matches the `align` prop of the shadcn-svelte popover content.
///
/// ```rust
/// use iced_shadcn_v2::PopoverAlign;
///
/// assert_eq!(PopoverAlign::default(), PopoverAlign::Center);
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PopoverAlign {
    /// Aligned with the start of the trigger edge.
    Start,
    /// Centered on the trigger edge.
    #[default]
    Center,
    /// Aligned with the end of the trigger edge.
    End,
}

impl PopoverAlign {
    /// The equivalent backend-agnostic alignment from `shadcn-common`.
    pub const fn to_floating(self) -> FloatingAlign {
        match self {
            Self::Start => FloatingAlign::Start,
            Self::Center => FloatingAlign::Center,
            Self::End => FloatingAlign::End,
        }
    }
}

impl From<PopoverAlign> for FloatingAlign {
    fn from(align: PopoverAlign) -> Self {
        align.to_floating()
    }
}

/// Open / transition state stored in the widget tree.
#[derive(Debug, Clone, Copy)]
pub(super) struct PopoverState {
    /// Uncontrolled open intent driven by trigger clicks and dismissals.
    pub(super) requested_open: bool,
    /// Current effective open target (after the controlled override).
    pub(super) open: bool,
    /// Backend-agnostic open/close transition state.
    pub(super) transition: TransitionValue,
    /// Set when the overlay dismissed a press that landed on the trigger,
    /// so the widget does not reopen on the very same press (toggle).
    pub(super) suppress_next_trigger_press: bool,
}

impl PopoverState {
    /// Creates the initial state honoring `defaultOpen`.
    pub(super) fn new(default_open: bool) -> Self {
        Self {
            requested_open: default_open,
            open: false,
            transition: TransitionValue::new(),
            suppress_next_trigger_press: false,
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
