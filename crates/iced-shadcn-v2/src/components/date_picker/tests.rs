//! Behavioral checks for the date-picker builders.

use shadcn_common::{DateParts, DateRange};

use super::*;
use crate::theme::Theme;

fn date(year: i32, month: u8, day: u8) -> DateParts {
    DateParts::new(year, month, day).expect("valid test date")
}

#[test]
fn date_picker_defaults() {
    let theme = Theme::light();
    let dp: DatePicker<'_, ()> = DatePicker::new(&theme);

    assert_eq!(dp.value, None);
    assert_eq!(dp.icon_position, DatePickerIconPosition::Leading);
    assert_eq!(dp.trigger_variant, ButtonVariant::Outline);
    assert!(dp.close_on_select);
    assert!(!dp.disabled);
    assert_eq!(dp.open, None);
    assert_eq!(dp.number_of_months, 1);
}

#[test]
fn date_picker_value_setter() {
    let theme = Theme::light();
    let dp: DatePicker<'_, ()> = DatePicker::new(&theme).value(Some(date(2026, 7, 4)));
    assert_eq!(dp.value, Some(date(2026, 7, 4)));
}

#[test]
fn date_picker_converts_to_element() {
    let theme = Theme::light();
    let _: crate::iced_compat::Element<'_, ()> = DatePicker::new(&theme)
        .value(Some(date(2026, 7, 4)))
        .on_value_change(|_| ())
        .on_open_change(|_| ())
        .into();
}

#[test]
fn date_range_picker_defaults() {
    let theme = Theme::light();
    let drp: DateRangePicker<'_, ()> = DateRangePicker::new(&theme);

    assert!(drp.value.is_empty());
    assert_eq!(drp.number_of_months, 2);
    assert!(!drp.close_on_select);
}

#[test]
fn date_range_picker_converts_to_element() {
    let theme = Theme::light();
    let _: crate::iced_compat::Element<'_, ()> = DateRangePicker::new(&theme)
        .value(DateRange::new(
            Some(date(2026, 7, 10)),
            Some(date(2026, 7, 20)),
        ))
        .on_value_change(|_| ())
        .on_open_change(|_| ())
        .into();
}

#[test]
fn convenience_helpers_work() {
    let theme = Theme::light();
    let _dp: DatePicker<'_, ()> = date_picker(&theme);
    let _drp: DateRangePicker<'_, ()> = date_range_picker(&theme);
}
