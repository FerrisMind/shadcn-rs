//! Visibility toggle for the password input.

use std::fmt;

use crate::components::toggle::{Toggle, ToggleSize, ToggleVariant};
use crate::iced_compat::{Color, Element, Length};
use crate::theme::Theme;
use twill_core::prelude::theme::SemanticColor;

use super::icon::{EyeGlyph, eye_icon};

/// Absolute visibility toggle (`Password.ToggleVisibility`).
///
/// Uses a transparent ghost-style toggle with Lucide eye / eye-off icons.
/// When a copy button is also mounted the toggle narrows to `max-w-6` (24 px)
/// and sits to the left of the copy control.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct PasswordToggleVisibility<'a, Message> {
    theme: &'a Theme,
    hidden: bool,
    disabled: bool,
    on_toggle: Option<Box<dyn Fn(bool) -> Message + 'a>>,
}

impl<Message> fmt::Debug for PasswordToggleVisibility<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PasswordToggleVisibility")
            .field("theme", &self.theme)
            .field("hidden", &self.hidden)
            .field("disabled", &self.disabled)
            .field("on_toggle", &self.on_toggle.is_some())
            .finish()
    }
}

impl<'a, Message> PasswordToggleVisibility<'a, Message> {
    /// Creates a visibility toggle.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            hidden: true,
            disabled: false,
            on_toggle: None,
        }
    }

    /// Whether the password is currently masked.
    ///
    /// `pressed` on the underlying toggle tracks `hidden` the same way the
    /// Svelte component binds `pressed={hidden}`.
    #[must_use = "builder methods return the modified toggle"]
    pub fn hidden(mut self, hidden: bool) -> Self {
        self.hidden = hidden;
        self
    }

    /// Disables the toggle.
    #[must_use = "builder methods return the modified toggle"]
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Sets the callback receiving the next `hidden` value.
    #[must_use = "builder methods return the modified toggle"]
    pub fn on_toggle(mut self, on_toggle: impl Fn(bool) -> Message + 'a) -> Self {
        self.on_toggle = Some(Box::new(on_toggle));
        self
    }

    /// Builds the toggle element.
    ///
    /// `compact` is `true` when a copy button is also mounted (`right-9 max-w-6`).
    pub(super) fn into_element(self, compact: bool) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        let PasswordToggleVisibility {
            theme,
            hidden,
            disabled,
            on_toggle,
        } = self;

        let recipe = theme.style.password();
        let muted: Color = theme.semantic_color(SemanticColor::MutedForeground);
        let glyph = if hidden {
            EyeGlyph::Eye
        } else {
            EyeGlyph::EyeOff
        };
        let icon = eye_icon(recipe.action_icon_px, muted, glyph);

        let width = if compact {
            recipe.toggle_compact_width_px
        } else {
            recipe.action_size_px
        };

        let mut toggle = Toggle::icon(icon, theme)
            .variant(ToggleVariant::Default)
            .size(ToggleSize::Default)
            .pressed(hidden)
            .disabled(disabled)
            .width(Length::Fixed(width))
            .height(Length::Fixed(recipe.action_size_px))
            .style_override(move |mut style, _status| {
                style.background = None;
                style.border.width = 0.0;
                style.text_color = muted;
                style
            });

        if let Some(on_toggle) = on_toggle {
            toggle = toggle.on_toggle(on_toggle);
        }

        toggle.into()
    }
}
