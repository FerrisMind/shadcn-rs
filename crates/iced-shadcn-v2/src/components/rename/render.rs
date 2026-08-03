//! Rendering and event handling for [`super::Rename`].

use std::rc::Rc;

use iced_core::keyboard;

use crate::components::button::{Button, ButtonVariant};
use crate::components::input::Input;
use crate::components::textarea::Textarea;
use crate::fonts::iced_font;
use crate::iced_compat::advanced::layout::{self, Layout};
use crate::iced_compat::advanced::widget::Tree;
use crate::iced_compat::advanced::{Clipboard, Shell, Widget, renderer};
use crate::iced_compat::widget::{text, text_editor};
use crate::iced_compat::{Element, Event, Length, Rectangle, Size, mouse, touch};
use crate::theme::Theme;
use iced_core::keyboard::key::{self, Key};

use super::types::{
    RenameAction, RenameActionHandler, RenameButtonProps, RenameContext, RenameInputTag,
    RenameRootProps, RenameState,
};

/// Builds the low-level root element used by the public builder and helper.
pub(super) fn root<'a, Message: Clone + 'a>(
    state: &'a RenameState,
    on_action: Option<RenameActionHandler<'a, Message>>,
    props: RenameRootProps,
    theme: &'a Theme,
) -> Element<'a, Message> {
    let mode = state.mode();
    let id = props.id.clone();
    let input_tag = props.input_tag;
    let input_id = props.input_id.clone();
    let width = props.width;
    let input_width = props.input_width.unwrap_or(width);
    let textarea_width = props.textarea_width.unwrap_or(width);
    let input_color = props.input_color.or(props.color);
    let textarea_color = props.textarea_color.or(props.color);
    let input_size = props.input_size;
    let input_radius = props.input_radius;
    let textarea_size = props.textarea_size;
    let textarea_radius = props.textarea_radius;
    let text_size = props.text_size;
    let invalid = state.is_invalid();
    let disabled = props.disabled;

    let content = match (mode, input_tag) {
        (super::RenameMode::View, _) => view_text(state, &props, theme),
        (super::RenameMode::Edit, RenameInputTag::Input) => {
            let mut input = Input::new(theme)
                .value(state.editing_value())
                .size(input_size)
                .width(input_width)
                .id(input_id)
                .invalid(invalid)
                .disabled(disabled);

            if let Some(radius) = input_radius {
                input = input.radius(radius);
            }
            if let Some(color) = input_color {
                input = input.color(color);
            }
            if let Some(text_size) = text_size {
                input = input.text_size(text_size);
            }

            if let Some(on_action) = on_action.as_ref() {
                let input_on_action = Rc::clone(on_action);
                input =
                    input.on_input(move |value| input_on_action(RenameAction::InputChanged(value)));

                let submit_message = on_action(RenameAction::SaveRequested);
                input = input.on_submit(submit_message);
            }

            input.into()
        }
        (super::RenameMode::Edit, RenameInputTag::Textarea) => {
            let mut textarea = Textarea::new(state.textarea_content(), theme)
                .size(textarea_size)
                .width(textarea_width)
                .id(input_id)
                .invalid(invalid)
                .disabled(disabled);

            if let Some(radius) = textarea_radius {
                textarea = textarea.radius(radius);
            }
            if let Some(color) = textarea_color {
                textarea = textarea.color(color);
            }
            if let Some(text_size) = text_size {
                textarea = textarea.text_size(text_size);
            }

            if let Some(on_action) = on_action.as_ref() {
                let on_action = Rc::clone(on_action);
                textarea = textarea.on_action(move |action| match action {
                    text_editor::Action::Edit(text_editor::Edit::Enter) => {
                        on_action(RenameAction::SaveRequested)
                    }
                    action => on_action(RenameAction::TextareaEdited(action)),
                });
            }

            textarea.into()
        }
    };

    RenameRootWidget {
        content,
        id,
        mode,
        on_action,
        click_to_edit: props.click_to_edit,
        disabled,
    }
    .into()
}

fn view_text<'a, Message>(
    state: &'a RenameState,
    props: &RenameRootProps,
    theme: &'a Theme,
) -> Element<'a, Message>
where
    Message: 'a,
{
    let text_size = props.text_size.unwrap_or(16.0);
    let line_height = props.text_line_height.unwrap_or(text_size * 1.4);
    text(state.value())
        .size(text_size)
        .line_height(line_height)
        .font(iced_font(theme.font_pack().sans))
        .color(props.text_color.unwrap_or(theme.palette.foreground))
        .width(props.width)
        .into()
}

/// Renders one of the external controls from the provider context.
pub(super) fn control<'a, Message: Clone + 'a>(
    context: &RenameContext<'a, Message>,
    theme: &'a Theme,
    props: RenameButtonProps<'a, Message>,
    default_label: &'static str,
    default_variant: ButtonVariant,
    action: RenameAction,
    disabled_by_mode: bool,
) -> Element<'a, Message> {
    let label = if props.label.is_empty() {
        default_label.to_owned()
    } else {
        props.label
    };
    let disabled = context.is_disabled() || props.disabled || disabled_by_mode;
    let mut button = match props.content {
        Some(content) => Button::new(content, theme),
        None => Button::text(label, theme),
    }
    .variant(props.variant.unwrap_or(default_variant))
    .size(props.size)
    .width(props.width)
    .disabled(disabled);

    if let Some(radius) = props.radius {
        button = button.radius(radius);
    }
    if let Some(color) = props.color {
        button = button.color(color);
    }
    if let Some(height) = props.height {
        button = button.height(height);
    }
    if !disabled && let Some(message) = context.action_message(action) {
        button = button.on_press(message);
    }

    button.into()
}

/// Root wrapper that adds the Svelte component's click-to-edit, Escape, and
/// outside-click behavior around the real v2 input primitive.
struct RenameRootWidget<'a, Message> {
    content: Element<'a, Message>,
    id: crate::iced_compat::widget::Id,
    mode: super::RenameMode,
    on_action: Option<RenameActionHandler<'a, Message>>,
    click_to_edit: bool,
    disabled: bool,
}

impl<Message> Widget<Message, crate::iced_compat::Theme, crate::iced_compat::Renderer>
    for RenameRootWidget<'_, Message>
where
    Message: Clone,
{
    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.content));
    }

    fn size(&self) -> Size<Length> {
        self.content.as_widget().size()
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &crate::iced_compat::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        self.content
            .as_widget_mut()
            .layout(&mut tree.children[0], renderer, limits)
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &crate::iced_compat::Renderer,
        operation: &mut dyn crate::iced_compat::advanced::widget::Operation,
    ) {
        operation.container(Some(&self.id), layout.bounds());
        operation.traverse(&mut |operation| {
            self.content.as_widget_mut().operate(
                &mut tree.children[0],
                layout,
                renderer,
                operation,
            );
        });
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &crate::iced_compat::Renderer,
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

        if self.disabled || shell.is_event_captured() {
            return;
        }

        let Some(on_action) = self.on_action.as_ref() else {
            return;
        };

        match (self.mode, event) {
            (
                super::RenameMode::View,
                Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)),
            )
            | (super::RenameMode::View, Event::Touch(touch::Event::FingerPressed { .. }))
                if self.click_to_edit && cursor.is_over(layout.bounds()) =>
            {
                shell.publish(on_action(RenameAction::StartEdit));
                shell.capture_event();
            }
            (
                super::RenameMode::Edit,
                Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)),
            )
            | (super::RenameMode::Edit, Event::Touch(touch::Event::FingerPressed { .. }))
                if !cursor.is_over(layout.bounds()) =>
            {
                shell.publish(on_action(RenameAction::BlurDetected));
            }
            (
                super::RenameMode::Edit,
                Event::Keyboard(keyboard::Event::KeyPressed {
                    key: Key::Named(key::Named::Escape),
                    ..
                }),
            ) => {
                shell.publish(on_action(RenameAction::EscapePressed));
                shell.capture_event();
            }
            (
                super::RenameMode::Edit,
                Event::Keyboard(keyboard::Event::KeyPressed {
                    key: Key::Named(key::Named::Tab),
                    ..
                }),
            ) => {
                shell.publish(on_action(RenameAction::BlurDetected));
            }
            _ => {}
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut crate::iced_compat::Renderer,
        theme: &crate::iced_compat::Theme,
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
        renderer: &crate::iced_compat::Renderer,
    ) -> mouse::Interaction {
        if !self.disabled
            && self.click_to_edit
            && self.mode == super::RenameMode::View
            && self.on_action.is_some()
            && cursor.is_over(layout.bounds())
        {
            mouse::Interaction::Pointer
        } else {
            self.content.as_widget().mouse_interaction(
                &tree.children[0],
                layout,
                cursor,
                viewport,
                renderer,
            )
        }
    }
}

impl<'a, Message: Clone + 'a> From<RenameRootWidget<'a, Message>> for Element<'a, Message> {
    fn from(widget: RenameRootWidget<'a, Message>) -> Self {
        Element::new(widget)
    }
}
