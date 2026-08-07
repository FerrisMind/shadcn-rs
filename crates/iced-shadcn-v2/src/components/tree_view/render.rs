//! Rendering paths for the unified tree builder.

use std::rc::Rc;

use crate::components::button::{Button, ButtonSize};
use crate::components::collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger};
use crate::components::scroll_area::{ScrollArea, ScrollAreaScrollbar};
use crate::iced_compat::widget::{MouseArea, Space, column, container, row, text};
use crate::iced_compat::{Background, Element, Length, alignment};
use crate::{Padding, Spacing};
use shadcn_common::{
    FolderState, TreeIconKey, TreeNode, TreeOrdering, TreeViewAction, TreeViewState,
    flatten_visible_ordered,
};

use super::geometry;
use super::icon;
use super::style;
use super::types::TreeIconRenderer;
use super::{
    TreeNavigationPolicy, TreeScrollbarPolicy, TreeSelectionMode, TreeView, TreeViewRenderMode,
};

fn tree_row_padding() -> Padding {
    Padding::all(Spacing::S0)
}

pub(super) fn build_tree_view<'a, Message>(tree: TreeView<'a, Message>) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let TreeView {
        theme,
        roots,
        state,
        width,
        height,
        row_height,
        indent,
        icon_size,
        text_size,
        content_offset,
        max_label_width,
        selection,
        ordering,
        navigation,
        scrollbar,
        render_mode,
        animated,
        duration,
        easing,
        on_action,
        icon_renderer,
    } = tree;

    match render_mode {
        TreeViewRenderMode::Animated => build_animated(
            theme,
            roots,
            state,
            width,
            height,
            row_height,
            indent,
            icon_size,
            text_size,
            content_offset,
            max_label_width,
            selection,
            ordering,
            navigation,
            scrollbar,
            animated,
            duration,
            easing,
            on_action,
            icon_renderer,
        ),
        TreeViewRenderMode::Virtualized => build_virtualized(
            theme,
            roots,
            state,
            width,
            height,
            row_height,
            indent,
            icon_size,
            text_size,
            content_offset,
            max_label_width,
            selection,
            ordering,
            navigation,
            scrollbar,
            on_action,
            icon_renderer,
        ),
    }
}

#[allow(clippy::too_many_arguments)]
fn build_animated<'a, Message>(
    theme: &'a crate::theme::Theme,
    roots: &'a [TreeNode],
    state: &'a TreeViewState,
    width: Length,
    height: Length,
    row_height: f32,
    indent: f32,
    icon_size: f32,
    text_size: f32,
    content_offset: f32,
    max_label_width: Option<f32>,
    selection: TreeSelectionMode,
    ordering: TreeOrdering,
    _navigation: TreeNavigationPolicy,
    scrollbar: TreeScrollbarPolicy,
    animated: bool,
    duration: std::time::Duration,
    easing: crate::components::collapsible::CollapsibleEasing,
    on_action: Option<Rc<dyn Fn(TreeViewAction) -> Message + 'a>>,
    icon_renderer: Option<Rc<TreeIconRenderer<'a, Message>>>,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let nodes = ordered_nodes(roots, ordering);
    let rows = build_animated_nodes(
        nodes,
        theme,
        state,
        0,
        ordering,
        row_height,
        indent,
        icon_size,
        text_size,
        content_offset,
        max_label_width,
        selection,
        animated,
        duration,
        easing,
        on_action.as_ref(),
        icon_renderer.as_ref(),
    );
    let body: Element<'a, Message> = column(rows).spacing(0).width(Length::Fill).into();
    build_scroll_area(body, theme, width, height, scrollbar)
}

#[allow(clippy::too_many_arguments)]
fn build_animated_nodes<'a, Message>(
    nodes: Vec<&'a TreeNode>,
    theme: &'a crate::theme::Theme,
    state: &'a TreeViewState,
    depth: usize,
    ordering: TreeOrdering,
    row_height: f32,
    indent: f32,
    icon_size: f32,
    text_size: f32,
    content_offset: f32,
    max_label_width: Option<f32>,
    selection: TreeSelectionMode,
    animated: bool,
    duration: std::time::Duration,
    easing: crate::components::collapsible::CollapsibleEasing,
    on_action: Option<&Rc<dyn Fn(TreeViewAction) -> Message + 'a>>,
    icon_renderer: Option<&Rc<TreeIconRenderer<'a, Message>>>,
) -> Vec<Element<'a, Message>>
where
    Message: Clone + 'a,
{
    nodes
        .into_iter()
        .map(|node| match node {
            TreeNode::Folder(folder) => {
                let expanded = state.is_expanded(folder.id());
                let row = build_row(
                    node,
                    expanded,
                    theme,
                    depth,
                    row_height,
                    indent,
                    icon_size,
                    text_size,
                    content_offset,
                    max_label_width,
                    selection == TreeSelectionMode::Single && state.is_selected(folder.id()),
                    on_action,
                    icon_renderer,
                );
                let trigger = CollapsibleTrigger::new(row, theme)
                    .size(ButtonSize::Default)
                    .height(Length::Fixed(row_height))
                    .full_width(true)
                    .variant(style::row_variant(false))
                    .padding(tree_row_padding())
                    .expect("Spacing::S0 padding is always resolvable")
                    .disabled(
                        folder.is_disabled()
                            || matches!(folder.folder_state(), FolderState::Loading),
                    );

                let children = if folder.folder_state() == FolderState::Loaded {
                    build_animated_nodes(
                        ordered_nodes(folder.children_ref(), ordering),
                        theme,
                        state,
                        0,
                        ordering,
                        row_height,
                        indent,
                        icon_size,
                        text_size,
                        content_offset,
                        max_label_width,
                        selection,
                        animated,
                        duration,
                        easing,
                        on_action,
                        icon_renderer,
                    )
                } else {
                    Vec::new()
                };

                let content = CollapsibleContent::with_children(
                    theme,
                    [guide_content(theme, children, indent)],
                )
                .spacing(0.0)
                .width(Length::Fill);
                let mut collapsible = Collapsible::new(theme)
                    .open(expanded)
                    .spacing(0.0)
                    .animated(animated)
                    .duration(duration)
                    .easing(easing)
                    .trigger(trigger)
                    .content(content);

                if let Some(callback) = on_action {
                    let id = folder.id().clone();
                    let callback = Rc::clone(callback);
                    collapsible = collapsible
                        .on_open_change(move |_| callback(TreeViewAction::Toggle(id.clone())));
                }

                if matches!(folder.folder_state(), FolderState::Unloaded)
                    && let Some(callback) = on_action
                {
                    let callback = Rc::clone(callback);
                    // The trigger callback is replaced below by an explicit
                    // load button so an unloaded folder never reports a
                    // misleading toggle action.
                    return build_load_folder(
                        node,
                        theme,
                        depth,
                        row_height,
                        indent,
                        icon_size,
                        text_size,
                        content_offset,
                        max_label_width,
                        selection == TreeSelectionMode::Single && state.is_selected(folder.id()),
                        callback,
                        icon_renderer,
                    );
                }

                collapsible.into()
            }
            TreeNode::File(_) => build_file_row(
                node,
                theme,
                depth,
                row_height,
                indent,
                icon_size,
                text_size,
                content_offset,
                max_label_width,
                selection == TreeSelectionMode::Single && state.is_selected(node.id()),
                selection,
                on_action,
                icon_renderer,
            ),
            _ => container(text("Unsupported tree node")).into(),
        })
        .collect()
}

#[allow(clippy::too_many_arguments)]
fn build_load_folder<'a, Message>(
    node: &'a TreeNode,
    theme: &'a crate::theme::Theme,
    depth: usize,
    row_height: f32,
    indent: f32,
    icon_size: f32,
    text_size: f32,
    content_offset: f32,
    max_label_width: Option<f32>,
    selected: bool,
    on_action: Rc<dyn Fn(TreeViewAction) -> Message + 'a>,
    icon_renderer: Option<&Rc<TreeIconRenderer<'a, Message>>>,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let row = build_row(
        node,
        false,
        theme,
        depth,
        row_height,
        indent,
        icon_size,
        text_size,
        content_offset,
        max_label_width,
        selected,
        Some(&on_action),
        icon_renderer,
    );
    Button::new(row, theme)
        .variant(style::row_variant(false))
        .size(ButtonSize::Default)
        .full_width()
        .height(Length::Fixed(row_height))
        .padding(tree_row_padding())
        .expect("Spacing::S0 padding is always resolvable")
        .on_press(on_action(TreeViewAction::Load(node.id().clone())))
        .into()
}

#[allow(clippy::too_many_arguments)]
fn build_file_row<'a, Message>(
    node: &'a TreeNode,
    theme: &'a crate::theme::Theme,
    depth: usize,
    row_height: f32,
    indent: f32,
    icon_size: f32,
    text_size: f32,
    content_offset: f32,
    max_label_width: Option<f32>,
    selected: bool,
    selection: TreeSelectionMode,
    on_action: Option<&Rc<dyn Fn(TreeViewAction) -> Message + 'a>>,
    icon_renderer: Option<&Rc<TreeIconRenderer<'a, Message>>>,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let row = build_row(
        node,
        selected,
        theme,
        depth,
        row_height,
        indent,
        icon_size,
        text_size,
        content_offset,
        max_label_width,
        selected,
        on_action,
        icon_renderer,
    );
    if selection != TreeSelectionMode::Single || on_action.is_none() {
        return container(row)
            .width(Length::Fill)
            .height(Length::Fixed(row_height))
            .into();
    }

    let mut button = Button::new(row, theme)
        .variant(style::row_variant(selected))
        .size(ButtonSize::Default)
        .full_width()
        .height(Length::Fixed(row_height))
        .padding(tree_row_padding())
        .expect("Spacing::S0 padding is always resolvable")
        .disabled(node.is_disabled());

    if selection == TreeSelectionMode::Single
        && !node.is_disabled()
        && let Some(callback) = on_action
    {
        button = button.on_press(callback(TreeViewAction::Select(node.id().clone())));
    }
    button.into()
}

#[allow(clippy::too_many_arguments)]
fn build_row<'a, Message>(
    node: &'a TreeNode,
    expanded: bool,
    theme: &'a crate::theme::Theme,
    depth: usize,
    row_height: f32,
    indent: f32,
    icon_size: f32,
    text_size: f32,
    content_offset: f32,
    max_label_width: Option<f32>,
    selected: bool,
    on_action: Option<&Rc<dyn Fn(TreeViewAction) -> Message + 'a>>,
    icon_renderer: Option<&Rc<TreeIconRenderer<'a, Message>>>,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let disabled = node.is_disabled();
    let icon_key = match node {
        TreeNode::Folder(folder) => match folder.folder_state() {
            FolderState::Loading => folder.icon_loading().clone(),
            FolderState::Loaded if expanded => folder.icon_open().clone(),
            FolderState::Loaded | FolderState::Unloaded => folder.icon_closed().clone(),
        },
        TreeNode::File(file) => file.icon().clone(),
        _ => TreeIconKey::File,
    };
    let icon_color = style::icon_color(theme, selected, disabled);
    let icon_element = if let Some(renderer) = icon_renderer {
        renderer(icon_key.clone(), icon_color, icon_size)
    } else {
        icon::element(icon_key, icon_size, icon_color)
    };
    let label = geometry::label_for_width(node.label(), max_label_width, text_size);
    let label = text(label)
        .size(text_size)
        .font(crate::fonts::iced_font(theme.font_pack().sans))
        .color(style::text_color(theme, selected, disabled))
        .width(Length::Fill)
        .align_y(alignment::Vertical::Center);
    let file_offset = if node.is_file() { 3.0 } else { 0.0 };

    let row: Element<'a, Message> = row::Row::new()
        .push(
            container(text(""))
                .width(Length::Fixed(
                    content_offset + depth as f32 * indent + file_offset,
                ))
                .height(Length::Fixed(row_height)),
        )
        .push(icon_element)
        .push(label)
        .spacing(4.0)
        .align_y(alignment::Vertical::Center)
        .width(Length::Fill)
        .height(Length::Fixed(row_height))
        .into();

    if let Some(callback) = on_action {
        let id = node.id().clone();
        MouseArea::new(row)
            .on_enter(callback(TreeViewAction::Hover(Some(id.clone()))))
            .on_exit(callback(TreeViewAction::Hover(None)))
            .on_right_press(callback(TreeViewAction::Context(id)))
            .into()
    } else {
        row
    }
}

fn guide_content<'a, Message: 'a>(
    theme: &'a crate::theme::Theme,
    children: Vec<Element<'a, Message>>,
    indent: f32,
) -> Element<'a, Message> {
    let guide = container(Space::new())
        .width(Length::Fixed(1.0))
        .height(Length::Fill)
        .style(
            move |_iced_theme| crate::iced_compat::widget::container::Style {
                background: Some(Background::Color(theme.palette.border)),
                ..Default::default()
            },
        );

    row::Row::new()
        .push(Space::new().width(Length::Fixed(8.0)))
        .push(guide)
        .push(Space::new().width(Length::Fixed((indent - 9.0).max(0.0))))
        .push(column(children).spacing(0).width(Length::Fill))
        .align_y(alignment::Vertical::Top)
        .width(Length::Fill)
        .into()
}

fn ordered_nodes(nodes: &[TreeNode], ordering: TreeOrdering) -> Vec<&TreeNode> {
    let mut ordered = nodes.iter().collect::<Vec<_>>();
    match ordering {
        TreeOrdering::Source => {}
        TreeOrdering::FoldersFirst => ordered.sort_by_key(|node| !node.is_folder()),
        TreeOrdering::Label => ordered.sort_by(|left, right| left.label().cmp(right.label())),
        _ => {}
    }
    ordered
}

fn build_scroll_area<'a, Message: 'a>(
    body: Element<'a, Message>,
    theme: &'a crate::theme::Theme,
    width: Length,
    height: Length,
    scrollbar: TreeScrollbarPolicy,
) -> Element<'a, Message> {
    let rail = match scrollbar {
        TreeScrollbarPolicy::Hidden => ScrollAreaScrollbar::hidden(),
        TreeScrollbarPolicy::Auto | TreeScrollbarPolicy::Visible => ScrollAreaScrollbar::new(),
    };
    ScrollArea::new(body, theme)
        .width(width)
        .height(height)
        .vertical_scrollbar(rail)
        .into()
}

#[allow(clippy::too_many_arguments)]
fn build_virtualized<'a, Message>(
    theme: &'a crate::theme::Theme,
    roots: &'a [TreeNode],
    state: &'a TreeViewState,
    width: Length,
    height: Length,
    row_height: f32,
    indent: f32,
    icon_size: f32,
    text_size: f32,
    content_offset: f32,
    max_label_width: Option<f32>,
    selection: TreeSelectionMode,
    ordering: TreeOrdering,
    navigation: TreeNavigationPolicy,
    scrollbar: TreeScrollbarPolicy,
    on_action: Option<Rc<dyn Fn(TreeViewAction) -> Message + 'a>>,
    icon_renderer: Option<Rc<TreeIconRenderer<'a, Message>>>,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    // Element-based custom icons cannot be painted from the allocation-free
    // draw path, so virtualized mode falls back to a flat row list that still
    // uses the same depth/indent math and action contract.
    if icon_renderer.is_some() {
        let body = build_flat_virtualized_rows(
            theme,
            roots,
            state,
            row_height,
            indent,
            icon_size,
            text_size,
            content_offset,
            max_label_width,
            selection,
            ordering,
            on_action.as_ref(),
            icon_renderer.as_ref(),
        );
        return build_scroll_area(body, theme, width, height, scrollbar);
    }

    let virtual_tree = VirtualTree {
        theme,
        roots,
        state,
        _width: width,
        row_height,
        indent,
        icon_size,
        text_size,
        content_offset,
        max_label_width,
        selection,
        ordering,
        _navigation: navigation,
        on_action,
    };
    let child: Element<'a, Message> = virtual_tree.into();
    build_scroll_area(child, theme, width, height, scrollbar)
}

#[allow(clippy::too_many_arguments)]
fn build_flat_virtualized_rows<'a, Message>(
    theme: &'a crate::theme::Theme,
    roots: &'a [TreeNode],
    state: &'a TreeViewState,
    row_height: f32,
    indent: f32,
    icon_size: f32,
    text_size: f32,
    content_offset: f32,
    max_label_width: Option<f32>,
    selection: TreeSelectionMode,
    ordering: TreeOrdering,
    on_action: Option<&Rc<dyn Fn(TreeViewAction) -> Message + 'a>>,
    icon_renderer: Option<&Rc<TreeIconRenderer<'a, Message>>>,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let rows = flatten_visible_ordered(roots, state, ordering);
    let elements = rows
        .iter()
        .map(|visible| {
            let selected =
                selection == TreeSelectionMode::Single && state.is_selected(visible.id());
            match visible.node() {
                TreeNode::Folder(folder)
                    if matches!(folder.folder_state(), FolderState::Unloaded) =>
                {
                    if let Some(callback) = on_action {
                        build_load_folder(
                            visible.node(),
                            theme,
                            visible.depth(),
                            row_height,
                            indent,
                            icon_size,
                            text_size,
                            content_offset,
                            max_label_width,
                            selected,
                            Rc::clone(callback),
                            icon_renderer,
                        )
                    } else {
                        build_row(
                            visible.node(),
                            false,
                            theme,
                            visible.depth(),
                            row_height,
                            indent,
                            icon_size,
                            text_size,
                            content_offset,
                            max_label_width,
                            selected,
                            on_action,
                            icon_renderer,
                        )
                    }
                }
                TreeNode::Folder(_) => {
                    let row = build_row(
                        visible.node(),
                        visible.is_expanded(),
                        theme,
                        visible.depth(),
                        row_height,
                        indent,
                        icon_size,
                        text_size,
                        content_offset,
                        max_label_width,
                        selected,
                        on_action,
                        icon_renderer,
                    );
                    if let Some(callback) = on_action {
                        Button::new(row, theme)
                            .variant(style::row_variant(selected))
                            .size(ButtonSize::Default)
                            .full_width()
                            .height(Length::Fixed(row_height))
                            .padding(tree_row_padding())
                            .expect("Spacing::S0 padding is always resolvable")
                            .disabled(visible.node().is_disabled())
                            .on_press(callback(TreeViewAction::Toggle(visible.id().clone())))
                            .into()
                    } else {
                        container(row)
                            .width(Length::Fill)
                            .height(Length::Fixed(row_height))
                            .into()
                    }
                }
                TreeNode::File(_) => build_file_row(
                    visible.node(),
                    theme,
                    visible.depth(),
                    row_height,
                    indent,
                    icon_size,
                    text_size,
                    content_offset,
                    max_label_width,
                    selected,
                    selection,
                    on_action,
                    icon_renderer,
                ),
                _ => container(text("Unsupported tree node")).into(),
            }
        })
        .collect::<Vec<_>>();
    column(elements).spacing(0).width(Length::Fill).into()
}

/// The advanced-widget path keeps a full logical height for the scroll
/// container, but only paints rows intersecting iced's viewport. This is the
/// same model as the v1 tree viewer without exposing a second public widget.
struct VirtualTree<'a, Message> {
    theme: &'a crate::theme::Theme,
    roots: &'a [TreeNode],
    state: &'a TreeViewState,
    _width: Length,
    row_height: f32,
    indent: f32,
    icon_size: f32,
    text_size: f32,
    content_offset: f32,
    max_label_width: Option<f32>,
    selection: TreeSelectionMode,
    ordering: TreeOrdering,
    _navigation: TreeNavigationPolicy,
    on_action: Option<Rc<dyn Fn(TreeViewAction) -> Message + 'a>>,
}

impl<'a, Message> From<VirtualTree<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(tree: VirtualTree<'a, Message>) -> Self {
        Self::new(tree)
    }
}

impl<'a, Message> VirtualTree<'a, Message> {
    fn rows(&self) -> Vec<shadcn_common::VisibleTreeNode<'a>> {
        flatten_visible_ordered(self.roots, self.state, self.ordering)
    }

    fn total_height(&self) -> f32 {
        (self.rows().len() as f32 + 1.0) * self.row_height
    }

    fn row_index_at(
        &self,
        bounds: crate::iced_compat::Rectangle,
        cursor: crate::iced_compat::Point,
    ) -> Option<usize> {
        if !bounds.contains(cursor) {
            return None;
        }
        let relative_y = cursor.y - bounds.y;
        let index = (relative_y / self.row_height).floor() as usize;
        (index < self.rows().len()).then_some(index)
    }

    fn keyboard_action(
        &self,
        rows: &[shadcn_common::VisibleTreeNode<'a>],
        key: &iced_core::keyboard::Key,
    ) -> Option<TreeViewAction> {
        use iced_core::keyboard::key::Named;

        let current = self
            .state
            .focused()
            .or_else(|| self.state.selected())
            .and_then(|id| rows.iter().position(|row| row.id() == id))
            .unwrap_or(0)
            .min(rows.len().saturating_sub(1));

        let target = match key {
            iced_core::keyboard::Key::Named(Named::ArrowDown) => {
                Some(current.saturating_add(1).min(rows.len().saturating_sub(1)))
            }
            iced_core::keyboard::Key::Named(Named::ArrowUp) => Some(current.saturating_sub(1)),
            iced_core::keyboard::Key::Named(Named::Home) => Some(0),
            iced_core::keyboard::Key::Named(Named::End) => Some(rows.len().saturating_sub(1)),
            _ => None,
        };

        if let Some(index) = target {
            if rows.is_empty() || self.selection != TreeSelectionMode::Single {
                return None;
            }
            return rows
                .get(index)
                .map(|row| TreeViewAction::Select(row.id().clone()));
        }

        let row = rows.get(current)?;
        match key {
            iced_core::keyboard::Key::Named(Named::Enter)
            | iced_core::keyboard::Key::Named(Named::Space) => {
                if row.is_folder() {
                    match row.node().folder_state() {
                        Some(FolderState::Unloaded) => Some(TreeViewAction::Load(row.id().clone())),
                        _ => Some(TreeViewAction::Toggle(row.id().clone())),
                    }
                } else if self.selection == TreeSelectionMode::Single {
                    Some(TreeViewAction::Select(row.id().clone()))
                } else {
                    None
                }
            }
            iced_core::keyboard::Key::Named(Named::ArrowRight)
                if row.is_folder() && !row.is_expanded() =>
            {
                Some(TreeViewAction::Toggle(row.id().clone()))
            }
            iced_core::keyboard::Key::Named(Named::ArrowLeft)
                if row.is_folder() && row.is_expanded() =>
            {
                Some(TreeViewAction::Toggle(row.id().clone()))
            }
            _ => None,
        }
    }
}

impl<'a, Message>
    crate::iced_compat::advanced::Widget<
        Message,
        crate::iced_compat::Theme,
        crate::iced_compat::Renderer,
    > for VirtualTree<'a, Message>
where
    Message: Clone,
{
    fn size(&self) -> crate::iced_compat::Size<Length> {
        crate::iced_compat::Size {
            width: Length::Fill,
            height: Length::Fixed(self.total_height()),
        }
    }

    fn layout(
        &mut self,
        _tree: &mut crate::iced_compat::advanced::widget::Tree,
        _renderer: &crate::iced_compat::Renderer,
        limits: &crate::iced_compat::advanced::layout::Limits,
    ) -> crate::iced_compat::advanced::layout::Node {
        let size = limits.resolve(
            Length::Fill,
            Length::Fixed(self.total_height()),
            crate::iced_compat::Size::ZERO,
        );
        crate::iced_compat::advanced::layout::Node::new(size)
    }

    fn draw(
        &self,
        _tree: &crate::iced_compat::advanced::widget::Tree,
        renderer: &mut crate::iced_compat::Renderer,
        _iced_theme: &crate::iced_compat::Theme,
        _style: &crate::iced_compat::advanced::renderer::Style,
        layout: crate::iced_compat::advanced::layout::Layout<'_>,
        cursor: crate::iced_compat::mouse::Cursor,
        viewport: &crate::iced_compat::Rectangle,
    ) {
        use iced_core::text::{LineHeight, Shaping, Wrapping};
        use iced_core::{Renderer as _, text::Renderer as _};

        let bounds = layout.bounds();
        let rows = self.rows();
        let relative_viewport = crate::iced_compat::Rectangle {
            x: viewport.x - bounds.x,
            y: viewport.y - bounds.y,
            width: viewport.width,
            height: viewport.height,
        };
        let first = (relative_viewport.y.max(0.0) / self.row_height).floor() as usize;
        let last = ((relative_viewport.y + relative_viewport.height).max(0.0) / self.row_height)
            .ceil() as usize;

        for (index, row) in rows
            .iter()
            .enumerate()
            .skip(first)
            .take(last.saturating_sub(first))
        {
            let row_bounds = crate::iced_compat::Rectangle {
                x: bounds.x,
                y: bounds.y + index as f32 * self.row_height,
                width: bounds.width,
                height: self.row_height,
            };
            if !row_bounds.intersects(viewport) {
                continue;
            }
            let clip = intersection(row_bounds, *viewport);
            if clip.width <= 0.0 || clip.height <= 0.0 {
                continue;
            }

            let hovered = cursor.position_over(row_bounds).is_some();
            let selected =
                self.selection == TreeSelectionMode::Single && self.state.is_selected(row.id());
            let row_surface = if selected {
                Some(self.theme.palette.secondary)
            } else if hovered {
                Some(self.theme.palette.accent)
            } else {
                None
            };
            let highlight = crate::iced_compat::Rectangle {
                x: row_bounds.x + self.content_offset,
                y: row_bounds.y,
                width: (row_bounds.width - self.content_offset * 2.0).max(0.0),
                height: row_bounds.height,
            };
            if let Some(color) = row_surface {
                renderer.fill_quad(
                    crate::iced_compat::advanced::renderer::Quad {
                        bounds: highlight,
                        border: crate::iced_compat::Border {
                            radius: self.theme.style.radius.md_px.into(),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    color,
                );
            }

            for guide_depth in 0..row.depth() {
                if !row.guide_at(guide_depth) {
                    continue;
                }
                let guide_x = bounds.x
                    + self.content_offset
                    + guide_depth as f32 * self.indent
                    + self.icon_size * 0.5;
                renderer.fill_quad(
                    crate::iced_compat::advanced::renderer::Quad {
                        bounds: crate::iced_compat::Rectangle {
                            x: guide_x.floor(),
                            y: row_bounds.y,
                            width: 1.0,
                            height: row_bounds.height,
                        },
                        ..Default::default()
                    },
                    self.theme.palette.border,
                );
            }

            let file_offset = if !row.is_folder() { 3.0 } else { 0.0 };
            let left =
                bounds.x + self.content_offset + row.depth() as f32 * self.indent + file_offset;
            let icon_key = match row.node() {
                TreeNode::Folder(folder) => match folder.folder_state() {
                    FolderState::Loading => folder.icon_loading().clone(),
                    FolderState::Loaded if row.is_expanded() => folder.icon_open().clone(),
                    FolderState::Loaded | FolderState::Unloaded => folder.icon_closed().clone(),
                },
                TreeNode::File(file) => file.icon().clone(),
                _ => TreeIconKey::File,
            };
            let icon_color = style::icon_color(self.theme, selected, row.node().is_disabled());
            let icon = icon::glyph(&icon_key, row.is_expanded());
            renderer.fill_text(
                iced_core::Text {
                    content: icon.to_owned(),
                    bounds: crate::iced_compat::Size::new(self.icon_size, self.row_height),
                    size: crate::iced_compat::Pixels(self.icon_size),
                    line_height: LineHeight::Absolute(crate::iced_compat::Pixels(self.icon_size)),
                    font: crate::fonts::iced_font(self.theme.font_pack().sans),
                    align_x: iced_core::text::Alignment::Center,
                    align_y: crate::iced_compat::alignment::Vertical::Center,
                    shaping: Shaping::Basic,
                    wrapping: Wrapping::None,
                },
                crate::iced_compat::Point::new(
                    left + self.icon_size * 0.5,
                    row_bounds.y + self.row_height * 0.5,
                ),
                icon_color,
                clip,
            );

            let text_x = left + self.icon_size + 4.0;
            let available = (row_bounds.width - (text_x - row_bounds.x) - 4.0).max(1.0);
            let label = geometry::label_for_width(
                row.label(),
                self.max_label_width
                    .map(|max| max.min(available))
                    .or(Some(available)),
                self.text_size,
            );
            renderer.fill_text(
                iced_core::Text {
                    content: label,
                    bounds: crate::iced_compat::Size::new(available, self.row_height),
                    size: crate::iced_compat::Pixels(self.text_size),
                    line_height: LineHeight::Absolute(crate::iced_compat::Pixels(self.text_size)),
                    font: crate::fonts::iced_font(self.theme.font_pack().sans),
                    align_x: iced_core::text::Alignment::Left,
                    align_y: crate::iced_compat::alignment::Vertical::Center,
                    shaping: Shaping::Advanced,
                    wrapping: Wrapping::None,
                },
                crate::iced_compat::Point::new(text_x, row_bounds.y + self.row_height * 0.5),
                style::text_color(self.theme, selected, row.node().is_disabled()),
                clip,
            );
        }
    }

    fn update(
        &mut self,
        _tree: &mut crate::iced_compat::advanced::widget::Tree,
        event: &crate::iced_compat::Event,
        layout: crate::iced_compat::advanced::layout::Layout<'_>,
        cursor: crate::iced_compat::mouse::Cursor,
        _renderer: &crate::iced_compat::Renderer,
        _clipboard: &mut dyn crate::iced_compat::advanced::Clipboard,
        shell: &mut crate::iced_compat::advanced::Shell<'_, Message>,
        _viewport: &crate::iced_compat::Rectangle,
    ) {
        let Some(callback) = self.on_action.as_ref() else {
            return;
        };
        let rows = self.rows();

        match event {
            crate::iced_compat::Event::Keyboard(iced_core::keyboard::Event::KeyPressed {
                key,
                ..
            }) if self._navigation == TreeNavigationPolicy::Full => {
                if let Some(action) = self.keyboard_action(&rows, key) {
                    shell.publish(callback(action));
                    shell.capture_event();
                }
            }
            crate::iced_compat::Event::Mouse(crate::iced_compat::mouse::Event::CursorMoved {
                ..
            }) => {
                if cursor.is_over(layout.bounds()) {
                    shell.request_redraw();
                }
                let hovered = cursor
                    .position_over(layout.bounds())
                    .and_then(|position| self.row_index_at(layout.bounds(), position))
                    .and_then(|index| rows.get(index).map(|row| row.id().clone()));
                shell.publish(callback(TreeViewAction::Hover(hovered)));
            }
            crate::iced_compat::Event::Mouse(crate::iced_compat::mouse::Event::ButtonPressed(
                crate::iced_compat::mouse::Button::Left,
            )) => {
                let Some(position) = cursor.position_over(layout.bounds()) else {
                    return;
                };
                let Some(index) = self.row_index_at(layout.bounds(), position) else {
                    return;
                };
                let Some(row) = rows.get(index) else {
                    return;
                };
                let action = if row.is_folder() {
                    match row.node().folder_state() {
                        Some(FolderState::Unloaded) => TreeViewAction::Load(row.id().clone()),
                        _ => TreeViewAction::Toggle(row.id().clone()),
                    }
                } else if self.selection == TreeSelectionMode::Single && !row.node().is_disabled() {
                    TreeViewAction::Select(row.id().clone())
                } else {
                    return;
                };
                shell.publish(callback(action));
            }
            crate::iced_compat::Event::Mouse(crate::iced_compat::mouse::Event::ButtonPressed(
                crate::iced_compat::mouse::Button::Right,
            )) => {
                let Some(position) = cursor.position_over(layout.bounds()) else {
                    return;
                };
                let Some(index) = self.row_index_at(layout.bounds(), position) else {
                    return;
                };
                if let Some(row) = rows.get(index) {
                    shell.publish(callback(TreeViewAction::Context(row.id().clone())));
                }
            }
            _ => {}
        }
    }

    fn mouse_interaction(
        &self,
        _state: &crate::iced_compat::advanced::widget::Tree,
        layout: crate::iced_compat::advanced::layout::Layout<'_>,
        cursor: crate::iced_compat::mouse::Cursor,
        _viewport: &crate::iced_compat::Rectangle,
        _renderer: &crate::iced_compat::Renderer,
    ) -> crate::iced_compat::mouse::Interaction {
        if cursor
            .position_over(layout.bounds())
            .and_then(|position| self.row_index_at(layout.bounds(), position))
            .is_some()
        {
            crate::iced_compat::mouse::Interaction::Pointer
        } else {
            crate::iced_compat::mouse::Interaction::default()
        }
    }
}

fn intersection(
    left: crate::iced_compat::Rectangle,
    right: crate::iced_compat::Rectangle,
) -> crate::iced_compat::Rectangle {
    let x1 = left.x.max(right.x);
    let y1 = left.y.max(right.y);
    let x2 = (left.x + left.width).min(right.x + right.width);
    let y2 = (left.y + left.height).min(right.y + right.height);
    crate::iced_compat::Rectangle {
        x: x1,
        y: y1,
        width: (x2 - x1).max(0.0),
        height: (y2 - y1).max(0.0),
    }
}
