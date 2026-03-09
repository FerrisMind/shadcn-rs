pub mod state;
pub mod widget;

pub use state::{FlatNode, FolderState, TreeViewerState};
pub use widget::{TreeViewer, TreeViewerProps};

/// Convenience helper to create a tree viewer widget.
pub fn tree_viewer<'a, Message: Clone + 'a>(
    state: &'a TreeViewerState,
    on_toggle: impl Fn(String) -> Message + 'a,
    on_select: impl Fn(String) -> Message + 'a,
    on_load: impl Fn(String) -> Message + 'a,
    on_hover: impl Fn(Option<String>) -> Message + 'a,
    on_context: impl Fn(String) -> Message + 'a,
    props: TreeViewerProps,
    theme: &'a crate::theme::Theme,
) -> TreeViewer<'a, Message> {
    TreeViewer::new(
        state, on_toggle, on_select, on_load, on_hover, on_context, props, theme,
    )
}
