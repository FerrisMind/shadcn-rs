//! Configuration types used by the carousel component.

/// Axis a [`super::Carousel`] scrolls along.
///
/// ```rust
/// use iced_shadcn_v2::CarouselOrientation;
///
/// assert_eq!(CarouselOrientation::default(), CarouselOrientation::Horizontal);
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CarouselOrientation {
    /// Slides advance along the x axis (`orientation="horizontal"`).
    #[default]
    Horizontal,
    /// Slides advance along the y axis (`orientation="vertical"`).
    Vertical,
}

impl CarouselOrientation {
    /// Whether this is the vertical axis.
    pub const fn is_vertical(self) -> bool {
        matches!(self, Self::Vertical)
    }
}
