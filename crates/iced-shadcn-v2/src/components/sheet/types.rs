//! Internal state used by the sheet component.

use shadcn_common::TransitionValue;

/// Open / transition state stored in the widget tree.
#[derive(Debug, Clone, Copy)]
pub(super) struct SheetState {
    /// Uncontrolled open intent driven by trigger clicks and dismissals.
    pub(super) requested_open: bool,
    /// Current effective open target (after the controlled override).
    pub(super) open: bool,
    /// Backend-agnostic open/close transition state.
    pub(super) transition: TransitionValue,
}

impl SheetState {
    /// Creates the initial state honoring `defaultOpen`.
    pub(super) fn new(default_open: bool) -> Self {
        Self {
            requested_open: default_open,
            open: false,
            transition: TransitionValue::new(),
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
