use iced::widget::{column, container, text as iced_text};
use iced::{Background, Border, Element, Length, Task};

use iced_shadcn::{
    FlatNode, FolderState, ScrollAreaProps, Theme, TreeViewerProps, TreeViewerState, scroll_area,
    scroll_area::ScrollAreaScrollbars, tree_viewer,
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
        let mut all_nodes = Vec::new();

        // Let's create a nested structure similar to tree-view
        // src/
        all_nodes.push(FlatNode::folder(
            "src",
            "/src",
            "src",
            0,
            true,
            FolderState::Loaded,
        ));

        // src/components
        all_nodes.push(FlatNode::folder(
            "components",
            "/src/components",
            "components",
            1,
            true,
            FolderState::Loaded,
        ));
        all_nodes.push(FlatNode::folder(
            "ui",
            "/src/components/ui",
            "ui",
            2,
            false,
            FolderState::Loaded,
        ));
        all_nodes.push(FlatNode::file(
            "button.rs",
            "/src/components/ui/button.rs",
            "button.rs",
            3,
        ));
        all_nodes.push(FlatNode::file(
            "tree_viewer.rs",
            "/src/components/tree_viewer.rs",
            "tree_viewer.rs",
            2,
        ));

        // src/lib.rs
        all_nodes.push(FlatNode::file("lib.rs", "/src/lib.rs", "lib.rs", 1));

        // Let's add thousands of nested generated files to show virtualization
        all_nodes.push(FlatNode::folder(
            "big_folder",
            "/big_folder",
            "big_folder (10,000 files)",
            0,
            false,
            FolderState::Loaded,
        ));
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

        example.update_visible_nodes();
        example
    }
}

#[derive(Debug, Clone)]
enum Message {
    Toggle(String),
    Select(String),
    Load(String),
}

impl Example {
    fn update_visible_nodes(&mut self) {
        // A simple algorithm to filter all_nodes into state.nodes based on what is expanded.
        // It skips children of collapsed folders.
        let mut visible = Vec::new();
        let mut skip_depth = None;

        for node in &self.all_nodes {
            if let Some(depth) = skip_depth {
                if node.depth > depth {
                    continue; // Skip because a parent is collapsed
                } else {
                    skip_depth = None; // Back to a visible level
                }
            }

            visible.push(node.clone());

            if node.is_folder && !node.is_expanded {
                skip_depth = Some(node.depth);
            }
        }

        self.state.nodes = visible;
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Toggle(path) => {
                // Find node and toggle it
                if let Some(node) = self.all_nodes.iter_mut().find(|n| n.path == path) {
                    node.is_expanded = !node.is_expanded;
                }

                // Re-evaluate visibility
                self.update_visible_nodes();
            }
            Message::Select(path) => {
                self.state.select(&path);
            }
            Message::Load(path) => {
                println!("Loading folder: {}", path);
                // Simulate loading
            }
        }
        Task::none()
    }
    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;

        let title = iced_text("Tree Viewer (Virtualized - 10,005 Nodes)")
            .size(18)
            .color(theme.palette.foreground);

        let viewer = tree_viewer(
            &self.state,
            Message::Toggle,
            Message::Select,
            Message::Load,
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
