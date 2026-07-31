//! Backend-agnostic calendar (month view) state and layout logic.
//!
//! Ports the pure parts of the bits-ui `Calendar` primitive plus the
//! shadcn-svelte wrapper defaults (`.cn-calendar` metrics), so egui and iced
//! backends share one behaviour layer: month grids, caption dropdown items,
//! prev/next paging, selection updates, and day-state resolution.

use crate::date_time::{
    DateParts, add_days, add_months, date_to_ordinal, month_days, start_of_month,
};
use crate::select_value::SelectMode;

/// Day/nav-button cell footprint (`--cell-size: --spacing(8)` → 32 px).
pub const CALENDAR_CELL_SIZE_PX: f32 = 32.0;
/// Root padding of `.cn-calendar` (`p-3`).
pub const CALENDAR_PADDING_PX: f32 = 12.0;
/// Gap between months and between a month header and its grid (`gap-4`).
pub const CALENDAR_MONTHS_GAP_PX: f32 = 16.0;
/// Gap between caption dropdowns in the header (`gap-1.5`).
pub const CALENDAR_HEADER_GAP_PX: f32 = 6.0;
/// Top margin of every week row (`mt-2` on `Calendar.GridRow`).
pub const CALENDAR_WEEK_ROW_GAP_PX: f32 = 8.0;
/// Weekday head-cell text size (`text-[0.8rem]`).
pub const CALENDAR_WEEKDAY_TEXT_PX: f32 = 12.8;
/// Day and caption text size (`text-sm`).
pub const CALENDAR_TEXT_PX: f32 = 14.0;
/// Prev/next chevron glyph size (`size-4`).
pub const CALENDAR_NAV_ICON_PX: f32 = 16.0;
/// Caption dropdown chevron glyph size (`size-3.5`).
pub const CALENDAR_DROPDOWN_CHEVRON_PX: f32 = 14.0;
/// Opacity applied to disabled days and nav buttons (`opacity-50`).
pub const CALENDAR_DISABLED_OPACITY: f32 = 0.5;
/// Alpha of the not-selected day hover fill (`hover:bg-accent/50`).
pub const CALENDAR_HOVER_ACCENT_ALPHA: f32 = 0.5;

/// How the month/year caption of a calendar month is rendered.
///
/// Mirrors the shadcn-svelte `captionLayout` prop.
///
/// ```rust
/// use shadcn_common::CalendarCaptionLayout;
///
/// assert_eq!(CalendarCaptionLayout::default(), CalendarCaptionLayout::Label);
/// assert!(CalendarCaptionLayout::Dropdown.has_month_dropdown());
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CalendarCaptionLayout {
    /// Static "Month Year" text (`captionLayout="label"`).
    #[default]
    Label,
    /// Month and year dropdowns (`captionLayout="dropdown"`).
    Dropdown,
    /// Month dropdown with a static year (`captionLayout="dropdown-months"`).
    DropdownMonths,
    /// Static month with a year dropdown (`captionLayout="dropdown-years"`).
    DropdownYears,
}

impl CalendarCaptionLayout {
    /// Whether the caption renders a month dropdown.
    #[must_use]
    pub const fn has_month_dropdown(self) -> bool {
        matches!(self, Self::Dropdown | Self::DropdownMonths)
    }

    /// Whether the caption renders a year dropdown.
    #[must_use]
    pub const fn has_year_dropdown(self) -> bool {
        matches!(self, Self::Dropdown | Self::DropdownYears)
    }
}

/// Weekday label style (`weekdayFormat`).
///
/// ```rust
/// use shadcn_common::{CalendarWeekdayFormat, calendar_weekday_name};
///
/// assert_eq!(calendar_weekday_name(0, CalendarWeekdayFormat::Short), "Sun");
/// assert_eq!(calendar_weekday_name(1, CalendarWeekdayFormat::Narrow), "M");
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CalendarWeekdayFormat {
    /// "Sunday", "Monday", …
    Long,
    /// "Sun", "Mon", … (shadcn-svelte wrapper default).
    #[default]
    Short,
    /// "S", "M", …
    Narrow,
}

/// Month label style (`monthFormat`).
///
/// ```rust
/// use shadcn_common::{CalendarMonthFormat, calendar_month_name};
///
/// assert_eq!(calendar_month_name(1, CalendarMonthFormat::Long), "January");
/// assert_eq!(calendar_month_name(2, CalendarMonthFormat::TwoDigit), "02");
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CalendarMonthFormat {
    /// "January", "February", … (bits-ui default).
    #[default]
    Long,
    /// "Jan", "Feb", …
    Short,
    /// "J", "F", …
    Narrow,
    /// "1", "2", …
    Numeric,
    /// "01", "02", …
    TwoDigit,
}

/// Year label style (`yearFormat`).
///
/// ```rust
/// use shadcn_common::{CalendarYearFormat, calendar_year_name};
///
/// assert_eq!(calendar_year_name(2026, CalendarYearFormat::Numeric), "2026");
/// assert_eq!(calendar_year_name(2026, CalendarYearFormat::TwoDigit), "26");
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CalendarYearFormat {
    /// "2026", "2027", … (bits-ui default).
    #[default]
    Numeric,
    /// "26", "27", …
    TwoDigit,
}

const MONTH_NAMES: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

const WEEKDAY_NAMES: [&str; 7] = [
    "Sunday",
    "Monday",
    "Tuesday",
    "Wednesday",
    "Thursday",
    "Friday",
    "Saturday",
];

/// English label for `month` (`1..=12`) in the requested format.
///
/// Out-of-range months are clamped into `1..=12`, mirroring the crate-wide
/// "clamp, don't panic" policy for display helpers.
#[must_use]
pub fn calendar_month_name(month: u8, format: CalendarMonthFormat) -> String {
    let index = usize::from(month.clamp(1, 12)) - 1;
    let name = MONTH_NAMES[index];
    match format {
        CalendarMonthFormat::Long => name.to_owned(),
        CalendarMonthFormat::Short => name[..3].to_owned(),
        CalendarMonthFormat::Narrow => name[..1].to_owned(),
        CalendarMonthFormat::Numeric => (index + 1).to_string(),
        CalendarMonthFormat::TwoDigit => format!("{:02}", index + 1),
    }
}

/// English label for `weekday` (`0 = Sunday` … `6 = Saturday`).
#[must_use]
pub fn calendar_weekday_name(weekday: u8, format: CalendarWeekdayFormat) -> String {
    let name = WEEKDAY_NAMES[usize::from(weekday % 7)];
    match format {
        CalendarWeekdayFormat::Long => name.to_owned(),
        CalendarWeekdayFormat::Short => name[..3].to_owned(),
        CalendarWeekdayFormat::Narrow => name[..1].to_owned(),
    }
}

/// Label for `year` in the requested format.
#[must_use]
pub fn calendar_year_name(year: i32, format: CalendarYearFormat) -> String {
    match format {
        CalendarYearFormat::Numeric => year.to_string(),
        CalendarYearFormat::TwoDigit => format!("{:02}", year.rem_euclid(100)),
    }
}

/// Weekday numbers of one header row starting at `week_starts_on`
/// (`0 = Sunday` … `6 = Saturday`).
#[must_use]
pub fn calendar_weekdays(week_starts_on: u8) -> [u8; 7] {
    std::array::from_fn(|index| (week_starts_on + index as u8) % 7)
}

/// Month grid for the month containing `date`.
///
/// Wraps [`month_days`] and, when `fixed_weeks` is set, pads the grid with
/// following weeks so every month renders exactly six rows (bits-ui
/// `fixedWeeks`).
#[must_use]
pub fn calendar_month_grid(
    date: DateParts,
    week_starts_on: u8,
    fixed_weeks: bool,
) -> Vec<[DateParts; 7]> {
    let mut weeks = month_days(date, week_starts_on);
    if fixed_weeks {
        while weeks.len() < 6 {
            let last = weeks
                .last()
                .map(|week| week[6])
                .unwrap_or_else(|| start_of_month(date));
            weeks.push(std::array::from_fn(|index| {
                add_days(last, index as i32 + 1)
            }));
        }
    }
    weeks
}

/// First days of the months shown at once (`numberOfMonths`).
///
/// A `number_of_months` of zero is treated as one month.
#[must_use]
pub fn calendar_visible_months(placeholder: DateParts, number_of_months: usize) -> Vec<DateParts> {
    let count = number_of_months.max(1);
    let first = start_of_month(placeholder);
    (0..count)
        .map(|offset| add_months(first, offset as i32))
        .collect()
}

/// Placeholder produced by a prev/next page navigation.
///
/// `paged_navigation` shifts by the full page (`numberOfMonths` months);
/// otherwise the view slides by a single month, matching bits-ui
/// `handleCalendarNextPage` / `handleCalendarPrevPage`.
#[must_use]
pub fn calendar_nav_target(
    first_visible: DateParts,
    forward: bool,
    paged_navigation: bool,
    number_of_months: usize,
) -> DateParts {
    let step = if paged_navigation {
        number_of_months.max(1) as i32
    } else {
        1
    };
    add_months(
        start_of_month(first_visible),
        if forward { step } else { -step },
    )
}

/// Whether the previous-page button must be disabled.
///
/// Mirrors bits-ui `getIsPrevButtonDisabled`: the month before the first
/// visible one ends before `min_value`.
#[must_use]
pub fn calendar_prev_disabled(first_visible: DateParts, min_value: Option<DateParts>) -> bool {
    let Some(min_value) = min_value else {
        return false;
    };
    let previous = add_months(start_of_month(first_visible), -1);
    let end_of_previous = add_days(add_months(previous, 1), -1);
    end_of_previous < min_value
}

/// Whether the next-page button must be disabled.
///
/// Mirrors bits-ui `getIsNextButtonDisabled`: the month after the last
/// visible one starts after `max_value`.
#[must_use]
pub fn calendar_next_disabled(last_visible: DateParts, max_value: Option<DateParts>) -> bool {
    let Some(max_value) = max_value else {
        return false;
    };
    let start_of_next = add_months(start_of_month(last_visible), 1);
    start_of_next > max_value
}

/// Default year list for the caption year dropdown.
///
/// Port of bits-ui `getDefaultYears`: `min`/`max` bound the range when set;
/// otherwise the list spans `latest - 100 ..= latest + 10` where `latest` is
/// the later of `placeholder_year` and `today_year` (extended downwards when
/// the placeholder sits before that window).
#[must_use]
pub fn calendar_default_years(
    placeholder_year: i32,
    today_year: i32,
    min_value: Option<DateParts>,
    max_value: Option<DateParts>,
) -> Vec<i32> {
    let latest = placeholder_year.max(today_year);
    let min_year = min_value.map(|value| value.year).unwrap_or_else(|| {
        let initial_min = latest - 100;
        if placeholder_year < initial_min {
            placeholder_year - 10
        } else {
            initial_min
        }
    });
    let max_year = max_value.map_or(latest + 10, |value| value.year);
    let min_year = min_year.min(max_year);
    (min_year..=max_year).collect()
}

/// Whether `date` lies inside the optional `[min, max]` selection bounds.
#[must_use]
pub fn calendar_date_in_bounds(
    date: DateParts,
    min_value: Option<DateParts>,
    max_value: Option<DateParts>,
) -> bool {
    min_value.is_none_or(|min| date >= min) && max_value.is_none_or(|max| date <= max)
}

/// Today's date in UTC, derived from the system clock.
///
/// bits-ui defaults the placeholder to "now" in the local timezone; without a
/// timezone database this helper uses UTC, which apps can override by passing
/// an explicit date.
#[must_use]
pub fn calendar_today_utc() -> DateParts {
    const UNIX_EPOCH_DATE: DateParts = DateParts {
        year: 1970,
        month: 1,
        day: 1,
    };
    let seconds = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|elapsed| elapsed.as_secs())
        .unwrap_or_default();
    let days = i32::try_from(seconds / 86_400).unwrap_or(i32::MAX);
    add_days(UNIX_EPOCH_DATE, days)
}

/// Controlled selection of a calendar (`type="single" | "multiple"`).
///
/// Mirrors the discriminated `value` prop of bits-ui `Calendar.Root`.
///
/// ```rust
/// use shadcn_common::{CalendarSelection, DateParts};
///
/// let date = DateParts::new(2026, 7, 4).expect("valid date literal");
/// let selection = CalendarSelection::single(Some(date));
/// assert!(selection.is_selected(date));
/// assert_eq!(selection.len(), 1);
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CalendarSelection {
    /// The selected date, or `None` when empty.
    Single(Option<DateParts>),
    /// The ordered selected dates.
    Multiple(Vec<DateParts>),
}

impl Default for CalendarSelection {
    fn default() -> Self {
        Self::Single(None)
    }
}

impl CalendarSelection {
    /// Creates a single-selection value.
    #[must_use]
    pub const fn single(value: Option<DateParts>) -> Self {
        Self::Single(value)
    }

    /// Creates a multiple-selection value, removing duplicates while
    /// preserving first-seen order.
    #[must_use]
    pub fn multiple(values: impl IntoIterator<Item = DateParts>) -> Self {
        let mut selected = Vec::new();
        for value in values {
            if !selected.contains(&value) {
                selected.push(value);
            }
        }
        Self::Multiple(selected)
    }

    /// Returns the selected date when this is [`Self::Single`].
    #[must_use]
    pub const fn as_single(&self) -> Option<DateParts> {
        match self {
            Self::Single(value) => *value,
            Self::Multiple(_) => None,
        }
    }

    /// Returns the selected dates when this is [`Self::Multiple`].
    #[must_use]
    pub fn as_multiple(&self) -> &[DateParts] {
        match self {
            Self::Single(_) => &[],
            Self::Multiple(values) => values,
        }
    }

    /// Whether `date` is currently selected.
    #[must_use]
    pub fn is_selected(&self, date: DateParts) -> bool {
        match self {
            Self::Single(value) => *value == Some(date),
            Self::Multiple(values) => values.contains(&date),
        }
    }

    /// Selection mode represented by this value.
    #[must_use]
    pub const fn mode(&self) -> SelectMode {
        match self {
            Self::Single(_) => SelectMode::Single,
            Self::Multiple(_) => SelectMode::Multiple,
        }
    }

    /// Number of selected dates.
    #[must_use]
    pub fn len(&self) -> usize {
        match self {
            Self::Single(value) => usize::from(value.is_some()),
            Self::Multiple(values) => values.len(),
        }
    }

    /// Whether nothing is selected.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Result of clicking a selectable day, produced by [`calendar_day_pick`].
#[derive(Debug, Clone, PartialEq, Eq)]
#[must_use = "the pick describes the next controlled state and must be applied"]
pub struct CalendarPick {
    /// Next controlled selection.
    pub selection: CalendarSelection,
    /// New placeholder, when bits-ui re-anchors the view (a deselect that
    /// empties the selection).
    pub placeholder: Option<DateParts>,
}

/// Applies a day click to `selection`, mirroring bits-ui `handleCellClick`.
///
/// Single mode replaces the value; clicking the selected date clears it
/// unless `prevent_deselect`. Multiple mode toggles membership; when adding
/// a date would exceed `max_days`, the selection resets to just the clicked
/// date (bits-ui `maxDays` behaviour). `max_days` of zero means "no limit".
pub fn calendar_day_pick(
    selection: CalendarSelection,
    date: DateParts,
    prevent_deselect: bool,
    max_days: Option<usize>,
) -> CalendarPick {
    match selection {
        CalendarSelection::Single(previous) => {
            if !prevent_deselect && previous == Some(date) {
                CalendarPick {
                    selection: CalendarSelection::Single(None),
                    placeholder: Some(date),
                }
            } else {
                CalendarPick {
                    selection: CalendarSelection::Single(Some(date)),
                    placeholder: None,
                }
            }
        }
        CalendarSelection::Multiple(mut values) => {
            if let Some(index) = values.iter().position(|value| *value == date) {
                if prevent_deselect {
                    return CalendarPick {
                        selection: CalendarSelection::Multiple(values),
                        placeholder: None,
                    };
                }
                values.remove(index);
                let placeholder = values.is_empty().then_some(date);
                CalendarPick {
                    selection: CalendarSelection::Multiple(values),
                    placeholder,
                }
            } else {
                values.push(date);
                if let Some(max_days) = max_days
                    && max_days > 0
                    && values.len() > max_days
                {
                    values = vec![date];
                }
                CalendarPick {
                    selection: CalendarSelection::Multiple(values),
                    placeholder: None,
                }
            }
        }
    }
}

/// Resolved visual state of one day cell.
///
/// Backends map these flags onto the `.cn-calendar` day styling
/// (`data-selected`, `data-today`, `data-outside-month`, `data-disabled`,
/// `data-unavailable`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CalendarDayState {
    /// The day is part of the current selection.
    pub selected: bool,
    /// The day equals "today".
    pub today: bool,
    /// The day belongs to a neighbouring month of the rendered grid.
    pub outside_month: bool,
    /// The day cannot be focused or selected.
    pub disabled: bool,
    /// The day is marked unavailable (rendered struck through).
    pub unavailable: bool,
}

impl CalendarDayState {
    /// Whether clicking the day may change the selection.
    ///
    /// Mirrors bits-ui `handleCellClick` guards: readonly calendars,
    /// disabled and unavailable dates ignore clicks, and days outside the
    /// month are inert when `disable_days_outside_month` is set.
    #[must_use]
    pub const fn is_interactive(
        self,
        readonly: bool,
        calendar_disabled: bool,
        disable_days_outside_month: bool,
    ) -> bool {
        !(readonly
            || calendar_disabled
            || self.disabled
            || self.unavailable
            || (self.outside_month && disable_days_outside_month))
    }
}

// ─── Range Calendar ───────────────────────────────────────────────────────────

/// A date range value: optional start and optional end.
///
/// Mirrors bits-ui `DateRange` (`{ start?: DateValue; end?: DateValue }`).
///
/// ```rust
/// use shadcn_common::{DateParts, DateRange};
///
/// let start = DateParts::new(2026, 7, 10).unwrap();
/// let end = DateParts::new(2026, 7, 20).unwrap();
/// let range = DateRange::new(Some(start), Some(end));
/// assert!(range.contains(DateParts::new(2026, 7, 15).unwrap()));
/// assert!(!range.contains(DateParts::new(2026, 7, 21).unwrap()));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct DateRange {
    /// Range start (first click).
    pub start: Option<DateParts>,
    /// Range end (second click).
    pub end: Option<DateParts>,
}

impl DateRange {
    /// Creates a range from optional endpoints.
    #[must_use]
    pub const fn new(start: Option<DateParts>, end: Option<DateParts>) -> Self {
        Self { start, end }
    }

    /// Whether both start and end are set.
    #[must_use]
    pub const fn is_complete(&self) -> bool {
        self.start.is_some() && self.end.is_some()
    }

    /// Whether the range is completely empty.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.start.is_none() && self.end.is_none()
    }

    /// Whether `date` lies within the closed `[start, end]` interval.
    #[must_use]
    pub fn contains(&self, date: DateParts) -> bool {
        match (self.start, self.end) {
            (Some(start), Some(end)) => date >= start && date <= end,
            _ => false,
        }
    }

    /// Number of days in the range (inclusive), or 0 when incomplete.
    #[must_use]
    pub fn days(&self) -> u32 {
        match (self.start, self.end) {
            (Some(start), Some(end)) => {
                let start_ord = date_to_ordinal(start);
                let end_ord = date_to_ordinal(end);
                (end_ord - start_ord + 1).max(0) as u32
            }
            _ => 0,
        }
    }
}

/// Position of a day relative to the selected range.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum RangeDayPosition {
    /// Not part of any range.
    #[default]
    None,
    /// The start endpoint (`data-range-start`).
    Start,
    /// Between start and end (`data-range-middle`).
    Middle,
    /// The end endpoint (`data-range-end`).
    End,
    /// Start and end are the same day (single-day range).
    StartEnd,
}

impl RangeDayPosition {
    /// Whether the day is part of the range at all.
    #[must_use]
    pub const fn is_selected(self) -> bool {
        !matches!(self, Self::None)
    }

    /// Whether this is one of the endpoints.
    #[must_use]
    pub const fn is_endpoint(self) -> bool {
        matches!(self, Self::Start | Self::End | Self::StartEnd)
    }

    /// Whether this is a middle (non-endpoint) range day.
    #[must_use]
    pub const fn is_middle(self) -> bool {
        matches!(self, Self::Middle)
    }
}

/// Resolves the range position of `date` relative to a committed range and
/// an optional "highlighted" preview range (when only start is picked and
/// the cursor is hovering another day).
///
/// `committed` is the stored `DateRange`; `highlight` is the ephemeral
/// preview range that bits-ui shows between the first click and the second.
#[must_use]
pub fn range_day_position(
    date: DateParts,
    committed: &DateRange,
    highlight: Option<&DateRange>,
) -> RangeDayPosition {
    // Committed range takes priority.
    if let (Some(start), Some(end)) = (committed.start, committed.end) {
        if start == end && date == start {
            return RangeDayPosition::StartEnd;
        }
        if date == start {
            return RangeDayPosition::Start;
        }
        if date == end {
            return RangeDayPosition::End;
        }
        if date > start && date < end {
            return RangeDayPosition::Middle;
        }
    }
    // Highlighted preview.
    if let Some(highlight) = highlight
        && let (Some(start), Some(end)) = (highlight.start, highlight.end)
    {
        if start == end && date == start {
            return RangeDayPosition::StartEnd;
        }
        if date == start {
            return RangeDayPosition::Start;
        }
        if date == end {
            return RangeDayPosition::End;
        }
        if date > start && date < end {
            return RangeDayPosition::Middle;
        }
    }
    RangeDayPosition::None
}

/// Result of clicking a day in a range calendar.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[must_use = "the pick describes the next state and must be applied"]
pub struct RangeCalendarPick {
    /// Next controlled range value.
    pub range: DateRange,
    /// New placeholder when the view should re-anchor.
    pub placeholder: Option<DateParts>,
}

/// Applies a day click to the current range, mirroring bits-ui
/// `handleCellClick` for range calendars.
///
/// The two-click flow:
/// 1. First click sets `start`, clears `end`.
/// 2. Second click sets `end`; if it precedes `start`, they swap.
/// 3. Clicking the start (when end is empty) deselects unless
///    `prevent_deselect`.
/// 4. Clicking the end (when range is complete) deselects the whole range
///    unless `prevent_deselect`.
/// 5. Clicking when a complete range exists resets to a new start.
///
/// `is_range_valid` is called with (ordered_start, ordered_end) and must
/// return `false` when `min_days`/`max_days`/disabled-days constraints are
/// violated; in that case the range resets to just the clicked day.
pub fn range_calendar_day_pick(
    range: DateRange,
    date: DateParts,
    prevent_deselect: bool,
    is_range_valid: impl Fn(DateParts, DateParts) -> bool,
) -> RangeCalendarPick {
    // Case: start is set, end is empty, and we click start again → deselect.
    if let Some(start) = range.start
        && range.end.is_none()
        && start == date
        && !prevent_deselect
    {
        return RangeCalendarPick {
            range: DateRange::default(),
            placeholder: Some(date),
        };
    }

    // Case: complete range exists, clicking end → deselect.
    if let (Some(_start), Some(end)) = (range.start, range.end)
        && end == date
        && !prevent_deselect
    {
        return RangeCalendarPick {
            range: DateRange::default(),
            placeholder: Some(date),
        };
    }

    // Case: nothing selected → set start.
    if range.start.is_none() {
        return RangeCalendarPick {
            range: DateRange::new(Some(date), None),
            placeholder: None,
        };
    }

    // Case: start is set, end is empty → complete the range.
    if range.end.is_none() {
        let start = range.start.expect("start checked above");
        let (ordered_start, ordered_end) = if date < start {
            (date, start)
        } else {
            (start, date)
        };
        if !is_range_valid(ordered_start, ordered_end) {
            // Invalid range → reset to just the clicked day.
            return RangeCalendarPick {
                range: DateRange::new(Some(date), None),
                placeholder: None,
            };
        }
        return RangeCalendarPick {
            range: DateRange::new(Some(ordered_start), Some(ordered_end)),
            placeholder: None,
        };
    }

    // Case: complete range exists, clicking elsewhere → start a new range.
    RangeCalendarPick {
        range: DateRange::new(Some(date), None),
        placeholder: None,
    }
}

/// Validates a range against `min_days` / `max_days` constraints.
///
/// Both `start` and `end` must already be ordered (`start <= end`).
/// A limit of `None` or `Some(0)` means "no limit".
#[must_use]
pub fn range_days_valid(
    start: DateParts,
    end: DateParts,
    min_days: Option<usize>,
    max_days: Option<usize>,
) -> bool {
    let start_ord = date_to_ordinal(start);
    let end_ord = date_to_ordinal(end);
    let days_in_range = (end_ord - start_ord + 1).max(0) as usize;
    if let Some(min) = min_days
        && min > 0
        && days_in_range < min
    {
        return false;
    }
    if let Some(max) = max_days
        && max > 0
        && days_in_range > max
    {
        return false;
    }
    true
}

/// Computes the highlighted (preview) range shown between the first click
/// and the cursor, mirroring bits-ui `highlightedRange`.
///
/// Returns `None` when a complete range is already set, when start is
/// unset, or when any day between start and the focused date is disabled
/// or unavailable.
#[must_use]
pub fn range_highlight(
    range: &DateRange,
    focused: Option<DateParts>,
    is_invalid: impl Fn(DateParts) -> bool,
) -> Option<DateRange> {
    // Only show highlight when exactly start is set (no end yet).
    if range.end.is_some() || range.start.is_none() {
        return None;
    }
    let start = range.start?;
    let focused = focused?;
    let (ordered_start, ordered_end) = if focused < start {
        (focused, start)
    } else {
        (start, focused)
    };
    // Validate all intermediate days.
    let mut cursor = add_days(ordered_start, 1);
    while cursor < ordered_end {
        if is_invalid(cursor) {
            return None;
        }
        cursor = add_days(cursor, 1);
    }
    Some(DateRange::new(Some(ordered_start), Some(ordered_end)))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(year: i32, month: u8, day: u8) -> DateParts {
        DateParts::new(year, month, day).expect("valid test date")
    }

    #[test]
    fn month_names_cover_all_formats() {
        assert_eq!(calendar_month_name(7, CalendarMonthFormat::Long), "July");
        assert_eq!(calendar_month_name(7, CalendarMonthFormat::Short), "Jul");
        assert_eq!(calendar_month_name(7, CalendarMonthFormat::Narrow), "J");
        assert_eq!(calendar_month_name(7, CalendarMonthFormat::Numeric), "7");
        assert_eq!(calendar_month_name(7, CalendarMonthFormat::TwoDigit), "07");
        // Clamped, not panicking.
        assert_eq!(calendar_month_name(0, CalendarMonthFormat::Long), "January");
    }

    #[test]
    fn weekdays_rotate_with_week_start() {
        assert_eq!(calendar_weekdays(0), [0, 1, 2, 3, 4, 5, 6]);
        assert_eq!(calendar_weekdays(1), [1, 2, 3, 4, 5, 6, 0]);
        assert_eq!(
            calendar_weekday_name(6, CalendarWeekdayFormat::Long),
            "Saturday"
        );
    }

    #[test]
    fn fixed_weeks_pads_to_six_rows() {
        // February 2026 spans exactly 4 weeks starting on Sunday.
        let plain = calendar_month_grid(date(2026, 2, 10), 0, false);
        assert_eq!(plain.len(), 4);

        let fixed = calendar_month_grid(date(2026, 2, 10), 0, true);
        assert_eq!(fixed.len(), 6);
        assert_eq!(fixed[4][0], date(2026, 3, 1));
        assert_eq!(fixed[5][6], date(2026, 3, 14));
    }

    #[test]
    fn visible_months_and_nav_targets() {
        let months = calendar_visible_months(date(2026, 7, 15), 2);
        assert_eq!(months, vec![date(2026, 7, 1), date(2026, 8, 1)]);

        assert_eq!(
            calendar_nav_target(months[0], true, false, 2),
            date(2026, 8, 1)
        );
        assert_eq!(
            calendar_nav_target(months[0], true, true, 2),
            date(2026, 9, 1)
        );
        assert_eq!(
            calendar_nav_target(months[0], false, true, 2),
            date(2026, 5, 1)
        );
    }

    #[test]
    fn nav_buttons_disable_at_bounds() {
        let june = date(2026, 6, 1);
        assert!(calendar_prev_disabled(june, Some(date(2026, 6, 1))));
        assert!(!calendar_prev_disabled(june, Some(date(2026, 5, 20))));
        assert!(!calendar_prev_disabled(june, None));

        assert!(calendar_next_disabled(june, Some(date(2026, 6, 30))));
        assert!(!calendar_next_disabled(june, Some(date(2026, 7, 1))));
        assert!(!calendar_next_disabled(june, None));
    }

    #[test]
    fn default_years_match_bits_ui_window() {
        let years = calendar_default_years(2026, 2026, None, None);
        assert_eq!(years.first(), Some(&1926));
        assert_eq!(years.last(), Some(&2036));

        let bounded =
            calendar_default_years(2026, 2026, Some(date(2020, 1, 1)), Some(date(2030, 12, 31)));
        assert_eq!(bounded, (2020..=2030).collect::<Vec<_>>());

        let early = calendar_default_years(1800, 2026, None, None);
        assert_eq!(early.first(), Some(&1790));
    }

    #[test]
    fn single_pick_toggles_and_reanchors() {
        let day = date(2026, 7, 4);
        let picked = calendar_day_pick(CalendarSelection::single(None), day, false, None);
        assert_eq!(picked.selection, CalendarSelection::single(Some(day)));
        assert_eq!(picked.placeholder, None);

        let cleared = calendar_day_pick(picked.selection, day, false, None);
        assert!(cleared.selection.is_empty());
        assert_eq!(cleared.placeholder, Some(day));

        let kept = calendar_day_pick(CalendarSelection::single(Some(day)), day, true, None);
        assert_eq!(kept.selection, CalendarSelection::single(Some(day)));
    }

    #[test]
    fn multiple_pick_respects_max_days() {
        let first = date(2026, 7, 1);
        let second = date(2026, 7, 2);
        let third = date(2026, 7, 3);

        let selection = CalendarSelection::multiple([first, second]);
        let overflow = calendar_day_pick(selection, third, false, Some(2));
        assert_eq!(overflow.selection, CalendarSelection::multiple([third]));

        let toggled = calendar_day_pick(
            CalendarSelection::multiple([first, second]),
            second,
            false,
            None,
        );
        assert_eq!(toggled.selection, CalendarSelection::multiple([first]));

        let emptied = calendar_day_pick(CalendarSelection::multiple([first]), first, false, None);
        assert!(emptied.selection.is_empty());
        assert_eq!(emptied.placeholder, Some(first));
    }

    #[test]
    fn day_state_interactivity_follows_guards() {
        let state = CalendarDayState::default();
        assert!(state.is_interactive(false, false, false));
        assert!(!state.is_interactive(true, false, false));
        assert!(!state.is_interactive(false, true, false));

        let outside = CalendarDayState {
            outside_month: true,
            ..CalendarDayState::default()
        };
        assert!(outside.is_interactive(false, false, false));
        assert!(!outside.is_interactive(false, false, true));

        let unavailable = CalendarDayState {
            unavailable: true,
            ..CalendarDayState::default()
        };
        assert!(!unavailable.is_interactive(false, false, false));
    }

    #[test]
    fn bounds_check_and_today() {
        let day = date(2026, 7, 4);
        assert!(calendar_date_in_bounds(day, None, None));
        assert!(!calendar_date_in_bounds(day, Some(date(2026, 7, 5)), None));
        assert!(!calendar_date_in_bounds(day, None, Some(date(2026, 7, 3))));

        let today = calendar_today_utc();
        assert!(today.year >= 2024);
    }

    // ---- Range calendar tests ----

    #[test]
    fn date_range_contains_and_days() {
        let range = DateRange::new(Some(date(2026, 7, 10)), Some(date(2026, 7, 15)));
        assert!(range.contains(date(2026, 7, 10)));
        assert!(range.contains(date(2026, 7, 12)));
        assert!(range.contains(date(2026, 7, 15)));
        assert!(!range.contains(date(2026, 7, 9)));
        assert!(!range.contains(date(2026, 7, 16)));
        assert_eq!(range.days(), 6);

        assert!(!DateRange::default().contains(date(2026, 7, 10)));
        assert_eq!(DateRange::default().days(), 0);
    }

    #[test]
    fn range_day_position_resolves_correctly() {
        let range = DateRange::new(Some(date(2026, 7, 10)), Some(date(2026, 7, 15)));
        assert_eq!(
            range_day_position(date(2026, 7, 10), &range, None),
            RangeDayPosition::Start
        );
        assert_eq!(
            range_day_position(date(2026, 7, 15), &range, None),
            RangeDayPosition::End
        );
        assert_eq!(
            range_day_position(date(2026, 7, 12), &range, None),
            RangeDayPosition::Middle
        );
        assert_eq!(
            range_day_position(date(2026, 7, 9), &range, None),
            RangeDayPosition::None
        );

        let single_day = DateRange::new(Some(date(2026, 7, 10)), Some(date(2026, 7, 10)));
        assert_eq!(
            range_day_position(date(2026, 7, 10), &single_day, None),
            RangeDayPosition::StartEnd
        );
    }

    #[test]
    fn range_pick_two_click_flow() {
        let empty = DateRange::default();
        let first = range_calendar_day_pick(empty, date(2026, 7, 10), false, |_, _| true);
        assert_eq!(first.range, DateRange::new(Some(date(2026, 7, 10)), None));

        let second = range_calendar_day_pick(first.range, date(2026, 7, 15), false, |_, _| true);
        assert_eq!(
            second.range,
            DateRange::new(Some(date(2026, 7, 10)), Some(date(2026, 7, 15)))
        );
    }

    #[test]
    fn range_pick_backward_selection_swaps() {
        let start_only = DateRange::new(Some(date(2026, 7, 20)), None);
        let pick = range_calendar_day_pick(start_only, date(2026, 7, 10), false, |_, _| true);
        assert_eq!(
            pick.range,
            DateRange::new(Some(date(2026, 7, 10)), Some(date(2026, 7, 20)))
        );
    }

    #[test]
    fn range_pick_deselect_start() {
        let start_only = DateRange::new(Some(date(2026, 7, 10)), None);
        let cleared = range_calendar_day_pick(start_only, date(2026, 7, 10), false, |_, _| true);
        assert!(cleared.range.is_empty());
        assert_eq!(cleared.placeholder, Some(date(2026, 7, 10)));

        let kept = range_calendar_day_pick(start_only, date(2026, 7, 10), true, |_, _| true);
        // With prevent_deselect, clicking start again completes a single-day range.
        assert_eq!(
            kept.range,
            DateRange::new(Some(date(2026, 7, 10)), Some(date(2026, 7, 10)))
        );
    }

    #[test]
    fn range_pick_deselect_complete_range() {
        let complete = DateRange::new(Some(date(2026, 7, 10)), Some(date(2026, 7, 15)));
        let cleared = range_calendar_day_pick(complete, date(2026, 7, 15), false, |_, _| true);
        assert!(cleared.range.is_empty());
    }

    #[test]
    fn range_pick_invalid_resets_to_clicked() {
        let start_only = DateRange::new(Some(date(2026, 7, 10)), None);
        let pick = range_calendar_day_pick(start_only, date(2026, 7, 20), false, |_, _| false);
        assert_eq!(pick.range, DateRange::new(Some(date(2026, 7, 20)), None));
    }

    #[test]
    fn range_pick_clicking_inside_complete_resets() {
        let complete = DateRange::new(Some(date(2026, 7, 10)), Some(date(2026, 7, 20)));
        let pick = range_calendar_day_pick(complete, date(2026, 7, 14), false, |_, _| true);
        assert_eq!(pick.range, DateRange::new(Some(date(2026, 7, 14)), None));
    }

    #[test]
    fn range_days_valid_enforces_limits() {
        let s = date(2026, 7, 10);
        let e = date(2026, 7, 15);
        assert!(range_days_valid(s, e, None, None));
        assert!(range_days_valid(s, e, Some(6), None));
        assert!(!range_days_valid(s, e, Some(7), None));
        assert!(range_days_valid(s, e, None, Some(6)));
        assert!(!range_days_valid(s, e, None, Some(5)));
    }

    #[test]
    fn range_highlight_computes_preview() {
        let range = DateRange::new(Some(date(2026, 7, 10)), None);
        let hl = range_highlight(&range, Some(date(2026, 7, 14)), |_| false);
        assert_eq!(
            hl,
            Some(DateRange::new(
                Some(date(2026, 7, 10)),
                Some(date(2026, 7, 14))
            ))
        );

        let blocked = range_highlight(&range, Some(date(2026, 7, 14)), |d| d.day == 12);
        assert_eq!(blocked, None);

        let complete = DateRange::new(Some(date(2026, 7, 10)), Some(date(2026, 7, 15)));
        assert_eq!(
            range_highlight(&complete, Some(date(2026, 7, 20)), |_| false),
            None
        );
    }
}
