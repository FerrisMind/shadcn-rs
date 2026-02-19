use iced::border::Border;
use iced::widget::container;
use iced::{Background, Element, Length};

use iced_shadcn::{AlertDialogProps, ButtonProps, ButtonVariant, Theme, alert_dialog, button};

pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view).run()
}

#[derive(Debug, Clone)]
enum Message {
    Open,
    Cancel,
    Confirm,
}

#[derive(Default)]
struct Example {
    theme: Theme,
    open: bool,
}

impl Example {
    fn update(&mut self, message: Message) {
        match message {
            Message::Open => self.open = true,
            Message::Cancel | Message::Confirm => self.open = false,
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;

        let base = app(
            theme,
            preview(
                theme,
                button(
                    "Open",
                    Some(Message::Open),
                    ButtonProps::new().variant(ButtonVariant::Solid),
                    theme,
                ),
            )
            .into(),
        );

        alert_dialog(
            base,
            self.open,
            AlertDialogProps::new(
                "Revoke setup link",
                "The setup link will no longer be accessible and any existing setup sessions will be revoked.",
                Message::Confirm,
                Message::Cancel,
            )
            .confirm_label("Revoke link")
            .cancel_label("Cancel"),
            theme,
        )
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

    iced::widget::container(content)
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
