use iced::widget::{column, row};
use iced::{Alignment, Element};

use crate::button::{ButtonProps, ButtonVariant, button};
use crate::dialog::{DialogProps, dialog};
use crate::theme::Theme;
use crate::typography::{HeadingProps, TextProps, TextSize, heading, text};

#[derive(Clone, Copy, Debug)]
pub struct AlertDialogProps<'a, Message> {
    pub title: &'a str,
    pub description: &'a str,
    pub confirm_label: &'a str,
    pub cancel_label: &'a str,
    pub on_confirm: Message,
    pub on_cancel: Message,
    pub dialog: DialogProps,
}

impl<'a, Message: Clone> AlertDialogProps<'a, Message> {
    pub fn new(
        title: &'a str,
        description: &'a str,
        on_confirm: Message,
        on_cancel: Message,
    ) -> Self {
        Self {
            title,
            description,
            confirm_label: "Continue",
            cancel_label: "Cancel",
            on_confirm,
            on_cancel,
            dialog: DialogProps::new(),
        }
    }

    pub fn confirm_label(mut self, label: &'a str) -> Self {
        self.confirm_label = label;
        self
    }

    pub fn cancel_label(mut self, label: &'a str) -> Self {
        self.cancel_label = label;
        self
    }

    pub fn dialog_props(mut self, dialog: DialogProps) -> Self {
        self.dialog = dialog;
        self
    }
}

pub fn alert_dialog<'a, Message: Clone + 'a>(
    base: impl Into<Element<'a, Message>>,
    open: bool,
    props: AlertDialogProps<'a, Message>,
    theme: &Theme,
) -> Element<'a, Message> {
    let content = column![
        heading(props.title, HeadingProps::new().size(TextSize::Five), theme),
        text(
            props.description,
            TextProps::new().size(TextSize::Size3),
            theme
        ),
        row![
            button(
                props.cancel_label,
                Some(props.on_cancel.clone()),
                ButtonProps::new().variant(ButtonVariant::Outline),
                theme
            ),
            button(
                props.confirm_label,
                Some(props.on_confirm.clone()),
                ButtonProps::new().variant(ButtonVariant::Solid),
                theme
            ),
        ]
        .spacing(12)
        .align_y(Alignment::Center)
    ]
    .spacing(12);

    dialog(base, open, content, props.on_cancel, props.dialog, theme)
}
