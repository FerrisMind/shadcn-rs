use iced::widget::{column, container, text};
use iced::{Element, Length};

use iced_shadcn::{Theme, label};

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
            text("Label examples").size(16),
            label("Email address", &self.theme),
            label("Password", &self.theme),
            label("Remember me", &self.theme),
        ]
        .spacing(8);

        container(content)
            .padding(24)
            .width(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
    }
}
