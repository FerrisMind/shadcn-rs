//! Configuration types for the sonner toast component.

use std::fmt;
use std::sync::atomic::{AtomicU64, Ordering};

/// Global monotonic counter for generating unique toast IDs.
static TOAST_COUNTER: AtomicU64 = AtomicU64::new(1);

/// Generates the next unique toast ID.
pub(super) fn next_toast_id() -> u64 {
    TOAST_COUNTER.fetch_add(1, Ordering::Relaxed)
}

/// Type of toast notification.
///
/// ```rust
/// use iced_shadcn_v2::ToastType;
///
/// assert_eq!(ToastType::default(), ToastType::Default);
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ToastType {
    /// Default toast without a specific type icon.
    #[default]
    Default,
    /// Success toast with a checkmark icon.
    Success,
    /// Informational toast with an info icon.
    Info,
    /// Warning toast with a triangle-alert icon.
    Warning,
    /// Error toast with an octagon-x icon.
    Error,
    /// Loading toast with a spinner icon.
    Loading,
}

/// Position of the toast container on screen.
///
/// Matches shadcn-svelte `Toaster` `position` prop.
///
/// ```rust
/// use iced_shadcn_v2::ToastPosition;
///
/// assert_eq!(ToastPosition::default(), ToastPosition::BottomRight);
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ToastPosition {
    /// Bottom-right corner (default).
    #[default]
    BottomRight,
    /// Bottom-left corner.
    BottomLeft,
    /// Bottom center.
    BottomCenter,
    /// Top-right corner.
    TopRight,
    /// Top-left corner.
    TopLeft,
    /// Top center.
    TopCenter,
}

impl ToastPosition {
    /// Whether this position is on the top edge of the screen.
    pub const fn is_top(self) -> bool {
        matches!(self, Self::TopRight | Self::TopLeft | Self::TopCenter)
    }

    /// Whether this position is on the bottom edge of the screen.
    pub const fn is_bottom(self) -> bool {
        matches!(
            self,
            Self::BottomRight | Self::BottomLeft | Self::BottomCenter
        )
    }

    /// Whether this position is on the left edge of the screen.
    pub const fn is_left(self) -> bool {
        matches!(self, Self::TopLeft | Self::BottomLeft)
    }

    /// Whether this position is on the right edge of the screen.
    pub const fn is_right(self) -> bool {
        matches!(self, Self::TopRight | Self::BottomRight)
    }

    /// Whether this position is horizontally centered.
    pub const fn is_center_x(self) -> bool {
        matches!(self, Self::TopCenter | Self::BottomCenter)
    }
}

/// Action button for a toast notification.
///
/// An action has a label and an optional callback that produces a `Message`
/// when the button is clicked.
pub struct ToastAction<Message> {
    /// Button label.
    pub label: String,
    /// Optional callback invoked when the action button is pressed.
    ///
    /// The callback receives a reference to the action and returns an optional
    /// `Message`. Return `None` to dismiss the toast without emitting a
    /// message.
    pub on_click: Option<Box<dyn Fn(&Self) -> Option<Message> + Send + Sync>>,
}

impl<Message> fmt::Debug for ToastAction<Message> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ToastAction")
            .field("label", &self.label)
            .field("on_click", &self.on_click.is_some())
            .finish()
    }
}

impl<Message> ToastAction<Message> {
    /// Creates a new action with a label and callback.
    pub fn new(
        label: impl Into<String>,
        on_click: impl Fn(&Self) -> Option<Message> + Send + Sync + 'static,
    ) -> Self {
        Self {
            label: label.into(),
            on_click: Some(Box::new(on_click)),
        }
    }

    /// Creates an action with only a label (no callback).
    pub fn label(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            on_click: None,
        }
    }
}

/// Options for creating a toast notification.
pub struct ToastOptions<Message> {
    /// Toast type (success, error, etc.).
    pub toast_type: ToastType,
    /// Description text below the title.
    pub description: Option<String>,
    /// Duration in milliseconds before auto-dismiss. `None` uses the
    /// toaster's default.
    pub duration: Option<u64>,
    /// Whether the toast can be dismissed by the user.
    pub dismissible: bool,
    /// Action button.
    pub action: Option<ToastAction<Message>>,
    /// Cancel button.
    pub cancel: Option<ToastAction<Message>>,
    /// Whether to show the close button.
    pub close_button: bool,
    /// Rich colors mode for this toast.
    pub rich_colors: bool,
    /// Invert colors for this toast.
    pub invert: bool,
    /// Per-toast position override.
    pub position: Option<ToastPosition>,
}

impl<Message> Default for ToastOptions<Message> {
    fn default() -> Self {
        Self {
            toast_type: ToastType::Default,
            description: None,
            duration: None,
            dismissible: true,
            action: None,
            cancel: None,
            close_button: false,
            rich_colors: false,
            invert: false,
            position: None,
        }
    }
}

impl<Message> ToastOptions<Message> {
    /// Creates default options for the given toast type.
    pub fn new(toast_type: ToastType) -> Self {
        Self {
            toast_type,
            ..Default::default()
        }
    }

    /// Sets the description text.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets the auto-dismiss duration in milliseconds.
    pub fn duration(mut self, duration: u64) -> Self {
        self.duration = Some(duration);
        self
    }

    /// Sets whether the toast is dismissible.
    pub fn dismissible(mut self, dismissible: bool) -> Self {
        self.dismissible = dismissible;
        self
    }

    /// Sets the action button.
    pub fn action(mut self, action: ToastAction<Message>) -> Self {
        self.action = Some(action);
        self
    }

    /// Sets the cancel button.
    pub fn cancel(mut self, cancel: ToastAction<Message>) -> Self {
        self.cancel = Some(cancel);
        self
    }

    /// Sets whether to show the close button.
    pub fn close_button(mut self, close_button: bool) -> Self {
        self.close_button = close_button;
        self
    }

    /// Sets rich colors mode.
    pub fn rich_colors(mut self, rich_colors: bool) -> Self {
        self.rich_colors = rich_colors;
        self
    }

    /// Sets invert mode.
    pub fn invert(mut self, invert: bool) -> Self {
        self.invert = invert;
        self
    }

    /// Sets a per-toast position override.
    pub fn position(mut self, position: ToastPosition) -> Self {
        self.position = Some(position);
        self
    }
}

/// Toast lifetime: the raw data stored in the global state.
///
/// Actions are stored as type-erased closures using `unsafe` transmute.
/// This is safe because:
/// - The `toast()` function and the `Toaster` widget always use the same
///   `Message` type (guaranteed by the app's type system).
/// - The transmuted closure is only called in the `Toaster` widget's update
///   method, which knows the correct `Message` type.
pub(super) struct RawToast {
    pub id: u64,
    pub title: String,
    pub toast_type: ToastType,
    pub description: Option<String>,
    pub duration: Option<u64>,
    pub dismissible: bool,
    pub close_button: bool,
    pub rich_colors: bool,
    pub invert: bool,
    pub position: Option<ToastPosition>,
    pub action_label: Option<String>,
    pub cancel_label: Option<String>,
    /// Type-erased action callback. Only safe to call from code that knows
    /// the concrete `Message` type.
    pub(super) action_cb: Option<Box<dyn Fn() -> Option<()> + Send + Sync>>,
    /// Type-erased cancel callback. Only safe to call from code that knows
    /// the concrete `Message` type.
    pub(super) cancel_cb: Option<Box<dyn Fn() -> Option<()> + Send + Sync>>,
    /// Timestamp (monotonic ms) when the toast was created, for timer
    /// management.
    pub created_at_ms: u64,
    /// Whether the toast has been dismissed (triggers exit animation).
    pub dismissed: bool,
    /// Whether the toast is being removed (after exit animation).
    pub removing: bool,
}

impl fmt::Debug for RawToast {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RawToast")
            .field("id", &self.id)
            .field("title", &self.title)
            .field("toast_type", &self.toast_type)
            .field("description", &self.description)
            .field("duration", &self.duration)
            .field("dismissible", &self.dismissible)
            .field("close_button", &self.close_button)
            .field("rich_colors", &self.rich_colors)
            .field("invert", &self.invert)
            .field("position", &self.position)
            .field("action_label", &self.action_label)
            .field("cancel_label", &self.cancel_label)
            .field("action_cb", &self.action_cb.is_some())
            .field("cancel_cb", &self.cancel_cb.is_some())
            .field("created_at_ms", &self.created_at_ms)
            .field("dismissed", &self.dismissed)
            .field("removing", &self.removing)
            .finish()
    }
}
