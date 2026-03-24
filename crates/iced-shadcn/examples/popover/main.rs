use iced::border::Border;
use iced::widget::{column, container, text as iced_text};
use iced::{Alignment, Background, Element, Length};

use iced_shadcn::{
    ButtonProps, ButtonSize, ButtonVariant, PopoverProps, PopoverSize, Theme, button, popover,
};

pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view).run()
}

#[derive(Default)]
struct Example {
    theme: Theme,
    is_open: bool,
}

#[derive(Debug, Clone)]
enum Message {
    Toggle,
}

impl Example {
    fn update(&mut self, message: Message) {
        if matches!(message, Message::Toggle) {
            self.is_open = !self.is_open;
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;

        let trigger = button(
            "Popover",
            Some(Message::Toggle),
            ButtonProps::new().variant(ButtonVariant::Solid),
            theme,
        );

        let content = column![
            iced_text("Jan Tschichold was a German calligrapher, typographer and book designer.")
                .size(14),
            button(
                "Share",
                None,
                ButtonProps::new()
                    .variant(ButtonVariant::Solid)
                    .size(ButtonSize::Size1),
                theme,
            ),
        ]
        .spacing(12)
        .align_x(Alignment::Start);

        let content = popover(
            trigger,
            content,
            PopoverProps::new()
                .size(PopoverSize::Size2)
                .max_width(240)
                .open(Some(self.is_open)),
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
