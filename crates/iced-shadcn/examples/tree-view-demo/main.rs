use iced::border::Border;
use iced::widget::{column, container, text as iced_text};
use iced::{Background, Element, Length};

use iced_shadcn::{Theme, TreeNode, TreeViewAction, TreeViewProps, TreeViewState, tree_view};
use lucide_icons::LUCIDE_FONT_BYTES;

pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view)
        .font(LUCIDE_FONT_BYTES)
        .run()
}

#[derive(Clone, Debug)]
struct Example {
    theme: Theme,
    state: TreeViewState,
    tree: Vec<TreeNode>,
}

impl Default for Example {
    fn default() -> Self {
        let tree = vec![
            TreeNode::folder(
                "src",
                vec![
                    TreeNode::folder(
                        "routes",
                        vec![
                            TreeNode::file("+layout.svelte"),
                            TreeNode::file("+page.svelte"),
                        ],
                    ),
                    TreeNode::folder(
                        "lib",
                        vec![TreeNode::folder(
                            "components",
                            vec![
                                TreeNode::folder(
                                    "ui",
                                    vec![
                                        TreeNode::folder(
                                            "collapsible",
                                            vec![TreeNode::file("index.ts")],
                                        ),
                                        TreeNode::folder(
                                            "tree-view",
                                            vec![
                                                TreeNode::file("index.ts"),
                                                TreeNode::file("tree-view.svelte"),
                                                TreeNode::file("tree-view-folder.svelte"),
                                                TreeNode::file(
                                                    "tree-view-file-with-a-very-long-name.svelte",
                                                ),
                                                TreeNode::file("types.ts"),
                                            ],
                                        ),
                                    ],
                                ),
                                TreeNode::file("utils.ts"),
                            ],
                        )],
                    ),
                    TreeNode::file("app.css"),
                    TreeNode::file("app.d.ts"),
                    TreeNode::file("app.html"),
                    TreeNode::file("hooks.server.ts"),
                    TreeNode::file("hooks.ts"),
                ],
            ),
            TreeNode::folder(
                ".github",
                vec![TreeNode::folder(
                    "workflows",
                    vec![TreeNode::file("ci.yml"), TreeNode::file("publish.yml")],
                )],
            ),
            TreeNode::file(".gitignore"),
            TreeNode::file("package.json"),
            TreeNode::file("README.md"),
            TreeNode::file("svelte.config.js"),
            TreeNode::file("tsconfig.json"),
        ];

        // Start with "src" expanded
        let state = TreeViewState::with_open(vec!["src".into()]);

        Self {
            theme: Theme::dark(),
            state,
            tree,
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    TreeAction(TreeViewAction),
}

impl Example {
    fn update(&mut self, message: Message) {
        match message {
            Message::TreeAction(action) => match action {
                TreeViewAction::ToggleFolder(path) => {
                    self.state.toggle_folder(&path);
                }
                TreeViewAction::SelectFile(path) => {
                    self.state.select(&path);
                }
                TreeViewAction::LoadFolder(path) => {
                    // Start loading async in real app, here we just toggle
                    self.state.toggle_folder(&path);
                }
            },
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;

        let title = iced_text("Tree View")
            .size(18)
            .color(theme.palette.foreground);

        let selected_info = if let Some(ref sel) = self.state.selected {
            iced_text(format!("Selected: {sel}"))
                .size(12)
                .color(theme.palette.muted_foreground)
        } else {
            iced_text("No file selected")
                .size(12)
                .color(theme.palette.muted_foreground)
        };

        let tree = tree_view(
            self.tree.clone(),
            self.state.clone(),
            Message::TreeAction,
            TreeViewProps::new(),
            theme,
        );

        let card = preview(
            theme,
            column![title, tree, selected_info]
                .spacing(12)
                .height(Length::Fill),
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
        .width(Length::Fixed(420.0))
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
