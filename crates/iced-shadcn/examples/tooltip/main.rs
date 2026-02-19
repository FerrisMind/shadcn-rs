use iced::border::Border;
use iced::widget::{container, row, text as iced_text};
use iced::{Alignment, Background, Element, Length};

use iced_shadcn::{
    ButtonProps, ButtonSize, ButtonVariant, Theme, TooltipPosition, TooltipProps, button, tooltip,
};

pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view).run()
}

#[derive(Default)]
struct Example {
    theme: Theme,
}

impl Example {
    fn update(&mut self, _message: ()) {}

    fn view(&self) -> Element<'_, ()> {
        let theme = &self.theme;

        let content = row![
            tooltip(
                button(
                    "Singleline",
                    None,
                    ButtonProps::new()
                        .variant(ButtonVariant::Solid)
                        .size(ButtonSize::Size1),
                    theme
                ),
                iced_text("The quick brown fox").size(12),
                TooltipProps::new()
                    .position(TooltipPosition::Top)
                    .max_width(360),
                theme,
            ),
            tooltip(
                button(
                    "Multiline",
                    None,
                    ButtonProps::new()
                        .variant(ButtonVariant::Solid)
                        .size(ButtonSize::Size1),
                    theme
                ),
                iced_text("The goal of typography is to relate font size, line height, and line width in a proportional way that maximizes beauty and makes reading easier and more pleasant.")
                    .size(12),
                TooltipProps::new()
                    .position(TooltipPosition::Top)
                    .max_width(360),
                theme,
            ),
        ]
        .spacing(16)
        .align_y(Alignment::Center);

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
