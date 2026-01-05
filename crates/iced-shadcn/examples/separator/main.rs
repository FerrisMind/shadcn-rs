use iced::border::Border;
use iced::widget::{column, container, row, space, text as iced_text};
use iced::{Alignment, Background, Element, Length};

use iced_shadcn::{
    SeparatorOrientation, SeparatorProps, SeparatorSize, TextProps, TextSize, TextWeight, Theme,
    separator, text,
};

pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view).run()
}

#[derive(Default)]
struct Example {
    theme: Theme,
}

#[derive(Debug, Clone)]
enum Message {}

impl Example {
    fn update(&mut self, _message: Message) {}

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let background = theme.palette.background;
        let border = theme.palette.border;
        let radius = theme.radius.md;

        let heading = column![
            text(
                "Radix Primitives",
                TextProps::new()
                    .size(TextSize::Two)
                    .weight(TextWeight::Medium),
                theme,
            ),
            iced_text("An open-source UI component library.")
                .size(14)
                .style(|_theme| iced::widget::text::Style {
                    color: Some(theme.palette.muted_foreground),
                }),
        ]
        .spacing(4);

        let nav = row![
            text("Blog", TextProps::new().size(TextSize::Two), theme),
            container(separator(
                SeparatorProps::new()
                    .orientation(SeparatorOrientation::Vertical)
                    .size(SeparatorSize::Two),
                theme,
            ),)
            .height(Length::Fixed(20.0)),
            text("Docs", TextProps::new().size(TextSize::Two), theme),
            container(separator(
                SeparatorProps::new()
                    .orientation(SeparatorOrientation::Vertical)
                    .size(SeparatorSize::Two),
                theme,
            ),)
            .height(Length::Fixed(20.0)),
            text("Source", TextProps::new().size(TextSize::Two), theme),
        ]
        .spacing(16)
        .align_y(Alignment::Center);

        let content = column![
            heading,
            space().height(Length::Fixed(16.0)),
            separator(
                SeparatorProps::new()
                    .orientation(SeparatorOrientation::Horizontal)
                    .size(SeparatorSize::Four),
                theme,
            ),
            space().height(Length::Fixed(16.0)),
            nav,
        ]
        .spacing(0);

        let preview = preview(theme, content);

        container(preview)
            .padding(24)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .style(move |_theme| iced::widget::container::Style {
                background: Some(Background::Color(background)),
                border: Border {
                    radius: radius.into(),
                    width: 1.0,
                    color: border,
                },
                ..iced::widget::container::Style::default()
            })
            .into()
    }
}

fn preview<'a, Message: 'a>(
    theme: &Theme,
    content: impl Into<Element<'a, Message>>,
) -> iced::widget::Container<'a, Message> {
    let background = theme.palette.card;
    let border = theme.palette.border;
    let radius = theme.radius.md;

    container(content)
        .padding(20)
        .width(Length::Fill)
        .style(move |_theme| iced::widget::container::Style {
            background: Some(Background::Color(background)),
            border: Border {
                radius: radius.into(),
                width: 1.0,
                color: border,
            },
            ..iced::widget::container::Style::default()
        })
}
