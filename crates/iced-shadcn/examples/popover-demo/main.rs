use iced::border::Border;
use iced::widget::{column, container, row, text as iced_text};
use iced::{Alignment, Background, Element, Length};

use iced_shadcn::{
    ButtonProps, ButtonVariant, InputProps, PopoverProps, PopoverSize, Theme, button, input, label,
    popover,
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

        let trigger = button(
            "Open popover",
            None,
            ButtonProps::new().variant(ButtonVariant::Outline),
            theme,
        );

        let field_props = InputProps::new().read_only(true);

        let popover_content = column![
            column![
                iced_text("Dimensions").size(16),
                iced_text("Set the dimensions for the layer.").size(12),
            ]
            .spacing(4),
            column![
                row![
                    label("Width", theme),
                    input("100%", "", None::<fn(String) -> ()>, field_props, theme)
                        .width(Length::Fixed(160.0)),
                ]
                .spacing(12)
                .align_y(Alignment::Center),
                row![
                    label("Max. width", theme),
                    input("300px", "", None::<fn(String) -> ()>, field_props, theme)
                        .width(Length::Fixed(160.0)),
                ]
                .spacing(12)
                .align_y(Alignment::Center),
                row![
                    label("Height", theme),
                    input("25px", "", None::<fn(String) -> ()>, field_props, theme)
                        .width(Length::Fixed(160.0)),
                ]
                .spacing(12)
                .align_y(Alignment::Center),
                row![
                    label("Max. height", theme),
                    input("none", "", None::<fn(String) -> ()>, field_props, theme)
                        .width(Length::Fixed(160.0)),
                ]
                .spacing(12)
                .align_y(Alignment::Center),
            ]
            .spacing(10),
        ]
        .spacing(12)
        .width(Length::Fixed(320.0));

        let content = popover(
            trigger,
            popover_content,
            PopoverProps::new().size(PopoverSize::Size2).max_width(320),
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
