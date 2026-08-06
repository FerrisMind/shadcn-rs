//! Native-select component ported from shadcn-svelte to iced-shadcn-v2.
//!
//! The web component styles only the trigger `<select>` field
//! (`.cn-native-select` + chevron icon) and leaves the dropdown to the OS.
//! This port keeps that split: shadcn tokens style the field alone, while
//! the dropdown is the stock iced overlay menu (the same "system" popup
//! `pick_list` uses, themed by the runtime `iced::Theme`) — extended
//! structurally with `<optgroup>` headings and disabled options, exactly
//! like the platform popup renders them. Interactions match the native
//! contract: click to open, click-away / Esc to close, arrow keys + Enter
//! while open.
//!
//! Web attributes map as follows: `<option value>` is the typed value of
//! [`NativeSelectOption`]; `<option disabled>` / `<optgroup disabled>` are
//! [`NativeSelectOption::disabled`] / [`NativeSelectGroup::disabled`];
//! `size="sm"` is [`NativeSelectSize::Sm`]; `aria-invalid` is
//! [`NativeSelect::invalid`] and `disabled` is [`NativeSelect::disabled`].
//! The web placeholder idiom — a first option with an empty value — becomes
//! the explicit [`NativeSelect::placeholder`].
//!
//! One web detail degrades on iced: the translucent `focus-visible:ring-*`
//! halo is approximated by recoloring the border with `ring`, exactly like
//! the input component.
//!
//! ```rust,no_run
//! use iced::Element;
//! use iced_shadcn_v2::{NativeSelect, Theme};
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     StatusPicked(&'static str),
//! }
//!
//! fn status<'a>(theme: &'a Theme, value: Option<&'static str>) -> Element<'a, Message> {
//!     NativeSelect::new(theme)
//!         .placeholder("Select status")
//!         .option(("todo", "Todo"))
//!         .option(("in-progress", "In Progress"))
//!         .option(("done", "Done"))
//!         .selected_maybe(value)
//!         .on_select(Message::StatusPicked)
//!         .into()
//! }
//! ```

mod render;
mod style;
mod types;

#[cfg(test)]
mod tests;

pub use style::{NativeSelectStatus, NativeSelectStyle};
pub use types::{NativeSelectGroup, NativeSelectOption, NativeSelectRadius, NativeSelectSize};

use std::fmt;

use crate::iced_compat::{Element, Length, Pixels};

use shadcn_common::AccentColor;

use crate::theme::Theme;

use render::NativeSelectWidget;
use types::Row;

/// Builder-first native select styled directly with iced types.
///
/// Theme tokens come from `shadcn-common` via [`Theme`]; iced styles are
/// built directly on top of `twill-core` tokens, without an intermediate
/// style layer. Pass `&theme` into every select — style packs (Vega, Nova, …)
/// live on the app's [`Theme`], not on this builder.
///
/// The value type `T` only needs `Clone + PartialEq`: labels are explicit
/// per option instead of `Display`-derived, mirroring how the web `<option>`
/// separates `value` from its text.
///
/// ```rust,no_run
/// use iced::Element;
/// use iced_shadcn_v2::{NativeSelect, NativeSelectGroup, Theme};
///
/// #[derive(Debug, Clone)]
/// enum Message {
///     DepartmentPicked(&'static str),
/// }
///
/// fn department<'a>(theme: &'a Theme, value: Option<&'static str>) -> Element<'a, Message> {
///     NativeSelect::new(theme)
///         .placeholder("Select department")
///         .group(
///             NativeSelectGroup::new("Engineering")
///                 .option(("frontend", "Frontend"))
///                 .option(("backend", "Backend")),
///         )
///         .group(
///             NativeSelectGroup::new("Sales")
///                 .option(("sales-rep", "Sales Rep"))
///                 .option(("account-manager", "Account Manager")),
///         )
///         .selected_maybe(value)
///         .on_select(Message::DepartmentPicked)
///         .into()
/// }
/// ```
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct NativeSelect<'a, T, Message>
where
    T: Clone + PartialEq,
{
    theme: &'a Theme,
    rows: Vec<Row<T>>,
    selected: Option<T>,
    placeholder: Option<String>,
    size: NativeSelectSize,
    radius: Option<NativeSelectRadius>,
    /// `None` = theme ring; `Some` = accent overlay from `shadcn-common`.
    color: Option<AccentColor>,
    width: Length,
    text_size: Option<f32>,
    disabled: bool,
    invalid: bool,
    on_select: Option<Box<dyn Fn(T) -> Message + 'a>>,
    on_open: Option<Message>,
    on_close: Option<Message>,
    style_override:
        Option<Box<dyn Fn(NativeSelectStyle, NativeSelectStatus) -> NativeSelectStyle + 'a>>,
}

impl<T, Message> fmt::Debug for NativeSelect<'_, T, Message>
where
    T: Clone + PartialEq + fmt::Debug,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("NativeSelect")
            .field("theme", &self.theme)
            .field("rows", &self.rows.len())
            .field("selected", &self.selected)
            .field("placeholder", &self.placeholder)
            .field("size", &self.size)
            .field("radius", &self.radius)
            .field("color", &self.color)
            .field("width", &self.width)
            .field("text_size", &self.text_size)
            .field("disabled", &self.disabled)
            .field("invalid", &self.invalid)
            .field("on_select", &self.on_select.is_some())
            .field("on_open", &self.on_open.is_some())
            .field("on_close", &self.on_close.is_some())
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

impl<'a, T, Message> NativeSelect<'a, T, Message>
where
    T: Clone + PartialEq,
{
    /// Creates an empty select.
    ///
    /// `theme` is required because styling is derived from `shadcn-common`
    /// theme tokens instead of `iced::Theme`.
    ///
    /// ```rust
    /// use iced_shadcn_v2::{NativeSelect, Theme};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let select = NativeSelect::<&str, Message>::new(&theme);
    /// ```
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            rows: Vec::new(),
            selected: None,
            placeholder: None,
            size: NativeSelectSize::Default,
            radius: None,
            color: None,
            width: Length::Shrink,
            text_size: None,
            disabled: false,
            invalid: false,
            on_select: None,
            on_open: None,
            on_close: None,
            style_override: None,
        }
    }

    /// Appends one top-level option (`<option>`).
    pub fn option(mut self, option: impl Into<NativeSelectOption<T>>) -> Self {
        let option = option.into();

        self.rows.push(Row::Option {
            value: option.value,
            label: option.label,
            disabled: option.disabled,
            indented: false,
        });
        self
    }

    /// Appends every option of the iterator at the top level.
    pub fn options<I, O>(mut self, options: I) -> Self
    where
        I: IntoIterator<Item = O>,
        O: Into<NativeSelectOption<T>>,
    {
        for option in options {
            self = self.option(option);
        }

        self
    }

    /// Appends a labelled group of options (`<optgroup>`).
    ///
    /// A disabled group disables every option inside it, like the web
    /// attribute.
    pub fn group(mut self, group: NativeSelectGroup<T>) -> Self {
        self.rows.push(Row::GroupLabel { label: group.label });

        for option in group.options {
            self.rows.push(Row::Option {
                value: option.value,
                label: option.label,
                disabled: option.disabled || group.disabled,
                indented: true,
            });
        }

        self
    }

    /// Sets the currently selected value.
    ///
    /// The application owns the selection: store it in state and feed it
    /// back on every [`Self::on_select`] message, mirroring `bind:value`.
    pub fn selected(mut self, selected: T) -> Self {
        self.selected = Some(selected);
        self
    }

    /// Sets or clears the currently selected value.
    pub fn selected_maybe(mut self, selected: Option<T>) -> Self {
        self.selected = selected;
        self
    }

    /// Sets the text shown while nothing is selected.
    ///
    /// Replaces the web idiom of a first `<option value="">` acting as the
    /// placeholder.
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    /// Sets the preset control size (`size="sm" | "default"` plus an iced
    /// `Lg` extension).
    pub fn size(mut self, size: NativeSelectSize) -> Self {
        self.size = size;
        self
    }

    /// Sets the field corner radius.
    ///
    /// Without an explicit radius the active style pack decides (`rounded-md`
    /// on Vega, pill on Maia/Luma, square on Lyra/Sera, …).
    pub fn radius(mut self, radius: NativeSelectRadius) -> Self {
        self.radius = Some(radius);
        self
    }

    /// Applies an accent color overlay to the open-state focus border.
    pub fn color(mut self, color: AccentColor) -> Self {
        self.color = Some(color);
        self
    }

    /// Alias for [`NativeSelect::color`] retained for semantic color APIs.
    pub fn tone(self, color: AccentColor) -> Self {
        self.color(color)
    }

    /// Sets a custom field width.
    ///
    /// Defaults to `Length::Shrink`: like the web `w-fit` wrapper, the field
    /// sizes to its widest option so it never resizes on selection.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the value text size. The pack's `.cn-native-select` size is used
    /// by default (`text-sm` on Vega, `text-xs` on Lyra/Mira).
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

    /// Sets the callback receiving the picked value.
    ///
    /// Without it the field renders but never opens, matching the iced
    /// convention that message-less controls are inert.
    pub fn on_select(mut self, on_select: impl Fn(T) -> Message + 'a) -> Self {
        self.on_select = Some(Box::new(on_select));
        self
    }

    /// Sets or clears the callback receiving the picked value.
    pub fn on_select_maybe(mut self, on_select: Option<impl Fn(T) -> Message + 'a>) -> Self {
        self.on_select = on_select.map(|on_select| Box::new(on_select) as _);
        self
    }

    /// Sets the message emitted when the dropdown opens.
    pub fn on_open(mut self, message: Message) -> Self {
        self.on_open = Some(message);
        self
    }

    /// Sets the message emitted when the dropdown closes without a pick.
    pub fn on_close(mut self, message: Message) -> Self {
        self.on_close = Some(message);
        self
    }

    /// Applies a narrow iced-style escape hatch after internal style
    /// resolution.
    pub fn style_override(
        mut self,
        style_override: impl Fn(NativeSelectStyle, NativeSelectStatus) -> NativeSelectStyle + 'a,
    ) -> Self {
        self.style_override = Some(Box::new(style_override));
        self
    }
}

/// Convenience wrapper mirroring [`iced::widget::pick_list()`](iced_widget::pick_list()).
///
/// ```rust,no_run
/// use iced::Element;
/// use iced_shadcn_v2::{Theme, native_select};
///
/// #[derive(Debug, Clone)]
/// enum Message {
///     FruitPicked(&'static str),
/// }
///
/// fn fruit<'a>(theme: &'a Theme, value: Option<&'static str>) -> Element<'a, Message> {
///     native_select("Select a fruit", theme)
///         .option(("apple", "Apple"))
///         .option(("banana", "Banana"))
///         .selected_maybe(value)
///         .on_select(Message::FruitPicked)
///         .into()
/// }
/// ```
pub fn native_select<'a, T, Message>(
    placeholder: impl Into<String>,
    theme: &'a Theme,
) -> NativeSelect<'a, T, Message>
where
    T: Clone + PartialEq,
{
    NativeSelect::new(theme).placeholder(placeholder)
}

impl<'a, T, Message> From<NativeSelect<'a, T, Message>> for Element<'a, Message>
where
    T: Clone + PartialEq + 'a,
    Message: Clone + 'a,
{
    fn from(select: NativeSelect<'a, T, Message>) -> Self {
        let NativeSelect {
            theme,
            rows,
            selected,
            placeholder,
            size,
            radius,
            color,
            width,
            text_size,
            disabled,
            invalid,
            on_select,
            on_open,
            on_close,
            style_override,
        } = select;

        Self::new(NativeSelectWidget {
            theme,
            rows,
            selected,
            placeholder,
            size,
            radius,
            color,
            width,
            text_size,
            disabled,
            invalid,
            on_select,
            on_open,
            on_close,
            style_override,
            last_status: None,
        })
    }
}
