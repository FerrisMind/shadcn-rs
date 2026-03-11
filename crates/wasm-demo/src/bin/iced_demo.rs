use iced::border::Border;
use iced::widget::{column, container, text};
use iced::{Background, Element, Length};
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
        let content = card(
            column![
                text("Login to account").size(18),
                text("Enter your email below to login.").size(13),
                text("m@example.com").size(14),
                text("••••••••").size(14),
            ]
            .spacing(10),
            CardProps::new()
                .variant(CardVariant::Surface)
                .size(CardSize::Size3),
            theme,
        )
        .width(Length::Fixed(350.0));

        let background = theme.palette.background;
        let panel = theme.palette.card;
        let border = theme.palette.border;
        let radius = theme.radius.md;

        container(
            container(content)
                .padding(16)
                .style(move |_theme| iced::widget::container::Style {
                    background: Some(Background::Color(panel)),
                    border: Border {
                        radius: radius.into(),
                        width: 1.0,
                        color: border,
                    },
                    ..iced::widget::container::Style::default()
                }),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .style(move |_theme| iced::widget::container::Style {
            background: Some(Background::Color(background)),
            ..iced::widget::container::Style::default()
        })
        .into()
    }
}
