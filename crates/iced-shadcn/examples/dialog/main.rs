use iced::border::Border;
use iced::widget::{column, container, row, text as iced_text};
use iced::{Alignment, Background, Element, Length};

use iced_shadcn::{ButtonProps, ButtonVariant, DialogProps, DialogSize, Theme, button, dialog};

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
            column![button(
                "Open",
                Some(Message::Open),
                ButtonProps::new().variant(ButtonVariant::Solid),
                theme
            )]
            .spacing(12)
            .align_x(Alignment::Start),
        );

        let base = app(theme, base_content.into());

        let dialog_content = column![
            iced_text("Share resource").size(20),
            iced_text("Jan Tschichold was a German calligrapher, typographer and book designer.")
                .size(14),
            row![
                button(
                    "Cancel",
                    Some(Message::Close),
                    ButtonProps::new().variant(ButtonVariant::Soft),
                    theme
                ),
                button(
                    "Share",
                    Some(Message::Close),
                    ButtonProps::new().variant(ButtonVariant::Solid),
                    theme
                ),
            ]
            .spacing(12)
            .align_y(Alignment::Center),
        ]
        .spacing(12);

        dialog(
            base,
            self.open,
            dialog_content,
            Message::Close,
            DialogProps::new().size(DialogSize::Size3).max_width(450),
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
