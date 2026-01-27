use iced::border::Border;
use iced::widget::{Row, container, text as iced_text};
use iced::{Alignment, Background, Element, Length};

use iced_shadcn::{ScrollAreaProps, ScrollAreaScrollbars, Theme, scroll_area};

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

        let mut row = Row::new().spacing(12).align_y(Alignment::Center);
        for i in 1..=20 {
            let bg = theme.palette.muted;
            row = row.push(
                container(iced_text(format!("Card {i}")).size(12))
                    .padding(12)
                    .width(Length::Fixed(140.0))
                    .height(Length::Fixed(80.0))
                    .style(move |_theme| iced::widget::container::Style {
                        background: Some(Background::Color(bg)),
                        ..iced::widget::container::Style::default()
                    }),
            );
        }

        let content = container(scroll_area(
            row,
            ScrollAreaProps::new().scrollbars(ScrollAreaScrollbars::Horizontal),
            theme,
        ))
        .width(Length::Fixed(420.0))
        .height(Length::Fixed(140.0));

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

fn preview<'a>(theme: &Theme, content: impl Into<Element<'a, ()>>) -> iced::widget::Container<'a, ()> {
    let background = theme.palette.card;
    let border = theme.palette.border;
    let radius = theme.radius.md;

    container(content)
        .padding(16)
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

