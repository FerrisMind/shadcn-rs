//! Behavioral tests for the drawer component.

use shadcn_common::{
    DRAWER_ANIMATION_MS, DRAWER_HANDLE_HEIGHT_PX, DRAWER_MAX_WIDTH_PX, DrawerDirection, StyleId,
    drawer_recipe,
};

use super::style;
use super::types::DrawerState;
use super::*;
use crate::iced_compat::widget::container;
use crate::iced_compat::{Element, time::Duration};
use crate::theme::Theme;

#[derive(Clone, Debug)]
#[allow(dead_code)]
enum Message {
    Pressed,
    OpenChanged(bool),
    SnapChanged(Option<f32>),
}

#[test]
fn builder_updates_semantic_fields() {
    let theme = Theme::light();
    let drawer: Drawer<'_, Message> = Drawer::new(container("Open"), container("Body"), &theme)
        .footer(DrawerFooter::new(&theme).push(container("Save")))
        .direction(DrawerDirection::Left)
        .max_width(320.0)
        .max_height(400.0)
        .duration(Duration::from_millis(100))
        .animated(false)
        .disabled(true)
        .open(true)
        .default_open(true)
        .on_open_change(Message::OpenChanged)
        .close_on_click_outside(false)
        .close_on_escape(false)
        .modal(false)
        .should_scale_background(false)
        .show_handle(false)
        .snap_points([0.4, 0.8])
        .active_snap_point(Some(0.4))
        .on_snap_point_change(Message::SnapChanged)
        .nested(true);

    assert!(drawer.footer.is_some());
    assert_eq!(drawer.direction, DrawerDirection::Left);
    assert_eq!(drawer.max_width, Some(320.0));
    assert_eq!(drawer.max_height, Some(400.0));
    assert_eq!(drawer.duration, Duration::from_millis(100));
    assert!(!drawer.animated);
    assert!(drawer.disabled);
    assert_eq!(drawer.open, Some(true));
    assert!(drawer.default_open);
    assert!(drawer.on_open_change.is_some());
    assert!(!drawer.close_on_click_outside);
    assert!(!drawer.close_on_escape);
    assert!(!drawer.modal);
    assert!(!drawer.should_scale_background);
    assert!(!drawer.show_handle);
    assert_eq!(drawer.snap_points, vec![0.4, 0.8]);
    assert_eq!(drawer.active_snap_point, Some(0.4));
    assert!(drawer.on_snap_point_change.is_some());
    assert!(drawer.nested);
    assert!(std::ptr::eq(drawer.theme, &theme));
}

#[test]
fn defaults_match_shadcn_svelte() {
    let theme = Theme::light();
    let drawer: Drawer<'_, Message> = Drawer::new(container("Open"), container("Body"), &theme);

    assert_eq!(drawer.direction, DrawerDirection::Bottom);
    assert_eq!(drawer.max_width, None);
    assert_eq!(drawer.max_height, None);
    assert_eq!(drawer.duration, Duration::from_millis(DRAWER_ANIMATION_MS));
    assert!(drawer.animated);
    assert!(!drawer.disabled);
    assert_eq!(drawer.open, None);
    assert!(!drawer.default_open);
    assert!(drawer.close_on_click_outside);
    assert!(drawer.close_on_escape);
    assert!(drawer.modal);
    assert!(drawer.should_scale_background);
    assert!(drawer.show_handle);
    assert!(drawer.snap_points.is_empty());
    assert!(drawer.active_snap_point.is_none());
    assert!(!drawer.nested);
    assert!(drawer.footer.is_none());
}

#[test]
fn drawers_and_slots_convert_to_elements() {
    let theme = Theme::light();

    let _: Element<'_, Message> = Drawer::new(
        container("Open"),
        DrawerHeader::new(&theme)
            .center(true)
            .title(DrawerTitle::text("Move Goal", &theme))
            .description(DrawerDescription::text(
                "Set your daily activity goal.",
                &theme,
            )),
        &theme,
    )
    .footer(DrawerFooter::new(&theme).push(container("Submit")))
    .direction(DrawerDirection::Bottom)
    .max_height(400.0)
    .open(true)
    .into();

    let _: Element<'_, Message> = DrawerBody::new(&theme).push(container("Lorem")).into();
}

#[test]
fn state_visibility_tracks_transition() {
    let mut state = DrawerState::new(false);
    assert!(!state.is_visible());
    assert_eq!(state.progress(), 0.0);

    state.open = true;
    state.transition.reset(1.0);
    assert!(state.is_visible());
    assert_eq!(state.progress(), 1.0);

    state.clear_drag();
    assert!(!state.dragging);
    assert_eq!(state.drag_offset, 0.0);
}

#[test]
fn style_resolution_tracks_packs() {
    let theme = Theme::light().with_style(StyleId::Vega);
    let style = style::resolve_style(&theme, DrawerDirection::Bottom);
    assert!((style.overlay.a - 0.10).abs() < f32::EPSILON);
    assert_eq!(style.handle_height_px, DRAWER_HANDLE_HEIGHT_PX);
    assert!(style.corner_mask.top_left);
    assert!(!style.corner_mask.bottom_left);

    let recipe = drawer_recipe(StyleId::Vega);
    assert_eq!(recipe.max_width_px, DRAWER_MAX_WIDTH_PX);

    let maia = Theme::light().with_style(StyleId::Maia);
    let maia_style = style::resolve_style(&maia, DrawerDirection::Bottom);
    assert!(maia_style.corner_mask.bottom_left);
    assert!(maia_style.floating_pad_px > 0.0);
}

#[test]
fn debug_impls_are_non_empty() {
    let theme = Theme::light();
    let drawer: Drawer<'_, Message> = Drawer::new(container("Open"), container("Body"), &theme);
    let text = format!("{drawer:?}");
    assert!(text.contains("Drawer"));
    assert!(!text.is_empty());
}
