use super::super::app::preview_card;
use super::super::app::{Message, PreviewApp};
use iced::widget::{column, row};
use iced::{Alignment, Element};
use iced_shadcn::{ButtonProps, ButtonSize, ButtonVariant, button};

pub fn render<'a>(app: &'a PreviewApp) -> Element<'a, Message> {
    let theme = app.theme();
    row![
        preview_card(
            theme,
            "Variants",
            column![
                button(
                    "Primary",
                    Some(Message::Noop),
                    ButtonProps::new()
                        .variant(ButtonVariant::Solid)
                        .size(ButtonSize::Size2),
                    theme,
                ),
                button(
                    "Outline",
                    Some(Message::Noop),
                    ButtonProps::new()
                        .variant(ButtonVariant::Outline)
                        .size(ButtonSize::Size2),
                    theme,
                ),
                button(
                    "Ghost",
                    Some(Message::Noop),
                    ButtonProps::new()
                        .variant(ButtonVariant::Ghost)
                        .size(ButtonSize::Size2),
                    theme,
                ),
            ]
            .spacing(8),
        ),
        preview_card(
            theme,
            "States",
            column![
                button(
                    "Icon",
                    Some(Message::Noop),
                    ButtonProps::new()
                        .variant(ButtonVariant::Outline)
                        .size(ButtonSize::Size1),
                    theme,
                ),
                button(
                    "Loading",
                    Some(Message::Noop),
                    ButtonProps::new()
                        .variant(ButtonVariant::Outline)
                        .size(ButtonSize::Size1)
                        .loading(true),
                    theme,
                ),
            ]
            .spacing(8),
        )
    ]
    .spacing(16)
    .align_y(Alignment::Start)
    .into()
}
