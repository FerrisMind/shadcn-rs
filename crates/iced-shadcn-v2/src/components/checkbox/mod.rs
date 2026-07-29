//! Checkbox component ported from shadcn-svelte to iced-shadcn-v2.
//!
//! The checkbox keeps its state controlled by the application, just like the
//! original web component. It supports checked, unchecked, and indeterminate
//! states, optional labels, disabled controls, variants, sizes, and callbacks.
//!
//! ```rust,no_run
//! use iced::Element;
//! use iced_shadcn_v2::{Checkbox, CheckboxState, Theme};
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     TermsChanged(CheckboxState),
//! }
//!
//! fn terms_checkbox(theme: &Theme) -> Element<'_, Message> {
//!     Checkbox::new(theme)
//!         .label("Accept terms and conditions")
//!         .state(CheckboxState::Unchecked)
//!         .on_toggle(Message::TermsChanged)
//!         .into()
//! }
//! ```

mod geometry;
mod render;
mod style;
mod types;

#[cfg(test)]
mod tests;

pub use types::{CheckboxConfig, CheckboxSize, CheckboxState, CheckboxVariant};

use crate::iced_compat::widget::checkbox as checkbox_widget;
use crate::iced_compat::{Element, Length, Pixels};
use std::fmt;

use crate::theme::Theme;

/// Builder-first checkbox component.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct Checkbox<'a, Message> {
    config: CheckboxConfig,
    theme: &'a Theme,
    width: Length,
    spacing: f32,
    text_size: Option<Pixels>,
    on_press: Option<Message>,
    on_toggle: Option<Box<dyn Fn(CheckboxState) -> Message + 'a>>,
}

impl<Message> fmt::Debug for Checkbox<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Checkbox")
            .field("config", &self.config)
            .field("theme", &self.theme)
            .field("width", &self.width)
            .field("spacing", &self.spacing)
            .field("text_size", &self.text_size)
            .field("on_press", &self.on_press.is_some())
            .field("on_toggle", &self.on_toggle.is_some())
            .finish()
    }
}

impl<'a, Message: 'a> Checkbox<'a, Message> {
    /// Creates a new checkbox with default state.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            config: CheckboxConfig {
                state: CheckboxState::Unchecked,
                variant: CheckboxVariant::Surface,
                size: CheckboxSize::Lg,
                label: None,
                disabled: false,
            },
            theme,
            width: Length::Shrink,
            spacing: 8.0,
            text_size: None,
            on_press: None,
            on_toggle: None,
        }
    }

    /// Sets the variant.
    pub fn variant(mut self, variant: CheckboxVariant) -> Self {
        self.config.variant = variant;
        self
    }

    /// Sets the size.
    pub fn size(mut self, size: CheckboxSize) -> Self {
        self.config.size = size;
        self
    }

    /// Sets the label text.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.config.label = Some(label.into());
        self
    }

    /// Sets the state.
    pub fn state(mut self, state: CheckboxState) -> Self {
        self.config.state = state;
        self
    }

    /// Sets indeterminate (alias for state).
    pub fn indeterminate(mut self) -> Self {
        self.config.state = CheckboxState::Indeterminate;
        self
    }

    /// Sets disabled.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.config.disabled = disabled;
        self
    }

    /// Sets a custom width for the checkbox and its optional label.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the gap between the indicator and the optional label.
    pub fn spacing(mut self, spacing: impl Into<Pixels>) -> Self {
        self.spacing = spacing.into().0;
        self
    }

    /// Sets the label text size. The theme font is used by default.
    pub fn text_size(mut self, text_size: impl Into<Pixels>) -> Self {
        self.text_size = Some(text_size.into());
        self
    }

    /// Sets the message emitted when the checkbox is pressed.
    ///
    /// The message is emitted for every press. Applications that need the
    /// resulting state can use [`Self::on_toggle`] instead.
    pub fn on_press(mut self, message: Message) -> Self {
        self.on_press = Some(message);
        self.on_toggle = None;
        self
    }

    /// Sets or clears the message emitted with the next controlled state.
    pub fn on_press_maybe(mut self, message: Option<Message>) -> Self {
        self.on_press = message;
        self.on_toggle = None;
        self
    }

    /// Sets a callback that receives the next tri-state value.
    ///
    /// The callback is evaluated by the application after the control is
    /// pressed; the checkbox itself remains controlled by [`Self::state`].
    pub fn on_toggle<F>(mut self, on_toggle: F) -> Self
    where
        F: Fn(CheckboxState) -> Message + 'a,
    {
        self.on_toggle = Some(Box::new(on_toggle));
        self.on_press = None;
        self
    }

    /// Sets or clears the controlled-state callback.
    pub fn on_toggle_maybe<F>(mut self, on_toggle: Option<F>) -> Self
    where
        F: Fn(CheckboxState) -> Message + 'a,
    {
        self.on_toggle = on_toggle.map(|callback| Box::new(callback) as _);
        self.on_press = None;
        self
    }

    /// Alias for [`Self::on_toggle`] using the terminology of shadcn-svelte.
    pub fn on_change<F>(self, on_change: F) -> Self
    where
        F: Fn(CheckboxState) -> Message + 'a,
    {
        self.on_toggle(on_change)
    }

    /// Builds the underlying iced checkbox widget.
    pub fn into_widget(self) -> checkbox_widget::Checkbox<'a, Message>
    where
        Message: Clone,
    {
        let Checkbox {
            config,
            theme,
            width,
            spacing,
            text_size,
            on_press,
            on_toggle,
        } = self;

        render::build_checkbox(
            config, theme, width, spacing, text_size, on_press, on_toggle,
        )
    }

    /// Builds the Element.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: Clone,
    {
        self.into_widget().into()
    }
}

impl<'a, Message: Clone + 'a> From<Checkbox<'a, Message>> for Element<'a, Message> {
    fn from(checkbox: Checkbox<'a, Message>) -> Self {
        checkbox.into_element()
    }
}
