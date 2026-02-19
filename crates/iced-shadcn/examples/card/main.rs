use iced::border::Border;
use iced::widget::{Column, container, row, scrollable, text as iced_text};
use iced::{Alignment, Background, Element, Length};

use iced_shadcn::{CardProps, CardSize, CardVariant, Theme, card};

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
        let variants = [
            CardVariant::Surface,
            CardVariant::Classic,
            CardVariant::Ghost,
        ];
        let sizes = [
            CardSize::Size1,
            CardSize::Size2,
            CardSize::Size3,
            CardSize::Size4,
            CardSize::Five,
        ];

        let mut content = Column::new().spacing(16);
        for variant in variants {
            let mut row = row![iced_text(format!("{variant:?}")).width(Length::Fixed(120.0))]
                .spacing(12)
                .align_y(Alignment::Center);
            for size in sizes {
                row = row.push(
                    card(
                        iced_text(format!("{size:?}")).size(12),
                        CardProps::new().variant(variant).size(size),
                        theme,
                    )
                    .width(Length::Fixed(130.0)),
                );
            }
            content = content.push(preview(theme, row));
        }

        app(theme, scrollable(content).into())
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
