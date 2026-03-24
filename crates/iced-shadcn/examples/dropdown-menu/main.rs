use std::borrow::Cow;

use iced::border::Border;
use iced::widget::{column, container, row, text as iced_text};
use iced::{Alignment, Background, Element, Length};

use iced_shadcn::{
    AccentColor, ButtonProps, ButtonVariant, DropdownMenuContentSize, DropdownMenuContentVariant,
    DropdownMenuEntry, DropdownMenuItem, DropdownMenuItemProps, DropdownMenuProps,
    DropdownMenuSubMenu, Theme, button, dropdown_menu,
};
use lucide_icons::LUCIDE_FONT_BYTES;

pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view)
        .font(LUCIDE_FONT_BYTES)
        .run()
}

#[derive(Debug, Clone)]
enum Message {
    Selected(&'static str),
}

#[derive(Default)]
struct Example {
    theme: Theme,
    last_action: Option<&'static str>,
}

impl Example {
    fn update(&mut self, message: Message) {
        match message {
            Message::Selected(value) => self.last_action = Some(value),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;

        let content = column![
            row![
                dropdown_menu(
                    button(
                        "More",
                        None,
                        ButtonProps::new().variant(ButtonVariant::Soft),
                        theme
                    ),
                    dropdown_entries(),
                    DropdownMenuProps::new()
                        .size(DropdownMenuContentSize::Size1)
                        .variant(DropdownMenuContentVariant::Solid)
                        .color(AccentColor::Gray),
                    theme,
                ),
                dropdown_menu(
                    button(
                        "More",
                        None,
                        ButtonProps::new().variant(ButtonVariant::Soft),
                        theme
                    ),
                    dropdown_entries(),
                    DropdownMenuProps::new()
                        .size(DropdownMenuContentSize::Size2)
                        .variant(DropdownMenuContentVariant::Soft)
                        .color(AccentColor::Blue)
                        .high_contrast(true),
                    theme,
                ),
            ]
            .spacing(16)
            .align_y(Alignment::Center),
            iced_text(self.last_action.unwrap_or("Select an item")).size(12),
        ]
        .spacing(12);

        app(theme, preview(theme, content).into())
    }
}

fn dropdown_entries() -> Vec<DropdownMenuEntry<'static, Message>> {
    vec![
        DropdownMenuEntry::Label(Cow::Borrowed("Actions")),
        DropdownMenuEntry::Item(
            DropdownMenuItem::new("New", Some(Message::Selected("New")))
                .props(DropdownMenuItemProps::new().shortcut("Ctrl+N")),
        ),
        DropdownMenuEntry::Item(
            DropdownMenuItem::new("Share", Some(Message::Selected("Share")))
                .props(DropdownMenuItemProps::new().shortcut("Ctrl+S")),
        ),
        DropdownMenuEntry::Separator,
        DropdownMenuEntry::SubMenu(DropdownMenuSubMenu::new(
            "More",
            vec![
                DropdownMenuEntry::Item(DropdownMenuItem::new(
                    "Duplicate",
                    Some(Message::Selected("Duplicate")),
                )),
                DropdownMenuEntry::Item(
                    DropdownMenuItem::new("Delete", Some(Message::Selected("Delete")))
                        .props(DropdownMenuItemProps::new().color(AccentColor::Red)),
                ),
            ],
        )),
    ]
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
