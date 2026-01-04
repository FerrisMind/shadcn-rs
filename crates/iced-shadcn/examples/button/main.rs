use iced::widget::{column, container, row, text};
use iced::{Element, Fill, Length};

use iced_shadcn::{ButtonSize, ButtonVariant, Theme, button};

pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view).run()
}

#[derive(Default)]
struct Example {
    theme: Theme,
    last_pressed: Option<&'static str>,
}

#[derive(Debug, Clone)]
enum Message {
    Pressed(&'static str),
}

impl Example {
    fn update(&mut self, message: Message) {
        let Message::Pressed(label) = message;
        self.last_pressed = Some(label);
    }

    fn view(&self) -> Element<'_, Message> {
        let variants = row![
            button(
                "Default",
                Some(Message::Pressed("default")),
                ButtonVariant::Default,
                ButtonSize::Md,
                &self.theme,
            ),
            button(
                "Outline",
                Some(Message::Pressed("outline")),
                ButtonVariant::Outline,
                ButtonSize::Md,
                &self.theme,
            ),
            button(
                "Ghost",
                Some(Message::Pressed("ghost")),
                ButtonVariant::Ghost,
                ButtonSize::Md,
                &self.theme,
            ),
            button(
                "Link",
                Some(Message::Pressed("link")),
                ButtonVariant::Link,
                ButtonSize::Md,
                &self.theme,
            ),
        ]
        .spacing(12);

        let sizes = row![
            button(
                "Small",
                Some(Message::Pressed("small")),
                ButtonVariant::Default,
                ButtonSize::Sm,
                &self.theme,
            ),
            button(
                "Medium",
                Some(Message::Pressed("medium")),
                ButtonVariant::Default,
                ButtonSize::Md,
                &self.theme,
            ),
            button(
                "Large",
                Some(Message::Pressed("large")),
                ButtonVariant::Default,
                ButtonSize::Lg,
                &self.theme,
            ),
        ]
        .spacing(12);

        let status = self
            .last_pressed
            .map(|label| format!("Last pressed: {}", label))
            .unwrap_or_else(|| "Press any button".to_string());

        let content = column![
            text("Button variants").size(16),
            variants,
            text("Button sizes").size(16),
            sizes,
            text(status).size(12),
        ]
        .spacing(16)
        .width(Fill);

        container(content)
            .padding(24)
            .width(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
    }
}
