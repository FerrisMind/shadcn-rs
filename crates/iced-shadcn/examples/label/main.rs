use iced::border::Border;
use iced::widget::{column, container, row, text_editor};
use iced::{Alignment, Background, Element, Length};

use iced_shadcn::{
    CheckboxProps, CheckboxSize, CheckboxState, LabelProps, TextFieldProps, TextareaProps, Theme,
    checkbox, label, label_with_props, text_field, textarea,
};
use lucide_icons::LUCIDE_FONT_BYTES;

pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view)
        .font(LUCIDE_FONT_BYTES)
        .run()
}

struct Example {
    theme: Theme,
    terms_state: CheckboxState,
    email_value: String,
    handle_value: String,
    message_value: text_editor::Content,
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::default(),
            terms_state: CheckboxState::Unchecked,
            email_value: String::new(),
            handle_value: String::new(),
            message_value: text_editor::Content::new(),
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    Toggle(CheckboxState),
    EmailChanged(String),
    HandleChanged(String),
    BodyChanged(text_editor::Action),
}

impl Example {
    fn update(&mut self, message: Message) {
        match message {
            Message::Toggle(state) => self.terms_state = state,
            Message::EmailChanged(value) => self.email_value = value,
            Message::HandleChanged(value) => self.handle_value = value,
            Message::BodyChanged(action) => self.message_value.perform(action),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let background = theme.palette.background;
        let border = theme.palette.border;
        let radius = theme.radius.md;
        let field_width = Length::Fixed(384.0);
        let field_pixels = 384.0;

        let checkbox_demo = preview(
            theme,
            column![
                row![
                    checkbox(
                        self.terms_state,
                        Some(Message::Toggle),
                        CheckboxProps::new().size(CheckboxSize::Two),
                        theme,
                    ),
                    label("Accept terms and conditions", theme),
                ]
                .spacing(8)
                .align_y(Alignment::Center),
                row![
                    checkbox(
                        CheckboxState::Unchecked,
                        None::<fn(CheckboxState) -> Message>,
                        CheckboxProps::new()
                            .size(CheckboxSize::Two)
                            .disabled(true),
                        theme,
                    ),
                    label_with_props(
                        "Label disabled by a peer control",
                        LabelProps::new().disabled(true),
                        theme,
                    ),
                ]
                .spacing(8)
                .align_y(Alignment::Center),
            ]
            .spacing(12),
        );

        let input_demo = preview(
            theme,
            column![
                label("Email", theme),
                text_field(
                    &self.email_value,
                    "Email",
                    Some(Message::EmailChanged),
                    TextFieldProps::new(),
                    theme,
                )
                .width(field_width),
            ]
            .spacing(12),
        );

        let textarea_demo = preview(
            theme,
            column![
                label("Your message", theme),
                textarea(
                    &self.message_value,
                    "Type your message here.",
                    Some(Message::BodyChanged),
                    TextareaProps::new(),
                    theme,
                )
                .width(field_pixels),
            ]
            .spacing(12),
        );

        let inline_demo = preview(
            theme,
            row![
                label("@", theme),
                text_field(
                    &self.handle_value,
                    "shadcn",
                    Some(Message::HandleChanged),
                    TextFieldProps::new(),
                    theme,
                )
                .width(field_width),
            ]
            .spacing(8)
            .align_y(Alignment::Center),
        );

        let content = column![checkbox_demo, input_demo, textarea_demo, inline_demo]
            .spacing(16)
            .align_x(Alignment::Start);

        container(content)
            .padding(24)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
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

fn preview<'a, Message: 'a>(
    theme: &Theme,
    content: impl Into<Element<'a, Message>>,
) -> iced::widget::Container<'a, Message> {
    let background = theme.palette.card;
    let border = theme.palette.border;
    let radius = theme.radius.md;

    container(content)
        .padding(20)
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
