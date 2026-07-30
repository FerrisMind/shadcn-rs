//! Builder-first sonner (toast notification) component.
//!
//! Port of the shadcn-svelte `sonner` component (`Toaster` + `toast()`
//! API). The `Toaster` widget occupies the full viewport and renders
//! toasts as an overlay at the configured screen position. The
//! imperative `toast()` API creates toasts through a global state
//! singleton, matching the original `svelte-sonner` design.
//!
//! # Architecture
//!
//! - **`toast()` function** — creates a toast in the global state.
//!   Called from anywhere; no app state needed.
//! - **`Toaster` widget** — placed once in the view tree (like a
//!   portal). Renders all active toasts, drives auto-close timers,
//!   and handles dismiss interactions.
//! - **`ToastState`** — global `Mutex<Vec<RawToast>>` with atomic
//!   change tracking for efficient polling.
//!
//! ```rust,no_run
//! use iced::Element;
//! use iced_shadcn_v2::{Toaster, Theme, toast, ToastOptions, ToastType};
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     ShowToast,
//! }
//!
//! fn view(theme: &Theme) -> Element<'_, Message> {
//!     iced::widget::column![
//!         Toaster::new(theme).into(),
//!         iced::widget::button("Show Toast")
//!             .on_press(Message::ShowToast),
//!     ]
//!     .into()
//! }
//! ```

mod render;
mod state;
mod style;
mod types;

#[cfg(test)]
mod tests;

pub use types::{ToastAction, ToastOptions, ToastPosition, ToastType};

use std::fmt;

use crate::iced_compat::Element;

use crate::theme::Theme;

/// Creates a default toast notification.
///
/// Returns a `SonnerToast` message that, when processed by the app's
/// `update` function, adds the toast to the global state. The
/// `Toaster` widget will then render it.
///
/// For a simpler API that modifies global state directly (without
/// requiring a message round-trip), use [`toast_immediate`].
///
/// ```rust,no_run
/// use iced_shadcn_v2::{toast, ToastOptions, ToastType};
///
/// // In an event handler:
/// // let msg = toast("Event has been created");
/// // // Then return msg from your update function.
/// ```
pub fn toast<Message>(message: impl Into<String>) -> SonnerToast<Message> {
    SonnerToast {
        title: message.into(),
        options: ToastOptions::default(),
    }
}

/// Creates a toast notification immediately (no message round-trip).
///
/// This modifies the global toast state directly. The `Toaster`
/// widget will pick it up on the next frame.
///
/// **Note:** Action and cancel callbacks are not supported with
/// immediate toasts because the global state doesn't know the
/// `Message` type. Use [`toast`] with a `SonnerToast` message
/// for toasts with action buttons.
///
/// ```rust,no_run
/// use iced_shadcn_v2::{toast_immediate, ToastType};
///
/// // In an event handler:
/// toast_immediate("Saved successfully", ToastType::Success);
/// ```
pub fn toast_immediate(message: impl Into<String>, toast_type: ToastType) {
    let options: ToastOptions<()> = ToastOptions::new(toast_type);
    state::create_typed_toast(message.into(), options);
}

/// Creates a success toast immediately.
pub fn toast_success(message: impl Into<String>) {
    toast_immediate(message, ToastType::Success);
}

/// Creates an error toast immediately.
pub fn toast_error(message: impl Into<String>) {
    toast_immediate(message, ToastType::Error);
}

/// Creates a warning toast immediately.
pub fn toast_warning(message: impl Into<String>) {
    toast_immediate(message, ToastType::Warning);
}

/// Creates an info toast immediately.
pub fn toast_info(message: impl Into<String>) {
    toast_immediate(message, ToastType::Info);
}

/// Creates a loading toast immediately.
pub fn toast_loading(message: impl Into<String>) {
    toast_immediate(message, ToastType::Loading);
}

/// Dismisses a specific toast by ID.
pub fn dismiss_toast(id: u64) {
    state::dismiss_toast(id);
}

/// Dismisses all active toasts.
pub fn dismiss_all_toasts() {
    state::dismiss_all_toasts();
}

/// A pending toast that can be configured before being sent to the
/// `Toaster`.
///
/// Create one with [`toast()`], chain builder methods, then return
/// it as a `Message` variant from your `update` function.
///
/// ```rust,no_run
/// use iced_shadcn_v2::{toast, ToastOptions, ToastType};
///
/// #[derive(Debug, Clone)]
/// enum Message {
///     ShowToast(iced_shadcn_v2::SonnerToast<Message>),
/// }
///
/// // In event handler:
/// // Message::ShowToast(
/// //     toast("Event created")
/// //         .description("Sunday, December 03, 2023")
/// //         .toast_type(ToastType::Success)
/// // )
/// ```
#[must_use = "a SonnerToast does nothing unless returned as a Message"]
pub struct SonnerToast<Message> {
    title: String,
    options: ToastOptions<Message>,
}

impl<Message> fmt::Debug for SonnerToast<Message> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SonnerToast")
            .field("title", &self.title)
            .field("toast_type", &self.options.toast_type)
            .finish()
    }
}

impl<Message> Clone for SonnerToast<Message>
where
    Message: Clone,
{
    fn clone(&self) -> Self {
        Self {
            title: self.title.clone(),
            options: ToastOptions {
                toast_type: self.options.toast_type,
                description: self.options.description.clone(),
                duration: self.options.duration,
                dismissible: self.options.dismissible,
                action: None, // Can't clone closures
                cancel: None,
                close_button: self.options.close_button,
                rich_colors: self.options.rich_colors,
                invert: self.options.invert,
                position: self.options.position,
            },
        }
    }
}

impl<Message> SonnerToast<Message> {
    /// Sets the description text.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.options = self.options.description(description);
        self
    }

    /// Sets the toast type.
    pub fn toast_type(mut self, toast_type: ToastType) -> Self {
        self.options.toast_type = toast_type;
        self
    }

    /// Sets the auto-dismiss duration in milliseconds.
    pub fn duration(mut self, duration: u64) -> Self {
        self.options = self.options.duration(duration);
        self
    }

    /// Sets whether the toast is dismissible.
    pub fn dismissible(mut self, dismissible: bool) -> Self {
        self.options = self.options.dismissible(dismissible);
        self
    }

    /// Sets the action button.
    pub fn action(mut self, action: ToastAction<Message>) -> Self {
        self.options = self.options.action(action);
        self
    }

    /// Sets the cancel button.
    pub fn cancel(mut self, cancel: ToastAction<Message>) -> Self {
        self.options = self.options.cancel(cancel);
        self
    }

    /// Sets whether to show the close button.
    pub fn close_button(mut self, close_button: bool) -> Self {
        self.options = self.options.close_button(close_button);
        self
    }

    /// Sets rich colors mode.
    pub fn rich_colors(mut self, rich_colors: bool) -> Self {
        self.options = self.options.rich_colors(rich_colors);
        self
    }

    /// Sets invert mode.
    pub fn invert(mut self, invert: bool) -> Self {
        self.options = self.options.invert(invert);
        self
    }

    /// Sets a per-toast position override.
    pub fn position(mut self, position: ToastPosition) -> Self {
        self.options = self.options.position(position);
        self
    }

    /// Adds this toast to the global state and returns its ID.
    ///
    /// This is called by the app's `update` function when processing
    /// a `SonnerToast` message.
    pub fn show(self) -> u64 {
        let title = self.title;
        let toast_type = self.options.toast_type;

        // Create a unit-typed options for the global state. Typed action
        // callbacks are discarded here because the global state can't
        // store them. For toasts with actions, use the `toast()` API
        // with a message variant that carries the callbacks.
        let unit_options: ToastOptions<()> = ToastOptions {
            toast_type,
            description: self.options.description,
            duration: self.options.duration,
            dismissible: self.options.dismissible,
            action: None,
            cancel: None,
            close_button: self.options.close_button,
            rich_colors: self.options.rich_colors,
            invert: self.options.invert,
            position: self.options.position,
        };

        state::create_typed_toast(title, unit_options)
    }

    /// Returns the toast title.
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Returns the toast options.
    pub fn options(&self) -> &ToastOptions<Message> {
        &self.options
    }

    /// Consumes self and returns the inner title and options.
    pub fn into_parts(self) -> (String, ToastOptions<Message>) {
        (self.title, self.options)
    }
}

/// Builder-first `Toaster` widget.
///
/// Place this once in your view tree (typically in the root layout).
/// It renders all active toasts as an overlay at the configured screen
/// position.
///
/// ```rust,no_run
/// use iced::Element;
/// use iced_shadcn_v2::{Toaster, Theme, toast, ToastPosition};
///
/// #[derive(Debug, Clone)]
/// enum Message {
///     ShowToast,
///     Toast(iced_shadcn_v2::SonnerToast<Message>),
/// }
///
/// fn view(theme: &Theme) -> Element<'_, Message> {
///     iced::widget::column![
///         Toaster::new(theme)
///             .position(ToastPosition::BottomRight)
///             .duration(4000)
///             .into(),
///         iced::widget::button("Show Toast")
///             .on_press(Message::ShowToast),
///     ]
///     .into()
/// }
/// ```
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct Toaster<'a, Message> {
    theme: &'a Theme,
    position: ToastPosition,
    duration_ms: u64,
    gap: f32,
    offset: f32,
    max_visible: usize,
    rich_colors: bool,
    invert: bool,
    close_button: bool,
    expand: bool,
}

impl<Message> fmt::Debug for Toaster<'_, Message> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Toaster")
            .field("theme", &self.theme)
            .field("position", &self.position)
            .field("duration_ms", &self.duration_ms)
            .field("gap", &self.gap)
            .field("offset", &self.offset)
            .field("max_visible", &self.max_visible)
            .field("rich_colors", &self.rich_colors)
            .field("invert", &self.invert)
            .field("close_button", &self.close_button)
            .field("expand", &self.expand)
            .finish()
    }
}

impl<'a, Message> Toaster<'a, Message> {
    /// Creates a new `Toaster` with default settings.
    ///
    /// Default position is `BottomRight`, duration is 4000ms, gap is
    /// 14px, offset is 24px, and max visible is 3.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            position: ToastPosition::BottomRight,
            duration_ms: style::DEFAULT_DURATION_MS,
            gap: style::TOAST_GAP,
            offset: style::TOAST_OFFSET,
            max_visible: style::MAX_VISIBLE_TOASTS,
            rich_colors: false,
            invert: false,
            close_button: false,
            expand: false,
        }
    }

    /// Sets the screen position for toasts.
    pub fn position(mut self, position: ToastPosition) -> Self {
        self.position = position;
        self
    }

    /// Sets the default auto-dismiss duration in milliseconds.
    pub fn duration(mut self, duration_ms: u64) -> Self {
        self.duration_ms = duration_ms;
        self
    }

    /// Sets the gap between toasts in pixels.
    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap.max(0.0);
        self
    }

    /// Sets the offset from screen edges in pixels.
    pub fn offset(mut self, offset: f32) -> Self {
        self.offset = offset.max(0.0);
        self
    }

    /// Sets the maximum number of visible toasts.
    pub fn max_visible(mut self, max_visible: usize) -> Self {
        self.max_visible = max_visible.max(1);
        self
    }

    /// Enables rich colors for all toasts (success, error, etc. get
    /// their own color scheme).
    pub fn rich_colors(mut self, rich_colors: bool) -> Self {
        self.rich_colors = rich_colors;
        self
    }

    /// Inverts all toasts (dark in light mode, light in dark mode).
    pub fn invert(mut self, invert: bool) -> Self {
        self.invert = invert;
        self
    }

    /// Shows the close button on all toasts.
    pub fn close_button(mut self, close_button: bool) -> Self {
        self.close_button = close_button;
        self
    }

    /// Expands all toasts to full size (no stacking scale effect).
    pub fn expand(mut self, expand: bool) -> Self {
        self.expand = expand;
        self
    }
}

impl<'a, Message> From<Toaster<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(toaster: Toaster<'a, Message>) -> Element<'a, Message> {
        Element::new(render::ToasterWidget {
            theme: toaster.theme,
            position: toaster.position,
            duration_ms: toaster.duration_ms,
            gap: toaster.gap,
            offset: toaster.offset,
            max_visible: toaster.max_visible,
            rich_colors: toaster.rich_colors,
            invert: toaster.invert,
            close_button: toaster.close_button,
            expand: toaster.expand,
            _marker: std::marker::PhantomData,
        })
    }
}
