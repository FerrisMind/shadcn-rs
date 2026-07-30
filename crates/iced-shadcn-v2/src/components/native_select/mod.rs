//! Native select component ported from shadcn-svelte to iced-shadcn-v2.
//!
//! [`NativeSelect`] keeps the native-select contract—one controlled selected
//! value, a placeholder option, disabled options, optgroups, invalid state,
//! and compact/default sizes—while using iced's native widget and overlay
//! primitives underneath. The selection callback receives the typed value,
//! so applications do not need to parse strings at the boundary.
//!
//! ```rust,no_run
//! use iced::Element;
//! use iced_shadcn_v2::{NativeSelect, NativeSelectOption, Theme};
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     CountryChanged(String),
//! }
//!
//! fn country<'a>(theme: &'a Theme, selected: Option<String>) -> Element<'a, Message> {
//!     NativeSelect::with_options(
//!         theme,
//!         [
//!             NativeSelectOption::new("us".to_owned(), "United States"),
//!             NativeSelectOption::new("ca".to_owned(), "Canada"),
//!         ],
//!         selected,
//!     )
//!     .placeholder("Select a country")
//!     .on_change(Message::CountryChanged)
//!     .into()
//! }
//! ```

mod render;
mod style;
mod types;

#[cfg(test)]
mod tests;

pub use types::{
    NativeSelectItem, NativeSelectOptGroup, NativeSelectOption, NativeSelectRadius,
    NativeSelectSize,
};

use std::fmt;

use crate::iced_compat::widget::{self, pick_list};
use crate::iced_compat::{Element, Length, Pixels};

use crate::theme::Theme;

/// Builder-first native select styled from the active shadcn style pack.
///
/// The application owns `selected` and updates it in the callback, mirroring
/// Svelte's controlled `bind:value` flow. `NativeSelect::new` is the concise
/// path for values whose [`ToString`] output is also their label; use
/// [`Self::with_options`] or [`Self::with_items`] when labels, disabled
/// options, or groups need to be explicit.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct NativeSelect<'a, T, Message> {
    theme: &'a Theme,
    items: Vec<NativeSelectItem<T>>,
    selected: Option<T>,
    placeholder: Option<String>,
    size: NativeSelectSize,
    radius: Option<NativeSelectRadius>,
    width: Length,
    menu_height: Length,
    text_size: Option<Pixels>,
    id: Option<widget::Id>,
    disabled: bool,
    invalid: bool,
    on_select: Option<Box<dyn Fn(T) -> Message + 'a>>,
    on_open: Option<Message>,
    on_close: Option<Message>,
    style_override:
        Option<Box<dyn Fn(pick_list::Style, pick_list::Status) -> pick_list::Style + 'a>>,
}

impl<T, Message> fmt::Debug for NativeSelect<'_, T, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("NativeSelect")
            .field("theme", &self.theme)
            .field("items", &self.items.len())
            .field("selected", &self.selected.is_some())
            .field("placeholder", &self.placeholder)
            .field("size", &self.size)
            .field("radius", &self.radius)
            .field("width", &self.width)
            .field("menu_height", &self.menu_height)
            .field("text_size", &self.text_size)
            .field("id", &self.id)
            .field("disabled", &self.disabled)
            .field("invalid", &self.invalid)
            .field("on_select", &self.on_select.is_some())
            .field("on_open", &self.on_open.is_some())
            .field("on_close", &self.on_close.is_some())
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

impl<'a, T, Message> NativeSelect<'a, T, Message> {
    /// Creates an empty native select.
    ///
    /// Add options with [`Self::push`] or use [`Self::with_options`] for a
    /// one-shot constructor.
    pub fn empty(theme: &'a Theme) -> Self {
        Self {
            theme,
            items: Vec::new(),
            selected: None,
            placeholder: None,
            size: NativeSelectSize::Default,
            radius: None,
            width: Length::Shrink,
            menu_height: Length::Shrink,
            text_size: None,
            id: None,
            disabled: false,
            invalid: false,
            on_select: None,
            on_open: None,
            on_close: None,
            style_override: None,
        }
    }

    /// Creates a native select from plain values.
    ///
    /// Each value's [`ToString`] output is used as its visible label.
    pub fn new<I>(theme: &'a Theme, options: I, selected: Option<T>) -> Self
    where
        I: IntoIterator<Item = T>,
        T: ToString,
    {
        Self::with_options(
            theme,
            options.into_iter().map(|value| {
                let label = value.to_string();
                NativeSelectOption::new(value, label)
            }),
            selected,
        )
    }

    /// Creates a select from explicitly labelled options.
    pub fn with_options<I>(theme: &'a Theme, options: I, selected: Option<T>) -> Self
    where
        I: IntoIterator<Item = NativeSelectOption<T>>,
    {
        Self::empty(theme)
            .extend(options.into_iter().map(NativeSelectItem::Option))
            .selected(selected)
    }

    /// Creates a select from options and optgroups in layout order.
    pub fn with_items<I>(theme: &'a Theme, items: I, selected: Option<T>) -> Self
    where
        I: IntoIterator<Item = NativeSelectItem<T>>,
    {
        Self::empty(theme).extend(items).selected(selected)
    }

    /// Appends one option.
    pub fn push(mut self, option: NativeSelectOption<T>) -> Self {
        self.items.push(NativeSelectItem::Option(option));
        self
    }

    /// Appends one optgroup.
    pub fn push_group(mut self, group: NativeSelectOptGroup<T>) -> Self {
        self.items.push(NativeSelectItem::OptGroup(group));
        self
    }

    /// Appends all items from an iterator.
    pub fn extend<I>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = NativeSelectItem<T>>,
    {
        self.items.extend(items);
        self
    }

    /// Returns the number of top-level items.
    #[must_use]
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Returns whether there are no top-level items.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Sets the controlled selected value.
    pub fn selected(mut self, selected: Option<T>) -> Self {
        self.selected = selected;
        self
    }

    /// Clears the controlled selected value.
    pub fn clear_selected(mut self) -> Self {
        self.selected = None;
        self
    }

    /// Sets the placeholder displayed while no value is selected.
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    /// Sets the compact or default control size.
    pub fn size(mut self, size: NativeSelectSize) -> Self {
        self.size = size;
        self
    }

    /// Sets the control corner radius.
    pub fn radius(mut self, radius: NativeSelectRadius) -> Self {
        self.radius = Some(radius);
        self
    }

    /// Sets the width of the control (`Length::Shrink` by default, matching
    /// the web root's `w-fit`).
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Makes the control fill its parent width.
    pub fn full_width(mut self) -> Self {
        self.width = Length::Fill;
        self
    }

    /// Sets the maximum height of the dropdown menu.
    pub fn menu_height(mut self, height: impl Into<Length>) -> Self {
        self.menu_height = height.into();
        self
    }

    /// Sets the label and menu text size in pixels.
    pub fn text_size(mut self, text_size: impl Into<Pixels>) -> Self {
        self.text_size = Some(text_size.into());
        self
    }

    /// Sets the widget id used by iced's focus operations.
    ///
    /// An id makes the control reachable through `iced::widget::operation`
    /// helpers and lets a surrounding form or label focus it programmatically.
    pub fn id(mut self, id: impl Into<widget::Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Disables the whole select and dims it to 50% opacity.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Marks the select invalid, making the border destructive.
    pub fn invalid(mut self, invalid: bool) -> Self {
        self.invalid = invalid;
        self
    }

    /// Sets the typed selection callback.
    pub fn on_select<F>(mut self, on_select: F) -> Self
    where
        F: Fn(T) -> Message + 'a,
    {
        self.on_select = Some(Box::new(on_select));
        self
    }

    /// Alias for [`Self::on_select`] matching the web component's change
    /// event naming.
    pub fn on_change<F>(self, on_change: F) -> Self
    where
        F: Fn(T) -> Message + 'a,
    {
        self.on_select(on_change)
    }

    /// Sets or clears the typed selection callback.
    pub fn on_select_maybe<F>(mut self, on_select: Option<F>) -> Self
    where
        F: Fn(T) -> Message + 'a,
    {
        self.on_select = on_select.map(|callback| Box::new(callback) as _);
        self
    }

    /// Emits a message when the dropdown opens.
    pub fn on_open(mut self, message: Message) -> Self {
        self.on_open = Some(message);
        self
    }

    /// Emits a message when the dropdown closes after an outside click.
    pub fn on_close(mut self, message: Message) -> Self {
        self.on_close = Some(message);
        self
    }

    /// Applies a narrow iced `pick_list::Style` override after token
    /// resolution.
    pub fn style_override(
        mut self,
        style_override: impl Fn(pick_list::Style, pick_list::Status) -> pick_list::Style + 'a,
    ) -> Self {
        self.style_override = Some(Box::new(style_override));
        self
    }

    /// Builds the iced element.
    pub fn into_element(self) -> Element<'a, Message>
    where
        T: Clone + PartialEq + ToString + 'a,
        Message: Clone + 'a,
    {
        render::build(self)
    }
}

impl<'a, T, Message> From<NativeSelect<'a, T, Message>> for Element<'a, Message>
where
    T: Clone + PartialEq + ToString + 'a,
    Message: Clone + 'a,
{
    fn from(select: NativeSelect<'a, T, Message>) -> Self {
        select.into_element()
    }
}
