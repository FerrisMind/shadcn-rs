//! Behavioral checks for the range-calendar builder.

use shadcn_common::{DateParts, DateRange};

use super::*;
use crate::theme::Theme;

fn date(year: i32, month: u8, day: u8) -> DateParts {
    DateParts::new(year, month, day).expect("valid test date")
}

#[test]
fn defaults_match_the_web_wrapper() {
    let theme = Theme::light();
    let rc: RangeCalendar<'_, ()> = RangeCalendar::new(&theme);

    assert_eq!(rc.caption_layout, CalendarCaptionLayout::Label);
    assert_eq!(rc.button_variant, ButtonVariant::Ghost);
    assert_eq!(rc.weekday_format, CalendarWeekdayFormat::Short);
    assert_eq!(rc.year_format, CalendarYearFormat::Numeric);
    assert_eq!(rc.number_of_months, 1);
    assert_eq!(rc.week_starts_on, 0);
    assert!(rc.value.is_empty());
    assert!(!rc.paged_navigation);
    assert!(!rc.fixed_weeks);
    assert!(!rc.prevent_deselect);
    assert!(!rc.exclude_disabled);
    assert_eq!(rc.min_days, None);
    assert_eq!(rc.max_days, None);
}

#[test]
fn value_setters_work() {
    let theme = Theme::light();
    let start = date(2026, 7, 10);
    let end = date(2026, 7, 20);

    let rc: RangeCalendar<'_, ()> =
        RangeCalendar::new(&theme).value(DateRange::new(Some(start), Some(end)));
    assert_eq!(rc.value.start, Some(start));
    assert_eq!(rc.value.end, Some(end));

    let rc2: RangeCalendar<'_, ()> = RangeCalendar::new(&theme).start(start).end(end);
    assert_eq!(rc2.value.start, Some(start));
    assert_eq!(rc2.value.end, Some(end));
}

#[test]
fn numeric_knobs_normalize() {
    let theme = Theme::light();

    let rc: RangeCalendar<'_, ()> = RangeCalendar::new(&theme)
        .number_of_months(0)
        .week_starts_on(9)
        .min_days(0)
        .max_days(0)
        .months([0, 3, 13, 12]);

    assert_eq!(rc.number_of_months, 1);
    assert_eq!(rc.week_starts_on, 2);
    assert_eq!(rc.min_days, None);
    assert_eq!(rc.max_days, None);
    assert_eq!(rc.months.as_deref(), Some(&[3, 12][..]));

    let rc2: RangeCalendar<'_, ()> = RangeCalendar::new(&theme).min_days(3).max_days(7);
    assert_eq!(rc2.min_days, Some(3));
    assert_eq!(rc2.max_days, Some(7));
}

#[test]
fn builder_converts_into_an_element() {
    let theme = Theme::light();

    let _: crate::iced_compat::Element<'_, ()> = RangeCalendar::new(&theme)
        .value(DateRange::new(
            Some(date(2026, 7, 10)),
            Some(date(2026, 7, 20)),
        ))
        .placeholder(date(2026, 7, 1))
        .today(date(2026, 7, 31))
        .caption_layout(CalendarCaptionLayout::Dropdown)
        .min_value(date(2026, 1, 1))
        .max_value(date(2026, 12, 31))
        .fixed_weeks(true)
        .number_of_months(2)
        .min_days(2)
        .max_days(14)
        .is_date_disabled(|day| day.day == 1)
        .is_date_unavailable(|day| day.day == 13)
        .on_value_change(|_| ())
        .on_placeholder_change(|_| ())
        .into();
}

#[test]
fn helper_mirrors_the_builder() {
    let theme = Theme::light();
    let built: RangeCalendar<'_, ()> = range_calendar(&theme);
    assert!(built.value.is_empty());
    assert!(built.placeholder.is_none());
}

#[test]
fn debug_reports_callback_presence() {
    let theme = Theme::light();
    let rc: RangeCalendar<'_, ()> = RangeCalendar::new(&theme).on_value_change(|_| ());
    let debug = format!("{rc:?}");
    assert!(debug.contains("on_value_change: true"));
    assert!(debug.contains("on_placeholder_change: false"));
}
