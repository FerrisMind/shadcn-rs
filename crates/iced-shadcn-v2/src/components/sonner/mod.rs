//! Sonner-style toast notifications for iced.
//!
//! This component ports the shadcn-svelte wrapper around `svelte-sonner`:
//! typed toast kinds, descriptions, action/cancel buttons, close controls,
//! rich colors, inverted surfaces, per-toast positions, automatic dismissal,
//! hover/focus pauses, stacked layouts, and promise-style loading updates.
//! The queue is process-wide so `toast(...).show()` can be called from an iced
//! update handler or a library callback; the `Toaster` is mounted once in the
//! root view and renders that queue as an iced overlay.
//!
//! # Examples
//!
//! ```rust,no_run
//! use iced::Element;
//! use iced_shadcn_v2::{Theme, Toaster, ToastAction, toast};
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     ShowToast,
//!     Undo,
//! }
//!
//! fn view(theme: &Theme) -> Element<'_, Message> {
//!     let toaster = Toaster::new(theme).close_button(true);
//!     let _toast = toast("Event has been created")
//!         .description("Sunday, December 03, 2023 at 9:00 AM")
//!         .action(ToastAction::new("Undo", || Message::Undo));
//!
//!     // A real application calls `_toast.show()` from `update`, then keeps
//!     // this overlay in its root `stack!`.
//!     toaster.into()
//! }
//! ```

mod render;
mod state;
mod style;
mod types;

#[cfg(test)]
mod tests;

pub use style::ToastStyle;
pub use types::{SonnerToast, Toast, ToastAction, ToastCallback, ToastId, ToastOptions};
pub use types::{ToastPosition, ToastType};

use std::fmt;
use std::marker::PhantomData;

use crate::iced_compat::Element;
use crate::theme::Theme;

/// Creates a neutral toast builder.
///
/// The builder is inert until [`SonnerToast::show`] is called. Typed action
/// callbacks are accepted without a message type parameter on this function,
/// which keeps the common `toast("Saved")` call ergonomic.
pub fn toast(message: impl Into<String>) -> SonnerToast {
    SonnerToast::new(message)
}

/// Creates a toast builder with a caller-provided identifier.
pub fn toast_with_id(id: ToastId, message: impl Into<String>) -> SonnerToast {
    SonnerToast::with_id(id).title_text(message)
}

/// Shows a toast immediately with the requested type.
pub fn toast_immediate(message: impl Into<String>, toast_type: ToastType) -> ToastId {
    toast(message).toast_type(toast_type).show()
}

/// Shows a success toast immediately.
pub fn toast_success(message: impl Into<String>) -> ToastId {
    toast_immediate(message, ToastType::Success)
}

/// Shows an error toast immediately.
pub fn toast_error(message: impl Into<String>) -> ToastId {
    toast_immediate(message, ToastType::Error)
}

/// Shows a warning toast immediately.
pub fn toast_warning(message: impl Into<String>) -> ToastId {
    toast_immediate(message, ToastType::Warning)
}

/// Shows an informational toast immediately.
pub fn toast_info(message: impl Into<String>) -> ToastId {
    toast_immediate(message, ToastType::Info)
}

/// Shows a loading toast immediately.
pub fn toast_loading(message: impl Into<String>) -> ToastId {
    toast(message)
        .toast_type(ToastType::Loading)
        .duration(0)
        .show()
}

/// Starts a persistent loading toast that can later be resolved.
pub fn toast_promise(message: impl Into<String>) -> ToastPromise {
    let id = toast_loading(message);
    ToastPromise { id }
}

/// Updates an existing toast in place.
pub fn update_toast(id: impl Into<ToastId>, toast: SonnerToast) -> ToastId {
    state::update_toast(id.into(), toast)
}

/// Dismisses one toast and starts its exit animation.
pub fn dismiss_toast(id: impl Into<ToastId>) {
    let _ = state::dismiss_toast(id.into());
}

/// Dismisses every active toast.
pub fn dismiss_all_toasts() {
    state::dismiss_all_toasts();
}

/// Returns the number of toasts that have not started dismissing.
pub fn active_toast_count() -> usize {
    state::active_toast_count()
}

/// A handle for resolving a loading toast in a later update.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[must_use = "a ToastPromise should be resolved, updated, or dismissed"]
pub struct ToastPromise {
    id: ToastId,
}

impl ToastPromise {
    /// Returns the identifier of the loading toast.
    pub const fn id(self) -> ToastId {
        self.id
    }

    /// Replaces the loading toast with a success toast.
    pub fn success(self, message: impl Into<String>) -> ToastId {
        update_toast(self.id, toast(message).toast_type(ToastType::Success))
    }

    /// Replaces the loading toast with an error toast.
    pub fn error(self, message: impl Into<String>) -> ToastId {
        update_toast(self.id, toast(message).toast_type(ToastType::Error))
    }

    /// Replaces the loading toast with another loading message.
    pub fn loading(self, message: impl Into<String>) -> ToastId {
        update_toast(
            self.id,
            toast(message).toast_type(ToastType::Loading).duration(0),
        )
    }

    /// Dismisses the loading toast.
    pub fn dismiss(self) {
        dismiss_toast(self.id);
    }
}

/// The process-wide Toaster overlay configuration.
///
/// Mount one instance in the root `stack!` of an application. Toasts are
/// created with [`toast`] and shown from update handlers; action callbacks are
/// published as the application's `Message` type when the overlay is clicked.
#[must_use = "builders do nothing unless converted into an iced Element"]
pub struct Toaster<'a, Message> {
    theme: &'a Theme,
    position: ToastPosition,
    duration_ms: u64,
    gap: f32,
    offset: f32,
    width: f32,
    visible_toasts: usize,
    rich_colors: bool,
    invert: bool,
    close_button: bool,
    expand: bool,
    pause_on_hover: bool,
    pause_when_page_is_hidden: bool,
    animated: bool,
    style_override: Option<Box<dyn Fn(ToastStyle) -> ToastStyle + 'a>>,
    marker: PhantomData<fn() -> Message>,
}

impl<Message> fmt::Debug for Toaster<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Toaster")
            .field("position", &self.position)
            .field("duration_ms", &self.duration_ms)
            .field("gap", &self.gap)
            .field("offset", &self.offset)
            .field("width", &self.width)
            .field("visible_toasts", &self.visible_toasts)
            .field("rich_colors", &self.rich_colors)
            .field("invert", &self.invert)
            .field("close_button", &self.close_button)
            .field("expand", &self.expand)
            .field("pause_on_hover", &self.pause_on_hover)
            .field("pause_when_page_is_hidden", &self.pause_when_page_is_hidden)
            .field("animated", &self.animated)
            .field("style_override", &self.style_override.is_some())
            .finish_non_exhaustive()
    }
}

impl<'a, Message> Toaster<'a, Message> {
    /// Creates a Toaster with shadcn-svelte-compatible defaults.
    ///
    /// The default position is bottom-right, the default duration is 4,000
    /// milliseconds, the default width is 356 px, and up to three toasts are
    /// visible at once.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            position: ToastPosition::BottomRight,
            duration_ms: 4_000,
            gap: 14.0,
            offset: 24.0,
            width: 356.0,
            visible_toasts: 3,
            rich_colors: false,
            invert: false,
            close_button: false,
            expand: false,
            pause_on_hover: true,
            pause_when_page_is_hidden: false,
            animated: true,
            style_override: None,
            marker: PhantomData,
        }
    }

    /// Sets the default stack position.
    #[must_use = "builder methods return the modified Toaster"]
    pub fn position(mut self, position: ToastPosition) -> Self {
        self.position = position;
        self
    }

    /// Sets the default auto-dismiss duration in milliseconds. `0` disables
    /// automatic dismissal for toasts without an explicit duration.
    #[must_use = "builder methods return the modified Toaster"]
    pub fn duration(mut self, duration_ms: u64) -> Self {
        self.duration_ms = duration_ms;
        self
    }

    /// Sets the gap between expanded toasts in pixels.
    #[must_use = "builder methods return the modified Toaster"]
    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = finite_non_negative(gap);
        self
    }

    /// Sets the distance from the window edges in pixels.
    #[must_use = "builder methods return the modified Toaster"]
    pub fn offset(mut self, offset: f32) -> Self {
        self.offset = finite_non_negative(offset);
        self
    }

    /// Sets the preferred toast width in pixels.
    #[must_use = "builder methods return the modified Toaster"]
    pub fn width(mut self, width: f32) -> Self {
        self.width = finite_non_negative(width).max(180.0);
        self
    }

    /// Sets the maximum number of visible toasts per position.
    #[must_use = "builder methods return the modified Toaster"]
    pub fn visible_toasts(mut self, visible_toasts: usize) -> Self {
        self.visible_toasts = visible_toasts.max(1);
        self
    }

    /// Backwards-compatible alias for [`Self::visible_toasts`].
    #[must_use = "builder methods return the modified Toaster"]
    pub fn max_visible(self, max_visible: usize) -> Self {
        self.visible_toasts(max_visible)
    }

    /// Enables rich semantic colors for success, info, warning, and error
    /// toasts.
    #[must_use = "builder methods return the modified Toaster"]
    pub fn rich_colors(mut self, rich_colors: bool) -> Self {
        self.rich_colors = rich_colors;
        self
    }

    /// Inverts all toast surfaces against the current theme.
    #[must_use = "builder methods return the modified Toaster"]
    pub fn invert(mut self, invert: bool) -> Self {
        self.invert = invert;
        self
    }

    /// Shows a close button on dismissible toasts.
    #[must_use = "builder methods return the modified Toaster"]
    pub fn close_button(mut self, close_button: bool) -> Self {
        self.close_button = close_button;
        self
    }

    /// Expands every visible stack instead of using the compact Sonner stack.
    #[must_use = "builder methods return the modified Toaster"]
    pub fn expand(mut self, expand: bool) -> Self {
        self.expand = expand;
        self
    }

    /// Pauses auto-dismiss timers while the pointer is over a toast.
    #[must_use = "builder methods return the modified Toaster"]
    pub fn pause_on_hover(mut self, pause_on_hover: bool) -> Self {
        self.pause_on_hover = pause_on_hover;
        self
    }

    /// Pauses auto-dismiss timers while the application window is unfocused.
    #[must_use = "builder methods return the modified Toaster"]
    pub fn pause_when_page_is_hidden(mut self, pause: bool) -> Self {
        self.pause_when_page_is_hidden = pause;
        self
    }

    /// Enables or disables enter and exit motion.
    #[must_use = "builder methods return the modified Toaster"]
    pub fn animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }

    /// Applies a resolved style patch to every toast surface.
    #[must_use = "builder methods return the modified Toaster"]
    pub fn style_override(
        mut self,
        style_override: impl Fn(ToastStyle) -> ToastStyle + 'a,
    ) -> Self {
        self.style_override = Some(Box::new(style_override));
        self
    }
}

impl<'a, Message> From<Toaster<'a, Message>> for Element<'a, Message>
where
    Message: 'a + 'static,
{
    fn from(toaster: Toaster<'a, Message>) -> Self {
        Element::new(render::ToasterWidget {
            theme: toaster.theme,
            position: toaster.position,
            duration_ms: toaster.duration_ms,
            gap: toaster.gap,
            offset: toaster.offset,
            width: toaster.width,
            visible_toasts: toaster.visible_toasts,
            rich_colors: toaster.rich_colors,
            invert: toaster.invert,
            close_button: toaster.close_button,
            expand: toaster.expand,
            pause_on_hover: toaster.pause_on_hover,
            pause_when_page_is_hidden: toaster.pause_when_page_is_hidden,
            animated: toaster.animated,
            style_override: toaster.style_override,
            marker: PhantomData,
        })
    }
}

fn finite_non_negative(value: f32) -> f32 {
    if value.is_finite() {
        value.max(0.0)
    } else {
        0.0
    }
}
