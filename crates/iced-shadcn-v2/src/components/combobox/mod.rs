//! Builder-first combobox composed from [`crate::Popover`] and
//! [`crate::Command`].
//!
//! The component mirrors shadcn-svelte's composition rather than introducing
//! a second dropdown implementation: the trigger is a themed
//! [`crate::Button`], the floating surface is [`crate::Popover`], and search,
//! filtering, groups, keyboard navigation, disabled rows, and scrolling come
//! from [`crate::Command`]. The application owns the query, open state, and
//! selection and feeds those values back into the builder on every view.
//!
//! ```rust,no_run
//! use iced::Element;
//! use iced_shadcn_v2::{Combobox, ComboboxSelection, Theme};
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     SearchChanged(String),
//!     SelectionChanged(ComboboxSelection<&'static str>),
//!     OpenChanged(bool),
//! }
//!
//! fn framework<'a>(
//!     theme: &'a Theme,
//!     query: &'a str,
//!     selection: ComboboxSelection<&'static str>,
//!     open: bool,
//! ) -> Element<'a, Message> {
//!     Combobox::new(theme)
//!         .width(200.0)
//!         .placeholder("Select a framework...")
//!         .search_placeholder("Search framework...")
//!         .item(("sveltekit", "SvelteKit"))
//!         .item(("next", "Next.js"))
//!         .query(query)
//!         .selection(selection)
//!         .open(open)
//!         .on_query_change(Message::SearchChanged)
//!         .on_selection_change(Message::SelectionChanged)
//!         .on_open_change(Message::OpenChanged)
//!         .into()
//! }
//! ```

#![allow(clippy::double_must_use)]

mod icon;
mod render;

#[cfg(test)]
mod tests;

use std::fmt;

use shadcn_common::{
    COMMAND_LIST_MAX_HEIGHT_PX, CommandFilter, SelectMode, default_command_filter,
};

use crate::components::button::{ButtonRadius, ButtonSize, ButtonVariant};
use crate::components::command::{
    CommandEmpty, CommandEntry, CommandGroup, CommandItem, CommandLoading, CommandRadius,
    CommandStyle,
};
use crate::components::popover::{PopoverAlign, PopoverSide, PopoverStyle};
use crate::components::select::{SelectSelection, SelectType};
use crate::iced_compat::widget::button as button_widget;
use crate::iced_compat::{Element, Length, Pixels};
use crate::theme::Theme;

use shadcn_common::AccentColor;

/// Selection value accepted by [`Combobox`].
pub type ComboboxSelection<T> = SelectSelection<T>;

/// Selection mode accepted by [`Combobox::select_type`].
pub type ComboboxType = SelectType;

/// Trigger size alias that makes a composed combobox read naturally.
pub type ComboboxSize = ButtonSize;

/// Trigger radius alias that makes a composed combobox read naturally.
pub type ComboboxRadius = ButtonRadius;

/// Command item alias for callers building combobox groups and entries.
pub type ComboboxItem<T> = CommandItem<T>;

/// Command group alias for callers building combobox sections.
pub type ComboboxGroup<T> = CommandGroup<T>;

/// Command entry alias for callers supplying arbitrary command rows.
pub type ComboboxEntry<T> = CommandEntry<T>;

/// Empty-state alias for the composed command list.
pub type ComboboxEmpty = CommandEmpty;

/// Loading-row alias for the composed command list.
pub type ComboboxLoading = CommandLoading;

/// Builder-first autocomplete control composed from button, popover, and
/// command primitives.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct Combobox<'a, T, Message>
where
    T: Clone + PartialEq,
{
    theme: &'a Theme,
    rows: Vec<CommandEntry<T>>,
    selection: SelectSelection<T>,
    select_type: SelectType,
    placeholder: String,
    search_placeholder: String,
    query: &'a str,
    empty: Option<CommandEmpty>,
    trigger_variant: ButtonVariant,
    trigger_size: ButtonSize,
    trigger_radius: Option<ButtonRadius>,
    trigger_color: Option<AccentColor>,
    trigger_width: Length,
    command_radius: Option<CommandRadius>,
    command_width: Length,
    command_max_height: f32,
    command_should_filter: bool,
    command_filter: CommandFilter,
    command_show_search_icon: bool,
    command_show_border: bool,
    command_show_shadow: bool,
    command_loop_highlight: bool,
    highlighted: Option<usize>,
    popover_width: Option<f32>,
    popover_content_padding: f32,
    popover_side: PopoverSide,
    popover_align: PopoverAlign,
    popover_side_offset: f32,
    popover_align_offset: f32,
    popover_animated: bool,
    popover_close_on_click_outside: bool,
    popover_close_on_escape: bool,
    disabled: bool,
    invalid: bool,
    deselectable: bool,
    open: Option<bool>,
    default_open: bool,
    on_open_change: Option<Box<dyn Fn(bool) -> Message + 'a>>,
    on_query_change: Option<Box<dyn Fn(String) -> Message + 'a>>,
    on_select: Option<Box<dyn Fn(T) -> Message + 'a>>,
    on_selection_change: Option<Box<dyn Fn(SelectSelection<T>) -> Message + 'a>>,
    on_highlight_change: Option<Box<dyn Fn(usize) -> Message + 'a>>,
    trigger_style_override: Option<
        Box<dyn Fn(button_widget::Style, button_widget::Status) -> button_widget::Style + 'a>,
    >,
    command_style_override: Option<Box<dyn Fn(CommandStyle) -> CommandStyle + 'a>>,
    popover_style_override: Option<Box<dyn Fn(PopoverStyle) -> PopoverStyle + 'a>>,
}

impl<T, Message> fmt::Debug for Combobox<'_, T, Message>
where
    T: Clone + PartialEq + fmt::Debug,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Combobox")
            .field("theme", &self.theme)
            .field("rows", &self.rows.len())
            .field("selection", &self.selection)
            .field("select_type", &self.select_type)
            .field("placeholder", &self.placeholder)
            .field("search_placeholder", &self.search_placeholder)
            .field("query", &self.query)
            .field("empty", &self.empty)
            .field("trigger_variant", &self.trigger_variant)
            .field("trigger_size", &self.trigger_size)
            .field("trigger_radius", &self.trigger_radius)
            .field("trigger_color", &self.trigger_color)
            .field("trigger_width", &self.trigger_width)
            .field("command_radius", &self.command_radius)
            .field("command_width", &self.command_width)
            .field("command_max_height", &self.command_max_height)
            .field("command_should_filter", &self.command_should_filter)
            .field("command_show_search_icon", &self.command_show_search_icon)
            .field("command_show_border", &self.command_show_border)
            .field("command_show_shadow", &self.command_show_shadow)
            .field("command_loop_highlight", &self.command_loop_highlight)
            .field("highlighted", &self.highlighted)
            .field("popover_width", &self.popover_width)
            .field("popover_content_padding", &self.popover_content_padding)
            .field("popover_side", &self.popover_side)
            .field("popover_align", &self.popover_align)
            .field("popover_side_offset", &self.popover_side_offset)
            .field("popover_align_offset", &self.popover_align_offset)
            .field("popover_animated", &self.popover_animated)
            .field(
                "popover_close_on_click_outside",
                &self.popover_close_on_click_outside,
            )
            .field("popover_close_on_escape", &self.popover_close_on_escape)
            .field("disabled", &self.disabled)
            .field("invalid", &self.invalid)
            .field("deselectable", &self.deselectable)
            .field("open", &self.open)
            .field("default_open", &self.default_open)
            .field("on_open_change", &self.on_open_change.is_some())
            .field("on_query_change", &self.on_query_change.is_some())
            .field("on_select", &self.on_select.is_some())
            .field("on_selection_change", &self.on_selection_change.is_some())
            .field("on_highlight_change", &self.on_highlight_change.is_some())
            .field(
                "trigger_style_override",
                &self.trigger_style_override.is_some(),
            )
            .field(
                "command_style_override",
                &self.command_style_override.is_some(),
            )
            .field(
                "popover_style_override",
                &self.popover_style_override.is_some(),
            )
            .finish()
    }
}

impl<'a, T, Message> Combobox<'a, T, Message>
where
    T: Clone + PartialEq,
{
    /// Creates an empty combobox with a compact outline trigger.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            rows: Vec::new(),
            selection: SelectSelection::default(),
            select_type: SelectMode::Single,
            placeholder: "Select an option...".to_owned(),
            search_placeholder: "Search...".to_owned(),
            query: "",
            empty: Some(CommandEmpty::new("No results found.")),
            trigger_variant: ButtonVariant::Outline,
            trigger_size: ButtonSize::Sm,
            trigger_radius: None,
            trigger_color: None,
            trigger_width: Length::Shrink,
            command_radius: None,
            command_width: Length::Fill,
            command_max_height: COMMAND_LIST_MAX_HEIGHT_PX,
            command_should_filter: true,
            command_filter: default_command_filter,
            command_show_search_icon: true,
            command_show_border: false,
            command_show_shadow: false,
            command_loop_highlight: true,
            highlighted: None,
            popover_width: None,
            popover_content_padding: 0.0,
            popover_side: PopoverSide::default(),
            popover_align: PopoverAlign::default(),
            popover_side_offset: 4.0,
            popover_align_offset: 0.0,
            popover_animated: true,
            popover_close_on_click_outside: true,
            popover_close_on_escape: true,
            disabled: false,
            invalid: false,
            deselectable: false,
            open: None,
            default_open: false,
            on_open_change: None,
            on_query_change: None,
            on_select: None,
            on_selection_change: None,
            on_highlight_change: None,
            trigger_style_override: None,
            command_style_override: None,
            popover_style_override: None,
        }
    }

    /// Appends one selectable command item.
    #[must_use]
    pub fn item(mut self, item: impl Into<CommandItem<T>>) -> Self {
        self.rows.push(CommandEntry::Item(item.into()));
        self
    }

    /// Appends every item from an iterator.
    #[must_use]
    pub fn items<I, O>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = O>,
        O: Into<CommandItem<T>>,
    {
        self.rows.extend(
            items
                .into_iter()
                .map(|item| CommandEntry::Item(item.into())),
        );
        self
    }

    /// Appends a labelled command group.
    #[must_use]
    pub fn group(mut self, group: CommandGroup<T>) -> Self {
        self.rows.push(CommandEntry::Group(group));
        self
    }

    /// Appends a separator row.
    #[must_use]
    pub fn separator(mut self) -> Self {
        self.rows
            .push(CommandEntry::Separator { force_mount: false });
        self
    }

    /// Appends a force-mounted separator row.
    #[must_use]
    pub fn separator_force_mount(mut self) -> Self {
        self.rows
            .push(CommandEntry::Separator { force_mount: true });
        self
    }

    /// Appends a loading row.
    #[must_use]
    pub fn loading(mut self, loading: impl Into<CommandLoading>) -> Self {
        self.rows.push(CommandEntry::Loading(loading.into()));
        self
    }

    /// Appends an arbitrary command entry.
    #[must_use]
    pub fn entry(mut self, entry: CommandEntry<T>) -> Self {
        self.rows.push(entry);
        self
    }

    /// Sets the controlled selection snapshot.
    #[must_use]
    pub fn selection(mut self, selection: SelectSelection<T>) -> Self {
        self.select_type = selection.selection_type();
        self.selection = selection;
        self
    }

    /// Sets the selected value in single-selection mode.
    #[must_use]
    pub fn selected(mut self, selected: T) -> Self {
        self.select_type = SelectMode::Single;
        self.selection = SelectSelection::Single(Some(selected));
        self
    }

    /// Sets or clears the selected value in single-selection mode.
    #[must_use]
    pub fn selected_maybe(mut self, selected: Option<T>) -> Self {
        self.select_type = SelectMode::Single;
        self.selection = SelectSelection::Single(selected);
        self
    }

    /// Sets the selected values in multiple-selection mode.
    #[must_use]
    pub fn values(mut self, values: impl IntoIterator<Item = T>) -> Self {
        self.select_type = SelectMode::Multiple;
        self.selection = SelectSelection::multiple(values);
        self
    }

    /// Sets the selection mode (`single` or `multiple`).
    #[must_use]
    pub fn select_type(mut self, select_type: SelectType) -> Self {
        self.select_type = select_type;
        self.selection = match select_type {
            SelectMode::Single => SelectSelection::Single(self.selection.as_single().cloned()),
            SelectMode::Multiple => {
                let values = match &self.selection {
                    SelectSelection::Multiple(values) => values.clone(),
                    SelectSelection::Single(Some(value)) => vec![value.clone()],
                    SelectSelection::Single(None) => Vec::new(),
                };
                SelectSelection::Multiple(values)
            }
            _ => self.selection,
        };
        self
    }

    /// Alias for [`Self::select_type`] using the source component's naming.
    #[must_use]
    pub fn type_(self, select_type: SelectType) -> Self {
        self.select_type(select_type)
    }

    /// Sets the text shown when no value is selected.
    #[must_use]
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Sets the search input placeholder.
    #[must_use]
    pub fn search_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.search_placeholder = placeholder.into();
        self
    }

    /// Sets the controlled search query.
    #[must_use]
    pub fn query(mut self, query: &'a str) -> Self {
        self.query = query;
        self
    }

    /// Sets the empty-state copy.
    #[must_use]
    pub fn empty(mut self, empty: impl Into<CommandEmpty>) -> Self {
        self.empty = Some(empty.into());
        self
    }

    /// Sets or clears the empty-state row.
    #[must_use]
    pub fn empty_maybe(mut self, empty: Option<CommandEmpty>) -> Self {
        self.empty = empty;
        self
    }

    /// Sets the trigger width. A fixed width also sizes the popover unless
    /// [`Self::content_width`] is supplied explicitly.
    #[must_use]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.trigger_width = width.into();
        self
    }

    /// Sets the command content width independently from the trigger.
    #[must_use]
    pub fn content_width(mut self, width: f32) -> Self {
        self.popover_width = Some(sanitize_dimension(width));
        self
    }

    /// Alias for [`Self::content_width`].
    #[must_use]
    pub fn popover_width(self, width: f32) -> Self {
        self.content_width(width)
    }

    /// Sets the internal command width.
    #[must_use]
    pub fn command_width(mut self, width: impl Into<Length>) -> Self {
        self.command_width = width.into();
        self
    }

    /// Sets the command list maximum height.
    #[must_use]
    pub fn max_height(mut self, max_height: impl Into<Pixels>) -> Self {
        self.command_max_height = max_height.into().0.max(1.0);
        self
    }

    /// Enables or disables command-side filtering.
    #[must_use]
    pub fn should_filter(mut self, should_filter: bool) -> Self {
        self.command_should_filter = should_filter;
        self
    }

    /// Replaces the default fuzzy command filter.
    #[must_use]
    pub fn filter(mut self, filter: CommandFilter) -> Self {
        self.command_filter = filter;
        self
    }

    /// Shows or hides the leading search icon.
    #[must_use]
    pub fn show_search_icon(mut self, show: bool) -> Self {
        self.command_show_search_icon = show;
        self
    }

    /// Shows or hides the internal command border.
    #[must_use]
    pub fn show_command_border(mut self, show: bool) -> Self {
        self.command_show_border = show;
        self
    }

    /// Shows or hides the internal command shadow.
    #[must_use]
    pub fn show_command_shadow(mut self, show: bool) -> Self {
        self.command_show_shadow = show;
        self
    }

    /// Enables wrapping when keyboard navigation passes the list edge.
    #[must_use]
    pub fn loop_highlight(mut self, looping: bool) -> Self {
        self.command_loop_highlight = looping;
        self
    }

    /// Controls the highlighted command item.
    #[must_use]
    pub fn highlighted(mut self, index: usize) -> Self {
        self.highlighted = Some(index);
        self
    }

    /// Controls the highlighted command item when `Some`.
    #[must_use]
    pub fn highlighted_maybe(mut self, index: Option<usize>) -> Self {
        self.highlighted = index;
        self
    }

    /// Sets the trigger button variant.
    #[must_use]
    pub fn trigger_variant(mut self, variant: ButtonVariant) -> Self {
        self.trigger_variant = variant;
        self
    }

    /// Sets the trigger button size.
    #[must_use]
    pub fn trigger_size(mut self, size: ButtonSize) -> Self {
        self.trigger_size = size;
        self
    }

    /// Sets the trigger button radius.
    #[must_use]
    pub fn trigger_radius(mut self, radius: ButtonRadius) -> Self {
        self.trigger_radius = Some(radius);
        self
    }

    /// Alias for [`Self::trigger_radius`].
    #[must_use]
    pub fn radius(self, radius: ButtonRadius) -> Self {
        self.trigger_radius(radius)
    }

    /// Applies a trigger accent color.
    #[must_use]
    pub fn trigger_color(mut self, color: AccentColor) -> Self {
        self.trigger_color = Some(color);
        self
    }

    /// Alias for [`Self::trigger_color`].
    #[must_use]
    pub fn color(self, color: AccentColor) -> Self {
        self.trigger_color(color)
    }

    /// Sets the command surface radius.
    #[must_use]
    pub fn command_radius(mut self, radius: CommandRadius) -> Self {
        self.command_radius = Some(radius);
        self
    }

    /// Sets the popover side.
    #[must_use]
    pub fn popover_side(mut self, side: PopoverSide) -> Self {
        self.popover_side = side;
        self
    }

    /// Sets popover alignment along the trigger edge.
    #[must_use]
    pub fn popover_align(mut self, align: PopoverAlign) -> Self {
        self.popover_align = align;
        self
    }

    /// Sets the popover-to-trigger gap.
    #[must_use]
    pub fn popover_side_offset(mut self, offset: f32) -> Self {
        self.popover_side_offset = finite_or(offset, 4.0);
        self
    }

    /// Sets the cross-axis popover offset.
    #[must_use]
    pub fn popover_align_offset(mut self, offset: f32) -> Self {
        self.popover_align_offset = finite_or(offset, 0.0);
        self
    }

    /// Sets the popover content padding.
    #[must_use]
    pub fn popover_content_padding(mut self, padding: f32) -> Self {
        self.popover_content_padding = sanitize_dimension(padding);
        self
    }

    /// Enables or disables the popover transition.
    #[must_use]
    pub fn animated(mut self, animated: bool) -> Self {
        self.popover_animated = animated;
        self
    }

    /// Keeps the popover open on outside clicks when `false`.
    #[must_use]
    pub fn close_on_click_outside(mut self, close: bool) -> Self {
        self.popover_close_on_click_outside = close;
        self
    }

    /// Keeps the popover open on Escape when `false`.
    #[must_use]
    pub fn close_on_escape(mut self, close: bool) -> Self {
        self.popover_close_on_escape = close;
        self
    }

    /// Disables the trigger and prevents the popover from opening.
    #[must_use]
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Marks the trigger invalid using the theme destructive border color.
    #[must_use]
    pub fn invalid(mut self, invalid: bool) -> Self {
        self.invalid = invalid;
        self
    }

    /// Controls whether selecting an already-selected single item clears it
    /// when using [`Self::on_selection_change`].
    #[must_use]
    pub fn deselectable(mut self, deselectable: bool) -> Self {
        self.deselectable = deselectable;
        self
    }

    /// Controls the open state.
    #[must_use]
    pub fn open(mut self, open: bool) -> Self {
        self.open = Some(open);
        self
    }

    /// Controls the open state when `Some`.
    #[must_use]
    pub fn open_maybe(mut self, open: Option<bool>) -> Self {
        self.open = open;
        self
    }

    /// Opens the popover on first mount when it is uncontrolled.
    #[must_use]
    pub fn default_open(mut self, default_open: bool) -> Self {
        self.default_open = default_open;
        self
    }

    /// Receives trigger, outside-click, and Escape open-state requests.
    #[must_use]
    pub fn on_open_change(mut self, on_open_change: impl Fn(bool) -> Message + 'a) -> Self {
        self.on_open_change = Some(Box::new(on_open_change));
        self
    }

    /// Receives controlled query edits.
    #[must_use]
    pub fn on_query_change(mut self, on_query_change: impl Fn(String) -> Message + 'a) -> Self {
        self.on_query_change = Some(Box::new(on_query_change));
        self
    }

    /// Receives the picked item value.
    ///
    /// When both this and [`Self::on_selection_change`] are configured, the
    /// selection-snapshot callback is used so one command activation emits a
    /// single deterministic message.
    #[must_use]
    pub fn on_select(mut self, on_select: impl Fn(T) -> Message + 'a) -> Self {
        self.on_select = Some(Box::new(on_select));
        self
    }

    /// Sets or clears the picked-item callback.
    #[must_use]
    pub fn on_select_maybe(mut self, on_select: Option<impl Fn(T) -> Message + 'a>) -> Self {
        self.on_select = on_select.map(|callback| Box::new(callback) as _);
        self
    }

    /// Receives the next controlled selection snapshot.
    #[must_use]
    pub fn on_selection_change(
        mut self,
        on_selection_change: impl Fn(SelectSelection<T>) -> Message + 'a,
    ) -> Self {
        self.on_selection_change = Some(Box::new(on_selection_change));
        self
    }

    /// Receives the highlighted command index.
    #[must_use]
    pub fn on_highlight_change(
        mut self,
        on_highlight_change: impl Fn(usize) -> Message + 'a,
    ) -> Self {
        self.on_highlight_change = Some(Box::new(on_highlight_change));
        self
    }

    /// Patches the composed trigger's resolved iced button style.
    #[must_use]
    pub fn trigger_style_override(
        mut self,
        style_override: impl Fn(button_widget::Style, button_widget::Status) -> button_widget::Style
        + 'a,
    ) -> Self {
        self.trigger_style_override = Some(Box::new(style_override));
        self
    }

    /// Alias for [`Self::trigger_style_override`].
    #[must_use]
    pub fn style_override(
        self,
        style_override: impl Fn(button_widget::Style, button_widget::Status) -> button_widget::Style
        + 'a,
    ) -> Self {
        self.trigger_style_override(style_override)
    }

    /// Patches the composed [`crate::Command`] style.
    #[must_use]
    pub fn command_style_override(
        mut self,
        style_override: impl Fn(CommandStyle) -> CommandStyle + 'a,
    ) -> Self {
        self.command_style_override = Some(Box::new(style_override));
        self
    }

    /// Patches the composed [`crate::Popover`] style.
    #[must_use]
    pub fn popover_style_override(
        mut self,
        style_override: impl Fn(PopoverStyle) -> PopoverStyle + 'a,
    ) -> Self {
        self.popover_style_override = Some(Box::new(style_override));
        self
    }
}

/// Convenience constructor that sets the trigger placeholder.
pub fn combobox<'a, T, Message>(
    placeholder: impl Into<String>,
    theme: &'a Theme,
) -> Combobox<'a, T, Message>
where
    T: Clone + PartialEq,
{
    Combobox::new(theme).placeholder(placeholder)
}

impl<'a, T, Message> From<Combobox<'a, T, Message>> for Element<'a, Message>
where
    T: Clone + PartialEq + 'a,
    Message: Clone + 'a,
{
    fn from(combobox: Combobox<'a, T, Message>) -> Self {
        render::build(combobox)
    }
}

fn sanitize_dimension(value: f32) -> f32 {
    if value.is_finite() {
        value.max(0.0)
    } else {
        0.0
    }
}

fn finite_or(value: f32, fallback: f32) -> f32 {
    if value.is_finite() { value } else { fallback }
}
