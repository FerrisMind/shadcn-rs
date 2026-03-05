use iced::alignment::Vertical;
use iced::border::Border;
use iced::widget::{
    Space, button as iced_button, column, container, row, rule, scrollable, stack, text,
};
use iced::{Background, Color, Element, Font, Length, Padding};
use lucide_icons::Icon as LucideIcon;

use crate::theme::Theme;

// ---------------------------------------------------------------------------
// TreeNode – declarative tree data model
// ---------------------------------------------------------------------------

/// A single node in the tree.  Can be either a folder (with children) or a
/// file (leaf).
#[derive(Clone, Debug)]
pub enum TreeNode {
    Folder {
        name: String,
        children: Vec<TreeNode>,
        icon_open: Option<LucideIcon>,
        icon_closed: Option<LucideIcon>,
    },
    File {
        name: String,
        icon: Option<LucideIcon>,
    },
}

impl TreeNode {
    /// Convenience constructor for a folder.
    pub fn folder(name: impl Into<String>, children: Vec<TreeNode>) -> Self {
        Self::Folder {
            name: name.into(),
            children,
            icon_open: None,
            icon_closed: None,
        }
    }

    /// Convenience constructor for a file.
    pub fn file(name: impl Into<String>) -> Self {
        Self::File {
            name: name.into(),
            icon: None,
        }
    }

    /// Set a custom icon for the file node.
    pub fn with_icon(mut self, icon: LucideIcon) -> Self {
        match &mut self {
            Self::File { icon: i, .. } => *i = Some(icon),
            Self::Folder { .. } => {}
        }
        self
    }

    /// Set custom icons for the folder node (open / closed states).
    pub fn with_folder_icons(mut self, open: LucideIcon, closed: LucideIcon) -> Self {
        match &mut self {
            Self::Folder {
                icon_open,
                icon_closed,
                ..
            } => {
                *icon_open = Some(open);
                *icon_closed = Some(closed);
            }
            Self::File { .. } => {}
        }
        self
    }

    fn name(&self) -> &str {
        match self {
            Self::Folder { name, .. } | Self::File { name, .. } => name,
        }
    }
}

// ---------------------------------------------------------------------------
// TreeViewState – tracks expand/collapse & selection
// ---------------------------------------------------------------------------

/// Persistent state for the tree: which folders are open and which file is
/// selected.  Keep this in your application `struct`.
#[derive(Clone, Debug, Default)]
pub struct TreeViewState {
    /// Set of folder paths (joined with `/`) that are currently expanded.
    pub open_folders: Vec<String>,
    /// Path to the currently-selected file, if any.
    pub selected: Option<String>,
}

impl TreeViewState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Create state with all folders matching `paths` expanded.
    pub fn with_open(paths: Vec<String>) -> Self {
        Self {
            open_folders: paths,
            selected: None,
        }
    }

    pub fn is_open(&self, path: &str) -> bool {
        self.open_folders.iter().any(|p| p == path)
    }

    pub fn toggle_folder(&mut self, path: &str) {
        if let Some(idx) = self.open_folders.iter().position(|p| p == path) {
            self.open_folders.remove(idx);
        } else {
            self.open_folders.push(path.to_string());
        }
    }

    pub fn select(&mut self, path: &str) {
        self.selected = Some(path.to_string());
    }

    pub fn is_selected(&self, path: &str) -> bool {
        self.selected.as_deref() == Some(path)
    }

    /// Expand all folders in the tree.
    pub fn expand_all(nodes: &[TreeNode]) -> Self {
        let mut paths = Vec::new();
        collect_folder_paths(nodes, "", &mut paths);
        Self {
            open_folders: paths,
            selected: None,
        }
    }
}

fn collect_folder_paths(nodes: &[TreeNode], prefix: &str, out: &mut Vec<String>) {
    for node in nodes {
        if let TreeNode::Folder { name, children, .. } = node {
            let path = if prefix.is_empty() {
                name.clone()
            } else {
                format!("{prefix}/{name}")
            };
            out.push(path.clone());
            collect_folder_paths(children, &path, out);
        }
    }
}

// ---------------------------------------------------------------------------
// Props
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug)]
pub struct TreeViewProps {
    /// Indent per nesting level in pixels.
    pub indent: f32,
    /// Icon size in pixels.
    pub icon_size: f32,
    /// Font size for labels.
    pub font_size: f32,
    /// Row height.
    pub row_height: f32,
    /// Whether file clicks emit messages.
    pub selectable: bool,
    /// Max characters before label is truncated with "…".
    pub max_label_chars: usize,
    /// Extra left shift for row content (independent from hover background).
    pub content_offset: f32,
    /// Scrollbar visibility behavior for the tree viewport.
    pub scrollbar_visibility: TreeScrollbarVisibility,
}

impl Default for TreeViewProps {
    fn default() -> Self {
        Self {
            indent: 16.0,
            icon_size: 16.0,
            font_size: 13.0,
            row_height: 28.0,
            selectable: true,
            max_label_chars: 30,
            content_offset: 0.0,
            scrollbar_visibility: TreeScrollbarVisibility::Auto,
        }
    }
}

impl TreeViewProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn indent(mut self, indent: f32) -> Self {
        self.indent = indent;
        self
    }

    pub fn icon_size(mut self, size: f32) -> Self {
        self.icon_size = size;
        self
    }

    pub fn font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    pub fn row_height(mut self, height: f32) -> Self {
        self.row_height = height;
        self
    }

    pub fn selectable(mut self, selectable: bool) -> Self {
        self.selectable = selectable;
        self
    }

    pub fn max_label_chars(mut self, n: usize) -> Self {
        self.max_label_chars = n;
        self
    }

    pub fn content_offset(mut self, offset: f32) -> Self {
        self.content_offset = offset.max(0.0);
        self
    }

    pub fn scrollbar_visibility(mut self, visibility: TreeScrollbarVisibility) -> Self {
        self.scrollbar_visibility = visibility;
        self
    }
}

/// Vertical scrollbar visibility policy for the tree view.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TreeScrollbarVisibility {
    /// Hidden by default, visible on hover/drag.
    Auto,
    /// Always visible.
    Visible,
    /// Always hidden.
    Hidden,
}

// ---------------------------------------------------------------------------
// Messages the tree can emit
// ---------------------------------------------------------------------------

/// Messages produced by the tree view.  Map these in your application `update`.
#[derive(Clone, Debug)]
pub enum TreeViewAction {
    /// A folder was toggled (path).
    ToggleFolder(String),
    /// A file was selected (path).
    SelectFile(String),
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn truncate_ellipsis(s: &str, max_chars: usize) -> String {
    if max_chars == 0 || s.chars().count() <= max_chars {
        return s.to_string();
    }
    if max_chars <= 3 {
        return ".".repeat(max_chars);
    }
    let truncated: String = s.chars().take(max_chars - 3).collect();
    format!("{truncated}...")
}

fn apply_opacity(color: Color, opacity: f32) -> Color {
    Color {
        a: color.a * opacity,
        ..color
    }
}

fn transparent_rail() -> scrollable::Rail {
    scrollable::Rail {
        background: Some(Background::Color(Color::TRANSPARENT)),
        border: Border::default(),
        scroller: scrollable::Scroller {
            background: Background::Color(Color::TRANSPARENT),
            border: Border::default(),
        },
    }
}

fn visible_vertical_rail(theme: &Theme, thumb_opacity: f32) -> scrollable::Rail {
    let radius = theme.radius.sm;

    scrollable::Rail {
        background: Some(Background::Color(apply_opacity(theme.palette.muted, 0.18))),
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: radius.into(),
        },
        scroller: scrollable::Scroller {
            background: Background::Color(apply_opacity(
                theme.palette.muted_foreground,
                thumb_opacity,
            )),
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: radius.into(),
            },
        },
    }
}

fn transparent_auto_scroll() -> scrollable::AutoScroll {
    scrollable::AutoScroll {
        background: Background::Color(Color::TRANSPARENT),
        border: Border::default(),
        shadow: Default::default(),
        icon: Color::TRANSPARENT,
    }
}

fn tree_scroll_style(
    theme: &Theme,
    visibility: TreeScrollbarVisibility,
    status: scrollable::Status,
) -> scrollable::Style {
    let transparent = transparent_rail();
    let hidden_style = scrollable::Style {
        container: container::Style::default(),
        vertical_rail: transparent,
        horizontal_rail: transparent,
        gap: None,
        auto_scroll: transparent_auto_scroll(),
    };

    let (show_vertical, thumb_opacity, disabled) = match status {
        scrollable::Status::Active {
            is_vertical_scrollbar_disabled,
            ..
        } => (
            matches!(visibility, TreeScrollbarVisibility::Visible),
            0.48,
            is_vertical_scrollbar_disabled,
        ),
        scrollable::Status::Hovered {
            is_vertical_scrollbar_disabled,
            ..
        } => (
            !matches!(visibility, TreeScrollbarVisibility::Hidden),
            0.62,
            is_vertical_scrollbar_disabled,
        ),
        scrollable::Status::Dragged {
            is_vertical_scrollbar_disabled,
            ..
        } => (
            !matches!(visibility, TreeScrollbarVisibility::Hidden),
            0.74,
            is_vertical_scrollbar_disabled,
        ),
    };

    if matches!(visibility, TreeScrollbarVisibility::Hidden) || disabled {
        return hidden_style;
    }

    let vertical_rail = if show_vertical {
        visible_vertical_rail(theme, thumb_opacity)
    } else {
        transparent
    };

    scrollable::Style {
        container: container::Style::default(),
        vertical_rail,
        horizontal_rail: transparent,
        gap: None,
        auto_scroll: transparent_auto_scroll(),
    }
}

// ---------------------------------------------------------------------------
// Public render function
// ---------------------------------------------------------------------------

/// Render a tree view widget.
///
/// * `nodes`  – the tree data.
/// * `state`  – current expand/selection state.
/// * `on_action` – closure that wraps [`TreeViewAction`] into your app `Message`.
/// * `props`  – visual tuning knobs.
/// * `theme`  – shadcn theme.
pub fn tree_view<'a, Message: Clone + 'a>(
    nodes: Vec<TreeNode>,
    state: TreeViewState,
    on_action: impl Fn(TreeViewAction) -> Message + 'a + Clone,
    props: TreeViewProps,
    theme: &Theme,
) -> Element<'a, Message> {
    let mut col = column![].spacing(0).width(Length::Fill);

    for node in &nodes {
        col = col.push(render_node(node, &state, &on_action, props, theme, 0, ""));
    }

    let inner = container(col)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(Padding {
            top: 0.0,
            right: 0.0,
            bottom: 24.0,
            left: 0.0,
        });

    let scrollbar = scrollable::Scrollbar::new()
        .width(6)
        .scroller_width(6)
        .margin(0);

    let theme = theme.clone();
    scrollable(inner)
        .direction(scrollable::Direction::Vertical(scrollbar))
        .width(Length::Fill)
        .height(Length::Fill)
        .style(move |_iced_theme, status| {
            tree_scroll_style(&theme, props.scrollbar_visibility, status)
        })
        .into()
}

// ---------------------------------------------------------------------------
// Recursive rendering
// ---------------------------------------------------------------------------

fn render_node<'a, Message: Clone + 'a>(
    node: &TreeNode,
    state: &TreeViewState,
    on_action: &(impl Fn(TreeViewAction) -> Message + 'a + Clone),
    props: TreeViewProps,
    theme: &Theme,
    depth: usize,
    parent_path: &str,
) -> Element<'a, Message> {
    let path = if parent_path.is_empty() {
        node.name().to_string()
    } else {
        format!("{parent_path}/{}", node.name())
    };

    match node {
        TreeNode::Folder {
            name,
            children,
            icon_open,
            icon_closed,
        } => render_folder(
            name,
            children,
            *icon_open,
            *icon_closed,
            &path,
            state,
            on_action,
            props,
            theme,
            depth,
        ),
        TreeNode::File { name, icon } => {
            render_file(name, *icon, &path, state, on_action, props, theme, depth)
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn render_folder<'a, Message: Clone + 'a>(
    name: &str,
    children: &[TreeNode],
    icon_open: Option<LucideIcon>,
    icon_closed: Option<LucideIcon>,
    path: &str,
    state: &TreeViewState,
    on_action: &(impl Fn(TreeViewAction) -> Message + 'a + Clone),
    props: TreeViewProps,
    theme: &Theme,
    depth: usize,
) -> Element<'a, Message> {
    let open = state.is_open(path);
    let left_pad = props.content_offset + props.indent * depth as f32;
    let fg = theme.palette.foreground;
    let muted_fg = theme.palette.muted_foreground;
    let border_color = theme.palette.border;
    let hover_bg = theme.palette.accent;
    let hover_fg = theme.palette.accent_foreground;
    let row_radius = theme.radius.sm;

    let icon = if open {
        icon_open.unwrap_or(LucideIcon::FolderOpen)
    } else {
        icon_closed.unwrap_or(LucideIcon::Folder)
    };

    let icon_el: Element<'a, Message> = text(char::from(icon).to_string())
        .font(Font::with_name("lucide"))
        .size(props.icon_size)
        .color(muted_fg)
        .into();

    let label = text(truncate_ellipsis(name, props.max_label_chars))
        .size(props.font_size)
        .color(fg)
        .wrapping(text::Wrapping::None);

    let trigger_row = row![icon_el, label]
        .spacing(6)
        .align_y(Vertical::Center)
        .width(Length::Fill);

    let path_owned = path.to_string();
    let on_action_clone = on_action.clone();
    let trigger_btn = iced_button(
        container(trigger_row)
            .padding(Padding {
                top: 0.0,
                right: 0.0,
                bottom: 0.0,
                left: left_pad,
            })
            .height(Length::Fixed(props.row_height))
            .width(Length::Fill)
            .clip(true)
            .align_y(Vertical::Center),
    )
    .on_press((on_action_clone)(TreeViewAction::ToggleFolder(path_owned)))
    .padding(0)
    .width(Length::Fill)
    .style(move |_theme, status| {
        let bg = match status {
            iced_button::Status::Hovered => Background::Color(hover_bg),
            _ => Background::Color(Color::TRANSPARENT),
        };
        iced_button::Style {
            background: Some(bg),
            text_color: if matches!(status, iced_button::Status::Hovered) {
                hover_fg
            } else {
                fg
            },
            border: Border {
                radius: row_radius.into(),
                ..Border::default()
            },
            shadow: Default::default(),
            snap: true,
        }
    });

    let mut col = column![
        container(trigger_btn)
            .padding(Padding::from([0.0, 4.0]))
            .width(Length::Fill)
    ]
    .spacing(0);

    if open {
        let mut children_col = column![].spacing(0).width(Length::Fill);
        for child in children {
            children_col = children_col.push(render_node(
                child,
                state,
                on_action,
                props,
                theme,
                depth + 1,
                path,
            ));
        }

        // Vertical guide line at the folder icon center
        let guide_x = left_pad + props.icon_size * 0.5;

        let guide_line = rule::vertical(1).style(move |_theme| rule::Style {
            color: border_color,
            radius: 0.0.into(),
            fill_mode: rule::FillMode::Full,
            snap: true,
        });

        // Guide layer: Space pushes the line to the right x-position, Fill height
        let guide_layer = row![Space::new().width(guide_x), guide_line]
            .spacing(0)
            .height(Length::Fill);

        // Stack: children_col first (determines size), guide overlaid on top
        let children_with_guide = stack![children_col, guide_layer].width(Length::Fill);

        col = col.push(children_with_guide);
    }

    col.into()
}

#[allow(clippy::too_many_arguments)]
fn render_file<'a, Message: Clone + 'a>(
    name: &str,
    icon: Option<LucideIcon>,
    path: &str,
    state: &TreeViewState,
    on_action: &(impl Fn(TreeViewAction) -> Message + 'a + Clone),
    props: TreeViewProps,
    theme: &Theme,
    depth: usize,
) -> Element<'a, Message> {
    let left_pad = props.content_offset + props.indent * depth as f32 + 3.0;
    let fg = theme.palette.foreground;
    let muted_fg = theme.palette.muted_foreground;
    let accent = theme.palette.accent;
    let accent_fg = theme.palette.accent_foreground;
    let hover_bg = theme.palette.accent;
    let row_radius = theme.radius.sm;
    let is_selected = state.is_selected(path);

    let icon_el: Element<'a, Message> =
        text(char::from(icon.unwrap_or(LucideIcon::File)).to_string())
            .font(Font::with_name("lucide"))
            .size(props.icon_size)
            .color(if is_selected { accent_fg } else { muted_fg })
            .into();

    let label_color = if is_selected { accent_fg } else { fg };
    let label = text(truncate_ellipsis(name, props.max_label_chars))
        .size(props.font_size)
        .color(label_color)
        .wrapping(text::Wrapping::None);

    let content_row = row![icon_el, label]
        .spacing(6)
        .align_y(Vertical::Center)
        .width(Length::Fill);

    let path_owned = path.to_string();
    let on_action_clone = on_action.clone();

    let bg_selected = accent;
    let mut file_btn = iced_button(
        container(content_row)
            .padding(Padding {
                top: 0.0,
                right: 0.0,
                bottom: 0.0,
                left: left_pad,
            })
            .height(Length::Fixed(props.row_height))
            .width(Length::Fill)
            .clip(true)
            .align_y(Vertical::Center),
    )
    .padding(0)
    .width(Length::Fill)
    .style(move |_theme, status| {
        let (bg, txt) = if is_selected {
            (Background::Color(bg_selected), accent_fg)
        } else {
            match status {
                iced_button::Status::Hovered => (Background::Color(hover_bg), fg),
                _ => (Background::Color(Color::TRANSPARENT), fg),
            }
        };
        iced_button::Style {
            background: Some(bg),
            text_color: txt,
            border: Border {
                radius: row_radius.into(),
                ..Border::default()
            },
            shadow: Default::default(),
            snap: true,
        }
    });

    if props.selectable {
        file_btn = file_btn.on_press((on_action_clone)(TreeViewAction::SelectFile(path_owned)));
    }

    container(file_btn)
        .padding(Padding::from([0.0, 4.0]))
        .width(Length::Fill)
        .into()
}
