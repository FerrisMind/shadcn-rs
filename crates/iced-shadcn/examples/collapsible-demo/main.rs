use iced::border::Border;
use iced::widget::{column, container, text as iced_text};
use iced::{Background, Element, Length};

use iced_shadcn::{CollapsibleContentProps, CollapsibleProps, Theme, collapsible};

pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view).run()
}

#[derive(Default)]
struct Example {
    theme: Theme,
    open: bool,
}

#[derive(Debug, Clone)]
enum Message {
    OpenChanged(bool),
}

impl Example {
    fn update(&mut self, message: Message) {
        match message {
            Message::OpenChanged(open) => self.open = open,
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;

        let content = collapsible(
            self.open,
            iced_text("Toggle details").size(14),
            column![
                iced_text("This is the collapsible content.").size(12),
                iced_text("It is shown when open.").size(12),
            ]
            .spacing(6),
            Some(Message::OpenChanged),
            CollapsibleContentProps::new(),
            CollapsibleProps::new(),
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
        .width(Length::Fixed(420.0))
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
