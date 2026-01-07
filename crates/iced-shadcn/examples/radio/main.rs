use iced::border::Border;
use iced::widget::{column, container, row};
use iced::{Alignment, Background, Element, Length};

use iced_shadcn::{RadioProps, RadioSize, Theme, label, radio};

pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view).run()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Plan {
    Starter,
    Pro,
    Team,
}

struct Example {
    theme: Theme,
    selection: Plan,
}

#[derive(Debug, Clone)]
enum Message {
    Selected(Plan),
}

impl Example {
    fn update(&mut self, message: Message) {
        match message {
            Message::Selected(plan) => self.selection = plan,
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let background = theme.palette.background;
        let border = theme.palette.border;
        let radius = theme.radius.md;

        let content = column![
            row![
                radio(
                    "",
                    Plan::Starter,
                    Some(self.selection),
                    Message::Selected,
                    RadioProps::new().size(RadioSize::Two),
                    theme,
                ),
                label("Default", theme),
            ]
            .spacing(12)
            .align_y(Alignment::Center),
            row![
                radio(
                    "",
                    Plan::Pro,
                    Some(self.selection),
                    Message::Selected,
                    RadioProps::new().size(RadioSize::Two),
                    theme,
                ),
                label("Comfortable", theme),
            ]
            .spacing(12)
            .align_y(Alignment::Center),
            row![
                radio(
                    "",
                    Plan::Team,
                    Some(self.selection),
                    Message::Selected,
                    RadioProps::new().size(RadioSize::Two),
                    theme,
                ),
                label("Compact", theme),
            ]
            .spacing(12)
            .align_y(Alignment::Center),
        ]
        .spacing(12)
        .align_x(Alignment::Start);

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
            selection: Plan::Pro,
        }
    }
}
