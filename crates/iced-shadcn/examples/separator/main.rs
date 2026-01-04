use iced::border::Border;
use iced::widget::{column, container, row, space};
use iced::{Alignment, Background, Element, Length};

use iced_shadcn::{SeparatorOrientation, TextVariant, Theme, separator, text};

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
        let muted = self.theme.palette.muted;
        let border = self.theme.palette.border;
        let radius = self.theme.radius.md;

        let heading = column![
            text("Radix Primitives", TextVariant::Small, &self.theme),
            text(
                "An open-source UI component library.",
                TextVariant::Muted,
                &self.theme,
            ),
        ]
        .spacing(4);

        let nav = row![
            text("Blog", TextVariant::Small, &self.theme),
            container(separator(SeparatorOrientation::Vertical, &self.theme))
                .height(Length::Fixed(20.0)),
            text("Docs", TextVariant::Small, &self.theme),
            container(separator(SeparatorOrientation::Vertical, &self.theme))
                .height(Length::Fixed(20.0)),
            text("Source", TextVariant::Small, &self.theme),
        ]
        .spacing(16)
        .align_y(Alignment::Center);

        let content = column![
            heading,
            space().height(Length::Fixed(16.0)),
            separator(SeparatorOrientation::Horizontal, &self.theme),
            space().height(Length::Fixed(16.0)),
            nav,
        ]
        .spacing(0);

        container(content)
            .padding(24)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(move |_theme| iced::widget::container::Style {
                background: Some(Background::Color(muted)),
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
