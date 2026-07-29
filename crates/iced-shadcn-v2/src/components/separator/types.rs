//! Public configuration types for the separator component.

use crate::iced_compat::{Color, Length};

use crate::theme::Theme;

/// Layout axis of a [`Separator`].
///
/// Mirrors the `orientation` prop of bits-ui `Separator.Root`
/// (`"horizontal"` by default).
///
/// ```rust
/// use iced_shadcn_v2::SeparatorOrientation;
///
/// assert_eq!(
///     SeparatorOrientation::default(),
///     SeparatorOrientation::Horizontal,
/// );
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SeparatorOrientation {
    /// A horizontal rule spanning the available width (`h-px w-full`).
    #[default]
    Horizontal,
    /// A vertical rule spanning the available height (`w-px min-h-full`).
    Vertical,
}

/// Thin rule that visually or semantically separates content.
///
/// Port of the shadcn-svelte `Separator`: a 1 px line painted with the theme
/// `border` token, filling the available main axis. The `class` override of
/// the svelte component maps to [`Self::color`], [`Self::thickness`], and
/// [`Self::length`].
///
/// ```rust
/// use iced_shadcn_v2::{Separator, SeparatorOrientation, Theme};
///
/// let theme = Theme::light();
/// let rule = Separator::new(&theme)
///     .orientation(SeparatorOrientation::Vertical)
///     .thickness(2.0);
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct Separator {
    pub(super) color: Color,
    pub(super) orientation: SeparatorOrientation,
    pub(super) thickness: f32,
    pub(super) length: Length,
    pub(super) radius: f32,
    pub(super) decorative: bool,
}

impl Separator {
    /// Separator painted with the theme `border` color (`bg-border`).
    pub fn new(theme: &Theme) -> Self {
        Self::from_color(theme.palette.border)
    }

    /// Separator with an explicit color.
    pub fn from_color(color: Color) -> Self {
        Self {
            color,
            orientation: SeparatorOrientation::default(),
            thickness: 1.0,
            length: Length::Fill,
            radius: 0.0,
            decorative: false,
        }
    }

    /// Sets the layout axis of the separator.
    pub fn orientation(mut self, orientation: SeparatorOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Sets the separator color, overriding the theme `border` token.
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Sets the rule thickness in pixels (clamped to at least 1 px).
    ///
    /// The default of 1 px matches the `h-px` / `w-px` classes of the
    /// reference component.
    pub fn thickness(mut self, thickness: f32) -> Self {
        self.thickness = thickness.max(1.0);
        self
    }

    /// Sets the extent along the main axis (default [`Length::Fill`]).
    ///
    /// The main axis is the width for [`SeparatorOrientation::Horizontal`]
    /// and the height for [`SeparatorOrientation::Vertical`].
    pub fn length(mut self, length: impl Into<Length>) -> Self {
        self.length = length.into();
        self
    }

    /// Sets the corner radius in pixels (clamped to at least 0 px).
    ///
    /// The reference component has square corners, so the default is 0.
    /// Rounding only becomes visible on separators thicker than ~2 px; pass
    /// half the thickness (or any larger value) for capsule ends, mirroring a
    /// `rounded-full` class override in shadcn-svelte.
    pub fn radius(mut self, radius: f32) -> Self {
        self.radius = radius.max(0.0);
        self
    }

    /// Marks the separator as purely decorative.
    ///
    /// Mirrors the `decorative` prop of bits-ui `Separator.Root`, which
    /// removes the element from the accessibility tree. iced does not expose
    /// an accessibility tree yet, so the flag is carried for API parity and
    /// will take effect once accessibility support lands.
    pub fn decorative(mut self, decorative: bool) -> Self {
        self.decorative = decorative;
        self
    }

    /// Width and height of the underlying widget for the configured axis.
    pub(super) fn resolved_axes(self) -> (Length, Length) {
        let thickness = Length::Fixed(self.thickness);
        match self.orientation {
            SeparatorOrientation::Horizontal => (self.length, thickness),
            SeparatorOrientation::Vertical => (thickness, self.length),
        }
    }
}
