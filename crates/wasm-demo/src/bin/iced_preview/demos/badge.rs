use super::super::app::preview_card;
use super::super::app::{Message, PreviewApp};
use iced::widget::{column, row};
use iced::{Alignment, Element};
use iced_shadcn::{AccentColor, BadgeProps, BadgeSize, BadgeVariant, badge};

pub fn render<'a>(app: &'a PreviewApp) -> Element<'a, Message> {
    let theme = app.theme();
    row![
        preview_card(
            theme,
            "Variants",
            column![
                row![
                    badge(
                        "Default",
                        BadgeProps::new().variant(BadgeVariant::Default),
                        theme
                    ),
                    badge(
                        "Secondary",
                        BadgeProps::new().variant(BadgeVariant::Secondary),
                        theme,
                    ),
                ]
                .spacing(8),
                row![
                    badge(
                        "Outline",
                        BadgeProps::new().variant(BadgeVariant::Outline),
                        theme
                    ),
                    badge(
                        "Destructive",
                        BadgeProps::new().variant(BadgeVariant::Destructive),
                        theme,
                    ),
                ]
                .spacing(8),
            ]
            .spacing(8),
        ),
        preview_card(
            theme,
            "Sizes & Colors",
            column![
                row![
                    badge("Size 1", BadgeProps::new().size(BadgeSize::Size1), theme),
                    badge("Size 3", BadgeProps::new().size(BadgeSize::Size3), theme),
                ]
                .spacing(8),
                row![
                    badge(
                        "Success",
                        BadgeProps::new().color(AccentColor::Green),
                        theme
                    ),
                    badge("Error", BadgeProps::new().color(AccentColor::Red), theme),
                ]
                .spacing(8),
            ]
            .spacing(8),
        )
    ]
    .spacing(16)
    .align_y(Alignment::Start)
    .into()
}
