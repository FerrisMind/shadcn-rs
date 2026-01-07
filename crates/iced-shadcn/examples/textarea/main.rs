use iced::border::Border;
use iced::widget::{column, container, text, text_editor};
use iced::{Alignment, Background, Element, Length};

use iced_shadcn::{ButtonProps, TextareaProps, TextareaSize, Theme, button, label, textarea};

pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view).run()
}

struct Example {
    theme: Theme,
    content_default: text_editor::Content,
    content_disabled: text_editor::Content,
    content_label: text_editor::Content,
    content_text: text_editor::Content,
    content_button: text_editor::Content,
}

#[derive(Debug, Clone)]
enum Message {
    EditDefault(text_editor::Action),
    EditLabel(text_editor::Action),
    EditText(text_editor::Action),
    EditButton(text_editor::Action),
    Send,
}

impl Example {
    fn update(&mut self, message: Message) {
        match message {
            Message::EditDefault(action) => self.content_default.perform(action),
            Message::EditLabel(action) => self.content_label.perform(action),
            Message::EditText(action) => self.content_text.perform(action),
            Message::EditButton(action) => self.content_button.perform(action),
            Message::Send => {}
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let background = theme.palette.background;
        let border = theme.palette.border;
        let radius = theme.radius.md;

        let content = column![
            textarea(
                &self.content_default,
                "Type your message here.",
                Some(Message::EditDefault),
                TextareaProps::new().size(TextareaSize::Two),
                theme,
            )
            .width(420.0),
            textarea(
                &self.content_disabled,
                "Type your message here.",
                None::<fn(text_editor::Action) -> Message>,
                TextareaProps::new().size(TextareaSize::Two).disabled(true),
                theme,
            )
            .width(420.0),
            column![
                label("Your message", theme),
                textarea(
                    &self.content_label,
                    "Type your message here.",
                    Some(Message::EditLabel),
                    TextareaProps::new().size(TextareaSize::Two),
                    theme,
                )
                .width(420.0),
            ]
            .spacing(12),
            column![
                label("Your Message", theme),
                textarea(
                    &self.content_text,
                    "Type your message here.",
                    Some(Message::EditText),
                    TextareaProps::new().size(TextareaSize::Two),
                    theme,
                )
                .width(420.0),
                text("Your message will be copied to the support team.")
                    .size(14)
                    .style(|_theme| iced::widget::text::Style {
                        color: Some(theme.palette.muted_foreground),
                    }),
            ]
            .spacing(12),
            column![
                textarea(
                    &self.content_button,
                    "Type your message here.",
                    Some(Message::EditButton),
                    TextareaProps::new().size(TextareaSize::Two),
                    theme,
                )
                .width(420.0),
                button(
                    "Send message",
                    Some(Message::Send),
                    ButtonProps::new(),
                    theme,
                ),
            ]
            .spacing(12),
        ]
        .spacing(24)
        .align_x(Alignment::Start);

        container(content)
            .width(Length::Fill)
            .padding(32)
            .style(move |_theme| iced::widget::container::Style {
                background: Some(Background::Color(background)),
                border: Border {
                    radius: radius.into(),
                    width: 1.0,
                    color: border,
                },
                ..iced::widget::container::Style::default()
            })
            .into()
    }
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::default(),
            content_default: text_editor::Content::new(),
            content_disabled: text_editor::Content::new(),
            content_label: text_editor::Content::new(),
            content_text: text_editor::Content::new(),
            content_button: text_editor::Content::new(),
        }
    }
}
