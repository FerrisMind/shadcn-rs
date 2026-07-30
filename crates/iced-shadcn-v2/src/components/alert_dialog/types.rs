//! Internal state and configuration types for the alert-dialog component.

use shadcn_common::TransitionValue;

/// Surface size of a [`super::AlertDialog`] (`data-size` on
/// `AlertDialog.Content`).
///
/// ```rust
/// use iced_shadcn_v2::AlertDialogSize;
///
/// assert_eq!(AlertDialogSize::default(), AlertDialogSize::Default);
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AlertDialogSize {
    /// Wide surface with a left-aligned header and a right-aligned footer
    /// row (`size="default"`, `sm:max-w-lg`).
    #[default]
    Default,
    /// Narrow surface with a centered header and a two-column footer grid
    /// (`size="sm"`, `max-w-xs`).
    Sm,
}

/// Open / transition state stored in the widget tree.
#[derive(Debug, Clone, Copy)]
pub(super) struct AlertDialogState {
    /// Uncontrolled open intent driven by trigger clicks and dismissals.
    pub(super) requested_open: bool,
    /// Current effective open target (after the controlled override).
    pub(super) open: bool,
    /// Backend-agnostic open/close transition state.
    pub(super) transition: TransitionValue,
    /// Footer child a left press started on; a matching release dismisses.
    pub(super) pressed_footer: Option<usize>,
}

impl AlertDialogState {
    /// Creates the initial state honoring `defaultOpen`.
    pub(super) fn new(default_open: bool) -> Self {
        Self {
            requested_open: default_open,
            open: false,
            transition: TransitionValue::new(),
            pressed_footer: None,
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
