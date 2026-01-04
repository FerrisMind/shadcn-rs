use iced::border::Border;
use iced::time::{self, Duration};
use iced::widget::{column, container, row, space, text};
use iced::{Alignment, Background, Color, Element, Length, Subscription};

use iced_shadcn::{
    ButtonSize, ButtonVariant, Spinner, TextVariant, Theme, button_content, spinner,
    text as shadcn_text,
};

pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view)
        .subscription(Example::subscription)
        .run()
}

#[derive(Default)]
struct Example {
    theme: Theme,
    progress: f32,
}

#[derive(Debug, Clone)]
enum Message {
    Tick,
}

impl Example {
    fn update(&mut self, message: Message) {
        match message {
            Message::Tick => {
                self.progress = (self.progress + 0.02) % 1.0;
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        time::every(Duration::from_millis(16)).map(|_| Message::Tick)
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let muted = theme.palette.muted;
        let border = theme.palette.border;
        let radius = theme.radius.md;
        let progress = self.progress;

        let make_spinner = |size: f32, color: Color| {
            spinner(
                Spinner::new(theme)
                    .progress(progress)
                    .size(size)
                    .color(color),
            )
        };

        let basic = column![make_spinner(24.0, theme.palette.primary)]
            .spacing(8)
            .align_x(Alignment::Center);

        let sizes = row![
            make_spinner(12.0, theme.palette.primary),
            make_spinner(16.0, theme.palette.primary),
            make_spinner(24.0, theme.palette.primary),
            make_spinner(32.0, theme.palette.primary),
        ]
        .spacing(24)
        .align_y(Alignment::Center);

        let colors = row![
            make_spinner(24.0, Color::from_rgb8(0xEF, 0x44, 0x44)),
            make_spinner(24.0, Color::from_rgb8(0x22, 0xC5, 0x5E)),
            make_spinner(24.0, Color::from_rgb8(0x3B, 0x82, 0xF6)),
            make_spinner(24.0, Color::from_rgb8(0xE5, 0xAB, 0x08)),
            make_spinner(24.0, Color::from_rgb8(0xA8, 0x55, 0xF7)),
        ]
        .spacing(24)
        .align_y(Alignment::Center);

        let spinner_button = column![
            button_content(
                spinner_label(theme, progress, "Loading..."),
                None,
                ButtonVariant::Default,
                ButtonSize::Sm,
                theme,
            ),
            button_content(
                spinner_label(theme, progress, "Please wait"),
                None,
                ButtonVariant::Outline,
                ButtonSize::Sm,
                theme,
            ),
            button_content(
                spinner_label(theme, progress, "Processing"),
                None,
                ButtonVariant::Secondary,
                ButtonSize::Sm,
                theme,
            ),
        ]
        .spacing(8)
        .align_x(Alignment::Center);

        let demo = {
            let amount = text("$100.00").size(14);
            let content = row![
                make_spinner(18.0, theme.palette.primary),
                shadcn_text("Processing payment...", TextVariant::Small, theme),
                space().width(Length::Fill),
                amount,
            ]
            .spacing(12)
            .align_y(Alignment::Center);

            container(content)
                .padding(16)
                .width(Length::Fixed(360.0))
                .style(move |_theme| iced::widget::container::Style {
                    background: Some(Background::Color(theme.palette.muted)),
                    border: Border {
                        radius: theme.radius.lg.into(),
                        width: 1.0,
                        color: theme.palette.border,
                    },
                    ..iced::widget::container::Style::default()
                })
        };

        let empty = {
            let content = column![
                make_spinner(20.0, theme.palette.primary),
                shadcn_text("Processing your request", TextVariant::Large, theme),
                shadcn_text(
                    "Please wait while we process your request. Do not refresh the page.",
                    TextVariant::Muted,
                    theme,
                )
                .width(Length::Fill),
                button_content(
                    text("Cancel").size(12),
                    Some(Message::Tick),
                    ButtonVariant::Outline,
                    ButtonSize::Sm,
                    theme,
                ),
            ]
            .spacing(8)
            .align_x(Alignment::Center);

            container(content)
                .padding(20)
                .width(Length::Fixed(360.0))
                .style(move |_theme| iced::widget::container::Style {
                    border: Border {
                        radius: theme.radius.lg.into(),
                        width: 1.0,
                        color: theme.palette.border,
                    },
                    ..iced::widget::container::Style::default()
                })
        };

        let item = {
            let header = row![
                make_spinner(18.0, theme.palette.primary),
                column![
                    shadcn_text("Downloading...", TextVariant::Small, theme),
                    shadcn_text("129 MB / 1000 MB", TextVariant::Muted, theme),
                ]
                .spacing(4),
                space().width(Length::Fill),
                button_content(
                    text("Cancel").size(12),
                    Some(Message::Tick),
                    ButtonVariant::Outline,
                    ButtonSize::Sm,
                    theme,
                ),
            ]
            .spacing(12)
            .align_y(Alignment::Center);

            let progress_bar = {
                let fill = container(space().height(Length::Fixed(6.0)))
                    .width(Length::Fixed(240.0))
                    .style(move |_theme| iced::widget::container::Style {
                        background: Some(Background::Color(theme.palette.primary)),
                        border: Border {
                            radius: theme.radius.sm.into(),
                            width: 0.0,
                            color: theme.palette.border,
                        },
                        ..iced::widget::container::Style::default()
                    });

                let track_content = row![fill, space().width(Length::Fill)]
                    .spacing(0)
                    .height(Length::Fixed(6.0));

                container(track_content)
                    .width(Length::Fixed(320.0))
                    .style(move |_theme| iced::widget::container::Style {
                        background: Some(Background::Color(theme.palette.muted)),
                        border: Border {
                            radius: theme.radius.sm.into(),
                            width: 0.0,
                            color: theme.palette.border,
                        },
                        ..iced::widget::container::Style::default()
                    })
            };

            container(column![header, progress_bar].spacing(12))
                .padding(16)
                .width(Length::Fixed(360.0))
                .style(move |_theme| iced::widget::container::Style {
                    border: Border {
                        radius: theme.radius.lg.into(),
                        width: 1.0,
                        color: theme.palette.border,
                    },
                    ..iced::widget::container::Style::default()
                })
        };

        let badges = row![
            spinner_badge(
                theme,
                progress,
                "Syncing",
                Some(theme.palette.foreground),
                false,
                theme.palette.primary_foreground,
            ),
            spinner_badge(
                theme,
                progress,
                "Updating",
                Some(theme.palette.secondary),
                false,
                theme.palette.secondary_foreground,
            ),
            spinner_badge(
                theme,
                progress,
                "Processing",
                None,
                true,
                theme.palette.foreground,
            ),
        ]
        .spacing(12)
        .align_y(Alignment::Center);

        let custom = row![make_spinner(16.0, theme.palette.primary)]
            .spacing(8)
            .align_y(Alignment::Center);

        let content = column![
            basic,
            sizes,
            colors,
            spinner_button,
            demo,
            empty,
            item,
            badges,
            custom,
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

fn spinner_label<'a, Message: 'a>(
    theme: &'a Theme,
    progress: f32,
    text_label: &'a str,
) -> iced::widget::Row<'a, Message> {
    row![
        spinner(
            Spinner::new(theme)
                .progress(progress)
                .size(14.0)
                .color(theme.palette.muted_foreground),
        ),
        text(text_label)
            .size(12)
            .style(|_theme| iced::widget::text::Style {
                color: Some(theme.palette.muted_foreground),
            }),
    ]
    .spacing(6)
    .align_y(Alignment::Center)
}

fn spinner_badge<'a, Message: 'a>(
    theme: &'a Theme,
    progress: f32,
    label_text: &'a str,
    background: Option<Color>,
    outlined: bool,
    text_color: Color,
) -> iced::widget::Container<'a, Message> {
    let content = row![
        spinner(
            Spinner::new(theme)
                .progress(progress)
                .size(12.0)
                .color(text_color),
        ),
        text(label_text)
            .size(12)
            .style(move |_theme| iced::widget::text::Style {
                color: Some(text_color),
            }),
    ]
    .spacing(6)
    .align_y(Alignment::Center);

    let bg = background.map(Background::Color);
    container(content)
        .padding([4.0, 8.0])
        .style(move |_theme| iced::widget::container::Style {
            background: bg,
            border: Border {
                radius: theme.radius.lg.into(),
                width: if outlined { 1.0 } else { 0.0 },
                color: theme.palette.border,
            },
            ..iced::widget::container::Style::default()
        })
}
