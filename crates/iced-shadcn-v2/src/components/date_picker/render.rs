//! Composition rendering for date-picker: Popover + Calendar trigger.

use shadcn_common::{CalendarSelection, calendar_recipe, format_date_long, format_date_range};

use crate::components::button::Button;
use crate::components::calendar::Calendar;
use crate::components::popover::{Popover, PopoverAlign};
use crate::components::range_calendar::RangeCalendar;
use crate::iced_compat::Element;

use super::{DatePicker, DateRangePicker};

pub(super) fn build_date_picker<'a, Message>(
    picker: DatePicker<'a, Message>,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let theme = picker.theme;

    // Format the trigger label.
    let label = match picker.value {
        Some(date) => match &picker.format_date {
            Some(formatter) => formatter(date),
            None => format_date_long(date),
        },
        None => picker.placeholder_text.clone(),
    };

    // Build the trigger button.
    let trigger: Element<'a, Message> = Button::text(label, theme)
        .variant(picker.trigger_variant)
        .width(picker.trigger_width)
        .disabled(picker.disabled)
        .on_press_maybe(picker.on_open_change.as_ref().map(|cb| cb(true)))
        .into();

    // Build the calendar content.
    let mut calendar = Calendar::new(theme)
        .selection(match picker.value {
            Some(date) => CalendarSelection::single(Some(date)),
            None => CalendarSelection::single(None),
        })
        .caption_layout(picker.caption_layout)
        .weekday_format(picker.weekday_format)
        .year_format(picker.year_format)
        .week_starts_on(picker.week_starts_on)
        .fixed_weeks(picker.fixed_weeks)
        .number_of_months(picker.number_of_months)
        .transparent(true);

    if let Some(month_format) = picker.month_format {
        calendar = calendar.month_format(month_format);
    }
    if let Some(min) = picker.min_value {
        calendar = calendar.min_value(min);
    }
    if let Some(max) = picker.max_value {
        calendar = calendar.max_value(max);
    }
    if let Some(month) = picker.placeholder_month {
        calendar = calendar.placeholder(month);
    }
    if let Some(matcher) = picker.is_date_disabled {
        calendar = calendar.is_date_disabled(matcher);
    }
    if let Some(matcher) = picker.is_date_unavailable {
        calendar = calendar.is_date_unavailable(matcher);
    }
    if let Some(callback) = picker.on_placeholder_change {
        calendar = calendar.on_placeholder_change(callback);
    }

    // Wire selection change: emit value + optionally close.
    if let Some(on_value_change) = picker.on_value_change {
        calendar =
            calendar.on_selection_change(move |selection| on_value_change(selection.as_single()));
    }

    let content: Element<'a, Message> = calendar.into();

    // Compose with Popover — p-0 + w-auto matching shadcn-svelte.
    // Calendar intrinsic width: cell*7*months + pad*2 + gap*(months-1).
    let cal_recipe = calendar_recipe(theme.style_id());
    let months_count = picker.number_of_months.max(1) as f32;
    let single_month_w = cal_recipe.cell_size_px * 7.0 + cal_recipe.pad_px * 2.0;
    let popover_w = single_month_w * months_count
        + shadcn_common::CALENDAR_MONTHS_GAP_PX * (months_count - 1.0).max(0.0);

    let mut popover = Popover::new(trigger, content, theme)
        .align(PopoverAlign::Start)
        .content_padding(0.0)
        .width(popover_w)
        .disabled(picker.disabled);

    if let Some(open) = picker.open {
        popover = popover.open(open);
    }
    if let Some(callback) = picker.on_open_change {
        popover = popover.on_open_change(callback);
    }

    popover.into()
}

pub(super) fn build_date_range_picker<'a, Message>(
    picker: DateRangePicker<'a, Message>,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let theme = picker.theme;

    // Format the trigger label.
    let label = match &picker.format_range {
        Some(formatter) => formatter(&picker.value),
        None => format_date_range(&picker.value, &picker.placeholder_text),
    };

    // Build the trigger button.
    let trigger: Element<'a, Message> = Button::text(label, theme)
        .variant(picker.trigger_variant)
        .width(picker.trigger_width)
        .disabled(picker.disabled)
        .on_press_maybe(picker.on_open_change.as_ref().map(|cb| cb(true)))
        .into();

    // Build the range calendar content.
    let mut range_cal = RangeCalendar::new(theme)
        .value(picker.value)
        .caption_layout(picker.caption_layout)
        .week_starts_on(picker.week_starts_on)
        .fixed_weeks(picker.fixed_weeks)
        .number_of_months(picker.number_of_months)
        .transparent(true);

    if let Some(min) = picker.min_value {
        range_cal = range_cal.min_value(min);
    }
    if let Some(max) = picker.max_value {
        range_cal = range_cal.max_value(max);
    }
    if let Some(min_days) = picker.min_days {
        range_cal = range_cal.min_days(min_days);
    }
    if let Some(max_days) = picker.max_days {
        range_cal = range_cal.max_days(max_days);
    }
    if let Some(month) = picker.placeholder_month {
        range_cal = range_cal.placeholder(month);
    }
    if let Some(matcher) = picker.is_date_disabled {
        range_cal = range_cal.is_date_disabled(matcher);
    }
    if let Some(matcher) = picker.is_date_unavailable {
        range_cal = range_cal.is_date_unavailable(matcher);
    }
    if let Some(callback) = picker.on_placeholder_change {
        range_cal = range_cal.on_placeholder_change(callback);
    }
    if let Some(on_value_change) = picker.on_value_change {
        range_cal = range_cal.on_value_change(on_value_change);
    }

    let content: Element<'a, Message> = range_cal.into();

    // Compose with Popover — p-0 + w-auto matching shadcn-svelte.
    let cal_recipe = calendar_recipe(theme.style_id());
    let months_count = picker.number_of_months.max(1) as f32;
    let single_month_w = cal_recipe.cell_size_px * 7.0 + cal_recipe.pad_px * 2.0;
    let popover_w = single_month_w * months_count
        + shadcn_common::CALENDAR_MONTHS_GAP_PX * (months_count - 1.0).max(0.0);

    let mut popover = Popover::new(trigger, content, theme)
        .align(PopoverAlign::Start)
        .content_padding(0.0)
        .width(popover_w)
        .disabled(picker.disabled);

    if let Some(open) = picker.open {
        popover = popover.open(open);
    }
    if let Some(callback) = picker.on_open_change {
        popover = popover.on_open_change(callback);
    }

    popover.into()
}
