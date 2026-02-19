use iced::border::Border;
use iced::widget::{column, container, row, text as iced_text};
use iced::{Alignment, Background, Element, Length};

use iced_shadcn::{AccentColor, BadgeProps, BadgeSize, BadgeVariant, Theme, badge};

pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view).run()
}

#[derive(Default)]
struct Example {
    theme: Theme,
}

impl Example {
    fn update(&mut self, _message: ()) {}

    fn view(&self) -> Element<'_, ()> {
        let theme = &self.theme;

        let content = column![
            iced_text("Badge demo").size(20),
            row![
                badge(
                    "New",
                    BadgeProps::new()
                        .variant(BadgeVariant::Secondary)
                        .size(BadgeSize::Size1)
                        .color(AccentColor::Blue),
                    theme
                ),
                badge(
                    "Beta",
                    BadgeProps::new()
                        .variant(BadgeVariant::Secondary)
                        .size(BadgeSize::Size2)
                        .color(AccentColor::Purple),
                    theme
                ),
                badge(
                    "Pro",
                    BadgeProps::new()
                        .variant(BadgeVariant::Secondary)
                        .size(BadgeSize::Size3)
                        .color(AccentColor::Green),
                    theme
                ),
            ]
            .spacing(12)
            .align_y(Alignment::Center)
        ]
        .spacing(16);

        app(theme, preview(theme, content).into())
    }
}

fn app<'a>(theme: &Theme, content: Element<'a, ()>) -> Element<'a, ()> {
    let background = theme.palette.background;
    container(content)
        .padding(24)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(move |_theme| iced::widget::container::Style {
            background: Some(Background::Color(background)),
            ..iced::widget::container::Style::default()
        })
        .into()
}

fn preview<'a>(
    theme: &Theme,
    content: impl Into<Element<'a, ()>>,
) -> iced::widget::Container<'a, ()> {
    let background = theme.palette.card;
    let border = theme.palette.border;
    let radius = theme.radius.md;

    container(content)
        .padding(24)
        .width(Length::Shrink)
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
