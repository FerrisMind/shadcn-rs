//! Interactive playground for `iced-shadcn-v2::Card`.
//!
//! The examples mirror shadcn-svelte card composition: typed header/title /
//! description/action slots, arbitrary content, compact spacing, bordered
//! sections, a column footer, and an edge-to-edge first child.
//!
//! Run with:
//! `cargo run -p iced-shadcn-v2 --example card`

use iced::alignment::Vertical;
use iced::border::Border;
use iced::widget::{column, container, pick_list, row, scrollable, text};
use iced::{Alignment, Background, Color, Element, Length, Task};

use iced_shadcn_v2::{
    Button, ButtonVariant, Card, CardContent, CardDescription, CardFooter, CardHeader, CardRadius,
    CardSize, CardTitle, FontId, StyleId, Theme, ThemeMode, fonts, iced_font,
};

pub fn main() -> iced::Result {
    let mut app = iced::application(Example::default, Example::update, Example::view)
        .title(Example::title)
        .default_font(iced_font(FontId::Geist));

    for face in fonts::ALL_FACES {
        app = app.font(*face);
    }

    app.run()
}

struct Example {
    theme: Theme,
    size: CardSize,
    spacing: SpacingChoice,
    notice: String,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    NextStyle,
    ToggleMode,
    ToggleSize,
    Spacing(SpacingChoice),
    Action,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SpacingChoice {
    Theme,
    Compact,
    Relaxed,
    Custom,
}

impl SpacingChoice {
    const ALL: [Self; 4] = [Self::Theme, Self::Compact, Self::Relaxed, Self::Custom];

    fn pixels(self, theme: &Theme) -> f32 {
        match self {
            Self::Theme => theme.style.card_padding_px,
            Self::Compact => 12.0,
            Self::Relaxed => 24.0,
            Self::Custom => 32.0,
        }
    }
}

impl std::fmt::Display for SpacingChoice {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::Theme => "Theme",
            Self::Compact => "12 px",
            Self::Relaxed => "24 px",
            Self::Custom => "32 px",
        })
    }
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::light(),
            size: CardSize::Default,
            spacing: SpacingChoice::Theme,
            notice: "Ready to create a project".to_owned(),
        }
    }
}

impl Example {
    fn title(&self) -> String {
        "iced-shadcn-v2 Card".to_owned()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::NextStyle => {
                let styles = StyleId::ALL;
                let current = styles
                    .iter()
                    .position(|style| *style == self.theme.style_id())
                    .unwrap_or(0);
                self.theme = self
                    .theme
                    .clone()
                    .with_style(styles[(current + 1) % styles.len()]);
            }
            Message::ToggleMode => {
                let mode = match self.theme.mode() {
                    ThemeMode::Light => ThemeMode::Dark,
                    ThemeMode::Dark => ThemeMode::Light,
                };
                self.theme = self.theme.clone().with_mode(mode);
            }
            Message::ToggleSize => {
                self.size = match self.size {
                    CardSize::Default => CardSize::Sm,
                    CardSize::Sm => CardSize::Default,
                    _ => CardSize::Default,
                };
            }
            Message::Spacing(spacing) => {
                self.spacing = spacing;
            }
            Message::Action => {
                self.notice = "Action pressed — the card emitted a message".to_owned();
            }
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let palette = &theme.palette;
        let sans = iced_font(theme.font_pack().sans);
        let heading = iced_font(theme.font_pack().heading);
        let spacing_px = self.spacing.pixels(theme);

        let controls = row![
            Button::text(format!("Style: {}", theme.style_id().as_str()), theme)
                .variant(ButtonVariant::Outline)
                .on_press(Message::NextStyle),
            Button::text(
                if theme.is_dark() {
                    "Switch to light"
                } else {
                    "Switch to dark"
                },
                theme,
            )
            .variant(ButtonVariant::Ghost)
            .on_press(Message::ToggleMode),
            Button::text(
                if self.size == CardSize::Default {
                    "Use small card"
                } else {
                    "Use default card"
                },
                theme,
            )
            .variant(ButtonVariant::Secondary)
            .on_press(Message::ToggleSize),
        ]
        .spacing(8)
        .align_y(Vertical::Center)
        .wrap();

        let spacing_picker = row![
            text("Card spacing")
                .size(13)
                .font(sans)
                .color(palette.muted_foreground),
            pick_list(
                &SpacingChoice::ALL[..],
                Some(self.spacing),
                Message::Spacing,
            )
            .font(sans)
            .text_size(13)
            .width(Length::Fixed(140.0))
            .style(move |_iced_theme, _status| pick_list::Style {
                background: Background::Color(palette.background),
                text_color: palette.foreground,
                placeholder_color: palette.muted_foreground,
                handle_color: palette.muted_foreground,
                border: Border {
                    color: palette.input,
                    width: 1.0,
                    radius: 8.0.into(),
                },
            }),
        ]
        .spacing(8)
        .align_y(Alignment::Center);

        let header = CardHeader::new(theme)
            .title(CardTitle::text("Create project", theme))
            .description(CardDescription::text(
                "Deploy your new project in one click.",
                theme,
            ))
            .action(
                Button::text("Docs", theme)
                    .variant(ButtonVariant::Link)
                    .on_press(Message::Action),
            );

        let form = column![
            field("Name", "shadcn-rs", theme),
            field("Framework", "SvelteKit", theme),
        ]
        .spacing(12)
        .width(Length::Fill);

        let form_card = Card::new(theme)
            .size(self.size)
            .spacing(spacing_px)
            .width(Length::Fixed(420.0))
            .header(header)
            .content(CardContent::new(theme).push(form))
            .footer(
                CardFooter::new(theme)
                    .column()
                    .spacing(8.0)
                    .push(
                        Button::text("Create project", theme)
                            .full_width()
                            .on_press(Message::Action),
                    )
                    .push(
                        Button::text("Cancel", theme)
                            .variant(ButtonVariant::Outline)
                            .full_width()
                            .on_press(Message::Action),
                    ),
            );

        let edge_card = Card::new(theme)
            .size(self.size)
            .spacing(spacing_px)
            .width(Length::Fixed(420.0))
            .top_padding(0.0)
            .push(
                container(
                    text("Edge-to-edge first child")
                        .size(20)
                        .font(heading)
                        .color(palette.card_foreground),
                )
                .height(Length::Fixed(132.0))
                .width(Length::Fill)
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .style(move |_| container::Style {
                    background: Some(Background::Color(palette.muted)),
                    text_color: Some(palette.foreground),
                    ..container::Style::default()
                }),
            )
            .header(
                CardHeader::new(theme)
                    .title(CardTitle::text("Media card", theme))
                    .description(CardDescription::text(
                        "The first child can touch the rounded card edge.",
                        theme,
                    )),
            )
            .footer(
                CardFooter::new(theme)
                    .border_top()
                    .justify_end()
                    .push(Button::text("View event", theme).on_press(Message::Action)),
            );

        let content = column![
            text("iced-shadcn-v2 Card")
                .size(32)
                .font(heading)
                .color(palette.foreground),
            text("shadcn-svelte parity: sections · action · density · spacing · overrides")
                .size(14)
                .font(sans)
                .color(palette.muted_foreground),
            controls,
            spacing_picker,
            text(&self.notice)
                .size(14)
                .font(sans)
                .color(palette.foreground),
            section_label("Composition", palette.muted_foreground, heading),
            row![form_card, edge_card]
                .spacing(24)
                .align_y(Alignment::Start)
                .wrap(),
            section_label("Radius override", palette.muted_foreground, heading),
            Card::new(theme)
                .width(Length::Fixed(420.0))
                .radius(CardRadius::None)
                .header(
                    CardHeader::new(theme)
                        .title(CardTitle::text("Square card", theme))
                        .description(CardDescription::text(
                            "The root radius is an explicit per-instance override.",
                            theme,
                        )),
                )
                .content(
                    CardContent::new(theme).push(text("Arbitrary child content is accepted."))
                ),
        ]
        .spacing(16)
        .max_width(960)
        .padding(8);

        container(scrollable(
            container(content)
                .width(Length::Fill)
                .center_x(Length::Fill)
                .padding(24),
        ))
        .width(Length::Fill)
        .height(Length::Fill)
        .style(move |_| container::Style {
            background: Some(Background::Color(palette.background)),
            text_color: Some(palette.foreground),
            ..container::Style::default()
        })
        .into()
    }
}

fn field<'a>(label: &'static str, value: &'static str, theme: &'a Theme) -> Element<'a, Message> {
    let palette = &theme.palette;
    column![
        text(label)
            .size(13)
            .font(iced_font(theme.font_pack().sans))
            .color(palette.foreground),
        container(
            text(value)
                .size(14)
                .font(iced_font(theme.font_pack().sans))
                .color(palette.muted_foreground),
        )
        .padding(8)
        .width(Length::Fill)
        .style(move |_| container::Style {
            background: Some(Background::Color(palette.background)),
            border: Border {
                color: palette.border,
                width: 1.0,
                radius: 8.0.into(),
            },
            ..container::Style::default()
        }),
    ]
    .spacing(6)
    .width(Length::Fill)
    .into()
}

fn section_label<'a>(
    label: &'static str,
    color: Color,
    heading: iced::Font,
) -> Element<'a, Message> {
    text(label).size(18).font(heading).color(color).into()
}
