use iced::widget::{column, container, text as iced_text};
use iced::{Background, Border, Element, Length, Task};

use iced_shadcn::{
    FlatNode, FolderState, Theme, TreeViewerProps, TreeViewerState, tree_viewer,
    scroll_area, ScrollAreaProps, scroll_area::ScrollAreaScrollbars,
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

        // Root folders
        all_nodes.push(FlatNode::folder("root", "/root", "Project Root", 0, true, FolderState::Loaded));
        
        // Add 100,000 files
        for i in 0..100_000 {
            all_nodes.push(FlatNode::file(
                format!("file_{i}"), 
                format!("/root/file_{i}.rs"), 
                format!("file_{i}.rs"), 
                1
            ));
        }

        // Visible nodes (initially just root and files)
        let state = TreeViewerState {
            nodes: all_nodes.clone(),
            selected_path: None,
        };

        Self {
            theme: Theme::dark(),
            state,
            all_nodes,
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    Toggle(String),
    Select(String),
    Load(String),
}

impl Example {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Toggle(path) => {
                if let Some(node) = self.state.nodes.iter_mut().find(|n| n.path == path) {
                    node.is_expanded = !node.is_expanded;
                    
                    // In a real app, we would filter or add/remove nodes from self.state.nodes 
                    // based on expansion. For this virtualization demo, we assume 
                    // they are all in one list.
                }
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

        let title = iced_text("Tree Viewer (Virtualized - 100,000 Nodes)")
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
            scroll_area(viewer, ScrollAreaProps::new().scrollbars(ScrollAreaScrollbars::Vertical), theme),
            iced_text(format!("Total nodes: {}", self.state.nodes.len()))
                .size(12)
                .color(theme.palette.muted_foreground)
        ]
        .spacing(12)
        .height(Length::Fill);

        let card = preview(
            theme,
            content
        );

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
