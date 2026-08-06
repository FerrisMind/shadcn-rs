//! Interactive playground for `iced-shadcn-v2::Button` + `shadcn-common` theme knobs.
//!
//! Mirrors the v1 `new-api-button` example, but the theme selects use
//! `iced::widget::pick_list` because v2 does not depend on `iced-shadcn` v1.
//!
//! Run: `cargo run -p iced-shadcn-v2 --example button`

use std::fmt;

use iced::widget::{column, container, pick_list, row, scrollable, text};
use iced::{Alignment, Background, Border, Color, Element, Length, Task};

use iced_shadcn_v2::{
    AccentColor, BaseColor, Button, ButtonBuildError, ButtonRadius, ButtonSize, ButtonVariant,
    FontHeading, FontId, FontPack, Padding, RadiusId, Spacing, StyleId, Theme, ThemeMode, fonts,
    iced_font,
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
        "iced-shadcn-v2 Button".to_owned()
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
            text(format!(
                "radius lg={:.0}px · control h={:.0}/{:.0}/{:.0} · sans={} · heading={}",
                theme.radius_scale().lg_px,
                theme.style.control_height_sm_px,
                theme.style.control_height_md_px,
                theme.style.control_height_lg_px,
                theme.font_pack().sans.title(),
                theme.font_heading().title(),
            ))
            .size(12)
            .font(iced_font(theme.font_pack().mono))
            .color(p.muted_foreground),
        ]
        .spacing(8);

        let swatches = row![
            swatch("bg", p.background, p.border),
            swatch("fg", p.foreground, p.border),
            swatch("primary", p.primary, p.border),
            swatch("secondary", p.secondary, p.border),
            swatch("muted", p.muted, p.border),
            swatch("destructive", p.destructive, p.border),
            swatch("border", p.border, p.foreground),
        ]
        .spacing(8)
        .wrap();

        let variants = row![
            Button::text("Default", theme)
                .variant(ButtonVariant::Default)
                .on_press(Message::Pressed),
            Button::text("Outline", theme)
                .variant(ButtonVariant::Outline)
                .on_press(Message::Pressed),
            Button::text("Secondary", theme)
                .variant(ButtonVariant::Secondary)
                .on_press(Message::Pressed),
            Button::text("Ghost", theme)
                .variant(ButtonVariant::Ghost)
                .on_press(Message::Pressed),
            Button::text("Link", theme)
                .variant(ButtonVariant::Link)
                .on_press(Message::Pressed),
            Button::text("Destructive", theme)
                .variant(ButtonVariant::Destructive)
                .on_press(Message::Pressed),
            Button::text("Soft", theme)
                .variant(ButtonVariant::Soft)
                .on_press(Message::Pressed),
            Button::text("Surface", theme)
                .variant(ButtonVariant::Surface)
                .on_press(Message::Pressed),
        ]
        .spacing(12)
        .align_y(Alignment::Center)
        .wrap();

        let accent_tones = row![
            Button::text("Theme primary", theme)
                .variant(ButtonVariant::Default)
                .on_press(Message::Pressed),
            Button::text("Blue", theme)
                .variant(ButtonVariant::Default)
                .color(AccentColor::Blue)
                .on_press(Message::Pressed),
            Button::text("Amber", theme)
                .variant(ButtonVariant::Default)
                .color(AccentColor::Amber)
                .on_press(Message::Pressed),
            Button::text("Emerald", theme)
                .variant(ButtonVariant::Default)
                .color(AccentColor::Emerald)
                .on_press(Message::Pressed),
            Button::text("Rose", theme)
                .variant(ButtonVariant::Default)
                .color(AccentColor::Rose)
                .on_press(Message::Pressed),
        ]
        .spacing(12)
        .align_y(Alignment::Center)
        .wrap();

        let sizes = row![
            Button::text("xs", theme)
                .size(ButtonSize::Xs)
                .variant(ButtonVariant::Outline)
                .on_press(Message::Pressed),
            Button::text("sm", theme)
                .size(ButtonSize::Sm)
                .variant(ButtonVariant::Outline)
                .on_press(Message::Pressed),
            Button::text("default", theme)
                .size(ButtonSize::Default)
                .variant(ButtonVariant::Outline)
                .on_press(Message::Pressed),
            Button::text("lg", theme)
                .size(ButtonSize::Lg)
                .variant(ButtonVariant::Outline)
                .on_press(Message::Pressed),
            Button::icon(text("+").size(14), theme)
                .size(ButtonSize::IconXs)
                .variant(ButtonVariant::Outline)
                .on_press(Message::Pressed),
            Button::icon(text("+").size(16), theme)
                .size(ButtonSize::IconSm)
                .variant(ButtonVariant::Outline)
                .on_press(Message::Pressed),
            Button::icon(text("+").size(18), theme)
                .size(ButtonSize::Icon)
                .radius(ButtonRadius::Full)
                .variant(ButtonVariant::Outline)
                .on_press(Message::Pressed),
            Button::icon(text("+").size(20), theme)
                .size(ButtonSize::IconLg)
                .variant(ButtonVariant::Outline)
                .on_press(Message::Pressed),
        ]
        .spacing(12)
        .align_y(Alignment::Center)
        .wrap();

        let states = column![
            Button::text(
                if self.loading {
                    "Stop loading"
                } else {
                    "Start loading"
                },
                theme,
            )
            .variant(ButtonVariant::Default)
            .loading(self.loading)
            .on_press(Message::ToggleLoading),
            Button::text("Disabled", theme)
                .variant(ButtonVariant::Outline)
                .disabled(true)
                .on_press(Message::Pressed),
            Button::text("Full width action", theme)
                .variant(ButtonVariant::Default)
                .full_width()
                .on_press(Message::Pressed),
            padded_button(theme).unwrap_or_else(|error| {
                text(format!("Padding error: {error}"))
                    .size(13)
                    .color(p.destructive)
                    .into()
            }),
        ]
        .spacing(12)
        .width(Length::Fill);

        let title_px = 32u32;

        let content = column![
            text("iced-shadcn-v2 Button")
                .size(title_px)
                .font(iced_font(theme.font_pack().heading))
                .color(p.foreground),
            text("shadcn-common fonts (Geist / Inter / Instrument Serif / JetBrains Mono)")
                .size(14)
                .font(iced_font(theme.font_pack().sans))
                .color(p.muted_foreground),
            text(format!("Pressed: {}", self.pressed_count))
                .size(14)
                .font(iced_font(theme.font_pack().sans))
                .color(p.foreground),
            controls,
            section_label("Palette", p.muted_foreground, theme.font_pack()),
            swatches,
            section_label("Variants", p.muted_foreground, theme.font_pack()),
            variants,
            section_label(
                "Per-button accent overlay",
                p.muted_foreground,
                theme.font_pack(),
            ),
            accent_tones,
            section_label(
                "Sizes / icon (shadcn xs·sm·default·lg·icon*)",
                p.muted_foreground,
                theme.font_pack()
            ),
            sizes,
            section_label("States / layout", p.muted_foreground, theme.font_pack()),
            states,
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

fn padded_button<'a>(theme: &'a Theme) -> Result<Element<'a, Message>, ButtonBuildError> {
    Ok(Button::text("Custom four-side padding", theme)
        .variant(ButtonVariant::Outline)
        .padding(Padding::individual(
            Spacing::S1,
            Spacing::S3,
            Spacing::S2,
            Spacing::S4,
        ))?
        .on_press(Message::Pressed)
        .into())
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

fn swatch<'a>(label: &'static str, fill: Color, border: Color) -> Element<'a, Message> {
    column![
        container(text(""))
            .width(36)
            .height(36)
            .style(move |_| container::Style {
                background: Some(Background::Color(fill)),
                border: Border {
                    color: border,
                    width: 1.0,
                    radius: 6.0.into(),
                },
                ..container::Style::default()
            }),
        text(label).size(10).color(border),
    ]
    .spacing(4)
    .align_x(Alignment::Center)
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
