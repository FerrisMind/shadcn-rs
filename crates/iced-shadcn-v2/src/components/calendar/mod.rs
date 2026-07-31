//! Calendar component ported from shadcn-svelte to iced-shadcn-v2.
//!
//! Port of the shadcn-svelte calendar (bits-ui `Calendar.Root` + the
//! `.cn-calendar` wrapper): month grid with selectable days, prev/next
//! paging, and a caption that is either a "Month Year" label or month/year
//! dropdowns (`captionLayout`). Nav buttons reuse [`Button`] variants and the
//! caption dropdowns reuse [`crate::Select`], mirroring how the web wrapper
//! composes `buttonVariants` and native selects.
//!
//! The calendar is fully controlled, like `bind:value` / `bind:placeholder`
//! on the web: the application owns the [`CalendarSelection`] and the
//! visible month, and feeds both back on every change.
//!
//! ```rust,no_run
//! use iced::Element;
//! use iced_shadcn_v2::{Calendar, DateParts, Theme};
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     DatePicked(Option<DateParts>),
//!     MonthShown(DateParts),
//! }
//!
//! fn view<'a>(
//!     theme: &'a Theme,
//!     selected: Option<DateParts>,
//!     month: DateParts,
//! ) -> Element<'a, Message> {
//!     Calendar::new(theme)
//!         .selected_maybe(selected)
//!         .placeholder(month)
//!         .on_selection_change(|selection| Message::DatePicked(selection.as_single()))
//!         .on_placeholder_change(Message::MonthShown)
//!         .into()
//! }
//! ```

mod render;

#[cfg(test)]
mod tests;

/// Backend-agnostic calendar value and configuration types re-exported from
/// `shadcn-common` so iced consumers can import them from this crate.
pub use shadcn_common::{
    CalendarCaptionLayout, CalendarMonthFormat, CalendarSelection, CalendarWeekdayFormat,
    CalendarYearFormat,
};

use std::fmt;
use std::rc::Rc;

use shadcn_common::DateParts;

use crate::components::button::ButtonVariant;
use crate::iced_compat::Element;
use crate::theme::Theme;

/// Builder-first calendar styled directly with iced types.
///
/// Theme tokens come from `shadcn-common` via [`Theme`]. Pass `&theme` into
/// every calendar — style packs live on the app's [`Theme`], not on this
/// builder. The application owns the selection and the visible month and
/// feeds them back through [`Self::selection`] / [`Self::placeholder`] on
/// every change, mirroring `bind:value` / `bind:placeholder`.
///
/// Day presses report the next state through
/// [`Self::on_selection_change`] (full snapshot, preferred) or
/// [`Self::on_select`] (just the picked day). Month navigation — the
/// prev/next buttons and the caption dropdowns — reports the new visible
/// month through [`Self::on_placeholder_change`]; without that callback the
/// navigation controls are disabled.
///
/// ```rust,no_run
/// use iced::Element;
/// use iced_shadcn_v2::{Calendar, CalendarCaptionLayout, CalendarSelection, DateParts, Theme};
///
/// #[derive(Debug, Clone)]
/// enum Message {
///     Selected(CalendarSelection),
///     MonthShown(DateParts),
/// }
///
/// fn booking<'a>(
///     theme: &'a Theme,
///     selection: CalendarSelection,
///     month: DateParts,
/// ) -> Element<'a, Message> {
///     Calendar::new(theme)
///         .selection(selection)
///         .placeholder(month)
///         .caption_layout(CalendarCaptionLayout::Dropdown)
///         .max_days(5)
///         .is_date_unavailable(|date| date.day == 13)
///         .on_selection_change(Message::Selected)
///         .on_placeholder_change(Message::MonthShown)
///         .into()
/// }
/// ```
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct Calendar<'a, Message> {
    pub(super) theme: &'a Theme,
    pub(super) selection: CalendarSelection,
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
    pub(super) max_days: Option<usize>,
    pub(super) disabled: bool,
    pub(super) readonly: bool,
    pub(super) transparent: bool,
    pub(super) bordered: bool,
    pub(super) is_date_disabled: Option<Box<dyn Fn(DateParts) -> bool + 'a>>,
    pub(super) is_date_unavailable: Option<Box<dyn Fn(DateParts) -> bool + 'a>>,
    pub(super) month_label: Option<Box<dyn Fn(u8) -> String + 'a>>,
    pub(super) year_label: Option<Box<dyn Fn(i32) -> String + 'a>>,
    pub(super) weekday_label: Option<Box<dyn Fn(u8) -> String + 'a>>,
    pub(super) on_select: Option<Box<dyn Fn(DateParts) -> Message + 'a>>,
    pub(super) on_selection_change: Option<Box<dyn Fn(CalendarSelection) -> Message + 'a>>,
    pub(super) on_placeholder_change: Option<Rc<dyn Fn(DateParts) -> Message + 'a>>,
}

impl<Message> fmt::Debug for Calendar<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Calendar")
            .field("theme", &self.theme)
            .field("selection", &self.selection)
            .field("placeholder", &self.placeholder)
            .field("today", &self.today)
            .field("caption_layout", &self.caption_layout)
            .field("button_variant", &self.button_variant)
            .field("weekday_format", &self.weekday_format)
            .field("month_format", &self.month_format)
            .field("year_format", &self.year_format)
            .field("months", &self.months)
            .field("years", &self.years)
            .field("min_value", &self.min_value)
            .field("max_value", &self.max_value)
            .field("number_of_months", &self.number_of_months)
            .field("paged_navigation", &self.paged_navigation)
            .field("fixed_weeks", &self.fixed_weeks)
            .field("week_starts_on", &self.week_starts_on)
            .field(
                "disable_days_outside_month",
                &self.disable_days_outside_month,
            )
            .field("prevent_deselect", &self.prevent_deselect)
            .field("max_days", &self.max_days)
            .field("disabled", &self.disabled)
            .field("readonly", &self.readonly)
            .field("transparent", &self.transparent)
            .field("bordered", &self.bordered)
            .field("is_date_disabled", &self.is_date_disabled.is_some())
            .field("is_date_unavailable", &self.is_date_unavailable.is_some())
            .field("on_select", &self.on_select.is_some())
            .field("on_selection_change", &self.on_selection_change.is_some())
            .field(
                "on_placeholder_change",
                &self.on_placeholder_change.is_some(),
            )
            .finish_non_exhaustive()
    }
}

impl<'a, Message> Calendar<'a, Message> {
    /// Creates a calendar showing the current (UTC) month with nothing
    /// selected.
    ///
    /// `theme` is required because styling is derived from `shadcn-common`
    /// theme tokens instead of `iced::Theme`.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            selection: CalendarSelection::default(),
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
            max_days: None,
            disabled: false,
            readonly: false,
            transparent: false,
            bordered: false,
            is_date_disabled: None,
            is_date_unavailable: None,
            month_label: None,
            year_label: None,
            weekday_label: None,
            on_select: None,
            on_selection_change: None,
            on_placeholder_change: None,
        }
    }

    /// Sets the controlled selection snapshot (`bind:value`).
    pub fn selection(mut self, selection: CalendarSelection) -> Self {
        self.selection = selection;
        self
    }

    /// Sets the selected date in single mode (`type="single"`).
    pub fn selected(mut self, selected: DateParts) -> Self {
        self.selection = CalendarSelection::single(Some(selected));
        self
    }

    /// Sets or clears the selected date in single mode.
    pub fn selected_maybe(mut self, selected: Option<DateParts>) -> Self {
        self.selection = CalendarSelection::single(selected);
        self
    }

    /// Sets the selected dates in multiple mode (`type="multiple"`).
    pub fn values(mut self, values: impl IntoIterator<Item = DateParts>) -> Self {
        self.selection = CalendarSelection::multiple(values);
        self
    }

    /// Sets the month shown when no navigation happened yet
    /// (`bind:placeholder`). Defaults to the selected date's month, or the
    /// current month.
    pub fn placeholder(mut self, placeholder: DateParts) -> Self {
        self.placeholder = Some(placeholder);
        self
    }

    /// Overrides the date highlighted as "today".
    ///
    /// Defaults to the current date in UTC
    /// ([`shadcn_common::calendar_today_utc`]); pass the local date when the
    /// app knows its timezone.
    pub fn today(mut self, today: DateParts) -> Self {
        self.today = Some(today);
        self
    }

    /// Sets how the month/year caption is rendered (`captionLayout`).
    pub fn caption_layout(mut self, caption_layout: CalendarCaptionLayout) -> Self {
        self.caption_layout = caption_layout;
        self
    }

    /// Sets the [`ButtonVariant`] of the prev/next buttons (`buttonVariant`,
    /// default ghost).
    pub fn button_variant(mut self, button_variant: ButtonVariant) -> Self {
        self.button_variant = button_variant;
        self
    }

    /// Sets the weekday header label style (`weekdayFormat`).
    ///
    /// Like the web wrapper, non-narrow labels are truncated to their first
    /// two letters ("Su", "Mo", …).
    pub fn weekday_format(mut self, weekday_format: CalendarWeekdayFormat) -> Self {
        self.weekday_format = weekday_format;
        self
    }

    /// Sets the month label style (`monthFormat`).
    ///
    /// Defaults to short names when the caption uses dropdowns and long
    /// names otherwise, matching the web wrapper.
    pub fn month_format(mut self, month_format: CalendarMonthFormat) -> Self {
        self.month_format = Some(month_format);
        self
    }

    /// Sets the year label style (`yearFormat`).
    pub fn year_format(mut self, year_format: CalendarYearFormat) -> Self {
        self.year_format = year_format;
        self
    }

    /// Restricts the month dropdown to the given month numbers (`months`),
    /// e.g. only quarters or seasons. Values outside `1..=12` are ignored.
    pub fn months(mut self, months: impl IntoIterator<Item = u8>) -> Self {
        self.months = Some(
            months
                .into_iter()
                .filter(|month| (1..=12).contains(month))
                .collect(),
        );
        self
    }

    /// Sets the year list of the year dropdown (`years`).
    ///
    /// Defaults to the bits-ui window derived from today, the placeholder,
    /// and the min/max bounds ([`shadcn_common::calendar_default_years`]).
    pub fn years(mut self, years: impl IntoIterator<Item = i32>) -> Self {
        self.years = Some(years.into_iter().collect());
        self
    }

    /// Sets the earliest selectable date (`minValue`). Also disables paging
    /// into months that end before it.
    pub fn min_value(mut self, min_value: DateParts) -> Self {
        self.min_value = Some(min_value);
        self
    }

    /// Sets the latest selectable date (`maxValue`). Also disables paging
    /// into months that start after it.
    pub fn max_value(mut self, max_value: DateParts) -> Self {
        self.max_value = Some(max_value);
        self
    }

    /// Sets how many consecutive months are displayed (`numberOfMonths`).
    /// Zero is treated as one.
    pub fn number_of_months(mut self, number_of_months: usize) -> Self {
        self.number_of_months = number_of_months.max(1);
        self
    }

    /// Makes prev/next shift the view by a whole page of months instead of
    /// one month (`pagedNavigation`).
    pub fn paged_navigation(mut self, paged_navigation: bool) -> Self {
        self.paged_navigation = paged_navigation;
        self
    }

    /// Always renders six week rows per month (`fixedWeeks`), so the
    /// calendar height never changes between months.
    pub fn fixed_weeks(mut self, fixed_weeks: bool) -> Self {
        self.fixed_weeks = fixed_weeks;
        self
    }

    /// Sets the first day of the week (`weekStartsOn`): `0 = Sunday` …
    /// `6 = Saturday`. Values above six wrap around.
    pub fn week_starts_on(mut self, week_starts_on: u8) -> Self {
        self.week_starts_on = week_starts_on % 7;
        self
    }

    /// Makes days outside the current month inert
    /// (`disableDaysOutsideMonth`).
    pub fn disable_days_outside_month(mut self, disable_days_outside_month: bool) -> Self {
        self.disable_days_outside_month = disable_days_outside_month;
        self
    }

    /// Prevents clearing the selection by clicking the selected date
    /// (`preventDeselect`).
    pub fn prevent_deselect(mut self, prevent_deselect: bool) -> Self {
        self.prevent_deselect = prevent_deselect;
        self
    }

    /// Caps how many dates can be selected in multiple mode (`maxDays`).
    /// Exceeding the cap resets the selection to the newly picked date;
    /// zero removes the cap.
    pub fn max_days(mut self, max_days: usize) -> Self {
        self.max_days = (max_days > 0).then_some(max_days);
        self
    }

    /// Disables the whole calendar (`disabled`): days dim and nothing is
    /// interactive.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Makes the calendar readonly (`readonly`): days keep their normal
    /// look but clicks no longer change the selection.
    pub fn readonly(mut self, readonly: bool) -> Self {
        self.readonly = readonly;
        self
    }

    /// Skips painting the `bg-background` fill, for embedding inside cards
    /// and popovers (`in-data-[slot=popover-content]:bg-transparent`).
    pub fn transparent(mut self, transparent: bool) -> Self {
        self.transparent = transparent;
        self
    }

    /// Wraps the calendar in the demo chrome (`rounded-lg border shadow-sm`).
    pub fn bordered(mut self, bordered: bool) -> Self {
        self.bordered = bordered;
        self
    }

    /// Marks matching dates as disabled (`isDateDisabled`): dimmed and not
    /// selectable. Dates outside [`Self::min_value`] / [`Self::max_value`]
    /// are disabled regardless of this matcher.
    pub fn is_date_disabled(mut self, matcher: impl Fn(DateParts) -> bool + 'a) -> Self {
        self.is_date_disabled = Some(Box::new(matcher));
        self
    }

    /// Marks matching dates as unavailable (`isDateUnavailable`): struck
    /// through and not selectable.
    pub fn is_date_unavailable(mut self, matcher: impl Fn(DateParts) -> bool + 'a) -> Self {
        self.is_date_unavailable = Some(Box::new(matcher));
        self
    }

    /// Overrides month labels with a custom formatter, e.g. for
    /// localization (the function form of `monthFormat`).
    pub fn month_label_with(mut self, label: impl Fn(u8) -> String + 'a) -> Self {
        self.month_label = Some(Box::new(label));
        self
    }

    /// Overrides year labels with a custom formatter (the function form of
    /// `yearFormat`).
    pub fn year_label_with(mut self, label: impl Fn(i32) -> String + 'a) -> Self {
        self.year_label = Some(Box::new(label));
        self
    }

    /// Overrides weekday header labels with a custom formatter receiving
    /// `0 = Sunday` … `6 = Saturday`. Labels are rendered verbatim (no
    /// two-letter truncation).
    pub fn weekday_label_with(mut self, label: impl Fn(u8) -> String + 'a) -> Self {
        self.weekday_label = Some(Box::new(label));
        self
    }

    /// Sets the callback receiving the picked day.
    ///
    /// Fires for every accepted pick, including one that deselects the day.
    /// When [`Self::on_selection_change`] is also set, that callback takes
    /// precedence and this one is not called (a press emits one message).
    pub fn on_select(mut self, on_select: impl Fn(DateParts) -> Message + 'a) -> Self {
        self.on_select = Some(Box::new(on_select));
        self
    }

    /// Sets the callback receiving the next controlled selection snapshot
    /// (`onValueChange`).
    pub fn on_selection_change(
        mut self,
        on_selection_change: impl Fn(CalendarSelection) -> Message + 'a,
    ) -> Self {
        self.on_selection_change = Some(Box::new(on_selection_change));
        self
    }

    /// Sets the callback receiving the new visible month after prev/next
    /// paging or a caption dropdown pick (`onPlaceholderChange`).
    ///
    /// Navigation controls are disabled while this callback is unset,
    /// because the calendar has no internal month state.
    pub fn on_placeholder_change(
        mut self,
        on_placeholder_change: impl Fn(DateParts) -> Message + 'a,
    ) -> Self {
        self.on_placeholder_change = Some(Rc::new(on_placeholder_change));
        self
    }
}

/// Convenience wrapper mirroring the [`select()`](crate::select) helpers of
/// peer components.
///
/// ```rust,no_run
/// use iced::Element;
/// use iced_shadcn_v2::{DateParts, Theme, calendar};
///
/// #[derive(Debug, Clone)]
/// enum Message {
///     Picked(DateParts),
/// }
///
/// fn view(theme: &Theme) -> Element<'_, Message> {
///     calendar(theme).on_select(Message::Picked).into()
/// }
/// ```
pub fn calendar<Message>(theme: &Theme) -> Calendar<'_, Message> {
    Calendar::new(theme)
}

impl<'a, Message> From<Calendar<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(calendar: Calendar<'a, Message>) -> Self {
        render::build_calendar(calendar)
    }
}
