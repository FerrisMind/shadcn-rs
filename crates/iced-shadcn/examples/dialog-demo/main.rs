use iced::border::Border;
use iced::widget::{column, container, row, text as iced_text};
use iced::{Alignment, Background, Element, Length};

use iced_shadcn::{
    ButtonProps, ButtonSize, ButtonVariant, DialogProps, InputProps, Theme, button, dialog, input,
    label,
};

pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view).run()
}

#[derive(Debug, Clone)]
enum Message {
    Open,
    Close,
    NameChanged(String),
    UsernameChanged(String),
    Save,
}

struct Example {
    theme: Theme,
    open: bool,
    name: String,
    username: String,
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::default(),
            open: false,
            name: "Pedro Duarte".to_owned(),
            username: "@peduarte".to_owned(),
        }
    }
}

impl Example {
    fn update(&mut self, message: Message) {
        match message {
            Message::Open => self.open = true,
            Message::Close => self.open = false,
            Message::NameChanged(value) => self.name = value,
            Message::UsernameChanged(value) => self.username = value,
            Message::Save => self.open = false,
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;

        let base_content = preview(
            theme,
            button(
                "Open Dialog",
                Some(Message::Open),
                ButtonProps::new().variant(ButtonVariant::Outline),
                theme,
            ),
        );

        let base = app(theme, base_content.into());

        let dialog_content = column![
            iced_text("Edit profile").size(20),
            iced_text("Make changes to your profile here. Click save when you're done.").size(14),
            column![
                label("Name", theme),
                input(
                    &self.name,
                    "Name",
                    Some(Message::NameChanged),
                    InputProps::new(),
                    theme
                ),
            ]
            .spacing(6),
            column![
                label("Username", theme),
                input(
                    &self.username,
                    "Username",
                    Some(Message::UsernameChanged),
                    InputProps::new(),
                    theme
                ),
            ]
            .spacing(6),
            row![
                button(
                    "Cancel",
                    Some(Message::Close),
                    ButtonProps::new().variant(ButtonVariant::Outline),
                    theme
                ),
                button(
                    "Save changes",
                    Some(Message::Save),
                    ButtonProps::new().size(ButtonSize::Size2),
                    theme
                ),
            ]
            .spacing(12)
            .align_y(Alignment::Center),
        ]
        .spacing(12)
        .width(Length::Fixed(380.0));

        dialog(
            base,
            self.open,
            dialog_content,
            Message::Close,
            DialogProps::new().max_width(425),
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
