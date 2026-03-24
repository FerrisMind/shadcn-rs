use iced::border::Border;
use iced::widget::{column, container, row, scrollable, text as iced_text};
use iced::{Alignment, Background, Element, Length};

use iced_shadcn::{
    ButtonProps, ButtonVariant, TextProps, TextSize, TextWeight, Theme, button,
    input_otp_separator, input_otp_unified, text,
};

pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view).run()
}

#[derive(Default)]
struct Example {
    theme: Theme,
    demo_value: String,
    pattern_value: String,
    separator_value: String,
    controlled_value: String,
    form_value: String,
    form_status: String,
}

#[derive(Debug, Clone)]
enum Message {
    DemoChanged(String),
    PatternChanged(String),
    SeparatorChanged(String),
    ControlledChanged(String),
    ControlledClear,
    FormChanged(String),
    FormSubmit,
}

impl Example {
    fn update(&mut self, message: Message) {
        match message {
            Message::DemoChanged(value) => {
                self.demo_value = value
                    .chars()
                    .filter(|c| c.is_ascii_digit())
                    .take(6)
                    .collect();
            }
            Message::PatternChanged(value) => {
                self.pattern_value = value
                    .chars()
                    .filter(|c| c.is_ascii_alphanumeric())
                    .take(6)
                    .enumerate()
                    .filter_map(|(index, ch)| {
                        if index < 3 {
                            ch.is_ascii_alphabetic().then_some(ch.to_ascii_uppercase())
                        } else {
                            ch.is_ascii_digit().then_some(ch)
                        }
                    })
                    .collect();
            }
            Message::SeparatorChanged(value) => {
                self.separator_value = value
                    .chars()
                    .filter(|c| c.is_ascii_alphanumeric())
                    .take(6)
                    .collect();
            }
            Message::ControlledChanged(value) => {
                self.controlled_value = value
                    .chars()
                    .filter(|c| c.is_ascii_digit())
                    .take(6)
                    .collect();
            }
            Message::ControlledClear => {
                self.controlled_value.clear();
            }
            Message::FormChanged(value) => {
                self.form_value = value
                    .chars()
                    .filter(|c| c.is_ascii_digit())
                    .take(6)
                    .collect();
                self.form_status.clear();
            }
            Message::FormSubmit => {
                if self.form_value.len() == 6 {
                    self.form_status = format!("Submitted: {}", self.form_value);
                } else {
                    self.form_status = "Please enter all 6 digits".to_string();
                }
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let background = theme.palette.background;
        let border = theme.palette.border;
        let radius = theme.radius.md;

        // Demo
        let demo_section = column![
            input_otp_unified(&self.demo_value, 6, Message::DemoChanged, theme),
            muted_text(format!("Value: '{}'", self.demo_value), theme),
        ]
        .spacing(8)
        .align_x(Alignment::Start);

        // Pattern (3 letters + 3 digits)
        let pattern_section = column![
            muted_text("Pattern: 3 letters + 3 digits (e.g., ABC123)", theme),
            input_otp_unified(&self.pattern_value, 6, Message::PatternChanged, theme),
            muted_text(
                format!("Formatted: '{}'", format_grouped(&self.pattern_value)),
                theme
            ),
        ]
        .spacing(8)
        .align_x(Alignment::Start);

        // Separator (2-2-2 groups)
        let separator_section = column![
            muted_text("Multiple groups with separators (2-2-2).", theme),
            input_otp_unified(&self.separator_value, 6, Message::SeparatorChanged, theme),
            row![
                iced_text(group_slice(&self.separator_value, 0, 2)).size(13),
                input_otp_separator(theme),
                iced_text(group_slice(&self.separator_value, 2, 4)).size(13),
                input_otp_separator(theme),
                iced_text(group_slice(&self.separator_value, 4, 6)).size(13),
            ]
            .spacing(6)
            .align_y(Alignment::Center),
            muted_text(
                format!("Value: '{}'", format_grouped(&self.separator_value)),
                theme
            ),
        ]
        .spacing(8)
        .align_x(Alignment::Start);

        // Controlled
        let controlled_helper = if self.controlled_value.is_empty() {
            "Enter your one-time password.".to_string()
        } else {
            format!("You entered: {}", self.controlled_value)
        };
        let controlled_section = column![
            input_otp_unified(&self.controlled_value, 6, Message::ControlledChanged, theme,),
            muted_text(controlled_helper, theme),
            button(
                "Clear",
                Some(Message::ControlledClear),
                ButtonProps::new().variant(ButtonVariant::Outline),
                theme,
            ),
        ]
        .spacing(8)
        .align_x(Alignment::Start);

        // Form
        let form_section = column![
            muted_text("Enter the 6-digit code sent to your phone.", theme),
            input_otp_unified(&self.form_value, 6, Message::FormChanged, theme),
            button(
                "Submit",
                if self.form_value.len() == 6 {
                    Some(Message::FormSubmit)
                } else {
                    None
                },
                ButtonProps::new().variant(ButtonVariant::Solid),
                theme,
            ),
            muted_text(&self.form_status, theme),
        ]
        .spacing(8)
        .align_x(Alignment::Start);

        let content = column![
            section(theme, "Demo", demo_section),
            section(theme, "Pattern", pattern_section),
            section(theme, "Separator", separator_section),
            section(theme, "Controlled", controlled_section),
            section(theme, "Form", form_section),
        ]
        .spacing(16);

        let content = scrollable(content).height(Length::Fill);

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

fn section<'a, Message: 'a>(
    theme: &Theme,
    title: impl iced::widget::text::IntoFragment<'a>,
    content: impl Into<Element<'a, Message>>,
) -> iced::widget::Container<'a, Message> {
    let title = text(
        title,
        TextProps::new()
            .size(TextSize::Size4)
            .weight(TextWeight::Medium),
        theme,
    );
    let background = theme.palette.card;
    let border = theme.palette.border;
    let radius = theme.radius.md;

    container(column![title, content.into()].spacing(12))
        .padding(16)
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

fn muted_text<'a>(
    content: impl iced::widget::text::IntoFragment<'a>,
    theme: &Theme,
) -> iced::widget::Text<'a> {
    let color = theme.palette.muted_foreground;
    iced_text(content)
        .size(13)
        .style(move |_theme| iced::widget::text::Style { color: Some(color) })
}

fn group_slice(value: &str, start: usize, end: usize) -> String {
    value
        .chars()
        .skip(start)
        .take(end.saturating_sub(start))
        .collect()
}

fn format_grouped(value: &str) -> String {
    let mut parts = Vec::new();
    let first = group_slice(value, 0, 2);
    let second = group_slice(value, 2, 4);
    let third = group_slice(value, 4, 6);

    if !first.is_empty() {
        parts.push(first);
    }
    if !second.is_empty() {
        parts.push(second);
    }
    if !third.is_empty() {
        parts.push(third);
    }

    parts.join("-")
}
