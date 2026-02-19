use iced::border::Border;
use iced::widget::{column, container, row, scrollable, space, text as iced_text};
use iced::{Alignment, Background, Element, Length};

use iced_shadcn::{
    AccentColor, SeparatorOrientation, SeparatorProps, SeparatorSize, TextProps, TextSize,
    TextWeight, Theme, separator, text,
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

        let mut content = column![].spacing(16).width(Length::Fill);

        // -- Basic horizontal + vertical --
        content = content.push(section_title("Basic Usage"));
        let heading = column![
            text(
                "Radix Primitives",
                TextProps::new()
                    .size(TextSize::Size2)
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
            text("Blog", TextProps::new().size(TextSize::Size2), theme),
            container(separator(
                SeparatorProps::new()
                    .orientation(SeparatorOrientation::Vertical)
                    .size(SeparatorSize::Size2),
                theme,
            ))
            .height(Length::Fixed(20.0)),
            text("Docs", TextProps::new().size(TextSize::Size2), theme),
            container(separator(
                SeparatorProps::new()
                    .orientation(SeparatorOrientation::Vertical)
                    .size(SeparatorSize::Size2),
                theme,
            ))
            .height(Length::Fixed(20.0)),
            text("Source", TextProps::new().size(TextSize::Size2), theme),
        ]
        .spacing(16)
        .align_y(Alignment::Center);

        content = content.push(preview(
            theme,
            column![
                heading,
                space().height(Length::Fixed(8.0)),
                separator(SeparatorProps::new(), theme),
                space().height(Length::Fixed(8.0)),
                nav,
            ]
            .spacing(0),
        ));

        // -- Sizes --
        content = content.push(section_title("Sizes"));
        content = content.push(preview(
            theme,
            column![
                separator(SeparatorProps::new().size(SeparatorSize::Size1), theme),
                separator(SeparatorProps::new().size(SeparatorSize::Size2), theme),
                separator(SeparatorProps::new().size(SeparatorSize::Size3), theme),
                separator(SeparatorProps::new().size(SeparatorSize::Size4), theme),
            ]
            .spacing(8),
        ));

        // -- Thickness --
        content = content.push(section_title("Thickness"));
        content = content.push(preview(
            theme,
            column![
                separator(SeparatorProps::new().thickness(1.0), theme),
                separator(SeparatorProps::new().thickness(2.0), theme),
                separator(SeparatorProps::new().thickness(4.0), theme),
            ]
            .spacing(8),
        ));

        // -- Custom Color --
        content = content.push(section_title("Custom Color"));
        content = content.push(preview(
            theme,
            separator(SeparatorProps::new().color(AccentColor::Red), theme),
        ));

        // -- High Contrast --
        content = content.push(section_title("High Contrast"));
        content = content.push(preview(
            theme,
            column![
                separator(SeparatorProps::new(), theme),
                separator(SeparatorProps::new().high_contrast(true), theme),
            ]
            .spacing(8),
        ));

        // -- With Gap --
        content = content.push(section_title("With Gap (8px)"));
        content = content.push(preview(
            theme,
            column![
                iced_text("Content above"),
                separator(SeparatorProps::new().gap(8.0), theme),
                iced_text("Content below"),
            ]
            .spacing(0),
        ));

        app(theme, scrollable(content).into())
    }
}

fn section_title(title: &str) -> Element<'_, Message> {
    iced_text(title).size(16).into()
}

fn app<'a>(theme: &Theme, content: Element<'a, Message>) -> Element<'a, Message> {
    let background = theme.palette.background;
    container(content)
        .padding(24)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(move |_theme| iced::widget::container::Style {
            background: Some(Background::Color(background)),
            ..iced::widget::container::Style::default()
        })
        .into()
}

fn preview<'a>(
    theme: &Theme,
    content: impl Into<Element<'a, Message>>,
) -> iced::widget::Container<'a, Message> {
    let background = theme.palette.card;
    let border = theme.palette.border;
    let radius = theme.radius.md;

    container(content)
        .padding(16)
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
