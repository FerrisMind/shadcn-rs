//! Date-picker composition: Popover + Calendar trigger button.
//!
//! Ports the shadcn-svelte date-picker pattern as a single builder that
//! internally composes [`crate::Popover`] (floating surface) with
//! [`crate::Calendar`] or [`crate::RangeCalendar`] (date selection grid)
//! and a styled trigger [`crate::Button`] (with a calendar icon).
//!
//! ```rust,no_run
//! use iced::Element;
//! use iced_shadcn_v2::{DateParts, DatePicker, Theme};
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     DateChanged(Option<DateParts>),
//!     PickerOpen(bool),
//! }
//!
//! fn view<'a>(
//!     theme: &'a Theme,
//!     date: Option<DateParts>,
//!     open: bool,
//!     month: DateParts,
//! ) -> Element<'a, Message> {
//!     DatePicker::new(theme)
//!         .value(date)
//!         .open(open)
//!         .placeholder(month)
//!         .on_value_change(Message::DateChanged)
//!         .on_open_change(Message::PickerOpen)
//!         .into()
//! }
//! ```

mod render;

#[cfg(test)]
mod tests;

pub use shadcn_common::DatePickerIconPosition;

use std::fmt;

use shadcn_common::{DateParts, DateRange};

use crate::components::button::ButtonVariant;
use crate::components::calendar::{
    CalendarCaptionLayout, CalendarMonthFormat, CalendarWeekdayFormat, CalendarYearFormat,
};
use crate::iced_compat::{Element, Length};
use crate::theme::Theme;

/// Builder-first single-date picker.
///
/// Composes a [`crate::Popover`] with a [`crate::Calendar`] inside and a
/// styled trigger button showing the formatted selected date (or a
/// placeholder). The application owns the open state, the selected date,
/// and the visible month.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct DatePicker<'a, Message> {
    theme: &'a Theme,
    value: Option<DateParts>,
    placeholder_text: String,
    placeholder_month: Option<DateParts>,
    icon_position: DatePickerIconPosition,
    trigger_variant: ButtonVariant,
    trigger_width: Length,
    caption_layout: CalendarCaptionLayout,
    weekday_format: CalendarWeekdayFormat,
    month_format: Option<CalendarMonthFormat>,
    year_format: CalendarYearFormat,
    min_value: Option<DateParts>,
    max_value: Option<DateParts>,
    week_starts_on: u8,
    fixed_weeks: bool,
    number_of_months: usize,
    close_on_select: bool,
    disabled: bool,
    open: Option<bool>,
    is_date_disabled: Option<Box<dyn Fn(DateParts) -> bool + 'a>>,
    is_date_unavailable: Option<Box<dyn Fn(DateParts) -> bool + 'a>>,
    format_date: Option<Box<dyn Fn(DateParts) -> String + 'a>>,
    on_value_change: Option<Box<dyn Fn(Option<DateParts>) -> Message + 'a>>,
    on_open_change: Option<Box<dyn Fn(bool) -> Message + 'a>>,
    on_placeholder_change: Option<Box<dyn Fn(DateParts) -> Message + 'a>>,
}

impl<Message> fmt::Debug for DatePicker<'_, Message> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DatePicker")
            .field("value", &self.value)
            .field("open", &self.open)
            .field("close_on_select", &self.close_on_select)
            .field("disabled", &self.disabled)
            .field("on_value_change", &self.on_value_change.is_some())
            .field("on_open_change", &self.on_open_change.is_some())
            .finish_non_exhaustive()
    }
}

impl<'a, Message> DatePicker<'a, Message> {
    /// Creates a date picker with no date selected.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            value: None,
            placeholder_text: "Pick a date".to_owned(),
            placeholder_month: None,
            icon_position: DatePickerIconPosition::Leading,
            trigger_variant: ButtonVariant::Outline,
            trigger_width: Length::Fixed(shadcn_common::DATE_PICKER_TRIGGER_WIDTH_PX),
            caption_layout: CalendarCaptionLayout::Label,
            weekday_format: CalendarWeekdayFormat::Short,
            month_format: None,
            year_format: CalendarYearFormat::Numeric,
            min_value: None,
            max_value: None,
            week_starts_on: 0,
            fixed_weeks: false,
            number_of_months: 1,
            close_on_select: true,
            disabled: false,
            open: None,
            is_date_disabled: None,
            is_date_unavailable: None,
            format_date: None,
            on_value_change: None,
            on_open_change: None,
            on_placeholder_change: None,
        }
    }

    /// Sets the currently selected date.
    pub fn value(mut self, value: Option<DateParts>) -> Self {
        self.value = value;
        self
    }

    /// Sets the text shown when no date is selected.
    pub fn placeholder_text(mut self, text: impl Into<String>) -> Self {
        self.placeholder_text = text.into();
        self
    }

    /// Sets the month the calendar shows when opened.
    pub fn placeholder(mut self, month: DateParts) -> Self {
        self.placeholder_month = Some(month);
        self
    }

    /// Sets the calendar icon position relative to the label.
    pub fn icon_position(mut self, position: DatePickerIconPosition) -> Self {
        self.icon_position = position;
        self
    }

    /// Sets the trigger button variant (default Outline).
    pub fn trigger_variant(mut self, variant: ButtonVariant) -> Self {
        self.trigger_variant = variant;
        self
    }

    /// Sets the trigger button width.
    pub fn trigger_width(mut self, width: impl Into<Length>) -> Self {
        self.trigger_width = width.into();
        self
    }

    /// Sets the calendar caption layout.
    pub fn caption_layout(mut self, layout: CalendarCaptionLayout) -> Self {
        self.caption_layout = layout;
        self
    }

    /// Sets the weekday header label style.
    pub fn weekday_format(mut self, format: CalendarWeekdayFormat) -> Self {
        self.weekday_format = format;
        self
    }

    /// Sets the month label style.
    pub fn month_format(mut self, format: CalendarMonthFormat) -> Self {
        self.month_format = Some(format);
        self
    }

    /// Sets the year label style.
    pub fn year_format(mut self, format: CalendarYearFormat) -> Self {
        self.year_format = format;
        self
    }

    /// Sets the earliest selectable date.
    pub fn min_value(mut self, min: DateParts) -> Self {
        self.min_value = Some(min);
        self
    }

    /// Sets the latest selectable date.
    pub fn max_value(mut self, max: DateParts) -> Self {
        self.max_value = Some(max);
        self
    }

    /// Sets the first day of the week (`0 = Sunday`).
    pub fn week_starts_on(mut self, day: u8) -> Self {
        self.week_starts_on = day % 7;
        self
    }

    /// Always renders six week rows.
    pub fn fixed_weeks(mut self, fixed: bool) -> Self {
        self.fixed_weeks = fixed;
        self
    }

    /// Sets how many months are shown at once.
    pub fn number_of_months(mut self, count: usize) -> Self {
        self.number_of_months = count.max(1);
        self
    }

    /// Whether the popover closes automatically after a date is picked.
    pub fn close_on_select(mut self, close: bool) -> Self {
        self.close_on_select = close;
        self
    }

    /// Disables the trigger button and calendar.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Controls the popover open state explicitly.
    pub fn open(mut self, open: bool) -> Self {
        self.open = Some(open);
        self
    }

    /// Controls the open state when `Some`, uncontrolled when `None`.
    pub fn open_maybe(mut self, open: Option<bool>) -> Self {
        self.open = open;
        self
    }

    /// Marks matching dates as disabled.
    pub fn is_date_disabled(mut self, matcher: impl Fn(DateParts) -> bool + 'a) -> Self {
        self.is_date_disabled = Some(Box::new(matcher));
        self
    }

    /// Marks matching dates as unavailable.
    pub fn is_date_unavailable(mut self, matcher: impl Fn(DateParts) -> bool + 'a) -> Self {
        self.is_date_unavailable = Some(Box::new(matcher));
        self
    }

    /// Overrides the date format shown on the trigger button.
    pub fn format_date(mut self, formatter: impl Fn(DateParts) -> String + 'a) -> Self {
        self.format_date = Some(Box::new(formatter));
        self
    }

    /// Callback when the selected date changes.
    pub fn on_value_change(mut self, callback: impl Fn(Option<DateParts>) -> Message + 'a) -> Self {
        self.on_value_change = Some(Box::new(callback));
        self
    }

    /// Callback when the popover open state changes.
    pub fn on_open_change(mut self, callback: impl Fn(bool) -> Message + 'a) -> Self {
        self.on_open_change = Some(Box::new(callback));
        self
    }

    /// Callback when the calendar navigates to a different month.
    pub fn on_placeholder_change(mut self, callback: impl Fn(DateParts) -> Message + 'a) -> Self {
        self.on_placeholder_change = Some(Box::new(callback));
        self
    }
}

/// Builder-first date-range picker.
///
/// Composes a [`crate::Popover`] with a [`crate::RangeCalendar`] inside.
#[must_use = "builders do nothing unless turned into an iced Element"]
#[allow(clippy::type_complexity)]
pub struct DateRangePicker<'a, Message> {
    theme: &'a Theme,
    value: DateRange,
    placeholder_text: String,
    placeholder_month: Option<DateParts>,
    icon_position: DatePickerIconPosition,
    trigger_variant: ButtonVariant,
    trigger_width: Length,
    caption_layout: CalendarCaptionLayout,
    min_value: Option<DateParts>,
    max_value: Option<DateParts>,
    week_starts_on: u8,
    fixed_weeks: bool,
    number_of_months: usize,
    min_days: Option<usize>,
    max_days: Option<usize>,
    close_on_select: bool,
    disabled: bool,
    open: Option<bool>,
    is_date_disabled: Option<Box<dyn Fn(DateParts) -> bool + 'a>>,
    is_date_unavailable: Option<Box<dyn Fn(DateParts) -> bool + 'a>>,
    format_range: Option<Box<dyn Fn(&DateRange) -> String + 'a>>,
    on_value_change: Option<Box<dyn Fn(DateRange) -> Message + 'a>>,
    on_open_change: Option<Box<dyn Fn(bool) -> Message + 'a>>,
    on_placeholder_change: Option<Box<dyn Fn(DateParts) -> Message + 'a>>,
}

impl<Message> fmt::Debug for DateRangePicker<'_, Message> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DateRangePicker")
            .field("value", &self.value)
            .field("open", &self.open)
            .field("disabled", &self.disabled)
            .field("on_value_change", &self.on_value_change.is_some())
            .field("on_open_change", &self.on_open_change.is_some())
            .finish_non_exhaustive()
    }
}

impl<'a, Message> DateRangePicker<'a, Message> {
    /// Creates a date-range picker with no range selected.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            value: DateRange::default(),
            placeholder_text: "Pick a date".to_owned(),
            placeholder_month: None,
            icon_position: DatePickerIconPosition::Leading,
            trigger_variant: ButtonVariant::Outline,
            trigger_width: Length::Fixed(shadcn_common::DATE_PICKER_RANGE_TRIGGER_WIDTH_PX),
            caption_layout: CalendarCaptionLayout::Label,
            min_value: None,
            max_value: None,
            week_starts_on: 0,
            fixed_weeks: false,
            number_of_months: 2,
            min_days: None,
            max_days: None,
            close_on_select: false,
            disabled: false,
            open: None,
            is_date_disabled: None,
            is_date_unavailable: None,
            format_range: None,
            on_value_change: None,
            on_open_change: None,
            on_placeholder_change: None,
        }
    }

    /// Sets the current range value.
    pub fn value(mut self, value: DateRange) -> Self {
        self.value = value;
        self
    }

    /// Sets the text shown when no range is selected.
    pub fn placeholder_text(mut self, text: impl Into<String>) -> Self {
        self.placeholder_text = text.into();
        self
    }

    /// Sets the month the calendar shows when opened.
    pub fn placeholder(mut self, month: DateParts) -> Self {
        self.placeholder_month = Some(month);
        self
    }

    /// Sets the calendar icon position.
    pub fn icon_position(mut self, position: DatePickerIconPosition) -> Self {
        self.icon_position = position;
        self
    }

    /// Sets the trigger button variant.
    pub fn trigger_variant(mut self, variant: ButtonVariant) -> Self {
        self.trigger_variant = variant;
        self
    }

    /// Sets the trigger button width.
    pub fn trigger_width(mut self, width: impl Into<Length>) -> Self {
        self.trigger_width = width.into();
        self
    }

    /// Sets the calendar caption layout.
    pub fn caption_layout(mut self, layout: CalendarCaptionLayout) -> Self {
        self.caption_layout = layout;
        self
    }

    /// Sets the earliest selectable date.
    pub fn min_value(mut self, min: DateParts) -> Self {
        self.min_value = Some(min);
        self
    }

    /// Sets the latest selectable date.
    pub fn max_value(mut self, max: DateParts) -> Self {
        self.max_value = Some(max);
        self
    }

    /// Sets the first day of the week.
    pub fn week_starts_on(mut self, day: u8) -> Self {
        self.week_starts_on = day % 7;
        self
    }

    /// Always renders six week rows.
    pub fn fixed_weeks(mut self, fixed: bool) -> Self {
        self.fixed_weeks = fixed;
        self
    }

    /// Sets how many months are shown at once (default 2).
    pub fn number_of_months(mut self, count: usize) -> Self {
        self.number_of_months = count.max(1);
        self
    }

    /// Minimum days in the selected range.
    pub fn min_days(mut self, days: usize) -> Self {
        self.min_days = (days > 0).then_some(days);
        self
    }

    /// Maximum days in the selected range.
    pub fn max_days(mut self, days: usize) -> Self {
        self.max_days = (days > 0).then_some(days);
        self
    }

    /// Whether the popover closes after a complete range is selected.
    pub fn close_on_select(mut self, close: bool) -> Self {
        self.close_on_select = close;
        self
    }

    /// Disables the picker.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Controls the popover open state explicitly.
    pub fn open(mut self, open: bool) -> Self {
        self.open = Some(open);
        self
    }

    /// Controls the open state when `Some`.
    pub fn open_maybe(mut self, open: Option<bool>) -> Self {
        self.open = open;
        self
    }

    /// Marks matching dates as disabled.
    pub fn is_date_disabled(mut self, matcher: impl Fn(DateParts) -> bool + 'a) -> Self {
        self.is_date_disabled = Some(Box::new(matcher));
        self
    }

    /// Marks matching dates as unavailable.
    pub fn is_date_unavailable(mut self, matcher: impl Fn(DateParts) -> bool + 'a) -> Self {
        self.is_date_unavailable = Some(Box::new(matcher));
        self
    }

    /// Overrides the range format shown on the trigger button.
    pub fn format_range(mut self, formatter: impl Fn(&DateRange) -> String + 'a) -> Self {
        self.format_range = Some(Box::new(formatter));
        self
    }

    /// Callback when the selected range changes.
    pub fn on_value_change(mut self, callback: impl Fn(DateRange) -> Message + 'a) -> Self {
        self.on_value_change = Some(Box::new(callback));
        self
    }

    /// Callback when the popover open state changes.
    pub fn on_open_change(mut self, callback: impl Fn(bool) -> Message + 'a) -> Self {
        self.on_open_change = Some(Box::new(callback));
        self
    }

    /// Callback when the calendar navigates to a different month.
    pub fn on_placeholder_change(mut self, callback: impl Fn(DateParts) -> Message + 'a) -> Self {
        self.on_placeholder_change = Some(Box::new(callback));
        self
    }
}

/// Convenience: creates a [`DatePicker`].
pub fn date_picker<Message>(theme: &Theme) -> DatePicker<'_, Message> {
    DatePicker::new(theme)
}

/// Convenience: creates a [`DateRangePicker`].
pub fn date_range_picker<Message>(theme: &Theme) -> DateRangePicker<'_, Message> {
    DateRangePicker::new(theme)
}

impl<'a, Message> From<DatePicker<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(picker: DatePicker<'a, Message>) -> Self {
        render::build_date_picker(picker)
    }
}

impl<'a, Message> From<DateRangePicker<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(picker: DateRangePicker<'a, Message>) -> Self {
        render::build_date_range_picker(picker)
    }
}
