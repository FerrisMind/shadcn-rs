use iced::widget::{column, container, text};
use iced::{Alignment, Element, Length, Sandbox, Settings};
use iced_shadcn::{CardProps, CardVariant, Theme, card};

pub fn main() -> iced::Result {
    IcedApp::run(Settings::default())
}

struct IcedApp {
    theme: Theme,
}

impl Sandbox for IcedApp {
    type Message = ();

    fn new() -> Self {
        Self {
            theme: Theme::default(),
        }
    }

    fn title(&self) -> String {
        String::from("Iced Shadcn Card Demo")
    }

    fn update(&mut self, _message: Self::Message) {}

    fn view(&self) -> Element<Self::Message> {
        let theme = &self.theme;
        
        container(
            card(
                column![
                    text("Login to account").size(18).bold(),
                    text("Enter your email below to login.").size(13),
                    column![
                        text("Email").size(12),
                        // Note: Iced Shadcn might have Input too, but for now simple Card content
                        text("m@example.com").size(14),
                    ]
                    .spacing(8),
                    column![
                        text("Password").size(12),
                        text("••••••••").size(14),
                    ]
                    .spacing(8),
                ]
                .spacing(20),
                CardProps::new()
                    .variant(CardVariant::Outline)
                    .padding(24.0),
                theme,
            )
            .width(Length::Fixed(350.0))
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
    }
}
