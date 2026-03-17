use super::super::app::preview_card;
use super::super::app::{Message, PreviewApp};
use iced::widget::{column, row, text};
use iced::{Alignment, Element};
use iced_shadcn::{
    ButtonProps, ButtonSize, ButtonVariant, Spinner, button, button_content, spinner,
};

pub fn render<'a>(app: &'a PreviewApp) -> Element<'a, Message> {
    let theme = app.theme();
    let spinner_phase = app.spinner_phase();
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
                        .size(ButtonSize::Size1),
                    theme,
                ),
                button_content(
                    row![
                        text("Loading"),
                        spinner(Spinner::new(theme).progress(spinner_phase))
                    ]
                    .spacing(8)
                    .align_y(Alignment::Center),
                    Some(Message::Noop),
                    ButtonProps::new()
                        .variant(ButtonVariant::Outline)
                        .size(ButtonSize::Size1),
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
