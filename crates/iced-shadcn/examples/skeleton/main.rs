use iced::border::Border;
use iced::widget::{Column, container, scrollable, text as iced_text};
use iced::{Background, Element, Length};

use iced_shadcn::{SkeletonProps, Theme, skeleton};

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

        let mut content = Column::new().spacing(12);
        content = content.push(iced_text("Skeleton").size(20));
        content = content.push(skeleton(
            SkeletonProps::new()
                .width(Length::Fixed(280.0))
                .height(Length::Fixed(14.0)),
            theme,
        ));
        content = content.push(skeleton(
            SkeletonProps::new()
                .width(Length::Fixed(220.0))
                .height(Length::Fixed(14.0)),
            theme,
        ));
        content = content.push(skeleton(
            SkeletonProps::new()
                .width(Length::Fixed(320.0))
                .height(Length::Fixed(180.0))
                .radius(theme.radius.lg),
            theme,
        ));

        app(theme, scrollable(preview(theme, content)).into())
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
