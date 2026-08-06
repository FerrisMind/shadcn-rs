//! Public configuration types for the resizable component.

use std::fmt;

use crate::iced_compat::Element;

/// Split axis of a [`super::ResizablePaneGroup`].
///
/// Mirrors the `direction` prop of paneforge `PaneGroup` (`horizontal` /
/// `vertical`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ResizableDirection {
    /// Panes are arranged left-to-right with vertical handles.
    #[default]
    Horizontal,
    /// Panes are arranged top-to-bottom with horizontal handles.
    Vertical,
}

impl ResizableDirection {
    /// Returns `true` when panes stack horizontally.
    pub const fn is_horizontal(self) -> bool {
        matches!(self, Self::Horizontal)
    }

    /// Returns `true` when panes stack vertically.
    pub const fn is_vertical(self) -> bool {
        matches!(self, Self::Vertical)
    }
}

/// Percentage sizes of every pane in a group, summing to `100`.
///
/// Returned by layout-change callbacks and accepted by
/// [`super::ResizablePaneGroup::sizes`].
#[derive(Debug, Clone, PartialEq)]
pub struct ResizableLayout(pub Vec<f32>);

impl ResizableLayout {
    /// Creates a layout from percentage sizes.
    ///
    /// Values are normalized to sum to `100` when possible.
    pub fn new(mut sizes: Vec<f32>) -> Self {
        geometry::normalize_layout(&mut sizes);
        Self(sizes)
    }

    /// Percentage sizes, one entry per pane.
    pub fn as_slice(&self) -> &[f32] {
        &self.0
    }

    /// Mutable access to the underlying percentages.
    pub fn as_mut_slice(&mut self) -> &mut [f32] {
        &mut self.0
    }

    /// Consumes the layout and returns the inner vector.
    pub fn into_inner(self) -> Vec<f32> {
        self.0
    }
}

impl From<Vec<f32>> for ResizableLayout {
    fn from(sizes: Vec<f32>) -> Self {
        Self::new(sizes)
    }
}

impl<const N: usize> From<[f32; N]> for ResizableLayout {
    fn from(sizes: [f32; N]) -> Self {
        Self::new(sizes.to_vec())
    }
}

/// Corner radius preset for a pane-group frame.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ResizableRadius {
    /// No rounding (`rounded-none`).
    None,
    /// Pack `radius.md` token.
    #[default]
    Medium,
    /// Pack `radius.lg` token (matches the shadcn examples).
    Large,
    /// Explicit radius in pixels.
    Px(f32),
}

/// Per-pane constraints mirrored from paneforge `Pane` props.
#[derive(Debug, Clone, Copy)]
pub(super) struct PaneConstraints {
    pub(super) default_size: f32,
    pub(super) min_size: f32,
    pub(super) max_size: f32,
    pub(super) collapsed_size: f32,
    pub(super) collapsible: bool,
    pub(super) collapsed: bool,
    pub(super) order: Option<u32>,
}

impl Default for PaneConstraints {
    fn default() -> Self {
        Self {
            default_size: 50.0,
            min_size: 0.0,
            max_size: 100.0,
            collapsed_size: 0.0,
            collapsible: false,
            collapsed: false,
            order: None,
        }
    }
}

/// Handle appearance and interaction flags.
#[derive(Debug, Clone, Copy, Default)]
pub(super) struct HandleConfig {
    pub(super) with_handle: bool,
    pub(super) disabled: bool,
}

/// Draggable divider between two panes.
#[must_use = "builders do nothing unless added to a ResizablePaneGroup"]
pub struct ResizableHandle {
    pub(super) config: HandleConfig,
}

impl fmt::Debug for ResizableHandle {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ResizableHandle")
            .field("config", &self.config)
            .finish_non_exhaustive()
    }
}

impl Default for ResizableHandle {
    fn default() -> Self {
        Self::new()
    }
}

impl ResizableHandle {
    /// Creates a handle with the default shadcn divider styling.
    pub fn new() -> Self {
        Self {
            config: HandleConfig::default(),
        }
    }

    /// Paints the grip icon from `.cn-resizable-handle-icon`.
    pub fn with_handle(mut self, with_handle: bool) -> Self {
        self.config.with_handle = with_handle;
        self
    }

    /// Disables pointer resizing on this handle.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.config.disabled = disabled;
        self
    }
}

/// One resizable pane slot.
#[must_use = "builders do nothing unless added to a ResizablePaneGroup"]
pub struct ResizablePane<'a, Message> {
    pub(super) content: Element<'a, Message>,
    pub(super) constraints: PaneConstraints,
}

impl<Message> fmt::Debug for ResizablePane<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ResizablePane")
            .field("constraints", &self.constraints)
            .finish_non_exhaustive()
    }
}

impl<'a, Message> ResizablePane<'a, Message> {
    /// Creates a pane around arbitrary content.
    pub fn new(content: impl Into<Element<'a, Message>>) -> Self {
        Self {
            content: content.into(),
            constraints: PaneConstraints::default(),
        }
    }

    /// Sets the default size in percent of the group (`defaultSize`).
    pub fn default_size(mut self, size: f32) -> Self {
        self.constraints.default_size = geometry::normalize_percent(size);
        self
    }

    /// Sets the minimum size in percent (`minSize`).
    pub fn min_size(mut self, size: f32) -> Self {
        self.constraints.min_size = geometry::normalize_percent(size);
        self
    }

    /// Sets the maximum size in percent (`maxSize`).
    pub fn max_size(mut self, size: f32) -> Self {
        self.constraints.max_size = geometry::normalize_percent(size);
        self
    }

    /// Allows the pane to collapse to [`Self::collapsed_size`].
    pub fn collapsible(mut self, collapsible: bool) -> Self {
        self.constraints.collapsible = collapsible;
        self
    }

    /// Size in percent when collapsed (`collapsedSize`).
    pub fn collapsed_size(mut self, size: f32) -> Self {
        self.constraints.collapsed_size = geometry::normalize_percent(size);
        self
    }

    /// Controlled collapsed state.
    pub fn collapsed(mut self, collapsed: bool) -> Self {
        self.constraints.collapsed = collapsed;
        self
    }

    /// Ordering hint when panes are conditionally rendered (`order`).
    pub fn order(mut self, order: u32) -> Self {
        self.constraints.order = Some(order);
        self
    }
}

use super::geometry;
