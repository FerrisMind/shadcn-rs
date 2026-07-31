//! Backend-agnostic date-picker configuration and formatting helpers.
//!
//! The date-picker is a composition pattern (Popover + Calendar/RangeCalendar),
//! not a standalone primitive. These shared types let egui and iced backends
//! expose a consistent API without duplicating formatting logic.

use crate::calendar::DateRange;
use crate::date_time::DateParts;

/// Default trigger button width (`w-[280px]` in the shadcn-svelte demo).
pub const DATE_PICKER_TRIGGER_WIDTH_PX: f32 = 280.0;

/// Default trigger width for the range variant (wider to fit two dates).
pub const DATE_PICKER_RANGE_TRIGGER_WIDTH_PX: f32 = 300.0;

/// Position of the calendar icon relative to the trigger label.
///
/// ```rust
/// use shadcn_common::DatePickerIconPosition;
///
/// assert_eq!(DatePickerIconPosition::default(), DatePickerIconPosition::Leading);
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DatePickerIconPosition {
    /// Icon before the label text (shadcn-svelte default).
    #[default]
    Leading,
    /// Icon after the label text.
    Trailing,
    /// No icon.
    None,
}

/// Selection mode of the date-picker.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DatePickerMode {
    /// Single date selection.
    #[default]
    Single,
    /// Date range selection (start + end).
    Range,
}

const MONTH_NAMES_LONG: [&str; 12] = [
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

const MONTH_NAMES_SHORT: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

/// Formats a date as "July 31, 2026" (English `dateStyle: "long"`).
///
/// ```rust
/// use shadcn_common::{DateParts, format_date_long};
///
/// let date = DateParts::new(2026, 7, 31).unwrap();
/// assert_eq!(format_date_long(date), "July 31, 2026");
/// ```
#[must_use]
pub fn format_date_long(date: DateParts) -> String {
    let month = MONTH_NAMES_LONG[usize::from(date.month.clamp(1, 12)) - 1];
    format!("{} {}, {}", month, date.day, date.year)
}

/// Formats a date as "Jul 31, 2026" (English `dateStyle: "medium"`).
///
/// ```rust
/// use shadcn_common::{DateParts, format_date_medium};
///
/// let date = DateParts::new(2026, 7, 4).unwrap();
/// assert_eq!(format_date_medium(date), "Jul 4, 2026");
/// ```
#[must_use]
pub fn format_date_medium(date: DateParts) -> String {
    let month = MONTH_NAMES_SHORT[usize::from(date.month.clamp(1, 12)) - 1];
    format!("{} {}, {}", month, date.day, date.year)
}

/// Formats a date range as "Jul 10, 2026 - Jul 20, 2026".
///
/// Returns the placeholder when the range is empty; shows just the start
/// when only the start is set.
///
/// ```rust
/// use shadcn_common::{DateParts, DateRange, format_date_range};
///
/// let range = DateRange::new(
///     Some(DateParts::new(2026, 7, 10).unwrap()),
///     Some(DateParts::new(2026, 7, 20).unwrap()),
/// );
/// assert_eq!(format_date_range(&range, "Pick dates"), "Jul 10, 2026 - Jul 20, 2026");
/// assert_eq!(format_date_range(&DateRange::default(), "Pick dates"), "Pick dates");
/// ```
#[must_use]
pub fn format_date_range(range: &DateRange, placeholder: &str) -> String {
    match (range.start, range.end) {
        (Some(start), Some(end)) => {
            format!(
                "{} - {}",
                format_date_medium(start),
                format_date_medium(end)
            )
        }
        (Some(start), None) => format_date_medium(start),
        _ => placeholder.to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(year: i32, month: u8, day: u8) -> DateParts {
        DateParts::new(year, month, day).expect("valid test date")
    }

    #[test]
    fn format_long_matches_english_style() {
        assert_eq!(format_date_long(date(2026, 1, 1)), "January 1, 2026");
        assert_eq!(format_date_long(date(2026, 12, 25)), "December 25, 2026");
    }

    #[test]
    fn format_medium_matches_english_style() {
        assert_eq!(format_date_medium(date(2026, 7, 4)), "Jul 4, 2026");
        assert_eq!(format_date_medium(date(2026, 11, 30)), "Nov 30, 2026");
    }

    #[test]
    fn format_range_handles_all_cases() {
        let full = DateRange::new(Some(date(2026, 7, 10)), Some(date(2026, 7, 20)));
        assert_eq!(
            format_date_range(&full, "Pick"),
            "Jul 10, 2026 - Jul 20, 2026"
        );

        let partial = DateRange::new(Some(date(2026, 7, 10)), None);
        assert_eq!(format_date_range(&partial, "Pick"), "Jul 10, 2026");

        assert_eq!(format_date_range(&DateRange::default(), "Pick"), "Pick");
    }

    #[test]
    fn icon_position_default_is_leading() {
        assert_eq!(
            DatePickerIconPosition::default(),
            DatePickerIconPosition::Leading
        );
    }
}
