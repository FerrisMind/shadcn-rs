//! Public configuration types for the scroll-area component.

/// Axes a [`super::ScrollArea`] mounts a scrollbar for.
///
/// Mirrors the `orientation` prop of the shadcn-svelte `ScrollArea`, which
/// defaults to `"vertical"`.
///
/// ```rust
/// use iced_shadcn_v2::ScrollAreaOrientation;
///
/// assert_eq!(
///     ScrollAreaOrientation::default(),
///     ScrollAreaOrientation::Vertical,
/// );
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ScrollAreaOrientation {
    /// Only the vertical scrollbar is mounted (`orientation="vertical"`).
    #[default]
    Vertical,
    /// Only the horizontal scrollbar is mounted (`orientation="horizontal"`).
    Horizontal,
    /// Both scrollbars are mounted, with a corner between them
    /// (`orientation="both"`).
    Both,
}

impl ScrollAreaOrientation {
    /// Whether the vertical scrollbar is mounted.
    ///
    /// ```rust
    /// use iced_shadcn_v2::ScrollAreaOrientation;
    ///
    /// assert!(ScrollAreaOrientation::Both.has_vertical());
    /// assert!(!ScrollAreaOrientation::Horizontal.has_vertical());
    /// ```
    pub const fn has_vertical(self) -> bool {
        matches!(self, Self::Vertical | Self::Both)
    }

    /// Whether the horizontal scrollbar is mounted.
    ///
    /// ```rust
    /// use iced_shadcn_v2::ScrollAreaOrientation;
    ///
    /// assert!(ScrollAreaOrientation::Both.has_horizontal());
    /// assert!(!ScrollAreaOrientation::Vertical.has_horizontal());
    /// ```
    pub const fn has_horizontal(self) -> bool {
        matches!(self, Self::Horizontal | Self::Both)
    }
}

/// Corner-radius preset for a [`super::ScrollArea`] frame or thumb.
///
/// [`ScrollAreaRadius::Theme`] resolves to the active style-pack value **of the
/// slot it is applied to**: the reference component leaves its frame square
/// (`rounded-[inherit]`), while its thumb follows `.cn-scroll-area-thumb` — a
/// pill in most packs and square corners in Lyra and Sera. Every other variant
/// resolves against the theme radius scale, independently of the slot.
///
/// ```rust
/// use iced_shadcn_v2::ScrollAreaRadius;
///
/// assert_eq!(ScrollAreaRadius::default(), ScrollAreaRadius::Theme);
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ScrollAreaRadius {
    /// The active style-pack radius of the slot this preset is applied to.
    #[default]
    Theme,
    /// Square corners.
    None,
    /// The theme's small radius slot.
    Small,
    /// The theme's medium radius slot.
    Medium,
    /// The theme's large radius slot.
    Large,
    /// The theme's extra-large radius slot.
    Xl,
    /// A pill radius, capped by the rendered bounds.
    Full,
    /// An explicit radius in logical pixels. Invalid values resolve to zero.
    Custom(f32),
}

/// Edge a [`super::ScrollArea`] keeps its content anchored to on one axis.
///
/// Anchoring to [`ScrollAreaAnchor::End`] keeps a growing list pinned to its
/// bottom (or trailing) edge, the way a chat transcript behaves. The web
/// component has no equivalent prop; it is offered because iced exposes it.
///
/// ```rust
/// use iced_shadcn_v2::ScrollAreaAnchor;
///
/// assert_eq!(ScrollAreaAnchor::default(), ScrollAreaAnchor::Start);
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ScrollAreaAnchor {
    /// Content stays anchored to the top (or leading) edge.
    #[default]
    Start,
    /// Content stays anchored to the bottom (or trailing) edge.
    End,
}

/// Geometry of one scrollbar of a [`super::ScrollArea`].
///
/// The defaults reproduce `.cn-scroll-area-scrollbar`: a 10 px rail
/// (`w-2.5` / `h-2.5`) with a 1 px inset around the thumb (`p-px`), floating
/// over the content. This is the typed counterpart of the `scrollbarXClasses`
/// and `scrollbarYClasses` props, which let the web component tune each axis
/// separately.
///
/// Every measurement is in logical pixels; negative and non-finite values are
/// normalized to `0.0`.
///
/// ```rust
/// use iced_shadcn_v2::{ScrollAreaAnchor, ScrollAreaScrollbar};
///
/// let slim = ScrollAreaScrollbar::new().width(6.0).padding(0.0);
/// assert_eq!(slim.thumb_width(), 6.0);
///
/// let hidden = ScrollAreaScrollbar::hidden();
/// assert!(hidden.is_hidden());
/// assert_eq!(
///     ScrollAreaScrollbar::new().anchor(ScrollAreaAnchor::End),
///     ScrollAreaScrollbar::default().anchor(ScrollAreaAnchor::End),
/// );
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
#[must_use = "configuring a scrollbar has no effect unless it is passed to a ScrollArea"]
pub struct ScrollAreaScrollbar {
    pub(super) width: f32,
    pub(super) padding: f32,
    pub(super) margin: f32,
    pub(super) spacing: Option<f32>,
    pub(super) anchor: ScrollAreaAnchor,
    pub(super) hidden: bool,
}

/// Rail width of `.cn-scroll-area-scrollbar` (`w-2.5` / `h-2.5`).
const DEFAULT_WIDTH: f32 = 10.0;
/// Inset between the rail and its thumb (`p-px`).
const DEFAULT_PADDING: f32 = 1.0;

impl Default for ScrollAreaScrollbar {
    fn default() -> Self {
        Self {
            width: DEFAULT_WIDTH,
            padding: DEFAULT_PADDING,
            margin: 0.0,
            spacing: None,
            anchor: ScrollAreaAnchor::Start,
            hidden: false,
        }
    }
}

impl ScrollAreaScrollbar {
    /// Creates a scrollbar with the style-pack defaults.
    ///
    /// ```rust
    /// use iced_shadcn_v2::ScrollAreaScrollbar;
    ///
    /// assert_eq!(ScrollAreaScrollbar::new(), ScrollAreaScrollbar::default());
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a scrollbar that scrolls without painting a rail or thumb.
    ///
    /// The axis keeps responding to the wheel and to touch, matching a
    /// `scrollbar-none` utility on the web viewport.
    ///
    /// ```rust
    /// use iced_shadcn_v2::ScrollAreaScrollbar;
    ///
    /// assert!(ScrollAreaScrollbar::hidden().is_hidden());
    /// ```
    pub fn hidden() -> Self {
        Self {
            hidden: true,
            ..Self::default()
        }
    }

    /// Sets the rail width in logical pixels.
    ///
    /// ```rust
    /// use iced_shadcn_v2::ScrollAreaScrollbar;
    ///
    /// assert_eq!(ScrollAreaScrollbar::new().width(-1.0).thumb_width(), 0.0);
    /// ```
    pub fn width(mut self, width: f32) -> Self {
        self.width = normalize_px(width);
        self
    }

    /// Sets the inset between the rail and its thumb in logical pixels.
    ///
    /// ```rust
    /// use iced_shadcn_v2::ScrollAreaScrollbar;
    ///
    /// assert_eq!(ScrollAreaScrollbar::new().padding(2.0).thumb_width(), 6.0);
    /// ```
    pub fn padding(mut self, padding: f32) -> Self {
        self.padding = normalize_px(padding);
        self
    }

    /// Sets the gap between the rail and the edge of the scroll area.
    ///
    /// ```rust
    /// use iced_shadcn_v2::ScrollAreaScrollbar;
    ///
    /// let inset = ScrollAreaScrollbar::new().margin(4.0);
    /// assert_ne!(inset, ScrollAreaScrollbar::new());
    /// ```
    pub fn margin(mut self, margin: f32) -> Self {
        self.margin = normalize_px(margin);
        self
    }

    /// Embeds the rail next to the content instead of floating over it,
    /// leaving `spacing` logical pixels between them.
    ///
    /// An embedded rail always takes layout space, so it is ignored for
    /// [`ScrollAreaOrientation::Both`], where iced cannot reserve space on two
    /// axes at once.
    ///
    /// ```rust
    /// use iced_shadcn_v2::ScrollAreaScrollbar;
    ///
    /// let gutter = ScrollAreaScrollbar::new().spacing(8.0);
    /// assert_ne!(gutter, ScrollAreaScrollbar::new());
    /// ```
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = Some(normalize_px(spacing));
        self
    }

    /// Sets the edge content stays anchored to on this axis.
    ///
    /// ```rust
    /// use iced_shadcn_v2::{ScrollAreaAnchor, ScrollAreaScrollbar};
    ///
    /// let transcript = ScrollAreaScrollbar::new().anchor(ScrollAreaAnchor::End);
    /// assert_ne!(transcript, ScrollAreaScrollbar::new());
    /// ```
    pub fn anchor(mut self, anchor: ScrollAreaAnchor) -> Self {
        self.anchor = anchor;
        self
    }

    /// Whether the rail and thumb are left unpainted.
    ///
    /// ```rust
    /// use iced_shadcn_v2::ScrollAreaScrollbar;
    ///
    /// assert!(!ScrollAreaScrollbar::new().is_hidden());
    /// ```
    pub const fn is_hidden(self) -> bool {
        self.hidden
    }

    /// Thumb width in logical pixels: the rail minus its inset on both sides.
    ///
    /// ```rust
    /// use iced_shadcn_v2::ScrollAreaScrollbar;
    ///
    /// assert_eq!(ScrollAreaScrollbar::new().thumb_width(), 8.0);
    /// ```
    pub fn thumb_width(self) -> f32 {
        (self.width - 2.0 * self.padding).max(0.0)
    }
}

/// Replaces negative and non-finite measurements with `0.0`.
fn normalize_px(value: f32) -> f32 {
    if value.is_finite() {
        value.max(0.0)
    } else {
        0.0
    }
}
