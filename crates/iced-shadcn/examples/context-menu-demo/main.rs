use std::borrow::Cow;

use iced::border::Border;
use iced::widget::{container, text as iced_text};
use iced::{Alignment, Background, Element, Length};

use iced_shadcn::{
    AccentColor, ContextMenuCheckboxItem, ContextMenuEntry, ContextMenuItem, ContextMenuItemProps,
    ContextMenuProps, ContextMenuRadioItem, ContextMenuSubMenu, Theme, context_menu,
};
use lucide_icons::LUCIDE_FONT_BYTES;

pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view)
        .font(LUCIDE_FONT_BYTES)
        .run()
}

#[derive(Debug, Clone)]
enum Message {
    ToggleBookmarks,
    ToggleUrls,
    SelectPerson(&'static str),
    Selected(&'static str),
}

struct Example {
    theme: Theme,
    show_bookmarks: bool,
    show_full_urls: bool,
    person: &'static str,
    last_action: Option<&'static str>,
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::dark(),
            show_bookmarks: true,
            show_full_urls: false,
            person: "pedro",
            last_action: None,
        }
    }
}

impl Example {
    fn update(&mut self, message: Message) {
        match message {
            Message::ToggleBookmarks => self.show_bookmarks = !self.show_bookmarks,
            Message::ToggleUrls => self.show_full_urls = !self.show_full_urls,
            Message::SelectPerson(value) => self.person = value,
            Message::Selected(value) => self.last_action = Some(value),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let background = theme.palette.background;
        let border_color = theme.palette.border;
        let radius = theme.radius.md;

        let trigger = container(iced_text("Right click here").size(14))
            .width(Length::Fixed(300.0))
            .height(Length::Fixed(150.0))
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .style(move |_t| iced::widget::container::Style {
                background: Some(Background::Color(background)),
                border: Border {
                    color: border_color,
                    width: 1.0,
                    radius: radius.into(),
                },
                ..iced::widget::container::Style::default()
            });

        let entries = vec![
            ContextMenuEntry::Item(
                ContextMenuItem::new("Back", Some(Message::Selected("Back")))
                    .props(ContextMenuItemProps::new().inset(true).shortcut("⌘[")),
            ),
            ContextMenuEntry::Item(
                ContextMenuItem::new("Forward", Some(Message::Selected("Forward"))).props(
                    ContextMenuItemProps::new()
                        .inset(true)
                        .shortcut("⌘]")
                        .disabled(true),
                ),
            ),
            ContextMenuEntry::Item(
                ContextMenuItem::new("Reload", Some(Message::Selected("Reload")))
                    .props(ContextMenuItemProps::new().inset(true).shortcut("⌘R")),
            ),
            ContextMenuEntry::SubMenu(
                ContextMenuSubMenu::new(
                    "More Tools",
                    vec![
                        ContextMenuEntry::Item(ContextMenuItem::new(
                            "Save Page...",
                            Some(Message::Selected("Save Page...")),
                        )),
                        ContextMenuEntry::Item(ContextMenuItem::new(
                            "Create Shortcut...",
                            Some(Message::Selected("Create Shortcut...")),
                        )),
                        ContextMenuEntry::Item(ContextMenuItem::new(
                            "Name Window...",
                            Some(Message::Selected("Name Window...")),
                        )),
                        ContextMenuEntry::Separator,
                        ContextMenuEntry::Item(ContextMenuItem::new(
                            "Developer Tools",
                            Some(Message::Selected("Developer Tools")),
                        )),
                        ContextMenuEntry::Separator,
                        ContextMenuEntry::Item(
                            ContextMenuItem::new("Delete", Some(Message::Selected("Delete")))
                                .props(ContextMenuItemProps::new().color(AccentColor::Red)),
                        ),
                    ],
                )
                .props(ContextMenuItemProps::new().inset(true)),
            ),
            ContextMenuEntry::Separator,
            ContextMenuEntry::CheckboxItem(ContextMenuCheckboxItem::new(
                "Show Bookmarks",
                self.show_bookmarks,
                Some(Message::ToggleBookmarks),
            )),
            ContextMenuEntry::CheckboxItem(ContextMenuCheckboxItem::new(
                "Show Full URLs",
                self.show_full_urls,
                Some(Message::ToggleUrls),
            )),
            ContextMenuEntry::Separator,
            ContextMenuEntry::Label(Cow::Borrowed("People")),
            ContextMenuEntry::RadioItem(ContextMenuRadioItem::new(
                "Pedro Duarte",
                self.person == "pedro",
                Some(Message::SelectPerson("pedro")),
            )),
            ContextMenuEntry::RadioItem(ContextMenuRadioItem::new(
                "Colm Tuite",
                self.person == "colm",
                Some(Message::SelectPerson("colm")),
            )),
        ];

        let content = context_menu(trigger, entries, ContextMenuProps::new().width(208), theme);

        let content = iced::widget::column![
            content,
            iced_text(self.last_action.unwrap_or("Select an item")).size(12),
        ]
        .spacing(12);

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
}
