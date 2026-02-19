use iced::{
    widget::{column, container, row, text},
    Alignment, Element, Length,
};
use iced_shadcn::{input_otp_separator, input_otp_unified, Theme};

pub fn main() -> iced::Result {
    iced::application(
        InputOtpSeparator::default,
        InputOtpSeparator::update,
        InputOtpSeparator::view,
    )
    .run()
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
}

#[derive(Default)]
struct InputOtpSeparator {
    value: String,
    theme: Theme,
}

impl InputOtpSeparator {
    fn update(&mut self, message: Message) {
        match message {
            Message::InputChanged(value) => {
                self.value = value.chars().filter(|c| c.is_ascii_alphanumeric()).take(6).collect();
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;

        // Split value into three groups of 2
        let first = &self.value[..self.value.len().min(2)];
        let second = if self.value.len() > 2 {
            &self.value[2..self.value.len().min(4)]
        } else {
            ""
        };
        let third = if self.value.len() > 4 {
            &self.value[4..self.value.len().min(6)]
        } else {
            ""
        };

        let content = column![
            text("Input OTP with Separator").size(24),
            text("OTP with visual separator between groups").size(14),

            // Three groups with separators (2-2-2)
            row![
                input_otp_unified(first, 2, |v| {
                    let rest = if self.value.len() > 2 { &self.value[2..] } else { "" };
                    Message::InputChanged(format!("{}{}", v, rest))
                }, theme),
                input_otp_separator(theme),
                input_otp_unified(second, 2, |v| {
                    let first = &self.value[..self.value.len().min(2)];
                    let third = if self.value.len() > 4 { &self.value[4..] } else { "" };
                    Message::InputChanged(format!("{}{}{}", first, v, third))
                }, theme),
                input_otp_separator(theme),
                input_otp_unified(third, 2, |v| {
                    let first_two = &self.value[..self.value.len().min(4)];
                    Message::InputChanged(format!("{}{}", first_two, v))
                }, theme),
            ]
            .spacing(8)
            .align_y(Alignment::Center),

            text(format!("Value: '{}'", self.value)).size(12),
        ]
        .spacing(20)
        .padding(20)
        .align_x(Alignment::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_| iced::widget::container::Style {
                background: Some(iced::Background::Color(theme.palette.background)),
                ..Default::default()
            })
            .into()
    }
}
