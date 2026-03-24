use iced::widget::{column, container, text as iced_text};
use iced::{Background, Border, Element, Length, Task};

use iced_shadcn::{
    FlatNode, FolderState, ScrollAreaProps, Theme, TreeViewerHandlers, TreeViewerProps,
    TreeViewerState, scroll_area, scroll_area::ScrollAreaScrollbars, tree_viewer,
};
use lucide_icons::LUCIDE_FONT_BYTES;

pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view)
        .font(LUCIDE_FONT_BYTES)
        .run()
}

#[derive(Clone, Debug)]
struct Example {
    theme: Theme,
    state: TreeViewerState,
    // Full data is kept separately, state.nodes contains only visible ones
    all_nodes: Vec<FlatNode>,
}

impl Default for Example {
    fn default() -> Self {
        let mut all_nodes = vec![
            // Let's create a nested structure similar to tree-view
            // src/
            FlatNode::folder("src", "/src", "src", 0, true, FolderState::Loaded),
            // src/components
            FlatNode::folder(
                "components",
                "/src/components",
                "components",
                1,
                true,
                FolderState::Loaded,
            ),
            FlatNode::folder(
                "ui",
                "/src/components/ui",
                "ui",
                2,
                false,
                FolderState::Loaded,
            ),
            FlatNode::file("button.rs", "/src/components/ui/button.rs", "button.rs", 3),
            FlatNode::file(
                "tree_viewer.rs",
                "/src/components/tree_viewer.rs",
                "tree_viewer.rs",
                2,
            ),
            // src/lib.rs
            FlatNode::file("lib.rs", "/src/lib.rs", "lib.rs", 1),
            // Let's create an Unloaded folder to demonstrate Async Lazy Loading
            FlatNode::folder(
                "unloaded_folder",
                "/unloaded_folder",
                "unloaded_folder (Click to load 10k files)",
                0,
                false,
                FolderState::Unloaded,
            ),
            // Let's add thousands of nested generated files to show virtualization
            FlatNode::folder(
                "big_folder",
                "/big_folder",
                "big_folder (10,000 files)",
                0,
                false,
                FolderState::Loaded,
            ),
        ];
        for i in 0..10_000 {
            all_nodes.push(FlatNode::file(
                format!("file_{i}"),
                format!("/big_folder/file_{i}.rs"),
                format!("file_{i}.rs"),
                1,
            ));
        }

        let mut example = Self {
            theme: Theme::dark(),
            state: TreeViewerState {
                nodes: vec![],
                selected_path: None,
            },
            all_nodes,
        };

        // Initial loading of visible nodes
        let mut visible = Vec::new();
        let mut skip_depth = None;
        for node in &example.all_nodes {
            if let Some(depth) = skip_depth {
                if node.depth > depth {
                    continue;
                } else {
                    skip_depth = None;
                }
            }
            visible.push(node.clone());
            if node.is_folder && !node.is_expanded {
                skip_depth = Some(node.depth);
            }
        }
        example.state.nodes = visible;

        example
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Toggle(String),
    Select(String),
    Load(String),
    FolderLoaded(String, Vec<FlatNode>),
    Hover(Option<String>),
    Context(String),
}

impl Example {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Toggle(path) => {
                // Find node in visible state to toggle
                if let Some(state_index) = self.state.nodes.iter().position(|n| n.path == path) {
                    let is_expanded = !self.state.nodes[state_index].is_expanded;
                    self.state.nodes[state_index].is_expanded = is_expanded;

                    // Also update it in all_nodes for persistence
                    if let Some(all_node) = self.all_nodes.iter_mut().find(|n| n.path == path) {
                        all_node.is_expanded = is_expanded;
                    }

                    if is_expanded {
                        // O(K) Insertion: Find children in all_nodes and splice them in
                        let depth = self.state.nodes[state_index].depth;

                        // We need to find this folder's immediate children, and if those children are expanded folders, their children too.
                        // Since we just want to mimic the old `update_visible_nodes` but locally:
                        let start_all =
                            self.all_nodes.iter().position(|n| n.path == path).unwrap() + 1;
                        let mut children_to_insert = Vec::new();
                        let mut skip_depth = None;

                        for node in self.all_nodes.iter().skip(start_all) {
                            if node.depth <= depth {
                                break; // Reached next sibling or uncle
                            }

                            if let Some(s_depth) = skip_depth {
                                if node.depth > s_depth {
                                    continue;
                                } else {
                                    skip_depth = None;
                                }
                            }

                            children_to_insert.push(node.clone());

                            if node.is_folder && !node.is_expanded {
                                skip_depth = Some(node.depth);
                            }
                        }

                        self.state
                            .nodes
                            .splice(state_index + 1..state_index + 1, children_to_insert);
                    } else {
                        // O(K) Deletion: Find how many visible descendants to remove
                        let depth = self.state.nodes[state_index].depth;
                        let mut end_index = state_index + 1;
                        while end_index < self.state.nodes.len()
                            && self.state.nodes[end_index].depth > depth
                        {
                            end_index += 1;
                        }

                        // Drain to remove elements without reallocating
                        self.state.nodes.drain(state_index + 1..end_index);
                    }
                }
                Task::none()
            }
            Message::Select(path) => {
                self.state.select(&path);
                Task::none()
            }
            Message::Load(path) => {
                // Find node and set to Loading
                if let Some(state_index) = self.state.nodes.iter().position(|n| n.path == path) {
                    self.state.nodes[state_index].folder_state = FolderState::Loading;
                    if let Some(all_node) = self.all_nodes.iter_mut().find(|n| n.path == path) {
                        all_node.folder_state = FolderState::Loading;
                    }

                    let depth = self.state.nodes[state_index].depth;
                    let path_clone = path.clone();

                    // Simulate async lazy loading
                    return Task::perform(
                        async move {
                            // Simulate async loading without external runtime explicitly
                            std::thread::sleep(std::time::Duration::from_millis(500));

                            let mut new_nodes = Vec::new();
                            for i in 0..10_000 {
                                new_nodes.push(FlatNode::file(
                                    format!("file_{}", i),
                                    format!("{}/lazy_file_{}.rs", path_clone, i),
                                    format!("lazy_file_{}.rs", i),
                                    depth + 1,
                                ));
                            }
                            new_nodes
                        },
                        move |children| Message::FolderLoaded(path.clone(), children),
                    );
                }
                Task::none()
            }
            Message::FolderLoaded(path, children) => {
                // 1. Update all_nodes with new loaded children right below the parent
                if let Some(all_index) = self.all_nodes.iter().position(|n| n.path == path) {
                    self.all_nodes[all_index].folder_state = FolderState::Loaded;
                    self.all_nodes[all_index].is_expanded = true;
                    // Because `all_nodes` holds the data structure, we splice it in
                    self.all_nodes
                        .splice(all_index + 1..all_index + 1, children.clone());
                }

                // 2. Update visible state
                if let Some(state_index) = self.state.nodes.iter().position(|n| n.path == path) {
                    self.state.nodes[state_index].folder_state = FolderState::Loaded;
                    self.state.nodes[state_index].is_expanded = true;
                    self.state
                        .nodes
                        .splice(state_index + 1..state_index + 1, children);
                }

                Task::none()
            }
            Message::Hover(_path) => Task::none(),
            Message::Context(_path) => Task::none(),
        }
    }
    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;

        let title = iced_text("Tree Viewer (Virtualized - 10,005 Nodes)")
            .size(18)
            .color(theme.palette.foreground);

        let viewer = tree_viewer(
            &self.state,
            None,
            TreeViewerHandlers::new(
                Message::Toggle,
                Message::Select,
                Message::Load,
                Message::Hover,
                Message::Context,
            ),
            TreeViewerProps::default(),
            theme,
        );

        let content = column![
            title,
            scroll_area(
                viewer,
                ScrollAreaProps::new().scrollbars(ScrollAreaScrollbars::Vertical),
                theme
            ),
            iced_text(format!("Total nodes: {}", self.state.nodes.len()))
                .size(12)
                .color(theme.palette.muted_foreground)
        ]
        .spacing(12)
        .height(Length::Fill);

        let card = preview(theme, content);

        app(theme, card.height(Length::Fill).into())
    }
}

fn app<'a, Message: 'a>(theme: &Theme, content: Element<'a, Message>) -> Element<'a, Message> {
    let background = theme.palette.background;
    container(content)
        .padding(24)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(move |_theme| iced::widget::container::Style {
            background: Some(Background::Color(background)),
            ..Default::default()
        })
        .into()
}

fn preview<'a, Message: 'a>(
    theme: &Theme,
    content: impl Into<Element<'a, Message>>,
) -> iced::widget::Container<'a, Message> {
    let background = theme.palette.card;
    let border_color = theme.palette.border;
    let radius = theme.radius.md;

    container(content)
        .padding(16)
        .width(Length::Fixed(500.0))
        .clip(true)
        .style(move |_theme| iced::widget::container::Style {
            background: Some(Background::Color(background)),
            border: Border {
                radius: radius.into(),
                width: 1.0,
                color: border_color,
            },
            ..Default::default()
        })
}
