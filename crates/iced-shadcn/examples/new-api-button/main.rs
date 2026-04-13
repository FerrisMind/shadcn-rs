use iced::widget::{column, container, row, text};
use iced::{Alignment, Element, Length, Task};

use iced_shadcn::new_api::{Button, ButtonRadius, ButtonSize, ButtonVariant};
use iced_shadcn::{AccentColor, Theme};

pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view)
        .title(Example::title)
        .run()
}

struct Example {
    theme: Theme,
    loading: bool,
    pressed_count: u32,
}

#[derive(Debug, Clone)]
enum Message {
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
        "iced-shadcn new_api::Button".to_owned()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
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
            Button::text("Default", theme)
                .variant(ButtonVariant::Default)
                .color(AccentColor::Blue)
                .on_press(Message::Pressed),
            Button::text("Outline", theme)
                .variant(ButtonVariant::Outline)
                .on_press(Message::Pressed),
            Button::text("Secondary", theme)
                .variant(ButtonVariant::Secondary)
                .on_press(Message::Pressed),
            Button::text("Ghost", theme)
                .variant(ButtonVariant::Ghost)
                .on_press(Message::Pressed),
            Button::text("Link", theme)
                .variant(ButtonVariant::Link)
                .color(AccentColor::Indigo)
                .on_press(Message::Pressed),
            Button::text("Destructive", theme)
                .variant(ButtonVariant::Destructive)
                .on_press(Message::Pressed),
        ]
        .spacing(12)
        .align_y(Alignment::Center);

        let sizes = row![
            Button::text("S1", theme)
                .size(ButtonSize::Size1)
                .variant(ButtonVariant::Outline)
                .on_press(Message::Pressed),
            Button::text("S2", theme)
                .size(ButtonSize::Size2)
                .variant(ButtonVariant::Outline)
                .on_press(Message::Pressed),
            Button::text("S3", theme)
                .size(ButtonSize::Size3)
                .variant(ButtonVariant::Outline)
                .on_press(Message::Pressed),
            Button::icon(text("+"), theme)
                .size(ButtonSize::Size2)
                .radius(ButtonRadius::Full)
                .variant(ButtonVariant::Outline)
                .on_press(Message::Pressed),
        ]
        .spacing(12)
        .align_y(Alignment::Center);

        let states = column![
            Button::text(
                if self.loading {
                    "Остановить loading"
                } else {
                    "Включить loading"
                },
                theme,
            )
            .variant(ButtonVariant::Default)
            .color(AccentColor::Teal)
            .loading(self.loading)
            .on_press(Message::ToggleLoading),
            Button::text("Disabled", theme)
                .variant(ButtonVariant::Outline)
                .disabled(true)
                .on_press(Message::Pressed),
            Button::text("Full width action", theme)
                .variant(ButtonVariant::Default)
                .color(AccentColor::Green)
                .full_width()
                .height(44)
                .on_press(Message::Pressed),
        ]
        .spacing(12)
        .width(Length::Fill);

        container(
            column![
                text("new_api::Button").size(32),
                text("Builder-first API поверх twill").size(16),
                text(format!("Нажатий: {}", self.pressed_count)).size(14),
                text("Variants").size(20),
                variants,
                text("Sizes / icon").size(20),
                sizes,
                text("States / layout").size(20),
                states,
            ]
            .spacing(16)
            .max_width(960),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .padding(24)
        .into()
    }
}
