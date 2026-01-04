use iced::widget::{column, container};
use iced::{Element, Length};

use iced_shadcn::{TextVariant, Theme, text};

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
        let content = column![
            text("Heading 1", TextVariant::H1, &self.theme),
            text("Heading 2", TextVariant::H2, &self.theme),
            text("Heading 3", TextVariant::H3, &self.theme),
            text("Heading 4", TextVariant::H4, &self.theme),
            text("Large text", TextVariant::Large, &self.theme),
            text("Body text", TextVariant::Body, &self.theme),
            text("Small text", TextVariant::Small, &self.theme),
            text("Muted text", TextVariant::Muted, &self.theme),
        ]
        .spacing(6);

        container(content)
            .padding(24)
            .width(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
    }
}
