//! Errors produced while building a kbd.

use std::fmt;

/// Error returned when a kbd padding value cannot be represented by iced.
///
/// ```rust
/// use iced_shadcn_v2::{Kbd, Padding, Spacing, Theme};
///
/// let theme = Theme::light();
/// let result = Kbd::<()>::text("B", &theme).padding(Padding::all(Spacing::Auto));
/// assert!(result.is_err());
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KbdBuildError {
    /// A custom-property padding variable has no value that iced can resolve.
    UnsupportedPaddingVariable {
        /// Name of the unsupported custom property.
        name: &'static str,
    },
    /// The CSS-like `auto` padding value has no iced equivalent.
    UnsupportedPaddingAuto,
}

impl fmt::Display for KbdBuildError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedPaddingVariable { name } => write!(
                formatter,
                "padding variable `{name}` is not supported by iced-shadcn-v2::Kbd"
            ),
            Self::UnsupportedPaddingAuto => {
                formatter.write_str("padding value `auto` is not supported by iced-shadcn-v2::Kbd")
            }
        }
    }
}

impl std::error::Error for KbdBuildError {}
