//! Custom select component ported from shadcn-svelte to iced-shadcn-v2.
//!
//! Port of the shadcn-svelte select (`Select.Root` / `Trigger` / `Content` /
//! `Item` / `Group` / `Label` / `Separator`) as a single iced builder. The
//! trigger paints `.cn-select-trigger`; the dropdown is a design-system
//! popover surface (not the OS menu used by [`crate::NativeSelect`]) with
//! checkable items, group labels, and separators. Interactions match
//! bits-ui: click to open, click-away / Esc to close, arrow keys + Enter
//! while open; single mode closes on pick, multiple mode toggles and stays
//! open.
//!
//! ```rust,no_run
//! use iced::Element;
//! use iced_shadcn_v2::{Select, Theme};
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     FruitPicked(&'static str),
//! }
//!
//! fn fruit<'a>(theme: &'a Theme, value: Option<&'static str>) -> Element<'a, Message> {
//!     Select::new(theme)
//!         .placeholder("Select a fruit")
//!         .item(("apple", "Apple"))
//!         .item(("banana", "Banana"))
//!         .selected_maybe(value)
//!         .on_select(Message::FruitPicked)
//!         .into()
//! }
//! ```

mod render;
mod style;
mod types;

#[cfg(test)]
mod tests;

pub use style::{SelectContentStyle, SelectStatus, SelectTriggerStyle};
pub use types::{SelectGroup, SelectItem, SelectRadius, SelectSelection, SelectSize, SelectType};

use std::fmt;

use shadcn_common::{SELECT_CONTENT_MAX_HEIGHT_PX, SelectMode};

use crate::iced_compat::{Element, Length, Pixels};
use crate::theme::Theme;

use render::SelectWidget;
use types::Row;

/// Builder-first custom select styled directly with iced types.
///
/// Theme tokens come from `shadcn-common` via [`Theme`]. Pass `&theme` into
/// every select — style packs live on the app's [`Theme`], not on this
/// builder. The application owns the selection and feeds it back through
/// [`Self::selected`] / [`Self::selection`] on every change, mirroring
/// `bind:value`.
///
/// ```rust,no_run
/// use iced::Element;
/// use iced_shadcn_v2::{Select, SelectGroup, SelectType, Theme};
///
/// #[derive(Debug, Clone)]
/// enum Message {
///     Changed(iced_shadcn_v2::SelectSelection<&'static str>),
/// }
///
/// fn foods<'a>(
///     theme: &'a Theme,
///     selection: iced_shadcn_v2::SelectSelection<&'static str>,
/// ) -> Element<'a, Message> {
///     Select::new(theme)
///         .select_type(SelectType::Multiple)
///         .placeholder("Select foods")
///         .group(
///             SelectGroup::new("Fruits")
///                 .item(("apple", "Apple"))
///                 .item(("banana", "Banana")),
///         )
///         .separator()
///         .group(
///             SelectGroup::new("Vegetables")
///                 .item(("carrot", "Carrot"))
///                 .item(("broccoli", "Broccoli")),
///         )
///         .selection(selection)
///         .on_selection_change(Message::Changed)
///         .into()
/// }
/// ```
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct Select<'a, T, Message>
where
    T: Clone + PartialEq,
{
    theme: &'a Theme,
    rows: Vec<Row<T>>,
    selection: SelectSelection<T>,
    select_type: SelectType,
    placeholder: Option<String>,
    size: SelectSize,
    radius: Option<SelectRadius>,
    width: Length,
    /// Dropdown content max height (`max-h-*` on `Select.Content`).
    max_height: f32,
    text_size: Option<f32>,
    disabled: bool,
    invalid: bool,
    deselectable: bool,
    on_select: Option<Box<dyn Fn(T) -> Message + 'a>>,
    on_selection_change: Option<Box<dyn Fn(SelectSelection<T>) -> Message + 'a>>,
    on_open: Option<Message>,
    on_close: Option<Message>,
    style_override:
        Option<Box<dyn Fn(SelectTriggerStyle, SelectStatus) -> SelectTriggerStyle + 'a>>,
}

impl<T, Message> fmt::Debug for Select<'_, T, Message>
where
    T: Clone + PartialEq + fmt::Debug,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Select")
            .field("theme", &self.theme)
            .field("rows", &self.rows.len())
            .field("selection", &self.selection)
            .field("select_type", &self.select_type)
            .field("placeholder", &self.placeholder)
            .field("size", &self.size)
            .field("radius", &self.radius)
            .field("width", &self.width)
            .field("max_height", &self.max_height)
            .field("text_size", &self.text_size)
            .field("disabled", &self.disabled)
            .field("invalid", &self.invalid)
            .field("deselectable", &self.deselectable)
            .field("on_select", &self.on_select.is_some())
            .field("on_selection_change", &self.on_selection_change.is_some())
            .field("on_open", &self.on_open.is_some())
            .field("on_close", &self.on_close.is_some())
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

impl<'a, T, Message> Select<'a, T, Message>
where
    T: Clone + PartialEq,
{
    /// Creates an empty select.
    ///
    /// `theme` is required because styling is derived from `shadcn-common`
    /// theme tokens instead of `iced::Theme`.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            rows: Vec::new(),
            selection: SelectSelection::default(),
            select_type: SelectMode::Single,
            placeholder: None,
            size: SelectSize::Default,
            radius: None,
            width: Length::Shrink,
            max_height: SELECT_CONTENT_MAX_HEIGHT_PX,
            text_size: None,
            disabled: false,
            invalid: false,
            deselectable: true,
            on_select: None,
            on_selection_change: None,
            on_open: None,
            on_close: None,
            style_override: None,
        }
    }

    /// Appends one selectable item (`Select.Item`).
    pub fn item(mut self, item: impl Into<SelectItem<T>>) -> Self {
        let item = item.into();
        self.rows.push(Row::Option {
            value: item.value,
            label: item.label,
            disabled: item.disabled,
        });
        self
    }

    /// Appends every item of the iterator.
    pub fn items<I, O>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = O>,
        O: Into<SelectItem<T>>,
    {
        for item in items {
            self = self.item(item);
        }
        self
    }

    /// Appends a labelled group of items (`Select.Group` + optional label).
    pub fn group(mut self, group: SelectGroup<T>) -> Self {
        if let Some(label) = group.label {
            self.rows.push(Row::Label { text: label });
        }

        for item in group.items {
            self.rows.push(Row::Option {
                value: item.value,
                label: item.label,
                disabled: item.disabled,
            });
        }

        self
    }

    /// Appends a non-interactive group / section label (`Select.Label`).
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.rows.push(Row::Label { text: label.into() });
        self
    }

    /// Appends a hairline separator (`Select.Separator`).
    pub fn separator(mut self) -> Self {
        self.rows.push(Row::Separator);
        self
    }

    /// Sets the selection mode (`type="single" | "multiple"`).
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

    /// Alias for [`Self::select_type`] using the bits-ui `type` naming.
    pub fn type_(self, select_type: SelectType) -> Self {
        self.select_type(select_type)
    }

    /// Sets the controlled selection snapshot.
    pub fn selection(mut self, selection: SelectSelection<T>) -> Self {
        self.select_type = selection.selection_type();
        self.selection = selection;
        self
    }

    /// Sets the currently selected value in single mode.
    pub fn selected(mut self, selected: T) -> Self {
        self.select_type = SelectMode::Single;
        self.selection = SelectSelection::Single(Some(selected));
        self
    }

    /// Sets or clears the currently selected value in single mode.
    pub fn selected_maybe(mut self, selected: Option<T>) -> Self {
        self.select_type = SelectMode::Single;
        self.selection = SelectSelection::Single(selected);
        self
    }

    /// Sets the selected values in multiple mode.
    pub fn values(mut self, values: impl IntoIterator<Item = T>) -> Self {
        self.select_type = SelectMode::Multiple;
        self.selection = SelectSelection::multiple(values);
        self
    }

    /// Sets the text shown while nothing is selected.
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    /// Sets the preset trigger size (`size="sm" | "default"`).
    pub fn size(mut self, size: SelectSize) -> Self {
        self.size = size;
        self
    }

    /// Sets the trigger corner radius.
    pub fn radius(mut self, radius: SelectRadius) -> Self {
        self.radius = Some(radius);
        self
    }

    /// Sets a custom trigger width.
    ///
    /// Defaults to `Length::Shrink` (`w-fit`): the trigger sizes to its
    /// widest option so it never resizes on selection. The open content is
    /// at least as wide as the trigger (`min-w-[anchor]`).
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the dropdown content max height (`max-h-*` on `Select.Content`).
    ///
    /// Defaults to [`SELECT_CONTENT_MAX_HEIGHT_PX`] (`max-h-96` → 384). The
    /// scrollable docs demo uses `max-h-[300px]`.
    pub fn max_height(mut self, max_height: impl Into<Pixels>) -> Self {
        self.max_height = max_height.into().0.max(0.0);
        self
    }

    /// Sets the value text size. The pack's `.cn-select-trigger` size is used
    /// by default.
    pub fn text_size(mut self, text_size: impl Into<Pixels>) -> Self {
        self.text_size = Some(text_size.into().0);
        self
    }

    /// Disables the select (`disabled` attribute: no dropdown, 50% opacity).
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Marks the value as invalid (`aria-invalid`): the border turns
    /// `destructive` and outranks the focus treatment.
    pub fn invalid(mut self, invalid: bool) -> Self {
        self.invalid = invalid;
        self
    }

    /// Whether picking the already-selected single value clears it.
    ///
    /// Defaults to `true`, matching bits-ui's deselectable single select.
    pub fn deselectable(mut self, deselectable: bool) -> Self {
        self.deselectable = deselectable;
        self
    }

    /// Sets the callback receiving the picked item value.
    ///
    /// In single mode this fires when an item is chosen. In multiple mode it
    /// fires for every toggle. Prefer [`Self::on_selection_change`] when the
    /// app needs the full next selection snapshot.
    pub fn on_select(mut self, on_select: impl Fn(T) -> Message + 'a) -> Self {
        self.on_select = Some(Box::new(on_select));
        self
    }

    /// Sets or clears the per-item pick callback.
    pub fn on_select_maybe(mut self, on_select: Option<impl Fn(T) -> Message + 'a>) -> Self {
        self.on_select = on_select.map(|on_select| Box::new(on_select) as _);
        self
    }

    /// Sets the callback receiving the next controlled selection snapshot.
    pub fn on_selection_change(
        mut self,
        on_selection_change: impl Fn(SelectSelection<T>) -> Message + 'a,
    ) -> Self {
        self.on_selection_change = Some(Box::new(on_selection_change));
        self
    }

    /// Sets the message emitted when the dropdown opens.
    pub fn on_open(mut self, message: Message) -> Self {
        self.on_open = Some(message);
        self
    }

    /// Sets the message emitted when the dropdown closes without a pick
    /// (outside click / Esc). Single-mode picks close without this message.
    pub fn on_close(mut self, message: Message) -> Self {
        self.on_close = Some(message);
        self
    }

    /// Applies a narrow iced-style escape hatch after trigger style
    /// resolution.
    pub fn style_override(
        mut self,
        style_override: impl Fn(SelectTriggerStyle, SelectStatus) -> SelectTriggerStyle + 'a,
    ) -> Self {
        self.style_override = Some(Box::new(style_override));
        self
    }
}

/// Convenience wrapper mirroring [`select()`](crate::select) helpers of peer
/// components.
///
/// ```rust,no_run
/// use iced::Element;
/// use iced_shadcn_v2::{Theme, select};
///
/// #[derive(Debug, Clone)]
/// enum Message {
///     FruitPicked(&'static str),
/// }
///
/// fn fruit<'a>(theme: &'a Theme, value: Option<&'static str>) -> Element<'a, Message> {
///     select("Select a fruit", theme)
///         .item(("apple", "Apple"))
///         .item(("banana", "Banana"))
///         .selected_maybe(value)
///         .on_select(Message::FruitPicked)
///         .into()
/// }
/// ```
pub fn select<'a, T, Message>(
    placeholder: impl Into<String>,
    theme: &'a Theme,
) -> Select<'a, T, Message>
where
    T: Clone + PartialEq,
{
    Select::new(theme).placeholder(placeholder)
}

impl<'a, T, Message> From<Select<'a, T, Message>> for Element<'a, Message>
where
    T: Clone + PartialEq + 'a,
    Message: Clone + 'a,
{
    fn from(select: Select<'a, T, Message>) -> Self {
        let Select {
            theme,
            rows,
            selection,
            select_type,
            placeholder,
            size,
            radius,
            width,
            max_height,
            text_size,
            disabled,
            invalid,
            deselectable,
            on_select,
            on_selection_change,
            on_open,
            on_close,
            style_override,
        } = select;

        Self::new(SelectWidget {
            theme,
            rows,
            selection,
            select_type,
            placeholder,
            size,
            radius,
            width,
            max_height,
            text_size,
            disabled,
            invalid,
            deselectable,
            on_select,
            on_selection_change,
            on_open,
            on_close,
            style_override,
            last_status: None,
        })
    }
}
