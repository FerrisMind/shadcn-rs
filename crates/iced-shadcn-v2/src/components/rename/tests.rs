use std::rc::Rc;

use crate::iced_compat::widget::{self, text_editor};

use super::*;

fn non_empty(value: &str) -> bool {
    !value.trim().is_empty()
}

#[test]
fn start_edit_restores_value_and_requests_input_selection() {
    let mut state = RenameState::new("hello");
    state.editing_value = "stale".to_owned();

    let update = rename_apply_action(
        &mut state,
        RenameAction::StartEdit,
        RenameInputTag::Input,
        RenameFallbackSelectionBehavior::End,
        RenameBlurBehavior::Exit,
        non_empty,
    );

    assert_eq!(state.mode(), RenameMode::Edit);
    assert_eq!(state.editing_value(), "hello");
    assert_eq!(state.textarea_content().text(), "hello");
    assert!(update.entered_edit_mode());
    assert!(update.request_focus());
    assert_eq!(update.selection(), Some(RenameSelectionRequest::End));
}

#[test]
fn invalid_input_is_reported_without_committing() {
    let mut state = RenameState::new("before");
    state.set_mode(RenameMode::Edit);

    let _ = rename_apply_action(
        &mut state,
        RenameAction::InputChanged(String::new()),
        RenameInputTag::Input,
        RenameFallbackSelectionBehavior::End,
        RenameBlurBehavior::Exit,
        non_empty,
    );
    let update = rename_apply_action(
        &mut state,
        RenameAction::SaveRequested,
        RenameInputTag::Input,
        RenameFallbackSelectionBehavior::End,
        RenameBlurBehavior::Exit,
        non_empty,
    );

    assert_eq!(state.mode(), RenameMode::Edit);
    assert_eq!(state.value(), "before");
    assert!(state.is_invalid());
    assert!(update.validation_failed());
    assert!(!update.committed());
}

#[test]
fn valid_save_commits_and_rebuilds_textarea_content() {
    let mut state = RenameState::new("before");
    state.set_mode(RenameMode::Edit);

    let _ = rename_apply_action(
        &mut state,
        RenameAction::InputChanged("after".to_owned()),
        RenameInputTag::Input,
        RenameFallbackSelectionBehavior::End,
        RenameBlurBehavior::Exit,
        non_empty,
    );
    let update = rename_apply_action(
        &mut state,
        RenameAction::SaveRequested,
        RenameInputTag::Input,
        RenameFallbackSelectionBehavior::End,
        RenameBlurBehavior::Exit,
        non_empty,
    );

    assert_eq!(state.mode(), RenameMode::View);
    assert_eq!(state.value(), "after");
    assert_eq!(state.textarea_content().text(), "after");
    assert!(!state.is_invalid());
    assert!(update.committed());
}

#[test]
fn cancel_escape_and_exit_blur_restore_the_accepted_value() {
    let mut state = RenameState::new("before");
    state.set_mode(RenameMode::Edit);
    state.editing_value = "changed".to_owned();

    let cancel = rename_apply_action(
        &mut state,
        RenameAction::CancelRequested,
        RenameInputTag::Input,
        RenameFallbackSelectionBehavior::End,
        RenameBlurBehavior::Exit,
        non_empty,
    );
    assert!(cancel.cancelled());
    assert_eq!(state.mode(), RenameMode::View);
    assert_eq!(state.editing_value(), "before");

    state.set_mode(RenameMode::Edit);
    state.editing_value = "changed again".to_owned();
    let escape = rename_apply_action(
        &mut state,
        RenameAction::EscapePressed,
        RenameInputTag::Input,
        RenameFallbackSelectionBehavior::End,
        RenameBlurBehavior::Exit,
        non_empty,
    );
    assert!(escape.cancelled());
    assert_eq!(state.editing_value(), "before");

    state.set_mode(RenameMode::Edit);
    state.editing_value = "keep me".to_owned();
    let blur = rename_apply_action(
        &mut state,
        RenameAction::BlurDetected,
        RenameInputTag::Input,
        RenameFallbackSelectionBehavior::End,
        RenameBlurBehavior::None,
        non_empty,
    );
    assert!(!blur.cancelled());
    assert_eq!(state.mode(), RenameMode::Edit);
    assert_eq!(state.editing_value(), "keep me");
}

#[test]
fn textarea_actions_keep_the_controlled_buffer_in_sync() {
    let mut state = RenameState::new("");
    state.set_mode(RenameMode::Edit);

    let _ = rename_apply_action(
        &mut state,
        RenameAction::TextareaEdited(text_editor::Action::Edit(text_editor::Edit::Insert('a'))),
        RenameInputTag::Textarea,
        RenameFallbackSelectionBehavior::End,
        RenameBlurBehavior::Exit,
        non_empty,
    );

    assert_eq!(state.textarea_content().text(), "a");
    assert_eq!(state.editing_value(), "a");
    assert!(!state.is_invalid());
}

#[test]
fn provider_defaults_to_no_blur_and_preserves_explicit_root_behavior() {
    let state = RenameState::new("value");
    let action_handler: RenameActionHandler<'_, ()> = Rc::new(|_| ());

    let _ = rename_provider(
        &state,
        Some(action_handler),
        RenameProviderProps::default(),
        |context| {
            assert_eq!(context.blur_behavior(), RenameBlurBehavior::None);
            assert!(!context.is_disabled());

            let inherited = context.root_props(RenameRootProps::default());
            assert_eq!(inherited.blur_behavior, RenameBlurBehavior::None);
            assert!(!inherited.click_to_edit);

            let explicit = context
                .root_props(RenameRootProps::default().blur_behavior(RenameBlurBehavior::Exit));
            assert_eq!(explicit.blur_behavior, RenameBlurBehavior::Exit);
            assert!(!explicit.click_to_edit);

            let explicit_click = context.root_props(
                RenameRootProps::default()
                    .click_to_edit(true)
                    .blur_behavior(RenameBlurBehavior::Exit),
            );
            assert!(explicit_click.click_to_edit);

            widget::text("").into()
        },
    );
}

#[test]
fn root_props_normalize_non_finite_text_sizes() {
    let props = RenameRootProps::default()
        .text_size(f32::NAN)
        .text_line_height(f32::INFINITY);

    assert_eq!(props.text_size, Some(1.0));
    assert_eq!(props.text_line_height, Some(1.0));
}

#[test]
fn root_id_and_input_id_are_independent_operation_targets() {
    let props = RenameRootProps::default()
        .id("rename-root")
        .input_id("rename-input");

    assert_eq!(props.id, widget::Id::new("rename-root"));
    assert_eq!(props.input_id_value(), widget::Id::new("rename-input"));
}

#[test]
fn external_button_props_accept_custom_content() {
    let props = RenameButtonProps::<()>::default().content(widget::text("custom"));

    assert!(props.content.is_some());
    assert!(format!("{props:?}").contains("content: true"));
}
