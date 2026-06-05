use std::rc::Rc;

use iced::advanced::layout;
use iced::advanced::renderer;
use iced::advanced::widget::Tree;
use iced::advanced::{Clipboard, Layout, Shell, Widget};
use iced::keyboard::key::{self, Key};
use iced::mouse;
use iced::touch;
use iced::widget::{Id, column, container, text, text_editor};
use iced::{Element, Event, Length, Rectangle, Size, Task, keyboard};

use crate::button::{ButtonProps, ButtonVariant, button};
use crate::input::{InputProps, input};
use crate::textarea::{TextareaProps, textarea};
use crate::theme::Theme;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum RenameMode {
    #[default]
    View,
    Edit,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum RenameInputTag {
    #[default]
    Input,
    Textarea,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum RenameBlurBehavior {
    #[default]
    Exit,
    None,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum RenameFallbackSelectionBehavior {
    Start,
    #[default]
    End,
    All,
}

#[derive(Clone, Debug, PartialEq)]
pub enum RenameAction {
    StartEdit,
    InputChanged(String),
    TextareaEdited(text_editor::Action),
    SaveRequested,
    CancelRequested,
    EscapePressed,
    BlurDetected,
}

#[derive(Debug)]
pub struct RenameState {
    pub mode: RenameMode,
    pub value: String,
    pub editing_value: String,
    pub textarea_content: text_editor::Content,
    pub invalid: bool,
}

impl RenameState {
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        let value = value.into();
        Self {
            mode: RenameMode::View,
            editing_value: value.clone(),
            textarea_content: text_editor::Content::with_text(&value),
            value,
            invalid: false,
        }
    }

    fn sync_textarea_content(&mut self) {
        self.textarea_content = text_editor::Content::with_text(&self.editing_value);
    }

    fn cancel_editing(&mut self) {
        self.mode = RenameMode::View;
        self.editing_value = self.value.clone();
        self.invalid = false;
        self.sync_textarea_content();
    }
}

impl Default for RenameState {
    fn default() -> Self {
        Self::new("")
    }
}

#[derive(Clone, Debug)]
pub struct RenameRootProps {
    pub id: Id,
    pub input_id: Id,
    pub input_tag: RenameInputTag,
    pub blur_behavior: RenameBlurBehavior,
    pub fallback_selection_behavior: RenameFallbackSelectionBehavior,
    pub input_props: InputProps,
    pub custom_height: Option<f32>,
    pub textarea_props: TextareaProps,
    pub disabled: bool,
    pub click_to_edit: bool,
}

impl RenameRootProps {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn id(mut self, id: impl Into<Id>) -> Self {
        self.id = id.into();
        self
    }

    #[must_use]
    pub fn input_id(mut self, input_id: impl Into<Id>) -> Self {
        self.input_id = input_id.into();
        self
    }

    #[must_use]
    pub fn input_tag(mut self, input_tag: RenameInputTag) -> Self {
        self.input_tag = input_tag;
        self
    }

    #[must_use]
    pub fn blur_behavior(mut self, blur_behavior: RenameBlurBehavior) -> Self {
        self.blur_behavior = blur_behavior;
        self
    }

    #[must_use]
    pub fn fallback_selection_behavior(
        mut self,
        fallback_selection_behavior: RenameFallbackSelectionBehavior,
    ) -> Self {
        self.fallback_selection_behavior = fallback_selection_behavior;
        self
    }

    #[must_use]
    pub fn input_props(mut self, input_props: InputProps) -> Self {
        self.input_props = input_props;
        self
    }

    #[must_use]
    pub fn custom_height(mut self, height: f32) -> Self {
        self.custom_height = Some(height.max(0.0));
        self
    }

    #[must_use]
    pub fn textarea_props(mut self, textarea_props: TextareaProps) -> Self {
        self.textarea_props = textarea_props;
        self
    }

    #[must_use]
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    #[must_use]
    pub fn click_to_edit(mut self, click_to_edit: bool) -> Self {
        self.click_to_edit = click_to_edit;
        self
    }
}

impl Default for RenameRootProps {
    fn default() -> Self {
        Self {
            id: Id::unique(),
            input_id: Id::unique(),
            input_tag: RenameInputTag::Input,
            blur_behavior: RenameBlurBehavior::Exit,
            fallback_selection_behavior: RenameFallbackSelectionBehavior::End,
            input_props: InputProps::default(),
            custom_height: None,
            textarea_props: TextareaProps::default(),
            disabled: false,
            click_to_edit: true,
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RenameProviderProps {
    pub disabled: bool,
}

impl RenameProviderProps {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

#[derive(Clone, Debug)]
pub struct RenameButtonProps {
    pub label: String,
    pub button_props: ButtonProps,
}

impl RenameButtonProps {
    #[must_use]
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            button_props: ButtonProps::default(),
        }
    }

    #[must_use]
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }

    #[must_use]
    pub fn button_props(mut self, button_props: ButtonProps) -> Self {
        self.button_props = button_props;
        self
    }
}

impl Default for RenameButtonProps {
    fn default() -> Self {
        Self::new("")
    }
}

pub type RenameActionHandler<'a, Message> = Rc<dyn Fn(RenameAction) -> Message + 'a>;

pub struct RenameContext<'a, Message> {
    pub mode: RenameMode,
    pub invalid: bool,
    on_action: Option<RenameActionHandler<'a, Message>>,
}

impl<'a, Message> RenameContext<'a, Message> {
    fn action_message(&self, action: RenameAction) -> Option<Message> {
        self.on_action.as_ref().map(|on_action| (on_action)(action))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenameSelectionRequest {
    Start,
    End,
    All,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RenameUpdate {
    pub entered_edit_mode: bool,
    pub committed: bool,
    pub cancelled: bool,
    pub validation_failed: bool,
    pub request_focus: bool,
    pub selection: Option<RenameSelectionRequest>,
}

#[must_use]
pub fn rename_provider<'a, Message: Clone + 'a>(
    state: &'a RenameState,
    on_action: Option<RenameActionHandler<'a, Message>>,
    props: RenameProviderProps,
    add_contents: impl FnOnce(&RenameContext<'a, Message>) -> Element<'a, Message>,
) -> Element<'a, Message> {
    let ctx = RenameContext {
        mode: state.mode,
        invalid: state.invalid,
        on_action: if props.disabled { None } else { on_action },
    };

    add_contents(&ctx)
}

#[must_use]
pub fn rename_edit<'a, Message: Clone + 'a>(
    ctx: &RenameContext<'a, Message>,
    theme: &Theme,
    props: RenameButtonProps,
) -> Element<'a, Message> {
    let disabled = matches!(ctx.mode, RenameMode::Edit) || props.button_props.disabled;
    let button_props = props
        .button_props
        .variant(ButtonVariant::Outline)
        .disabled(disabled);
    let on_press = if disabled {
        None
    } else {
        ctx.action_message(RenameAction::StartEdit)
    };

    button(props.label, on_press, button_props, theme).into()
}

#[must_use]
pub fn rename_save<'a, Message: Clone + 'a>(
    ctx: &RenameContext<'a, Message>,
    theme: &Theme,
    props: RenameButtonProps,
) -> Element<'a, Message> {
    let disabled = matches!(ctx.mode, RenameMode::View) || props.button_props.disabled;
    let button_props = props
        .button_props
        .variant(ButtonVariant::Default)
        .disabled(disabled);
    let on_press = if disabled {
        None
    } else {
        ctx.action_message(RenameAction::SaveRequested)
    };

    button(props.label, on_press, button_props, theme).into()
}

#[must_use]
pub fn rename_cancel<'a, Message: Clone + 'a>(
    ctx: &RenameContext<'a, Message>,
    theme: &Theme,
    props: RenameButtonProps,
) -> Element<'a, Message> {
    let disabled = matches!(ctx.mode, RenameMode::View) || props.button_props.disabled;
    let button_props = props
        .button_props
        .variant(ButtonVariant::Outline)
        .disabled(disabled);
    let on_press = if disabled {
        None
    } else {
        ctx.action_message(RenameAction::CancelRequested)
    };

    button(props.label, on_press, button_props, theme).into()
}

#[must_use]
pub fn rename_root<'a, Message: Clone + 'a>(
    state: &'a RenameState,
    on_action: Option<RenameActionHandler<'a, Message>>,
    props: RenameRootProps,
    theme: &'a Theme,
) -> Element<'a, Message> {
    let on_action = if props.disabled { None } else { on_action };
    let input_props = props.custom_height.map_or(props.input_props, |height| {
        props.input_props.custom_height(height)
    });
    let content: Element<'a, Message> = match (state.mode, props.input_tag) {
        (RenameMode::View, _) => container(text(state.value.as_str()))
            .width(Length::Fill)
            .padding([8.0, 12.0])
            .into(),
        (RenameMode::Edit, RenameInputTag::Input) => {
            let on_input = on_action.as_ref().map(|on_action| {
                let on_action = Rc::clone(on_action);
                move |value: String| (on_action)(RenameAction::InputChanged(value))
            });

            let mut text_input_widget =
                input(&state.editing_value, "", on_input, input_props, theme)
                    .id(props.input_id.clone())
                    .width(Length::Fill);

            if let Some(on_action) = on_action.as_ref() {
                text_input_widget =
                    text_input_widget.on_submit((on_action)(RenameAction::SaveRequested));
            }

            let input_element: Element<'a, Message> = text_input_widget.into();
            let input_element: Element<'a, Message> = if let Some(height) = props.custom_height {
                container(input_element)
                    .width(Length::Fill)
                    .height(Length::Fixed(height))
                    .into()
            } else {
                input_element
            };
            if state.invalid {
                let invalid = text("Invalid value")
                    .size(12)
                    .style(|_| iced::widget::text::Style {
                        color: Some(theme.palette.destructive),
                    });
                column![input_element, invalid].spacing(6).into()
            } else {
                input_element
            }
        }
        (RenameMode::Edit, RenameInputTag::Textarea) => {
            let on_textarea_action = on_action.as_ref().map(|on_action| {
                let on_action = Rc::clone(on_action);
                move |action: text_editor::Action| match action {
                    text_editor::Action::Edit(text_editor::Edit::Enter) => {
                        (on_action)(RenameAction::SaveRequested)
                    }
                    _ => (on_action)(RenameAction::TextareaEdited(action)),
                }
            });

            textarea(
                &state.textarea_content,
                "",
                on_textarea_action,
                props.textarea_props.invalid(state.invalid),
                theme,
            )
            .id(props.input_id.clone())
            .into()
        }
    };

    RenameRootWidget {
        content,
        mode: state.mode,
        on_action,
        click_to_edit: props.click_to_edit,
        disabled: props.disabled,
    }
    .into()
}

#[must_use]
pub fn rename_apply_action<F>(
    state: &mut RenameState,
    action: RenameAction,
    input_tag: RenameInputTag,
    fallback_selection_behavior: RenameFallbackSelectionBehavior,
    blur_behavior: RenameBlurBehavior,
    validate: F,
) -> RenameUpdate
where
    F: Fn(&str) -> bool,
{
    match action {
        RenameAction::StartEdit => {
            state.mode = RenameMode::Edit;
            state.editing_value = state.value.clone();
            state.invalid = !validate(&state.editing_value);
            state.sync_textarea_content();

            if matches!(input_tag, RenameInputTag::Textarea) {
                apply_textarea_selection(&mut state.textarea_content, fallback_selection_behavior);
                RenameUpdate {
                    entered_edit_mode: true,
                    request_focus: true,
                    ..RenameUpdate::default()
                }
            } else {
                RenameUpdate {
                    entered_edit_mode: true,
                    request_focus: true,
                    selection: Some(map_selection(fallback_selection_behavior)),
                    ..RenameUpdate::default()
                }
            }
        }
        RenameAction::InputChanged(value) => {
            state.editing_value = value;
            state.invalid = !validate(&state.editing_value);
            state.sync_textarea_content();
            RenameUpdate::default()
        }
        RenameAction::TextareaEdited(action) => {
            if matches!(input_tag, RenameInputTag::Textarea) {
                state.textarea_content.perform(action);
                state.editing_value = state.textarea_content.text();
                state.invalid = !validate(&state.editing_value);
            }
            RenameUpdate::default()
        }
        RenameAction::SaveRequested => {
            if validate(&state.editing_value) {
                state.value = state.editing_value.clone();
                state.mode = RenameMode::View;
                state.invalid = false;
                state.sync_textarea_content();
                RenameUpdate {
                    committed: true,
                    ..RenameUpdate::default()
                }
            } else {
                state.invalid = true;
                RenameUpdate {
                    validation_failed: true,
                    ..RenameUpdate::default()
                }
            }
        }
        RenameAction::CancelRequested | RenameAction::EscapePressed => {
            state.cancel_editing();
            RenameUpdate {
                cancelled: true,
                ..RenameUpdate::default()
            }
        }
        RenameAction::BlurDetected => {
            if matches!(blur_behavior, RenameBlurBehavior::Exit) {
                state.cancel_editing();
                RenameUpdate {
                    cancelled: true,
                    ..RenameUpdate::default()
                }
            } else {
                RenameUpdate::default()
            }
        }
    }
}

pub fn rename_update_task<Message: 'static>(
    props: &RenameRootProps,
    update: RenameUpdate,
) -> Task<Message> {
    if !update.request_focus {
        return Task::none();
    }

    let mut tasks = vec![iced::widget::operation::focus(props.input_id.clone())];

    if matches!(props.input_tag, RenameInputTag::Input)
        && let Some(selection) = update.selection
    {
        let selection_task = match selection {
            RenameSelectionRequest::Start => {
                iced::widget::operation::move_cursor_to_front(props.input_id.clone())
            }
            RenameSelectionRequest::End => {
                iced::widget::operation::move_cursor_to_end(props.input_id.clone())
            }
            RenameSelectionRequest::All => {
                iced::widget::operation::select_all(props.input_id.clone())
            }
        };
        tasks.push(selection_task);
    }

    Task::batch(tasks)
}

fn map_selection(
    fallback_selection_behavior: RenameFallbackSelectionBehavior,
) -> RenameSelectionRequest {
    match fallback_selection_behavior {
        RenameFallbackSelectionBehavior::Start => RenameSelectionRequest::Start,
        RenameFallbackSelectionBehavior::End => RenameSelectionRequest::End,
        RenameFallbackSelectionBehavior::All => RenameSelectionRequest::All,
    }
}

fn apply_textarea_selection(
    content: &mut text_editor::Content,
    fallback_selection_behavior: RenameFallbackSelectionBehavior,
) {
    match fallback_selection_behavior {
        RenameFallbackSelectionBehavior::Start => content.perform(text_editor::Action::Move(
            text_editor::Motion::DocumentStart,
        )),
        RenameFallbackSelectionBehavior::End => {
            content.perform(text_editor::Action::Move(text_editor::Motion::DocumentEnd))
        }
        RenameFallbackSelectionBehavior::All => content.perform(text_editor::Action::SelectAll),
    }
}

struct RenameRootWidget<'a, Message> {
    content: Element<'a, Message>,
    mode: RenameMode,
    on_action: Option<RenameActionHandler<'a, Message>>,
    click_to_edit: bool,
    disabled: bool,
}

impl<Message> Widget<Message, iced::Theme, iced::Renderer> for RenameRootWidget<'_, Message>
where
    Message: Clone,
{
    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&[self.content.as_widget()]);
    }

    fn size(&self) -> Size<Length> {
        self.content.as_widget().size()
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        self.content
            .as_widget_mut()
            .layout(&mut tree.children[0], renderer, limits)
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &iced::Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        self.content.as_widget_mut().update(
            &mut tree.children[0],
            event,
            layout,
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );

        if self.disabled {
            return;
        }

        let Some(on_action) = self.on_action.as_ref() else {
            return;
        };

        match (self.mode, event) {
            (RenameMode::View, Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)))
            | (RenameMode::View, Event::Touch(touch::Event::FingerPressed { .. }))
                if self.click_to_edit && cursor.is_over(layout.bounds()) =>
            {
                shell.publish((on_action)(RenameAction::StartEdit));
                shell.capture_event();
            }
            (RenameMode::Edit, Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)))
            | (RenameMode::Edit, Event::Touch(touch::Event::FingerPressed { .. }))
                if !cursor.is_over(layout.bounds()) =>
            {
                shell.publish((on_action)(RenameAction::BlurDetected));
            }
            (
                RenameMode::Edit,
                Event::Keyboard(keyboard::Event::KeyPressed {
                    key: Key::Named(key::Named::Escape),
                    ..
                }),
            ) => {
                shell.publish((on_action)(RenameAction::EscapePressed));
                shell.capture_event();
            }
            (
                RenameMode::Edit,
                Event::Keyboard(keyboard::Event::KeyPressed {
                    key: Key::Named(key::Named::Tab),
                    ..
                }),
            ) => {
                shell.publish((on_action)(RenameAction::BlurDetected));
            }
            _ => {}
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut iced::Renderer,
        theme: &iced::Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        self.content.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            style,
            layout,
            cursor,
            viewport,
        );
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        self.content.as_widget().mouse_interaction(
            &tree.children[0],
            layout,
            cursor,
            viewport,
            renderer,
        )
    }
}

impl<'a, Message: Clone + 'a> From<RenameRootWidget<'a, Message>> for Element<'a, Message> {
    fn from(widget: RenameRootWidget<'a, Message>) -> Self {
        Element::new(widget)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn non_empty(value: &str) -> bool {
        !value.trim().is_empty()
    }

    #[test]
    fn rename_root_props_builder_accepts_custom_height() {
        let props = RenameRootProps::new().custom_height(28.0);
        assert_eq!(props.custom_height, Some(28.0));
    }

    #[test]
    fn start_edit_syncs_editing_value_and_mode() {
        let mut state = RenameState::new("hello");
        state.editing_value = "stale".to_string();

        let update = rename_apply_action(
            &mut state,
            RenameAction::StartEdit,
            RenameInputTag::Input,
            RenameFallbackSelectionBehavior::End,
            RenameBlurBehavior::Exit,
            non_empty,
        );

        assert_eq!(state.mode, RenameMode::Edit);
        assert_eq!(state.editing_value, "hello");
        assert_eq!(state.textarea_content.text(), "hello");
        assert!(update.entered_edit_mode);
        assert!(update.request_focus);
        assert_eq!(update.selection, Some(RenameSelectionRequest::End));
    }

    #[test]
    fn save_requested_commits_when_valid() {
        let mut state = RenameState::new("before");
        state.mode = RenameMode::Edit;
        state.editing_value = "after".to_string();

        let update = rename_apply_action(
            &mut state,
            RenameAction::SaveRequested,
            RenameInputTag::Input,
            RenameFallbackSelectionBehavior::End,
            RenameBlurBehavior::Exit,
            non_empty,
        );

        assert_eq!(state.mode, RenameMode::View);
        assert_eq!(state.value, "after");
        assert!(!state.invalid);
        assert!(update.committed);
    }

    #[test]
    fn save_requested_stays_in_edit_when_invalid() {
        let mut state = RenameState::new("before");
        state.mode = RenameMode::Edit;
        state.editing_value = String::new();

        let update = rename_apply_action(
            &mut state,
            RenameAction::SaveRequested,
            RenameInputTag::Input,
            RenameFallbackSelectionBehavior::End,
            RenameBlurBehavior::Exit,
            non_empty,
        );

        assert_eq!(state.mode, RenameMode::Edit);
        assert_eq!(state.value, "before");
        assert!(state.invalid);
        assert!(update.validation_failed);
        assert!(!update.committed);
    }

    #[test]
    fn cancel_and_escape_revert_changes() {
        let mut state = RenameState::new("before");
        state.mode = RenameMode::Edit;
        state.editing_value = "changed".to_string();

        let cancel_update = rename_apply_action(
            &mut state,
            RenameAction::CancelRequested,
            RenameInputTag::Input,
            RenameFallbackSelectionBehavior::End,
            RenameBlurBehavior::Exit,
            non_empty,
        );

        assert_eq!(state.mode, RenameMode::View);
        assert_eq!(state.editing_value, "before");
        assert!(cancel_update.cancelled);

        state.mode = RenameMode::Edit;
        state.editing_value = "changed again".to_string();

        let escape_update = rename_apply_action(
            &mut state,
            RenameAction::EscapePressed,
            RenameInputTag::Input,
            RenameFallbackSelectionBehavior::End,
            RenameBlurBehavior::Exit,
            non_empty,
        );

        assert_eq!(state.mode, RenameMode::View);
        assert_eq!(state.editing_value, "before");
        assert!(escape_update.cancelled);
    }

    #[test]
    fn blur_detected_respects_blur_behavior() {
        let mut state = RenameState::new("name");
        state.mode = RenameMode::Edit;
        state.editing_value = "edited".to_string();

        let exit_update = rename_apply_action(
            &mut state,
            RenameAction::BlurDetected,
            RenameInputTag::Input,
            RenameFallbackSelectionBehavior::End,
            RenameBlurBehavior::Exit,
            non_empty,
        );

        assert_eq!(state.mode, RenameMode::View);
        assert_eq!(state.editing_value, "name");
        assert!(exit_update.cancelled);

        state.mode = RenameMode::Edit;
        state.editing_value = "edited".to_string();

        let none_update = rename_apply_action(
            &mut state,
            RenameAction::BlurDetected,
            RenameInputTag::Input,
            RenameFallbackSelectionBehavior::End,
            RenameBlurBehavior::None,
            non_empty,
        );

        assert_eq!(state.mode, RenameMode::Edit);
        assert_eq!(state.editing_value, "edited");
        assert!(!none_update.cancelled);
    }

    #[test]
    fn textarea_edited_syncs_content_to_editing_value() {
        let mut state = RenameState::new("");
        state.mode = RenameMode::Edit;

        let _ = rename_apply_action(
            &mut state,
            RenameAction::TextareaEdited(text_editor::Action::Edit(text_editor::Edit::Insert('a'))),
            RenameInputTag::Textarea,
            RenameFallbackSelectionBehavior::End,
            RenameBlurBehavior::Exit,
            non_empty,
        );

        assert_eq!(state.textarea_content.text(), "a");
        assert_eq!(state.editing_value, "a");
    }
}
