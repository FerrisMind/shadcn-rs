use iced::border::Border;
use iced::widget::{column, container, row, text as iced_text};
use iced::{Alignment, Background, Element, Length};

use iced_shadcn::{AspectRatioProps, CardProps, Theme, aspect_ratio, card};

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

        let placeholder = |label: &'static str| {
            let bg = theme.palette.muted;
            container(iced_text(label).size(12))
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .width(Length::Fill)
                .height(Length::Fill)
                .style(move |_theme| iced::widget::container::Style {
                    background: Some(Background::Color(bg)),
                    ..iced::widget::container::Style::default()
                })
        };

        let ratios = row![
            card(
                column![
                    iced_text("16:9").size(12),
                    container(aspect_ratio(
                        placeholder("Video"),
                        AspectRatioProps::new().ratio(16.0 / 9.0)
                    ))
                    .width(Length::Fixed(240.0))
                    .height(Length::Fixed(180.0)),
                ]
                .spacing(8),
                CardProps::new(),
                theme
            )
            .width(Length::Fixed(280.0)),
            card(
                column![
                    iced_text("1:1").size(12),
                    container(aspect_ratio(
                        placeholder("Square"),
                        AspectRatioProps::new().ratio(1.0)
                    ))
                    .width(Length::Fixed(180.0))
                    .height(Length::Fixed(180.0)),
                ]
                .spacing(8),
                CardProps::new(),
                theme
            )
            .width(Length::Fixed(240.0)),
        ]
        .spacing(16)
        .align_y(Alignment::Start);

        app(theme, preview(theme, ratios).into())
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
