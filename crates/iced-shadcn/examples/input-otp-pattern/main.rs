use iced::{
    widget::{column, container, row, text},
    Alignment, Element, Length,
};
use iced_shadcn::{input_otp_separator, input_otp_unified, Theme};

pub fn main() -> iced::Result {
    iced::application(
        InputOtpPattern::default,
        InputOtpPattern::update,
        InputOtpPattern::view,
    )
    .run()
}

#[derive(Debug, Clone)]
enum Message {
    LettersChanged(String),
    DigitsChanged(String),
}

#[derive(Default)]
struct InputOtpPattern {
    letters: String,
    digits: String,
    theme: Theme,
}

impl InputOtpPattern {
    fn update(&mut self, message: Message) {
        match message {
            Message::LettersChanged(value) => {
                // Only accept letters, max 3
                self.letters = value
                    .chars()
                    .filter(|c| c.is_ascii_alphabetic())
                    .take(3)
                    .map(|c| c.to_ascii_uppercase())
                    .collect();
            }
            Message::DigitsChanged(value) => {
                // Only accept digits, max 3
                self.digits = value.chars().filter(|c| c.is_ascii_digit()).take(3).collect();
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;

        let content = column![
            text("Pattern Input OTP").size(24),
            text("Pattern: 3 letters + 3 digits (e.g., ABC123)").size(14),

            // Two groups: letters and digits with separator
            row![
                input_otp_unified(&self.letters, 3, Message::LettersChanged, theme),
                input_otp_separator(theme),
                input_otp_unified(&self.digits, 3, Message::DigitsChanged, theme),
            ]
            .spacing(8)
            .align_y(Alignment::Center),

            text(format!("Value: '{}{}'", self.letters, self.digits)).size(12),
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
