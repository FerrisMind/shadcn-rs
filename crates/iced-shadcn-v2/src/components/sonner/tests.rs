//! Behavioral tests for the sonner component.

use super::*;
use crate::theme::Theme;

#[test]
fn toast_type_default_is_default() {
    assert_eq!(ToastType::default(), ToastType::Default);
}

#[test]
fn toast_position_default_is_bottom_right() {
    assert_eq!(ToastPosition::default(), ToastPosition::BottomRight);
}

#[test]
fn toast_position_queries() {
    assert!(ToastPosition::TopRight.is_top());
    assert!(ToastPosition::TopLeft.is_top());
    assert!(ToastPosition::TopCenter.is_top());
    assert!(!ToastPosition::BottomRight.is_top());

    assert!(ToastPosition::BottomRight.is_bottom());
    assert!(ToastPosition::BottomLeft.is_bottom());
    assert!(ToastPosition::BottomCenter.is_bottom());
    assert!(!ToastPosition::TopRight.is_bottom());

    assert!(ToastPosition::TopLeft.is_left());
    assert!(ToastPosition::BottomLeft.is_left());
    assert!(!ToastPosition::TopRight.is_left());

    assert!(ToastPosition::TopRight.is_right());
    assert!(ToastPosition::BottomRight.is_right());
    assert!(!ToastPosition::TopLeft.is_right());

    assert!(ToastPosition::TopCenter.is_center_x());
    assert!(ToastPosition::BottomCenter.is_center_x());
    assert!(!ToastPosition::TopLeft.is_center_x());
}

#[test]
fn toast_options_builder() {
    let options: ToastOptions<()> = ToastOptions::new(ToastType::Success)
        .description("A description")
        .duration(5000)
        .dismissible(false)
        .close_button(true)
        .rich_colors(true)
        .invert(true)
        .position(ToastPosition::TopCenter);

    assert_eq!(options.toast_type, ToastType::Success);
    assert_eq!(options.description.as_deref(), Some("A description"));
    assert_eq!(options.duration, Some(5000));
    assert!(!options.dismissible);
    assert!(options.close_button);
    assert!(options.rich_colors);
    assert!(options.invert);
    assert_eq!(options.position, Some(ToastPosition::TopCenter));
}

#[test]
fn toast_action_label_only() {
    let action: ToastAction<()> = ToastAction::label("Undo");
    assert_eq!(action.label, "Undo");
    assert!(action.on_click.is_none());
}

#[test]
fn toast_action_with_callback() {
    let action: ToastAction<()> = ToastAction::new("Undo", |_| Some(()));
    assert_eq!(action.label, "Undo");
    assert!(action.on_click.is_some());
}

#[test]
fn sonner_toast_builder() {
    let t: SonnerToast<()> = toast("Hello")
        .description("World")
        .toast_type(ToastType::Success)
        .duration(3000)
        .close_button(true);

    assert_eq!(t.title(), "Hello");
    assert_eq!(t.options().toast_type, ToastType::Success);
    assert_eq!(t.options().description.as_deref(), Some("World"));
    assert_eq!(t.options().duration, Some(3000));
    assert!(t.options().close_button);
}

#[test]
fn sonner_toast_show_adds_to_global_state() {
    super::state::clear_all_toasts();
    let t: SonnerToast<()> = toast("Test").toast_type(ToastType::Info);
    let id = t.show();
    assert!(id > 0);
    assert_eq!(super::state::active_toast_count(), 1);
    super::state::clear_all_toasts();
}

#[test]
fn toaster_builder_defaults() {
    let theme = Theme::light();
    let toaster: Toaster<'_, ()> = Toaster::new(&theme);
    assert_eq!(toaster.position, ToastPosition::BottomRight);
    assert_eq!(toaster.duration_ms, style::DEFAULT_DURATION_MS);
    assert_eq!(toaster.gap, style::TOAST_GAP);
    assert_eq!(toaster.offset, style::TOAST_OFFSET);
    assert_eq!(toaster.max_visible, style::MAX_VISIBLE_TOASTS);
    assert!(!toaster.rich_colors);
    assert!(!toaster.invert);
    assert!(!toaster.close_button);
    assert!(!toaster.expand);
}

#[test]
fn toaster_builder_configurable() {
    let theme = Theme::light();
    let toaster: Toaster<'_, ()> = Toaster::new(&theme)
        .position(ToastPosition::TopCenter)
        .duration(2000)
        .gap(8.0)
        .offset(16.0)
        .max_visible(5)
        .rich_colors(true)
        .invert(true)
        .close_button(true)
        .expand(true);

    assert_eq!(toaster.position, ToastPosition::TopCenter);
    assert_eq!(toaster.duration_ms, 2000);
    assert_eq!(toaster.gap, 8.0);
    assert_eq!(toaster.offset, 16.0);
    assert_eq!(toaster.max_visible, 5);
    assert!(toaster.rich_colors);
    assert!(toaster.invert);
    assert!(toaster.close_button);
    assert!(toaster.expand);
}

#[test]
fn toaster_converts_to_element() {
    let theme = Theme::light();
    let _: Element<'_, ()> = Toaster::new(&theme).into();
}

#[test]
fn debug_does_not_require_message_debug() {
    struct NoDebug;

    let theme = Theme::light();
    let toaster = Toaster::<NoDebug>::new(&theme);
    let debug = format!("{toaster:?}");
    assert!(debug.contains("Toaster"));

    let t = toast::<NoDebug>("Hello");
    let debug = format!("{t:?}");
    assert!(debug.contains("SonnerToast"));
}

#[test]
fn gap_clamped_to_non_negative() {
    let theme = Theme::light();
    let toaster: Toaster<'_, ()> = Toaster::new(&theme).gap(-5.0);
    assert_eq!(toaster.gap, 0.0);
}

#[test]
fn offset_clamped_to_non_negative() {
    let theme = Theme::light();
    let toaster: Toaster<'_, ()> = Toaster::new(&theme).offset(-10.0);
    assert_eq!(toaster.offset, 0.0);
}

#[test]
fn max_visible_clamped_to_at_least_one() {
    let theme = Theme::light();
    let toaster: Toaster<'_, ()> = Toaster::new(&theme).max_visible(0);
    assert_eq!(toaster.max_visible, 1);
}
