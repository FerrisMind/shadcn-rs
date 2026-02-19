use iced::{
    widget::{column, container, text},
    Alignment, Element, Length,
};
use iced_shadcn::{button, input_otp_unified, ButtonProps, ButtonVariant, Theme};

pub fn main() -> iced::Result {
    iced::application(
        InputOtpControlled::default,
        InputOtpControlled::update,
        InputOtpControlled::view,
    )
    .run()
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    Clear,
}

#[derive(Default)]
struct InputOtpControlled {
    value: String,
    theme: Theme,
}

impl InputOtpControlled {
    fn update(&mut self, message: Message) {
        match message {
            Message::InputChanged(value) => {
                self.value = value.chars().filter(|c| c.is_ascii_digit()).take(6).collect();
            }
            Message::Clear => {
                self.value.clear();
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;

        let content = column![
            text("Controlled Input OTP").size(24),
            text("Enter your verification code").size(14),

            input_otp_unified(&self.value, 6, Message::InputChanged, theme),

            text(if self.value.is_empty() {
                "Enter your one-time password.".to_string()
            } else {
                format!("You entered: {}", self.value)
            })
            .size(12),

            button(
                "Clear",
                Some(Message::Clear),
                ButtonProps::new().variant(ButtonVariant::Outline),
                theme
            ),
        ]
        .spacing(16)
        .padding(24)
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
