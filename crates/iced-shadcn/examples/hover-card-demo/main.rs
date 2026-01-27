use iced::border::Border;
use iced::widget::{column, container, row, text as iced_text};
use iced::{Alignment, Background, Element, Length};

use iced_shadcn::{ButtonProps, ButtonVariant, HoverCardProps, Theme, button, hover_card};

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

        let avatar_bg = theme.palette.muted;
        let avatar_fg = theme.palette.muted_foreground;
        let avatar_radius = 999.0;

        let avatar = container(iced_text("VC").size(14))
            .width(Length::Fixed(44.0))
            .height(Length::Fixed(44.0))
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .style(move |_t| iced::widget::container::Style {
                background: Some(Background::Color(avatar_bg)),
                text_color: Some(avatar_fg),
                border: Border {
                    radius: avatar_radius.into(),
                    width: 1.0,
                    color: avatar_bg,
                },
                ..iced::widget::container::Style::default()
            });

        let card = row![
            avatar,
            column![
                iced_text("@nextjs").size(14),
                iced_text("The React Framework – created and maintained by @vercel.").size(13),
                iced_text("Joined December 2021").size(12),
            ]
            .spacing(4)
        ]
        .spacing(12)
        .align_y(Alignment::Start)
        .width(Length::Fixed(320.0));

        let content = hover_card(
            button(
                "@nextjs",
                None,
                ButtonProps::new().variant(ButtonVariant::Link),
                theme,
            ),
            card,
            HoverCardProps::new().max_width(320),
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
