use iced::border::Border;
use iced::widget::{container};
use iced::{Background, Element, Length};

use iced_shadcn::{SliderProps, Theme, slider};

pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view).run()
}

struct Example {
    theme: Theme,
    value: f32,
}

#[derive(Debug, Clone)]
enum Message {
    Changed(f32),
}

impl Example {
    fn update(&mut self, message: Message) {
        match message {
            Message::Changed(value) => self.value = value,
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let background = theme.palette.background;
        let border = theme.palette.border;
        let radius = theme.radius.md;

        let content = slider(
            0.0..=100.0,
            self.value,
            Message::Changed,
            SliderProps::new(),
            theme,
        )
        .width(Length::Fixed(360.0));

        container(content)
            .width(Length::Fill)
            .padding(32)
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

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::default(),
            value: 50.0,
        }
    }
}
