//! Interactive playground for `iced-shadcn-v2::Calendar` + `shadcn-common` theme knobs.
//!
//! Mirrors the button example structure: the theme selects use
//! `iced::widget::pick_list` because v2 does not depend on `iced-shadcn` v1.
//!
//! Run: `cargo run -p iced-shadcn-v2 --example calendar`

use std::fmt;

use iced::widget::{column, container, pick_list, row, scrollable, text};
use iced::{Alignment, Background, Border, Color, Element, Length, Task};

use iced_shadcn_v2::{
    AccentColor, BaseColor, ButtonVariant, Calendar, CalendarCaptionLayout, CalendarSelection,
    DateParts, FontHeading, FontId, FontPack, RadiusId, StyleId, Theme, ThemeMode, fonts,
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

/// Month + selection pair owned by the app for one calendar, mirroring
/// `bind:placeholder` / `bind:value`.
#[derive(Debug, Clone)]
struct CalendarState {
    month: DateParts,
    selection: CalendarSelection,
}

impl CalendarState {
    fn new(month: DateParts, selection: CalendarSelection) -> Self {
        Self { month, selection }
    }
}

struct Example {
    theme: Theme,
    today: DateParts,
    single: CalendarState,
    dropdown: CalendarState,
    dropdown_months: CalendarState,
    dropdown_years: CalendarState,
    multiple: CalendarState,
    two_months: CalendarState,
    bounded: CalendarState,
    paged: bool,
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
    Single(Section, CalendarSelection),
    Month(Section, DateParts),
    TogglePaged,
}

/// Which playground calendar produced a message.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Section {
    Single,
    Dropdown,
    DropdownMonths,
    DropdownYears,
    Multiple,
    TwoMonths,
    Bounded,
}

impl Default for Example {
    fn default() -> Self {
        let today = calendar_today_utc();
        let month = DateParts { day: 1, ..today };
        let empty = || CalendarState::new(month, CalendarSelection::single(None));

        Self {
            theme: Theme::light(),
            today,
            single: CalendarState::new(month, CalendarSelection::single(Some(today))),
            dropdown: empty(),
            dropdown_months: empty(),
            dropdown_years: empty(),
            multiple: CalendarState::new(month, CalendarSelection::multiple([])),
            two_months: empty(),
            bounded: empty(),
            paged: false,
        }
    }
}

impl Example {
    fn title(&self) -> String {
        "iced-shadcn-v2 Calendar".to_owned()
    }

    fn state_mut(&mut self, section: Section) -> &mut CalendarState {
        match section {
            Section::Single => &mut self.single,
            Section::Dropdown => &mut self.dropdown,
            Section::DropdownMonths => &mut self.dropdown_months,
            Section::DropdownYears => &mut self.dropdown_years,
            Section::Multiple => &mut self.multiple,
            Section::TwoMonths => &mut self.two_months,
            Section::Bounded => &mut self.bounded,
        }
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
            Message::Single(section, selection) => {
                self.state_mut(section).selection = selection;
            }
            Message::Month(section, month) => {
                self.state_mut(section).month = month;
            }
            Message::TogglePaged => {
                self.paged = !self.paged;
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
        ]
        .spacing(8);

        let single = self.demo(
            Section::Single,
            &self.single,
            Calendar::new(theme)
                .selection(self.single.selection.clone())
                .bordered(true),
            format!(
                "single · selected: {}",
                describe_selection(&self.single.selection)
            ),
        );

        let dropdown = self.demo(
            Section::Dropdown,
            &self.dropdown,
            Calendar::new(theme)
                .selection(self.dropdown.selection.clone())
                .caption_layout(CalendarCaptionLayout::Dropdown)
                .bordered(true),
            "captionLayout: dropdown".to_owned(),
        );
        let dropdown_months = self.demo(
            Section::DropdownMonths,
            &self.dropdown_months,
            Calendar::new(theme)
                .selection(self.dropdown_months.selection.clone())
                .caption_layout(CalendarCaptionLayout::DropdownMonths)
                .bordered(true),
            "captionLayout: dropdown-months".to_owned(),
        );
        let dropdown_years = self.demo(
            Section::DropdownYears,
            &self.dropdown_years,
            Calendar::new(theme)
                .selection(self.dropdown_years.selection.clone())
                .caption_layout(CalendarCaptionLayout::DropdownYears)
                .bordered(true),
            "captionLayout: dropdown-years".to_owned(),
        );

        let multiple = self.demo(
            Section::Multiple,
            &self.multiple,
            Calendar::new(theme)
                .selection(self.multiple.selection.clone())
                .max_days(5)
                .fixed_weeks(true)
                .bordered(true),
            format!(
                "multiple · maxDays 5 · fixedWeeks · {} picked",
                self.multiple.selection.len()
            ),
        );

        let two_months = self.demo(
            Section::TwoMonths,
            &self.two_months,
            Calendar::new(theme)
                .selection(self.two_months.selection.clone())
                .number_of_months(2)
                .paged_navigation(self.paged)
                .bordered(true),
            format!("numberOfMonths 2 · pagedNavigation {}", self.paged),
        );
        let paged_toggle = iced_shadcn_v2::Button::text(
            if self.paged {
                "Disable paged navigation"
            } else {
                "Enable paged navigation"
            },
            theme,
        )
        .variant(ButtonVariant::Outline)
        .on_press(Message::TogglePaged);

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
            Calendar::new(theme)
                .selection(self.bounded.selection.clone())
                .min_value(bounds_min)
                .max_value(bounds_max)
                .is_date_unavailable(|date| date.day == 13 || date.day == 14)
                .week_starts_on(1)
                .bordered(true),
            "min day 5 · max day 26 · 13/14 unavailable · week starts Monday".to_owned(),
        );

        let states = row![
            labelled_calendar(
                "disabled",
                Calendar::<'_, Message>::new(theme)
                    .selected(self.today)
                    .placeholder(self.single.month)
                    .disabled(true)
                    .bordered(true)
                    .into(),
                theme,
            ),
            labelled_calendar(
                "readonly",
                Calendar::<'_, Message>::new(theme)
                    .selected(self.today)
                    .placeholder(self.single.month)
                    .readonly(true)
                    .on_selection_change(|selection| Message::Single(Section::Single, selection))
                    .bordered(true)
                    .into(),
                theme,
            ),
        ]
        .spacing(16)
        .wrap();

        let title_px = 32u32;

        let content = column![
            text("iced-shadcn-v2 Calendar")
                .size(title_px)
                .font(iced_font(theme.font_pack().heading))
                .color(p.foreground),
            text("Port of the shadcn-svelte calendar (bits-ui Calendar + .cn-calendar)")
                .size(14)
                .font(iced_font(theme.font_pack().sans))
                .color(p.muted_foreground),
            controls,
            section_label("Single selection", p.muted_foreground, theme.font_pack()),
            single,
            section_label("Caption layouts", p.muted_foreground, theme.font_pack()),
            row![dropdown, dropdown_months, dropdown_years]
                .spacing(16)
                .wrap(),
            section_label(
                "Multiple selection / fixed weeks",
                p.muted_foreground,
                theme.font_pack()
            ),
            multiple,
            section_label("Two months", p.muted_foreground, theme.font_pack()),
            column![two_months, paged_toggle].spacing(12),
            section_label(
                "Bounds / unavailable / week start",
                p.muted_foreground,
                theme.font_pack()
            ),
            bounded,
            section_label("States", p.muted_foreground, theme.font_pack()),
            states,
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

    /// Wires one calendar to its section state and adds a status line.
    fn demo<'a>(
        &self,
        section: Section,
        state: &CalendarState,
        calendar: Calendar<'a, Message>,
        status: String,
    ) -> Element<'a, Message> {
        let theme = &self.theme;

        column![
            container(
                calendar
                    .placeholder(state.month)
                    .on_selection_change(move |selection| Message::Single(section, selection))
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

fn describe_selection(selection: &CalendarSelection) -> String {
    match selection.as_single() {
        Some(date) => format!("{:04}-{:02}-{:02}", date.year, date.month, date.day),
        None => "none".to_owned(),
    }
}

fn labelled_calendar<'a>(
    label: &'static str,
    calendar: Element<'a, Message>,
    theme: &Theme,
) -> Element<'a, Message> {
    column![
        calendar,
        text(label)
            .size(12)
            .font(iced_font(theme.font_pack().mono))
            .color(theme.palette.muted_foreground),
    ]
    .spacing(8)
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
