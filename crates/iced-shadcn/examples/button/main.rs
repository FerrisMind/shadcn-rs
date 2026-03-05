use iced::border::Border;
use iced::time::{self, Duration};
use iced::widget::text::{Rich, Span};
use iced::widget::{column, container, row, text};
use iced::{Alignment, Background, Element, Length, Subscription, mouse};

use iced_shadcn::{
    AccentColor, ButtonProps, ButtonRadius, ButtonSize, ButtonVariant, Spinner, SpinnerSize,
    TextProps, TextSize, TextWeight, Theme, button, button_content, icon_button, spinner,
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
                self.progress += 0.02;
                if self.progress > 1.0 {
                    self.progress = 0.0;
                }
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
        let border_color = theme.palette.border;
        let radius = theme.radius.md;
        let progress = self.progress;

        // Variants section
        let variants = column![
            tile(
                theme,
                "Primary (default)",
                button(
                    "Button",
                    Some(Message::Pressed),
                    ButtonProps::new()
                        .variant(ButtonVariant::Solid)
                        .size(ButtonSize::Size2),
                    theme,
                ),
            ),
            tile(
                theme,
                "Secondary",
                button(
                    "Secondary",
                    Some(Message::Pressed),
                    ButtonProps::new()
                        .variant(ButtonVariant::Soft)
                        .size(ButtonSize::Size2),
                    theme,
                ),
            ),
            tile(
                theme,
                "Outline",
                button(
                    "Outline",
                    Some(Message::Pressed),
                    ButtonProps::new()
                        .variant(ButtonVariant::Outline)
                        .size(ButtonSize::Size2),
                    theme,
                ),
            ),
            tile(
                theme,
                "Ghost",
                button(
                    "Ghost",
                    Some(Message::Pressed),
                    ButtonProps::new()
                        .variant(ButtonVariant::Ghost)
                        .size(ButtonSize::Size2),
                    theme,
                ),
            ),
            tile(
                theme,
                "Destructive",
                button(
                    "Destructive",
                    Some(Message::Pressed),
                    ButtonProps::new()
                        .variant(ButtonVariant::Solid)
                        .size(ButtonSize::Size2)
                        .color(AccentColor::Red),
                    theme,
                ),
            ),
            tile(theme, "Link", {
                let link_label = Rich::<(), Message>::with_spans(vec![
                    Span::new("Link").underline(self.link_hovered),
                ])
                .size(14);
                let btn = button_content(
                    link_label,
                    Some(Message::Pressed),
                    ButtonProps::new()
                        .variant(ButtonVariant::Link)
                        .size(ButtonSize::Size2),
                    theme,
                );
                iced::widget::mouse_area(btn)
                    .on_enter(Message::LinkHover(true))
                    .on_exit(Message::LinkHover(false))
                    .interaction(mouse::Interaction::Pointer)
            }),
        ]
        .spacing(12);

        // Icons section
        let icons = column![
            tile(
                theme,
                "Icon only",
                icon_button(
                    icon_circle_fading_arrow_up().size(16),
                    Some(Message::Pressed),
                    ButtonProps::new()
                        .variant(ButtonVariant::Outline)
                        .size(ButtonSize::Size2),
                    theme,
                ),
            ),
            tile(
                theme,
                "With leading icon",
                button_content(
                    row![icon_git_branch().size(12), text("New Branch").size(12)]
                        .spacing(8)
                        .align_y(Alignment::Center),
                    Some(Message::Pressed),
                    ButtonProps::new()
                        .variant(ButtonVariant::Outline)
                        .size(ButtonSize::Size1),
                    theme,
                ),
            ),
            tile(
                theme,
                "Rounded full",
                icon_button(
                    icon_arrow_up().size(16),
                    Some(Message::Pressed),
                    ButtonProps::new()
                        .variant(ButtonVariant::Outline)
                        .size(ButtonSize::Size2)
                        .radius(ButtonRadius::Full),
                    theme,
                ),
            ),
        ]
        .spacing(12);

        // Sizes section
        let sizes = column![
            tile(
                theme,
                "Small",
                row![
                    button(
                        "Small",
                        Some(Message::Pressed),
                        ButtonProps::new()
                            .variant(ButtonVariant::Outline)
                            .size(ButtonSize::Size1),
                        theme,
                    ),
                    icon_button(
                        icon_arrow_up_right().size(12),
                        Some(Message::Pressed),
                        ButtonProps::new()
                            .variant(ButtonVariant::Outline)
                            .size(ButtonSize::Size1),
                        theme,
                    ),
                ]
                .spacing(8),
            ),
            tile(
                theme,
                "Default",
                row![
                    button(
                        "Default",
                        Some(Message::Pressed),
                        ButtonProps::new()
                            .variant(ButtonVariant::Outline)
                            .size(ButtonSize::Size2),
                        theme,
                    ),
                    icon_button(
                        icon_arrow_up_right().size(14),
                        Some(Message::Pressed),
                        ButtonProps::new()
                            .variant(ButtonVariant::Outline)
                            .size(ButtonSize::Size2),
                        theme,
                    ),
                ]
                .spacing(8),
            ),
            tile(
                theme,
                "Large",
                row![
                    button(
                        "Large",
                        Some(Message::Pressed),
                        ButtonProps::new()
                            .variant(ButtonVariant::Outline)
                            .size(ButtonSize::Size3),
                        theme,
                    ),
                    icon_button(
                        icon_arrow_up_right().size(16),
                        Some(Message::Pressed),
                        ButtonProps::new()
                            .variant(ButtonVariant::Outline)
                            .size(ButtonSize::Size3),
                        theme,
                    ),
                ]
                .spacing(8),
            ),
        ]
        .spacing(12);

        // States section
        let states = column![
            tile(
                theme,
                "Loading (disabled)",
                button_content(
                    row![
                        spinner(
                            Spinner::new(theme)
                                .progress(progress)
                                .size(SpinnerSize::Size1)
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
                        .size(ButtonSize::Size1),
                    theme,
                ),
            ),
            tile(
                theme,
                "Solid default",
                button(
                    "Login",
                    Some(Message::Pressed),
                    ButtonProps::new()
                        .variant(ButtonVariant::Solid)
                        .size(ButtonSize::Size2),
                    theme,
                ),
            ),
        ]
        .spacing(12);

        let content = column![
            section(theme, "Variants", variants),
            section(theme, "Icons", icons),
            section(theme, "Sizes", sizes),
            section(theme, "States", states),
        ]
        .spacing(20)
        .align_x(Alignment::Start);

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
                    color: border_color,
                },
                ..iced::widget::container::Style::default()
            })
            .into()
    }
}

fn section<'a, Message: 'a>(
    theme: &Theme,
    title: &'a str,
    content: impl Into<Element<'a, Message>>,
) -> iced::widget::Container<'a, Message> {
    let title = iced_shadcn::text(
        title,
        TextProps::new()
            .size(TextSize::Size4)
            .weight(TextWeight::Medium),
        theme,
    );
    let bg = theme.palette.card;
    let border_c = theme.palette.border;
    let r = theme.radius.md;

    container(column![title, content.into()].spacing(12))
        .padding(16)
        .width(Length::Fill)
        .style(move |_theme| iced::widget::container::Style {
            background: Some(Background::Color(bg)),
            border: Border {
                radius: r.into(),
                width: 1.0,
                color: border_c,
            },
            ..iced::widget::container::Style::default()
        })
}

fn tile<'a, Message: 'a>(
    theme: &Theme,
    label: &'a str,
    content: impl Into<Element<'a, Message>>,
) -> iced::widget::Container<'a, Message> {
    let bg = theme.palette.background;
    let border_c = theme.palette.border;
    let r = theme.radius.md;
    let muted = theme.palette.muted_foreground;

    let label_text = iced::widget::text(label)
        .size(11)
        .style(move |_theme| iced::widget::text::Style { color: Some(muted) });

    container(column![label_text, content.into()].spacing(8))
        .padding(12)
        .style(move |_theme| iced::widget::container::Style {
            background: Some(Background::Color(bg)),
            border: Border {
                radius: r.into(),
                width: 1.0,
                color: border_c,
            },
            ..iced::widget::container::Style::default()
        })
}
