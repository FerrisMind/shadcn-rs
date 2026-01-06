use iced::border::Border;
use iced::widget::{column, container, row, text};
use iced::{Alignment, Background, Color, Element, Length};

use iced_shadcn::{AccentColor, CheckboxProps, CheckboxSize, Theme, checkbox, label};
use lucide_icons::LUCIDE_FONT_BYTES;

pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view)
        .font(LUCIDE_FONT_BYTES)
        .run()
}

struct Example {
    theme: Theme,
    checked: bool,
    terms: bool,
    card: bool,
}

#[derive(Debug, Clone)]
enum Message {
    Toggle(bool),
    Terms(bool),
    Card(bool),
}

impl Example {
    fn update(&mut self, message: Message) {
        match message {
            Message::Toggle(value) => self.checked = value,
            Message::Terms(value) => self.terms = value,
            Message::Card(value) => self.card = value,
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let background = theme.palette.background;
        let border = theme.palette.border;
        let radius = theme.radius.md;

        let content = column![
            row![
                checkbox(
                    "",
                    self.checked,
                    Some(Message::Toggle),
                    CheckboxProps::new().size(CheckboxSize::Two),
                    theme,
                ),
                label("Accept terms and conditions", theme),
            ]
            .spacing(12)
            .align_y(Alignment::Center),
            row![
                checkbox(
                    "",
                    self.terms,
                    Some(Message::Terms),
                    CheckboxProps::new().size(CheckboxSize::Two),
                    theme,
                ),
                column![
                    label("Accept terms and conditions", theme),
                    text("By clicking this checkbox, you agree to the terms and conditions.")
                        .size(14)
                        .style(|_theme| iced::widget::text::Style {
                            color: Some(theme.palette.muted_foreground),
                        }),
                ]
                .spacing(6),
            ]
            .spacing(12)
            .align_y(Alignment::Start),
            row![
                checkbox(
                    "",
                    false,
                    None::<fn(bool) -> Message>,
                    CheckboxProps::new()
                        .size(CheckboxSize::Two)
                        .disabled(true),
                    theme,
                ),
                label("Enable notifications", theme),
            ]
            .spacing(12)
            .align_y(Alignment::Center),
            {
                let border = if self.card {
                    theme.palette.primary
                } else {
                    theme.palette.border
                };
                let background = if self.card {
                    Color {
                        a: 0.08,
                        ..theme.palette.primary
                    }
                } else {
                    theme.palette.card
                };

                container(
                    row![
                        checkbox(
                            "",
                            self.card,
                            Some(Message::Card),
                            CheckboxProps::new()
                                .size(CheckboxSize::Two)
                                .color(AccentColor::Blue),
                            theme,
                        ),
                        column![
                            text("Enable notifications")
                                .size(14)
                                .style(|_theme| iced::widget::text::Style {
                                    color: Some(theme.palette.foreground),
                                }),
                            text("You can enable or disable notifications at any time.")
                                .size(14)
                                .style(|_theme| iced::widget::text::Style {
                                    color: Some(theme.palette.muted_foreground),
                                }),
                        ]
                        .spacing(6),
                    ]
                    .spacing(12)
                    .align_y(Alignment::Start),
                )
                .padding(12)
                .style(move |_theme| iced::widget::container::Style {
                    background: Some(Background::Color(background)),
                    border: Border {
                        radius: theme.radius.md.into(),
                        width: 1.0,
                        color: border,
                    },
                    ..iced::widget::container::Style::default()
                })
            },
        ]
        .spacing(24)
        .align_x(Alignment::Start);

        container(content)
            .width(Length::Fill)
            .padding(32)
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

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::default(),
            checked: false,
            terms: true,
            card: true,
        }
    }
}
