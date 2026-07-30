//! Resizable pane groups ported from shadcn-svelte to iced-shadcn-v2.
//!
//! The web component wraps paneforge: a [`ResizablePaneGroup`] owns the split
//! axis and layout percentages, [`ResizablePane`] slots carry content with
//! min/max/default sizes, and [`ResizableHandle`] dividers resize adjacent
//! panes. The port keeps that composition and the controlled contract — the
//! application owns the percentages and receives the next layout through
//! [`ResizablePaneGroup::on_layout_change`], like `onLayoutChange` on the web.
//!
//! Nested groups are supported by placing another pane group inside a pane's
//! content, matching the shadcn nested example.
//!
//! ```rust,no_run
//! use iced::Element;
//! use iced::widget::text;
//! use iced_shadcn_v2::{
//!     ResizableDirection, ResizableHandle, ResizablePane, ResizablePaneGroup, Theme,
//! };
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     LayoutChanged(iced_shadcn_v2::ResizableLayout),
//! }
//!
//! fn view<'a>(theme: &'a Theme, sizes: &[f32]) -> Element<'a, Message> {
//!     ResizablePaneGroup::new(theme)
//!         .direction(ResizableDirection::Horizontal)
//!         .sizes(sizes.to_vec())
//!         .pane(ResizablePane::new(text("Sidebar")).default_size(25.0))
//!         .handle(ResizableHandle::new())
//!         .pane(ResizablePane::new(text("Content")).default_size(75.0))
//!         .on_layout_change(Message::LayoutChanged)
//!         .into_element()
//!         .expect("valid pane group")
//! }
//! ```

mod error;
mod geometry;
mod render;
mod style;
mod types;

#[cfg(test)]
mod tests;

pub use error::ResizableBuildError;
pub use types::{
    ResizableDirection, ResizableHandle, ResizableLayout, ResizablePane, ResizableRadius,
};

use std::fmt;

use crate::iced_compat::widget::container;
use crate::iced_compat::{Element, Length};

use twill_core::prelude::Padding;

use crate::theme::Theme;

/// Builder-first resizable pane group styled from `shadcn-common` theme tokens.
///
/// ```rust
/// use iced::widget::text;
/// use iced_shadcn_v2::{ResizableDirection, ResizablePane, ResizablePaneGroup, Theme};
///
/// # #[derive(Debug, Clone)]
/// # enum Message {}
/// let theme = Theme::light();
/// let group = ResizablePaneGroup::<Message>::new(&theme)
///     .direction(ResizableDirection::Vertical)
///     .pane(ResizablePane::new(text("Header")).default_size(25.0));
/// ```
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct ResizablePaneGroup<'a, Message> {
    pub(super) theme: &'a Theme,
    pub(super) direction: ResizableDirection,
    pub(super) sizes: Option<ResizableLayout>,
    pub(super) slots: Vec<ResizableSlot<'a, Message>>,
    pub(super) width: Length,
    pub(super) height: Length,
    pub(super) padding: Option<crate::iced_compat::Padding>,
    pub(super) bordered: bool,
    pub(super) radius: ResizableRadius,
    pub(super) on_layout_change: Option<Box<dyn Fn(ResizableLayout) -> Message + 'a>>,
    pub(super) on_dragging_change: Option<Box<dyn Fn(bool) -> Message + 'a>>,
    pub(super) style_override: Option<Box<dyn Fn(container::Style) -> container::Style + 'a>>,
}

pub(super) enum ResizableSlot<'a, Message> {
    Pane(ResizablePane<'a, Message>),
    Handle(ResizableHandle),
}

impl<Message> fmt::Debug for ResizablePaneGroup<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ResizablePaneGroup")
            .field("theme", &self.theme)
            .field("direction", &self.direction)
            .field("sizes", &self.sizes)
            .field("slots", &self.slots.len())
            .field("width", &self.width)
            .field("height", &self.height)
            .field("padding", &self.padding)
            .field("bordered", &self.bordered)
            .field("radius", &self.radius)
            .field("on_layout_change", &self.on_layout_change.is_some())
            .field("on_dragging_change", &self.on_dragging_change.is_some())
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

impl<'a, Message> ResizablePaneGroup<'a, Message> {
    /// Creates an empty horizontal pane group.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            direction: ResizableDirection::default(),
            sizes: None,
            slots: Vec::new(),
            width: Length::Fill,
            height: Length::Fill,
            padding: None,
            bordered: false,
            radius: ResizableRadius::default(),
            on_layout_change: None,
            on_dragging_change: None,
            style_override: None,
        }
    }

    /// Sets the split axis (`direction` on the web).
    pub fn direction(mut self, direction: ResizableDirection) -> Self {
        self.direction = direction;
        self
    }

    /// Sets the controlled layout percentages (`onLayoutChange` / bound sizes).
    pub fn sizes(mut self, sizes: impl Into<ResizableLayout>) -> Self {
        self.sizes = Some(sizes.into());
        self
    }

    /// Sets the controlled layout from a slice of percentages.
    pub fn sizes_slice(mut self, sizes: &[f32]) -> Self {
        self.sizes = Some(ResizableLayout::new(sizes.to_vec()));
        self
    }

    /// Appends a pane slot.
    pub fn pane(mut self, pane: ResizablePane<'a, Message>) -> Self {
        self.slots.push(ResizableSlot::Pane(pane));
        self
    }

    /// Appends a handle between the previous and next pane.
    pub fn handle(mut self, handle: ResizableHandle) -> Self {
        self.slots.push(ResizableSlot::Handle(handle));
        self
    }

    /// Sets the outer width (`w-full` by default).
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the outer height (`h-full` by default).
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Sets the outer padding (`min-h-[200px] rounded-lg border p-*` on the web).
    ///
    /// # Errors
    ///
    /// Returns [`ResizableBuildError`] when any padding side contains a custom
    /// variable or `auto` value.
    pub fn padding(mut self, padding: Padding) -> Result<Self, ResizableBuildError> {
        self.padding = Some(geometry::resolve_padding(padding)?);
        Ok(self)
    }

    /// Paints a hairline frame with the theme `border` token.
    pub fn bordered(mut self, bordered: bool) -> Self {
        self.bordered = bordered;
        self
    }

    /// Sets the outer corner radius (`rounded-lg` in the shadcn examples).
    pub fn radius(mut self, radius: ResizableRadius) -> Self {
        self.radius = radius;
        self
    }

    /// Sets the callback invoked with the next layout when a handle is dragged.
    pub fn on_layout_change<F>(mut self, on_layout_change: F) -> Self
    where
        F: Fn(ResizableLayout) -> Message + 'a,
    {
        self.on_layout_change = Some(Box::new(on_layout_change));
        self
    }

    /// Sets or clears the layout-change callback.
    pub fn on_layout_change_maybe<F>(mut self, on_layout_change: Option<F>) -> Self
    where
        F: Fn(ResizableLayout) -> Message + 'a,
    {
        self.on_layout_change = on_layout_change.map(|callback| Box::new(callback) as _);
        self
    }

    /// Sets a callback for handle drag lifecycle (`onDraggingChange` on the web).
    pub fn on_dragging_change<F>(mut self, on_dragging_change: F) -> Self
    where
        F: Fn(bool) -> Message + 'a,
    {
        self.on_dragging_change = Some(Box::new(on_dragging_change));
        self
    }

    /// Applies an iced container-style override after semantic resolution.
    pub fn style_override(
        mut self,
        style_override: impl Fn(container::Style) -> container::Style + 'a,
    ) -> Self {
        self.style_override = Some(Box::new(style_override));
        self
    }

    /// Builds the pane group as an iced [`Element`](iced_core::Element).
    ///
    /// # Errors
    ///
    /// Returns [`ResizableBuildError`] when the slot sequence is invalid or
    /// padding cannot be resolved.
    pub fn into_element(self) -> Result<Element<'a, Message>, ResizableBuildError>
    where
        Message: Clone + 'a,
    {
        render::resizable_pane_group(self)
    }
}

impl<'a, Message> ResizablePaneGroup<'a, Message> {
    /// Builds the pane group, panicking if the slot sequence is invalid.
    ///
    /// Prefer [`Self::into_element`] in application code.
    pub fn into_element_or_panic(self) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        self.into_element()
            .expect("invalid resizable pane group configuration")
    }
}

/// Builds a resizable pane group as an iced [`Element`](iced_core::Element).
pub fn resizable_pane_group<'a, Message>(
    group: ResizablePaneGroup<'a, Message>,
) -> Result<Element<'a, Message>, ResizableBuildError>
where
    Message: Clone + 'a,
{
    group.into_element()
}
