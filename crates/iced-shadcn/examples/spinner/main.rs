use iced::border::Border;
use iced::time::{self, Duration};
use iced::widget::{column, container, row, space, text};
use iced::{Alignment, Background, Color, Element, Length, Subscription};

use iced_shadcn::{
    ButtonProps, ButtonSize, ButtonVariant, Spinner, SpinnerSize, TextFieldProps, TextFieldSize,
    TextFieldVariant, Theme, button_content, spinner, text_field,
};
use lucide_icons::LUCIDE_FONT_BYTES;
use lucide_icons::iced::icon_arrow_up;

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
}

#[derive(Debug, Clone)]
enum Message {
    Tick,
    Pressed,
}

impl Example {
    fn update(&mut self, message: Message) {
        match message {
            Message::Tick => {
                self.progress = (self.progress + 0.02) % 1.0;
            }
            Message::Pressed => {}
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

        let make_spinner = |size: SpinnerSize, color: Color| {
            spinner(
                Spinner::new(theme)
                    .progress(progress)
                    .size(size)
                    .color(color),
            )
        };

        let basic = preview(
            theme,
            column![make_spinner(SpinnerSize::Two, theme.palette.primary)]
                .spacing(8)
                .align_x(Alignment::Center)
                .width(Length::Fill),
        );

        let sizes = preview(
            theme,
            row![
                make_spinner(SpinnerSize::One, theme.palette.primary),
                make_spinner(SpinnerSize::Two, theme.palette.primary),
                make_spinner(SpinnerSize::Custom(24.0), theme.palette.primary),
                make_spinner(SpinnerSize::Custom(32.0), theme.palette.primary),
            ]
            .spacing(24)
            .align_y(Alignment::Center),
        );

        let colors = preview(
            theme,
            row![
                make_spinner(
                    SpinnerSize::Custom(24.0),
                    Color::from_rgb8(0xEF, 0x44, 0x44)
                ),
                make_spinner(
                    SpinnerSize::Custom(24.0),
                    Color::from_rgb8(0x22, 0xC5, 0x5E)
                ),
                make_spinner(
                    SpinnerSize::Custom(24.0),
                    Color::from_rgb8(0x3B, 0x82, 0xF6)
                ),
                make_spinner(
                    SpinnerSize::Custom(24.0),
                    Color::from_rgb8(0xEA, 0xB3, 0x08)
                ),
                make_spinner(
                    SpinnerSize::Custom(24.0),
                    Color::from_rgb8(0xA8, 0x55, 0xF7)
                ),
            ]
            .spacing(24)
            .align_y(Alignment::Center),
        );

        let spinner_button = preview(
            theme,
            column![
                button_content(
                    spinner_label(theme, progress, "Loading..."),
                    None,
                    ButtonProps::new()
                        .variant(ButtonVariant::Solid)
                        .size(ButtonSize::One),
                    theme,
                ),
                button_content(
                    spinner_label(theme, progress, "Please wait"),
                    None,
                    ButtonProps::new()
                        .variant(ButtonVariant::Outline)
                        .size(ButtonSize::One),
                    theme,
                ),
                button_content(
                    spinner_label(theme, progress, "Processing"),
                    None,
                    ButtonProps::new()
                        .variant(ButtonVariant::Soft)
                        .size(ButtonSize::One),
                    theme,
                ),
            ]
            .spacing(16)
            .align_x(Alignment::Center),
        );

        let demo = preview(theme, spinner_demo(theme, progress));
        let empty = preview(theme, spinner_empty(theme, progress));
        let item = preview(theme, spinner_item(theme, progress));

        let badges = preview(
            theme,
            row![
                spinner_badge(
                    theme,
                    progress,
                    "Syncing",
                    BadgeStyle::Solid(theme.palette.primary, theme.palette.primary_foreground),
                ),
                spinner_badge(
                    theme,
                    progress,
                    "Updating",
                    BadgeStyle::Solid(theme.palette.secondary, theme.palette.secondary_foreground),
                ),
                spinner_badge(
                    theme,
                    progress,
                    "Processing",
                    BadgeStyle::Outline(theme.palette.foreground),
                ),
            ]
            .spacing(16)
            .align_y(Alignment::Center),
        );

        let input_group = preview(theme, spinner_input_group(theme, progress));

        let custom = preview(
            theme,
            row![make_spinner(
                SpinnerSize::Custom(16.0),
                theme.palette.foreground
            )]
            .spacing(16)
            .align_y(Alignment::Center),
        );

        let content = column![
            row![basic, sizes].spacing(20).align_y(Alignment::Center),
            row![colors, spinner_button]
                .spacing(20)
                .align_y(Alignment::Center),
            row![demo, empty].spacing(20).align_y(Alignment::Center),
            row![item, badges].spacing(20).align_y(Alignment::Center),
            row![input_group, custom]
                .spacing(20)
                .align_y(Alignment::Center),
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

fn spinner_label<'a>(
    theme: &'a Theme,
    progress: f32,
    text_label: &'a str,
) -> iced::widget::Row<'a, Message> {
    row![
        spinner(
            Spinner::new(theme)
                .progress(progress)
                .size(SpinnerSize::One)
                .color(theme.palette.muted_foreground),
        ),
        text(text_label)
            .size(12)
            .style(|_theme| iced::widget::text::Style {
                color: Some(theme.palette.muted_foreground),
            }),
    ]
    .spacing(8)
    .align_y(Alignment::Center)
}

fn spinner_demo<'a>(theme: &'a Theme, progress: f32) -> iced::widget::Container<'a, Message> {
    let content = row![
        spinner(
            Spinner::new(theme)
                .progress(progress)
                .size(SpinnerSize::Custom(18.0))
                .color(theme.palette.primary),
        ),
        small_text(theme, "Processing payment..."),
        space().width(Length::Fill),
        text("$100.00").size(14),
    ]
    .spacing(12)
    .align_y(Alignment::Center);

    container(content)
        .padding(16)
        .width(Length::Fixed(320.0))
        .style(move |_theme| iced::widget::container::Style {
            background: Some(Background::Color(theme.palette.muted)),
            border: Border {
                radius: 16.0.into(),
                width: 1.0,
                color: theme.palette.border,
            },
            ..iced::widget::container::Style::default()
        })
}

fn spinner_empty<'a>(theme: &'a Theme, progress: f32) -> iced::widget::Container<'a, Message> {
    let content = column![
        spinner(
            Spinner::new(theme)
                .progress(progress)
                .size(SpinnerSize::Custom(20.0))
                .color(theme.palette.primary),
        ),
        large_text(theme, "Processing your request"),
        muted_text(
            theme,
            "Please wait while we process your request. Do not refresh the page.",
        )
        .width(Length::Fill),
        button_content(
            text("Cancel").size(12),
            Some(Message::Pressed),
            ButtonProps::new()
                .variant(ButtonVariant::Outline)
                .size(ButtonSize::One),
            theme,
        ),
    ]
    .spacing(8)
    .align_x(Alignment::Center);

    container(content)
        .padding(20)
        .width(Length::Fill)
        .style(move |_theme| iced::widget::container::Style {
            border: Border {
                radius: theme.radius.lg.into(),
                width: 1.0,
                color: theme.palette.border,
            },
            ..iced::widget::container::Style::default()
        })
}

fn spinner_item<'a>(theme: &'a Theme, progress: f32) -> iced::widget::Container<'a, Message> {
    let header = row![
        spinner(
            Spinner::new(theme)
                .progress(progress)
                .size(SpinnerSize::Custom(18.0))
                .color(theme.palette.primary),
        ),
        column![
            small_text(theme, "Downloading..."),
            muted_text(theme, "129 MB / 1000 MB"),
        ]
        .spacing(4),
        space().width(Length::Fill),
        button_content(
            text("Cancel").size(12),
            Some(Message::Pressed),
            ButtonProps::new()
                .variant(ButtonVariant::Outline)
                .size(ButtonSize::One),
            theme,
        ),
    ]
    .spacing(12)
    .align_y(Alignment::Center);

    let item_width = 448.0;
    let item_padding = 16.0;
    let item_radius = 16.0;
    let progress_track_width = item_width - item_padding * 2.0;
    let progress_fill_width = progress_track_width * 0.75;

    let progress_bar = {
        let fill = container(space().height(Length::Fixed(6.0)))
            .width(Length::Fixed(progress_fill_width))
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
            .width(Length::Fixed(progress_track_width))
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
        .padding(item_padding)
        .width(Length::Fixed(item_width))
        .style(move |_theme| iced::widget::container::Style {
            border: Border {
                radius: item_radius.into(),
                width: 1.0,
                color: theme.palette.border,
            },
            ..iced::widget::container::Style::default()
        })
}

enum BadgeStyle {
    Solid(Color, Color),
    Outline(Color),
}

fn spinner_badge<'a>(
    theme: &'a Theme,
    progress: f32,
    label_text: &'a str,
    style: BadgeStyle,
) -> iced::widget::Container<'a, Message> {
    let (background, outlined, text_color) = match style {
        BadgeStyle::Solid(bg, text_color) => (Some(bg), false, text_color),
        BadgeStyle::Outline(text_color) => (None, true, text_color),
    };

    let content = row![
        spinner(
            Spinner::new(theme)
                .progress(progress)
                .size(SpinnerSize::One)
                .color(text_color),
        ),
        text(label_text)
            .size(12)
            .style(move |_theme| iced::widget::text::Style {
                color: Some(text_color),
            }),
    ]
    .spacing(8)
    .align_y(Alignment::Center);

    let background = background.map(Background::Color);
    container(content)
        .padding([4.0, 8.0])
        .style(move |_theme| iced::widget::container::Style {
            background,
            border: Border {
                radius: 19.2.into(),
                width: if outlined { 1.0 } else { 0.0 },
                color: theme.palette.border,
            },
            ..iced::widget::container::Style::default()
        })
}

fn spinner_input_group<'a>(theme: &'a Theme, progress: f32) -> iced::widget::Column<'a, Message> {
    let input_props = TextFieldProps::new()
        .size(TextFieldSize::Two)
        .variant(TextFieldVariant::Surface)
        .disabled(true);

    let first = row![
        text_field(
            "",
            "Send a message...",
            None::<fn(String) -> Message>,
            input_props,
            theme
        )
        .width(Length::Fill),
        spinner(
            Spinner::new(theme)
                .progress(progress)
                .size(SpinnerSize::One)
                .color(theme.palette.muted_foreground),
        ),
    ]
    .spacing(8)
    .align_y(Alignment::Center)
    .width(Length::Fill);

    let second = column![
        container(text("Send a message...").size(14).style(|_theme| {
            iced::widget::text::Style {
                color: Some(theme.palette.muted_foreground),
            }
        }),)
        .padding([10.0, 12.0])
        .width(Length::Fill)
        .style(move |_theme| iced::widget::container::Style {
            background: Some(Background::Color(theme.palette.background)),
            border: Border {
                radius: theme.radius.sm.into(),
                width: 1.0,
                color: theme.palette.border,
            },
            ..iced::widget::container::Style::default()
        }),
        row![
            spinner(
                Spinner::new(theme)
                    .progress(progress)
                    .size(SpinnerSize::One)
                    .color(theme.palette.muted_foreground),
            ),
            text("Validating...")
                .size(12)
                .style(|_theme| iced::widget::text::Style {
                    color: Some(theme.palette.muted_foreground),
                }),
            space().width(Length::Fill),
            button_content(
                row![icon_arrow_up().size(12)].align_y(Alignment::Center),
                Some(Message::Pressed),
                ButtonProps::new()
                    .variant(ButtonVariant::Solid)
                    .size(ButtonSize::One),
                theme,
            ),
        ]
        .spacing(8)
        .align_y(Alignment::Center),
    ]
    .spacing(8)
    .width(Length::Fill);

    column![first, second]
        .spacing(16)
        .width(Length::Fixed(448.0))
}

fn small_text<'a>(
    theme: &'a Theme,
    content: impl iced::widget::text::IntoFragment<'a>,
) -> iced::widget::Text<'a> {
    text(content)
        .size(14)
        .style(move |_theme| iced::widget::text::Style {
            color: Some(theme.palette.foreground),
        })
}

fn large_text<'a>(
    theme: &'a Theme,
    content: impl iced::widget::text::IntoFragment<'a>,
) -> iced::widget::Text<'a> {
    text(content)
        .size(18)
        .style(move |_theme| iced::widget::text::Style {
            color: Some(theme.palette.foreground),
        })
}

fn muted_text<'a>(
    theme: &'a Theme,
    content: impl iced::widget::text::IntoFragment<'a>,
) -> iced::widget::Text<'a> {
    text(content)
        .size(14)
        .style(move |_theme| iced::widget::text::Style {
            color: Some(theme.palette.muted_foreground),
        })
}
