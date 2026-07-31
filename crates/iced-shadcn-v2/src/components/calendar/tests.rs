//! Behavioral checks for the calendar builder.

use shadcn_common::DateParts;

use super::*;
use crate::theme::Theme;

fn date(year: i32, month: u8, day: u8) -> DateParts {
    DateParts::new(year, month, day).expect("valid test date")
}

#[test]
fn defaults_match_the_web_wrapper() {
    let theme = Theme::light();
    let calendar: Calendar<'_, ()> = Calendar::new(&theme);

    assert_eq!(calendar.caption_layout, CalendarCaptionLayout::Label);
    assert_eq!(
        calendar.button_variant,
        crate::components::button::ButtonVariant::Ghost
    );
    assert_eq!(calendar.weekday_format, CalendarWeekdayFormat::Short);
    assert_eq!(calendar.year_format, CalendarYearFormat::Numeric);
    assert_eq!(calendar.number_of_months, 1);
    assert_eq!(calendar.week_starts_on, 0);
    assert!(calendar.selection.is_empty());
    assert!(!calendar.paged_navigation);
    assert!(!calendar.fixed_weeks);
    assert!(!calendar.prevent_deselect);
    assert!(!calendar.disable_days_outside_month);
}

#[test]
fn selection_setters_switch_modes() {
    let theme = Theme::light();
    let day = date(2026, 7, 4);

    let single: Calendar<'_, ()> = Calendar::new(&theme).selected(day);
    assert_eq!(single.selection.as_single(), Some(day));

    let cleared: Calendar<'_, ()> = Calendar::new(&theme).selected_maybe(None);
    assert!(cleared.selection.is_empty());

    let multiple: Calendar<'_, ()> = Calendar::new(&theme).values([day, day, date(2026, 7, 5)]);
    assert_eq!(multiple.selection.as_multiple().len(), 2);
}

#[test]
fn numeric_knobs_are_normalized() {
    let theme = Theme::light();

    let calendar: Calendar<'_, ()> = Calendar::new(&theme)
        .number_of_months(0)
        .week_starts_on(8)
        .max_days(0)
        .months([0, 3, 13, 12]);

    assert_eq!(calendar.number_of_months, 1);
    assert_eq!(calendar.week_starts_on, 1);
    assert_eq!(calendar.max_days, None);
    assert_eq!(calendar.months.as_deref(), Some(&[3, 12][..]));

    let capped: Calendar<'_, ()> = Calendar::new(&theme).max_days(3);
    assert_eq!(capped.max_days, Some(3));
}

#[test]
fn builder_converts_into_an_element() {
    let theme = Theme::light();

    let _: crate::iced_compat::Element<'_, ()> = Calendar::new(&theme)
        .selected(date(2026, 7, 4))
        .placeholder(date(2026, 7, 1))
        .today(date(2026, 7, 30))
        .caption_layout(CalendarCaptionLayout::Dropdown)
        .min_value(date(2026, 1, 1))
        .max_value(date(2026, 12, 31))
        .fixed_weeks(true)
        .number_of_months(2)
        .is_date_disabled(|day| day.day == 1)
        .is_date_unavailable(|day| day.day == 13)
        .on_selection_change(|_| ())
        .on_placeholder_change(|_| ())
        .into();
}

#[test]
fn helper_mirrors_the_builder() {
    let theme = Theme::light();
    let built: Calendar<'_, ()> = calendar(&theme);
    assert!(built.selection.is_empty());
    assert!(built.placeholder.is_none());
}

#[test]
fn debug_reports_callback_presence() {
    let theme = Theme::light();
    let calendar: Calendar<'_, ()> = Calendar::new(&theme).on_select(|_| ());
    let debug = format!("{calendar:?}");
    assert!(debug.contains("on_select: true"));
    assert!(debug.contains("on_placeholder_change: false"));
}
