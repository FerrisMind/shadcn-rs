use iced::{
    widget::{column, container, text},
    Alignment, Element, Length,
};
use iced_shadcn::{input_otp_unified, Theme};

pub fn main() -> iced::Result {
    iced::application(InputOtpDemo::default, InputOtpDemo::update, InputOtpDemo::view).run()
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
}

#[derive(Default)]
struct InputOtpDemo {
    value: String,
    theme: Theme,
}

impl InputOtpDemo {
    fn update(&mut self, message: Message) {
        match message {
            Message::InputChanged(value) => {
                // Filter only digits
                self.value = value.chars().filter(|c| c.is_ascii_digit()).take(6).collect();
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;

        let content = column![
            text("Input OTP Demo").size(24),
            text("Click on slots and enter digits").size(14),

            // Unified OTP input - one hidden input with visual slots
            input_otp_unified(&self.value, 6, Message::InputChanged, theme),

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
