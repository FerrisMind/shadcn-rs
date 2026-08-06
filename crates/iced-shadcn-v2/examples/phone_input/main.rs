//! Interactive playground for `iced-shadcn-v2::PhoneInput`.
//!
//! Mirrors the shadcn-svelte-extras Phone Input demos (basic, default country,
//! default value, custom ordering) in the same layout style as the `button`
//! example.
//!
//! Run: `cargo run -p iced-shadcn-v2 --example phone_input`

use std::cmp::Ordering;
use std::fmt;

use iced::widget::{column, container, pick_list, row, scrollable, text};
use iced::{Alignment, Background, Border, Color, Element, Length, Task};

use iced_shadcn_v2::{
    AccentColor, BaseColor, CountryCode, Field, FieldLabel, FontHeading, FontId, FontPack,
    PhoneCountry, PhoneInput, PhoneInputChange, StyleId, Theme, ThemeMode, default_country_order,
    fonts, iced_font,
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
    value: String,
    country: Option<CountryCode>,
    valid: bool,
    open: bool,
    query: String,
    disabled: bool,
    readonly: bool,
    order_mode: OrderMode,
    default_country_demo: Option<CountryCode>,
    default_value: String,
    default_value_country: Option<CountryCode>,
    custom_value: String,
    custom_country: Option<CountryCode>,
    custom_open: bool,
    custom_query: String,
}

#[derive(Debug, Clone)]
enum Message {
    Style(Labelled<StyleId>),
    Base(Labelled<BaseColor>),
    Accent(AccentOpt),
    Mode(Labelled<ThemeMode>),
    Font(Labelled<FontId>),
    Heading(Labelled<FontHeading>),
    Changed(PhoneInputChange),
    Open(bool),
    Query(String),
    DefaultCountryChanged(PhoneInputChange),
    DefaultValueChanged(PhoneInputChange),
    CustomChanged(PhoneInputChange),
    CustomOpen(bool),
    CustomQuery(String),
    ToggleDisabled,
    ToggleReadonly,
    Order(OrderMode),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum OrderMode {
    #[default]
    Alphabetical,
    UsCnFirst,
}

impl fmt::Display for OrderMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Alphabetical => f.write_str("alphabetical"),
            Self::UsCnFirst => f.write_str("US / CN first"),
        }
    }
}

fn us_cn_first(a: &PhoneCountry, b: &PhoneCountry) -> Ordering {
    for preferred in ["US", "CN"] {
        match (a.iso2.as_str() == preferred, b.iso2.as_str() == preferred) {
            (true, false) => return Ordering::Less,
            (false, true) => return Ordering::Greater,
            _ => {}
        }
    }
    default_country_order(a, b)
}

impl Default for Example {
    fn default() -> Self {
        let us = CountryCode::parse("US").ok();
        Self {
            theme: Theme::light(),
            value: String::new(),
            country: None,
            valid: false,
            open: false,
            query: String::new(),
            disabled: false,
            readonly: false,
            order_mode: OrderMode::Alphabetical,
            default_country_demo: us,
            default_value: "+1 418 543 8090".to_owned(),
            default_value_country: us,
            custom_value: String::new(),
            custom_country: None,
            custom_open: false,
            custom_query: String::new(),
        }
    }
}

impl Example {
    fn title(&self) -> String {
        "iced-shadcn-v2 PhoneInput".to_owned()
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
            Message::Changed(change) => self.apply_basic(change),
            Message::Open(open) => self.open = open,
            Message::Query(query) => self.query = query,
            Message::DefaultCountryChanged(change) => {
                self.default_country_demo = change.country;
            }
            Message::DefaultValueChanged(change) => {
                self.default_value = change.value;
                self.default_value_country = change.country;
            }
            Message::CustomChanged(change) => {
                self.custom_value = change.value;
                self.custom_country = change.country;
                if let Some(open) = change.open {
                    self.custom_open = open;
                }
            }
            Message::CustomOpen(open) => self.custom_open = open,
            Message::CustomQuery(query) => self.custom_query = query,
            Message::ToggleDisabled => self.disabled = !self.disabled,
            Message::ToggleReadonly => self.readonly = !self.readonly,
            Message::Order(mode) => self.order_mode = mode,
        }

        Task::none()
    }

    fn apply_basic(&mut self, change: PhoneInputChange) {
        self.value = change.value;
        self.country = change.country;
        self.valid = change.valid;
        if let Some(open) = change.open {
            self.open = open;
        }
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
                "Order",
                &ORDERS,
                Some(self.order_mode),
                Message::Order,
                theme,
            ),
        ]
        .spacing(8);

        let basic = Field::new(theme)
            .push(FieldLabel::text("Phone Number", theme))
            .push(
                PhoneInput::new(theme)
                    .value(&self.value)
                    .country(self.country)
                    .open(self.open)
                    .query(&self.query)
                    .placeholder("Enter a phone number")
                    .disabled(self.disabled)
                    .readonly(self.readonly)
                    .order(match self.order_mode {
                        OrderMode::Alphabetical => default_country_order,
                        OrderMode::UsCnFirst => us_cn_first,
                    })
                    .width(Length::Fixed(360.0))
                    .on_change(Message::Changed)
                    .on_open_change(Message::Open)
                    .on_query_change(Message::Query),
            );

        let default_country = Field::new(theme)
            .push(FieldLabel::text("Default Country (US)", theme))
            .push(
                PhoneInput::new(theme)
                    .country(self.default_country_demo)
                    .default_country(CountryCode::parse("US").ok())
                    .placeholder("Enter a phone number")
                    .width(Length::Fixed(360.0))
                    .on_change(Message::DefaultCountryChanged),
            );

        let default_value = Field::new(theme)
            .push(FieldLabel::text("Default Value", theme))
            .push(
                PhoneInput::new(theme)
                    .value(&self.default_value)
                    .country(self.default_value_country)
                    .placeholder("Enter a phone number")
                    .width(Length::Fixed(360.0))
                    .on_change(Message::DefaultValueChanged),
            );

        let custom = Field::new(theme)
            .push(FieldLabel::text("Custom Country Ordering", theme))
            .push(
                PhoneInput::new(theme)
                    .value(&self.custom_value)
                    .country(self.custom_country)
                    .open(self.custom_open)
                    .query(&self.custom_query)
                    .placeholder("Enter a phone number")
                    .order(us_cn_first)
                    .width(Length::Fixed(360.0))
                    .on_change(Message::CustomChanged)
                    .on_open_change(Message::CustomOpen)
                    .on_query_change(Message::CustomQuery),
            );

        let states = row![
            iced_shadcn_v2::Button::text(if self.disabled { "Enable" } else { "Disable" }, theme,)
                .on_press(Message::ToggleDisabled),
            iced_shadcn_v2::Button::text(
                if self.readonly {
                    "Editable"
                } else {
                    "Readonly"
                },
                theme,
            )
            .variant(iced_shadcn_v2::ButtonVariant::Outline)
            .on_press(Message::ToggleReadonly),
        ]
        .spacing(12);

        let status = text(format!(
            "value={} · country={} · valid={}",
            if self.value.is_empty() {
                "\"\""
            } else {
                &self.value
            },
            self.country
                .map(|c| c.to_string())
                .unwrap_or_else(|| "null".to_owned()),
            self.valid,
        ))
        .size(13)
        .font(iced_font(theme.font_pack().mono))
        .color(p.muted_foreground);

        let content = column![
            text("iced-shadcn-v2 PhoneInput")
                .size(32)
                .font(iced_font(theme.font_pack().heading))
                .color(p.foreground),
            text("shadcn-svelte-extras phone input · country selector + E.164 field")
                .size(14)
                .font(iced_font(theme.font_pack().sans))
                .color(p.muted_foreground),
            controls,
            section_label("Basic", p.muted_foreground, theme.font_pack()),
            basic,
            status,
            states,
            section_label("Default Country", p.muted_foreground, theme.font_pack()),
            default_country,
            section_label("Default Value", p.muted_foreground, theme.font_pack()),
            default_value,
            section_label("Custom Ordering", p.muted_foreground, theme.font_pack()),
            custom,
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

const ACCENTS: [AccentOpt; 5] = [
    AccentOpt::None,
    AccentOpt::Color(AccentColor::Blue),
    AccentOpt::Color(AccentColor::Amber),
    AccentOpt::Color(AccentColor::Emerald),
    AccentOpt::Color(AccentColor::Rose),
];

const MODES: [Labelled<ThemeMode>; 2] = [Labelled(ThemeMode::Light), Labelled(ThemeMode::Dark)];

const FONTS: [Labelled<FontId>; 3] = [
    Labelled(FontId::Geist),
    Labelled(FontId::Inter),
    Labelled(FontId::JetBrainsMono),
];

const HEADINGS: [Labelled<FontHeading>; 3] = [
    Labelled(FontHeading::Inherit),
    Labelled(FontHeading::Font(FontId::Geist)),
    Labelled(FontHeading::Font(FontId::InstrumentSerif)),
];

const ORDERS: [OrderMode; 2] = [OrderMode::Alphabetical, OrderMode::UsCnFirst];
