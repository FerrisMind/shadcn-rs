//! Internal state used by the drawer component.

use shadcn_common::TransitionValue;

/// Open / transition / drag state stored in the widget tree.
#[derive(Debug, Clone, Copy)]
pub(super) struct DrawerState {
    /// Uncontrolled open intent driven by trigger clicks and dismissals.
    pub(super) requested_open: bool,
    /// Current effective open target (after the controlled override).
    pub(super) open: bool,
    /// Backend-agnostic open/close transition state.
    pub(super) transition: TransitionValue,
    /// Live drag offset along the dismiss axis (px, signed).
    pub(super) drag_offset: f32,
    /// Whether a dismiss drag is currently tracked.
    pub(super) dragging: bool,
    /// Pointer position captured when the drag started.
    pub(super) drag_origin: Option<(f32, f32)>,
}

impl DrawerState {
    /// Creates the initial state honoring `defaultOpen`.
    pub(super) fn new(default_open: bool) -> Self {
        Self {
            requested_open: default_open,
            open: false,
            transition: TransitionValue::new(),
            drag_offset: 0.0,
            dragging: false,
            drag_origin: None,
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

    /// Clears any in-progress dismiss drag.
    pub(super) fn clear_drag(&mut self) {
        self.dragging = false;
        self.drag_offset = 0.0;
        self.drag_origin = None;
    }
}
