//! A single controlled tree view for small and large hierarchical data.
//!
//! The default renderer composes [`crate::Collapsible`] folders and
//! [`crate::Button`] rows, so expansion animation and row styling inherit from
//! the same [`crate::Theme`]. [`TreeViewRenderMode::Virtualized`] uses the same
//! common model and action contract while drawing only rows inside the scroll
//! viewport.
//!
//! ```rust,no_run
//! use iced::Element;
//! use iced_shadcn_v2::{
//!     TreeNode, TreeNodeId, TreeView, TreeViewAction, TreeViewState, Theme,
//! };
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     Tree(TreeViewAction),
//! }
//!
//! fn view<'a>(
//!     theme: &'a Theme,
//!     roots: &'a [TreeNode],
//!     state: &'a TreeViewState,
//! ) -> Result<Element<'a, Message>, iced_shadcn_v2::TreeViewBuildError> {
//!     Ok(TreeView::new(theme, roots, state)?
//!         .on_action(Message::Tree)
//!         .into())
//! }
//!
//! # let _ = (view, TreeNodeId::new("example"));
//! ```

mod error;
mod geometry;
mod icon;
mod render;
mod style;
mod types;

#[cfg(test)]
mod tests;

pub use error::TreeViewBuildError;
pub use types::{
    TreeIconRenderer, TreeNavigationPolicy, TreeScrollbarPolicy, TreeSelectionMode, TreeView,
    TreeViewMeasurement, TreeViewRenderMode,
};

use std::fmt;
use std::rc::Rc;
use std::time::Duration;

use crate::iced_compat::{Color, Element, Length};
use crate::theme::Theme;
use shadcn_common::{
    TreeIconKey, TreeNode, TreeOrdering, TreeViewAction, TreeViewState, validate_tree,
};

use self::types::TreeView as TreeViewBuilder;

impl<Message> fmt::Debug for TreeView<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TreeView")
            .field("theme", &self.theme)
            .field("roots", &self.roots.len())
            .field("state", &self.state)
            .field("width", &self.width)
            .field("height", &self.height)
            .field("row_height", &self.row_height)
            .field("indent", &self.indent)
            .field("icon_size", &self.icon_size)
            .field("text_size", &self.text_size)
            .field("content_offset", &self.content_offset)
            .field("max_label_width", &self.max_label_width)
            .field("selection", &self.selection)
            .field("ordering", &self.ordering)
            .field("navigation", &self.navigation)
            .field("scrollbar", &self.scrollbar)
            .field("render_mode", &self.render_mode)
            .field("animated", &self.animated)
            .field("duration", &self.duration)
            .field("easing", &self.easing)
            .field("on_action", &self.on_action.is_some())
            .field("icon_renderer", &self.icon_renderer.is_some())
            .finish()
    }
}

impl<'a, Message> TreeView<'a, Message> {
    /// Validates `roots` and creates a controlled tree builder.
    ///
    /// The root and state are borrowed for the lifetime of the resulting
    /// element. The application remains the source of truth after an action is
    /// emitted.
    ///
    /// # Errors
    ///
    /// Returns [`TreeViewBuildError`] when IDs are duplicated or a lazy-folder
    /// invariant is violated.
    pub fn new(
        theme: &'a Theme,
        roots: &'a [TreeNode],
        state: &'a TreeViewState,
    ) -> Result<Self, TreeViewBuildError> {
        validate_tree(roots).map_err(TreeViewBuildError::from)?;
        Ok(Self::with_defaults(theme, roots, state))
    }

    /// Alias for [`Self::new`] that reads naturally at fallible call sites.
    pub fn try_new(
        theme: &'a Theme,
        roots: &'a [TreeNode],
        state: &'a TreeViewState,
    ) -> Result<Self, TreeViewBuildError> {
        Self::new(theme, roots, state)
    }

    /// Sets the outer tree width.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the outer tree height.
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Sets the row height in logical pixels.
    ///
    /// # Errors
    ///
    /// Returns [`TreeViewBuildError::InvalidMeasurement`] for zero, negative,
    /// or non-finite values.
    pub fn row_height(mut self, value: f32) -> Result<Self, TreeViewBuildError> {
        self.row_height = geometry::positive(value, TreeViewMeasurement::RowHeight)?;
        Ok(self)
    }

    /// Sets the per-depth horizontal indent in logical pixels.
    ///
    /// # Errors
    ///
    /// Returns [`TreeViewBuildError::InvalidMeasurement`] for zero, negative,
    /// or non-finite values.
    pub fn indent(mut self, value: f32) -> Result<Self, TreeViewBuildError> {
        self.indent = geometry::positive(value, TreeViewMeasurement::Indent)?;
        Ok(self)
    }

    /// Sets the icon size in logical pixels.
    ///
    /// # Errors
    ///
    /// Returns [`TreeViewBuildError::InvalidMeasurement`] for zero, negative,
    /// or non-finite values.
    pub fn icon_size(mut self, value: f32) -> Result<Self, TreeViewBuildError> {
        self.icon_size = geometry::positive(value, TreeViewMeasurement::IconSize)?;
        Ok(self)
    }

    /// Sets the label size in logical pixels.
    ///
    /// # Errors
    ///
    /// Returns [`TreeViewBuildError::InvalidMeasurement`] for zero, negative,
    /// or non-finite values.
    pub fn text_size(mut self, value: f32) -> Result<Self, TreeViewBuildError> {
        self.text_size = geometry::positive(value, TreeViewMeasurement::TextSize)?;
        Ok(self)
    }

    /// Sets an additional non-negative inset before row content.
    ///
    /// # Errors
    ///
    /// Returns [`TreeViewBuildError::InvalidMeasurement`] for negative or
    /// non-finite values.
    pub fn content_offset(mut self, value: f32) -> Result<Self, TreeViewBuildError> {
        self.content_offset = geometry::non_negative(value, TreeViewMeasurement::ContentOffset)?;
        Ok(self)
    }

    /// Sets the maximum label width in logical pixels.
    ///
    /// # Errors
    ///
    /// Returns [`TreeViewBuildError::InvalidMeasurement`] for zero, negative,
    /// or non-finite values.
    pub fn max_label_width(mut self, value: f32) -> Result<Self, TreeViewBuildError> {
        self.max_label_width = Some(geometry::positive(
            value,
            TreeViewMeasurement::MaxLabelWidth,
        )?);
        Ok(self)
    }

    /// Removes a label width limit.
    pub fn without_label_limit(mut self) -> Self {
        self.max_label_width = None;
        self
    }

    /// Sets the controlled selection policy.
    pub fn selection(mut self, selection: TreeSelectionMode) -> Self {
        self.selection = selection;
        self
    }

    /// Sets the visible-row ordering policy.
    pub fn ordering(mut self, ordering: TreeOrdering) -> Self {
        self.ordering = ordering;
        self
    }

    /// Sets the optional keyboard navigation policy.
    pub fn navigation(mut self, navigation: TreeNavigationPolicy) -> Self {
        self.navigation = navigation;
        self
    }

    /// Sets the vertical scrollbar policy.
    pub fn scrollbar(mut self, scrollbar: TreeScrollbarPolicy) -> Self {
        self.scrollbar = scrollbar;
        self
    }

    /// Selects animated or virtualized rendering.
    pub fn render_mode(mut self, render_mode: TreeViewRenderMode) -> Self {
        self.render_mode = render_mode;
        self
    }

    /// Enables or disables the folder reveal transition.
    pub fn animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }

    /// Sets the folder reveal duration, clamped to one millisecond.
    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = duration.max(Duration::from_millis(1));
        self
    }

    /// Sets the folder reveal duration in milliseconds.
    pub fn duration_ms(self, duration_ms: u32) -> Self {
        self.duration(Duration::from_millis(u64::from(duration_ms)))
    }

    /// Sets the folder reveal easing.
    pub fn easing(mut self, easing: crate::CollapsibleEasing) -> Self {
        self.easing = easing;
        self
    }

    /// Sets one callback for toggle, select, load, hover, and context actions.
    pub fn on_action<F>(mut self, callback: F) -> Self
    where
        F: Fn(TreeViewAction) -> Message + 'a,
    {
        self.on_action = Some(Rc::new(callback));
        self
    }

    /// Sets or clears the action callback.
    pub fn on_action_maybe<F>(mut self, callback: Option<F>) -> Self
    where
        F: Fn(TreeViewAction) -> Message + 'a,
    {
        self.on_action = callback.map(|callback| Rc::new(callback) as _);
        self
    }

    /// Replaces built-in icons with application-provided iced content.
    pub fn icon_renderer<F>(mut self, renderer: F) -> Self
    where
        F: Fn(TreeIconKey, Color, f32) -> Element<'a, Message> + 'a,
    {
        self.icon_renderer = Some(Rc::new(renderer));
        self
    }

    /// Builds the tree as an iced element.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        render::build_tree_view(self)
    }
}

impl<'a, Message> From<TreeView<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(tree: TreeView<'a, Message>) -> Self {
        tree.into_element()
    }
}

/// Converts a [`TreeView`] builder into an iced element.
pub fn tree_view<'a, Message>(tree: TreeViewBuilder<'a, Message>) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    tree.into_element()
}
