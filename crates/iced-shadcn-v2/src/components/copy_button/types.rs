//! State and action types for the copy button.

/// The result currently shown by a [`super::CopyButton`].
///
/// The state is controlled by the application. This is deliberate: iced's
/// [`iced_core::Clipboard`] trait exposes a fire-and-forget `write` operation,
/// so only the application knows whether its clipboard backend succeeded.
///
/// ```rust
/// use iced_shadcn_v2::CopyButtonStatus;
///
/// assert!(CopyButtonStatus::Idle.is_idle());
/// assert!(!CopyButtonStatus::Success.is_idle());
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CopyButtonStatus {
    /// The normal copy icon is shown.
    #[default]
    Idle,
    /// The application confirmed that the text was copied.
    Success,
    /// The application reported that copying failed.
    Failure,
}

impl CopyButtonStatus {
    /// Returns `true` when the normal copy icon should be shown.
    pub const fn is_idle(self) -> bool {
        matches!(self, Self::Idle)
    }

    /// Returns the short accessible description associated with the status.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Idle => "Copy",
            Self::Success => "Copied",
            Self::Failure => "Failed to copy",
        }
    }
}

/// Controlled state for a [`super::CopyButton`].
///
/// Keep this value in application state and pass it to
/// [`super::CopyButton::status`]. The private representation leaves room for
/// adding metadata without making downstream code depend on the layout.
///
/// ```rust
/// use iced_shadcn_v2::{CopyButtonState, CopyButtonStatus};
///
/// let state = CopyButtonState::new().with_status(CopyButtonStatus::Success);
/// assert_eq!(state.status(), CopyButtonStatus::Success);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CopyButtonState {
    status: CopyButtonStatus,
}

impl CopyButtonState {
    /// Creates the idle state.
    pub const fn new() -> Self {
        Self {
            status: CopyButtonStatus::Idle,
        }
    }

    /// Returns the current visual status.
    pub const fn status(self) -> CopyButtonStatus {
        self.status
    }

    /// Returns a copy of this state with a new visual status.
    #[must_use = "state builders return the modified state"]
    pub const fn with_status(mut self, status: CopyButtonStatus) -> Self {
        self.status = status;
        self
    }
}

/// Application actions understood by [`copy_button_reduce`].
///
/// A copy button emits the `Pressed` message configured with
/// [`super::CopyButton::on_copy`]. The application performs its clipboard
/// operation and dispatches `Success` or `Failure`, then dispatches `Reset`
/// after the configured feedback delay.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CopyButtonAction {
    /// The user pressed the button and a copy operation should start.
    Pressed,
    /// The clipboard operation succeeded.
    Success,
    /// The clipboard operation failed.
    Failure,
    /// Clear the transient success or failure feedback.
    Reset,
}

/// Result of reducing a [`CopyButtonAction`].
///
/// `should_reset` is `true` for `Success` and `Failure`. Applications should
/// schedule [`CopyButtonAction::Reset`] after the same delay used by
/// [`super::CopyButton::animation_duration`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CopyButtonUpdate {
    state: CopyButtonState,
    should_reset: bool,
}

impl CopyButtonUpdate {
    /// Returns the reduced state.
    pub const fn state(self) -> CopyButtonState {
        self.state
    }

    /// Returns whether the application should schedule a `Reset` action.
    pub const fn should_reset(self) -> bool {
        self.should_reset
    }
}

/// Reduces one copy-button action into the next controlled state.
///
/// The reducer is intentionally independent of iced runtime types, which
/// makes it usable from an application's `update` function and easy to test.
///
/// ```rust
/// use iced_shadcn_v2::{
///     CopyButtonAction, CopyButtonState, CopyButtonStatus, copy_button_reduce,
/// };
///
/// let update = copy_button_reduce(CopyButtonState::new(), CopyButtonAction::Success);
/// assert_eq!(update.state().status(), CopyButtonStatus::Success);
/// assert!(update.should_reset());
/// ```
#[must_use]
pub fn copy_button_reduce(_state: CopyButtonState, action: CopyButtonAction) -> CopyButtonUpdate {
    let (status, should_reset) = match action {
        CopyButtonAction::Pressed => (CopyButtonStatus::Idle, false),
        CopyButtonAction::Success => (CopyButtonStatus::Success, true),
        CopyButtonAction::Failure => (CopyButtonStatus::Failure, true),
        CopyButtonAction::Reset => (CopyButtonStatus::Idle, false),
    };

    CopyButtonUpdate {
        state: CopyButtonState::new().with_status(status),
        should_reset,
    }
}
