pub mod state;
pub mod widget;

pub use state::{FlatNode, FolderState, TreeViewerState};
pub use widget::{TreeViewer, TreeViewerHandlers, TreeViewerProps};

/// Convenience helper to create a tree viewer widget.
pub fn tree_viewer<'a, Message: Clone + 'a>(
    state: &'a TreeViewerState,
    context_path: Option<String>,
    handlers: TreeViewerHandlers<'a, Message>,
    props: TreeViewerProps,
    theme: &'a crate::theme::Theme,
) -> TreeViewer<'a, Message> {
    TreeViewer::new(state, context_path, handlers, props, theme)
}
