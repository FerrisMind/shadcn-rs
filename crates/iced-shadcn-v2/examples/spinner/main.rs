//! Interactive playground for `iced-shadcn-v2::Spinner` + `shadcn-common` theme knobs.
//!
//! Run: `cargo run -p iced-shadcn-v2 --example spinner`

use std::fmt;
use std::time::Duration;

use iced::widget::{column, container, pick_list, row, scrollable, text};
use iced::{Alignment, Background, Border, Color, Element, Length, Task};

use iced_shadcn_v2::{
    AccentColor, BaseColor, Button, ButtonVariant, FontId, Spinner, SpinnerSize, SpinnerVariant,
    StyleId, Theme, ThemeMode, fonts, iced_font, spinner,
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
    variant: SpinnerVariant,
    size: SizeOpt,
    color: ColorOpt,
    duration_ms: u32,
    animated: bool,
    loading: bool,
}

#[derive(Debug, Clone)]
enum Message {
    Style(Labelled<StyleId>),
    Base(Labelled<BaseColor>),
    Accent(AccentOpt),
    Mode(Labelled<ThemeMode>),
    Variant(LabelledVariant),
    Size(SizeOpt),
    Color(ColorOpt),
    Duration(DurationOpt),
    ToggleAnimated,
    ToggleLoading,
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::light(),
            variant: SpinnerVariant::AiLoaderIcon,
            size: SizeOpt::Default,
            color: ColorOpt::Primary,
            duration_ms: 1000,
            animated: true,
            loading: true,
        }
    }
}

impl Example {
    fn title(&self) -> String {
        "iced-shadcn-v2 Spinner".to_owned()
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
            Message::Variant(variant) => {
                self.variant = variant.0;
            }
            Message::Size(size) => {
                self.size = size;
            }
            Message::Color(color) => {
                self.color = color;
            }
            Message::Duration(duration) => {
                self.duration_ms = duration.0;
            }
            Message::ToggleAnimated => {
                self.animated = !self.animated;
            }
            Message::ToggleLoading => {
                self.loading = !self.loading;
            }
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let p = &theme.palette;
        let color = self.color.resolve(theme);

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
            section_label("Spinner knobs", p.muted_foreground, theme),
            control_select(
                "Variant",
                &VARIANTS,
                Some(LabelledVariant(self.variant)),
                Message::Variant,
                theme,
            ),
            control_select("Size", &SIZES, Some(self.size), Message::Size, theme),
            control_select("Color", &COLORS, Some(self.color), Message::Color, theme),
            control_select(
                "Duration",
                &DURATIONS,
                Some(DurationOpt(self.duration_ms)),
                Message::Duration,
                theme,
            ),
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
                    if self.loading {
                        "Loading on"
                    } else {
                        "Loading off"
                    },
                    theme,
                )
                .variant(ButtonVariant::Outline)
                .on_press(Message::ToggleLoading),
            ]
            .spacing(12)
            .wrap(),
        ]
        .spacing(8);

        let preview = column![
            section_label("Preview", p.muted_foreground, theme),
            container(spinner(self.build_spinner(theme, color, self.variant)))
                .padding(24)
                .center_x(Length::Fill)
                .style(move |_| container::Style {
                    background: Some(Background::Color(p.card)),
                    border: Border {
                        color: p.border,
                        width: 1.0,
                        radius: 8.0.into(),
                    },
                    ..container::Style::default()
                }),
            text(format!(
                "{} · {}px · {}ms · animated={} · loading={}",
                LabelledVariant(self.variant),
                self.size.pixels(),
                self.duration_ms,
                self.animated,
                self.loading,
            ))
            .size(12)
            .font(iced_font(theme.font_pack().mono))
            .color(p.muted_foreground),
        ]
        .spacing(8);

        let sizes = row![
            labelled_spinner(
                "xs",
                self.build_spinner(theme, color, self.variant)
                    .size(SpinnerSize::Xs),
                theme
            ),
            labelled_spinner(
                "sm",
                self.build_spinner(theme, color, self.variant)
                    .size(SpinnerSize::Sm),
                theme
            ),
            labelled_spinner(
                "default",
                self.build_spinner(theme, color, self.variant)
                    .size(SpinnerSize::Default),
                theme,
            ),
            labelled_spinner(
                "lg",
                self.build_spinner(theme, color, self.variant)
                    .size(SpinnerSize::Lg),
                theme
            ),
            labelled_spinner(
                "xl",
                self.build_spinner(theme, color, self.variant)
                    .size(SpinnerSize::Xl),
                theme
            ),
            labelled_spinner(
                "custom",
                self.build_spinner(theme, color, self.variant)
                    .size(SpinnerSize::Custom(28.0)),
                theme,
            ),
        ]
        .spacing(24)
        .align_y(Alignment::Center)
        .wrap();

        let colors = row![
            labelled_spinner(
                "primary",
                self.build_spinner(theme, p.primary, self.variant),
                theme,
            ),
            labelled_spinner(
                "fg",
                self.build_spinner(theme, p.foreground, self.variant),
                theme,
            ),
            labelled_spinner(
                "muted",
                self.build_spinner(theme, p.muted_foreground, self.variant),
                theme,
            ),
            labelled_spinner(
                "destructive",
                self.build_spinner(theme, p.destructive, self.variant),
                theme,
            ),
            labelled_spinner(
                "accent",
                self.build_spinner(theme, p.accent, self.variant),
                theme,
            ),
        ]
        .spacing(24)
        .align_y(Alignment::Center)
        .wrap();

        let mut gallery = row![].spacing(20).align_y(Alignment::Center);
        for labelled in VARIANTS {
            gallery = gallery.push(labelled_spinner(
                labelled.to_string(),
                self.build_spinner(theme, color, labelled.0),
                theme,
            ));
        }
        let gallery = gallery.wrap();

        let in_button = row![
            Button::text("Save", theme)
                .variant(ButtonVariant::Default)
                .loading(self.loading)
                .on_press(Message::ToggleLoading),
            Button::text("Outline", theme)
                .variant(ButtonVariant::Outline)
                .loading(self.loading)
                .on_press(Message::ToggleLoading),
        ]
        .spacing(12)
        .align_y(Alignment::Center);

        let content = column![
            text("iced-shadcn-v2 Spinner")
                .size(32)
                .font(iced_font(theme.font_pack().heading))
                .color(p.foreground),
            text("Canvas loaders driven by Theme palette colors and SpinnerVariant")
                .size(14)
                .font(iced_font(theme.font_pack().sans))
                .color(p.muted_foreground),
            controls,
            preview,
            section_label("Sizes", p.muted_foreground, theme),
            sizes,
            section_label("Theme colors", p.muted_foreground, theme),
            colors,
            section_label("All variants", p.muted_foreground, theme),
            gallery,
            section_label(
                "Inside Button::loading (spinner left of label)",
                p.muted_foreground,
                theme
            ),
            in_button,
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

    fn build_spinner(&self, theme: &Theme, color: Color, variant: SpinnerVariant) -> Spinner {
        Spinner::new(theme)
            .variant(variant)
            .size(self.size.to_spinner_size())
            .color(color)
            .duration(Duration::from_millis(u64::from(self.duration_ms)))
            .animated(self.animated)
            .loading(self.loading)
    }
}

fn labelled_spinner<'a>(
    label: impl Into<String>,
    indicator: Spinner,
    theme: &'a Theme,
) -> Element<'a, Message> {
    let p = theme.palette;
    column![
        spinner(indicator),
        text(label.into())
            .size(11)
            .font(iced_font(theme.font_pack().mono))
            .color(p.muted_foreground),
    ]
    .spacing(6)
    .align_x(Alignment::Center)
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
struct LabelledVariant(SpinnerVariant);

impl fmt::Display for LabelledVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self.0 {
            SpinnerVariant::LegacyLucide => "legacy-lucide",
            SpinnerVariant::AiLoaderIcon => "ai-loader",
            SpinnerVariant::Circular => "circular",
            SpinnerVariant::Classic => "classic",
            SpinnerVariant::Pulse => "pulse",
            SpinnerVariant::PulseDot => "pulse-dot",
            SpinnerVariant::Dots => "dots",
            SpinnerVariant::Typing => "typing",
            SpinnerVariant::Wave => "wave",
            SpinnerVariant::Bars => "bars",
            SpinnerVariant::Terminal => "terminal",
            SpinnerVariant::TextBlink => "text-blink",
            SpinnerVariant::TextShimmer => "text-shimmer",
            SpinnerVariant::LoadingDots => "loading-dots",
            // `SpinnerVariant` is `#[non_exhaustive]`.
            _ => "unknown",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SizeOpt {
    Xs,
    Sm,
    Default,
    Lg,
    Xl,
    Custom28,
}

impl SizeOpt {
    const fn to_spinner_size(self) -> SpinnerSize {
        match self {
            Self::Xs => SpinnerSize::Xs,
            Self::Sm => SpinnerSize::Sm,
            Self::Default => SpinnerSize::Default,
            Self::Lg => SpinnerSize::Lg,
            Self::Xl => SpinnerSize::Xl,
            Self::Custom28 => SpinnerSize::Custom(28.0),
        }
    }

    const fn pixels(self) -> f32 {
        match self {
            Self::Xs => 12.0,
            Self::Sm | Self::Default => 16.0,
            Self::Lg => 24.0,
            Self::Xl => 32.0,
            Self::Custom28 => 28.0,
        }
    }
}

impl fmt::Display for SizeOpt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Xs => f.write_str("xs (12px)"),
            Self::Sm => f.write_str("sm (16px)"),
            Self::Default => f.write_str("default (16px)"),
            Self::Lg => f.write_str("lg (24px)"),
            Self::Xl => f.write_str("xl (32px)"),
            Self::Custom28 => f.write_str("custom (28px)"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ColorOpt {
    Primary,
    Foreground,
    Muted,
    Destructive,
    Accent,
}

impl ColorOpt {
    fn resolve(self, theme: &Theme) -> Color {
        let p = &theme.palette;
        match self {
            Self::Primary => p.primary,
            Self::Foreground => p.foreground,
            Self::Muted => p.muted_foreground,
            Self::Destructive => p.destructive,
            Self::Accent => p.accent,
        }
    }
}

impl fmt::Display for ColorOpt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Primary => f.write_str("primary"),
            Self::Foreground => f.write_str("foreground"),
            Self::Muted => f.write_str("muted"),
            Self::Destructive => f.write_str("destructive"),
            Self::Accent => f.write_str("accent"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DurationOpt(u32);

impl fmt::Display for DurationOpt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ms", self.0)
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
    AccentOpt::Color(AccentColor::Amber),
    AccentOpt::Color(AccentColor::Emerald),
    AccentOpt::Color(AccentColor::Rose),
    AccentOpt::Color(AccentColor::Violet),
];

const MODES: [Labelled<ThemeMode>; 2] = [Labelled(ThemeMode::Light), Labelled(ThemeMode::Dark)];

const VARIANTS: [LabelledVariant; 14] = [
    LabelledVariant(SpinnerVariant::LegacyLucide),
    LabelledVariant(SpinnerVariant::AiLoaderIcon),
    LabelledVariant(SpinnerVariant::Circular),
    LabelledVariant(SpinnerVariant::Classic),
    LabelledVariant(SpinnerVariant::Pulse),
    LabelledVariant(SpinnerVariant::PulseDot),
    LabelledVariant(SpinnerVariant::Dots),
    LabelledVariant(SpinnerVariant::Typing),
    LabelledVariant(SpinnerVariant::Wave),
    LabelledVariant(SpinnerVariant::Bars),
    LabelledVariant(SpinnerVariant::Terminal),
    LabelledVariant(SpinnerVariant::TextBlink),
    LabelledVariant(SpinnerVariant::TextShimmer),
    LabelledVariant(SpinnerVariant::LoadingDots),
];

const SIZES: [SizeOpt; 6] = [
    SizeOpt::Xs,
    SizeOpt::Sm,
    SizeOpt::Default,
    SizeOpt::Lg,
    SizeOpt::Xl,
    SizeOpt::Custom28,
];

const COLORS: [ColorOpt; 5] = [
    ColorOpt::Primary,
    ColorOpt::Foreground,
    ColorOpt::Muted,
    ColorOpt::Destructive,
    ColorOpt::Accent,
];

const DURATIONS: [DurationOpt; 5] = [
    DurationOpt(500),
    DurationOpt(800),
    DurationOpt(1000),
    DurationOpt(1500),
    DurationOpt(2000),
];
