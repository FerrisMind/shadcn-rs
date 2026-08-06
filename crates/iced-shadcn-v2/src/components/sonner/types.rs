//! Public configuration types for the Sonner toast component.

use std::any::Any;
use std::fmt;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

// The framework-agnostic toast vocabulary — toast kinds, positions, and the
// identifier newtype — is shared with egui-shadcn through
// `shadcn_common::toast`, so both backends model the same Sonner concepts.
// Only the iced-owned pieces (the callback wrappers, the `SonnerToast`
// builder, the queue) remain in this module.
pub use shadcn_common::toast::{ToastId, ToastPosition, ToastType};

/// Monotonic counter backing [`next_toast_id`].
///
/// It stays in iced (rather than `shadcn-common`) so independent iced/egui
/// instances never share a single global toast counter.
static NEXT_TOAST_ID: AtomicU64 = AtomicU64::new(1);

/// Allocates the next process-wide toast identifier.
fn next_toast_id() -> ToastId {
    ToastId::from(NEXT_TOAST_ID.fetch_add(1, Ordering::Relaxed))
}

/// A callback that publishes an application message when it is invoked.
///
/// Callbacks are type-erased only at the storage boundary. The `Toaster`
/// overlay restores the concrete message with `Any::downcast`, and silently
/// ignores a callback created for a different application message type. This
/// keeps the process-wide imperative API sound without using `unsafe`.
pub struct ToastCallback {
    callback: RawCallback,
}

impl fmt::Debug for ToastCallback {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ToastCallback")
            .field("configured", &true)
            .finish()
    }
}

impl Clone for ToastCallback {
    fn clone(&self) -> Self {
        Self {
            callback: Arc::clone(&self.callback),
        }
    }
}

impl ToastCallback {
    /// Creates a callback that returns an application message.
    pub fn new<Message>(callback: impl Fn() -> Message + Send + Sync + 'static) -> Self
    where
        Message: Any + 'static,
    {
        Self {
            callback: Arc::new(move || Some(Box::new(callback()) as Box<dyn Any>)),
        }
    }

    pub(super) fn callback(&self) -> RawCallback {
        Arc::clone(&self.callback)
    }
}

/// An optional action button rendered inside a toast.
pub struct ToastAction {
    label: String,
    callback: Option<RawCallback>,
}

impl fmt::Debug for ToastAction {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ToastAction")
            .field("label", &self.label)
            .field("callback", &self.callback.is_some())
            .finish()
    }
}

impl Clone for ToastAction {
    fn clone(&self) -> Self {
        Self {
            label: self.label.clone(),
            callback: self.callback.as_ref().map(Arc::clone),
        }
    }
}

impl ToastAction {
    /// Creates an action button with a typed callback.
    ///
    /// The callback is invoked once per click and its returned message is
    /// published by the [`crate::Toaster`] overlay.
    pub fn new<Message>(
        label: impl Into<String>,
        callback: impl Fn() -> Message + Send + Sync + 'static,
    ) -> Self
    where
        Message: Any + 'static,
    {
        Self {
            label: label.into(),
            callback: Some(Arc::new(move || Some(Box::new(callback()) as Box<dyn Any>))),
        }
    }

    /// Creates a visible action without a callback.
    pub fn label(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            callback: None,
        }
    }

    /// Returns the action label.
    pub fn label_text(&self) -> &str {
        &self.label
    }

    /// Returns whether this action has a callback.
    pub const fn has_callback(&self) -> bool {
        self.callback.is_some()
    }

    pub(super) fn callback(&self) -> Option<RawCallback> {
        self.callback.as_ref().map(Arc::clone)
    }
}

/// Options shared by a toast's type-specific helper and its builder methods.
#[derive(Debug, Clone)]
pub struct ToastOptions {
    pub(super) id: Option<ToastId>,
    pub(super) toast_type: ToastType,
    pub(super) description: Option<String>,
    pub(super) duration_ms: Option<u64>,
    pub(super) dismissible: bool,
    pub(super) action: Option<ToastAction>,
    pub(super) cancel: Option<ToastAction>,
    pub(super) close_button: bool,
    pub(super) rich_colors: bool,
    pub(super) invert: bool,
    pub(super) position: Option<ToastPosition>,
    pub(super) important: bool,
    pub(super) on_dismiss: Option<ToastCallback>,
    pub(super) on_auto_close: Option<ToastCallback>,
}

impl Default for ToastOptions {
    fn default() -> Self {
        Self {
            id: None,
            toast_type: ToastType::Default,
            description: None,
            duration_ms: None,
            dismissible: true,
            action: None,
            cancel: None,
            close_button: false,
            rich_colors: false,
            invert: false,
            position: None,
            important: false,
            on_dismiss: None,
            on_auto_close: None,
        }
    }
}

impl ToastOptions {
    /// Creates default options for a specific toast type.
    pub fn new(toast_type: ToastType) -> Self {
        Self {
            toast_type,
            ..Self::default()
        }
    }

    /// Sets the optional stable identifier used for updates.
    #[must_use = "builder methods return the modified options"]
    pub fn id(mut self, id: ToastId) -> Self {
        self.id = Some(id);
        self
    }

    /// Sets the secondary description shown below the title.
    #[must_use = "builder methods return the modified options"]
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets the toast's semantic and visual type.
    #[must_use = "builder methods return the modified options"]
    pub fn toast_type(mut self, toast_type: ToastType) -> Self {
        self.toast_type = toast_type;
        self
    }

    /// Sets the auto-dismiss duration in milliseconds.
    ///
    /// A value of `0` creates a persistent toast. The default `None` uses the
    /// duration configured on [`crate::Toaster`].
    #[must_use = "builder methods return the modified options"]
    pub fn duration(mut self, duration_ms: u64) -> Self {
        self.duration_ms = Some(duration_ms);
        self
    }

    /// Makes the toast dismissible or persistent.
    #[must_use = "builder methods return the modified options"]
    pub fn dismissible(mut self, dismissible: bool) -> Self {
        self.dismissible = dismissible;
        self
    }

    /// Adds an action button.
    #[must_use = "builder methods return the modified options"]
    pub fn action(mut self, action: ToastAction) -> Self {
        self.action = Some(action);
        self
    }

    /// Adds a secondary cancel button.
    #[must_use = "builder methods return the modified options"]
    pub fn cancel(mut self, cancel: ToastAction) -> Self {
        self.cancel = Some(cancel);
        self
    }

    /// Shows a close button on this toast.
    #[must_use = "builder methods return the modified options"]
    pub fn close_button(mut self, close_button: bool) -> Self {
        self.close_button = close_button;
        self
    }

    /// Enables the type-specific rich color palette.
    #[must_use = "builder methods return the modified options"]
    pub fn rich_colors(mut self, rich_colors: bool) -> Self {
        self.rich_colors = rich_colors;
        self
    }

    /// Inverts this toast against the current theme.
    #[must_use = "builder methods return the modified options"]
    pub fn invert(mut self, invert: bool) -> Self {
        self.invert = invert;
        self
    }

    /// Overrides the stack position for this toast.
    #[must_use = "builder methods return the modified options"]
    pub fn position(mut self, position: ToastPosition) -> Self {
        self.position = Some(position);
        self
    }

    /// Keeps this toast visible even when the stack reaches its limit.
    #[must_use = "builder methods return the modified options"]
    pub fn important(mut self, important: bool) -> Self {
        self.important = important;
        self
    }

    /// Publishes a message when the toast is dismissed manually.
    #[must_use = "builder methods return the modified options"]
    pub fn on_dismiss(mut self, callback: ToastCallback) -> Self {
        self.on_dismiss = Some(callback);
        self
    }

    /// Publishes a message when the toast expires automatically.
    #[must_use = "builder methods return the modified options"]
    pub fn on_auto_close(mut self, callback: ToastCallback) -> Self {
        self.on_auto_close = Some(callback);
        self
    }

    /// Returns the caller-provided identifier, if one was configured.
    pub const fn id_value(&self) -> Option<ToastId> {
        self.id
    }

    /// Returns the configured toast type.
    pub const fn toast_type_value(&self) -> ToastType {
        self.toast_type
    }

    /// Returns the optional description.
    pub fn description_text(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Returns the optional auto-dismiss duration in milliseconds.
    pub const fn duration_ms(&self) -> Option<u64> {
        self.duration_ms
    }

    /// Returns whether the toast can be dismissed by the user.
    pub const fn is_dismissible(&self) -> bool {
        self.dismissible
    }

    /// Returns the configured action, if any.
    pub fn action_ref(&self) -> Option<&ToastAction> {
        self.action.as_ref()
    }

    /// Returns the configured cancel action, if any.
    pub fn cancel_ref(&self) -> Option<&ToastAction> {
        self.cancel.as_ref()
    }

    /// Returns whether this toast requests a close button.
    pub const fn has_close_button(&self) -> bool {
        self.close_button
    }

    /// Returns whether rich semantic colors are enabled.
    pub const fn uses_rich_colors(&self) -> bool {
        self.rich_colors
    }

    /// Returns whether the toast uses inverted colors.
    pub const fn is_inverted(&self) -> bool {
        self.invert
    }

    /// Returns the optional per-toast position override.
    pub const fn position_override(&self) -> Option<ToastPosition> {
        self.position
    }

    /// Returns whether this toast bypasses the visible-stack limit.
    pub const fn is_important(&self) -> bool {
        self.important
    }
}

/// A toast notification built with the Sonner-style fluent API.
#[derive(Clone)]
#[must_use = "a toast does nothing until it is shown or passed to a Toaster"]
pub struct SonnerToast {
    pub(super) id: ToastId,
    pub(super) title: String,
    pub(super) options: ToastOptions,
}

impl fmt::Debug for SonnerToast {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SonnerToast")
            .field("id", &self.id)
            .field("title", &self.title)
            .field("options", &self.options)
            .finish()
    }
}

impl SonnerToast {
    /// Creates a toast with a generated identifier.
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            id: next_toast_id(),
            title: title.into(),
            options: ToastOptions::default(),
        }
    }

    /// Creates a toast using a caller-provided identifier.
    pub fn with_id(id: ToastId) -> Self {
        Self {
            id,
            title: String::new(),
            options: ToastOptions::default().id(id),
        }
    }

    /// Returns this toast's identifier.
    pub const fn id(&self) -> ToastId {
        self.id
    }

    /// Returns the title text.
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Returns the configured options.
    pub const fn options(&self) -> &ToastOptions {
        &self.options
    }

    /// Sets the title text.
    #[must_use = "builder methods return the modified toast"]
    pub fn title_text(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// Sets the secondary description.
    #[must_use = "builder methods return the modified toast"]
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.options = self.options.description(description);
        self
    }

    /// Sets the toast type.
    #[must_use = "builder methods return the modified toast"]
    pub fn toast_type(mut self, toast_type: ToastType) -> Self {
        self.options = self.options.toast_type(toast_type);
        self
    }

    /// Sets an auto-dismiss duration in milliseconds; `0` is persistent.
    #[must_use = "builder methods return the modified toast"]
    pub fn duration(mut self, duration_ms: u64) -> Self {
        self.options = self.options.duration(duration_ms);
        self
    }

    /// Sets whether the toast can be dismissed by the user.
    #[must_use = "builder methods return the modified toast"]
    pub fn dismissible(mut self, dismissible: bool) -> Self {
        self.options = self.options.dismissible(dismissible);
        self
    }

    /// Adds an action button.
    #[must_use = "builder methods return the modified toast"]
    pub fn action(mut self, action: ToastAction) -> Self {
        self.options = self.options.action(action);
        self
    }

    /// Adds a cancel button.
    #[must_use = "builder methods return the modified toast"]
    pub fn cancel(mut self, cancel: ToastAction) -> Self {
        self.options = self.options.cancel(cancel);
        self
    }

    /// Shows or hides the close button for this toast.
    #[must_use = "builder methods return the modified toast"]
    pub fn close_button(mut self, close_button: bool) -> Self {
        self.options = self.options.close_button(close_button);
        self
    }

    /// Enables or disables rich colors for this toast.
    #[must_use = "builder methods return the modified toast"]
    pub fn rich_colors(mut self, rich_colors: bool) -> Self {
        self.options = self.options.rich_colors(rich_colors);
        self
    }

    /// Inverts this toast against the current theme.
    #[must_use = "builder methods return the modified toast"]
    pub fn invert(mut self, invert: bool) -> Self {
        self.options = self.options.invert(invert);
        self
    }

    /// Overrides the stack position for this toast.
    #[must_use = "builder methods return the modified toast"]
    pub fn position(mut self, position: ToastPosition) -> Self {
        self.options = self.options.position(position);
        self
    }

    /// Marks this toast as important when the visible stack is full.
    #[must_use = "builder methods return the modified toast"]
    pub fn important(mut self, important: bool) -> Self {
        self.options = self.options.important(important);
        self
    }

    /// Adds a callback for manual dismissal.
    #[must_use = "builder methods return the modified toast"]
    pub fn on_dismiss(mut self, callback: ToastCallback) -> Self {
        self.options = self.options.on_dismiss(callback);
        self
    }

    /// Adds a callback for automatic expiration.
    #[must_use = "builder methods return the modified toast"]
    pub fn on_auto_close(mut self, callback: ToastCallback) -> Self {
        self.options = self.options.on_auto_close(callback);
        self
    }

    /// Shows this toast in the process-wide Sonner queue.
    pub fn show(self) -> ToastId {
        crate::components::sonner::state::create_toast(self)
    }

    pub(super) fn into_parts(self) -> (ToastId, String, ToastOptions) {
        (self.id, self.title, self.options)
    }
}

/// The conventional short name for [`SonnerToast`].
pub type Toast = SonnerToast;

/// Type-erased callback storage used only inside the process-wide queue.
pub(super) type RawCallback = Arc<dyn Fn() -> Option<Box<dyn Any>> + Send + Sync + 'static>;
