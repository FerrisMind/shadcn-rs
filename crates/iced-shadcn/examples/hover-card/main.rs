use iced::border::Border;
use iced::widget::{container, text as iced_text};
use iced::{Background, Element, Length};

use iced_shadcn::{ButtonProps, ButtonVariant, HoverCardProps, Theme, button, hover_card};

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

        let content = hover_card(
            button(
                "A fancy link",
                None,
                ButtonProps::new().variant(ButtonVariant::Link),
                theme,
            ),
            iced_text("Jan Tschichold was a German calligrapher, typographer and book designer.")
                .size(14),
            HoverCardProps::new().max_width(200),
            theme,
        );

        app(theme, preview(theme, content).into())
    }
}

fn app<'a, Message: 'a>(theme: &Theme, content: Element<'a, Message>) -> Element<'a, Message> {
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

fn preview<'a, Message: 'a>(
    theme: &Theme,
    content: impl Into<Element<'a, Message>>,
) -> iced::widget::Container<'a, Message> {
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
