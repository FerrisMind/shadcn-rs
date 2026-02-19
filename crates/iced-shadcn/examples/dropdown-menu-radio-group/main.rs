use std::borrow::Cow;

use iced::border::Border;
use iced::widget::container;
use iced::{Background, Element, Length};

use iced_shadcn::{
    ButtonProps, ButtonVariant, DropdownMenuEntry, DropdownMenuItemProps, DropdownMenuProps,
    DropdownMenuRadioItem, Theme, button, dropdown_menu,
};

pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view).run()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum Position {
    Top,
    #[default]
    Bottom,
    Right,
}

#[derive(Debug, Clone)]
enum Message {
    SetPosition(Position),
}

#[derive(Default)]
struct Example {
    theme: Theme,
    position: Position,
}

impl Example {
    fn update(&mut self, message: Message) {
        match message {
            Message::SetPosition(value) => self.position = value,
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;

        let entries = vec![
            DropdownMenuEntry::Label(Cow::Borrowed("Panel Position")),
            DropdownMenuEntry::Separator,
            DropdownMenuEntry::RadioItem(DropdownMenuRadioItem::new(
                "Top",
                self.position == Position::Top,
                Some(Message::SetPosition(Position::Top)),
            )),
            DropdownMenuEntry::RadioItem(DropdownMenuRadioItem::new(
                "Bottom",
                self.position == Position::Bottom,
                Some(Message::SetPosition(Position::Bottom)),
            )),
            DropdownMenuEntry::RadioItem(DropdownMenuRadioItem::new(
                "Right",
                self.position == Position::Right,
                Some(Message::SetPosition(Position::Right)),
            )),
            DropdownMenuEntry::Separator,
            DropdownMenuEntry::Item(iced_shadcn::DropdownMenuItem::new("Selected", None).props(
                DropdownMenuItemProps::new().shortcut(match self.position {
                    Position::Top => "top",
                    Position::Bottom => "bottom",
                    Position::Right => "right",
                }),
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
