//! Backend-agnostic sidebar state, widths, and keyboard helpers.
//!
//! Mirrors shadcn-svelte `Sidebar.Provider` / `useSidebar` so iced and egui
//! share one behaviour layer. Persistence (cookies) stays an application
//! concern — this module only exposes the cookie name and max-age constants.

/// Cookie key used by the web provider (`sidebar_state`).
pub const SIDEBAR_COOKIE_NAME: &str = "sidebar_state";

/// Cookie lifetime in seconds (`60 * 60 * 24 * 7`).
pub const SIDEBAR_COOKIE_MAX_AGE_SECS: u64 = 60 * 60 * 24 * 7;

/// Expanded desktop width (`16rem`).
pub const SIDEBAR_WIDTH_PX: f32 = 256.0;

/// Mobile sheet width (`18rem`).
pub const SIDEBAR_WIDTH_MOBILE_PX: f32 = 288.0;

/// Collapsed icon-rail width (`3rem`).
pub const SIDEBAR_WIDTH_ICON_PX: f32 = 48.0;

/// Extra gap added to the icon rail for floating / inset variants
/// (`spacing(4)` → 16px).
pub const SIDEBAR_FLOATING_ICON_EXTRA_PX: f32 = 16.0;

/// Floating / inset container padding (`p-2` → 8px).
pub const SIDEBAR_FLOATING_PAD_PX: f32 = 8.0;

/// Keyboard shortcut letter (`b`) used with Ctrl / Meta.
pub const SIDEBAR_KEYBOARD_SHORTCUT: char = 'b';

/// Viewport width below which the sidebar switches to the mobile sheet
/// (`md` breakpoint → 768px).
pub const SIDEBAR_MOBILE_BREAKPOINT_PX: f32 = 768.0;

/// Open/close transition (`duration-200`).
pub const SIDEBAR_ANIMATION_MS: u64 = 200;

/// Which edge the sidebar docks to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum SidebarSide {
    /// Docks to the start edge (`left` in LTR).
    #[default]
    Left,
    /// Docks to the end edge (`right` in LTR).
    Right,
}

impl SidebarSide {
    /// Every supported edge.
    pub const ALL: [Self; 2] = [Self::Left, Self::Right];
}

/// Visual treatment of the sidebar panel (`variant` prop).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum SidebarVariant {
    /// Flush rail with an outer border (`sidebar`).
    #[default]
    Sidebar,
    /// Floating panel with padding, ring, and soft shadow.
    Floating,
    /// Inset panel that pairs with a rounded [`crate`] inset main area.
    Inset,
}

impl SidebarVariant {
    /// Every supported variant.
    pub const ALL: [Self; 3] = [Self::Sidebar, Self::Floating, Self::Inset];

    /// Whether the container adds the floating / inset padding ring.
    #[must_use]
    pub const fn is_padded(self) -> bool {
        matches!(self, Self::Floating | Self::Inset)
    }
}

/// Collapse behaviour when the desktop sidebar is closed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum SidebarCollapsible {
    /// Slides fully off-canvas (`width → 0`).
    #[default]
    Offcanvas,
    /// Shrinks to the icon rail (`3rem`).
    Icon,
    /// Never collapses — always expanded.
    None,
}

impl SidebarCollapsible {
    /// Every supported mode.
    pub const ALL: [Self; 3] = [Self::Offcanvas, Self::Icon, Self::None];
}

/// Expanded vs collapsed display state (`data-state`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum SidebarDisplayState {
    /// Desktop sidebar is open (`expanded`).
    Expanded,
    /// Desktop sidebar is closed (`collapsed`).
    Collapsed,
}

/// Pure sidebar controller shared by iced and egui backends.
///
/// Owns the open / mobile-open flags and the mobile breakpoint decision.
/// Callers feed viewport width (or an explicit `is_mobile` override) and
/// read derived layout metrics via [`Self::gap_width`] / [`Self::panel_width`].
#[derive(Debug, Clone, PartialEq)]
pub struct SidebarController {
    open: bool,
    open_mobile: bool,
    is_mobile: bool,
    width_px: f32,
    width_mobile_px: f32,
    width_icon_px: f32,
}

impl Default for SidebarController {
    fn default() -> Self {
        Self::new(true)
    }
}

impl SidebarController {
    /// Creates a controller with the given desktop open state.
    #[must_use]
    pub fn new(open: bool) -> Self {
        Self {
            open,
            open_mobile: false,
            is_mobile: false,
            width_px: SIDEBAR_WIDTH_PX,
            width_mobile_px: SIDEBAR_WIDTH_MOBILE_PX,
            width_icon_px: SIDEBAR_WIDTH_ICON_PX,
        }
    }

    /// Desktop open flag (`bind:open`).
    #[must_use]
    pub const fn open(&self) -> bool {
        self.open
    }

    /// Mobile sheet open flag.
    #[must_use]
    pub const fn open_mobile(&self) -> bool {
        self.open_mobile
    }

    /// Whether the layout is currently in the mobile breakpoint.
    #[must_use]
    pub const fn is_mobile(&self) -> bool {
        self.is_mobile
    }

    /// Expanded desktop width.
    #[must_use]
    pub const fn width_px(&self) -> f32 {
        self.width_px
    }

    /// Mobile sheet width.
    #[must_use]
    pub const fn width_mobile_px(&self) -> f32 {
        self.width_mobile_px
    }

    /// Collapsed icon-rail width.
    #[must_use]
    pub const fn width_icon_px(&self) -> f32 {
        self.width_icon_px
    }

    /// Sets the desktop open flag.
    pub fn set_open(&mut self, open: bool) {
        self.open = open;
    }

    /// Sets the mobile sheet open flag.
    pub fn set_open_mobile(&mut self, open: bool) {
        self.open_mobile = open;
    }

    /// Overrides the mobile flag explicitly.
    pub fn set_is_mobile(&mut self, is_mobile: bool) {
        self.is_mobile = is_mobile;
        if !is_mobile {
            self.open_mobile = false;
        }
    }

    /// Updates [`Self::is_mobile`] from a viewport width.
    pub fn set_viewport_width(&mut self, width_px: f32) {
        self.set_is_mobile(width_px > 0.0 && width_px < SIDEBAR_MOBILE_BREAKPOINT_PX);
    }

    /// Overrides the expanded desktop width (must be finite and `> 0`).
    pub fn set_width_px(&mut self, width_px: f32) {
        if width_px.is_finite() && width_px > 0.0 {
            self.width_px = width_px;
        }
    }

    /// Overrides the mobile sheet width (must be finite and `> 0`).
    pub fn set_width_mobile_px(&mut self, width_px: f32) {
        if width_px.is_finite() && width_px > 0.0 {
            self.width_mobile_px = width_px;
        }
    }

    /// Overrides the icon-rail width (must be finite and `> 0`).
    pub fn set_width_icon_px(&mut self, width_px: f32) {
        if width_px.is_finite() && width_px > 0.0 {
            self.width_icon_px = width_px;
        }
    }

    /// Toggles the appropriate open flag for the current breakpoint.
    pub fn toggle(&mut self) {
        if self.is_mobile {
            self.open_mobile = !self.open_mobile;
        } else {
            self.open = !self.open;
        }
    }

    /// `data-state` for the desktop sidebar.
    #[must_use]
    pub const fn display_state(&self) -> SidebarDisplayState {
        if self.open {
            SidebarDisplayState::Expanded
        } else {
            SidebarDisplayState::Collapsed
        }
    }

    /// Whether the desktop sidebar is collapsed.
    #[must_use]
    pub const fn is_collapsed(&self) -> bool {
        !self.open
    }

    /// Whether menu-button tooltips should show (collapsed desktop, not mobile).
    #[must_use]
    pub const fn show_menu_tooltip(&self) -> bool {
        self.is_collapsed() && !self.is_mobile
    }

    /// Layout gap reserved by the sidebar peer (`cn-sidebar-gap`).
    #[must_use]
    pub fn gap_width(&self, collapsible: SidebarCollapsible, variant: SidebarVariant) -> f32 {
        sidebar_gap_width(
            self.open,
            collapsible,
            variant,
            self.width_px,
            self.width_icon_px,
        )
    }

    /// Visible panel width for the desktop container.
    #[must_use]
    pub fn panel_width(&self, collapsible: SidebarCollapsible, variant: SidebarVariant) -> f32 {
        sidebar_panel_width(
            self.open,
            collapsible,
            variant,
            self.width_px,
            self.width_icon_px,
        )
    }
}

/// Returns `true` when `key` matches the sidebar shortcut with Ctrl or Meta.
#[must_use]
pub fn matches_sidebar_shortcut(key: char, ctrl_or_meta: bool) -> bool {
    ctrl_or_meta && key.eq_ignore_ascii_case(&SIDEBAR_KEYBOARD_SHORTCUT)
}

/// Computes the peer gap width that pushes the inset content.
#[must_use]
pub fn sidebar_gap_width(
    open: bool,
    collapsible: SidebarCollapsible,
    variant: SidebarVariant,
    width_px: f32,
    width_icon_px: f32,
) -> f32 {
    let width = width_px.max(0.0);
    let icon = width_icon_px.max(0.0);

    match collapsible {
        SidebarCollapsible::None => width,
        SidebarCollapsible::Offcanvas => {
            if open {
                width
            } else {
                0.0
            }
        }
        SidebarCollapsible::Icon => {
            if open {
                width
            } else if variant.is_padded() {
                icon + SIDEBAR_FLOATING_ICON_EXTRA_PX
            } else {
                icon
            }
        }
    }
}

/// Computes the visible desktop panel width (including floating padding).
#[must_use]
pub fn sidebar_panel_width(
    open: bool,
    collapsible: SidebarCollapsible,
    variant: SidebarVariant,
    width_px: f32,
    width_icon_px: f32,
) -> f32 {
    let width = width_px.max(0.0);
    let icon = width_icon_px.max(0.0);

    match collapsible {
        SidebarCollapsible::None => {
            if variant.is_padded() {
                width + SIDEBAR_FLOATING_PAD_PX * 2.0
            } else {
                width
            }
        }
        SidebarCollapsible::Offcanvas => {
            if open {
                if variant.is_padded() {
                    width + SIDEBAR_FLOATING_PAD_PX * 2.0
                } else {
                    width
                }
            } else {
                0.0
            }
        }
        SidebarCollapsible::Icon => {
            if open {
                if variant.is_padded() {
                    width + SIDEBAR_FLOATING_PAD_PX * 2.0
                } else {
                    width
                }
            } else if variant.is_padded() {
                icon + SIDEBAR_FLOATING_ICON_EXTRA_PX + 2.0
            } else {
                icon
            }
        }
    }
}

/// Interpolates gap width during the open/close animation (`t` in `0..=1`).
#[must_use]
pub fn lerp_sidebar_gap(
    open_progress: f32,
    collapsible: SidebarCollapsible,
    variant: SidebarVariant,
    width_px: f32,
    width_icon_px: f32,
) -> f32 {
    let t = open_progress.clamp(0.0, 1.0);
    let closed = sidebar_gap_width(false, collapsible, variant, width_px, width_icon_px);
    let opened = sidebar_gap_width(true, collapsible, variant, width_px, width_icon_px);
    closed + (opened - closed) * t
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toggle_respects_mobile() {
        let mut ctrl = SidebarController::new(true);
        ctrl.toggle();
        assert!(!ctrl.open());
        assert!(!ctrl.open_mobile());

        ctrl.set_is_mobile(true);
        ctrl.toggle();
        assert!(!ctrl.open());
        assert!(ctrl.open_mobile());
    }

    #[test]
    fn viewport_sets_mobile_flag() {
        let mut ctrl = SidebarController::new(true);
        ctrl.set_viewport_width(400.0);
        assert!(ctrl.is_mobile());
        ctrl.set_viewport_width(1024.0);
        assert!(!ctrl.is_mobile());
        assert!(!ctrl.open_mobile());
    }

    #[test]
    fn offcanvas_gap_collapses_to_zero() {
        assert_eq!(
            sidebar_gap_width(
                false,
                SidebarCollapsible::Offcanvas,
                SidebarVariant::Sidebar,
                SIDEBAR_WIDTH_PX,
                SIDEBAR_WIDTH_ICON_PX
            ),
            0.0
        );
        assert_eq!(
            sidebar_gap_width(
                true,
                SidebarCollapsible::Offcanvas,
                SidebarVariant::Sidebar,
                SIDEBAR_WIDTH_PX,
                SIDEBAR_WIDTH_ICON_PX
            ),
            SIDEBAR_WIDTH_PX
        );
    }

    #[test]
    fn icon_gap_keeps_rail() {
        assert_eq!(
            sidebar_gap_width(
                false,
                SidebarCollapsible::Icon,
                SidebarVariant::Sidebar,
                SIDEBAR_WIDTH_PX,
                SIDEBAR_WIDTH_ICON_PX
            ),
            SIDEBAR_WIDTH_ICON_PX
        );
        assert_eq!(
            sidebar_gap_width(
                false,
                SidebarCollapsible::Icon,
                SidebarVariant::Floating,
                SIDEBAR_WIDTH_PX,
                SIDEBAR_WIDTH_ICON_PX
            ),
            SIDEBAR_WIDTH_ICON_PX + SIDEBAR_FLOATING_ICON_EXTRA_PX
        );
    }

    #[test]
    fn shortcut_matches_ctrl_b() {
        assert!(matches_sidebar_shortcut('b', true));
        assert!(matches_sidebar_shortcut('B', true));
        assert!(!matches_sidebar_shortcut('b', false));
        assert!(!matches_sidebar_shortcut('x', true));
    }

    #[test]
    fn tooltip_only_when_collapsed_desktop() {
        let mut ctrl = SidebarController::new(false);
        assert!(ctrl.show_menu_tooltip());
        ctrl.set_is_mobile(true);
        assert!(!ctrl.show_menu_tooltip());
    }
}
