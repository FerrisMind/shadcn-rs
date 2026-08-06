//! Interactive playground for `iced-shadcn-v2::Meter`.
//!
//! Meter has no pack-specific tokens in shadcn-svelte-extras (hard-coded `h-2`
//! / `rounded-full` / `/20` track) — same idea as Form. The example therefore
//! exposes the shared Theme knobs (Style / Base / Accent / Mode / Font /
//! Radius): Style picks Rhea/Nova fonts and Button chrome; Base / Accent /
//! Mode retint Meter fills from the theme palette.
//!
//! Run: `cargo run -p iced-shadcn-v2 --example meter`

use std::fmt;
use std::time::Duration;

use iced::widget::{column, container, pick_list, row, scrollable, slider, text};
use iced::{Alignment, Background, Border, Color, Element, Length, Task};

use iced_shadcn_v2::{
    AccentColor, BaseColor, Button, ButtonVariant, FontHeading, FontId, FontPack, Meter,
    MeterFillTone, MeterOrientation, MeterSize, RadiusId, StyleId, Theme, ThemeMode, fonts,
    iced_font, meter_value_label,
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
    value: f32,
    min: f32,
    max: f32,
    animated: bool,
    auto_tone: bool,
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
    ValueChanged(f32),
    ToggleAnimated,
    ToggleAutoTone,
    FillTokens,
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::light(),
            value: 0.0,
            min: 0.0,
            max: 100.0,
            animated: true,
            auto_tone: true,
        }
    }
}

impl Example {
    fn title(&self) -> String {
        "iced-shadcn-v2 Meter".to_owned()
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
            Message::ValueChanged(value) => self.value = value,
            Message::ToggleAnimated => self.animated = !self.animated,
            Message::ToggleAutoTone => self.auto_tone = !self.auto_tone,
            Message::FillTokens => self.value = self.max,
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
                "Font",
                &FONTS,
                Some(Labelled(theme.font_id())),
                Message::Font,
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
                "Radius",
                &RADII,
                Some(Labelled(theme.radius_id())),
                Message::Radius,
                theme,
            ),
            text(format!(
                "style={} · Meter geometry shared · Button/fonts from pack · fills from Theme palette",
                theme.style_id().as_str(),
            ))
            .size(12)
            .font(iced_font(theme.font_pack().mono))
            .color(p.muted_foreground),
            section_label("Meter", p.muted_foreground, theme.font_pack()),
            text("Value").size(13).color(p.muted_foreground),
            slider(self.min..=self.max, self.value, Message::ValueChanged),
            row![
                Button::text(
                    if self.animated {
                        "Animation on"
                    } else {
                        "Animation off"
                    },
                    theme,
                )
                .variant(ButtonVariant::Outline)
                .on_press(Message::ToggleAnimated),
                Button::text(
                    if self.auto_tone {
                        "Auto tone on"
                    } else {
                        "Auto tone off"
                    },
                    theme,
                )
                .variant(ButtonVariant::Outline)
                .on_press(Message::ToggleAutoTone),
                Button::text("Fill to max", theme)
                    .variant(ButtonVariant::Outline)
                    .on_press(Message::FillTokens),
            ]
            .spacing(8)
            .wrap(),
        ]
        .spacing(8)
        .max_width(320);

        let tokens = tokens_demo(self, theme);

        let percentages = column![
            labeled_meter(
                "0%",
                Meter::new(theme)
                    .value(0.0)
                    .animated(self.animated)
                    .width(Length::Fixed(320.0)),
            ),
            labeled_meter(
                "25%",
                Meter::new(theme)
                    .value(25.0)
                    .animated(self.animated)
                    .width(Length::Fixed(320.0)),
            ),
            labeled_meter(
                "50%",
                Meter::new(theme)
                    .value(50.0)
                    .animated(self.animated)
                    .width(Length::Fixed(320.0)),
            ),
            labeled_meter(
                "75%",
                Meter::new(theme)
                    .value(75.0)
                    .animated(self.animated)
                    .width(Length::Fixed(320.0)),
            ),
            labeled_meter(
                "100%",
                Meter::new(theme)
                    .value(100.0)
                    .animated(self.animated)
                    .width(Length::Fixed(320.0)),
            ),
        ]
        .spacing(12);

        let colors = row![
            labeled_meter(
                "primary",
                Meter::new(theme)
                    .value(self.value)
                    .max(self.max)
                    .min(self.min)
                    .theme_primary()
                    .animated(self.animated)
                    .width(Length::Fixed(180.0)),
            ),
            labeled_meter(
                "blue",
                Meter::new(theme)
                    .value(self.value)
                    .max(self.max)
                    .min(self.min)
                    .color(AccentColor::Blue)
                    .animated(self.animated)
                    .width(Length::Fixed(180.0)),
            ),
            labeled_meter(
                "warning",
                Meter::new(theme)
                    .value(self.value)
                    .max(self.max)
                    .min(self.min)
                    .tone(MeterFillTone::Warning)
                    .animated(self.animated)
                    .width(Length::Fixed(180.0)),
            ),
            labeled_meter(
                "danger",
                Meter::new(theme)
                    .value(self.value)
                    .max(self.max)
                    .min(self.min)
                    .tone(MeterFillTone::Danger)
                    .animated(self.animated)
                    .width(Length::Fixed(180.0)),
            ),
        ]
        .spacing(20)
        .align_y(Alignment::Center)
        .wrap();

        let sizes = row![
            labeled_meter(
                "sm",
                Meter::new(theme)
                    .value(60.0)
                    .size(MeterSize::Sm)
                    .width(Length::Fixed(160.0)),
            ),
            labeled_meter(
                "default (h-2)",
                Meter::new(theme)
                    .value(60.0)
                    .size(MeterSize::Default)
                    .width(Length::Fixed(160.0)),
            ),
            labeled_meter(
                "lg",
                Meter::new(theme)
                    .value(60.0)
                    .size(MeterSize::Lg)
                    .width(Length::Fixed(160.0)),
            ),
        ]
        .spacing(20)
        .align_y(Alignment::Center)
        .wrap();

        let orientations = row![
            labeled_meter(
                "horizontal",
                Meter::new(theme)
                    .value(self.value)
                    .max(self.max)
                    .auto_tone(self.auto_tone)
                    .animated(self.animated)
                    .width(Length::Fixed(220.0)),
            ),
            column![
                text("vertical").size(11).color(p.muted_foreground),
                Meter::new(theme)
                    .value(self.value)
                    .max(self.max)
                    .auto_tone(self.auto_tone)
                    .animated(self.animated)
                    .orientation(MeterOrientation::Vertical)
                    .height(Length::Fixed(140.0)),
            ]
            .spacing(8)
            .align_x(Alignment::Center),
        ]
        .spacing(28)
        .align_y(Alignment::Center);

        let content = column![
            text("Meter")
                .size(30)
                .font(iced_font(theme.font_pack().heading))
                .color(p.foreground),
            text(
                "No Meter style variants in extras — pick Style (e.g. Rhea) for pack fonts / Button chrome; Base / Accent / Mode retint Meter primary and destructive."
            )
            .size(14)
            .font(iced_font(theme.font_pack().sans))
            .color(p.muted_foreground),
            row![controls, column![tokens,].spacing(16).padding(8),]
                .spacing(32)
                .align_y(Alignment::Start),
            section_heading("Pack primary (theme.style_id)", theme),
            labeled_meter(
                "theme primary @ 66%",
                Meter::new(theme)
                    .value(66.0)
                    .theme_primary()
                    .animated(self.animated)
                    .width(Length::Fixed(320.0)),
            ),
            section_heading("Percentage gallery", theme),
            percentages,
            section_heading("Colors", theme),
            colors,
            section_heading("Sizes", theme),
            sizes,
            section_heading("Orientation", theme),
            orientations,
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

    fn current_meter<'a>(&self, theme: &'a Theme) -> Meter<'a> {
        Meter::new(theme)
            .value(self.value)
            .min(self.min)
            .max(self.max)
            .auto_tone(self.auto_tone)
            .animated(self.animated)
            .color(AccentColor::Blue)
            .transition_duration(if self.animated {
                Duration::from_millis(2500)
            } else {
                Duration::from_millis(1)
            })
    }
}

fn tokens_demo<'a>(example: &Example, theme: &'a Theme) -> Element<'a, Message> {
    let p = &theme.palette;
    let label = meter_value_label(example.current_meter(theme).config());

    column![
        section_heading("Tokens (extras demo)", theme),
        container(
            column![
                row![
                    text("Tokens").size(14).color(p.foreground),
                    text(label).size(14).color(p.muted_foreground),
                ]
                .spacing(12)
                .align_y(Alignment::Center)
                .width(Length::Fixed(200.0)),
                example
                    .current_meter(theme)
                    .width(Length::Fixed(200.0))
                    .auto_tone(example.auto_tone),
            ]
            .spacing(8),
        )
        .padding(16)
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
    .spacing(8)
    .into()
}

fn labeled_meter<'a>(label: &'static str, meter: Meter<'a>) -> Element<'a, Message> {
    column![text(label).size(11), meter].spacing(6).into()
}

fn section_heading<'a>(label: &'static str, theme: &'a Theme) -> Element<'a, Message> {
    text(label)
        .size(18)
        .font(iced_font(theme.font_pack().heading))
        .color(theme.palette.muted_foreground)
        .into()
}

fn section_label<'a>(label: &'static str, color: Color, fonts: FontPack) -> Element<'a, Message> {
    text(label)
        .size(14)
        .font(iced_font(fonts.heading))
        .color(color)
        .into()
}

fn control_select<'a, T, F>(
    label: &'static str,
    options: &'static [T],
    selected: Option<T>,
    on_select: F,
    theme: &'a Theme,
) -> Element<'a, Message>
where
    T: Clone + Eq + fmt::Display + 'static,
    F: Fn(T) -> Message + 'a,
{
    let p = &theme.palette;
    column![
        text(label).size(12).color(p.muted_foreground),
        pick_list(options, selected, on_select).width(Length::Fill),
    ]
    .spacing(4)
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

const ACCENTS: [AccentOpt; 6] = [
    AccentOpt::None,
    AccentOpt::Color(AccentColor::Blue),
    AccentOpt::Color(AccentColor::Orange),
    AccentOpt::Color(AccentColor::Green),
    AccentOpt::Color(AccentColor::Red),
    AccentOpt::Color(AccentColor::Violet),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokens_meter_uses_blue_and_auto_tone() {
        let example = Example::default();
        let meter = example.current_meter(&example.theme);
        let debug = format!("{meter:?}");
        assert!(debug.contains("auto_tone: true"), "{debug}");
        assert!(debug.contains("color: Some(Blue)"), "{debug}");
    }
}
