use std::borrow::Cow;

use iced::border::Border;
use iced::widget::{column, container, row, text as iced_text};
use iced::{Alignment, Background, Element, Length};

use iced_shadcn::{
    AccentColor, ContextMenuCheckboxItem, ContextMenuContentSize, ContextMenuContentVariant,
    ContextMenuEntry, ContextMenuItem, ContextMenuItemProps, ContextMenuProps,
    ContextMenuRadioItem, ContextMenuSubMenu, Theme, context_menu,
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
    Selected,
}

struct Example {
    theme: Theme,
    show_bookmarks: bool,
    show_full_urls: bool,
    person: &'static str,
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::dark(),
            show_bookmarks: true,
            show_full_urls: false,
            person: "pedro",
        }
    }
}

impl Example {
    fn update(&mut self, message: Message) {
        match message {
            Message::ToggleBookmarks => self.show_bookmarks = !self.show_bookmarks,
            Message::ToggleUrls => self.show_full_urls = !self.show_full_urls,
            Message::SelectPerson(value) => self.person = value,
            Message::Selected => {}
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

        let content = context_menu(
            trigger,
            entries(self),
            ContextMenuProps::new()
                .size(ContextMenuContentSize::Size2)
                .variant(ContextMenuContentVariant::Solid)
                .width(208),
            theme,
        );

        let content = column![row![content].align_y(Alignment::Center)].spacing(12);

        app(theme, content.into())
    }
}

fn entries(example: &Example) -> Vec<ContextMenuEntry<'static, Message>> {
    vec![
        ContextMenuEntry::Item(
            ContextMenuItem::new("Back", Some(Message::Selected))
                .props(ContextMenuItemProps::new().shortcut("Ctrl+[")),
        ),
        ContextMenuEntry::Item(
            ContextMenuItem::new("Forward", Some(Message::Selected)).props(
                ContextMenuItemProps::new()
                    .shortcut("Ctrl+]")
                    .disabled(true),
            ),
        ),
        ContextMenuEntry::Item(
            ContextMenuItem::new("Reload", Some(Message::Selected))
                .props(ContextMenuItemProps::new().shortcut("Ctrl+R")),
        ),
        ContextMenuEntry::SubMenu(
            ContextMenuSubMenu::new(
                "More Tools",
                vec![
                    ContextMenuEntry::Item(ContextMenuItem::new(
                        "Save Page...",
                        Some(Message::Selected),
                    )),
                    ContextMenuEntry::Item(ContextMenuItem::new(
                        "Create Shortcut...",
                        Some(Message::Selected),
                    )),
                    ContextMenuEntry::Item(ContextMenuItem::new(
                        "Name Window...",
                        Some(Message::Selected),
                    )),
                    ContextMenuEntry::Separator,
                    ContextMenuEntry::Item(ContextMenuItem::new(
                        "Developer Tools",
                        Some(Message::Selected),
                    )),
                    ContextMenuEntry::Separator,
                    ContextMenuEntry::Item(
                        ContextMenuItem::new("Delete", Some(Message::Selected))
                            .props(ContextMenuItemProps::new().color(AccentColor::Red)),
                    ),
                ],
            )
            .props(ContextMenuItemProps::new()),
        ),
        ContextMenuEntry::Separator,
        ContextMenuEntry::CheckboxItem(ContextMenuCheckboxItem::new(
            "Show Bookmarks",
            example.show_bookmarks,
            Some(Message::ToggleBookmarks),
        )),
        ContextMenuEntry::CheckboxItem(ContextMenuCheckboxItem::new(
            "Show Full URLs",
            example.show_full_urls,
            Some(Message::ToggleUrls),
        )),
        ContextMenuEntry::Separator,
        ContextMenuEntry::Label(Cow::Borrowed("People")),
        ContextMenuEntry::RadioItem(ContextMenuRadioItem::new(
            "Pedro Duarte",
            example.person == "pedro",
            Some(Message::SelectPerson("pedro")),
        )),
        ContextMenuEntry::RadioItem(ContextMenuRadioItem::new(
            "Colm Tuite",
            example.person == "colm",
            Some(Message::SelectPerson("colm")),
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
