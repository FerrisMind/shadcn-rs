use iced::border::Border;
use iced::widget::{Column, column, container, row, scrollable, text as iced_text};
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

        let sizes = [BadgeSize::One, BadgeSize::Two, BadgeSize::Three];
        let variants = [
            BadgeVariant::Solid,
            BadgeVariant::Soft,
            BadgeVariant::Surface,
            BadgeVariant::Outline,
        ];

        let mut grid = Column::new().spacing(16);
        for variant in variants {
            let mut row = row![iced_text(format!("{variant:?}")).width(Length::Fixed(120.0))]
                .spacing(12)
                .align_y(Alignment::Center);
            for size in sizes {
                row = row.push(badge(
                    "Badge",
                    BadgeProps::new()
                        .variant(variant)
                        .size(size)
                        .color(AccentColor::Blue),
                    theme,
                ));
            }
            grid = grid.push(preview(theme, row));
        }

        app(theme, scrollable(column![grid].spacing(16)).into())
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

