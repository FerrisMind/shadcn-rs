use iced::border::Border;
use iced::widget::container;
use iced::{Background, Element, Length};
use lucide_icons::LUCIDE_FONT_BYTES;

use iced_shadcn::{ButtonProps, ButtonVariant, Theme, Toast, Toaster, button};

pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view)
        .font(LUCIDE_FONT_BYTES)
        .run()
}

#[derive(Debug, Clone)]
enum Message {
    ShowToast,
}

#[derive(Default)]
struct Example {
    theme: Theme,
    toaster: Toaster,
}

impl Example {
    fn update(&mut self, message: Message) {
        match message {
            Message::ShowToast => {
                self.toaster.show(
                    Toast::new("Event has been created")
                        .description("Sunday, December 03, 2023 at 9:00 AM"),
                );
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;

        let base = app(
            theme,
            preview(
                theme,
                button(
                    "Show Toast",
                    Some(Message::ShowToast),
                    ButtonProps::new().variant(ButtonVariant::Outline),
                    theme,
                ),
            )
            .into(),
        );

        self.toaster.overlay(base, theme)
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
