//! Minimal playground for `iced-shadcn-v2::Button`.
//!
//! Run: `cargo run -p iced-shadcn-v2 --example button`

use iced::widget::{column, row, text};
use iced::{Alignment, Element, Length, Task};

use iced_shadcn_v2::{
    AccentColor, Button, ButtonRadius, ButtonSize, ButtonVariant, FontId, Theme, ThemeMode, fonts,
    iced_font,
};

pub fn main() -> iced::Result {
    let mut app = iced::application(Example::default, Example::update, Example::view)
        .title(Example::title)
        .default_font(iced_font(FontId::Geist));

    for face in fonts::ALL_FACES {
        app = app.font(*face);
    }

    app.run()
}

struct Example {
    theme: Theme,
    loading: bool,
    pressed_count: u32,
}

#[derive(Debug, Clone)]
enum Message {
    ToggleMode,
    ToggleLoading,
    Pressed,
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::light(),
            loading: false,
            pressed_count: 0,
        }
    }
}

impl Example {
    fn title(&self) -> String {
        "iced-shadcn-v2 Button".to_owned()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ToggleMode => {
                let mode = if self.theme.is_dark() {
                    ThemeMode::Light
                } else {
                    ThemeMode::Dark
                };
                self.theme = self.theme.clone().with_mode(mode);
            }
            Message::ToggleLoading => {
                self.loading = !self.loading;
            }
            Message::Pressed => {
                self.pressed_count += 1;
            }
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;

        let variants = row![
            Button::text("Default", theme).on_press(Message::Pressed),
            Button::text("Secondary", theme)
                .variant(ButtonVariant::Secondary)
                .on_press(Message::Pressed),
            Button::text("Destructive", theme)
                .variant(ButtonVariant::Destructive)
                .on_press(Message::Pressed),
            Button::text("Outline", theme)
                .variant(ButtonVariant::Outline)
                .on_press(Message::Pressed),
            Button::text("Ghost", theme)
                .variant(ButtonVariant::Ghost)
                .on_press(Message::Pressed),
            Button::text("Link", theme)
                .variant(ButtonVariant::Link)
                .on_press(Message::Pressed),
            Button::text("Soft", theme)
                .variant(ButtonVariant::Soft)
                .on_press(Message::Pressed),
            Button::text("Surface", theme)
                .variant(ButtonVariant::Surface)
                .on_press(Message::Pressed),
        ]
        .spacing(8)
        .align_y(Alignment::Center);

        let sizes = row![
            Button::text("Size0", theme)
                .size(ButtonSize::Size0)
                .on_press(Message::Pressed),
            Button::text("Size1", theme)
                .size(ButtonSize::Size1)
                .on_press(Message::Pressed),
            Button::text("Size2", theme)
                .size(ButtonSize::Size2)
                .on_press(Message::Pressed),
            Button::text("Size3", theme)
                .size(ButtonSize::Size3)
                .on_press(Message::Pressed),
            Button::text("Size4", theme)
                .size(ButtonSize::Size4)
                .on_press(Message::Pressed),
        ]
        .spacing(8)
        .align_y(Alignment::Center);

        let accents = row![
            Button::text("Blue", theme)
                .color(AccentColor::Blue)
                .on_press(Message::Pressed),
            Button::text("Emerald", theme)
                .color(AccentColor::Emerald)
                .on_press(Message::Pressed),
            Button::text("Rose", theme)
                .color(AccentColor::Rose)
                .radius(ButtonRadius::Full)
                .on_press(Message::Pressed),
            Button::text("Loading", theme)
                .loading(self.loading)
                .on_press(Message::Pressed),
            Button::text("Disabled", theme)
                .disabled(true)
                .on_press(Message::Pressed),
        ]
        .spacing(8)
        .align_y(Alignment::Center);

        let controls = row![
            Button::text("Toggle mode", theme)
                .variant(ButtonVariant::Outline)
                .on_press(Message::ToggleMode),
            Button::text("Toggle loading", theme)
                .variant(ButtonVariant::Outline)
                .on_press(Message::ToggleLoading),
            text(format!("Pressed: {}", self.pressed_count)),
        ]
        .spacing(8)
        .align_y(Alignment::Center);

        iced::widget::container(
            column![variants, sizes, accents, controls]
                .spacing(16)
                .padding(24),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .style(move |_| iced::widget::container::Style {
            background: Some(iced::Background::Color(theme.palette.background)),
            text_color: Some(theme.palette.foreground),
            ..Default::default()
        })
        .into()
    }
}
