use std::borrow::Cow;

use iced::border::Border;
use iced::widget::{column, container, text as iced_text};
use iced::{Background, Element, Length};

use iced_shadcn::{
    ButtonProps, ButtonVariant, DropdownMenuEntry, DropdownMenuItem, DropdownMenuItemProps,
    DropdownMenuProps, DropdownMenuSubMenu, Theme, button, dropdown_menu,
};

pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view).run()
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

        let menu = dropdown_menu(
            button(
                "Open",
                None,
                ButtonProps::new().variant(ButtonVariant::Outline),
                theme,
            ),
            entries(),
            DropdownMenuProps::new().width(224),
            theme,
        );

        let content = column![
            menu,
            iced_text(self.last_action.unwrap_or("Select an item")).size(12),
        ]
        .spacing(12);

        app(theme, preview(theme, content).into())
    }
}

fn entries() -> Vec<DropdownMenuEntry<'static, Message>> {
    vec![
        DropdownMenuEntry::Label(Cow::Borrowed("My Account")),
        DropdownMenuEntry::Item(
            DropdownMenuItem::new("Profile", Some(Message::Selected("Profile")))
                .props(DropdownMenuItemProps::new().shortcut("⇧⌘P")),
        ),
        DropdownMenuEntry::Item(
            DropdownMenuItem::new("Billing", Some(Message::Selected("Billing")))
                .props(DropdownMenuItemProps::new().shortcut("⌘B")),
        ),
        DropdownMenuEntry::Item(
            DropdownMenuItem::new("Settings", Some(Message::Selected("Settings")))
                .props(DropdownMenuItemProps::new().shortcut("⌘S")),
        ),
        DropdownMenuEntry::Item(
            DropdownMenuItem::new(
                "Keyboard shortcuts",
                Some(Message::Selected("Keyboard shortcuts")),
            )
            .props(DropdownMenuItemProps::new().shortcut("⌘K")),
        ),
        DropdownMenuEntry::Separator,
        DropdownMenuEntry::Item(DropdownMenuItem::new(
            "Team",
            Some(Message::Selected("Team")),
        )),
        DropdownMenuEntry::SubMenu(DropdownMenuSubMenu::new(
            "Invite users",
            vec![
                DropdownMenuEntry::Item(DropdownMenuItem::new(
                    "Email",
                    Some(Message::Selected("Invite via Email")),
                )),
                DropdownMenuEntry::Item(DropdownMenuItem::new(
                    "Message",
                    Some(Message::Selected("Invite via Message")),
                )),
                DropdownMenuEntry::Separator,
                DropdownMenuEntry::Item(DropdownMenuItem::new(
                    "More...",
                    Some(Message::Selected("Invite More...")),
                )),
            ],
        )),
        DropdownMenuEntry::Item(
            DropdownMenuItem::new("New Team", Some(Message::Selected("New Team")))
                .props(DropdownMenuItemProps::new().shortcut("⌘+T")),
        ),
        DropdownMenuEntry::Separator,
        DropdownMenuEntry::Item(DropdownMenuItem::new(
            "GitHub",
            Some(Message::Selected("GitHub")),
        )),
        DropdownMenuEntry::Item(DropdownMenuItem::new(
            "Support",
            Some(Message::Selected("Support")),
        )),
        DropdownMenuEntry::Item(
            DropdownMenuItem::new("API", Some(Message::Selected("API")))
                .props(DropdownMenuItemProps::new().disabled(true)),
        ),
        DropdownMenuEntry::Separator,
        DropdownMenuEntry::Item(
            DropdownMenuItem::new("Log out", Some(Message::Selected("Log out")))
                .props(DropdownMenuItemProps::new().shortcut("⇧⌘Q")),
        ),
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
