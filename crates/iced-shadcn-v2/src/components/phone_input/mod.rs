//! Builder-first phone input composed from country selector + text field.
//!
//! Ports shadcn-svelte-extras `PhoneInput` to iced-shadcn-v2: an outline
//! country trigger (popover + command search) joined to a themed
//! [`crate::Input`]. Country tables, E.164 parse/format/validate, and
//! extras-only layout tokens live in [`shadcn_common`] so egui can reuse them.
//!
//! Pack look (Rhea / Nova / …) is **not** owned by Phone Input — the upstream
//! extras markup has no pack tables. Pass the app [`Theme`] through; Button,
//! Input, Command, and Popover resolve their own pack recipes from
//! `theme.style_id()`, the same rule as [`crate::Form`].
//!
//! The application owns country, value, open/query state and applies
//! [`PhoneInputChange`] snapshots from [`PhoneInput::on_change`].
//!
//! ```rust,no_run
//! use iced::Element;
//! use iced_shadcn_v2::{PhoneInput, PhoneInputChange, Theme};
//! use shadcn_common::CountryCode;
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     Changed(PhoneInputChange),
//!     Open(bool),
//!     Query(String),
//! }
//!
//! fn phone<'a>(
//!     theme: &'a Theme,
//!     value: &'a str,
//!     country: Option<CountryCode>,
//!     open: bool,
//!     query: &'a str,
//! ) -> Element<'a, Message> {
//!     PhoneInput::new(theme)
//!         .value(value)
//!         .country(country)
//!         .open(open)
//!         .query(query)
//!         .placeholder("Enter a phone number")
//!         .on_change(Message::Changed)
//!         .on_open_change(Message::Open)
//!         .on_query_change(Message::Query)
//!         .into()
//! }
//! ```

#![allow(clippy::double_must_use)]

mod icon;
mod render;
mod types;

#[cfg(test)]
mod tests;

pub use types::PhoneInputChange;

use std::cmp::Ordering;
use std::fmt;
use std::rc::Rc;

use shadcn_common::{CountryCode, PhoneCountry, PhoneInputOptions, default_country_order};

use crate::iced_compat::{Element, Length};
use crate::theme::Theme;

/// Builder-first phone input styled from `shadcn-common` theme tokens.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct PhoneInput<'a, Message> {
    pub(super) theme: &'a Theme,
    pub(super) value: &'a str,
    pub(super) country: Option<CountryCode>,
    pub(super) default_country: Option<CountryCode>,
    pub(super) placeholder: Option<&'a str>,
    pub(super) name: Option<&'a str>,
    pub(super) options: PhoneInputOptions,
    pub(super) disabled: bool,
    pub(super) readonly: bool,
    pub(super) required: bool,
    pub(super) invalid: Option<bool>,
    pub(super) width: Length,
    pub(super) open: Option<bool>,
    pub(super) query: &'a str,
    pub(super) order: fn(&PhoneCountry, &PhoneCountry) -> Ordering,
    pub(super) on_change: Option<Rc<dyn Fn(PhoneInputChange) -> Message + 'a>>,
    pub(super) on_open_change: Option<Box<dyn Fn(bool) -> Message + 'a>>,
    pub(super) on_query_change: Option<Box<dyn Fn(String) -> Message + 'a>>,
    pub(super) on_submit: Option<Message>,
}

impl<Message> fmt::Debug for PhoneInput<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PhoneInput")
            .field("theme", &self.theme)
            .field("value", &self.value)
            .field("country", &self.country)
            .field("default_country", &self.default_country)
            .field("placeholder", &self.placeholder)
            .field("name", &self.name)
            .field("options", &self.options)
            .field("disabled", &self.disabled)
            .field("readonly", &self.readonly)
            .field("required", &self.required)
            .field("invalid", &self.invalid)
            .field("width", &self.width)
            .field("open", &self.open)
            .field("query", &self.query)
            .field("on_change", &self.on_change.is_some())
            .field("on_open_change", &self.on_open_change.is_some())
            .field("on_query_change", &self.on_query_change.is_some())
            .field("on_submit", &self.on_submit.is_some())
            .finish()
    }
}

impl<'a, Message> PhoneInput<'a, Message> {
    /// Creates an empty phone input with default tel options.
    ///
    /// ```rust
    /// use iced_shadcn_v2::{PhoneInput, Theme};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let _input = PhoneInput::<Message>::new(&theme);
    /// ```
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            value: "",
            country: None,
            default_country: None,
            placeholder: None,
            name: None,
            options: PhoneInputOptions::DEFAULT,
            disabled: false,
            readonly: false,
            required: false,
            invalid: None,
            width: Length::Fill,
            open: None,
            query: "",
            order: default_country_order,
            on_change: None,
            on_open_change: None,
            on_query_change: None,
            on_submit: None,
        }
    }

    /// Sets the controlled phone value (E.164 or working text).
    pub fn value(mut self, value: &'a str) -> Self {
        self.value = value;
        self
    }

    /// Sets the selected country (`country` / `bind:country`).
    pub fn country(mut self, country: Option<CountryCode>) -> Self {
        self.country = country;
        self
    }

    /// Sets the fallback country when none is selected (`defaultCountry`).
    pub fn default_country(mut self, country: Option<CountryCode>) -> Self {
        self.default_country = country;
        self
    }

    /// Sets the input placeholder. When omitted and `auto_placeholder` is on,
    /// an example national number for the active country is used.
    pub fn placeholder(mut self, placeholder: impl Into<&'a str>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    /// Optional form field name (API parity; iced has no hidden input).
    pub fn name(mut self, name: impl Into<&'a str>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Sets tel-input options (`spaces`, `autoPlaceholder`).
    pub fn options(mut self, options: PhoneInputOptions) -> Self {
        self.options = options;
        self
    }

    /// Disables both the country trigger and the text field.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Keeps the value visible but non-editable (`readonly`).
    pub fn readonly(mut self, readonly: bool) -> Self {
        self.readonly = readonly;
        self
    }

    /// Marks the control as required for form APIs (`required`).
    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    /// Forces the invalid (destructive) border. When `None`, invalid styling
    /// follows the latest parse result once a non-empty value is present.
    pub fn invalid(mut self, invalid: bool) -> Self {
        self.invalid = Some(invalid);
        self
    }

    /// Sets the combined control width (`Length::Fill` by default).
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Controlled open state for the country popover.
    pub fn open(mut self, open: bool) -> Self {
        self.open = Some(open);
        self
    }

    /// Country-list search query owned by the application.
    pub fn query(mut self, query: &'a str) -> Self {
        self.query = query;
        self
    }

    /// Custom country sort, matching the web `order` prop.
    pub fn order(mut self, order: fn(&PhoneCountry, &PhoneCountry) -> Ordering) -> Self {
        self.order = order;
        self
    }

    /// Receives value / country / validity / detailed snapshots.
    pub fn on_change(mut self, callback: impl Fn(PhoneInputChange) -> Message + 'a) -> Self {
        self.on_change = Some(Rc::new(callback));
        self
    }

    /// Receives country-popover open changes.
    pub fn on_open_change(mut self, callback: impl Fn(bool) -> Message + 'a) -> Self {
        self.on_open_change = Some(Box::new(callback));
        self
    }

    /// Receives country-list search query edits.
    pub fn on_query_change(mut self, callback: impl Fn(String) -> Message + 'a) -> Self {
        self.on_query_change = Some(Box::new(callback));
        self
    }

    /// Message emitted when Enter is pressed in the text field.
    pub fn on_submit(mut self, message: Message) -> Self {
        self.on_submit = Some(message);
        self
    }

    /// Resolved country: explicit selection, else `default_country`.
    #[must_use]
    pub fn resolved_country(&self) -> Option<CountryCode> {
        self.country.or(self.default_country)
    }
}

/// Convenience constructor.
pub fn phone_input<'a, Message>(theme: &'a Theme) -> PhoneInput<'a, Message> {
    PhoneInput::new(theme)
}

impl<'a, Message: Clone + 'a> From<PhoneInput<'a, Message>> for Element<'a, Message> {
    fn from(input: PhoneInput<'a, Message>) -> Self {
        render::build(input)
    }
}
