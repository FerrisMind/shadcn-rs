use iced::widget::{column, container, row, text};
use iced::{Element, Length};

use iced_shadcn::{SeparatorOrientation, Theme, separator};

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
        let horizontal = column![
            text("Above the separator").size(14),
            separator(SeparatorOrientation::Horizontal, &self.theme),
            text("Below the separator").size(14),
        ]
        .spacing(12);

        let vertical = row![
            text("Left").size(14),
            container(separator(SeparatorOrientation::Vertical, &self.theme))
                .height(Length::Fixed(48.0)),
            text("Right").size(14),
        ]
        .spacing(12);

        let content = column![horizontal, vertical].spacing(24);

        container(content)
            .padding(24)
            .width(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
    }
}
