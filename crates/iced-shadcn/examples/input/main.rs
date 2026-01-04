use iced::border::Border;
use iced::widget::{column, container, row, text};
use iced::{Alignment, Background, Element, Length};

use iced_shadcn::{ButtonSize, ButtonVariant, InputSize, Theme, button, input, label};

pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view).run()
}

#[derive(Default)]
struct Example {
    theme: Theme,
    value: String,
    labeled_value: String,
    with_text_value: String,
    with_button_value: String,
    file_value: String,
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    LabeledChanged(String),
    WithTextChanged(String),
    WithButtonChanged(String),
    FileChanged(String),
    Submit,
}

impl Example {
    fn update(&mut self, message: Message) {
        match message {
            Message::InputChanged(value) => self.value = value,
            Message::LabeledChanged(value) => self.labeled_value = value,
            Message::WithTextChanged(value) => self.with_text_value = value,
            Message::WithButtonChanged(value) => self.with_button_value = value,
            Message::FileChanged(value) => self.file_value = value,
            Message::Submit => {}
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let muted = theme.palette.muted;
        let border = theme.palette.border;
        let radius = theme.radius.md;

        let demo = input(
            &self.value,
            "Email",
            Message::InputChanged,
            InputSize::Md,
            theme,
        );

        let disabled = input(
            &self.value,
            "Email",
            Message::InputChanged,
            InputSize::Md,
            theme,
        )
        .on_input_maybe(None::<fn(String) -> Message>);

        let with_label = column![
            label("Email", theme),
            input(
                &self.labeled_value,
                "Email",
                Message::LabeledChanged,
                InputSize::Md,
                theme,
            ),
        ]
        .spacing(8)
        .width(Length::Fixed(320.0));

        let with_text = column![
            label("Email", theme),
            input(
                &self.with_text_value,
                "Email",
                Message::WithTextChanged,
                InputSize::Md,
                theme,
            ),
            text("Enter your email address.")
                .size(14)
                .style(|_theme| iced::widget::text::Style {
                    color: Some(theme.palette.muted_foreground),
                }),
        ]
        .spacing(8)
        .width(Length::Fixed(320.0));

        let with_button = row![
            input(
                &self.with_button_value,
                "Email",
                Message::WithButtonChanged,
                InputSize::Md,
                theme,
            ),
            button(
                "Subscribe",
                Some(Message::Submit),
                ButtonVariant::Outline,
                ButtonSize::Md,
                theme,
            ),
        ]
        .spacing(8)
        .align_y(Alignment::Center)
        .width(Length::Fixed(320.0));

        let file_input = column![
            label("Picture", theme),
            input(
                &self.file_value,
                "Choose file",
                Message::FileChanged,
                InputSize::Md,
                theme,
            ),
        ]
        .spacing(8)
        .width(Length::Fixed(320.0));

        let content = column![
            demo,
            disabled,
            with_label,
            with_text,
            with_button,
            file_input,
        ]
        .spacing(20)
        .align_x(Alignment::Start);

        container(content)
            .padding(24)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(move |_theme| iced::widget::container::Style {
                background: Some(Background::Color(muted)),
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
