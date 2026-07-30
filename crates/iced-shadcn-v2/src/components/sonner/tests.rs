//! Public API and builder tests for Sonner.

use super::*;
use crate::iced_compat::Element;
use crate::theme::Theme;

#[test]
fn toast_kind_and_position_defaults_match_sonner() {
    assert_eq!(ToastType::default(), ToastType::Default);
    assert_eq!(ToastType::Success.as_str(), "success");
    assert_eq!(ToastPosition::default(), ToastPosition::BottomRight);
    assert!(ToastPosition::TopCenter.is_top());
    assert!(ToastPosition::BottomLeft.is_left());
    assert!(ToastPosition::BottomRight.is_right());
    assert!(ToastPosition::BottomCenter.is_center_x());
}

#[test]
fn toast_options_builder_keeps_configured_values() {
    let options = ToastOptions::new(ToastType::Success)
        .id(ToastId::from(7))
        .description("A description")
        .duration(5_000)
        .dismissible(false)
        .action(ToastAction::label("Undo"))
        .cancel(ToastAction::label("Cancel"))
        .close_button(true)
        .rich_colors(true)
        .invert(true)
        .position(ToastPosition::TopCenter)
        .important(true);

    assert_eq!(options.id_value(), Some(ToastId::from(7)));
    assert_eq!(options.toast_type_value(), ToastType::Success);
    assert_eq!(options.description_text(), Some("A description"));
    assert_eq!(options.duration_ms(), Some(5_000));
    assert!(!options.is_dismissible());
    assert_eq!(
        options.action_ref().map(ToastAction::label_text),
        Some("Undo")
    );
    assert_eq!(
        options.cancel_ref().map(ToastAction::label_text),
        Some("Cancel")
    );
    assert!(options.has_close_button());
    assert!(options.uses_rich_colors());
    assert!(options.is_inverted());
    assert_eq!(options.position_override(), Some(ToastPosition::TopCenter));
    assert!(options.is_important());
}

#[test]
fn toast_builder_exposes_the_expected_fluent_api() {
    let toast = toast("Hello")
        .description("World")
        .toast_type(ToastType::Info)
        .duration(3_000)
        .close_button(true)
        .rich_colors(true)
        .invert(true)
        .position(ToastPosition::TopLeft);

    assert!(!toast.id().is_zero());
    assert_eq!(toast.title(), "Hello");
    assert_eq!(toast.options().toast_type_value(), ToastType::Info);
    assert_eq!(toast.options().description_text(), Some("World"));
    assert_eq!(toast.options().duration_ms(), Some(3_000));
    assert!(toast.options().has_close_button());
}

#[test]
fn typed_action_and_callback_are_debuggable_without_message_debug() {
    #[derive(Clone)]
    struct Message;

    let action = ToastAction::new("Undo", || Message);
    let callback = ToastCallback::new(|| Message);

    assert_eq!(action.label_text(), "Undo");
    assert!(action.has_callback());
    assert!(format!("{action:?}").contains("ToastAction"));
    assert!(format!("{callback:?}").contains("ToastCallback"));
}

#[test]
fn toaster_is_a_theme_aware_iced_element() {
    struct NoDebug;

    let theme = Theme::light();
    let toaster = Toaster::<NoDebug>::new(&theme)
        .position(ToastPosition::TopCenter)
        .duration(2_000)
        .gap(8.0)
        .offset(16.0)
        .width(420.0)
        .visible_toasts(5)
        .rich_colors(true)
        .invert(true)
        .close_button(true)
        .expand(true)
        .pause_on_hover(false)
        .pause_when_page_is_hidden(false)
        .animated(false);

    assert_eq!(toaster.position, ToastPosition::TopCenter);
    assert_eq!(toaster.duration_ms, 2_000);
    assert_eq!(toaster.gap, 8.0);
    assert_eq!(toaster.offset, 16.0);
    assert_eq!(toaster.width, 420.0);
    assert_eq!(toaster.visible_toasts, 5);
    assert!(toaster.rich_colors);
    assert!(toaster.invert);
    assert!(toaster.close_button);
    assert!(toaster.expand);
    assert!(!toaster.pause_on_hover);
    assert!(!toaster.pause_when_page_is_hidden);
    assert!(!toaster.animated);
    assert!(format!("{toaster:?}").contains("Toaster"));

    let _: Element<'_, NoDebug> = toaster.into();
}

#[test]
fn process_wide_queue_supports_show_update_and_promise_resolution() {
    let _guard = super::state::TEST_LOCK
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    super::state::clear_all_toasts();

    let id = toast("First").show();
    assert_eq!(active_toast_count(), 1);

    let updated = update_toast(id, toast("Updated").toast_type(ToastType::Success));
    assert_eq!(updated, id);
    assert_eq!(active_toast_count(), 1);

    let promise = toast_promise("Loading");
    let promise_id = promise.id();
    assert_eq!(promise_id, ToastId::from(promise_id.as_u64()));
    let resolved = promise.success("Done");
    assert_eq!(resolved, promise_id);
    assert_eq!(active_toast_count(), 2);

    dismiss_toast(id);
    dismiss_all_toasts();
    super::state::clear_all_toasts();
}

#[test]
fn duration_and_layout_values_are_clamped() {
    let theme = Theme::light();
    let toaster = Toaster::<()>::new(&theme)
        .gap(-5.0)
        .offset(-10.0)
        .width(-1.0)
        .visible_toasts(0);

    assert_eq!(toaster.gap, 0.0);
    assert_eq!(toaster.offset, 0.0);
    assert_eq!(toaster.width, 180.0);
    assert_eq!(toaster.visible_toasts, 1);
}
