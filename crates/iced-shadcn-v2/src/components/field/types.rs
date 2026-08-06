//! Public configuration types for the field component.

use std::fmt;

/// Layout orientation of a [`super::Field`].
///
/// `Responsive` uses the available width during layout and switches to the
/// horizontal arrangement at the field's configured breakpoint.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FieldOrientation {
    /// Stack the label, control, and supporting text vertically.
    #[default]
    Vertical,
    /// Place the field's content in a row.
    Horizontal,
    /// Use a vertical layout below the responsive breakpoint and a horizontal
    /// layout at or above it.
    Responsive,
}

impl FieldOrientation {
    /// Returns `true` when this orientation can change during layout.
    pub const fn is_responsive(self) -> bool {
        matches!(self, Self::Responsive)
    }
}

/// Typography treatment for a [`super::FieldLegend`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FieldLegendVariant {
    /// Section legend (`text-base`).
    #[default]
    Legend,
    /// Compact label legend (`text-sm`).
    Label,
}

/// One item accepted by [`super::FieldError::errors`].
///
/// An item may intentionally have no message. This mirrors the Svelte
/// component's `{ message?: string }` shape: empty items are ignored when the
/// error list is rendered.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FieldErrorItem {
    message: Option<String>,
}

impl FieldErrorItem {
    /// Creates an error item with a message.
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: Some(message.into()),
        }
    }

    /// Creates an error item without a message.
    pub const fn empty() -> Self {
        Self { message: None }
    }

    /// Returns the message, if this item has one.
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}

impl From<String> for FieldErrorItem {
    fn from(message: String) -> Self {
        Self::new(message)
    }
}

impl From<&str> for FieldErrorItem {
    fn from(message: &str) -> Self {
        Self::new(message)
    }
}

impl fmt::Display for FieldErrorItem {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(message) = self.message() {
            formatter.write_str(message)
        } else {
            formatter.write_str("<empty field error>")
        }
    }
}
