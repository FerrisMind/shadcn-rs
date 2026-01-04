use iced::time::{self, Duration};
use iced::widget::{column, container, text};
use iced::{Element, Length, Subscription};

use iced_shadcn::{Spinner, Theme, spinner};

pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view)
        .subscription(Example::subscription)
        .run()
}

#[derive(Default)]
struct Example {
    theme: Theme,
    progress: f32,
}

#[derive(Debug, Clone)]
enum Message {
    Tick,
}

impl Example {
    fn update(&mut self, message: Message) {
        match message {
            Message::Tick => {
                self.progress = (self.progress + 0.02) % 1.0;
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        time::every(Duration::from_millis(16)).map(|_| Message::Tick)
    }

    fn view(&self) -> Element<'_, Message> {
        let spinner_widget = spinner(Spinner::new(&self.theme).progress(self.progress));

        let content = column![
            text("Spinner").size(16),
            spinner_widget,
            text("Animated canvas spinner").size(12),
        ]
        .spacing(12);

        container(content)
            .padding(24)
            .width(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
    }
}
