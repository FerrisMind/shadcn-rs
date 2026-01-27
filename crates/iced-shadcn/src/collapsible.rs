use iced::widget::{column, container};
use iced::{Alignment, Element, Length};

use crate::button::{ButtonProps, ButtonSize, ButtonVariant, button_content};
use crate::theme::Theme;

#[derive(Clone, Copy, Debug, Default)]
pub struct CollapsibleProps {
    pub disabled: bool,
    pub compact: bool,
}

impl CollapsibleProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn compact(mut self, compact: bool) -> Self {
        self.compact = compact;
        self
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct CollapsibleContentProps {
    pub force_mount: bool,
}

impl CollapsibleContentProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn force_mount(mut self, force_mount: bool) -> Self {
        self.force_mount = force_mount;
        self
    }
}

pub fn collapsible<'a, Message: Clone + 'a, F>(
    open: bool,
    trigger: impl Into<Element<'a, Message>>,
    content: impl Into<Element<'a, Message>>,
    on_open_change: Option<F>,
    content_props: CollapsibleContentProps,
    props: CollapsibleProps,
    theme: &Theme,
) -> Element<'a, Message>
where
    F: Fn(bool) -> Message + 'a,
{
    let padding_y = if props.compact { 8.0 } else { 12.0 };
    let trigger_size = if props.compact {
        ButtonSize::One
    } else {
        ButtonSize::Two
    };

    let trigger = button_content(
        trigger,
        on_open_change.map(|f| (f)(!open)),
        ButtonProps::new()
            .variant(ButtonVariant::Ghost)
            .size(trigger_size)
            .disabled(props.disabled),
        theme,
    );

    let mut body = column![container(trigger)
        .width(Length::Fill)
        .padding([padding_y, 0.0])]
    .spacing(0)
    .align_x(Alignment::Start);

    if open || content_props.force_mount {
        body = body.push(container(content).width(Length::Fill));
    }

    container(body.width(Length::Fill)).width(Length::Fill).into()
}
