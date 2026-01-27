use iced::border::Border;
use iced::widget::{Column, column, container, row, text as iced_text};
use iced::{Alignment, Background, Element, Length};

use iced_shadcn::{ScrollAreaProps, ScrollAreaScrollbars, ScrollAreaSize, Theme, scroll_area};

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

        let long_list = |count: usize| {
            let mut col = Column::new().spacing(8);
            for i in 1..=count {
                col = col.push(iced_text(format!("Item {i}")).size(12));
            }
            col
        };

        let sizes = [ScrollAreaSize::One, ScrollAreaSize::Two, ScrollAreaSize::Three];
        let mut content = Column::new().spacing(16);
        for size in sizes {
            content = content.push(preview(
                theme,
                row![
                    iced_text(format!("size {size:?}")).width(Length::Fixed(120.0)),
                    container(scroll_area(
                        long_list(50),
                        ScrollAreaProps::new()
                            .size(size)
                            .scrollbars(ScrollAreaScrollbars::Vertical),
                        theme
                    ))
                    .width(Length::Fixed(320.0))
                    .height(Length::Fixed(180.0)),
                ]
                .spacing(12)
                .align_y(Alignment::Start),
            ));
        }

        app(theme, container(column![content].spacing(16)).into())
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

