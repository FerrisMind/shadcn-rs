//! Interactive playground for `iced-shadcn-v2::Badge`.
//!
//! Mirrors shadcn-svelte badge examples: variants, icons, spinner, as-link,
//! accent colors, and long text — plus theme knobs from `shadcn-common`.
//!
//! Run: `cargo run -p iced-shadcn-v2 --example badge`

use std::fmt;

use iced::border::Border;
use iced::widget::{column, container, pick_list, row, scrollable, text};
use iced::{Alignment, Background, Color, Element, Length, Task};

use iced_shadcn_v2::{
    AccentColor, Badge, BadgeRadius, BadgeVariant, BaseColor, FontHeading, FontId, FontPack,
    RadiusId, StyleId, Theme, ThemeMode, fonts, iced_font,
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
    loading: bool,
    pressed_count: u32,
}

#[derive(Debug, Clone)]
enum Message {
    Style(Labelled<StyleId>),
    Base(Labelled<BaseColor>),
    Accent(AccentOpt),
    Mode(Labelled<ThemeMode>),
    Font(Labelled<FontId>),
    Heading(Labelled<FontHeading>),
    Radius(Labelled<RadiusId>),
    ToggleLoading,
    Pressed,
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::light(),
            loading: false,
            pressed_count: 0,
        }
    }
}

impl Example {
    fn title(&self) -> String {
        "iced-shadcn-v2 Badge".to_owned()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Style(style) => {
                self.theme = self.theme.clone().with_style(style.0);
            }
            Message::Base(base) => {
                self.theme = self.theme.clone().with_base(base.0);
            }
            Message::Accent(accent) => {
                self.theme = self.theme.clone().with_accent(accent.into_option());
            }
            Message::Mode(mode) => {
                self.theme = self.theme.clone().with_mode(mode.0);
            }
            Message::Font(font) => {
                self.theme = self.theme.clone().with_font(font.0);
            }
            Message::Heading(heading) => {
                self.theme = self.theme.clone().with_font_heading(heading.0);
            }
            Message::Radius(radius) => {
                self.theme = self.theme.clone().with_radius(radius.0);
            }
            Message::ToggleLoading => {
                self.loading = !self.loading;
            }
            Message::Pressed => {
                self.pressed_count += 1;
            }
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let p = &theme.palette;
        let font = iced_font(theme.font_pack().sans);

        let controls = column![
            section_label(
                "Theme (shadcn-common)",
                p.muted_foreground,
                theme.font_pack()
            ),
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
                "Accent",
                &ACCENTS,
                Some(AccentOpt::from_option(theme.accent())),
                Message::Accent,
                theme,
            ),
            control_select(
                "Mode",
                &MODES,
                Some(Labelled(theme.mode())),
                Message::Mode,
                theme,
            ),
            control_select(
                "Heading",
                &HEADINGS,
                Some(Labelled(theme.font_heading())),
                Message::Heading,
                theme,
            ),
            control_select(
                "Font",
                &FONTS,
                Some(Labelled(theme.font_id())),
                Message::Font,
                theme,
            ),
            control_select(
                "Radius",
                &RADII,
                Some(Labelled(theme.radius_id())),
                Message::Radius,
                theme,
            ),
        ]
        .spacing(8);

        let variants = row![
            Badge::text("Default", theme).variant(BadgeVariant::Default),
            Badge::text("Secondary", theme).variant(BadgeVariant::Secondary),
            Badge::text("Destructive", theme).variant(BadgeVariant::Destructive),
            Badge::text("Outline", theme).variant(BadgeVariant::Outline),
            Badge::text("Ghost", theme).variant(BadgeVariant::Ghost),
            Badge::text("Link", theme).variant(BadgeVariant::Link),
        ]
        .spacing(8)
        .align_y(Alignment::Center)
        .wrap();

        let with_icons = row![
            Badge::text("Default", theme)
                .variant(BadgeVariant::Default)
                .icon_start(text("✓").size(12).font(font)),
            Badge::text("Secondary", theme)
                .variant(BadgeVariant::Secondary)
                .icon_start(text("✓").size(12).font(font)),
            Badge::text("Destructive", theme)
                .variant(BadgeVariant::Destructive)
                .icon_start(text("✓").size(12).font(font)),
            Badge::text("Outline", theme)
                .variant(BadgeVariant::Outline)
                .icon_end(text("↗").size(12).font(font)),
            Badge::text("Ghost", theme)
                .variant(BadgeVariant::Ghost)
                .icon_end(text("↗").size(12).font(font)),
            Badge::text("Link", theme)
                .variant(BadgeVariant::Link)
                .icon_end(text("↗").size(12).font(font)),
        ]
        .spacing(8)
        .align_y(Alignment::Center)
        .wrap();

        let with_spinner = row![
            Badge::text("Default", theme)
                .variant(BadgeVariant::Default)
                .loading(self.loading),
            Badge::text("Secondary", theme)
                .variant(BadgeVariant::Secondary)
                .loading(self.loading),
            Badge::text("Destructive", theme)
                .variant(BadgeVariant::Destructive)
                .loading(self.loading),
            Badge::text("Outline", theme)
                .variant(BadgeVariant::Outline)
                .loading(self.loading),
            Badge::text("Ghost", theme)
                .variant(BadgeVariant::Ghost)
                .loading(self.loading),
            Badge::text("Link", theme)
                .variant(BadgeVariant::Link)
                .loading(self.loading),
        ]
        .spacing(8)
        .align_y(Alignment::Center)
        .wrap();

        let as_link = row![
            Badge::text("Link", theme)
                .variant(BadgeVariant::Default)
                .icon_end(text("↗").size(12).font(font))
                .on_press(Message::Pressed),
            Badge::text("Link", theme)
                .variant(BadgeVariant::Secondary)
                .icon_end(text("↗").size(12).font(font))
                .on_press(Message::Pressed),
            Badge::text("Link", theme)
                .variant(BadgeVariant::Destructive)
                .icon_end(text("↗").size(12).font(font))
                .on_press(Message::Pressed),
            Badge::text("Link", theme)
                .variant(BadgeVariant::Outline)
                .icon_end(text("↗").size(12).font(font))
                .on_press(Message::Pressed),
            Badge::text("Link", theme)
                .variant(BadgeVariant::Ghost)
                .icon_end(text("↗").size(12).font(font))
                .on_press(Message::Pressed),
            Badge::text("Link", theme)
                .variant(BadgeVariant::Link)
                .icon_end(text("↗").size(12).font(font))
                .on_press(Message::Pressed),
        ]
        .spacing(8)
        .align_y(Alignment::Center)
        .wrap();

        let accent_tones = row![
            Badge::text("Theme primary", theme).variant(BadgeVariant::Default),
            Badge::text("Blue", theme)
                .variant(BadgeVariant::Default)
                .color(AccentColor::Blue),
            Badge::text("Green", theme)
                .variant(BadgeVariant::Default)
                .color(AccentColor::Green),
            Badge::text("Amber", theme)
                .variant(BadgeVariant::Default)
                .color(AccentColor::Amber),
            Badge::text("Rose", theme)
                .variant(BadgeVariant::Default)
                .color(AccentColor::Rose),
            Badge::text("Violet soft", theme)
                .variant(BadgeVariant::Secondary)
                .color(AccentColor::Violet),
        ]
        .spacing(8)
        .align_y(Alignment::Center)
        .wrap();

        let radii = row![
            Badge::text("None", theme)
                .variant(BadgeVariant::Outline)
                .radius(BadgeRadius::None),
            Badge::text("Small", theme)
                .variant(BadgeVariant::Outline)
                .radius(BadgeRadius::Small),
            Badge::text("Medium", theme)
                .variant(BadgeVariant::Outline)
                .radius(BadgeRadius::Medium),
            Badge::text("Large", theme)
                .variant(BadgeVariant::Outline)
                .radius(BadgeRadius::Large),
            Badge::text("Full", theme)
                .variant(BadgeVariant::Outline)
                .radius(BadgeRadius::Full),
        ]
        .spacing(8)
        .align_y(Alignment::Center)
        .wrap();

        let long_text = Badge::text(
            "A badge with a lot of text to see how it stays on one line",
            theme,
        )
        .variant(BadgeVariant::Secondary);

        let toggle = Badge::text(
            if self.loading {
                "Stop spinner"
            } else {
                "Start spinner"
            },
            theme,
        )
        .variant(BadgeVariant::Outline)
        .on_press(Message::ToggleLoading);

        let content = column![
            text("iced-shadcn-v2 Badge")
                .size(32)
                .font(iced_font(theme.font_pack().heading))
                .color(p.foreground),
            text("shadcn-svelte parity: variants · icons · spinner · as-link · accents")
                .size(14)
                .font(font)
                .color(p.muted_foreground),
            text(format!("Pressed: {}", self.pressed_count))
                .size(14)
                .font(font)
                .color(p.foreground),
            controls,
            section_label("Variants", p.muted_foreground, theme.font_pack()),
            variants,
            section_label("With icons", p.muted_foreground, theme.font_pack()),
            with_icons,
            section_label("With spinner", p.muted_foreground, theme.font_pack()),
            row![toggle, with_spinner]
                .spacing(12)
                .align_y(Alignment::Center)
                .wrap(),
            section_label("As link (on_press)", p.muted_foreground, theme.font_pack()),
            as_link,
            section_label("Accent colors", p.muted_foreground, theme.font_pack()),
            accent_tones,
            section_label("Radius", p.muted_foreground, theme.font_pack()),
            radii,
            section_label("Long text", p.muted_foreground, theme.font_pack()),
            long_text,
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
            .width(72)
            .font(font)
            .color(p.muted_foreground),
        pick_list(options, selected, on_select)
            .text_size(13)
            .font(font)
            .width(Length::Fixed(200.0))
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

fn section_label<'a>(label: &'static str, color: Color, pack: FontPack) -> Element<'a, Message> {
    text(label)
        .size(18)
        .font(iced_font(pack.heading))
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

impl fmt::Display for Labelled<FontId> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0.title())
    }
}

impl fmt::Display for Labelled<FontHeading> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0.title())
    }
}

impl fmt::Display for Labelled<RadiusId> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0.label())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AccentOpt {
    None,
    Color(AccentColor),
}

impl AccentOpt {
    const fn from_option(accent: Option<AccentColor>) -> Self {
        match accent {
            None => Self::None,
            Some(color) => Self::Color(color),
        }
    }

    const fn into_option(self) -> Option<AccentColor> {
        match self {
            Self::None => None,
            Self::Color(color) => Some(color),
        }
    }
}

impl fmt::Display for AccentOpt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => f.write_str("none"),
            Self::Color(color) => f.write_str(color.as_str()),
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

const ACCENTS: [AccentOpt; 18] = [
    AccentOpt::None,
    AccentOpt::Color(AccentColor::Amber),
    AccentOpt::Color(AccentColor::Blue),
    AccentOpt::Color(AccentColor::Cyan),
    AccentOpt::Color(AccentColor::Emerald),
    AccentOpt::Color(AccentColor::Fuchsia),
    AccentOpt::Color(AccentColor::Green),
    AccentOpt::Color(AccentColor::Indigo),
    AccentOpt::Color(AccentColor::Lime),
    AccentOpt::Color(AccentColor::Orange),
    AccentOpt::Color(AccentColor::Pink),
    AccentOpt::Color(AccentColor::Purple),
    AccentOpt::Color(AccentColor::Red),
    AccentOpt::Color(AccentColor::Rose),
    AccentOpt::Color(AccentColor::Sky),
    AccentOpt::Color(AccentColor::Teal),
    AccentOpt::Color(AccentColor::Violet),
    AccentOpt::Color(AccentColor::Yellow),
];

const MODES: [Labelled<ThemeMode>; 2] = [Labelled(ThemeMode::Light), Labelled(ThemeMode::Dark)];

const FONTS: [Labelled<FontId>; 5] = [
    Labelled(FontId::Geist),
    Labelled(FontId::Inter),
    Labelled(FontId::InstrumentSerif),
    Labelled(FontId::GeistMono),
    Labelled(FontId::JetBrainsMono),
];

const HEADINGS: [Labelled<FontHeading>; 6] = [
    Labelled(FontHeading::Inherit),
    Labelled(FontHeading::Font(FontId::Geist)),
    Labelled(FontHeading::Font(FontId::Inter)),
    Labelled(FontHeading::Font(FontId::InstrumentSerif)),
    Labelled(FontHeading::Font(FontId::GeistMono)),
    Labelled(FontHeading::Font(FontId::JetBrainsMono)),
];

const RADII: [Labelled<RadiusId>; 5] = [
    Labelled(RadiusId::Default),
    Labelled(RadiusId::None),
    Labelled(RadiusId::Small),
    Labelled(RadiusId::Medium),
    Labelled(RadiusId::Large),
];
