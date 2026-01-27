use iced::border::Border;
use iced::widget::{column, container, row, text as iced_text};
use iced::{Alignment, Background, Element, Length};

use iced_shadcn::{
    ButtonProps, ButtonSize, CardProps, CardVariant, TextFieldProps, TextFieldSize, Theme, button,
    card, text_field,
};

pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view).run()
}

#[derive(Default)]
struct Example {
    theme: Theme,
    email: String,
    password: String,
}

#[derive(Debug, Clone)]
enum Message {
    EmailChanged(String),
    PasswordChanged(String),
    Submit,
}

impl Example {
    fn update(&mut self, message: Message) {
        match message {
            Message::EmailChanged(value) => self.email = value,
            Message::PasswordChanged(value) => self.password = value,
            Message::Submit => {}
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;

        let form = column![
            iced_text("Sign in").size(18),
            column![
                iced_text("Email").size(12),
                text_field(
                    &self.email,
                    "you@example.com",
                    Some(Message::EmailChanged),
                    TextFieldProps::new().size(TextFieldSize::Two),
                    theme
                )
            ]
            .spacing(6),
            column![
                iced_text("Password").size(12),
                text_field(
                    &self.password,
                    "••••••••",
                    Some(Message::PasswordChanged),
                    TextFieldProps::new().size(TextFieldSize::Two),
                    theme
                )
            ]
            .spacing(6),
            row![button(
                "Continue",
                Some(Message::Submit),
                ButtonProps::new().size(ButtonSize::Two),
                theme
            )]
            .align_y(Alignment::Center)
        ]
        .spacing(12);

        let card = card(form, CardProps::new().variant(CardVariant::Surface), theme)
            .width(Length::Fixed(360.0));

        app(theme, preview(theme, card).into())
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

