//! Unit tests for the sidebar builders and style wiring.

use shadcn_common::{
    SIDEBAR_WIDTH_ICON_PX, SIDEBAR_WIDTH_PX, SidebarCollapsible, SidebarController, SidebarVariant,
    sidebar_gap_width,
};

use super::style::resolve_style;
use super::{
    Sidebar, SidebarHeader, SidebarMenuButton, SidebarMenuButtonVariant, SidebarProvider,
    SidebarTrigger,
};
use crate::theme::Theme;

#[test]
fn provider_debug_lists_children() {
    let theme = Theme::light();
    let provider = SidebarProvider::<()>::new(&theme);
    let debug = format!("{provider:?}");
    assert!(debug.contains("SidebarProvider"));
    assert!(debug.contains("children: 0"));
}

#[test]
fn sidebar_defaults_match_web() {
    let theme = Theme::light();
    let ctrl = SidebarController::new(true);
    let sidebar = Sidebar::<()>::new(&ctrl, &theme);
    assert_eq!(sidebar.side, super::SidebarSide::Left);
    assert_eq!(sidebar.variant, SidebarVariant::Sidebar);
    assert_eq!(sidebar.collapsible, SidebarCollapsible::Offcanvas);
}

#[test]
fn style_uses_sidebar_palette() {
    let theme = Theme::light();
    let style = resolve_style(&theme);
    assert_eq!(style.background, theme.palette.sidebar);
    assert_eq!(style.foreground, theme.palette.sidebar_foreground);
    assert_eq!(style.accent, theme.palette.sidebar_accent);
    assert_eq!(style.border, theme.palette.sidebar_border);
    assert!(style.recipe.menu_button_height_px > 0.0);
}

#[test]
fn icon_collapse_gap_matches_common() {
    let gap = sidebar_gap_width(
        false,
        SidebarCollapsible::Icon,
        SidebarVariant::Sidebar,
        SIDEBAR_WIDTH_PX,
        SIDEBAR_WIDTH_ICON_PX,
    );
    assert_eq!(gap, SIDEBAR_WIDTH_ICON_PX);
}

#[test]
fn menu_button_builder_retains_active_and_tooltip() {
    let theme = Theme::light();
    let ctrl = SidebarController::new(false);
    let button = SidebarMenuButton::<()>::text("Playground", &ctrl, &theme)
        .active(true)
        .tooltip("Playground")
        .variant(SidebarMenuButtonVariant::Outline);
    assert!(button.active);
    assert_eq!(button.tooltip.as_deref(), Some("Playground"));
    assert_eq!(button.variant, SidebarMenuButtonVariant::Outline);
    assert!(ctrl.show_menu_tooltip());
}

#[test]
fn header_and_trigger_build() {
    let theme = Theme::light();
    let _header = SidebarHeader::<()>::new(&theme);
    let _trigger = SidebarTrigger::<()>::new(&theme);
}

#[test]
fn sidebar_animation_defaults_on() {
    let theme = Theme::light();
    let ctrl = SidebarController::new(true);
    let sidebar = Sidebar::<()>::new(&ctrl, &theme);
    assert!(sidebar.animated);
    let muted = Sidebar::<()>::new(&ctrl, &theme).animated(false);
    assert!(!muted.animated);
}

#[test]
fn provider_viewport_callback_wires() {
    let theme = Theme::light();
    let provider = SidebarProvider::<f32>::new(&theme).on_viewport_change(|w| w);
    assert!(provider.on_viewport_change.is_some());
}
