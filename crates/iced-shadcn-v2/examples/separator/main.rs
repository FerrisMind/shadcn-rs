//! Interactive playground for `iced-shadcn-v2::Separator` + `shadcn-common` theme knobs.
//!
//! Run: `cargo run -p iced-shadcn-v2 --example separator`

use std::fmt;

use iced::widget::{column, container, pick_list, row, scrollable, text};
use iced::{Alignment, Background, Border, Color, Element, Length, Task};

use iced_shadcn_v2::{
    BaseColor, FontId, Separator, SeparatorOrientation, StyleId, Theme, ThemeMode, fonts,
    iced_font, separator,
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
    thickness: ThicknessOpt,
    color: ColorOpt,
}

#[derive(Debug, Clone)]
enum Message {
    Style(Labelled<StyleId>),
    Base(Labelled<BaseColor>),
    Mode(Labelled<ThemeMode>),
    Thickness(ThicknessOpt),
    Color(ColorOpt),
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::light(),
            thickness: ThicknessOpt(1),
            color: ColorOpt::Border,
        }
    }
}

impl Example {
    fn title(&self) -> String {
        "iced-shadcn-v2 Separator".to_owned()
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
            Message::Thickness(thickness) => {
                self.thickness = thickness;
            }
            Message::Color(color) => {
                self.color = color;
            }
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let p = &theme.palette;

        let controls = column![
            section_label("Theme (shadcn-common)", p.muted_foreground, theme),
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
            section_label("Separator knobs", p.muted_foreground, theme),
            control_select(
                "Thickness",
                &THICKNESSES,
                Some(self.thickness),
                Message::Thickness,
                theme,
            ),
            control_select("Color", &COLORS, Some(self.color), Message::Color, theme),
        ]
        .spacing(8);

        // Mirror of the shadcn-svelte `separator-demo` block.
        let demo = column![
            column![
                text("Radix Primitives")
                    .size(14)
                    .font(iced_font(theme.font_pack().sans))
                    .color(p.foreground),
                text("An open-source UI component library.")
                    .size(14)
                    .font(iced_font(theme.font_pack().sans))
                    .color(p.muted_foreground),
            ]
            .spacing(4),
            self.build_separator(theme),
            row![
                text("Blog").size(14).color(p.foreground),
                separator(
                    self.build_separator(theme)
                        .orientation(SeparatorOrientation::Vertical)
                ),
                text("Docs").size(14).color(p.foreground),
                separator(
                    self.build_separator(theme)
                        .orientation(SeparatorOrientation::Vertical)
                ),
                text("Source").size(14).color(p.foreground),
            ]
            .spacing(16)
            .height(Length::Fixed(20.0))
            .align_y(Alignment::Center),
        ]
        .spacing(16);

        let preview = column![
            section_label("Preview (separator-demo)", p.muted_foreground, theme),
            container(demo)
                .padding(24)
                .width(Length::Fill)
                .style(move |_| container::Style {
                    background: Some(Background::Color(p.card)),
                    border: Border {
                        color: p.border,
                        width: 1.0,
                        radius: 8.0.into(),
                    },
                    ..container::Style::default()
                }),
        ]
        .spacing(8);

        let lengths = column![
            section_label("Fixed lengths", p.muted_foreground, theme),
            self.build_separator(theme).length(Length::Fixed(240.0)),
            self.build_separator(theme).length(Length::Fixed(120.0)),
            self.build_separator(theme).length(Length::Fixed(60.0)),
        ]
        .spacing(12);

        let content = column![
            text("iced-shadcn-v2 Separator")
                .size(32)
                .font(iced_font(theme.font_pack().heading))
                .color(p.foreground),
            text("Theme-driven horizontal and vertical rules")
                .size(14)
                .font(iced_font(theme.font_pack().sans))
                .color(p.muted_foreground),
            controls,
            preview,
            lengths,
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

    fn build_separator(&self, theme: &Theme) -> Separator {
        Separator::new(theme)
            .color(self.color.resolve(theme))
            .thickness(self.thickness.0 as f32)
    }
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
    .align_y(Alignment::Center)
    .into()
}

fn section_label<'a>(label: &'static str, color: Color, theme: &'a Theme) -> Element<'a, Message> {
    text(label)
        .size(18)
        .font(iced_font(theme.font_pack().heading))
        .color(color)
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
struct ThicknessOpt(u32);

impl fmt::Display for ThicknessOpt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} px", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ColorOpt {
    Border,
    Muted,
    Primary,
    Destructive,
}

impl ColorOpt {
    fn resolve(self, theme: &Theme) -> Color {
        let p = &theme.palette;
        match self {
            Self::Border => p.border,
            Self::Muted => p.muted_foreground,
            Self::Primary => p.primary,
            Self::Destructive => p.destructive,
        }
    }
}

impl fmt::Display for ColorOpt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Border => f.write_str("border (default)"),
            Self::Muted => f.write_str("muted"),
            Self::Primary => f.write_str("primary"),
            Self::Destructive => f.write_str("destructive"),
        }
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

const THICKNESSES: [ThicknessOpt; 4] = [
    ThicknessOpt(1),
    ThicknessOpt(2),
    ThicknessOpt(4),
    ThicknessOpt(8),
];

const COLORS: [ColorOpt; 4] = [
    ColorOpt::Border,
    ColorOpt::Muted,
    ColorOpt::Primary,
    ColorOpt::Destructive,
];
