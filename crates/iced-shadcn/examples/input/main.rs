use iced::border::Border;
use iced::widget::{column, container, row, text};
use iced::{Alignment, Background, Element, Length, Task};
use rfd::FileDialog;
use std::path::PathBuf;

use iced_shadcn::{
    ButtonProps, ButtonSize, ButtonVariant, TextFieldProps, TextFieldSize, TextFieldVariant, Theme,
    button, label, text_field,
};

pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view).run()
}

#[derive(Default)]
struct Example {
    theme: Theme,
    demo_value: String,
    labeled_value: String,
    with_text_value: String,
    with_button_value: String,
    file_value: String,
    form_value: String,
}

#[derive(Debug, Clone)]
enum Message {
    Demo(String),
    Labeled(String),
    WithText(String),
    WithButton(String),
    File(String),
    PickFile,
    FilePicked(Option<PathBuf>),
    Form(String),
    Submit,
}

impl Example {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Demo(value) => self.demo_value = value,
            Message::Labeled(value) => self.labeled_value = value,
            Message::WithText(value) => self.with_text_value = value,
            Message::WithButton(value) => self.with_button_value = value,
            Message::File(value) => self.file_value = value,
            Message::PickFile => {
                return Task::perform(async { FileDialog::new().pick_file() }, Message::FilePicked);
            }
            Message::FilePicked(path) => {
                if let Some(path) = path {
                    self.file_value = path.display().to_string();
                }
            }
            Message::Form(value) => self.form_value = value,
            Message::Submit => {}
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let background = theme.palette.background;
        let border = theme.palette.border;
        let radius = theme.radius.md;
        let field_width = Length::Fixed(384.0);

        let demo = preview(
            theme,
            text_field(
                &self.demo_value,
                "Email",
                Some(Message::Demo),
                TextFieldProps::new()
                    .size(TextFieldSize::Two)
                    .variant(TextFieldVariant::Surface),
                theme,
            )
            .width(field_width),
        );

        let disabled = preview(
            theme,
            text_field(
                &self.demo_value,
                "Email",
                None::<fn(String) -> Message>,
                TextFieldProps::new()
                    .size(TextFieldSize::Two)
                    .variant(TextFieldVariant::Surface)
                    .disabled(true),
                theme,
            )
            .width(field_width),
        );

        let with_label = preview(
            theme,
            column![
                label("Email", theme),
                text_field(
                    &self.labeled_value,
                    "Email",
                    Some(Message::Labeled),
                    TextFieldProps::new()
                        .size(TextFieldSize::Two)
                        .variant(TextFieldVariant::Surface),
                    theme,
                )
                .width(field_width),
            ]
            .spacing(12),
        );

        let with_text = preview(
            theme,
            column![
                label("Email", theme),
                text_field(
                    &self.with_text_value,
                    "Email",
                    Some(Message::WithText),
                    TextFieldProps::new()
                        .size(TextFieldSize::Two)
                        .variant(TextFieldVariant::Surface),
                    theme,
                )
                .width(field_width),
                text("Enter your email address.").size(14).style(|_theme| {
                    iced::widget::text::Style {
                        color: Some(theme.palette.muted_foreground),
                    }
                }),
            ]
            .spacing(12),
        );

        let with_button = preview(
            theme,
            row![
                text_field(
                    &self.with_button_value,
                    "Email",
                    Some(Message::WithButton),
                    TextFieldProps::new()
                        .size(TextFieldSize::Two)
                        .variant(TextFieldVariant::Surface),
                    theme,
                )
                .width(Length::Fill),
                button(
                    "Subscribe",
                    Some(Message::Submit),
                    ButtonProps::new()
                        .variant(ButtonVariant::Outline)
                        .size(ButtonSize::Two),
                    theme,
                ),
            ]
            .spacing(8)
            .align_y(Alignment::Center)
            .width(field_width),
        );

        let file_input = preview(
            theme,
            column![
                label("Picture", theme),
                row![
                    text_field(
                        &self.file_value,
                        "Choose file",
                        Some(Message::File),
                        TextFieldProps::new()
                            .size(TextFieldSize::Two)
                            .variant(TextFieldVariant::Surface)
                            .read_only(true),
                        theme,
                    )
                    .width(Length::Fill),
                    button(
                        "Browse",
                        Some(Message::PickFile),
                        ButtonProps::new()
                            .variant(ButtonVariant::Outline)
                            .size(ButtonSize::Two),
                        theme,
                    ),
                ]
                .spacing(8)
                .align_y(Alignment::Center)
                .width(field_width),
            ]
            .spacing(12),
        );

        let form = preview(
            theme,
            column![
                label("Username", theme),
                text_field(
                    &self.form_value,
                    "shadcn",
                    Some(Message::Form),
                    TextFieldProps::new()
                        .size(TextFieldSize::Two)
                        .variant(TextFieldVariant::Surface),
                    theme,
                )
                .width(field_width),
                text("This is your public display name.")
                    .size(14)
                    .style(|_theme| iced::widget::text::Style {
                        color: Some(theme.palette.muted_foreground),
                    }),
                button(
                    "Submit",
                    Some(Message::Submit),
                    ButtonProps::new()
                        .variant(ButtonVariant::Solid)
                        .size(ButtonSize::Two),
                    theme,
                ),
            ]
            .spacing(12),
        );

        let content = column![
            row![demo, disabled].spacing(20).align_y(Alignment::Center),
            row![with_label, with_text]
                .spacing(20)
                .align_y(Alignment::Center),
            row![with_button, file_input]
                .spacing(20)
                .align_y(Alignment::Center),
            row![form].spacing(20).align_y(Alignment::Center),
        ]
        .spacing(20)
        .align_x(Alignment::Center);

        container(content)
            .padding(24)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
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

fn preview<'a, Message: 'a>(
    theme: &Theme,
    content: impl Into<Element<'a, Message>>,
) -> iced::widget::Container<'a, Message> {
    let background = theme.palette.card;
    let border = theme.palette.border;
    let radius = theme.radius.md;

    container(content)
        .padding(20)
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
