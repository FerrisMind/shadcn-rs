use iced::{
    widget::{column, container, text},
    Alignment, Element, Length,
};
use iced_shadcn::{button, input_otp_unified, ButtonProps, ButtonVariant, Theme};

pub fn main() -> iced::Result {
    iced::application(InputOtpForm::default, InputOtpForm::update, InputOtpForm::view).run()
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    Submit,
}

#[derive(Default)]
struct InputOtpForm {
    value: String,
    status: String,
    theme: Theme,
}

impl InputOtpForm {
    fn update(&mut self, message: Message) {
        match message {
            Message::InputChanged(value) => {
                self.value = value.chars().filter(|c| c.is_ascii_digit()).take(6).collect();
                self.status.clear();
            }
            Message::Submit => {
                if self.value.len() == 6 {
                    self.status = format!("Submitted: {}", self.value);
                } else {
                    self.status = "Please enter all 6 digits".to_string();
                }
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;

        let content = column![
            text("Verify Your Account").size(24),
            text("Enter the 6-digit code sent to your phone").size(14),

            input_otp_unified(&self.value, 6, Message::InputChanged, theme),

            button(
                "Submit",
                if self.value.len() == 6 { Some(Message::Submit) } else { None },
                ButtonProps::new().variant(ButtonVariant::Solid),
                theme
            ),

            text(&self.status).size(12),
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
