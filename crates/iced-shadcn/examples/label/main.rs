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
        let theme = &self.theme;
        let background = theme.palette.background;
        let border = theme.palette.border;
        let radius = theme.radius.md;

        let content = row![
            checkbox(self.accepted)
                .label("")
                .on_toggle(Message::Toggled),
            label("Accept terms and conditions", theme),
        ]
        .spacing(8)
        .align_y(Alignment::Center);

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
