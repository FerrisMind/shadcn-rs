//! Interactive playground for `iced-shadcn-v2::AspectRatio`.
//!
//! Run: `cargo run -p iced-shadcn-v2 --example aspect_ratio`

use std::fmt;

use iced::widget::{column, container, pick_list, row, scrollable, text};
use iced::{Background, Border, Element, Length, Task};

use iced_shadcn_v2::{AspectRatio, BaseColor, FontId, StyleId, Theme, ThemeMode, fonts, iced_font};

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
    ratio: RatioOpt,
}

#[derive(Debug, Clone)]
enum Message {
    Style(Labelled<StyleId>),
    Base(Labelled<BaseColor>),
    Mode(Labelled<ThemeMode>),
    Ratio(RatioOpt),
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::light(),
            ratio: RatioOpt::R16x9,
        }
    }
}

impl Example {
    fn title(&self) -> String {
        "iced-shadcn-v2 AspectRatio".to_owned()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Style(style) => {
                self.theme = self.theme.clone().with_style(style.0);
            }
            Message::Base(base) => {
                self.theme = self.theme.clone().with_base(base.0);
            }
            Message::Mode(mode) => {
                self.theme = self.theme.clone().with_mode(mode.0);
            }
            Message::Ratio(ratio) => {
                self.ratio = ratio;
            }
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let p = &theme.palette;

        let controls = column![
            control_select(
                "Style",
                &STYLES,
                Some(Labelled(theme.style_id())),
                Message::Style,
                theme,
            ),
            control_select(
                "Base",
                &BASES,
                Some(Labelled(theme.base())),
                Message::Base,
                theme,
            ),
            control_select(
                "Mode",
                &MODES,
                Some(Labelled(theme.mode())),
                Message::Mode,
                theme,
            ),
            control_select("Ratio", &RATIOS, Some(self.ratio), Message::Ratio, theme),
        ]
        .spacing(8);

        let preview = container(
            AspectRatio::new(
                container(
                    text(self.ratio.label())
                        .size(20)
                        .font(iced_font(theme.font_pack().heading))
                        .color(p.muted_foreground),
                )
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x(Length::Fill)
                .center_y(Length::Fill),
            )
            .ratio(self.ratio.value())
            .muted(theme)
            .radius(theme.style.radius.md_px),
        )
        .width(Length::Fixed(420.0))
        .style(move |_| container::Style {
            background: Some(Background::Color(p.card)),
            border: Border {
                color: p.border,
                width: 1.0,
                radius: theme.style.radius.md_px.into(),
            },
            ..container::Style::default()
        });

        let gallery = row![
            card("16:9", 16.0 / 9.0, theme),
            card("1:1", 1.0, theme),
            card("9:16", 9.0 / 16.0, theme),
        ]
        .spacing(12)
        .wrap();

        let content = column![
            text("iced-shadcn-v2 AspectRatio")
                .size(32)
                .font(iced_font(theme.font_pack().heading))
                .color(p.foreground),
            text("Layout wrapper preserving content ratio (shadcn-svelte parity)")
                .size(14)
                .font(iced_font(theme.font_pack().sans))
                .color(p.muted_foreground),
            controls,
            preview,
            gallery,
        ]
        .spacing(16)
        .max_width(960)
        .padding(8);

        container(
            scrollable(
                container(content)
                    .width(Length::Fill)
                    .center_x(Length::Fill)
                    .padding(24),
            )
            .width(Length::Fill)
            .height(Length::Fill),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .style(move |_| container::Style {
            background: Some(Background::Color(p.background)),
            text_color: Some(p.foreground),
            ..container::Style::default()
        })
        .into()
    }
}

fn card<'a>(label: &'static str, ratio: f32, theme: &'a Theme) -> Element<'a, Message> {
    let p = &theme.palette;

    container(
        column![
            text(label)
                .size(12)
                .font(iced_font(theme.font_pack().mono))
                .color(p.muted_foreground),
            container(
                AspectRatio::new(
                    container(text("preview").size(12).color(p.muted_foreground))
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .center_x(Length::Fill)
                        .center_y(Length::Fill),
                )
                .ratio(ratio)
                .muted(theme)
                .radius(theme.style.radius.sm_px),
            )
            .width(Length::Fixed(180.0)),
        ]
        .spacing(8),
    )
    .padding(12)
    .style(move |_| container::Style {
        background: Some(Background::Color(p.card)),
        border: Border {
            color: p.border,
            width: 1.0,
            radius: theme.style.radius.md_px.into(),
        },
        ..container::Style::default()
    })
    .into()
}

fn control_select<'a, T, F>(
    label: &'static str,
    options: &'a [T],
    selected: Option<T>,
    on_select: F,
    theme: &'a Theme,
) -> Element<'a, Message>
where
    T: Clone + PartialEq + fmt::Display + 'a,
    F: Fn(T) -> Message + 'a,
{
    let p = theme.palette;
    let font = iced_font(theme.font_pack().sans);

    row![
        text(label)
            .size(13)
            .width(80)
            .font(font)
            .color(p.muted_foreground),
        pick_list(options, selected, on_select)
            .text_size(13)
            .font(font)
            .width(Length::Fixed(220.0))
            .style(move |_theme, _status| pick_list::Style {
                background: Background::Color(p.background),
                text_color: p.foreground,
                placeholder_color: p.muted_foreground,
                handle_color: p.muted_foreground,
                border: Border {
                    color: p.input,
                    width: 1.0,
                    radius: 6.0.into(),
                },
            }),
    ]
    .spacing(8)
    .into()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Labelled<T>(T);

impl fmt::Display for Labelled<StyleId> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0.as_str())
    }
}

impl fmt::Display for Labelled<BaseColor> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0.as_str())
    }
}

impl fmt::Display for Labelled<ThemeMode> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RatioOpt {
    R1x1,
    R16x9,
    R21x9,
    R9x16,
}

impl RatioOpt {
    const fn value(self) -> f32 {
        match self {
            Self::R1x1 => 1.0,
            Self::R16x9 => 16.0 / 9.0,
            Self::R21x9 => 21.0 / 9.0,
            Self::R9x16 => 9.0 / 16.0,
        }
    }

    const fn label(self) -> &'static str {
        match self {
            Self::R1x1 => "1:1",
            Self::R16x9 => "16:9",
            Self::R21x9 => "21:9",
            Self::R9x16 => "9:16",
        }
    }
}

impl fmt::Display for RatioOpt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.label())
    }
}

const STYLES: [Labelled<StyleId>; 8] = [
    Labelled(StyleId::Vega),
    Labelled(StyleId::Nova),
    Labelled(StyleId::Maia),
    Labelled(StyleId::Lyra),
    Labelled(StyleId::Mira),
    Labelled(StyleId::Luma),
    Labelled(StyleId::Sera),
    Labelled(StyleId::Rhea),
];

const BASES: [Labelled<BaseColor>; 7] = [
    Labelled(BaseColor::Neutral),
    Labelled(BaseColor::Zinc),
    Labelled(BaseColor::Stone),
    Labelled(BaseColor::Mauve),
    Labelled(BaseColor::Mist),
    Labelled(BaseColor::Olive),
    Labelled(BaseColor::Taupe),
];

const MODES: [Labelled<ThemeMode>; 2] = [Labelled(ThemeMode::Light), Labelled(ThemeMode::Dark)];

const RATIOS: [RatioOpt; 4] = [
    RatioOpt::R16x9,
    RatioOpt::R21x9,
    RatioOpt::R1x1,
    RatioOpt::R9x16,
];
