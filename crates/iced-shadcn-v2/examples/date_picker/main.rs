//! Interactive playground for `iced-shadcn-v2::DatePicker` / `DateRangePicker`
//! with full shadcn-common theme knobs (Style, Base, Accent, Mode, Font,
//! Heading, Radius).
//!
//! Run: `cargo run -p iced-shadcn-v2 --example date_picker`

use std::fmt;

use iced::widget::{column, container, pick_list, row, scrollable, text};
use iced::{Alignment, Background, Border, Color, Element, Length, Task};

use iced_shadcn_v2::{
    AccentColor, BaseColor, CalendarCaptionLayout, DateParts, DatePicker, DateRange,
    DateRangePicker, FontHeading, FontId, FontPack, RadiusId, StyleId, Theme, ThemeMode, fonts,
    iced_font,
};
use shadcn_common::calendar_today_utc;

pub fn main() -> iced::Result {
    let mut app = iced::application(Example::default, Example::update, Example::view)
        .title(Example::title)
        .default_font(iced_font(FontId::Geist));

    for face in fonts::ALL_FACES {
        app = app.font(*face);
    }

    app.run()
}

#[allow(dead_code)]
struct Example {
    theme: Theme,
    today: DateParts,
    // Single picker
    date: Option<DateParts>,
    picker_open: bool,
    picker_month: DateParts,
    // Dropdown caption picker
    date_dropdown: Option<DateParts>,
    dropdown_open: bool,
    dropdown_month: DateParts,
    // Range picker
    range: DateRange,
    range_open: bool,
    range_month: DateParts,
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
    // Single
    DateChanged(Option<DateParts>),
    PickerOpen(bool),
    PickerMonth(DateParts),
    // Dropdown
    DateDropdownChanged(Option<DateParts>),
    DropdownOpen(bool),
    DropdownMonth(DateParts),
    // Range
    RangeChanged(DateRange),
    RangeOpen(bool),
    RangeMonth(DateParts),
}

impl Default for Example {
    fn default() -> Self {
        let today = calendar_today_utc();
        let month = DateParts { day: 1, ..today };
        Self {
            theme: Theme::light(),
            today,
            date: Some(today),
            picker_open: false,
            picker_month: month,
            date_dropdown: None,
            dropdown_open: false,
            dropdown_month: month,
            range: DateRange::default(),
            range_open: false,
            range_month: month,
        }
    }
}

impl Example {
    fn title(&self) -> String {
        "iced-shadcn-v2 DatePicker".to_owned()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Style(s) => self.theme = self.theme.clone().with_style(s.0),
            Message::Base(b) => self.theme = self.theme.clone().with_base(b.0),
            Message::Accent(a) => self.theme = self.theme.clone().with_accent(a.into_option()),
            Message::Mode(m) => self.theme = self.theme.clone().with_mode(m.0),
            Message::Font(f) => self.theme = self.theme.clone().with_font(f.0),
            Message::Heading(h) => self.theme = self.theme.clone().with_font_heading(h.0),
            Message::Radius(r) => self.theme = self.theme.clone().with_radius(r.0),
            Message::DateChanged(date) => {
                self.date = date;
                if date.is_some() {
                    self.picker_open = false;
                }
            }
            Message::PickerOpen(open) => self.picker_open = open,
            Message::PickerMonth(month) => self.picker_month = month,
            Message::DateDropdownChanged(date) => self.date_dropdown = date,
            Message::DropdownOpen(open) => self.dropdown_open = open,
            Message::DropdownMonth(month) => self.dropdown_month = month,
            Message::RangeChanged(range) => {
                if range.is_complete() {
                    self.range_open = false;
                }
                self.range = range;
            }
            Message::RangeOpen(open) => self.range_open = open,
            Message::RangeMonth(month) => self.range_month = month,
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
                theme
            ),
            control_select(
                "Base",
                &BASES,
                Some(Labelled(theme.base())),
                Message::Base,
                theme
            ),
            control_select(
                "Accent",
                &ACCENTS,
                Some(AccentOpt::from_option(theme.accent())),
                Message::Accent,
                theme
            ),
            control_select(
                "Mode",
                &MODES,
                Some(Labelled(theme.mode())),
                Message::Mode,
                theme
            ),
            control_select(
                "Heading",
                &HEADINGS,
                Some(Labelled(theme.font_heading())),
                Message::Heading,
                theme
            ),
            control_select(
                "Font",
                &FONTS,
                Some(Labelled(theme.font_id())),
                Message::Font,
                theme
            ),
            control_select(
                "Radius",
                &RADII,
                Some(Labelled(theme.radius_id())),
                Message::Radius,
                theme
            ),
        ]
        .spacing(8);

        let simple = column![
            section_label(
                "Simple date picker (close on select)",
                p.muted_foreground,
                theme.font_pack()
            ),
            DatePicker::new(theme)
                .value(self.date)
                .open(self.picker_open)
                .placeholder(self.picker_month)
                .on_value_change(Message::DateChanged)
                .on_open_change(Message::PickerOpen)
                .on_placeholder_change(Message::PickerMonth),
            text(format!(
                "selected: {}",
                self.date
                    .map(shadcn_common::format_date_long)
                    .unwrap_or_else(|| "none".to_owned())
            ))
            .size(12)
            .font(iced_font(theme.font_pack().mono))
            .color(p.muted_foreground),
        ]
        .spacing(8);

        let dropdown = column![
            section_label(
                "Date picker with dropdown caption",
                p.muted_foreground,
                theme.font_pack()
            ),
            DatePicker::new(theme)
                .value(self.date_dropdown)
                .open(self.dropdown_open)
                .placeholder(self.dropdown_month)
                .caption_layout(CalendarCaptionLayout::Dropdown)
                .close_on_select(false)
                .on_value_change(Message::DateDropdownChanged)
                .on_open_change(Message::DropdownOpen)
                .on_placeholder_change(Message::DropdownMonth),
        ]
        .spacing(8);

        let range = column![
            section_label(
                "Date range picker (2 months)",
                p.muted_foreground,
                theme.font_pack()
            ),
            DateRangePicker::new(theme)
                .value(self.range)
                .open(self.range_open)
                .placeholder(self.range_month)
                .number_of_months(2)
                .on_value_change(Message::RangeChanged)
                .on_open_change(Message::RangeOpen)
                .on_placeholder_change(Message::RangeMonth),
            text(format!(
                "range: {}",
                shadcn_common::format_date_range(&self.range, "none")
            ))
            .size(12)
            .font(iced_font(theme.font_pack().mono))
            .color(p.muted_foreground),
        ]
        .spacing(8);

        let disabled = column![
            section_label("Disabled", p.muted_foreground, theme.font_pack()),
            row![
                DatePicker::<'_, Message>::new(theme).disabled(true),
                DateRangePicker::<'_, Message>::new(theme).disabled(true),
            ]
            .spacing(12),
        ]
        .spacing(8);

        let content = column![
            text("iced-shadcn-v2 DatePicker")
                .size(32)
                .font(iced_font(theme.font_pack().heading))
                .color(p.foreground),
            text("Composition of Popover + Calendar / RangeCalendar")
                .size(14)
                .font(iced_font(theme.font_pack().sans))
                .color(p.muted_foreground),
            controls,
            simple,
            dropdown,
            range,
            disabled,
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

// ── Theme option types ───────────────────────────────────────────────────────

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
    const fn from_option(a: Option<AccentColor>) -> Self {
        match a {
            None => Self::None,
            Some(c) => Self::Color(c),
        }
    }
    const fn into_option(self) -> Option<AccentColor> {
        match self {
            Self::None => None,
            Self::Color(c) => Some(c),
        }
    }
}
impl fmt::Display for AccentOpt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => f.write_str("none"),
            Self::Color(c) => f.write_str(c.as_str()),
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
