use std::borrow::Cow;

use iced::border::Border;
use iced::widget::{column, container, text as iced_text, text_editor};
use iced::{Background, Element, Length};

use iced_shadcn::{
    ButtonProps, ButtonVariant, DialogProps, DropdownMenuEntry, DropdownMenuItem,
    DropdownMenuProps, TextFieldProps, TextareaProps, Theme, button, dialog, dropdown_menu, label,
    text_field, textarea,
};

pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view).run()
}

#[derive(Debug, Clone)]
enum Message {
    OpenNew,
    OpenShare,
    CloseNew,
    CloseShare,
    FileNameChanged(String),
    EmailChanged(String),
    ContentChanged(text_editor::Action),
    Submit,
}

struct Example {
    theme: Theme,
    show_new: bool,
    show_share: bool,
    file_name: String,
    email: String,
    message: text_editor::Content,
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::default(),
            show_new: false,
            show_share: false,
            file_name: String::new(),
            email: String::new(),
            message: text_editor::Content::new(),
        }
    }
}

impl Example {
    fn update(&mut self, message: Message) {
        match message {
            Message::OpenNew => self.show_new = true,
            Message::OpenShare => self.show_share = true,
            Message::CloseNew => self.show_new = false,
            Message::CloseShare => self.show_share = false,
            Message::FileNameChanged(value) => self.file_name = value,
            Message::EmailChanged(value) => self.email = value,
            Message::ContentChanged(action) => self.message.perform(action),
            Message::Submit => {
                self.show_new = false;
                self.show_share = false;
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;

        let entries = vec![
            DropdownMenuEntry::Label(Cow::Borrowed("File Actions")),
            DropdownMenuEntry::Item(DropdownMenuItem::new("New File...", Some(Message::OpenNew))),
            DropdownMenuEntry::Item(DropdownMenuItem::new("Share...", Some(Message::OpenShare))),
            DropdownMenuEntry::Item(
                DropdownMenuItem::new("Download", None)
                    .props(iced_shadcn::DropdownMenuItemProps::new().disabled(true)),
            ),
        ];

        let trigger = button(
            "Open menu",
            None,
            ButtonProps::new().variant(ButtonVariant::Outline),
            theme,
        );

        let base = app(
            theme,
            preview(
                theme,
                dropdown_menu(trigger, entries, DropdownMenuProps::new().width(160), theme),
            )
            .into(),
        );

        let new_dialog_content = column![
            iced_text("Create New File").size(20),
            iced_text("Provide a name for your new file. Click create when you're done.").size(14),
            column![
                label("File Name", theme),
                text_field(
                    &self.file_name,
                    "document.txt",
                    Some(Message::FileNameChanged),
                    TextFieldProps::new(),
                    theme
                ),
            ]
            .spacing(6),
            button(
                "Create",
                Some(Message::Submit),
                ButtonProps::new().variant(ButtonVariant::Solid),
                theme
            )
        ]
        .spacing(12)
        .width(Length::Fixed(380.0));

        let share_dialog_content = column![
            iced_text("Share File").size(20),
            iced_text("Anyone with the link will be able to view this file.").size(14),
            column![
                label("Email Address", theme),
                text_field(
                    &self.email,
                    "shadcn@vercel.com",
                    Some(Message::EmailChanged),
                    TextFieldProps::new(),
                    theme
                ),
            ]
            .spacing(6),
            column![
                label("Message (Optional)", theme),
                textarea(
                    &self.message,
                    "Check out this file",
                    Some(Message::ContentChanged),
                    TextareaProps::new(),
                    theme
                ),
            ]
            .spacing(6),
            button(
                "Send Invite",
                Some(Message::Submit),
                ButtonProps::new().variant(ButtonVariant::Solid),
                theme
            )
        ]
        .spacing(12)
        .width(Length::Fixed(380.0));

        let with_new = dialog(
            base,
            self.show_new,
            new_dialog_content,
            Message::CloseNew,
            DialogProps::new().max_width(425),
            theme,
        );

        dialog(
            with_new,
            self.show_share,
            share_dialog_content,
            Message::CloseShare,
            DialogProps::new().max_width(425),
            theme,
        )
    }
}

fn app<'a, Message: 'a>(theme: &Theme, content: Element<'a, Message>) -> Element<'a, Message> {
    let background = theme.palette.background;
    container(content)
        .padding(24)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(move |_theme| iced::widget::container::Style {
            background: Some(Background::Color(background)),
            ..iced::widget::container::Style::default()
        })
        .into()
}

fn preview<'a, Message: 'a>(
    theme: &Theme,
    content: impl Into<Element<'a, Message>>,
) -> iced::widget::Container<'a, Message> {
    let background = theme.palette.card;
    let border = theme.palette.border;
    let radius = theme.radius.md;

    container(content)
        .padding(16)
        .width(Length::Shrink)
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
