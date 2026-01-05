use iced::border::Border;
use iced::time::{self, Duration};
use iced::widget::text::{Rich, Span};
use iced::widget::{column, container, mouse_area, row, text};
use iced::{Alignment, Background, Element, Length, Subscription, mouse};

use iced_shadcn::{
    AccentColor, ButtonProps, ButtonRadius, ButtonSize, ButtonVariant, Spinner, SpinnerSize, Theme,
    button, button_content, icon_button, spinner,
};
use lucide_icons::LUCIDE_FONT_BYTES;
use lucide_icons::iced::{
    icon_arrow_up, icon_arrow_up_right, icon_circle_fading_arrow_up, icon_git_branch,
};

pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view)
        .subscription(Example::subscription)
        .font(LUCIDE_FONT_BYTES)
        .run()
}

#[derive(Default)]
struct Example {
    theme: Theme,
    progress: f32,
    link_hovered: bool,
}

#[derive(Debug, Clone)]
enum Message {
    Tick,
    Pressed,
    LinkHover(bool),
}

impl Example {
    fn update(&mut self, message: Message) {
        match message {
            Message::Tick => {
                self.progress = (self.progress + 0.02) % 1.0;
            }
            Message::Pressed => {}
            Message::LinkHover(hovered) => {
                self.link_hovered = hovered;
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        time::every(Duration::from_millis(16)).map(|_| Message::Tick)
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let background = theme.palette.background;
        let border = theme.palette.border;
        let radius = theme.radius.md;
        let progress = self.progress;

        let demo = preview(
            theme,
            row![
                button(
                    "Button",
                    Some(Message::Pressed),
                    ButtonProps::new()
                        .variant(ButtonVariant::Outline)
                        .size(ButtonSize::Two),
                    theme,
                ),
                icon_button(
                    icon_arrow_up().size(16),
                    Some(Message::Pressed),
                    ButtonProps::new()
                        .variant(ButtonVariant::Outline)
                        .size(ButtonSize::Two),
                    theme,
                ),
            ]
            .spacing(8)
            .align_y(Alignment::Center),
        );

        let default_button = preview(
            theme,
            button(
                "Button",
                Some(Message::Pressed),
                ButtonProps::new()
                    .variant(ButtonVariant::Solid)
                    .size(ButtonSize::Two),
                theme,
            ),
        );

        let secondary_button = preview(
            theme,
            button(
                "Secondary",
                Some(Message::Pressed),
                ButtonProps::new()
                    .variant(ButtonVariant::Soft)
                    .size(ButtonSize::Two),
                theme,
            ),
        );

        let destructive_button = preview(
            theme,
            button(
                "Destructive",
                Some(Message::Pressed),
                ButtonProps::new()
                    .variant(ButtonVariant::Solid)
                    .size(ButtonSize::Two)
                    .color(AccentColor::Red),
                theme,
            ),
        );

        let outline_button = preview(
            theme,
            button(
                "Outline",
                Some(Message::Pressed),
                ButtonProps::new()
                    .variant(ButtonVariant::Outline)
                    .size(ButtonSize::Two),
                theme,
            ),
        );

        let ghost_button = preview(
            theme,
            button(
                "Ghost",
                Some(Message::Pressed),
                ButtonProps::new()
                    .variant(ButtonVariant::Ghost)
                    .size(ButtonSize::Two),
                theme,
            ),
        );

        let link_label =
            Rich::<(), Message>::with_spans(vec![Span::new("Link").underline(self.link_hovered)])
                .size(14);
        let link_button = {
            let button = button_content(
                link_label,
                Some(Message::Pressed),
                ButtonProps::new()
                    .variant(ButtonVariant::Link)
                    .size(ButtonSize::Two),
                theme,
            );
            preview(
                theme,
                mouse_area(button)
                    .on_enter(Message::LinkHover(true))
                    .on_exit(Message::LinkHover(false))
                    .interaction(mouse::Interaction::Pointer),
            )
        };

        let icon_only = preview(
            theme,
            icon_button(
                icon_circle_fading_arrow_up().size(16),
                Some(Message::Pressed),
                ButtonProps::new()
                    .variant(ButtonVariant::Outline)
                    .size(ButtonSize::Two),
                theme,
            ),
        );

        let with_icon = preview(
            theme,
            button_content(
                row![icon_git_branch().size(12), text("New Branch").size(12)]
                    .spacing(8)
                    .align_y(Alignment::Center),
                Some(Message::Pressed),
                ButtonProps::new()
                    .variant(ButtonVariant::Outline)
                    .size(ButtonSize::One),
                theme,
            ),
        );

        let loading_button = preview(
            theme,
            button_content(
                row![
                    spinner(
                        Spinner::new(theme)
                            .progress(progress)
                            .size(SpinnerSize::One)
                            .color(theme.palette.muted_foreground),
                    ),
                    text("Submit")
                        .size(12)
                        .style(|_theme| iced::widget::text::Style {
                            color: Some(theme.palette.muted_foreground),
                        }),
                ]
                .spacing(8)
                .align_y(Alignment::Center),
                None,
                ButtonProps::new()
                    .variant(ButtonVariant::Outline)
                    .size(ButtonSize::One),
                theme,
            ),
        );

        let size_sm = row![
            button(
                "Small",
                Some(Message::Pressed),
                ButtonProps::new()
                    .variant(ButtonVariant::Outline)
                    .size(ButtonSize::One),
                theme,
            ),
            icon_button(
                icon_arrow_up_right().size(12),
                Some(Message::Pressed),
                ButtonProps::new()
                    .variant(ButtonVariant::Outline)
                    .size(ButtonSize::One),
                theme,
            ),
        ]
        .spacing(8);

        let size_md = row![
            button(
                "Default",
                Some(Message::Pressed),
                ButtonProps::new()
                    .variant(ButtonVariant::Outline)
                    .size(ButtonSize::Two),
                theme,
            ),
            icon_button(
                icon_arrow_up_right().size(14),
                Some(Message::Pressed),
                ButtonProps::new()
                    .variant(ButtonVariant::Outline)
                    .size(ButtonSize::Two),
                theme,
            ),
        ]
        .spacing(8);

        let size_lg = row![
            button(
                "Large",
                Some(Message::Pressed),
                ButtonProps::new()
                    .variant(ButtonVariant::Outline)
                    .size(ButtonSize::Three),
                theme,
            ),
            icon_button(
                icon_arrow_up_right().size(16),
                Some(Message::Pressed),
                ButtonProps::new()
                    .variant(ButtonVariant::Outline)
                    .size(ButtonSize::Three),
                theme,
            ),
        ]
        .spacing(8);

        let sizes = preview(
            theme,
            row![size_sm, size_md, size_lg]
                .spacing(32)
                .align_y(Alignment::Start),
        );

        let rounded = preview(
            theme,
            icon_button(
                icon_arrow_up().size(16),
                Some(Message::Pressed),
                ButtonProps::new()
                    .variant(ButtonVariant::Outline)
                    .size(ButtonSize::Two)
                    .radius(ButtonRadius::Full),
                theme,
            ),
        );

        let as_child = preview(
            theme,
            button(
                "Login",
                Some(Message::Pressed),
                ButtonProps::new()
                    .variant(ButtonVariant::Solid)
                    .size(ButtonSize::Two),
                theme,
            ),
        );

        let content = column![
            row![demo, default_button]
                .spacing(20)
                .align_y(Alignment::Center),
            row![secondary_button, destructive_button]
                .spacing(20)
                .align_y(Alignment::Center),
            row![outline_button, ghost_button]
                .spacing(20)
                .align_y(Alignment::Center),
            row![link_button, icon_only]
                .spacing(20)
                .align_y(Alignment::Center),
            row![with_icon, loading_button]
                .spacing(20)
                .align_y(Alignment::Center),
            row![sizes, rounded].spacing(20).align_y(Alignment::Center),
            row![as_child].spacing(20).align_y(Alignment::Center),
        ]
        .spacing(20)
        .align_x(Alignment::Center);

        container(content)
            .padding(24)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .style(move |_theme| iced::widget::container::Style {
                background: Some(Background::Color(background)),
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

fn preview<'a, Message: 'a>(
    theme: &Theme,
    content: impl Into<Element<'a, Message>>,
) -> iced::widget::Container<'a, Message> {
    let background = theme.palette.card;
    let border = theme.palette.border;
    let radius = theme.radius.md;

    container(content)
        .padding(20)
        .width(Length::Fill)
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
