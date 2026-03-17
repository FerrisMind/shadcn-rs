use super::super::app::preview_card;
use super::super::app::{Message, PreviewApp};
use iced::widget::{column, text};
use iced::{Element, Length};
use iced_shadcn::{InputProps, InputSize, InputVariant, input};

pub fn render<'a>(app: &'a PreviewApp) -> Element<'a, Message> {
    let theme = app.theme();
    column![
        preview_card(
            theme,
            "Email",
            column![
                text("Email").size(13),
                input(
                    app.email(),
                    "you@example.com",
                    Some(Message::EmailChanged),
                    InputProps::new()
                        .size(InputSize::Size2)
                        .variant(InputVariant::Surface),
                    theme,
                )
                .width(Length::Fixed(320.0)),
            ]
            .spacing(8),
        ),
        preview_card(
            theme,
            "Username",
            column![
                text("Username").size(13),
                input(
                    app.username(),
                    "shadcn",
                    Some(Message::UsernameChanged),
                    InputProps::new()
                        .size(InputSize::Size2)
                        .variant(InputVariant::Surface),
                    theme,
                )
                .width(Length::Fixed(320.0)),
                text("This is your public display name.")
                    .size(12)
                    .style(|_theme| iced::widget::text::Style {
                        color: Some(theme.palette.muted_foreground),
                    }),
            ]
            .spacing(8),
        ),
    ]
    .spacing(16)
    .into()
}
