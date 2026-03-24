use chrono::{Datelike, Duration, Months, NaiveDate};
use iced::widget::{column, container, row, text};
use iced::{Alignment, Element, Length, alignment};

use crate::button::{ButtonProps, ButtonSize, ButtonVariant, button_content};
use crate::theme::Theme;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CalendarMode {
    Single,
    Range,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CalendarView {
    Month,
    Year,
    Decade,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum CalendarCaptionLayout {
    #[default]
    Label,
    Dropdown,
}

#[derive(Clone, Copy, Debug)]
pub struct CalendarState {
    pub current_month: NaiveDate,
}

impl CalendarState {
    pub fn new(current_month: NaiveDate) -> Self {
        Self {
            current_month: normalize_month(current_month),
        }
    }
}

impl Default for CalendarState {
    fn default() -> Self {
        Self::new(fallback_month())
    }
}

#[derive(Clone, Debug)]
pub enum CalendarAction {
    MonthChanged(NaiveDate),
    Selected(Option<NaiveDate>),
    RangeSelected(Option<NaiveDate>, Option<NaiveDate>),
}

pub struct CalendarProps<Id> {
    pub id_source: Id,
    pub selected: Option<NaiveDate>,
    pub range_start: Option<NaiveDate>,
    pub range_end: Option<NaiveDate>,
    pub mode: CalendarMode,
    pub caption_layout: CalendarCaptionLayout,
    pub number_of_months: usize,
    pub default_month: Option<NaiveDate>,
    pub min_date: Option<NaiveDate>,
    pub max_date: Option<NaiveDate>,
    pub disabled_dates: Vec<NaiveDate>,
}

impl<Id> CalendarProps<Id> {
    pub fn new(id_source: Id) -> Self {
        Self {
            id_source,
            selected: None,
            range_start: None,
            range_end: None,
            mode: CalendarMode::Single,
            caption_layout: CalendarCaptionLayout::Label,
            number_of_months: 1,
            default_month: None,
            min_date: None,
            max_date: None,
            disabled_dates: Vec::new(),
        }
    }

    pub fn caption_layout(mut self, layout: CalendarCaptionLayout) -> Self {
        self.caption_layout = layout;
        self
    }

    pub fn selected(mut self, date: Option<NaiveDate>) -> Self {
        self.selected = date;
        self
    }

    pub fn range_start(mut self, date: Option<NaiveDate>) -> Self {
        self.range_start = date;
        self
    }

    pub fn range_end(mut self, date: Option<NaiveDate>) -> Self {
        self.range_end = date;
        self
    }

    pub fn mode(mut self, mode: CalendarMode) -> Self {
        self.mode = mode;
        self
    }

    pub fn number_of_months(mut self, months: usize) -> Self {
        self.number_of_months = months.max(1);
        self
    }

    pub fn default_month(mut self, date: NaiveDate) -> Self {
        self.default_month = Some(date);
        self
    }

    pub fn min_date(mut self, date: Option<NaiveDate>) -> Self {
        self.min_date = date;
        self
    }

    pub fn max_date(mut self, date: Option<NaiveDate>) -> Self {
        self.max_date = date;
        self
    }

    pub fn disabled_dates(mut self, dates: Vec<NaiveDate>) -> Self {
        self.disabled_dates = dates;
        self
    }
}

const MONTH_LABELS: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

pub fn calendar<'a, Message: Clone + 'a, Id, F>(
    props: CalendarProps<Id>,
    state: CalendarState,
    on_action: Option<F>,
    theme: &'a Theme,
) -> Element<'a, Message>
where
    F: Fn(CalendarAction) -> Message + 'a,
{
    let on_action = on_action.as_ref();
    let months_count = props.number_of_months.max(1);
    let min_month = props.min_date.map(normalize_month);
    let max_month = props.max_date.map(normalize_month);

    let mut base_month = normalize_month(props.default_month.unwrap_or(state.current_month));
    if let Some(min) = min_month
        && base_month < min
    {
        base_month = min;
    }
    if let Some(max) = max_month
        && base_month > max
    {
        base_month = max;
    }

    let last_visible_month = add_months(base_month, (months_count - 1) as u32);
    let prev_disabled = min_month.map(|min| base_month <= min).unwrap_or(false);
    let next_disabled = max_month
        .map(|max| last_visible_month >= max)
        .unwrap_or(false);

    let prev_month = sub_month(base_month);
    let next_month = add_months(base_month, 1);

    let prev_press = on_action
        .map(|f| f(CalendarAction::MonthChanged(prev_month)))
        .filter(|_| !prev_disabled);
    let next_press = on_action
        .map(|f| f(CalendarAction::MonthChanged(next_month)))
        .filter(|_| !next_disabled);

    let nav_label = format!("{} {}", month_label(base_month.month()), base_month.year());
    let nav = row![
        button_content(
            text("‹"),
            prev_press,
            ButtonProps::new()
                .variant(ButtonVariant::Ghost)
                .size(ButtonSize::Size1)
                .disabled(prev_disabled || on_action.is_none()),
            theme,
        ),
        text(nav_label).size(13),
        button_content(
            text("›"),
            next_press,
            ButtonProps::new()
                .variant(ButtonVariant::Ghost)
                .size(ButtonSize::Size1)
                .disabled(next_disabled || on_action.is_none()),
            theme,
        )
    ]
    .spacing(8)
    .align_y(Alignment::Center);

    let mut months: Vec<Element<'a, Message>> = Vec::new();
    for offset in 0..months_count {
        let month = add_months(base_month, offset as u32);
        months.push(month_view(month, &props, on_action, theme));
    }

    let months_row = row(months).spacing(16).align_y(Alignment::Start);

    column![nav, months_row].spacing(12).into()
}

fn month_view<'a, Message: Clone + 'a, Id, F>(
    month: NaiveDate,
    props: &CalendarProps<Id>,
    on_action: Option<&F>,
    theme: &'a Theme,
) -> Element<'a, Message>
where
    F: Fn(CalendarAction) -> Message + 'a,
{
    let month_label = format!("{} {}", month_label(month.month()), month.year());
    let header = text(month_label).size(12);

    let weekday_row = row(["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]
        .iter()
        .map(|label| {
            container(
                text(*label)
                    .size(11)
                    .style(move |_t| iced::widget::text::Style {
                        color: Some(theme.palette.muted_foreground),
                    }),
            )
            .width(Length::Fixed(32.0))
            .height(Length::Fixed(16.0))
            .align_x(alignment::Horizontal::Center)
            .align_y(alignment::Vertical::Center)
            .into()
        })
        .collect::<Vec<Element<'a, Message>>>())
    .spacing(4)
    .align_y(Alignment::Center);

    let first_day = NaiveDate::from_ymd_opt(month.year(), month.month(), 1).unwrap_or(month);
    let start_offset = first_day.weekday().number_from_monday().saturating_sub(1) as usize;
    let days_in_month = days_in_month(first_day);

    let mut day = 1u32;
    let mut week_rows: Vec<Element<'a, Message>> = Vec::new();
    for _ in 0..6 {
        let mut cells: Vec<Element<'a, Message>> = Vec::new();
        for weekday in 0..7 {
            let cell_index = week_rows.len() * 7 + weekday;
            if cell_index < start_offset || day > days_in_month {
                cells.push(empty_day_cell());
                continue;
            }

            let date =
                NaiveDate::from_ymd_opt(month.year(), month.month(), day).unwrap_or(first_day);
            day += 1;
            cells.push(day_cell(date, props, on_action, theme));
        }
        week_rows.push(row(cells).spacing(4).align_y(Alignment::Center).into());
    }

    column![header, weekday_row, column(week_rows).spacing(4)]
        .spacing(8)
        .into()
}

fn day_cell<'a, Message: Clone + 'a, Id, F>(
    date: NaiveDate,
    props: &CalendarProps<Id>,
    on_action: Option<&F>,
    theme: &'a Theme,
) -> Element<'a, Message>
where
    F: Fn(CalendarAction) -> Message + 'a,
{
    let disabled = is_date_disabled(date, props.min_date, props.max_date, &props.disabled_dates);

    let (variant, on_press, is_selected_surface) = match props.mode {
        CalendarMode::Single => {
            let is_selected = props.selected == Some(date);
            let next_selected = if is_selected { None } else { Some(date) };
            let variant = if is_selected {
                ButtonVariant::Solid
            } else {
                ButtonVariant::Ghost
            };
            let on_press = on_action
                .map(|f| f(CalendarAction::Selected(next_selected)))
                .filter(|_| !disabled);
            (variant, on_press, is_selected)
        }
        CalendarMode::Range => {
            let (start, end) = next_range(props.range_start, props.range_end, date);
            let is_start = props.range_start == Some(date);
            let is_end = props.range_end == Some(date);
            let is_between = is_in_range(date, props.range_start, props.range_end);
            let variant = if is_start || is_end {
                ButtonVariant::Solid
            } else if is_between {
                ButtonVariant::Soft
            } else {
                ButtonVariant::Ghost
            };
            let on_press = on_action
                .map(|f| f(CalendarAction::RangeSelected(start, end)))
                .filter(|_| !disabled);
            (variant, on_press, is_start || is_end)
        }
    };

    let day_text_color = if disabled {
        theme.palette.muted_foreground
    } else if is_selected_surface {
        theme.palette.primary_foreground
    } else {
        theme.palette.foreground
    };

    let button = button_content(
        text(date.day().to_string())
            .size(12)
            .style(move |_t| iced::widget::text::Style {
                color: Some(day_text_color),
            }),
        on_press,
        ButtonProps::new()
            .variant(variant)
            .size(ButtonSize::Size0)
            .disabled(disabled || on_action.is_none()),
        theme,
    );

    container(button)
        .width(Length::Fixed(32.0))
        .height(Length::Fixed(32.0))
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
}

fn empty_day_cell<'a, Message: Clone + 'a>() -> Element<'a, Message> {
    container(text(""))
        .width(Length::Fixed(32.0))
        .height(Length::Fixed(32.0))
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
}

fn is_date_disabled(
    date: NaiveDate,
    min_date: Option<NaiveDate>,
    max_date: Option<NaiveDate>,
    disabled_dates: &[NaiveDate],
) -> bool {
    if let Some(min) = min_date
        && date < min
    {
        return true;
    }
    if let Some(max) = max_date
        && date > max
    {
        return true;
    }
    disabled_dates.contains(&date)
}

fn next_range(
    start: Option<NaiveDate>,
    end: Option<NaiveDate>,
    clicked: NaiveDate,
) -> (Option<NaiveDate>, Option<NaiveDate>) {
    match (start, end) {
        (None, _) => (Some(clicked), None),
        (Some(current_start), None) => {
            if clicked < current_start {
                (Some(clicked), Some(current_start))
            } else {
                (Some(current_start), Some(clicked))
            }
        }
        (Some(_), Some(_)) => (Some(clicked), None),
    }
}

fn is_in_range(date: NaiveDate, start: Option<NaiveDate>, end: Option<NaiveDate>) -> bool {
    if let (Some(start), Some(end)) = (start, end) {
        date > start && date < end
    } else {
        false
    }
}

fn month_label(month: u32) -> &'static str {
    MONTH_LABELS
        .get(month.saturating_sub(1) as usize)
        .copied()
        .unwrap_or("")
}

fn normalize_month(date: NaiveDate) -> NaiveDate {
    date.with_day(1).unwrap_or(date)
}

fn days_in_month(date: NaiveDate) -> u32 {
    let next_month = if date.month() == 12 {
        NaiveDate::from_ymd_opt(date.year() + 1, 1, 1).unwrap_or(date)
    } else {
        NaiveDate::from_ymd_opt(date.year(), date.month() + 1, 1).unwrap_or(date)
    };
    let last_day = next_month - Duration::days(1);
    last_day.day()
}

fn add_months(date: NaiveDate, offset: u32) -> NaiveDate {
    if offset == 0 {
        return date;
    }
    date.checked_add_months(Months::new(offset))
        .unwrap_or(date)
        .with_day(1)
        .unwrap_or(date)
}

fn sub_month(date: NaiveDate) -> NaiveDate {
    date.checked_sub_months(Months::new(1))
        .unwrap_or(date)
        .with_day(1)
        .unwrap_or(date)
}

fn fallback_month() -> NaiveDate {
    NaiveDate::from_ymd_opt(2024, 1, 1)
        .unwrap_or_else(|| NaiveDate::from_ymd_opt(2023, 1, 1).unwrap())
}
