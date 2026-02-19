use std::cell::Cell;

use iced::widget::{row, text};
use iced::{Alignment, Element};

use crate::button::{ButtonProps, ButtonRadius, ButtonVariant, button_content};
use crate::theme::Theme;

use crate::tokens::ControlSize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ToggleVariant {
    #[default]
    Default,
    Outline,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ToggleGroupProps {
    pub variant: ToggleVariant,
    pub size: ControlSize,
}

impl Default for ToggleGroupProps {
    fn default() -> Self {
        Self {
            variant: ToggleVariant::Default,
            size: ControlSize::Md,
        }
    }
}

pub struct ToggleGroupContext {
    pub variant: ToggleVariant,
    pub size: ControlSize,
    item_count: Cell<usize>,
}

pub fn toggle_group<'a, Message: Clone + 'a>(
    props: ToggleGroupProps,
    content: impl FnOnce(&ToggleGroupContext) -> Vec<Element<'a, Message>>,
) -> Element<'a, Message> {
    let context = ToggleGroupContext {
        variant: props.variant,
        size: props.size,
        item_count: Cell::new(0),
    };

    let items = content(&context);
    row(items).spacing(0).align_y(Alignment::Center).into()
}

pub fn toggle_group_item<'a, Message: Clone + 'a, F>(
    theme: &Theme,
    context: &ToggleGroupContext,
    on: bool,
    label: impl Into<String>,
    on_toggle: Option<F>,
) -> Element<'a, Message>
where
    F: Fn(bool) -> Message + 'a,
{
    toggle_group_item_with_position(
        theme,
        context,
        on,
        label,
        on_toggle,
        ToggleGroupItemPosition::Middle,
    )
}

pub fn toggle_group_item_last<'a, Message: Clone + 'a, F>(
    theme: &Theme,
    context: &ToggleGroupContext,
    on: bool,
    label: impl Into<String>,
    on_toggle: Option<F>,
) -> Element<'a, Message>
where
    F: Fn(bool) -> Message + 'a,
{
    toggle_group_item_with_position(
        theme,
        context,
        on,
        label,
        on_toggle,
        ToggleGroupItemPosition::Last,
    )
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ToggleGroupItemPosition {
    Middle,
    Last,
}

fn toggle_group_item_with_position<'a, Message: Clone + 'a, F>(
    theme: &Theme,
    context: &ToggleGroupContext,
    on: bool,
    label: impl Into<String>,
    on_toggle: Option<F>,
    position: ToggleGroupItemPosition,
) -> Element<'a, Message>
where
    F: Fn(bool) -> Message + 'a,
{
    let index = context.item_count.get();
    context.item_count.set(index + 1);

    let radius = if index == 0 || matches!(position, ToggleGroupItemPosition::Last) {
        context.size.radius()
    } else {
        ButtonRadius::None
    };

    let variant = match (context.variant, on) {
        (ToggleVariant::Default, true) => ButtonVariant::Soft,
        (ToggleVariant::Default, false) => ButtonVariant::Ghost,
        (ToggleVariant::Outline, true) => ButtonVariant::Soft,
        (ToggleVariant::Outline, false) => ButtonVariant::Outline,
    };

    let on_press = on_toggle.map(|f| f(!on));

    button_content(
        text(label.into()),
        on_press,
        ButtonProps::new()
            .variant(variant)
            .size(context.size.button_size())
            .radius(radius),
        theme,
    )
    .into()
}
