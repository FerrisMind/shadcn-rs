//! Password component ported from shadcn-svelte-extras to iced-shadcn-v2.
//!
//! Mirrors the composable extras suite: [`Password`] (Root), [`PasswordInput`],
//! [`PasswordToggleVisibility`], [`PasswordCopy`], and [`PasswordStrength`].
//! Behaviour and extras-only geometry live in `shadcn-common`
//! ([`shadcn_common::PasswordState`], [`shadcn_common::password_recipe`]).
//!
//! **Style packs:** the extras Password has no pack-specific `.cn-password`
//! tables (only shared Tailwind utilities). Choosing Rhea (or Nova, …) on the
//! shared [`Theme`] styles Password by styling its parts — [`PasswordInput`] →
//! Input / `.cn-input`, [`PasswordToggleVisibility`] → Toggle,
//! [`PasswordCopy`] → CopyButton / Button — all via `theme.style_id()`. Do not
//! invent a separate Password style table.
//!
//! ```rust,no_run
//! use iced::Element;
//! use iced_shadcn_v2::{
//!     Password, PasswordInput, PasswordStrength, PasswordToggleVisibility, Theme,
//! };
//! use shadcn_common::{PasswordAction, PasswordState};
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     Password(PasswordAction),
//! }
//!
//! fn view<'a>(theme: &'a Theme, state: &'a PasswordState) -> Element<'a, Message> {
//!     Password::new(theme)
//!         .push(
//!             PasswordInput::new(theme)
//!                 .value(state.value())
//!                 .hidden(state.hidden())
//!                 .invalid(state.is_invalid())
//!                 .placeholder("Password")
//!                 .on_input(|value| Message::Password(PasswordAction::SetValue(value)))
//!                 .toggle(
//!                     PasswordToggleVisibility::new(theme)
//!                         .hidden(state.hidden())
//!                         .on_toggle(|_| Message::Password(PasswordAction::ToggleHidden)),
//!                 ),
//!         )
//!         .push(PasswordStrength::new(theme).score(state.score()))
//!         .into()
//! }
//! ```

mod copy;
mod icon;
mod input;
mod render;
mod strength;
mod toggle;
mod types;

#[cfg(test)]
mod tests;

pub use copy::PasswordCopy;
pub use input::PasswordInput;
pub use strength::PasswordStrength;
pub use toggle::PasswordToggleVisibility;
pub use types::PasswordActionSlot;

use std::fmt;

use crate::iced_compat::widget::column;
use crate::iced_compat::{Element, Length};
use crate::theme::Theme;

/// Root column for the password suite (`flex flex-col gap-2`).
///
/// Compose [`PasswordInput`], [`PasswordStrength`], and any supporting text
/// with [`Self::push`], matching the Svelte `Password.Root` children slot.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct Password<'a, Message> {
    theme: &'a Theme,
    children: Vec<Element<'a, Message>>,
    width: Length,
    gap: Option<f32>,
}

impl<Message> fmt::Debug for Password<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Password")
            .field("theme", &self.theme)
            .field("children", &self.children.len())
            .field("width", &self.width)
            .field("gap", &self.gap)
            .finish()
    }
}

impl<'a, Message> Password<'a, Message> {
    /// Creates an empty password root.
    ///
    /// ```rust
    /// use iced_shadcn_v2::{Password, Theme};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let _root = Password::<Message>::new(&theme);
    /// ```
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            children: Vec::new(),
            width: Length::Fill,
            gap: None,
        }
    }

    /// Appends a child element (input, strength, label, …).
    #[must_use = "builder methods return the modified password"]
    pub fn push(mut self, child: impl Into<Element<'a, Message>>) -> Self {
        self.children.push(child.into());
        self
    }

    /// Appends several children.
    #[must_use = "builder methods return the modified password"]
    pub fn extend(mut self, children: impl IntoIterator<Item = Element<'a, Message>>) -> Self {
        self.children.extend(children);
        self
    }

    /// Sets the root width (`w-full` by default).
    #[must_use = "builder methods return the modified password"]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Overrides the root gap (`gap-2` → 8 px by default).
    #[must_use = "builder methods return the modified password"]
    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = Some(gap.max(0.0));
        self
    }

    /// Builds the root as an iced element.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        let gap = self
            .gap
            .unwrap_or_else(|| self.theme.style.password().root_gap_px);
        column(self.children).spacing(gap).width(self.width).into()
    }
}

impl<'a, Message: 'a> From<Password<'a, Message>> for Element<'a, Message> {
    fn from(password: Password<'a, Message>) -> Self {
        password.into_element()
    }
}

/// Convenience constructor matching other component free functions.
pub fn password<'a, Message: 'a>(theme: &'a Theme) -> Password<'a, Message> {
    Password::new(theme)
}
