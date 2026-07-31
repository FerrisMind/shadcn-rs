//! Backend-agnostic date/time value types and normalization helpers.
//!
//! These utilities mirror the value-normalization behavior used by bits-ui
//! date/time fields while staying independent from any GUI or timezone crate.
//! They are intended for component state and defaults logic in backend adapters.

use core::cmp::Ordering;
use core::fmt;

/// Date precision for date-valued controls.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum DateGranularity {
    /// Date only (`YYYY-MM-DD`).
    #[default]
    Day,
    /// Date + hour.
    Hour,
    /// Date + hour/minute.
    Minute,
    /// Date + hour/minute/second.
    Second,
}

impl DateGranularity {
    const fn includes_time(self) -> bool {
        !matches!(self, Self::Day)
    }
}

/// Time precision for time-valued controls.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum TimeGranularity {
    /// Hour precision.
    Hour,
    /// Hour/minute precision.
    #[default]
    Minute,
    /// Hour/minute/second precision.
    Second,
}

/// Calendar date without timezone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DateParts {
    /// Year in proleptic Gregorian calendar.
    pub year: i32,
    /// Month in `1..=12`.
    pub month: u8,
    /// Day of month in `1..=31` (validated by month/year).
    pub day: u8,
}

impl DateParts {
    /// Creates a validated calendar date.
    pub fn new(year: i32, month: u8, day: u8) -> Result<Self, DateTimeError> {
        if !(1..=12).contains(&month) {
            return Err(DateTimeError::InvalidMonth { month });
        }

        let max_day = days_in_month(year, month);
        if day == 0 || day > max_day {
            return Err(DateTimeError::InvalidDay { day, max_day });
        }

        Ok(Self { year, month, day })
    }
}

/// Clock time without timezone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TimeParts {
    /// Hour in `0..=23`.
    pub hour: u8,
    /// Minute in `0..=59`.
    pub minute: u8,
    /// Second in `0..=59`.
    pub second: u8,
}

impl TimeParts {
    /// Midnight `00:00:00`.
    pub const MIDNIGHT: Self = Self {
        hour: 0,
        minute: 0,
        second: 0,
    };

    /// Creates a validated time.
    pub fn new(hour: u8, minute: u8, second: u8) -> Result<Self, DateTimeError> {
        if hour > 23 {
            return Err(DateTimeError::InvalidHour { hour });
        }
        if minute > 59 {
            return Err(DateTimeError::InvalidMinute { minute });
        }
        if second > 59 {
            return Err(DateTimeError::InvalidSecond { second });
        }

        Ok(Self {
            hour,
            minute,
            second,
        })
    }

    /// Returns this time truncated to `granularity`.
    pub const fn truncate(self, granularity: TimeGranularity) -> Self {
        match granularity {
            TimeGranularity::Hour => Self {
                hour: self.hour,
                minute: 0,
                second: 0,
            },
            TimeGranularity::Minute => Self {
                hour: self.hour,
                minute: self.minute,
                second: 0,
            },
            TimeGranularity::Second => self,
        }
    }
}

/// Date + time without timezone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DateTimeParts {
    /// Date portion.
    pub date: DateParts,
    /// Time portion.
    pub time: TimeParts,
}

impl DateTimeParts {
    /// Creates a new date-time from parts.
    pub const fn new(date: DateParts, time: TimeParts) -> Self {
        Self { date, time }
    }

    /// Returns this date-time truncated to `granularity`.
    pub const fn truncate(self, granularity: DateGranularity) -> Self {
        match granularity {
            DateGranularity::Day => Self::new(self.date, TimeParts::MIDNIGHT),
            DateGranularity::Hour => {
                Self::new(self.date, self.time.truncate(TimeGranularity::Hour))
            }
            DateGranularity::Minute => {
                Self::new(self.date, self.time.truncate(TimeGranularity::Minute))
            }
            DateGranularity::Second => self,
        }
    }
}

/// Date control value: date-only or date-time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DateValue {
    /// Date-only value.
    Date(DateParts),
    /// Date-time value.
    DateTime(DateTimeParts),
}

impl DateValue {
    fn normalized_for_cmp(self) -> DateTimeParts {
        match self {
            Self::Date(date) => DateTimeParts::new(date, TimeParts::MIDNIGHT),
            Self::DateTime(value) => value,
        }
    }
}

impl PartialOrd for DateValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DateValue {
    fn cmp(&self, other: &Self) -> Ordering {
        self.normalized_for_cmp().cmp(&other.normalized_for_cmp())
    }
}

/// Errors returned by date/time parsing and validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DateTimeError {
    /// Invalid month value.
    InvalidMonth { month: u8 },
    /// Invalid day value for the given month/year.
    InvalidDay { day: u8, max_day: u8 },
    /// Invalid hour value.
    InvalidHour { hour: u8 },
    /// Invalid minute value.
    InvalidMinute { minute: u8 },
    /// Invalid second value.
    InvalidSecond { second: u8 },
    /// Input does not match the expected format.
    InvalidFormat,
    /// Numeric field failed to parse.
    ParseNumber,
}

impl fmt::Display for DateTimeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidMonth { month } => write!(formatter, "invalid month `{month}`"),
            Self::InvalidDay { day, max_day } => {
                write!(
                    formatter,
                    "invalid day `{day}` for month with {max_day} days"
                )
            }
            Self::InvalidHour { hour } => write!(formatter, "invalid hour `{hour}`"),
            Self::InvalidMinute { minute } => write!(formatter, "invalid minute `{minute}`"),
            Self::InvalidSecond { second } => write!(formatter, "invalid second `{second}`"),
            Self::InvalidFormat => formatter.write_str("invalid date/time format"),
            Self::ParseNumber => formatter.write_str("failed to parse numeric date/time field"),
        }
    }
}

impl std::error::Error for DateTimeError {}

/// Configuration for computing a default date value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[must_use = "the config has no effect until passed to default_date_value"]
pub struct DateDefaultConfig {
    default_value: Option<DateValue>,
    min_value: Option<DateValue>,
    max_value: Option<DateValue>,
    granularity: DateGranularity,
}

impl Default for DateDefaultConfig {
    fn default() -> Self {
        Self {
            default_value: None,
            min_value: None,
            max_value: None,
            granularity: DateGranularity::Day,
        }
    }
}

impl DateDefaultConfig {
    /// Creates a config with default settings.
    pub const fn new() -> Self {
        Self {
            default_value: None,
            min_value: None,
            max_value: None,
            granularity: DateGranularity::Day,
        }
    }

    /// Sets the candidate default value.
    pub const fn default_value(mut self, value: Option<DateValue>) -> Self {
        self.default_value = value;
        self
    }

    /// Sets minimum allowed value.
    pub const fn min_value(mut self, value: Option<DateValue>) -> Self {
        self.min_value = value;
        self
    }

    /// Sets maximum allowed value.
    pub const fn max_value(mut self, value: Option<DateValue>) -> Self {
        self.max_value = value;
        self
    }

    /// Sets output granularity.
    pub const fn granularity(mut self, granularity: DateGranularity) -> Self {
        self.granularity = granularity;
        self
    }
}

/// Configuration for computing a default time value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[must_use = "the config has no effect until passed to default_time_value"]
pub struct TimeDefaultConfig {
    default_value: Option<TimeParts>,
    granularity: TimeGranularity,
}

impl Default for TimeDefaultConfig {
    fn default() -> Self {
        Self {
            default_value: None,
            granularity: TimeGranularity::Minute,
        }
    }
}

impl TimeDefaultConfig {
    /// Creates a config with default settings.
    pub const fn new() -> Self {
        Self {
            default_value: None,
            granularity: TimeGranularity::Minute,
        }
    }

    /// Sets the candidate default time value.
    pub const fn default_value(mut self, value: Option<TimeParts>) -> Self {
        self.default_value = value;
        self
    }

    /// Sets output granularity.
    pub const fn granularity(mut self, granularity: TimeGranularity) -> Self {
        self.granularity = granularity;
        self
    }
}

/// Computes a default date value and clamps it to optional min/max bounds.
pub fn default_date_value(config: DateDefaultConfig, today: DateParts) -> DateValue {
    let candidate = config.default_value.unwrap_or_else(|| {
        if config.granularity.includes_time() {
            DateValue::DateTime(DateTimeParts::new(today, TimeParts::MIDNIGHT))
        } else {
            DateValue::Date(today)
        }
    });

    let clamped = clamp_date_value(candidate, config.min_value, config.max_value);
    truncate_date_value(clamped, config.granularity)
}

/// Computes a default time value with granularity normalization.
pub fn default_time_value(config: TimeDefaultConfig) -> TimeParts {
    config
        .default_value
        .unwrap_or(TimeParts::MIDNIGHT)
        .truncate(config.granularity)
}

/// Clamps `value` into the optional `[min, max]` range.
pub fn clamp_date_value(
    value: DateValue,
    min: Option<DateValue>,
    max: Option<DateValue>,
) -> DateValue {
    let mut current = value;
    if let Some(minimum) = min
        && current < minimum
    {
        current = minimum;
    }
    if let Some(maximum) = max
        && current > maximum
    {
        current = maximum;
    }
    current
}

/// Truncates a date value to `granularity`.
pub fn truncate_date_value(value: DateValue, granularity: DateGranularity) -> DateValue {
    match (value, granularity) {
        (DateValue::Date(date), DateGranularity::Day) => DateValue::Date(date),
        (DateValue::Date(date), _) => {
            DateValue::DateTime(DateTimeParts::new(date, TimeParts::MIDNIGHT))
        }
        (DateValue::DateTime(date_time), DateGranularity::Day) => DateValue::Date(date_time.date),
        (DateValue::DateTime(date_time), _) => DateValue::DateTime(date_time.truncate(granularity)),
    }
}

/// Parses `YYYY-MM-DD`.
pub fn parse_date(input: &str) -> Result<DateParts, DateTimeError> {
    let mut chunks = input.split('-');
    let year: i32 = chunks
        .next()
        .ok_or(DateTimeError::InvalidFormat)?
        .parse()
        .map_err(|_| DateTimeError::ParseNumber)?;
    let month: u8 = chunks
        .next()
        .ok_or(DateTimeError::InvalidFormat)?
        .parse()
        .map_err(|_| DateTimeError::ParseNumber)?;
    let day: u8 = chunks
        .next()
        .ok_or(DateTimeError::InvalidFormat)?
        .parse()
        .map_err(|_| DateTimeError::ParseNumber)?;
    if chunks.next().is_some() {
        return Err(DateTimeError::InvalidFormat);
    }
    DateParts::new(year, month, day)
}

/// Parses `HH:MM` or `HH:MM:SS`.
pub fn parse_time(input: &str) -> Result<TimeParts, DateTimeError> {
    let mut chunks = input.split(':');
    let hour: u8 = chunks
        .next()
        .ok_or(DateTimeError::InvalidFormat)?
        .parse()
        .map_err(|_| DateTimeError::ParseNumber)?;
    let minute: u8 = chunks
        .next()
        .ok_or(DateTimeError::InvalidFormat)?
        .parse()
        .map_err(|_| DateTimeError::ParseNumber)?;
    let second: u8 = chunks
        .next()
        .map(|value| value.parse().map_err(|_| DateTimeError::ParseNumber))
        .transpose()?
        .unwrap_or(0);
    if chunks.next().is_some() {
        return Err(DateTimeError::InvalidFormat);
    }
    TimeParts::new(hour, minute, second)
}

/// Parses `YYYY-MM-DDTHH:MM` or `YYYY-MM-DDTHH:MM:SS`.
pub fn parse_date_time(input: &str) -> Result<DateTimeParts, DateTimeError> {
    let (date, time) = input.split_once('T').ok_or(DateTimeError::InvalidFormat)?;
    Ok(DateTimeParts::new(parse_date(date)?, parse_time(time)?))
}

/// Parses `input` as the same value kind as `reference`.
pub fn parse_like_reference(input: &str, reference: DateValue) -> Result<DateValue, DateTimeError> {
    match reference {
        DateValue::Date(_) => parse_date(input).map(DateValue::Date),
        DateValue::DateTime(_) => parse_date_time(input).map(DateValue::DateTime),
    }
}

/// Days in the month of `date`.
#[must_use]
pub const fn days_in_month_of(date: DateParts) -> u8 {
    days_in_month(date.year, date.month)
}

/// First day of the month containing `date`.
#[must_use]
pub const fn start_of_month(date: DateParts) -> DateParts {
    DateParts {
        year: date.year,
        month: date.month,
        day: 1,
    }
}

/// Adds `days` (may be negative) using proleptic Gregorian rules.
#[must_use]
pub fn add_days(date: DateParts, days: i32) -> DateParts {
    let mut ordinal = date_to_ordinal(date) + i64::from(days);
    // Guard extreme underflows so callers get a clamped epoch day.
    if ordinal < 1 {
        ordinal = 1;
    }
    ordinal_to_date(ordinal)
}

/// Adds `months` (may be negative), clamping the day into the target month.
#[must_use]
pub fn add_months(date: DateParts, months: i32) -> DateParts {
    let total = i32::from(date.month) - 1 + months;
    let year = date.year + total.div_euclid(12);
    let month = u8::try_from(total.rem_euclid(12) + 1).unwrap_or(1);
    let max_day = days_in_month(year, month).max(1);
    DateParts {
        year,
        month,
        day: date.day.min(max_day),
    }
}

/// Weekday of `date` where `0 = Sunday` … `6 = Saturday` (Sakamoto).
#[must_use]
pub const fn weekday_sunday(date: DateParts) -> u8 {
    let mut year = date.year;
    let month = date.month as usize;
    let day = date.day as i32;
    if month < 3 {
        year -= 1;
    }
    let t = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    let value = (year + year / 4 - year / 100 + year / 400 + t[month - 1] + day) % 7;
    if value < 0 {
        (value + 7) as u8
    } else {
        value as u8
    }
}

/// Start of the week containing `date`.
///
/// `first_day_of_week` uses `0 = Sunday` … `6 = Saturday`.
#[must_use]
pub fn start_of_week(date: DateParts, first_day_of_week: u8) -> DateParts {
    let first = first_day_of_week % 7;
    let current = weekday_sunday(date);
    let delta = i32::from((current + 7 - first) % 7);
    add_days(date, -delta)
}

/// Seven days starting at [`start_of_week`] for `date`.
#[must_use]
pub fn days_in_week(date: DateParts, first_day_of_week: u8) -> [DateParts; 7] {
    let start = start_of_week(date, first_day_of_week);
    std::array::from_fn(|index| add_days(start, index as i32))
}

/// Month calendar grid: weeks of seven days covering `date`'s month.
///
/// Leading/trailing days from adjacent months are included so every row is a
/// full week starting on `first_day_of_week`.
#[must_use]
pub fn month_days(date: DateParts, first_day_of_week: u8) -> Vec<[DateParts; 7]> {
    let start = start_of_week(start_of_month(date), first_day_of_week);
    let month_end = DateParts {
        year: date.year,
        month: date.month,
        day: days_in_month(date.year, date.month).max(1),
    };
    let mut weeks = Vec::with_capacity(6);
    let mut cursor = start;
    loop {
        let week = std::array::from_fn(|index| add_days(cursor, index as i32));
        weeks.push(week);
        let last = week[6];
        if last >= month_end {
            break;
        }
        cursor = add_days(cursor, 7);
        if weeks.len() >= 6 {
            break;
        }
    }
    weeks
}

/// Clamps a calendar date into an optional `[min, max]` range.
#[must_use]
pub fn clamp_date_parts(
    date: DateParts,
    min: Option<DateParts>,
    max: Option<DateParts>,
) -> DateParts {
    let mut current = date;
    if let Some(minimum) = min
        && current < minimum
    {
        current = minimum;
    }
    if let Some(maximum) = max
        && current > maximum
    {
        current = maximum;
    }
    current
}

const fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

const fn days_in_month(year: i32, month: u8) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => 0,
    }
}

pub(crate) fn date_to_ordinal(date: DateParts) -> i64 {
    let mut days = 0_i64;
    let year = i64::from(date.year);
    for y in 1..year {
        days += if is_leap_year(y as i32) { 366 } else { 365 };
    }
    for month in 1..date.month {
        days += i64::from(days_in_month(date.year, month));
    }
    days + i64::from(date.day)
}

fn ordinal_to_date(mut ordinal: i64) -> DateParts {
    let mut year = 1_i32;
    loop {
        let year_days = if is_leap_year(year) { 366 } else { 365 };
        if ordinal <= year_days {
            break;
        }
        ordinal -= year_days;
        year += 1;
        if year > 999_999 {
            break;
        }
    }
    let mut month = 1_u8;
    while month <= 12 {
        let month_days = i64::from(days_in_month(year, month));
        if ordinal <= month_days {
            break;
        }
        ordinal -= month_days;
        month += 1;
    }
    DateParts {
        year,
        month,
        day: ordinal.clamp(1, 31) as u8,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn date_validation_enforces_month_and_day() {
        assert!(DateParts::new(2025, 0, 10).is_err());
        assert!(DateParts::new(2025, 2, 29).is_err());
        assert!(DateParts::new(2024, 2, 29).is_ok());
    }

    #[test]
    fn default_date_clamps_to_bounds() {
        let today = DateParts::new(2025, 1, 10).expect("valid date literal");
        let min = DateValue::Date(DateParts::new(2025, 1, 20).expect("valid date literal"));
        let value = default_date_value(DateDefaultConfig::new().min_value(Some(min)), today);
        assert_eq!(value, min);
    }

    #[test]
    fn date_time_granularity_truncates_clock_part() {
        let raw = DateValue::DateTime(DateTimeParts::new(
            DateParts::new(2025, 7, 30).expect("valid date literal"),
            TimeParts::new(13, 42, 59).expect("valid time literal"),
        ));
        let minute = truncate_date_value(raw, DateGranularity::Minute);
        assert_eq!(
            minute,
            DateValue::DateTime(DateTimeParts::new(
                DateParts::new(2025, 7, 30).expect("valid date literal"),
                TimeParts::new(13, 42, 0).expect("valid time literal"),
            ))
        );
    }

    #[test]
    fn parse_helpers_accept_iso_like_formats() {
        let date = parse_date("2026-07-30").expect("valid date string");
        assert_eq!(date.day, 30);

        let time = parse_time("08:15").expect("valid time string");
        assert_eq!(time.second, 0);

        let date_time = parse_date_time("2026-07-30T08:15:45").expect("valid datetime string");
        assert_eq!(date_time.time.second, 45);
    }

    #[test]
    fn parse_like_reference_uses_reference_kind() {
        let as_date = parse_like_reference(
            "2026-07-30",
            DateValue::Date(DateParts::new(2020, 1, 1).expect("valid date literal")),
        )
        .expect("reference parse");
        assert!(matches!(as_date, DateValue::Date(_)));

        let as_datetime = parse_like_reference(
            "2026-07-30T08:15:45",
            DateValue::DateTime(DateTimeParts::new(
                DateParts::new(2020, 1, 1).expect("valid date literal"),
                TimeParts::MIDNIGHT,
            )),
        )
        .expect("reference parse");
        assert!(matches!(as_datetime, DateValue::DateTime(_)));
    }

    #[test]
    fn calendar_navigation_helpers() {
        let date = DateParts::new(2024, 1, 31).expect("valid");
        assert_eq!(add_months(date, 1).day, 29);
        assert_eq!(add_days(date, 1).month, 2);

        let week = start_of_week(DateParts::new(2024, 7, 30).expect("valid"), 1);
        assert_eq!(weekday_sunday(week), 1);

        let grid = month_days(DateParts::new(2024, 7, 15).expect("valid"), 0);
        assert!(!grid.is_empty());
        assert_eq!(grid[0].len(), 7);
    }
}
