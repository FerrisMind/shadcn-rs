use iced::border::Border;
use iced::widget::{column, container, row, text as iced_text};
use iced::{Alignment, Background, Element, Length};

use iced_shadcn::{
    ButtonProps, ButtonVariant, DialogProps, InputProps, Theme, button, dialog, input, label,
};

pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view).run()
}

#[derive(Debug, Clone)]
enum Message {
    Open,
    Close,
}

#[derive(Default)]
struct Example {
    theme: Theme,
    open: bool,
}

impl Example {
    fn update(&mut self, message: Message) {
        match message {
            Message::Open => self.open = true,
            Message::Close => self.open = false,
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;

        let base_content = preview(
            theme,
            button(
                "Share",
                Some(Message::Open),
                ButtonProps::new().variant(ButtonVariant::Outline),
                theme,
            ),
        );

        let base = app(theme, base_content.into());

        let link = "https://ui.shadcn.com/docs/installation";

        let dialog_content = column![
            iced_text("Share link").size(20),
            iced_text("Anyone who has this link will be able to view this.").size(14),
            column![
                label("Link", theme),
                input(
                    link,
                    "",
                    None::<fn(String) -> Message>,
                    InputProps::new().read_only(true),
                    theme
                ),
            ]
            .spacing(6),
            row![button(
                "Close",
                Some(Message::Close),
                ButtonProps::new().variant(ButtonVariant::Soft),
                theme
            )]
            .align_y(Alignment::Center),
        ]
        .spacing(12)
        .width(Length::Fixed(360.0));

        dialog(
            base,
            self.open,
            dialog_content,
            Message::Close,
            DialogProps::new().max_width(448),
            theme,
        )
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
