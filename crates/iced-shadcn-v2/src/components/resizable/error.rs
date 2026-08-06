//! Errors produced while building a resizable pane group.

use std::fmt;

/// Error returned when a resizable padding value cannot be represented by iced.
///
/// ```rust
/// use iced_shadcn_v2::{Padding, ResizablePaneGroup, Spacing, Theme};
///
/// let theme = Theme::light();
/// let result =
///     ResizablePaneGroup::<()>::new(&theme).padding(Padding::all(Spacing::Auto));
/// assert!(result.is_err());
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResizableBuildError {
    /// A custom-property padding variable has no value that iced can resolve.
    UnsupportedPaddingVariable {
        /// Name of the unsupported custom property.
        name: &'static str,
    },
    /// The CSS-like `auto` padding value has no iced equivalent.
    UnsupportedPaddingAuto,
    /// The pane group must contain at least one pane.
    EmptyPaneGroup,
    /// Handles must sit between panes; the slot sequence is invalid.
    InvalidSlotSequence,
}

impl fmt::Display for ResizableBuildError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedPaddingVariable { name } => write!(
                formatter,
                "padding variable `{name}` is not supported by iced-shadcn-v2::Resizable"
            ),
            Self::UnsupportedPaddingAuto => formatter
                .write_str("padding value `auto` is not supported by iced-shadcn-v2::Resizable"),
            Self::EmptyPaneGroup => {
                formatter.write_str("a resizable pane group requires at least one pane")
            }
            Self::InvalidSlotSequence => formatter.write_str(
                "resizable slots must alternate pane, handle, pane, … with a pane on each end",
            ),
        }
    }
}

impl std::error::Error for ResizableBuildError {}
