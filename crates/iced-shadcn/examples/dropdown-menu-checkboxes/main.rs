use std::borrow::Cow;

use iced::border::Border;
use iced::widget::container;
use iced::{Background, Element, Length};

use iced_shadcn::{
    ButtonProps, ButtonVariant, DropdownMenuCheckboxItem, DropdownMenuEntry, DropdownMenuItemProps,
    DropdownMenuProps, Theme, button, dropdown_menu,
};

pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view).run()
}

#[derive(Debug, Clone)]
enum Message {
    StatusBar,
    ActivityBar,
    Panel,
}

#[derive(Default)]
struct Example {
    theme: Theme,
    show_status_bar: bool,
    show_activity_bar: bool,
    show_panel: bool,
}

impl Example {
    fn update(&mut self, message: Message) {
        match message {
            Message::StatusBar => self.show_status_bar = !self.show_status_bar,
            Message::ActivityBar => self.show_activity_bar = !self.show_activity_bar,
            Message::Panel => self.show_panel = !self.show_panel,
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;

        let entries = vec![
            DropdownMenuEntry::Label(Cow::Borrowed("Appearance")),
            DropdownMenuEntry::Separator,
            DropdownMenuEntry::CheckboxItem(DropdownMenuCheckboxItem::new(
                "Status Bar",
                self.show_status_bar,
                Some(Message::StatusBar),
            )),
            DropdownMenuEntry::CheckboxItem(
                DropdownMenuCheckboxItem::new(
                    "Activity Bar",
                    self.show_activity_bar,
                    Some(Message::ActivityBar),
                )
                .props(DropdownMenuItemProps::new().disabled(true)),
            ),
            DropdownMenuEntry::CheckboxItem(DropdownMenuCheckboxItem::new(
                "Panel",
                self.show_panel,
                Some(Message::Panel),
            )),
        ];

        let content = dropdown_menu(
            button(
                "Open",
                None,
                ButtonProps::new().variant(ButtonVariant::Outline),
                theme,
            ),
            entries,
            DropdownMenuProps::new().width(224),
            theme,
        );

        app(theme, preview(theme, content).into())
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
