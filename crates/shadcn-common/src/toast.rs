//! Framework-agnostic Sonner toast vocabulary shared by the iced and egui
//! backends.
//!
//! These value types model *what* a toast is — its semantic [`ToastType`],
//! its [`ToastPosition`], and a stable [`ToastId`] newtype — with no
//! dependency on any GUI framework. Each backend owns its own queue, rendering,
//! and message-publishing mechanism on top of this shared model, so iced and
//! egui stay decoupled while speaking the same toast language.
//!
//! The serialisation derives are gated behind the optional `serde` Cargo
//! feature, matching the iced-shadcn-v2 `serde` feature which forwards to it.

use std::fmt;

/// A stable identifier assigned to a toast.
///
/// The identifier is useful for updating or dismissing a toast after it has
/// been shown. It is intentionally a newtype so toast identifiers cannot be
/// accidentally mixed with unrelated integers.
///
/// Backends mint identifiers from their own monotonic counter (see, for
/// example, `next_toast_id` in iced-shadcn-v2) rather than from this type,
/// so independent iced/egui instances never share a single global counter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[must_use = "a ToastId identifies a toast; keep it to update or dismiss that toast"]
pub struct ToastId(u64);

impl ToastId {
    /// Returns the numeric representation of this identifier.
    pub const fn as_u64(self) -> u64 {
        self.0
    }

    /// Returns whether this identifier is the reserved zero value.
    ///
    /// Zero is never assigned by a backend counter; it is conventionally used
    /// to mean "no toast".
    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }
}

impl From<u64> for ToastId {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<ToastId> for u64 {
    fn from(value: ToastId) -> Self {
        value.0
    }
}

impl fmt::Display for ToastId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

/// The visual and semantic kind of a toast notification.
///
/// The default kind is [`ToastType::Default`]: a neutral notification with no
/// status color. `Success` / `Info` / `Warning` / `Error` add a status icon,
/// and `Loading` renders an indeterminate spinner and never auto-dismisses
/// unless the backend pairs it with a finite duration.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ToastType {
    /// A neutral notification without a status color.
    #[default]
    Default,
    /// A successful operation.
    Success,
    /// Informational content.
    Info,
    /// A caution or recoverable problem.
    Warning,
    /// A failed or destructive operation.
    Error,
    /// An operation that is still in progress.
    Loading,
}

impl ToastType {
    /// Returns the stable kebab-case name used by the web Sonner API.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Success => "success",
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Error => "error",
            Self::Loading => "loading",
        }
    }

    /// Returns whether this kind represents an in-progress operation.
    pub const fn is_loading(self) -> bool {
        matches!(self, Self::Loading)
    }
}

/// A screen edge/corner where the toast stack is rendered.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ToastPosition {
    /// Bottom-right corner, the default position.
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
    /// Returns the stable kebab-case name used by the web Sonner API.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BottomRight => "bottom-right",
            Self::BottomLeft => "bottom-left",
            Self::BottomCenter => "bottom-center",
            Self::TopRight => "top-right",
            Self::TopLeft => "top-left",
            Self::TopCenter => "top-center",
        }
    }

    /// Returns the stable kebab-case name; kept for symmetry with [`as_str`].
    ///
    /// [`as_str`]: ToastPosition::as_str
    pub const fn as_kebab(self) -> &'static str {
        self.as_str()
    }

    /// Returns whether this position is anchored to the top edge.
    pub const fn is_top(self) -> bool {
        matches!(self, Self::TopRight | Self::TopLeft | Self::TopCenter)
    }

    /// Returns whether this position is anchored to the bottom edge.
    pub const fn is_bottom(self) -> bool {
        matches!(
            self,
            Self::BottomRight | Self::BottomLeft | Self::BottomCenter
        )
    }

    /// Returns whether this position is anchored to the left edge.
    pub const fn is_left(self) -> bool {
        matches!(self, Self::TopLeft | Self::BottomLeft)
    }

    /// Returns whether this position is anchored to the right edge.
    pub const fn is_right(self) -> bool {
        matches!(self, Self::TopRight | Self::BottomRight)
    }

    /// Returns whether this position is horizontally centered.
    pub const fn is_center_x(self) -> bool {
        matches!(self, Self::TopCenter | Self::BottomCenter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toast_type_defaults_and_names_match_sonner() {
        assert_eq!(ToastType::default(), ToastType::Default);
        assert_eq!(ToastType::Default.as_str(), "default");
        assert_eq!(ToastType::Success.as_str(), "success");
        assert_eq!(ToastType::Loading.as_str(), "loading");
        assert!(ToastType::Loading.is_loading());
        assert!(!ToastType::Default.is_loading());
    }

    #[test]
    fn toast_position_anchor_predicates() {
        assert!(ToastPosition::default().is_bottom());
        assert!(ToastPosition::default().is_right());
        assert!(ToastPosition::TopLeft.is_top() && ToastPosition::TopLeft.is_left());
        assert!(ToastPosition::BottomCenter.is_center_x());
        assert_eq!(ToastPosition::TopRight.as_kebab(), "top-right");
    }

    #[test]
    fn toast_id_round_trips_through_u64() {
        let id = ToastId::from(7);
        assert_eq!(id.as_u64(), 7);
        assert_eq!(u64::from(id), 7);
        assert!(!id.is_zero());
        assert!(ToastId::from(0).is_zero());
        assert_eq!(format!("{id}"), "7");
    }
}
