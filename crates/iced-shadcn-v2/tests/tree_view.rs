use iced::Element;
use iced_shadcn_v2::{
    FolderState, Theme, TreeIconKey, TreeNode, TreeNodeId, TreeSelectionMode, TreeView,
    TreeViewAction, TreeViewBuildError, TreeViewRenderMode, TreeViewState,
};

fn id(value: &str) -> TreeNodeId {
    TreeNodeId::new(value).expect("test ids are non-empty")
}

fn roots() -> Vec<TreeNode> {
    vec![
        TreeNode::folder(id("src"), "src")
            .children([TreeNode::file(id("src/lib"), "lib.rs").into_node()])
            .into_node(),
        TreeNode::file(id("readme"), "README.md")
            .with_icon(TreeIconKey::named("markdown"))
            .into_node(),
    ]
}

#[derive(Clone)]
struct Message;

#[test]
fn tree_view_defaults_match_the_source_and_uses_borrowed_inputs() {
    let theme = Theme::light();
    let roots = roots();
    let state = TreeViewState::new();
    let tree = TreeView::<Message>::new(&theme, &roots, &state).expect("valid tree");

    assert_eq!(tree.configured_render_mode(), TreeViewRenderMode::Animated);
    assert_eq!(tree.selection_mode(), TreeSelectionMode::None);
    assert_eq!(tree.configured_row_height(), 20.0);
    assert_eq!(tree.configured_indent(), 26.0);
    assert!(tree.is_animated());
}

#[test]
fn invalid_layout_values_are_reported_without_panicking() {
    let theme = Theme::light();
    let roots = roots();
    let state = TreeViewState::new();
    let tree = TreeView::<Message>::new(&theme, &roots, &state).expect("valid tree");

    assert_eq!(
        tree.row_height(0.0).expect_err("zero rows are invalid"),
        TreeViewBuildError::InvalidMeasurement(iced_shadcn_v2::TreeViewMeasurement::RowHeight)
    );
}

#[test]
fn duplicate_ids_are_rejected_at_the_component_boundary() {
    let theme = Theme::light();
    let roots = vec![
        TreeNode::file(id("same"), "one").into_node(),
        TreeNode::file(id("same"), "two").into_node(),
    ];

    assert!(matches!(
        TreeView::<Message>::new(&theme, &roots, &TreeViewState::new()),
        Err(TreeViewBuildError::DuplicateId(_))
    ));
}

#[test]
fn builder_exposes_controlled_actions_and_virtualized_mode() {
    let theme = Theme::light();
    let roots = roots();
    let state = TreeViewState::new();
    let tree = TreeView::<Message>::new(&theme, &roots, &state)
        .expect("valid tree")
        .selection(TreeSelectionMode::Single)
        .render_mode(TreeViewRenderMode::Virtualized)
        .on_action(|_action: TreeViewAction| Message);

    let _: Element<'_, Message> = tree.into();
}

#[test]
fn lazy_folder_state_is_part_of_the_shared_model() {
    let folder = TreeNode::unloaded_folder(id("packages"), "packages").into_node();

    assert_eq!(folder.folder_state(), Some(FolderState::Unloaded));
}
