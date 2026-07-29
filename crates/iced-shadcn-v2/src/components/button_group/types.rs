//! Configuration types used by the button-group component.

/// Layout axis of a [`super::ButtonGroup`].
///
/// Mirrors the `orientation` variant of the shadcn-svelte
/// `buttonGroupVariants` (`"horizontal"` by default).
///
/// ```rust
/// use iced_shadcn_v2::ButtonGroupOrientation;
///
/// assert_eq!(
///     ButtonGroupOrientation::default(),
///     ButtonGroupOrientation::Horizontal,
/// );
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ButtonGroupOrientation {
    /// Children flow in a row and merge their vertical edges.
    #[default]
    Horizontal,
    /// Children flow in a column and merge their horizontal edges.
    Vertical,
}
