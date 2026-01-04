use iced::border::Border;
use iced::widget::{column, container, row, text};
use iced::{Alignment, Background, Element, Length};

use iced_shadcn::{ButtonSize, ButtonVariant, Spinner, Theme, button, button_content, spinner};

pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view).run()
}

#[derive(Default)]
struct Example {
    theme: Theme,
}

#[derive(Debug, Clone)]
enum Message {
    Pressed,
}

impl Example {
    fn update(&mut self, message: Message) {
        let _ = message;
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let muted = theme.palette.muted;
        let border = theme.palette.border;
        let radius = theme.radius.md;

        let default_button = button(
            "Button",
            Some(Message::Pressed),
            ButtonVariant::Default,
            ButtonSize::Md,
            theme,
        );

        let outline_button = button(
            "Outline",
            Some(Message::Pressed),
            ButtonVariant::Outline,
            ButtonSize::Md,
            theme,
        );

        let ghost_button = button(
            "Ghost",
            Some(Message::Pressed),
            ButtonVariant::Ghost,
            ButtonSize::Md,
            theme,
        );

        let destructive_button = button(
            "Destructive",
            Some(Message::Pressed),
            ButtonVariant::Destructive,
            ButtonSize::Md,
            theme,
        );

        let secondary_button = button(
            "Secondary",
            Some(Message::Pressed),
            ButtonVariant::Secondary,
            ButtonSize::Md,
            theme,
        );

        let link_button = button(
            "Link",
            Some(Message::Pressed),
            ButtonVariant::Link,
            ButtonSize::Md,
            theme,
        );

        let icon_button = button(
            "^",
            Some(Message::Pressed),
            ButtonVariant::Outline,
            ButtonSize::Icon,
            theme,
        );

        let demo = row![
            button(
                "Button",
                Some(Message::Pressed),
                ButtonVariant::Outline,
                ButtonSize::Md,
                theme,
            ),
            button(
                "^",
                Some(Message::Pressed),
                ButtonVariant::Outline,
                ButtonSize::Icon,
                theme,
            ),
        ]
        .spacing(8)
        .align_y(Alignment::Center);

        let size_sm = row![
            button(
                "Small",
                Some(Message::Pressed),
                ButtonVariant::Outline,
                ButtonSize::Sm,
                theme,
            ),
            button(
                "^",
                Some(Message::Pressed),
                ButtonVariant::Outline,
                ButtonSize::IconSm,
                theme,
            ),
        ]
        .spacing(8);

        let size_md = row![
            button(
                "Default",
                Some(Message::Pressed),
                ButtonVariant::Outline,
                ButtonSize::Md,
                theme,
            ),
            button(
                "^",
                Some(Message::Pressed),
                ButtonVariant::Outline,
                ButtonSize::Icon,
                theme,
            ),
        ]
        .spacing(8);

        let size_lg = row![
            button(
                "Large",
                Some(Message::Pressed),
                ButtonVariant::Outline,
                ButtonSize::Lg,
                theme,
            ),
            button(
                "^",
                Some(Message::Pressed),
                ButtonVariant::Outline,
                ButtonSize::IconLg,
                theme,
            ),
        ]
        .spacing(8);

        let sizes = row![size_sm, size_md, size_lg]
            .spacing(24)
            .align_y(Alignment::Start);

        let with_icon = {
            let label = row![text("^").size(12), text("New Branch").size(12),]
                .spacing(6)
                .align_y(Alignment::Center);

            button_content(
                label,
                Some(Message::Pressed),
                ButtonVariant::Outline,
                ButtonSize::Sm,
                theme,
            )
        };

        let loading_button = {
            let spinner_widget = spinner(
                Spinner::new(theme)
                    .size(14.0)
                    .stroke_width(2.0)
                    .color(theme.palette.muted_foreground),
            );
            let label = row![
                spinner_widget,
                text("Submit")
                    .size(12)
                    .style(|_theme| iced::widget::text::Style {
                        color: Some(theme.palette.muted_foreground),
                    }),
            ]
            .spacing(6)
            .align_y(Alignment::Center);

            button_content(label, None, ButtonVariant::Outline, ButtonSize::Sm, theme)
        };

        let rounded_button = {
            let mut rounded_theme = theme.clone();
            rounded_theme.radius.sm = 999.0;
            rounded_theme.radius.md = 999.0;
            rounded_theme.radius.lg = 999.0;
            button(
                "^",
                Some(Message::Pressed),
                ButtonVariant::Outline,
                ButtonSize::Icon,
                &rounded_theme,
            )
        };

        let as_child = button(
            "Login",
            Some(Message::Pressed),
            ButtonVariant::Default,
            ButtonSize::Md,
            theme,
        );

        let content = column![
            default_button,
            outline_button,
            ghost_button,
            destructive_button,
            secondary_button,
            link_button,
            icon_button,
            sizes,
            demo,
            with_icon,
            loading_button,
            rounded_button,
            as_child,
        ]
        .spacing(20)
        .align_x(Alignment::Start);

        container(content)
            .padding(24)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(move |_theme| iced::widget::container::Style {
                background: Some(Background::Color(muted)),
                border: Border {
                    radius: radius.into(),
                    width: 1.0,
                    color: border,
                },
                ..iced::widget::container::Style::default()
            })
            .into()
    }
}
