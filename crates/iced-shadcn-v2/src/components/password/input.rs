//! Password input with optional absolute trailing actions.

use std::fmt;

use crate::components::input::{Input, InputSize};
use crate::iced_compat::alignment::{Horizontal, Vertical};
use crate::iced_compat::widget::text::{Fragment, IntoFragment};
use crate::iced_compat::widget::{container, row, stack};
use crate::iced_compat::{Element, Length, Padding};
use crate::theme::Theme;

use super::copy::PasswordCopy;
use super::render;
use super::toggle::PasswordToggleVisibility;

/// Password text field with absolute toggle / copy overlays.
///
/// Mirrors `Password.Input`: a relative wrapper around a secure [`Input`],
/// with trailing actions stacked on the right and end padding of `pr-9` or
/// `pr-[4.5rem]` depending on which actions are mounted.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct PasswordInput<'a, Message> {
    theme: &'a Theme,
    value: Fragment<'a>,
    placeholder: Fragment<'a>,
    hidden: bool,
    invalid: bool,
    disabled: bool,
    width: Length,
    size: InputSize,
    toggle: Option<PasswordToggleVisibility<'a, Message>>,
    copy: Option<PasswordCopy<'a, Message>>,
    on_input: Option<Box<dyn Fn(String) -> Message + 'a>>,
    on_submit: Option<Message>,
}

impl<Message> fmt::Debug for PasswordInput<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PasswordInput")
            .field("theme", &self.theme)
            .field("value", &"<redacted>")
            .field("placeholder", &self.placeholder)
            .field("hidden", &self.hidden)
            .field("invalid", &self.invalid)
            .field("disabled", &self.disabled)
            .field("width", &self.width)
            .field("size", &self.size)
            .field("toggle", &self.toggle.is_some())
            .field("copy", &self.copy.is_some())
            .field("on_input", &self.on_input.is_some())
            .field("on_submit", &self.on_submit.is_some())
            .finish()
    }
}

impl<'a, Message> PasswordInput<'a, Message> {
    /// Creates an empty password input.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            value: Fragment::default(),
            placeholder: Fragment::default(),
            hidden: true,
            invalid: false,
            disabled: false,
            width: Length::Fill,
            size: InputSize::Default,
            toggle: None,
            copy: None,
            on_input: None,
            on_submit: None,
        }
    }

    /// Sets the controlled value.
    #[must_use = "builder methods return the modified password input"]
    pub fn value(mut self, value: impl IntoFragment<'a>) -> Self {
        self.value = value.into_fragment();
        self
    }

    /// Sets the placeholder text.
    #[must_use = "builder methods return the modified password input"]
    pub fn placeholder(mut self, placeholder: impl IntoFragment<'a>) -> Self {
        self.placeholder = placeholder.into_fragment();
        self
    }

    /// Masks the value when `true` (`type="password"`).
    #[must_use = "builder methods return the modified password input"]
    pub fn hidden(mut self, hidden: bool) -> Self {
        self.hidden = hidden;
        self
    }

    /// Marks the field invalid (`aria-invalid`).
    #[must_use = "builder methods return the modified password input"]
    pub fn invalid(mut self, invalid: bool) -> Self {
        self.invalid = invalid;
        self
    }

    /// Disables editing.
    #[must_use = "builder methods return the modified password input"]
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Sets the input width.
    #[must_use = "builder methods return the modified password input"]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the control size ladder.
    #[must_use = "builder methods return the modified password input"]
    pub fn size(mut self, size: InputSize) -> Self {
        self.size = size;
        self
    }

    /// Composes the visibility toggle inside the input.
    #[must_use = "builder methods return the modified password input"]
    pub fn toggle(mut self, toggle: PasswordToggleVisibility<'a, Message>) -> Self {
        self.toggle = Some(toggle);
        self
    }

    /// Composes the copy button inside the input.
    #[must_use = "builder methods return the modified password input"]
    pub fn copy(mut self, copy: PasswordCopy<'a, Message>) -> Self {
        self.copy = Some(copy);
        self
    }

    /// Sets the edit callback.
    #[must_use = "builder methods return the modified password input"]
    pub fn on_input(mut self, on_input: impl Fn(String) -> Message + 'a) -> Self {
        self.on_input = Some(Box::new(on_input));
        self
    }

    /// Sets the submit message (Enter).
    #[must_use = "builder methods return the modified password input"]
    pub fn on_submit(mut self, message: Message) -> Self {
        self.on_submit = Some(message);
        self
    }

    /// Builds the stacked input + trailing actions.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        let PasswordInput {
            theme,
            value,
            placeholder,
            hidden,
            invalid,
            disabled,
            width,
            size,
            toggle,
            copy,
            on_input,
            on_submit,
        } = self;

        let toggle_mounted = toggle.is_some();
        let copy_mounted = copy.is_some();
        let recipe = theme.style.password();

        let padding = render::input_padding(theme, toggle_mounted, copy_mounted);
        let mut input = Input::new(theme)
            .value(value)
            .placeholder(placeholder)
            .size(size)
            .width(Length::Fill)
            .secure(hidden)
            .invalid(invalid)
            .disabled(disabled)
            .padding(padding)
            .expect("password field padding is a finite inset");

        if let Some(on_input) = on_input {
            input = input.on_input(on_input);
        }
        if let Some(on_submit) = on_submit {
            input = input.on_submit(on_submit);
        }

        let field: Element<'a, Message> = input.into();

        if !toggle_mounted && !copy_mounted {
            return container(field).width(width).into();
        }

        let mut actions = row![].spacing(0).align_y(Vertical::Center);

        if let Some(toggle) = toggle {
            let compact = copy_mounted;
            actions = actions.push(toggle.into_element(compact));
        }
        if let Some(copy) = copy {
            actions = actions.push(copy.into_element());
        }

        let actions_layer = container(actions)
            .width(Length::Fill)
            .height(Length::Fixed(recipe.action_size_px))
            .align_x(Horizontal::Right)
            .align_y(Vertical::Center)
            .padding(Padding::ZERO);

        stack![field, actions_layer]
            .width(width)
            .height(Length::Shrink)
            .into()
    }
}

impl<'a, Message: Clone + 'a> From<PasswordInput<'a, Message>> for Element<'a, Message> {
    fn from(input: PasswordInput<'a, Message>) -> Self {
        input.into_element()
    }
}
