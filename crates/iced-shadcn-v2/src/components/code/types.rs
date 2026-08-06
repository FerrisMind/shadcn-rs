//! Public configuration types for the [`crate::components::code`] component.
//!
//! [`CodeOverflow`] and [`CodeCopyButton`] are optional decorations that the
//! [`crate::components::code::Code`] builder attaches to a code block. They are
//! consumed by the root builder and do nothing on their own.

use crate::components::button::{ButtonRadius, ButtonSize, ButtonVariant};
use crate::components::copy_button::CopyButtonStatus;
use crate::iced_compat::Element;
use std::fmt;

/// Visual treatment of a [`crate::components::code::Code`] block.
///
/// Mirrors the `codeVariants` of the reference Svelte component.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CodeVariant {
    /// Card surface with the theme border.
    #[default]
    Default,
    /// Soft secondary surface with a transparent border.
    Secondary,
}

/// Collapse / expand configuration for a [`crate::components::code::Code`] block.
///
/// Mirrors the `CodeOverflow` slot of the reference Svelte component: while
/// collapsed the block is clipped to `max_height` and a fade-out gradient is
/// painted at the bottom; the expand button sits centered below the fade.
///
/// The collapsed state follows the Svelte `$bindable` semantics: it starts at
/// [`new`](Self::new)'s value, lives inside the widget (uncontrolled mode) and
/// toggles when the expand button is pressed. Use
/// [`collapsed_override`](Self::collapsed_override) to fully control the value
/// from the application and [`on_collapse_change`](Self::on_collapse_change) to
/// observe toggles, either way.
///
/// # Examples
///
/// ```rust,no_run
/// use iced_shadcn_v2::{Code, CodeOverflow, Theme};
///
/// # #[derive(Debug, Clone)]
/// # enum Message { ExpandChanged(bool) }
/// # let theme = Theme::light();
/// let code_block = Code::new("let x = 42;", "rust", &theme)
///     .overflow(
///         CodeOverflow::new(true)
///             .max_height(300.0)
///             .on_collapse_change(Message::ExpandChanged),
///     )
///     .into_element();
/// ```
#[must_use = "a CodeOverflow does nothing unless attached to a Code"]
pub struct CodeOverflow<'a, Message> {
    pub(super) default_collapsed: bool,
    pub(super) collapsed_override: Option<bool>,
    pub(super) max_height: f32,
    pub(super) expand_button: Option<Element<'a, Message>>,
    pub(super) on_collapse_change: Option<Box<dyn Fn(bool) -> Message + 'a>>,
}

impl<Message> fmt::Debug for CodeOverflow<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CodeOverflow")
            .field("default_collapsed", &self.default_collapsed)
            .field("collapsed_override", &self.collapsed_override)
            .field("max_height", &self.max_height)
            .field("expand_button", &self.expand_button.is_some())
            .field("on_collapse_change", &self.on_collapse_change.is_some())
            .finish()
    }
}

impl<'a, Message> CodeOverflow<'a, Message> {
    /// Creates an overflow config whose block starts collapsed when
    /// `default_collapsed` is `true`.
    ///
    /// The block toggles its own state when the expand button is pressed
    /// (uncontrolled mode). See [`collapsed_override`](Self::collapsed_override)
    /// for the controlled alternative.
    pub fn new(default_collapsed: bool) -> Self {
        Self {
            default_collapsed,
            collapsed_override: None,
            max_height: 300.0,
            expand_button: None,
            on_collapse_change: None,
        }
    }

    /// Sets the clipped height in logical pixels while collapsed.
    ///
    /// Defaults to `300.0`, matching the reference `max-h-[300px]`.
    pub fn max_height(mut self, height: f32) -> Self {
        self.max_height = height;
        self
    }

    /// Overrides the collapsed state, turning the block into a *controlled*
    /// component: the widget no longer tracks the state itself and the value
    /// is read from `collapsed` every frame.
    ///
    /// Pass `None` to go back to the internal (uncontrolled) state.
    pub fn collapsed_override(mut self, collapsed: Option<bool>) -> Self {
        self.collapsed_override = collapsed;
        self
    }

    /// Sets the button shown below the fade while collapsed.
    ///
    /// By default a secondary `"Expand"` button is rendered; it toggles the
    /// collapsed state internally even when it carries no `on_press`. Provide
    /// your own button to customize the label or appearance (attach
    /// `on_press` to react to presses in addition to the internal toggle).
    pub fn expand_button(mut self, button: impl Into<Element<'a, Message>>) -> Self {
        self.expand_button = Some(button.into());
        self
    }

    /// Publishes `callback(expanded)` whenever the collapsed state changes,
    /// whether the toggle comes from the internal expand button or (in
    /// controlled mode) from the application flipping the state.
    pub fn on_collapse_change<F>(mut self, callback: F) -> Self
    where
        F: Fn(bool) -> Message + 'a,
    {
        self.on_collapse_change = Some(Box::new(callback));
        self
    }
}

/// Configuration for the copy button the root builder renders in the top-right
/// corner of a [`crate::components::code::Code`] block.
///
/// Mirrors the `CodeCopyButton` slot of the reference Svelte component: the
/// button copies the whole code block when pressed. Like
/// [`crate::components::CopyButton`], the actual clipboard write is performed
/// by the application from the `on_copy` message.
#[must_use = "a CodeCopyButton does nothing unless attached to a Code"]
pub struct CodeCopyButton<'a, Message> {
    pub(super) variant: ButtonVariant,
    pub(super) size: ButtonSize,
    pub(super) radius: Option<ButtonRadius>,
    pub(super) status: CopyButtonStatus,
    pub(super) icon: Option<Element<'a, Message>>,
    pub(super) on_copy: Message,
}

impl<Message> fmt::Debug for CodeCopyButton<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CodeCopyButton")
            .field("variant", &self.variant)
            .field("size", &self.size)
            .field("radius", &self.radius)
            .field("status", &self.status)
            .field("icon", &self.icon.is_some())
            .finish()
    }
}

impl<'a, Message> CodeCopyButton<'a, Message> {
    /// Creates a copy button config that publishes `on_copy` when pressed.
    pub fn new(on_copy: Message) -> Self {
        Self {
            variant: ButtonVariant::Ghost,
            size: ButtonSize::Icon,
            radius: None,
            status: CopyButtonStatus::Idle,
            icon: None,
            on_copy,
        }
    }

    /// Sets the visual variant of the underlying [`crate::components::CopyButton`].
    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Sets the size of the underlying [`crate::components::CopyButton`].
    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    /// Overrides the corner radius of the underlying button.
    pub fn radius(mut self, radius: ButtonRadius) -> Self {
        self.radius = Some(radius);
        self
    }

    /// Sets the status hint of the underlying [`crate::components::CopyButton`].
    pub fn status(mut self, status: CopyButtonStatus) -> Self {
        self.status = status;
        self
    }

    /// Replaces the idle icon with an arbitrary widget.
    pub fn icon(mut self, icon: impl Into<Element<'a, Message>>) -> Self {
        self.icon = Some(icon.into());
        self
    }
}
