//! Interactive playground for `iced-shadcn-v2::StarRating` + `shadcn-common` theme knobs.
//!
//! Mirrors the shadcn-svelte-extras Star Rating demos (basic, custom stars, half,
//! disabled, readonly, custom color, custom size) in the same layout style as the
//! `button` / `spinner` examples.
//!
//! Run: `cargo run -p iced-shadcn-v2 --example star_rating`

use std::fmt;

use iced::keyboard::{self, Key};
use iced::widget::{column, container, pick_list, row, scrollable, text};
use iced::{Alignment, Background, Border, Color, Element, Event, Length, Subscription, Task};

use iced_shadcn_v2::{
    AccentColor, BaseColor, Button, ButtonVariant, Direction, FontId, StarRating, StarRatingKey,
    StarRatingOrientation, StarRatingSize, StyleId, Theme, ThemeMode, fonts, iced_font,
    star_rating,
};

pub fn main() -> iced::Result {
    let mut app = iced::application(Example::default, Example::update, Example::view)
        .title(Example::title)
        .subscription(Example::subscription)
        .default_font(iced_font(FontId::Geist));

    for face in fonts::ALL_FACES {
        app = app.font(*face);
    }

    app.run()
}

struct Example {
    theme: Theme,
    value: f32,
    max: f32,
    allow_half: bool,
    disabled: bool,
    readonly: bool,
    hover_preview: bool,
    focused: bool,
    orientation: StarRatingOrientation,
    direction: Direction,
    star_size: StarRatingSize,
    color: ColorOpt,
    custom_stars_value: f32,
    half_value: f32,
}

#[derive(Debug, Clone)]
enum Message {
    Style(Labelled<StyleId>),
    Base(Labelled<BaseColor>),
    Accent(AccentOpt),
    Mode(Labelled<ThemeMode>),
    Rated(f32),
    CustomStarsRated(f32),
    HalfRated(f32),
    Max(MaxOpt),
    Size(StarRatingSize),
    Color(ColorOpt),
    Orientation(StarRatingOrientation),
    Direction(DirectionOpt),
    ToggleAllowHalf,
    ToggleDisabled,
    ToggleReadonly,
    ToggleHoverPreview,
    ToggleFocused,
    Key(StarRatingKey),
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::light(),
            value: 0.0,
            max: 5.0,
            allow_half: false,
            disabled: false,
            readonly: false,
            hover_preview: true,
            focused: false,
            orientation: StarRatingOrientation::Horizontal,
            direction: Direction::Ltr,
            star_size: StarRatingSize::Default,
            color: ColorOpt::Primary,
            custom_stars_value: 0.0,
            half_value: 3.5,
        }
    }
}

impl Example {
    fn title(&self) -> String {
        "iced-shadcn-v2 StarRating".to_owned()
    }

    fn subscription(&self) -> Subscription<Message> {
        iced::event::listen_with(|event, _status, _id| match event {
            Event::Keyboard(keyboard::Event::KeyPressed { key, .. }) => map_key(key),
            _ => None,
        })
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
            Message::Rated(value) => self.value = value,
            Message::CustomStarsRated(value) => self.custom_stars_value = value,
            Message::HalfRated(value) => self.half_value = value,
            Message::Max(max) => {
                self.max = max.0 as f32;
                self.value = self.value.min(self.max);
            }
            Message::Size(size) => self.star_size = size,
            Message::Color(color) => self.color = color,
            Message::Orientation(orientation) => self.orientation = orientation,
            Message::Direction(direction) => self.direction = direction.0,
            Message::ToggleAllowHalf => {
                self.allow_half = !self.allow_half;
            }
            Message::ToggleDisabled => {
                self.disabled = !self.disabled;
            }
            Message::ToggleReadonly => {
                self.readonly = !self.readonly;
            }
            Message::ToggleHoverPreview => {
                self.hover_preview = !self.hover_preview;
            }
            Message::ToggleFocused => {
                self.focused = !self.focused;
            }
            Message::Key(key) => {
                let next = self.interactive_rating().apply_key(key);
                if let Some(next) = next {
                    self.value = next;
                }
            }
        }

        Task::none()
    }

    fn interactive_rating(&self) -> StarRating<'_, Message> {
        StarRating::new(&self.theme)
            .value(self.value)
            .max(self.max)
            .allow_half(self.allow_half)
            .disabled(self.disabled)
            .readonly(self.readonly)
            .hover_preview(self.hover_preview)
            .focused(self.focused)
            .orientation(self.orientation)
            .direction(self.direction)
            .star_size(self.star_size)
            .color(self.color.resolve(&self.theme))
            .on_change(Message::Rated)
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
            section_label("StarRating knobs", p.muted_foreground, theme),
            control_select(
                "Max",
                &MAXES,
                Some(MaxOpt(self.max as u32)),
                Message::Max,
                theme
            ),
            control_select("Size", &SIZES, Some(self.star_size), Message::Size, theme),
            control_select("Color", &COLORS, Some(self.color), Message::Color, theme),
            control_select(
                "Orient",
                &ORIENTATIONS,
                Some(self.orientation),
                Message::Orientation,
                theme,
            ),
            control_select(
                "Dir",
                &DIRECTIONS,
                Some(DirectionOpt(self.direction)),
                Message::Direction,
                theme,
            ),
            row![
                toggle_button(
                    "allowHalf",
                    self.allow_half,
                    Message::ToggleAllowHalf,
                    theme
                ),
                toggle_button("disabled", self.disabled, Message::ToggleDisabled, theme),
                toggle_button("readonly", self.readonly, Message::ToggleReadonly, theme),
                toggle_button(
                    "hoverPreview",
                    self.hover_preview,
                    Message::ToggleHoverPreview,
                    theme
                ),
                toggle_button("focused", self.focused, Message::ToggleFocused, theme),
            ]
            .spacing(8)
            .wrap(),
        ]
        .spacing(8);

        let preview = column![
            section_label("Preview (basic)", p.muted_foreground, theme),
            container(
                column![
                    star_rating(self.interactive_rating()),
                    text(format!("Rating is {}", format_rating(self.value)))
                        .size(13)
                        .font(iced_font(theme.font_pack().sans))
                        .color(p.muted_foreground),
                ]
                .spacing(8)
                .align_x(Alignment::Center)
            )
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
        ]
        .spacing(8);

        let custom_stars = demo_block(
            "Custom Stars (max=10)",
            star_rating(
                StarRating::new(theme)
                    .value(self.custom_stars_value)
                    .max(10.0)
                    .star_size(self.star_size)
                    .color(self.color.resolve(theme))
                    .on_change(Message::CustomStarsRated),
            )
            .into(),
            None,
            theme,
        );

        let half = demo_block(
            "Half Rating + RTL",
            column![
                star_rating(
                    StarRating::new(theme)
                        .value(self.half_value)
                        .allow_half(true)
                        .star_size(self.star_size)
                        .color(self.color.resolve(theme))
                        .on_change(Message::HalfRated),
                ),
                text(format!("Rating is {}", format_rating(self.half_value)))
                    .size(13)
                    .font(iced_font(theme.font_pack().sans))
                    .color(p.muted_foreground),
                text("تقييم بالنجوم (RTL)")
                    .size(13)
                    .font(iced_font(theme.font_pack().sans))
                    .color(p.muted_foreground),
                star_rating(
                    StarRating::new(theme)
                        .value(3.5)
                        .allow_half(true)
                        .direction(Direction::Rtl)
                        .readonly(true)
                        .star_size(self.star_size)
                        .color(self.color.resolve(theme)),
                ),
            ]
            .spacing(8)
            .align_x(Alignment::Center)
            .into(),
            None,
            theme,
        );

        let disabled = demo_block(
            "Disabled",
            star_rating(
                StarRating::new(theme)
                    .value(3.0)
                    .disabled(true)
                    .star_size(self.star_size)
                    .color(self.color.resolve(theme)),
            )
            .into(),
            None,
            theme,
        );

        let readonly = demo_block(
            "Readonly",
            star_rating(
                StarRating::new(theme)
                    .value(2.0)
                    .readonly(true)
                    .star_size(self.star_size)
                    .color(self.color.resolve(theme)),
            )
            .into(),
            None,
            theme,
        );

        // Tailwind yellow-400 ≈ oklch(0.852 0.199 91.936) from the extras demo.
        let yellow = Color::from_rgb(0.984, 0.749, 0.141);
        let custom_color = demo_block(
            "Custom Color (yellow-400)",
            star_rating(
                StarRating::new(theme)
                    .value(self.value)
                    .allow_half(self.allow_half)
                    .color(yellow)
                    .star_size(self.star_size)
                    .on_change(Message::Rated),
            )
            .into(),
            None,
            theme,
        );

        let custom_size = demo_block(
            "Custom Size (size-10)",
            star_rating(
                StarRating::new(theme)
                    .value(self.value)
                    .allow_half(self.allow_half)
                    .star_size(StarRatingSize::Xl)
                    .color(self.color.resolve(theme))
                    .on_change(Message::Rated),
            )
            .into(),
            None,
            theme,
        );

        let content = column![
            text("iced-shadcn-v2 StarRating")
                .size(32)
                .font(iced_font(theme.font_pack().heading))
                .color(p.foreground),
            text("Port of shadcn-svelte-extras StarRating / bits-ui RatingGroup")
                .size(14)
                .font(iced_font(theme.font_pack().sans))
                .color(p.muted_foreground),
            controls,
            preview,
            custom_stars,
            half,
            disabled,
            readonly,
            custom_color,
            custom_size,
            text("Keyboard: arrows / Home / End / digits (when focused via Toggle)")
                .size(12)
                .font(iced_font(theme.font_pack().mono))
                .color(p.muted_foreground),
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

fn demo_block<'a>(
    title: &'static str,
    body: Element<'a, Message>,
    _caption: Option<&'static str>,
    theme: &'a Theme,
) -> Element<'a, Message> {
    let p = theme.palette;
    column![
        section_label(title, p.muted_foreground, theme),
        container(body)
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
    ]
    .spacing(8)
    .into()
}

fn toggle_button<'a>(
    label: &'static str,
    on: bool,
    message: Message,
    theme: &'a Theme,
) -> Element<'a, Message> {
    Button::text(
        if on {
            format!("{label}: on")
        } else {
            format!("{label}: off")
        },
        theme,
    )
    .variant(if on {
        ButtonVariant::Default
    } else {
        ButtonVariant::Outline
    })
    .on_press(message)
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

fn format_rating(value: f32) -> String {
    if (value.fract()).abs() < f32::EPSILON {
        format!("{:.0}", value)
    } else {
        format!("{value}")
    }
}

fn map_key(key: Key) -> Option<Message> {
    match key {
        Key::Named(keyboard::key::Named::ArrowLeft) => Some(Message::Key(StarRatingKey::ArrowLeft)),
        Key::Named(keyboard::key::Named::ArrowRight) => {
            Some(Message::Key(StarRatingKey::ArrowRight))
        }
        Key::Named(keyboard::key::Named::ArrowUp) => Some(Message::Key(StarRatingKey::ArrowUp)),
        Key::Named(keyboard::key::Named::ArrowDown) => Some(Message::Key(StarRatingKey::ArrowDown)),
        Key::Named(keyboard::key::Named::Home) => Some(Message::Key(StarRatingKey::Home)),
        Key::Named(keyboard::key::Named::End) => Some(Message::Key(StarRatingKey::End)),
        Key::Named(keyboard::key::Named::PageUp) => Some(Message::Key(StarRatingKey::PageUp)),
        Key::Named(keyboard::key::Named::PageDown) => Some(Message::Key(StarRatingKey::PageDown)),
        Key::Character(c) => {
            let digit = c.chars().next()?.to_digit(10)? as u8;
            Some(Message::Key(StarRatingKey::Digit(digit)))
        }
        _ => None,
    }
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
struct MaxOpt(u32);

impl fmt::Display for MaxOpt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DirectionOpt(Direction);

impl fmt::Display for DirectionOpt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            Direction::Rtl => f.write_str("rtl"),
            Direction::Ltr | _ => f.write_str("ltr"),
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

const MAXES: [MaxOpt; 4] = [MaxOpt(3), MaxOpt(5), MaxOpt(7), MaxOpt(10)];

const SIZES: [StarRatingSize; 5] = [
    StarRatingSize::Sm,
    StarRatingSize::Default,
    StarRatingSize::Md,
    StarRatingSize::Lg,
    StarRatingSize::Xl,
];

const COLORS: [ColorOpt; 5] = [
    ColorOpt::Primary,
    ColorOpt::Foreground,
    ColorOpt::Muted,
    ColorOpt::Destructive,
    ColorOpt::Accent,
];

const ORIENTATIONS: [StarRatingOrientation; 2] = [
    StarRatingOrientation::Horizontal,
    StarRatingOrientation::Vertical,
];

const DIRECTIONS: [DirectionOpt; 2] = [DirectionOpt(Direction::Ltr), DirectionOpt(Direction::Rtl)];
