use shadcn_common::{
    FolderState, TreeNode, TreeNodeId, TreeViewAction, TreeViewState, flatten_visible,
    flatten_visible_ordered, truncate_tree_label, validate_tree,
};

fn id(value: &str) -> TreeNodeId {
    TreeNodeId::new(value).expect("test ids are non-empty")
}

fn sample_tree() -> Vec<TreeNode> {
    vec![
        TreeNode::folder(id("src"), "src")
            .children([
                TreeNode::file(id("src/lib"), "lib.rs").into_node(),
                TreeNode::folder(id("src/ui"), "ui")
                    .children([TreeNode::file(id("src/ui/button"), "button.rs").into_node()])
                    .into_node(),
            ])
            .into_node(),
        TreeNode::file(id("readme"), "README.md").into_node(),
    ]
}

#[test]
fn ids_are_typed_and_reject_empty_values() {
    assert_eq!(id("src").as_str(), "src");
    assert!(TreeNodeId::new("").is_err());
    assert_eq!(id("src").to_string(), "src");
}

#[test]
fn typed_folder_and_file_builders_keep_variant_specific_configuration() {
    let folder = TreeNode::folder(id("src"), "src")
        .with_icons(
            shadcn_common::TreeIconKey::FolderOpen,
            shadcn_common::TreeIconKey::Folder,
        )
        .state(FolderState::Unloaded)
        .into_node();
    let file = TreeNode::file(id("readme"), "README.md")
        .with_icon(shadcn_common::TreeIconKey::File)
        .into_node();

    assert!(folder.is_folder());
    assert_eq!(folder.folder_state(), Some(FolderState::Unloaded));
    assert!(file.is_file());
    assert_eq!(file.label(), "README.md");
}

#[test]
fn default_state_matches_open_svelte_folders_and_preserves_source_order() {
    let tree = sample_tree();
    let rows = flatten_visible(&tree, &TreeViewState::new());

    assert_eq!(
        rows.iter().map(|row| row.label()).collect::<Vec<_>>(),
        ["src", "lib.rs", "ui", "button.rs", "README.md"]
    );
    assert_eq!(rows[1].depth(), 1);
    assert!(rows[1].guide_at(0));
    assert_eq!(rows[4].depth(), 0);
}

#[test]
fn collapsing_a_folder_hides_only_its_descendants() {
    let tree = sample_tree();
    let mut state = TreeViewState::new();
    state.collapse(&id("src"));

    let rows = flatten_visible(&tree, &state);

    assert_eq!(
        rows.iter().map(|row| row.id().as_str()).collect::<Vec<_>>(),
        ["src", "readme"]
    );
}

#[test]
fn selection_hover_and_context_are_controlled_by_state() {
    let mut state = TreeViewState::new();
    state.select(id("readme"));
    state.hover(id("src"));
    state.context(id("readme"));

    assert!(state.is_selected(&id("readme")));
    assert_eq!(state.hovered(), Some(&id("src")));
    assert_eq!(state.context_target(), Some(&id("readme")));

    state.clear_selection();
    assert!(state.selected().is_none());
}

#[test]
fn duplicate_ids_are_reported_before_rendering() {
    let nodes = vec![
        TreeNode::file(id("duplicate"), "a").into_node(),
        TreeNode::file(id("duplicate"), "b").into_node(),
    ];

    assert!(validate_tree(&nodes).is_err());
}

#[test]
fn actions_carry_stable_ids_instead_of_display_paths() {
    let action = TreeViewAction::Toggle(id("src"));

    assert_eq!(action.node_id(), Some(&id("src")));
}

#[test]
fn truncation_is_unicode_safe_and_uses_an_ellipsis() {
    assert_eq!(truncate_tree_label("日本語のファイル", 5), "日本語の…");
    assert_eq!(truncate_tree_label("filename", 0), "filename");
    assert_eq!(truncate_tree_label("filename", 2), "..");
}

#[test]
fn optional_folder_first_ordering_is_explicit() {
    let tree = vec![
        TreeNode::file(id("readme"), "README.md").into_node(),
        TreeNode::folder(id("src"), "src").into_node(),
    ];

    let rows = flatten_visible_ordered(
        &tree,
        &TreeViewState::new(),
        shadcn_common::TreeOrdering::FoldersFirst,
    );

    assert_eq!(
        rows.iter().map(|row| row.id().as_str()).collect::<Vec<_>>(),
        ["src", "readme"]
    );
}
