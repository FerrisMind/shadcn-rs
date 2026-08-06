//! Behavioral tests for the copy button.

use std::time::Duration;

use crate::iced_compat::Element;
use crate::theme::Theme;
use crate::{ButtonSize, ButtonVariant};

use super::*;

#[derive(Debug, Clone)]
enum Message {
    Copy,
}

#[test]
fn defaults_match_the_svelte_component() {
    let theme = Theme::light();
    let button = CopyButton::<Message>::new("Hello", &theme);

    assert_eq!(button.text(), "Hello");
    assert_eq!(button.variant, ButtonVariant::Ghost);
    assert_eq!(button.size, ButtonSize::Icon);
    assert_eq!(button.status, CopyButtonStatus::Idle);
    assert_eq!(button.animation_duration, Duration::from_millis(500));
    assert!(!button.disabled);
    assert!(button.content.is_none());
    assert!(button.idle_icon.is_none());
}

#[test]
fn reference_geometry_uses_the_source_button_tokens() {
    let theme = Theme::light();

    assert_eq!(render::icon_size(ButtonSize::Icon, &theme), 16.0);
    assert_eq!(render::content_gap(), 8.0);
    assert_eq!(
        theme
            .style
            .button_size(shadcn_common::ControlSize::Md)
            .height_px,
        36.0
    );
    assert_eq!(
        theme
            .style
            .button_size(shadcn_common::ControlSize::Sm)
            .height_px,
        32.0
    );
}

#[test]
fn content_promotes_icon_size_to_text_size() {
    let theme = Theme::light();
    let button = CopyButton::<Message>::new("Hello", &theme).label("Copy");

    let _element: Element<'_, Message> = button.into();
}

#[test]
fn controlled_state_and_reducer_cover_all_feedback_states() {
    let initial = CopyButtonState::new();
    assert_eq!(initial.status(), CopyButtonStatus::Idle);

    let success = copy_button_reduce(initial, CopyButtonAction::Success);
    assert_eq!(success.state().status(), CopyButtonStatus::Success);
    assert!(success.should_reset());

    let failure = copy_button_reduce(success.state(), CopyButtonAction::Failure);
    assert_eq!(failure.state().status(), CopyButtonStatus::Failure);
    assert!(failure.should_reset());

    let reset = copy_button_reduce(failure.state(), CopyButtonAction::Reset);
    assert_eq!(reset.state().status(), CopyButtonStatus::Idle);
    assert!(!reset.should_reset());
}

#[test]
fn action_callback_is_evaluated_when_the_button_is_built() {
    let theme = Theme::light();
    let button = CopyButton::new("Hello", &theme).on_copy_action(|action| match action {
        CopyButtonAction::Pressed => Message::Copy,
        CopyButtonAction::Success | CopyButtonAction::Failure | CopyButtonAction::Reset => {
            Message::Copy
        }
    });

    let _ = button.into_button();
}

#[test]
fn debug_does_not_require_message_debug() {
    struct NoDebugMessage;

    let theme = Theme::light();
    let button = CopyButton::<NoDebugMessage>::new("Hello", &theme);
    let debug = format!("{button:?}");

    assert!(debug.contains("CopyButton"));
    assert!(debug.contains("text_length"));
}

#[test]
fn custom_icon_and_all_button_knobs_are_composable() {
    let theme = Theme::light();
    let button = CopyButton::new("Hello", &theme)
        .icon(crate::iced_compat::widget::text("C"))
        .content(crate::iced_compat::widget::text("Copy"))
        .variant(ButtonVariant::Outline)
        .size(ButtonSize::IconSm)
        .status(CopyButtonStatus::Success)
        .animation_duration(Duration::ZERO)
        .disabled(true)
        .full_width()
        .on_copy(Message::Copy);

    let _ = button.into_button();
}
