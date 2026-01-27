use iced::border::Border;
use iced::widget::{column, container, text as iced_text};
use iced::{Background, Element, Length};

use iced_shadcn::{CardProps, CardVariant, Theme, card};

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

        let card_content = column![
            iced_text("Create project").size(18),
            iced_text("Deploy your new project in one-click.").size(14),
        ]
        .spacing(6);

        let content = preview(
            theme,
            card(card_content, CardProps::new().variant(CardVariant::Surface), theme),
        );

        app(theme, content.into())
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

