//! Backend-agnostic tree data, controlled state, and visible-row helpers.
//!
//! The model deliberately knows nothing about `iced` or `egui`. Renderers
//! borrow [`TreeNode`] values, resolve [`TreeIconKey`] values to native icons,
//! and turn [`TreeViewAction`] values into application messages.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

/// A stable identity for one tree node.
///
/// Display labels are intentionally kept separate from this value. This
/// prevents two files named `index.ts` in different folders from becoming the
/// same logical item merely because their labels match.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TreeNodeId(String);

impl TreeNodeId {
    /// Creates an ID from a non-empty string.
    ///
    /// # Errors
    ///
    /// Returns [`TreeNodeIdError::Empty`] when the value contains no
    /// non-whitespace characters.
    pub fn new(value: impl Into<String>) -> Result<Self, TreeNodeIdError> {
        let value = value.into();

        if value.trim().is_empty() {
            Err(TreeNodeIdError::Empty)
        } else {
            Ok(Self(value))
        }
    }

    /// Borrows the stable ID text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the newtype and returns its owned text.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for TreeNodeId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for TreeNodeId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl TryFrom<&str> for TreeNodeId {
    type Error = TreeNodeIdError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<String> for TreeNodeId {
    type Error = TreeNodeIdError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

/// Error returned when a tree ID cannot be constructed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TreeNodeIdError {
    /// The supplied ID was empty or contained only whitespace.
    Empty,
}

impl fmt::Display for TreeNodeIdError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("tree node id cannot be empty"),
        }
    }
}

impl std::error::Error for TreeNodeIdError {}

/// State of a folder's data source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FolderState {
    /// Children are not loaded yet.
    Unloaded,
    /// Children are currently being loaded by the application.
    Loading,
    /// Children are available locally.
    #[default]
    Loaded,
}

/// Backend-neutral icon choices used by tree nodes.
///
/// The built-in variants cover the reference file-tree icons. [`Self::Named`]
/// carries a backend-neutral name for an application-provided icon resolver;
/// it does not require either GUI backend in `shadcn-common`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum TreeIconKey {
    /// Closed-folder icon.
    Folder,
    /// Open-folder icon.
    FolderOpen,
    /// File icon.
    File,
    /// Loading indicator icon.
    Loader,
    /// An application-defined icon name.
    Named(String),
}

impl TreeIconKey {
    /// Creates an application-defined icon name.
    #[must_use]
    pub fn named(name: impl Into<String>) -> Self {
        Self::Named(name.into())
    }
}

/// A configured folder node.
///
/// Folder-only methods live on this type, so applying a folder icon or loading
/// state to a file cannot silently do nothing.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TreeFolder {
    id: TreeNodeId,
    label: String,
    children: Vec<TreeNode>,
    icon_open: TreeIconKey,
    icon_closed: TreeIconKey,
    icon_loading: TreeIconKey,
    state: FolderState,
    disabled: bool,
}

impl TreeFolder {
    /// Creates a loaded folder with the reference's default icons.
    #[must_use]
    pub fn new(id: TreeNodeId, label: impl Into<String>) -> Self {
        Self {
            id,
            label: label.into(),
            children: Vec::new(),
            icon_open: TreeIconKey::FolderOpen,
            icon_closed: TreeIconKey::Folder,
            icon_loading: TreeIconKey::Loader,
            state: FolderState::Loaded,
            disabled: false,
        }
    }

    /// Replaces the folder's children while preserving source order.
    #[must_use]
    pub fn children(mut self, children: impl IntoIterator<Item = TreeNode>) -> Self {
        self.children = children.into_iter().collect();
        self
    }

    /// Appends one child to the folder.
    #[must_use]
    pub fn push(mut self, child: impl Into<TreeNode>) -> Self {
        self.children.push(child.into());
        self
    }

    /// Appends all children from an iterator.
    #[must_use]
    pub fn extend(self, children: impl IntoIterator<Item = TreeNode>) -> Self {
        children.into_iter().fold(self, Self::push)
    }

    /// Sets the folder's loading state.
    #[must_use]
    pub fn state(mut self, state: FolderState) -> Self {
        self.state = state;
        self
    }

    /// Configures the open and closed folder icons.
    #[must_use]
    pub fn with_icons(mut self, open: TreeIconKey, closed: TreeIconKey) -> Self {
        self.icon_open = open;
        self.icon_closed = closed;
        self
    }

    /// Configures the icon shown while the folder is loading.
    #[must_use]
    pub fn with_loading_icon(mut self, icon: TreeIconKey) -> Self {
        self.icon_loading = icon;
        self
    }

    /// Disables the folder trigger.
    #[must_use]
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Borrows the stable node ID.
    #[must_use]
    pub fn id(&self) -> &TreeNodeId {
        &self.id
    }

    /// Borrows the display label.
    #[must_use]
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Borrows the folder children in source order.
    #[must_use]
    pub fn children_ref(&self) -> &[TreeNode] {
        &self.children
    }

    /// Returns the folder data state.
    #[must_use]
    pub const fn folder_state(&self) -> FolderState {
        self.state
    }

    /// Returns whether the folder is disabled.
    #[must_use]
    pub const fn is_disabled(&self) -> bool {
        self.disabled
    }

    /// Returns the icon used when the folder is open.
    #[must_use]
    pub const fn icon_open(&self) -> &TreeIconKey {
        &self.icon_open
    }

    /// Returns the icon used when the folder is closed.
    #[must_use]
    pub const fn icon_closed(&self) -> &TreeIconKey {
        &self.icon_closed
    }

    /// Returns the icon used while the folder is loading.
    #[must_use]
    pub const fn icon_loading(&self) -> &TreeIconKey {
        &self.icon_loading
    }

    /// Converts this configured folder into the canonical node enum.
    #[must_use]
    pub fn into_node(self) -> TreeNode {
        self.into()
    }
}

/// A configured file node.
///
/// Files are leaves by construction; child-oriented configuration is not
/// available on this type.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TreeFile {
    id: TreeNodeId,
    label: String,
    icon: TreeIconKey,
    disabled: bool,
}

impl TreeFile {
    /// Creates a file with the reference's default file icon.
    #[must_use]
    pub fn new(id: TreeNodeId, label: impl Into<String>) -> Self {
        Self {
            id,
            label: label.into(),
            icon: TreeIconKey::File,
            disabled: false,
        }
    }

    /// Configures the file icon.
    #[must_use]
    pub fn with_icon(mut self, icon: TreeIconKey) -> Self {
        self.icon = icon;
        self
    }

    /// Disables the file button.
    #[must_use]
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Borrows the stable node ID.
    #[must_use]
    pub fn id(&self) -> &TreeNodeId {
        &self.id
    }

    /// Borrows the display label.
    #[must_use]
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Returns the configured file icon.
    #[must_use]
    pub const fn icon(&self) -> &TreeIconKey {
        &self.icon
    }

    /// Returns whether the file is disabled.
    #[must_use]
    pub const fn is_disabled(&self) -> bool {
        self.disabled
    }

    /// Converts this configured file into the canonical node enum.
    #[must_use]
    pub fn into_node(self) -> TreeNode {
        self.into()
    }
}

/// One folder or file in a tree.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum TreeNode {
    /// A folder with ordered children.
    Folder(TreeFolder),
    /// A leaf file.
    File(TreeFile),
}

impl TreeNode {
    /// Starts a typed folder configuration.
    #[must_use]
    pub fn folder(id: TreeNodeId, label: impl Into<String>) -> TreeFolder {
        TreeFolder::new(id, label)
    }

    /// Starts an unloaded typed folder configuration.
    #[must_use]
    pub fn unloaded_folder(id: TreeNodeId, label: impl Into<String>) -> TreeFolder {
        TreeFolder::new(id, label).state(FolderState::Unloaded)
    }

    /// Starts a typed file configuration.
    #[must_use]
    pub fn file(id: TreeNodeId, label: impl Into<String>) -> TreeFile {
        TreeFile::new(id, label)
    }

    /// Borrows this node's stable ID.
    #[must_use]
    pub fn id(&self) -> &TreeNodeId {
        match self {
            Self::Folder(folder) => folder.id(),
            Self::File(file) => file.id(),
        }
    }

    /// Borrows this node's display label.
    #[must_use]
    pub fn label(&self) -> &str {
        match self {
            Self::Folder(folder) => folder.label(),
            Self::File(file) => file.label(),
        }
    }

    /// Returns whether this node is a folder.
    #[must_use]
    pub const fn is_folder(&self) -> bool {
        matches!(self, Self::Folder(_))
    }

    /// Returns whether this node is a file.
    #[must_use]
    pub const fn is_file(&self) -> bool {
        matches!(self, Self::File(_))
    }

    /// Borrows children when this node is a folder.
    #[must_use]
    pub fn children(&self) -> Option<&[TreeNode]> {
        match self {
            Self::Folder(folder) => Some(folder.children_ref()),
            Self::File(_) => None,
        }
    }

    /// Returns the folder data state, or `None` for files.
    #[must_use]
    pub const fn folder_state(&self) -> Option<FolderState> {
        match self {
            Self::Folder(folder) => Some(folder.folder_state()),
            Self::File(_) => None,
        }
    }

    /// Returns whether the node is disabled.
    #[must_use]
    pub const fn is_disabled(&self) -> bool {
        match self {
            Self::Folder(folder) => folder.is_disabled(),
            Self::File(file) => file.is_disabled(),
        }
    }

    /// Returns the open folder icon, if this is a folder.
    #[must_use]
    pub fn folder_icon_open(&self) -> Option<&TreeIconKey> {
        match self {
            Self::Folder(folder) => Some(folder.icon_open()),
            Self::File(_) => None,
        }
    }

    /// Returns the closed folder icon, if this is a folder.
    #[must_use]
    pub fn folder_icon_closed(&self) -> Option<&TreeIconKey> {
        match self {
            Self::Folder(folder) => Some(folder.icon_closed()),
            Self::File(_) => None,
        }
    }

    /// Returns the loading folder icon, if this is a folder.
    #[must_use]
    pub fn folder_icon_loading(&self) -> Option<&TreeIconKey> {
        match self {
            Self::Folder(folder) => Some(folder.icon_loading()),
            Self::File(_) => None,
        }
    }

    /// Returns the file icon, if this is a file.
    #[must_use]
    pub fn file_icon(&self) -> Option<&TreeIconKey> {
        match self {
            Self::Folder(_) => None,
            Self::File(file) => Some(file.icon()),
        }
    }
}

impl From<TreeFolder> for TreeNode {
    fn from(folder: TreeFolder) -> Self {
        Self::Folder(folder)
    }
}

impl From<TreeFile> for TreeNode {
    fn from(file: TreeFile) -> Self {
        Self::File(file)
    }
}

/// Controlled expansion, selection, hover, context, and focus state.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TreeViewState {
    expanded: BTreeMap<TreeNodeId, bool>,
    default_open: bool,
    selected: Option<TreeNodeId>,
    hovered: Option<TreeNodeId>,
    context_target: Option<TreeNodeId>,
    focused: Option<TreeNodeId>,
}

impl Default for TreeViewState {
    fn default() -> Self {
        Self {
            expanded: BTreeMap::new(),
            default_open: true,
            selected: None,
            hovered: None,
            context_target: None,
            focused: None,
        }
    }
}

impl TreeViewState {
    /// Creates state matching the Svelte reference's open folders.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates state with an explicit default expansion policy.
    #[must_use]
    pub fn with_default_open(default_open: bool) -> Self {
        Self {
            default_open,
            ..Self::default()
        }
    }

    /// Returns the default expansion policy for folders without an override.
    #[must_use]
    pub const fn default_open(&self) -> bool {
        self.default_open
    }

    /// Returns whether a folder is currently expanded.
    #[must_use]
    pub fn is_expanded(&self, id: &TreeNodeId) -> bool {
        self.expanded.get(id).copied().unwrap_or(self.default_open)
    }

    /// Sets a folder's controlled expansion state.
    pub fn set_expanded(&mut self, id: TreeNodeId, expanded: bool) {
        self.expanded.insert(id, expanded);
    }

    /// Toggles a folder and returns its next expansion state.
    pub fn toggle(&mut self, id: &TreeNodeId) -> bool {
        let expanded = !self.is_expanded(id);
        self.set_expanded(id.clone(), expanded);
        expanded
    }

    /// Expands one folder.
    pub fn expand(&mut self, id: &TreeNodeId) {
        self.set_expanded(id.clone(), true);
    }

    /// Collapses one folder.
    pub fn collapse(&mut self, id: &TreeNodeId) {
        self.set_expanded(id.clone(), false);
    }

    /// Expands every folder reachable from `roots`.
    pub fn expand_all(&mut self, roots: &[TreeNode]) {
        for node in roots {
            if let Some(children) = node.children() {
                self.expand(node.id());
                self.expand_all(children);
            }
        }
    }

    /// Creates state with every folder reachable from `roots` expanded.
    #[must_use]
    pub fn all_expanded(roots: &[TreeNode]) -> Self {
        let mut state = Self::with_default_open(false);
        state.expand_all(roots);
        state
    }

    /// Collapses every folder reachable from `roots`.
    pub fn collapse_all(&mut self, roots: &[TreeNode]) {
        for node in roots {
            if let Some(children) = node.children() {
                self.collapse(node.id());
                self.collapse_all(children);
            }
        }
    }

    /// Borrows the selected node ID, if any.
    #[must_use]
    pub fn selected(&self) -> Option<&TreeNodeId> {
        self.selected.as_ref()
    }

    /// Sets the selected node ID.
    pub fn select(&mut self, id: TreeNodeId) {
        self.selected = Some(id);
    }

    /// Clears the current selection.
    pub fn clear_selection(&mut self) {
        self.selected = None;
    }

    /// Returns whether `id` is selected.
    #[must_use]
    pub fn is_selected(&self, id: &TreeNodeId) -> bool {
        self.selected.as_ref() == Some(id)
    }

    /// Records the currently hovered node.
    pub fn hover(&mut self, id: TreeNodeId) {
        self.hovered = Some(id);
    }

    /// Clears the hover target.
    pub fn clear_hover(&mut self) {
        self.hovered = None;
    }

    /// Borrows the hovered node ID, if any.
    #[must_use]
    pub fn hovered(&self) -> Option<&TreeNodeId> {
        self.hovered.as_ref()
    }

    /// Records the node receiving a context interaction.
    pub fn context(&mut self, id: TreeNodeId) {
        self.context_target = Some(id);
    }

    /// Clears the context target.
    pub fn clear_context(&mut self) {
        self.context_target = None;
    }

    /// Borrows the context target ID, if any.
    #[must_use]
    pub fn context_target(&self) -> Option<&TreeNodeId> {
        self.context_target.as_ref()
    }

    /// Records the focused node for an optional keyboard navigation policy.
    pub fn focus(&mut self, id: TreeNodeId) {
        self.focused = Some(id);
    }

    /// Clears the focused node.
    pub fn clear_focus(&mut self) {
        self.focused = None;
    }

    /// Borrows the focused node ID, if any.
    #[must_use]
    pub fn focused(&self) -> Option<&TreeNodeId> {
        self.focused.as_ref()
    }
}

/// Action emitted by a tree renderer for the application to handle.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum TreeViewAction {
    /// Toggle one folder's expansion state.
    Toggle(TreeNodeId),
    /// Select one node.
    Select(TreeNodeId),
    /// Request children for an unloaded folder.
    Load(TreeNodeId),
    /// Report the current hover target.
    Hover(Option<TreeNodeId>),
    /// Report a context interaction on one node.
    Context(TreeNodeId),
}

impl TreeViewAction {
    /// Returns the node ID carried by this action, when there is one.
    #[must_use]
    pub fn node_id(&self) -> Option<&TreeNodeId> {
        match self {
            Self::Toggle(id) | Self::Select(id) | Self::Load(id) | Self::Context(id) => Some(id),
            Self::Hover(id) => id.as_ref(),
        }
    }
}

/// Ordering policy for visible tree rows.
///
/// [`Self::Source`] is the default and preserves the input order exactly,
/// matching the shadcn-svelte source. The other policies are explicit opt-ins
/// for applications that want the ordering used by older iced tree widgets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum TreeOrdering {
    /// Preserve the order supplied by the application.
    #[default]
    Source,
    /// Put folders before files while preserving stable order within groups.
    FoldersFirst,
    /// Sort labels lexicographically while preserving stable ties.
    Label,
}

/// Error found while validating a tree before rendering.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TreeValidationError {
    /// Two nodes use the same stable ID.
    DuplicateId(TreeNodeId),
    /// An unloaded folder contains children that are not available yet.
    UnloadedFolderHasChildren(TreeNodeId),
}

impl fmt::Display for TreeValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateId(id) => write!(formatter, "duplicate tree node id: {id}"),
            Self::UnloadedFolderHasChildren(id) => {
                write!(formatter, "unloaded folder has children: {id}")
            }
        }
    }
}

impl std::error::Error for TreeValidationError {}

/// Validates IDs and lazy-folder invariants before a tree is rendered.
///
/// # Errors
///
/// Returns the first duplicate ID or invalid unloaded folder encountered in
/// source order.
pub fn validate_tree(roots: &[TreeNode]) -> Result<(), TreeValidationError> {
    let mut ids = BTreeSet::new();
    validate_nodes(roots, &mut ids)
}

fn validate_nodes(
    nodes: &[TreeNode],
    ids: &mut BTreeSet<TreeNodeId>,
) -> Result<(), TreeValidationError> {
    for node in nodes {
        if !ids.insert(node.id().clone()) {
            return Err(TreeValidationError::DuplicateId(node.id().clone()));
        }

        if let Some(children) = node.children() {
            if node.folder_state() == Some(FolderState::Unloaded) && !children.is_empty() {
                return Err(TreeValidationError::UnloadedFolderHasChildren(
                    node.id().clone(),
                ));
            }

            validate_nodes(children, ids)?;
        }
    }

    Ok(())
}

/// One visible row produced by [`flatten_visible`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VisibleTreeNode<'a> {
    node: &'a TreeNode,
    depth: usize,
    guides: Vec<bool>,
    expanded: bool,
}

impl<'a> VisibleTreeNode<'a> {
    /// Borrows the source node.
    #[must_use]
    pub fn node(&self) -> &'a TreeNode {
        self.node
    }

    /// Borrows the stable node ID.
    #[must_use]
    pub fn id(&self) -> &'a TreeNodeId {
        self.node.id()
    }

    /// Borrows the display label.
    #[must_use]
    pub fn label(&self) -> &'a str {
        self.node.label()
    }

    /// Returns the nesting depth, where roots are zero.
    #[must_use]
    pub const fn depth(&self) -> usize {
        self.depth
    }

    /// Returns whether this row represents a folder.
    #[must_use]
    pub const fn is_folder(&self) -> bool {
        self.node.is_folder()
    }

    /// Returns whether this folder row is expanded.
    #[must_use]
    pub const fn is_expanded(&self) -> bool {
        self.expanded
    }

    /// Returns whether a guide line should be drawn at an ancestor depth.
    #[must_use]
    pub fn guide_at(&self, depth: usize) -> bool {
        self.guides.get(depth).copied().unwrap_or(false)
    }

    /// Borrows the complete ancestor guide metadata.
    #[must_use]
    pub fn guides(&self) -> &[bool] {
        &self.guides
    }
}

/// Flattens expanded branches into source-order visible rows.
///
/// The returned rows borrow the original nodes. Closed folders remain in the
/// result while their descendants are omitted, which lets a renderer build
/// either a recursive or virtualized presentation without cloning the model.
#[must_use]
pub fn flatten_visible<'a>(
    roots: &'a [TreeNode],
    state: &TreeViewState,
) -> Vec<VisibleTreeNode<'a>> {
    flatten_visible_ordered(roots, state, TreeOrdering::Source)
}

/// Flattens visible rows using an explicit ordering policy.
#[must_use]
pub fn flatten_visible_ordered<'a>(
    roots: &'a [TreeNode],
    state: &TreeViewState,
    ordering: TreeOrdering,
) -> Vec<VisibleTreeNode<'a>> {
    let ordered_roots = ordered_nodes(roots, ordering);
    let mut rows = Vec::new();
    flatten_nodes_visible(&ordered_roots, 0, &[], state, ordering, &mut rows);
    rows
}

fn ordered_nodes(nodes: &[TreeNode], ordering: TreeOrdering) -> Vec<&TreeNode> {
    let mut ordered = nodes.iter().collect::<Vec<_>>();

    match ordering {
        TreeOrdering::Source => {}
        TreeOrdering::FoldersFirst => {
            ordered.sort_by_key(|node| !node.is_folder());
        }
        TreeOrdering::Label => {
            ordered.sort_by(|left, right| left.label().cmp(right.label()));
        }
    }

    ordered
}

fn flatten_nodes_visible<'a>(
    nodes: &[&'a TreeNode],
    depth: usize,
    ancestor_guides: &[bool],
    state: &TreeViewState,
    ordering: TreeOrdering,
    rows: &mut Vec<VisibleTreeNode<'a>>,
) {
    for (index, node) in nodes.iter().enumerate() {
        let has_next_sibling = index + 1 < nodes.len();
        let expanded = if node.is_folder() {
            state.is_expanded(node.id())
        } else {
            false
        };

        rows.push(VisibleTreeNode {
            node,
            depth,
            guides: ancestor_guides.to_vec(),
            expanded,
        });

        if expanded && let Some(children) = node.children() {
            let mut child_guides = ancestor_guides.to_vec();
            child_guides.push(has_next_sibling);
            let ordered_children = ordered_nodes(children, ordering);
            flatten_nodes_visible(
                &ordered_children,
                depth + 1,
                &child_guides,
                state,
                ordering,
                rows,
            );
        }
    }
}

/// Truncates a label by Unicode scalar values without splitting UTF-8.
///
/// Values up to three characters use dots to preserve the compact behavior of
/// the original iced tree. Larger values reserve one character for `…`.
#[must_use]
pub fn truncate_tree_label(label: &str, max_chars: usize) -> String {
    let char_count = label.chars().count();

    if max_chars == 0 || char_count <= max_chars {
        return label.to_owned();
    }

    if max_chars <= 3 {
        return ".".repeat(max_chars);
    }

    let prefix: String = label.chars().take(max_chars - 1).collect();
    format!("{prefix}…")
}
