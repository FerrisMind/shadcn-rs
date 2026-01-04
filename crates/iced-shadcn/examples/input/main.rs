use iced::widget::{column, container, text};
use iced::{Element, Fill, Length};

use iced_shadcn::{InputSize, Theme, input, label};

pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view).run()
}

#[derive(Default)]
struct Example {
    theme: Theme,
    small: String,
    medium: String,
    large: String,
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(InputSize, String),
}

impl Example {
    fn update(&mut self, message: Message) {
        match message {
            Message::InputChanged(InputSize::Sm, value) => self.small = value,
            Message::InputChanged(InputSize::Md, value) => self.medium = value,
            Message::InputChanged(InputSize::Lg, value) => self.large = value,
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let content = column![
            label("Small input", &self.theme),
            input(
                &self.small,
                "Small input",
                |value| Message::InputChanged(InputSize::Sm, value),
                InputSize::Sm,
                &self.theme,
            ),
            label("Medium input", &self.theme),
            input(
                &self.medium,
                "Medium input",
                |value| Message::InputChanged(InputSize::Md, value),
                InputSize::Md,
                &self.theme,
            ),
            label("Large input", &self.theme),
            input(
                &self.large,
                "Large input",
                |value| Message::InputChanged(InputSize::Lg, value),
                InputSize::Lg,
                &self.theme,
            ),
            text("Typing updates the corresponding field.").size(12),
        ]
        .spacing(8)
        .width(Fill);

        container(content)
            .padding(24)
            .width(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
    }
}
