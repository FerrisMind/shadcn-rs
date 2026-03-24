use iced::border::Border;
use iced::widget::{container, text as iced_text};
use iced::{Background, Element, Length};

use iced_shadcn::{
    AccordionItemProps, AccordionProps, AccordionState, AccordionType, Theme, accordion,
};

pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view).run()
}

#[derive(Default)]
struct Example {
    theme: Theme,
    state: AccordionState,
}

#[derive(Debug, Clone)]
enum Message {
    AccordionChanged(AccordionState),
}

impl Example {
    fn update(&mut self, message: Message) {
        match message {
            Message::AccordionChanged(state) => self.state = state,
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;

        let items = vec![
            AccordionItemProps::new(
                "item-1",
                "Is it accessible?",
                iced_text("Yes. It adheres to the WAI-ARIA design pattern."),
            ),
            AccordionItemProps::new(
                "item-2",
                "Is it styled?",
                iced_text("Yes. It comes with default styles that match the rest of the kit."),
            ),
            AccordionItemProps::new(
                "item-3",
                "Is it animated?",
                iced_text("This iced version is a minimal implementation."),
            ),
        ];

        let widget = accordion(
            items,
            self.state.clone(),
            Some(Message::AccordionChanged),
            AccordionProps::new()
                .accordion_type(AccordionType::Single)
                .collapsible(true),
            theme,
        );

        app(theme, preview(theme, widget).into())
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
        .width(Length::Fixed(560.0))
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
