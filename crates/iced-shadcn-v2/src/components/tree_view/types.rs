//! Public configuration types and builder storage for [`super::TreeView`].

use std::rc::Rc;
use std::time::Duration;

use crate::iced_compat::{Color, Element, Length};
use crate::theme::Theme;
use shadcn_common::{TreeIconKey, TreeNode, TreeOrdering, TreeViewAction, TreeViewState};

use crate::components::collapsible::CollapsibleEasing;

/// Controls whether file rows can emit selection actions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum TreeSelectionMode {
    /// Do not make file rows selectable.
    #[default]
    None,
    /// Allow one controlled selection at a time.
    Single,
}

/// Controls how keyboard events are interpreted by a tree renderer.
///
/// The default preserves the source component's ordinary button interaction
/// model. Arrow-key tree semantics are an explicit opt-in contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum TreeNavigationPolicy {
    /// Use ordinary button activation only.
    #[default]
    Basic,
    /// Enable the renderer's optional roving tree navigation.
    Full,
}

/// Controls the native scroll rail used by a tree.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum TreeScrollbarPolicy {
    /// Let iced reveal the rail as needed.
    #[default]
    Auto,
    /// Keep the vertical rail visible.
    Visible,
    /// Keep the vertical rail hidden while retaining scrolling.
    Hidden,
}

/// Selects the rendering strategy of one [`TreeView`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum TreeViewRenderMode {
    /// Compose rows from buttons and animated collapsible content.
    #[default]
    Animated,
    /// Draw only rows intersecting the scroll viewport.
    Virtualized,
}

/// Numeric measurement validated by a [`TreeView`](super::TreeView) builder.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum TreeViewMeasurement {
    /// Height of one visible row.
    RowHeight,
    /// Horizontal nesting offset per depth.
    Indent,
    /// Size of a row icon.
    IconSize,
    /// Size of row labels.
    TextSize,
    /// Extra horizontal inset before row content.
    ContentOffset,
    /// Maximum label width.
    MaxLabelWidth,
}

/// Callback type used to replace a built-in tree icon with native content.
///
/// Both animated and virtualized renderers honor this callback. Virtualized
/// mode composes flat iced rows when a custom renderer is present so Element
/// icons stay visible; without one it keeps the allocation-free draw path.
pub type TreeIconRenderer<'a, Message> =
    dyn Fn(TreeIconKey, Color, f32) -> Element<'a, Message> + 'a;

/// Builder-first controlled tree view.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct TreeView<'a, Message> {
    pub(super) theme: &'a Theme,
    pub(super) roots: &'a [TreeNode],
    pub(super) state: &'a TreeViewState,
    pub(super) width: Length,
    pub(super) height: Length,
    pub(super) row_height: f32,
    pub(super) indent: f32,
    pub(super) icon_size: f32,
    pub(super) text_size: f32,
    pub(super) content_offset: f32,
    pub(super) max_label_width: Option<f32>,
    pub(super) selection: TreeSelectionMode,
    pub(super) ordering: TreeOrdering,
    pub(super) navigation: TreeNavigationPolicy,
    pub(super) scrollbar: TreeScrollbarPolicy,
    pub(super) render_mode: TreeViewRenderMode,
    pub(super) animated: bool,
    pub(super) duration: Duration,
    pub(super) easing: CollapsibleEasing,
    pub(super) on_action: Option<Rc<dyn Fn(TreeViewAction) -> Message + 'a>>,
    pub(super) icon_renderer: Option<Rc<TreeIconRenderer<'a, Message>>>,
}

impl<'a, Message> TreeView<'a, Message> {
    pub(super) fn with_defaults(
        theme: &'a Theme,
        roots: &'a [TreeNode],
        state: &'a TreeViewState,
    ) -> Self {
        Self {
            theme,
            roots,
            state,
            width: Length::Fill,
            height: Length::Fill,
            row_height: 20.0,
            indent: 26.0,
            icon_size: 16.0,
            text_size: 14.0,
            content_offset: 0.0,
            max_label_width: None,
            selection: TreeSelectionMode::None,
            ordering: TreeOrdering::Source,
            navigation: TreeNavigationPolicy::Basic,
            scrollbar: TreeScrollbarPolicy::Auto,
            render_mode: TreeViewRenderMode::Animated,
            animated: true,
            duration: Duration::from_millis(200),
            easing: CollapsibleEasing::EaseInOut,
            on_action: None,
            icon_renderer: None,
        }
    }

    /// Borrows the root nodes supplied to the builder.
    #[must_use]
    pub const fn roots(&self) -> &'a [TreeNode] {
        self.roots
    }

    /// Borrows the controlled state supplied to the builder.
    #[must_use]
    pub const fn state(&self) -> &'a TreeViewState {
        self.state
    }

    /// Returns the active render strategy.
    #[must_use]
    pub const fn configured_render_mode(&self) -> TreeViewRenderMode {
        self.render_mode
    }

    /// Returns the configured selection mode.
    #[must_use]
    pub const fn selection_mode(&self) -> TreeSelectionMode {
        self.selection
    }

    /// Returns the configured row height.
    #[must_use]
    pub const fn configured_row_height(&self) -> f32 {
        self.row_height
    }

    /// Returns the configured indentation.
    #[must_use]
    pub const fn configured_indent(&self) -> f32 {
        self.indent
    }

    /// Returns the configured icon size.
    #[must_use]
    pub const fn configured_icon_size(&self) -> f32 {
        self.icon_size
    }

    /// Returns the configured text size.
    #[must_use]
    pub const fn configured_text_size(&self) -> f32 {
        self.text_size
    }

    /// Returns whether folder content reveal is animated.
    #[must_use]
    pub const fn is_animated(&self) -> bool {
        self.animated
    }

    /// Returns the configured ordering policy.
    #[must_use]
    pub const fn configured_ordering(&self) -> TreeOrdering {
        self.ordering
    }

    /// Returns the optional maximum label width.
    #[must_use]
    pub const fn configured_max_label_width(&self) -> Option<f32> {
        self.max_label_width
    }
}
