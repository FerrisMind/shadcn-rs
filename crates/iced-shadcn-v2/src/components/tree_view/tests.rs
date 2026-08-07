use shadcn_common::{TreeIconKey, TreeNode, TreeNodeId, TreeViewState};

use super::geometry;
use super::icon;
use super::{
    TreeSelectionMode, TreeView, TreeViewBuildError, TreeViewMeasurement, TreeViewRenderMode,
};

#[derive(Clone)]
struct Message;

fn id(value: &str) -> TreeNodeId {
    match TreeNodeId::new(value) {
        Ok(id) => id,
        Err(error) => panic!("test ID must be valid: {error}"),
    }
}

#[test]
fn builder_defaults_are_source_compatible() {
    let theme = crate::Theme::light();
    let roots = vec![TreeNode::file(id("readme"), "README.md").into_node()];
    let state = TreeViewState::new();
    let tree = TreeView::<Message>::new(&theme, &roots, &state).expect("valid test tree");

    assert_eq!(tree.configured_render_mode(), TreeViewRenderMode::Animated);
    assert_eq!(tree.selection_mode(), TreeSelectionMode::None);
    assert_eq!(tree.configured_row_height(), 20.0);
    assert_eq!(tree.configured_indent(), 26.0);
    assert!(tree.is_animated());
}

#[test]
fn measurement_validation_rejects_non_finite_values() {
    assert_eq!(
        geometry::positive(f32::NAN, TreeViewMeasurement::RowHeight),
        Err(TreeViewBuildError::InvalidMeasurement(
            TreeViewMeasurement::RowHeight
        ))
    );
    assert_eq!(
        geometry::non_negative(-1.0, TreeViewMeasurement::ContentOffset),
        Err(TreeViewBuildError::InvalidMeasurement(
            TreeViewMeasurement::ContentOffset
        ))
    );
}

#[test]
fn icon_fallbacks_preserve_named_and_folder_semantics() {
    assert_eq!(icon::glyph(&TreeIconKey::Folder, false), "▸");
    assert_eq!(icon::glyph(&TreeIconKey::Folder, true), "▾");
    assert_eq!(icon::glyph(&TreeIconKey::named("markdown"), false), "M");
}

#[test]
fn virtualized_mode_invokes_custom_icon_renderer() {
    use std::cell::Cell;
    use std::rc::Rc;

    use crate::iced_compat::widget::text;

    let theme = crate::Theme::light();
    let roots = vec![TreeNode::file(id("readme"), "README.md").into_node()];
    let state = TreeViewState::new();
    let calls = Rc::new(Cell::new(0));
    let counter = Rc::clone(&calls);

    let tree = TreeView::<Message>::new(&theme, &roots, &state)
        .expect("valid test tree")
        .render_mode(TreeViewRenderMode::Virtualized)
        .icon_renderer(move |key, color, size| {
            counter.set(counter.get() + 1);
            text(icon::glyph(&key, false))
                .size(size)
                .color(color)
                .into()
        });

    let _: crate::iced_compat::Element<'_, Message> = tree.into();
    assert!(
        calls.get() > 0,
        "virtualized mode must call the custom icon renderer"
    );
}
