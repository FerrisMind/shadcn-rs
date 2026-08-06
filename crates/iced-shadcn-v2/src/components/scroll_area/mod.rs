//! Builder-first scroll-area component.
//!
//! Port of the shadcn-svelte `ScrollArea` (bits-ui `ScrollArea.Root`): a frame
//! that clips its content and paints a themed rail and thumb instead of the
//! platform scrollbar. The web component augments native scrolling to make it
//! stylable across browsers; on iced, [`mod@iced::widget::scrollable`](mod@iced_widget::scrollable) already
//! owns the scrolling, so this builder maps the shadcn geometry
//! (`.cn-scroll-area-scrollbar`, `.cn-scroll-area-thumb`) and the theme
//! `border` token onto it.
//!
//! The `orientation` prop becomes [`ScrollAreaOrientation`], and the
//! `scrollbarXClasses` / `scrollbarYClasses` escape hatches become typed
//! per-axis [`ScrollAreaScrollbar`] geometry. Sizing that the web component
//! takes from utility classes (`h-[200px] w-[350px] rounded-md border p-4`) is
//! expressed with [`ScrollArea::width`], [`ScrollArea::height`],
//! [`ScrollArea::radius`], [`ScrollArea::bordered`], and
//! [`ScrollArea::padding`].
//!
//! A scroll area only scrolls along a bounded axis, exactly like its web
//! counterpart: give a vertical scroll area a [`ScrollArea::height`] (or a
//! bounded parent), otherwise it grows with its content and never overflows.
//!
//! ```rust,no_run
//! use iced::widget::text;
//! use iced::{Element, Length};
//! use iced_shadcn_v2::{ScrollArea, ScrollAreaRadius, Theme};
//!
//! #[derive(Debug, Clone)]
//! enum Message {}
//!
//! fn jokes(theme: &Theme) -> Element<'_, Message> {
//!     ScrollArea::new(text("Jokester began sneaking into the castle…"), theme)
//!         .width(Length::Fixed(350.0))
//!         .height(Length::Fixed(200.0))
//!         .radius(ScrollAreaRadius::Medium)
//!         .bordered(true)
//!         .into()
//! }
//! ```

mod error;
mod geometry;
mod render;
mod style;
mod types;

#[cfg(test)]
mod tests;

pub use error::ScrollAreaBuildError;
pub use types::{ScrollAreaAnchor, ScrollAreaOrientation, ScrollAreaRadius, ScrollAreaScrollbar};

use std::fmt;

use crate::iced_compat::widget::scrollable;
use crate::iced_compat::{Color, Element, Length};

use twill_core::prelude::Padding;

use crate::theme::Theme;

/// Builder-first scroll area styled from `shadcn-common` theme tokens.
///
/// The rail is transparent and the thumb uses the theme `border` token, so a
/// scroll area restyles together with its [`Theme`]. Per-instance overrides
/// ([`Self::thumb_color`], [`Self::track_color`], [`Self::thumb_radius`]) take
/// priority over the pack, and [`Self::style_override`] patches the resolved
/// iced [`scrollable::Style`] last of all.
///
/// ```rust,no_run
/// use iced::widget::column;
/// use iced::{Element, Length};
/// use iced_shadcn_v2::{ScrollArea, ScrollAreaOrientation, ScrollAreaScrollbar, Theme};
///
/// #[derive(Debug, Clone)]
/// enum Message {}
///
/// fn gallery<'a>(theme: &'a Theme, row: Element<'a, Message>) -> Element<'a, Message> {
///     ScrollArea::new(row, theme)
///         .orientation(ScrollAreaOrientation::Horizontal)
///         .horizontal_scrollbar(ScrollAreaScrollbar::new().width(6.0))
///         .width(Length::Fixed(384.0))
///         .into()
/// }
/// ```
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct ScrollArea<'a, Message> {
    content: Element<'a, Message>,
    theme: &'a Theme,
    orientation: ScrollAreaOrientation,
    vertical: ScrollAreaScrollbar,
    horizontal: ScrollAreaScrollbar,
    width: Option<Length>,
    height: Option<Length>,
    padding: Option<crate::iced_compat::Padding>,
    radius: ScrollAreaRadius,
    thumb_radius: ScrollAreaRadius,
    bordered: bool,
    background: Option<Color>,
    track_color: Option<Color>,
    thumb_color: Option<Color>,
    auto_scroll: bool,
    id: Option<crate::iced_compat::widget::Id>,
    on_scroll: Option<Box<dyn Fn(scrollable::Viewport) -> Message + 'a>>,
    style_override:
        Option<Box<dyn Fn(scrollable::Style, scrollable::Status) -> scrollable::Style + 'a>>,
}

impl<Message> fmt::Debug for ScrollArea<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ScrollArea")
            .field("theme", &self.theme)
            .field("orientation", &self.orientation)
            .field("vertical", &self.vertical)
            .field("horizontal", &self.horizontal)
            .field("width", &self.width)
            .field("height", &self.height)
            .field("padding", &self.padding)
            .field("radius", &self.radius)
            .field("thumb_radius", &self.thumb_radius)
            .field("bordered", &self.bordered)
            .field("background", &self.background)
            .field("track_color", &self.track_color)
            .field("thumb_color", &self.thumb_color)
            .field("auto_scroll", &self.auto_scroll)
            .field("id", &self.id)
            .field("on_scroll", &self.on_scroll.is_some())
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

impl<'a, Message> ScrollArea<'a, Message> {
    /// Creates a vertical scroll area around arbitrary content.
    ///
    /// `theme` is required because styling is derived from `shadcn-common`
    /// theme tokens instead of `iced::Theme`.
    ///
    /// ```rust
    /// use iced::widget::text;
    /// use iced_shadcn_v2::{ScrollArea, Theme};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let area = ScrollArea::<Message>::new(text("Notes"), &theme);
    /// ```
    pub fn new(content: impl Into<Element<'a, Message>>, theme: &'a Theme) -> Self {
        Self {
            content: content.into(),
            theme,
            orientation: ScrollAreaOrientation::Vertical,
            vertical: ScrollAreaScrollbar::default(),
            horizontal: ScrollAreaScrollbar::default(),
            width: None,
            height: None,
            padding: None,
            radius: ScrollAreaRadius::Theme,
            thumb_radius: ScrollAreaRadius::Theme,
            bordered: false,
            background: None,
            track_color: None,
            thumb_color: None,
            auto_scroll: false,
            id: None,
            on_scroll: None,
            style_override: None,
        }
    }

    /// Sets the axes that mount a scrollbar (`orientation`).
    ///
    /// ```rust
    /// use iced::widget::text;
    /// use iced_shadcn_v2::{ScrollArea, ScrollAreaOrientation, Theme};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let area = ScrollArea::<Message>::new(text("Notes"), &theme)
    ///     .orientation(ScrollAreaOrientation::Both);
    /// ```
    pub fn orientation(mut self, orientation: ScrollAreaOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Sets the geometry of both scrollbars.
    ///
    /// ```rust
    /// use iced::widget::text;
    /// use iced_shadcn_v2::{ScrollArea, ScrollAreaScrollbar, Theme};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let area = ScrollArea::<Message>::new(text("Notes"), &theme)
    ///     .scrollbar(ScrollAreaScrollbar::new().width(6.0));
    /// ```
    pub fn scrollbar(mut self, scrollbar: ScrollAreaScrollbar) -> Self {
        self.vertical = scrollbar;
        self.horizontal = scrollbar;
        self
    }

    /// Sets the geometry of the vertical scrollbar only (`scrollbarYClasses`).
    ///
    /// ```rust
    /// use iced::widget::text;
    /// use iced_shadcn_v2::{ScrollArea, ScrollAreaScrollbar, Theme};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let area = ScrollArea::<Message>::new(text("Notes"), &theme)
    ///     .vertical_scrollbar(ScrollAreaScrollbar::hidden());
    /// ```
    pub fn vertical_scrollbar(mut self, scrollbar: ScrollAreaScrollbar) -> Self {
        self.vertical = scrollbar;
        self
    }

    /// Sets the geometry of the horizontal scrollbar only
    /// (`scrollbarXClasses`).
    ///
    /// ```rust
    /// use iced::widget::text;
    /// use iced_shadcn_v2::{ScrollArea, ScrollAreaScrollbar, Theme};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let area = ScrollArea::<Message>::new(text("Notes"), &theme)
    ///     .horizontal_scrollbar(ScrollAreaScrollbar::new().margin(2.0));
    /// ```
    pub fn horizontal_scrollbar(mut self, scrollbar: ScrollAreaScrollbar) -> Self {
        self.horizontal = scrollbar;
        self
    }

    /// Sets the frame width.
    ///
    /// Left unset, iced shrinks the frame around its content on the
    /// non-scrolling axis.
    ///
    /// ```rust
    /// use iced::widget::text;
    /// use iced::Length;
    /// use iced_shadcn_v2::{ScrollArea, Theme};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let area = ScrollArea::<Message>::new(text("Notes"), &theme)
    ///     .width(Length::Fixed(350.0));
    /// ```
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = Some(width.into());
        self
    }

    /// Sets the frame height.
    ///
    /// A vertical scroll area needs a bounded height to overflow at all — the
    /// counterpart of the `h-[200px]` class in the reference usage.
    ///
    /// ```rust
    /// use iced::widget::text;
    /// use iced::Length;
    /// use iced_shadcn_v2::{ScrollArea, Theme};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let area = ScrollArea::<Message>::new(text("Notes"), &theme)
    ///     .height(Length::Fixed(200.0));
    /// ```
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = Some(height.into());
        self
    }

    /// Insets the scrolled content on every supported side.
    ///
    /// The inset is applied inside the scrolled content, so the rails keep
    /// hugging the frame edge as they do on the web.
    /// [`twill_core::prelude::PaddingValue::Var`] cannot be resolved by iced
    /// and is rejected with
    /// [`ScrollAreaBuildError::UnsupportedPaddingVariable`]; the same applies
    /// to [`twill_core::prelude::Spacing::Auto`], which has no fixed-size iced
    /// representation.
    ///
    /// # Errors
    ///
    /// Returns [`ScrollAreaBuildError`] when any padding side contains a custom
    /// variable or `auto` value. The builder is consumed either way; rebuild
    /// the scroll area with a supported padding to recover.
    ///
    /// ```rust
    /// use iced::widget::text;
    /// use iced_shadcn_v2::{Padding, ScrollArea, ScrollAreaBuildError, Spacing, Theme};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// # fn main() -> Result<(), ScrollAreaBuildError> {
    /// let theme = Theme::light();
    /// let area = ScrollArea::<Message>::new(text("Notes"), &theme)
    ///     .padding(Padding::all(Spacing::S4))?;
    /// # let _ = area;
    /// # Ok(())
    /// # }
    /// ```
    pub fn padding(mut self, padding: Padding) -> Result<Self, ScrollAreaBuildError> {
        self.padding = Some(geometry::resolve_padding(padding)?);
        Ok(self)
    }

    /// Sets the frame corner radius.
    ///
    /// ```rust
    /// use iced::widget::text;
    /// use iced_shadcn_v2::{ScrollArea, ScrollAreaRadius, Theme};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let area = ScrollArea::<Message>::new(text("Notes"), &theme)
    ///     .radius(ScrollAreaRadius::Medium);
    /// ```
    pub fn radius(mut self, radius: ScrollAreaRadius) -> Self {
        self.radius = radius;
        self
    }

    /// Sets the thumb corner radius, overriding `.cn-scroll-area-thumb`.
    ///
    /// ```rust
    /// use iced::widget::text;
    /// use iced_shadcn_v2::{ScrollArea, ScrollAreaRadius, Theme};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let area = ScrollArea::<Message>::new(text("Notes"), &theme)
    ///     .thumb_radius(ScrollAreaRadius::None);
    /// ```
    pub fn thumb_radius(mut self, radius: ScrollAreaRadius) -> Self {
        self.thumb_radius = radius;
        self
    }

    /// Paints a hairline frame with the theme `border` token, matching the
    /// `border` class of the reference usage.
    ///
    /// ```rust
    /// use iced::widget::text;
    /// use iced_shadcn_v2::{ScrollArea, Theme};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let area = ScrollArea::<Message>::new(text("Notes"), &theme).bordered(true);
    /// ```
    pub fn bordered(mut self, bordered: bool) -> Self {
        self.bordered = bordered;
        self
    }

    /// Fills the frame with an explicit surface color.
    ///
    /// The reference component is transparent; pass
    /// [`Theme::semantic_color`] of a surface slot to sit the scroll area on a
    /// card or popover instead.
    ///
    /// ```rust
    /// use iced::widget::text;
    /// use iced_shadcn_v2::{ScrollArea, Theme};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let area = ScrollArea::<Message>::new(text("Notes"), &theme)
    ///     .background(theme.palette.card);
    /// ```
    pub fn background(mut self, background: Color) -> Self {
        self.background = Some(background);
        self
    }

    /// Fills the rail behind the thumb, which is transparent by default.
    ///
    /// ```rust
    /// use iced::widget::text;
    /// use iced_shadcn_v2::{ScrollArea, Theme};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let area = ScrollArea::<Message>::new(text("Notes"), &theme)
    ///     .track_color(theme.palette.muted);
    /// ```
    pub fn track_color(mut self, color: Color) -> Self {
        self.track_color = Some(color);
        self
    }

    /// Overrides the thumb color, which follows the theme `border` token.
    ///
    /// Hover and drag keep emphasising whichever color is set.
    ///
    /// ```rust
    /// use iced::widget::text;
    /// use iced_shadcn_v2::{ScrollArea, Theme};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let area = ScrollArea::<Message>::new(text("Notes"), &theme)
    ///     .thumb_color(theme.palette.primary);
    /// ```
    pub fn thumb_color(mut self, color: Color) -> Self {
        self.thumb_color = Some(color);
        self
    }

    /// Lets the middle mouse button auto-scroll the content.
    ///
    /// This is an iced affordance with no web counterpart, so it is off by
    /// default.
    ///
    /// ```rust
    /// use iced::widget::text;
    /// use iced_shadcn_v2::{ScrollArea, Theme};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let area = ScrollArea::<Message>::new(text("Notes"), &theme).auto_scroll(true);
    /// ```
    pub fn auto_scroll(mut self, auto_scroll: bool) -> Self {
        self.auto_scroll = auto_scroll;
        self
    }

    /// Names the scroll area so the application can scroll it programmatically
    /// with [`iced::widget::operation::scroll_to`](iced_core::widget::operation) and its siblings.
    ///
    /// This replaces the `ref` / `viewportRef` bindings of the web component.
    ///
    /// ```rust
    /// use iced::widget::text;
    /// use iced_shadcn_v2::{ScrollArea, Theme};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let area = ScrollArea::<Message>::new(text("Notes"), &theme)
    ///     .id(iced::widget::Id::new("notes"));
    /// ```
    pub fn id(mut self, id: impl Into<crate::iced_compat::widget::Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Sets the callback invoked with the [`scrollable::Viewport`] whenever the
    /// content is scrolled.
    ///
    /// ```rust
    /// use iced::widget::text;
    /// use iced::widget::scrollable::Viewport;
    /// use iced_shadcn_v2::{ScrollArea, Theme};
    ///
    /// #[derive(Debug, Clone)]
    /// enum Message {
    ///     Scrolled(Viewport),
    /// }
    ///
    /// let theme = Theme::light();
    /// let area = ScrollArea::new(text("Notes"), &theme).on_scroll(Message::Scrolled);
    /// ```
    pub fn on_scroll(mut self, on_scroll: impl Fn(scrollable::Viewport) -> Message + 'a) -> Self {
        self.on_scroll = Some(Box::new(on_scroll));
        self
    }

    /// Applies a narrow iced-style escape hatch after internal style
    /// resolution.
    ///
    /// ```rust
    /// use iced::widget::text;
    /// use iced_shadcn_v2::{ScrollArea, Theme};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let area = ScrollArea::<Message>::new(text("Notes"), &theme)
    ///     .style_override(|mut style, _status| {
    ///         style.gap = None;
    ///         style
    ///     });
    /// ```
    pub fn style_override(
        mut self,
        style_override: impl Fn(scrollable::Style, scrollable::Status) -> scrollable::Style + 'a,
    ) -> Self {
        self.style_override = Some(Box::new(style_override));
        self
    }

    /// Builds the underlying `iced` scrollable widget.
    ///
    /// Use this instead of [`Into::into`] to keep tweaking the widget with the
    /// iced API before turning it into an [`Element`](iced_core::Element).
    ///
    /// ```rust
    /// use iced::widget::text;
    /// use iced_shadcn_v2::{ScrollArea, Theme};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let widget = ScrollArea::<Message>::new(text("Notes"), &theme).into_scrollable();
    /// ```
    pub fn into_scrollable(self) -> scrollable::Scrollable<'a, Message>
    where
        Message: 'a,
    {
        render::build_scrollable(self)
    }
}

impl<'a, Message: 'a> From<ScrollArea<'a, Message>> for Element<'a, Message> {
    fn from(scroll_area: ScrollArea<'a, Message>) -> Self {
        scroll_area.into_scrollable().into()
    }
}
