//! Copy action for the password input.

use std::fmt;
use std::time::Duration;

use crate::components::button::ButtonVariant;
use crate::components::copy_button::{CopyButton, CopyButtonAction, CopyButtonStatus};
use crate::iced_compat::{Element, Length};
use crate::theme::Theme;
use twill_core::prelude::theme::SemanticColor;

/// Absolute copy button (`Password.Copy`).
///
/// Clipboard ownership stays application-side, matching [`CopyButton`].
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct PasswordCopy<'a, Message> {
    theme: &'a Theme,
    text: String,
    status: CopyButtonStatus,
    disabled: bool,
    on_copy: Option<Box<dyn Fn(CopyButtonAction) -> Message + 'a>>,
}

impl<Message> fmt::Debug for PasswordCopy<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PasswordCopy")
            .field("theme", &self.theme)
            .field("text_length", &self.text.len())
            .field("status", &self.status)
            .field("disabled", &self.disabled)
            .field("on_copy", &self.on_copy.is_some())
            .finish()
    }
}

impl<'a, Message> PasswordCopy<'a, Message> {
    /// Creates a copy button for the current password value.
    pub fn new(text: impl Into<String>, theme: &'a Theme) -> Self {
        Self {
            theme,
            text: text.into(),
            status: CopyButtonStatus::Idle,
            disabled: false,
            on_copy: None,
        }
    }

    /// Sets the controlled copy feedback status.
    #[must_use = "builder methods return the modified copy button"]
    pub fn status(mut self, status: CopyButtonStatus) -> Self {
        self.status = status;
        self
    }

    /// Disables the copy button.
    #[must_use = "builder methods return the modified copy button"]
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Sets the copy-action callback.
    #[must_use = "builder methods return the modified copy button"]
    pub fn on_copy(mut self, on_copy: impl Fn(CopyButtonAction) -> Message + 'a) -> Self {
        self.on_copy = Some(Box::new(on_copy));
        self
    }

    /// Builds the copy button element.
    pub(super) fn into_element(self) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        let PasswordCopy {
            theme,
            text,
            status,
            disabled,
            on_copy,
        } = self;

        let recipe = theme.style.password();
        let muted = theme.semantic_color(SemanticColor::MutedForeground);

        let mut button = CopyButton::new(text, theme)
            .variant(ButtonVariant::Ghost)
            .status(status)
            .disabled(disabled)
            .width(Length::Fixed(recipe.action_size_px))
            .height(Length::Fixed(recipe.action_size_px))
            .animation_duration(Duration::from_millis(recipe.strength_transition_ms as u64))
            .style_override(move |mut style, _status| {
                style.background = None;
                style.border.width = 0.0;
                style.text_color = muted;
                style
            });

        if let Some(on_copy) = on_copy {
            button = button.on_copy_action(on_copy);
        }

        button.into()
    }
}
