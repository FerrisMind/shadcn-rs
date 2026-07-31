//! Interactive playground for `iced-shadcn-v2::RangeCalendar`.
//!
//! Run: `cargo run -p iced-shadcn-v2 --example range_calendar`

use std::fmt;

use iced::widget::{column, container, pick_list, row, scrollable, text};
use iced::{Alignment, Background, Border, Color, Element, Length, Task};

use iced_shadcn_v2::{
    AccentColor, BaseColor, CalendarCaptionLayout, DateParts, DateRange, FontHeading, FontId,
    FontPack, RadiusId, RangeCalendar, StyleId, Theme, ThemeMode, fonts, iced_font,
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

#[derive(Debug, Clone)]
struct CalState {
    month: DateParts,
    range: DateRange,
}

impl CalState {
    fn new(month: DateParts) -> Self {
        Self {
            month,
            range: DateRange::default(),
        }
    }
}

struct Example {
    theme: Theme,
    today: DateParts,
    basic: CalState,
    two_months: CalState,
    bounded: CalState,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
enum Message {
    Style(Labelled<StyleId>),
    Base(Labelled<BaseColor>),
    Accent(AccentOpt),
    Mode(Labelled<ThemeMode>),
    Font(Labelled<FontId>),
    Heading(Labelled<FontHeading>),
    Radius(Labelled<RadiusId>),
    Range(Section, DateRange),
    Month(Section, DateParts),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Section {
    Basic,
    TwoMonths,
    Bounded,
}

impl Default for Example {
    fn default() -> Self {
        let today = calendar_today_utc();
        let month = DateParts { day: 1, ..today };

        Self {
            theme: Theme::light(),
            today,
            basic: CalState::new(month),
            two_months: CalState::new(month),
            bounded: CalState::new(month),
        }
    }
}

impl Example {
    fn title(&self) -> String {
        "iced-shadcn-v2 RangeCalendar".to_owned()
    }

    fn state_mut(&mut self, section: Section) -> &mut CalState {
        match section {
            Section::Basic => &mut self.basic,
            Section::TwoMonths => &mut self.two_months,
            Section::Bounded => &mut self.bounded,
        }
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
            Message::Range(section, range) => self.state_mut(section).range = range,
            Message::Month(section, month) => self.state_mut(section).month = month,
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
                "Radius",
                &RADII,
                Some(Labelled(theme.radius_id())),
                Message::Radius,
                theme
            ),
        ]
        .spacing(8);

        let basic = self.demo(
            Section::Basic,
            &self.basic,
            RangeCalendar::new(theme)
                .value(self.basic.range)
                .bordered(true),
            format!("basic · {}", describe_range(&self.basic.range)),
        );

        let two_months = self.demo(
            Section::TwoMonths,
            &self.two_months,
            RangeCalendar::new(theme)
                .value(self.two_months.range)
                .number_of_months(2)
                .paged_navigation(true)
                .caption_layout(CalendarCaptionLayout::Dropdown)
                .bordered(true),
            format!(
                "2 months · paged · dropdown · {}",
                describe_range(&self.two_months.range)
            ),
        );

        let bounds_min = DateParts {
            day: 5,
            ..self.today
        };
        let bounds_max = DateParts {
            day: 26,
            ..self.today
        };
        let bounded = self.demo(
            Section::Bounded,
            &self.bounded,
            RangeCalendar::new(theme)
                .value(self.bounded.range)
                .min_value(bounds_min)
                .max_value(bounds_max)
                .min_days(3)
                .max_days(10)
                .week_starts_on(1)
                .is_date_unavailable(|date| date.day == 13)
                .bordered(true),
            "min 5 · max 26 · minDays 3 · maxDays 10 · 13 unavail · Mon start".to_owned(),
        );

        let content = column![
            text("iced-shadcn-v2 RangeCalendar")
                .size(32)
                .font(iced_font(theme.font_pack().heading))
                .color(p.foreground),
            text("Port of shadcn-svelte range-calendar (bits-ui RangeCalendar)")
                .size(14)
                .font(iced_font(theme.font_pack().sans))
                .color(p.muted_foreground),
            controls,
            section_label(
                "Basic range selection",
                p.muted_foreground,
                theme.font_pack()
            ),
            basic,
            section_label(
                "Two months + dropdown",
                p.muted_foreground,
                theme.font_pack()
            ),
            two_months,
            section_label(
                "Bounded + constraints",
                p.muted_foreground,
                theme.font_pack()
            ),
            bounded,
        ]
        .spacing(16)
        .max_width(1100)
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

    fn demo<'a>(
        &self,
        section: Section,
        state: &CalState,
        calendar: RangeCalendar<'a, Message>,
        status: String,
    ) -> Element<'a, Message> {
        let theme = &self.theme;
        column![
            container(
                calendar
                    .placeholder(state.month)
                    .on_value_change(move |range| Message::Range(section, range))
                    .on_placeholder_change(move |month| Message::Month(section, month)),
            )
            .width(Length::Shrink),
            text(status)
                .size(12)
                .font(iced_font(theme.font_pack().mono))
                .color(theme.palette.muted_foreground),
        ]
        .spacing(8)
        .into()
    }
}

fn describe_range(range: &DateRange) -> String {
    match (range.start, range.end) {
        (Some(s), Some(e)) => format!(
            "{:04}-{:02}-{:02} \u{2192} {:04}-{:02}-{:02} ({} days)",
            s.year,
            s.month,
            s.day,
            e.year,
            e.month,
            e.day,
            range.days()
        ),
        (Some(s), None) => format!(
            "{:04}-{:02}-{:02} \u{2192} \u{2026}",
            s.year, s.month, s.day
        ),
        _ => "none".to_owned(),
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
                    radius: 6.0.into()
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
const ACCENTS: [AccentOpt; 5] = [
    AccentOpt::None,
    AccentOpt::Color(AccentColor::Blue),
    AccentOpt::Color(AccentColor::Emerald),
    AccentOpt::Color(AccentColor::Rose),
    AccentOpt::Color(AccentColor::Violet),
];
const MODES: [Labelled<ThemeMode>; 2] = [Labelled(ThemeMode::Light), Labelled(ThemeMode::Dark)];
#[allow(dead_code)]
const FONTS: [Labelled<FontId>; 3] = [
    Labelled(FontId::Geist),
    Labelled(FontId::Inter),
    Labelled(FontId::JetBrainsMono),
];
#[allow(dead_code)]
const HEADINGS: [Labelled<FontHeading>; 3] = [
    Labelled(FontHeading::Inherit),
    Labelled(FontHeading::Font(FontId::Geist)),
    Labelled(FontHeading::Font(FontId::InstrumentSerif)),
];
const RADII: [Labelled<RadiusId>; 5] = [
    Labelled(RadiusId::Default),
    Labelled(RadiusId::None),
    Labelled(RadiusId::Small),
    Labelled(RadiusId::Medium),
    Labelled(RadiusId::Large),
];
