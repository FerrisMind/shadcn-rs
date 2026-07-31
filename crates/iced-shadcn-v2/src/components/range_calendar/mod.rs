//! Range-calendar component ported from shadcn-svelte to iced-shadcn-v2.
//!
//! Port of the shadcn-svelte range-calendar (bits-ui `RangeCalendar.Root` +
//! `.cn-calendar`): month grid with a two-click date-range selection, prev/next
//! paging, and a caption layout identical to [`crate::Calendar`]. The visual
//! difference from the single-selection calendar is the range "band": middle
//! days receive an accent background with square corners; endpoints receive a
//! primary fill with rounded corners on the outward-facing side.
//!
//! The range-calendar is fully controlled: the application owns a
//! [`DateRange`] and the visible month, feeding both back on every change.
//!
//! ```rust,no_run
//! use iced::Element;
//! use iced_shadcn_v2::{DateParts, DateRange, RangeCalendar, Theme};
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     RangeChanged(DateRange),
//!     MonthShown(DateParts),
//! }
//!
//! fn view<'a>(
//!     theme: &'a Theme,
//!     range: DateRange,
//!     month: DateParts,
//! ) -> Element<'a, Message> {
//!     RangeCalendar::new(theme)
//!         .value(range)
//!         .placeholder(month)
//!         .on_value_change(Message::RangeChanged)
//!         .on_placeholder_change(Message::MonthShown)
//!         .into()
//! }
//! ```

mod render;

#[cfg(test)]
mod tests;

pub use shadcn_common::{DateRange, RangeDayPosition};

use std::fmt;
use std::rc::Rc;

use shadcn_common::DateParts;

use crate::components::button::ButtonVariant;
use crate::components::calendar::{
    CalendarCaptionLayout, CalendarMonthFormat, CalendarWeekdayFormat, CalendarYearFormat,
};
use crate::iced_compat::Element;
use crate::theme::Theme;

/// Builder-first range-calendar styled directly with iced types.
///
/// Theme tokens come from `shadcn-common` via [`Theme`]. Pass `&theme` into
/// every range-calendar. The application owns the [`DateRange`] and the
/// visible month, feeding them back through [`Self::value`] /
/// [`Self::placeholder`] on every change, mirroring `bind:value` /
/// `bind:placeholder`.
///
/// ```rust,no_run
/// use iced::Element;
/// use iced_shadcn_v2::{
///     DateParts, DateRange, RangeCalendar, CalendarCaptionLayout, Theme,
/// };
///
/// #[derive(Debug, Clone)]
/// enum Message {
///     Range(DateRange),
///     Month(DateParts),
/// }
///
/// fn booking<'a>(
///     theme: &'a Theme,
///     range: DateRange,
///     month: DateParts,
/// ) -> Element<'a, Message> {
///     RangeCalendar::new(theme)
///         .value(range)
///         .placeholder(month)
///         .caption_layout(CalendarCaptionLayout::Dropdown)
///         .number_of_months(2)
///         .min_days(3)
///         .max_days(14)
///         .on_value_change(Message::Range)
///         .on_placeholder_change(Message::Month)
///         .into()
/// }
/// ```
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct RangeCalendar<'a, Message> {
    pub(super) theme: &'a Theme,
    pub(super) value: DateRange,
    pub(super) placeholder: Option<DateParts>,
    pub(super) today: Option<DateParts>,
    pub(super) caption_layout: CalendarCaptionLayout,
    pub(super) button_variant: ButtonVariant,
    pub(super) weekday_format: CalendarWeekdayFormat,
    pub(super) month_format: Option<CalendarMonthFormat>,
    pub(super) year_format: CalendarYearFormat,
    pub(super) months: Option<Vec<u8>>,
    pub(super) years: Option<Vec<i32>>,
    pub(super) min_value: Option<DateParts>,
    pub(super) max_value: Option<DateParts>,
    pub(super) number_of_months: usize,
    pub(super) paged_navigation: bool,
    pub(super) fixed_weeks: bool,
    pub(super) week_starts_on: u8,
    pub(super) disable_days_outside_month: bool,
    pub(super) prevent_deselect: bool,
    pub(super) min_days: Option<usize>,
    pub(super) max_days: Option<usize>,
    pub(super) disabled: bool,
    pub(super) readonly: bool,
    pub(super) transparent: bool,
    pub(super) bordered: bool,
    pub(super) exclude_disabled: bool,
    pub(super) is_date_disabled: Option<Box<dyn Fn(DateParts) -> bool + 'a>>,
    pub(super) is_date_unavailable: Option<Box<dyn Fn(DateParts) -> bool + 'a>>,
    pub(super) month_label: Option<Box<dyn Fn(u8) -> String + 'a>>,
    pub(super) year_label: Option<Box<dyn Fn(i32) -> String + 'a>>,
    pub(super) weekday_label: Option<Box<dyn Fn(u8) -> String + 'a>>,
    pub(super) on_value_change: Option<Box<dyn Fn(DateRange) -> Message + 'a>>,
    pub(super) on_start_value_change: Option<Box<dyn Fn(Option<DateParts>) -> Message + 'a>>,
    pub(super) on_end_value_change: Option<Box<dyn Fn(Option<DateParts>) -> Message + 'a>>,
    pub(super) on_placeholder_change: Option<Rc<dyn Fn(DateParts) -> Message + 'a>>,
}

impl<Message> fmt::Debug for RangeCalendar<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RangeCalendar")
            .field("theme", &self.theme)
            .field("value", &self.value)
            .field("placeholder", &self.placeholder)
            .field("today", &self.today)
            .field("caption_layout", &self.caption_layout)
            .field("button_variant", &self.button_variant)
            .field("number_of_months", &self.number_of_months)
            .field("min_days", &self.min_days)
            .field("max_days", &self.max_days)
            .field("disabled", &self.disabled)
            .field("readonly", &self.readonly)
            .field("exclude_disabled", &self.exclude_disabled)
            .field("on_value_change", &self.on_value_change.is_some())
            .field(
                "on_placeholder_change",
                &self.on_placeholder_change.is_some(),
            )
            .finish_non_exhaustive()
    }
}

impl<'a, Message> RangeCalendar<'a, Message> {
    /// Creates a range-calendar showing the current (UTC) month with no
    /// range selected.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            value: DateRange::default(),
            placeholder: None,
            today: None,
            caption_layout: CalendarCaptionLayout::Label,
            button_variant: ButtonVariant::Ghost,
            weekday_format: CalendarWeekdayFormat::Short,
            month_format: None,
            year_format: CalendarYearFormat::Numeric,
            months: None,
            years: None,
            min_value: None,
            max_value: None,
            number_of_months: 1,
            paged_navigation: false,
            fixed_weeks: false,
            week_starts_on: 0,
            disable_days_outside_month: false,
            prevent_deselect: false,
            min_days: None,
            max_days: None,
            disabled: false,
            readonly: false,
            transparent: false,
            bordered: false,
            exclude_disabled: false,
            is_date_disabled: None,
            is_date_unavailable: None,
            month_label: None,
            year_label: None,
            weekday_label: None,
            on_value_change: None,
            on_start_value_change: None,
            on_end_value_change: None,
            on_placeholder_change: None,
        }
    }

    /// Sets the controlled range value (`bind:value`).
    pub fn value(mut self, value: DateRange) -> Self {
        self.value = value;
        self
    }

    /// Sets just the start of the range.
    pub fn start(mut self, start: DateParts) -> Self {
        self.value.start = Some(start);
        self
    }

    /// Sets just the end of the range.
    pub fn end(mut self, end: DateParts) -> Self {
        self.value.end = Some(end);
        self
    }

    /// Sets the visible month (`bind:placeholder`).
    pub fn placeholder(mut self, placeholder: DateParts) -> Self {
        self.placeholder = Some(placeholder);
        self
    }

    /// Overrides the date highlighted as "today".
    pub fn today(mut self, today: DateParts) -> Self {
        self.today = Some(today);
        self
    }

    /// Sets how the month/year caption is rendered (`captionLayout`).
    pub fn caption_layout(mut self, caption_layout: CalendarCaptionLayout) -> Self {
        self.caption_layout = caption_layout;
        self
    }

    /// Sets the [`ButtonVariant`] of the prev/next buttons.
    pub fn button_variant(mut self, button_variant: ButtonVariant) -> Self {
        self.button_variant = button_variant;
        self
    }

    /// Sets the weekday header label style (`weekdayFormat`).
    pub fn weekday_format(mut self, weekday_format: CalendarWeekdayFormat) -> Self {
        self.weekday_format = weekday_format;
        self
    }

    /// Sets the month label style (`monthFormat`).
    pub fn month_format(mut self, month_format: CalendarMonthFormat) -> Self {
        self.month_format = Some(month_format);
        self
    }

    /// Sets the year label style (`yearFormat`).
    pub fn year_format(mut self, year_format: CalendarYearFormat) -> Self {
        self.year_format = year_format;
        self
    }

    /// Restricts the month dropdown to the given month numbers.
    pub fn months(mut self, months: impl IntoIterator<Item = u8>) -> Self {
        self.months = Some(
            months
                .into_iter()
                .filter(|month| (1..=12).contains(month))
                .collect(),
        );
        self
    }

    /// Sets the year list of the year dropdown.
    pub fn years(mut self, years: impl IntoIterator<Item = i32>) -> Self {
        self.years = Some(years.into_iter().collect());
        self
    }

    /// Sets the earliest selectable date (`minValue`).
    pub fn min_value(mut self, min_value: DateParts) -> Self {
        self.min_value = Some(min_value);
        self
    }

    /// Sets the latest selectable date (`maxValue`).
    pub fn max_value(mut self, max_value: DateParts) -> Self {
        self.max_value = Some(max_value);
        self
    }

    /// Sets how many consecutive months are displayed (`numberOfMonths`).
    pub fn number_of_months(mut self, number_of_months: usize) -> Self {
        self.number_of_months = number_of_months.max(1);
        self
    }

    /// Makes prev/next shift the view by a whole page of months.
    pub fn paged_navigation(mut self, paged_navigation: bool) -> Self {
        self.paged_navigation = paged_navigation;
        self
    }

    /// Always renders six week rows per month (`fixedWeeks`).
    pub fn fixed_weeks(mut self, fixed_weeks: bool) -> Self {
        self.fixed_weeks = fixed_weeks;
        self
    }

    /// Sets the first day of the week (`weekStartsOn`).
    pub fn week_starts_on(mut self, week_starts_on: u8) -> Self {
        self.week_starts_on = week_starts_on % 7;
        self
    }

    /// Makes days outside the current month inert.
    pub fn disable_days_outside_month(mut self, disable_days_outside_month: bool) -> Self {
        self.disable_days_outside_month = disable_days_outside_month;
        self
    }

    /// Prevents clearing the range by clicking the endpoint again.
    pub fn prevent_deselect(mut self, prevent_deselect: bool) -> Self {
        self.prevent_deselect = prevent_deselect;
        self
    }

    /// Minimum number of days that can be selected in a range (`minDays`).
    pub fn min_days(mut self, min_days: usize) -> Self {
        self.min_days = (min_days > 0).then_some(min_days);
        self
    }

    /// Maximum number of days that can be selected in a range (`maxDays`).
    pub fn max_days(mut self, max_days: usize) -> Self {
        self.max_days = (max_days > 0).then_some(max_days);
        self
    }

    /// Disables the whole calendar.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Makes the calendar readonly.
    pub fn readonly(mut self, readonly: bool) -> Self {
        self.readonly = readonly;
        self
    }

    /// Skips painting the background fill.
    pub fn transparent(mut self, transparent: bool) -> Self {
        self.transparent = transparent;
        self
    }

    /// Wraps the calendar in demo chrome (`rounded-md border shadow-sm`).
    pub fn bordered(mut self, bordered: bool) -> Self {
        self.bordered = bordered;
        self
    }

    /// Auto-clears the range if a disabled date appears inside it.
    pub fn exclude_disabled(mut self, exclude_disabled: bool) -> Self {
        self.exclude_disabled = exclude_disabled;
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

    /// Overrides month labels with a custom formatter.
    pub fn month_label_with(mut self, label: impl Fn(u8) -> String + 'a) -> Self {
        self.month_label = Some(Box::new(label));
        self
    }

    /// Overrides year labels with a custom formatter.
    pub fn year_label_with(mut self, label: impl Fn(i32) -> String + 'a) -> Self {
        self.year_label = Some(Box::new(label));
        self
    }

    /// Overrides weekday header labels with a custom formatter.
    pub fn weekday_label_with(mut self, label: impl Fn(u8) -> String + 'a) -> Self {
        self.weekday_label = Some(Box::new(label));
        self
    }

    /// Sets the callback receiving the next range value (`onValueChange`).
    pub fn on_value_change(mut self, on_value_change: impl Fn(DateRange) -> Message + 'a) -> Self {
        self.on_value_change = Some(Box::new(on_value_change));
        self
    }

    /// Cosmetic callback fired when the start value changes
    /// (`onStartValueChange`).
    pub fn on_start_value_change(
        mut self,
        callback: impl Fn(Option<DateParts>) -> Message + 'a,
    ) -> Self {
        self.on_start_value_change = Some(Box::new(callback));
        self
    }

    /// Cosmetic callback fired when the end value changes
    /// (`onEndValueChange`).
    pub fn on_end_value_change(
        mut self,
        callback: impl Fn(Option<DateParts>) -> Message + 'a,
    ) -> Self {
        self.on_end_value_change = Some(Box::new(callback));
        self
    }

    /// Sets the callback receiving the new visible month after navigation.
    pub fn on_placeholder_change(
        mut self,
        on_placeholder_change: impl Fn(DateParts) -> Message + 'a,
    ) -> Self {
        self.on_placeholder_change = Some(Rc::new(on_placeholder_change));
        self
    }
}

/// Convenience wrapper.
///
/// ```rust,no_run
/// use iced::Element;
/// use iced_shadcn_v2::{DateRange, Theme, range_calendar};
///
/// #[derive(Debug, Clone)]
/// enum Message {
///     Range(DateRange),
/// }
///
/// fn view(theme: &Theme) -> Element<'_, Message> {
///     range_calendar(theme).on_value_change(Message::Range).into()
/// }
/// ```
pub fn range_calendar<Message>(theme: &Theme) -> RangeCalendar<'_, Message> {
    RangeCalendar::new(theme)
}

impl<'a, Message> From<RangeCalendar<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(range_calendar: RangeCalendar<'a, Message>) -> Self {
        render::build_range_calendar(range_calendar)
    }
}
