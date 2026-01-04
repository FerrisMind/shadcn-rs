use iced::border::Border;
use iced::widget::{checkbox, container, row};
use iced::{Alignment, Background, Element, Length};

use iced_shadcn::{Theme, label};

pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view).run()
}

#[derive(Default)]
struct Example {
    theme: Theme,
    accepted: bool,
}

#[derive(Debug, Clone)]
enum Message {
    Toggled(bool),
}

impl Example {
    fn update(&mut self, message: Message) {
        let Message::Toggled(value) = message;
        self.accepted = value;
    }

    fn view(&self) -> Element<'_, Message> {
        let muted = self.theme.palette.muted;
        let border = self.theme.palette.border;
        let radius = self.theme.radius.md;

        let content = row![
            checkbox(self.accepted)
                .label("")
                .on_toggle(Message::Toggled),
            label("Accept terms and conditions", &self.theme),
        ]
        .spacing(8)
        .align_y(Alignment::Center);

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
