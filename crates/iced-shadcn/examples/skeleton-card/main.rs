use iced::border::Border;
use iced::widget::{column, container, row, text as iced_text};
use iced::{Alignment, Background, Element, Length};

use iced_shadcn::{CardProps, SkeletonProps, Theme, card, skeleton};

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

        let content = card(
            column![
                skeleton(
                    SkeletonProps::new()
                        .width(Length::Fixed(320.0))
                        .height(Length::Fixed(160.0))
                        .radius(theme.radius.lg),
                    theme
                ),
                row![
                    skeleton(
                        SkeletonProps::new()
                            .width(Length::Fixed(40.0))
                            .height(Length::Fixed(40.0))
                            .radius(9999.0),
                        theme
                    ),
                    column![
                        skeleton(
                            SkeletonProps::new()
                                .width(Length::Fixed(160.0))
                                .height(Length::Fixed(12.0)),
                            theme
                        ),
                        skeleton(
                            SkeletonProps::new()
                                .width(Length::Fixed(120.0))
                                .height(Length::Fixed(12.0)),
                            theme
                        ),
                    ]
                    .spacing(8),
                ]
                .spacing(12)
                .align_y(Alignment::Center),
                iced_text("Loading...").size(12),
            ]
            .spacing(12),
            CardProps::new(),
            theme,
        );

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

