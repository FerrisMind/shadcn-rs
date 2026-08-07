//! Interactive playground for `iced-shadcn-v2::TreeView`.
//!
//! Left column: theme + tree controls (button-style playground).
//! Right column: live TreeView preview.
//!
//! Run with:
//! `cargo run -p iced-shadcn-v2 --example tree-view`

use std::fmt;

use iced::widget::{column, container, pick_list, row, scrollable, text};
use iced::{Alignment, Background, Border, Color, Element, Length, Task};

use iced_shadcn_v2::{
    AccentColor, BaseColor, Button, ButtonSize, ButtonVariant, FolderState, FontHeading, FontId,
    FontPack, RadiusId, StyleId, Theme, ThemeMode, TreeIconKey, TreeNavigationPolicy, TreeNode,
    TreeNodeId, TreeOrdering, TreeSelectionMode, TreeView, TreeViewAction, TreeViewRenderMode,
    TreeViewState, fonts, iced_font,
};

pub fn main() -> iced::Result {
    let mut app = iced::application(Example::default, Example::update, Example::view)
        .title(Example::title)
        .window_size(iced::Size::new(1180.0, 820.0))
        .default_font(iced_font(FontId::Geist));

    for face in fonts::ALL_FACES {
        app = app.font(*face);
    }

    app.run()
}

struct Example {
    theme: Theme,
    roots: Vec<TreeNode>,
    state: TreeViewState,
    selection: TreeSelectionMode,
    render_mode: TreeViewRenderMode,
    navigation: TreeNavigationPolicy,
    ordering: TreeOrdering,
    custom_icons: bool,
    large_tree: bool,
    last_action: Option<String>,
}

#[derive(Debug, Clone)]
enum Message {
    Style(Labelled<StyleId>),
    Base(Labelled<BaseColor>),
    Accent(AccentOpt),
    Mode(Labelled<ThemeMode>),
    Font(Labelled<FontId>),
    Heading(Labelled<FontHeading>),
    Radius(Labelled<RadiusId>),
    Selection(Labelled<TreeSelectionMode>),
    RenderMode(Labelled<TreeViewRenderMode>),
    Navigation(Labelled<TreeNavigationPolicy>),
    Ordering(Labelled<TreeOrdering>),
    ToggleCustomIcons,
    ToggleLargeTree,
    ExpandAll,
    CollapseAll,
    Tree(TreeViewAction),
}

impl Default for Example {
    fn default() -> Self {
        let roots = sample_roots();
        let mut state = TreeViewState::with_default_open(false);
        state.expand(&id("src"));
        state.expand(&id("lib"));

        Self {
            theme: Theme::light().with_style(StyleId::Vega),
            roots,
            state,
            selection: TreeSelectionMode::Single,
            render_mode: TreeViewRenderMode::Animated,
            navigation: TreeNavigationPolicy::Basic,
            ordering: TreeOrdering::Source,
            custom_icons: false,
            large_tree: false,
            last_action: None,
        }
    }
}

impl Example {
    fn title(&self) -> String {
        "iced-shadcn-v2 Tree View".to_owned()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Style(style) => {
                self.theme = self.theme.clone().with_style(style.0);
            }
            Message::Base(base) => {
                self.theme = self.theme.clone().with_base(base.0);
            }
            Message::Accent(accent) => {
                self.theme = self.theme.clone().with_accent(accent.into_option());
            }
            Message::Mode(mode) => {
                self.theme = self.theme.clone().with_mode(mode.0);
            }
            Message::Font(font) => {
                self.theme = self.theme.clone().with_font(font.0);
            }
            Message::Heading(heading) => {
                self.theme = self.theme.clone().with_font_heading(heading.0);
            }
            Message::Radius(radius) => {
                self.theme = self.theme.clone().with_radius(radius.0);
            }
            Message::Selection(selection) => {
                self.selection = selection.0;
                if matches!(self.selection, TreeSelectionMode::None) {
                    self.state.clear_selection();
                }
            }
            Message::RenderMode(mode) => {
                self.render_mode = mode.0;
            }
            Message::Navigation(policy) => {
                self.navigation = policy.0;
            }
            Message::Ordering(ordering) => {
                self.ordering = ordering.0;
            }
            Message::ToggleCustomIcons => {
                self.custom_icons = !self.custom_icons;
            }
            Message::ToggleLargeTree => {
                self.large_tree = !self.large_tree;
                self.rebuild_roots();
            }
            Message::ExpandAll => {
                self.state.expand_all(&self.roots);
            }
            Message::CollapseAll => {
                self.state.collapse_all(&self.roots);
            }
            Message::Tree(action) => {
                self.last_action = Some(format!("{action:?}"));
                match &action {
                    TreeViewAction::Toggle(node_id) => {
                        self.state.toggle(node_id);
                    }
                    TreeViewAction::Select(node_id) => {
                        if self.selection == TreeSelectionMode::Single {
                            self.state.select(node_id.clone());
                        }
                    }
                    TreeViewAction::Load(node_id) => {
                        self.load_folder(node_id);
                    }
                    TreeViewAction::Hover(node_id) => match node_id {
                        Some(id) => self.state.hover(id.clone()),
                        None => self.state.clear_hover(),
                    },
                    TreeViewAction::Context(node_id) => {
                        self.state.context(node_id.clone());
                    }
                    _ => {}
                }
            }
        }

        Task::none()
    }

    fn load_folder(&mut self, node_id: &TreeNodeId) {
        let docs = id("docs");
        if node_id != &docs {
            return;
        }

        let Some(index) = self.roots.iter().position(|node| node.id() == &docs) else {
            return;
        };

        if self.roots[index].folder_state() != Some(FolderState::Unloaded) {
            return;
        }

        self.roots[index] = TreeNode::folder(docs.clone(), "docs")
            .children([
                TreeNode::file(id("docs-intro"), "intro.md").into_node(),
                TreeNode::file(id("docs-api"), "api.md").into_node(),
                TreeNode::folder(id("docs-guides"), "guides")
                    .children([
                        TreeNode::file(id("docs-getting-started"), "getting-started.md")
                            .into_node(),
                    ])
                    .into_node(),
            ])
            .into_node();
        self.state.expand(&docs);
        self.last_action = Some("lazy-loaded docs".to_owned());
    }

    fn rebuild_roots(&mut self) {
        self.roots = if self.large_tree {
            large_roots()
        } else {
            sample_roots()
        };
        self.state = TreeViewState::with_default_open(false);
        if self.large_tree {
            self.state.expand(&id("root-0"));
            self.render_mode = TreeViewRenderMode::Virtualized;
        } else {
            self.state.expand(&id("src"));
            self.state.expand(&id("lib"));
            self.render_mode = TreeViewRenderMode::Animated;
        }
        self.last_action = None;
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let palette = theme.palette;
        let font = iced_font(theme.font_pack().sans);

        let playground = column![
            section_label("Playground", palette.foreground, &theme.font_pack()),
            section_label("Theme", palette.muted_foreground, &theme.font_pack()),
            control_select(
                "Style",
                &STYLES,
                Some(Labelled(theme.style_id())),
                Message::Style,
                theme,
            ),
            control_select(
                "Base",
                &BASES,
                Some(Labelled(theme.base())),
                Message::Base,
                theme,
            ),
            control_select(
                "Accent",
                &ACCENTS,
                Some(AccentOpt::from_option(theme.accent())),
                Message::Accent,
                theme,
            ),
            control_select(
                "Mode",
                &MODES,
                Some(Labelled(theme.mode())),
                Message::Mode,
                theme,
            ),
            control_select(
                "Heading",
                &HEADINGS,
                Some(Labelled(theme.font_heading())),
                Message::Heading,
                theme,
            ),
            control_select(
                "Font",
                &FONTS,
                Some(Labelled(theme.font_id())),
                Message::Font,
                theme,
            ),
            control_select(
                "Radius",
                &RADII,
                Some(Labelled(theme.radius_id())),
                Message::Radius,
                theme,
            ),
            text(format!(
                "radius lg={:.0}px, sans={}, heading={}",
                theme.radius_scale().lg_px,
                theme.font_pack().sans.title(),
                theme.font_heading().title(),
            ))
            .size(12)
            .font(iced_font(theme.font_pack().mono))
            .color(palette.muted_foreground),
            section_label("Tree", palette.muted_foreground, &theme.font_pack()),
            control_select(
                "Selection",
                &SELECTIONS,
                Some(Labelled(self.selection)),
                Message::Selection,
                theme,
            ),
            control_select(
                "Render",
                &RENDER_MODES,
                Some(Labelled(self.render_mode)),
                Message::RenderMode,
                theme,
            ),
            control_select(
                "Nav",
                &NAVIGATION,
                Some(Labelled(self.navigation)),
                Message::Navigation,
                theme,
            ),
            control_select(
                "Order",
                &ORDERINGS,
                Some(Labelled(self.ordering)),
                Message::Ordering,
                theme,
            ),
            row![
                Button::text("Expand all", theme)
                    .variant(ButtonVariant::Outline)
                    .size(ButtonSize::Sm)
                    .on_press(Message::ExpandAll),
                Button::text("Collapse", theme)
                    .variant(ButtonVariant::Outline)
                    .size(ButtonSize::Sm)
                    .on_press(Message::CollapseAll),
            ]
            .spacing(8),
            row![
                Button::text(
                    if self.custom_icons {
                        "Custom icons: on"
                    } else {
                        "Custom icons: off"
                    },
                    theme,
                )
                .variant(ButtonVariant::Outline)
                .size(ButtonSize::Sm)
                .on_press(Message::ToggleCustomIcons),
                Button::text(
                    if self.large_tree {
                        "Large tree: on"
                    } else {
                        "Large tree: off"
                    },
                    theme,
                )
                .variant(ButtonVariant::Outline)
                .size(ButtonSize::Sm)
                .on_press(Message::ToggleLargeTree),
            ]
            .spacing(8),
            text("Open docs/ to trigger lazy load")
                .size(12)
                .font(font)
                .color(palette.muted_foreground),
            text(format!(
                "last action: {}",
                self.last_action.as_deref().unwrap_or("none")
            ))
            .size(12)
            .font(iced_font(theme.font_pack().mono))
            .color(palette.muted_foreground),
        ]
        .spacing(10)
        .width(Length::Fill);

        let mut tree = TreeView::new(theme, &self.roots, &self.state)
            .expect("example tree must validate")
            .selection(self.selection)
            .render_mode(self.render_mode)
            .navigation(self.navigation)
            .ordering(self.ordering)
            .width(Length::Fill)
            .height(Length::Fill)
            .on_action(Message::Tree);

        if self.custom_icons {
            tree = tree.icon_renderer(|key, color, size| {
                let glyph = match key {
                    TreeIconKey::Folder => "▸",
                    TreeIconKey::FolderOpen => "▾",
                    TreeIconKey::File => "·",
                    TreeIconKey::Loader => "…",
                    TreeIconKey::Named(_) => "★",
                    _ => "•",
                };
                text(glyph).size(size).color(color).into()
            });
        }

        let tree: Element<'_, Message> = tree.into();

        let preview = column![
            text("Tree View")
                .size(32)
                .font(iced_font(theme.font_pack().heading))
                .color(palette.foreground),
            text("Controlled folders, optional selection, animated or virtualized render")
                .size(14)
                .font(font)
                .color(palette.muted_foreground),
            container(tree)
                .padding(12)
                .width(Length::Fill)
                .height(Length::Fill)
                .style(move |_| {
                    card_style(palette.card, palette.border, theme.radius_scale().lg_px)
                }),
        ]
        .spacing(16)
        .width(Length::Fill)
        .height(Length::Fill);

        let playground = container(scrollable(playground).height(Length::Fill))
            .width(Length::Fixed(300.0))
            .height(Length::Fill)
            .padding(16)
            .style(move |_| card_style(palette.card, palette.border, theme.radius_scale().lg_px));

        container(
            row![
                playground,
                scrollable(preview).width(Length::Fill).height(Length::Fill)
            ]
            .spacing(24)
            .width(Length::Fill)
            .height(Length::Fill),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(24)
        .style(move |_| container::Style {
            background: Some(Background::Color(palette.background)),
            text_color: Some(palette.foreground),
            ..container::Style::default()
        })
        .into()
    }
}

fn id(value: &str) -> TreeNodeId {
    TreeNodeId::new(value).expect("example ids are non-empty")
}

fn sample_roots() -> Vec<TreeNode> {
    vec![
        TreeNode::folder(id("src"), "src")
            .children([
                TreeNode::file(id("main-rs"), "main.rs").into_node(),
                TreeNode::file(id("lib-rs"), "lib.rs").into_node(),
                TreeNode::folder(id("components"), "components")
                    .children([
                        TreeNode::file(id("button-rs"), "button.rs").into_node(),
                        TreeNode::file(id("tree-view-rs"), "tree_view.rs").into_node(),
                    ])
                    .into_node(),
            ])
            .into_node(),
        TreeNode::folder(id("lib"), "lib")
            .children([
                TreeNode::file(id("utils-ts"), "utils.ts").into_node(),
                TreeNode::file(id("types-ts"), "types.ts").into_node(),
            ])
            .into_node(),
        TreeNode::unloaded_folder(id("docs"), "docs").into_node(),
        TreeNode::file(id("cargo-toml"), "Cargo.toml").into_node(),
        TreeNode::file(id("readme"), "README.md").into_node(),
    ]
}

fn large_roots() -> Vec<TreeNode> {
    (0..40)
        .map(|i| {
            TreeNode::folder(id(&format!("root-{i}")), format!("group-{i}"))
                .children((0..25).map(|j| {
                    TreeNode::file(id(&format!("file-{i}-{j}")), format!("item-{j}.txt"))
                        .into_node()
                }))
                .into_node()
        })
        .collect()
}

fn section_label<'a>(label: &'static str, color: Color, fonts: &FontPack) -> Element<'a, Message> {
    text(label)
        .size(13)
        .font(iced_font(fonts.sans))
        .color(color)
        .into()
}

fn card_style(background: Color, border: Color, radius: f32) -> container::Style {
    container::Style {
        background: Some(Background::Color(background)),
        border: Border {
            color: border,
            width: 1.0,
            radius: radius.into(),
        },
        ..container::Style::default()
    }
}

fn control_select<'a, T, F>(
    label: &'static str,
    options: &'a [T],
    selected: Option<T>,
    on_select: F,
    theme: &'a Theme,
) -> Element<'a, Message>
where
    T: Clone + PartialEq + fmt::Display + 'a,
    F: Fn(T) -> Message + 'a,
{
    let palette = &theme.palette;
    let font = iced_font(theme.font_pack().sans);

    row![
        text(label)
            .size(13)
            .width(82)
            .font(font)
            .color(palette.muted_foreground),
        pick_list(options, selected, on_select)
            .text_size(13)
            .font(font)
            .width(Length::Fixed(180.0))
            .style(move |_theme, _status| pick_list::Style {
                background: Background::Color(palette.background),
                text_color: palette.foreground,
                placeholder_color: palette.muted_foreground,
                handle_color: palette.muted_foreground,
                border: Border {
                    color: palette.border,
                    width: 1.0,
                    radius: theme.radius_scale().md_px.into(),
                },
            }),
    ]
    .spacing(8)
    .align_y(Alignment::Center)
    .into()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Labelled<T>(T);

impl fmt::Display for Labelled<StyleId> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0.as_str())
    }
}

impl fmt::Display for Labelled<BaseColor> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0.as_str())
    }
}

impl fmt::Display for Labelled<ThemeMode> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0.as_str())
    }
}

impl fmt::Display for Labelled<FontId> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0.title())
    }
}

impl fmt::Display for Labelled<FontHeading> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0.title())
    }
}

impl fmt::Display for Labelled<RadiusId> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0.label())
    }
}

impl fmt::Display for Labelled<TreeSelectionMode> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self.0 {
            TreeSelectionMode::None => "none",
            TreeSelectionMode::Single => "single",
            _ => "other",
        })
    }
}

impl fmt::Display for Labelled<TreeViewRenderMode> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self.0 {
            TreeViewRenderMode::Animated => "animated",
            TreeViewRenderMode::Virtualized => "virtualized",
            _ => "other",
        })
    }
}

impl fmt::Display for Labelled<TreeNavigationPolicy> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self.0 {
            TreeNavigationPolicy::Basic => "basic",
            TreeNavigationPolicy::Full => "full",
            _ => "other",
        })
    }
}

impl fmt::Display for Labelled<TreeOrdering> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self.0 {
            TreeOrdering::Source => "source",
            TreeOrdering::FoldersFirst => "folders-first",
            TreeOrdering::Label => "label",
            _ => "other",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AccentOpt {
    None,
    Color(AccentColor),
}

impl AccentOpt {
    const fn from_option(accent: Option<AccentColor>) -> Self {
        match accent {
            None => Self::None,
            Some(color) => Self::Color(color),
        }
    }

    const fn into_option(self) -> Option<AccentColor> {
        match self {
            Self::None => None,
            Self::Color(color) => Some(color),
        }
    }
}

impl fmt::Display for AccentOpt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => f.write_str("none"),
            Self::Color(color) => f.write_str(color.as_str()),
        }
    }
}

const STYLES: [Labelled<StyleId>; 8] = [
    Labelled(StyleId::Vega),
    Labelled(StyleId::Nova),
    Labelled(StyleId::Maia),
    Labelled(StyleId::Lyra),
    Labelled(StyleId::Mira),
    Labelled(StyleId::Luma),
    Labelled(StyleId::Sera),
    Labelled(StyleId::Rhea),
];

const BASES: [Labelled<BaseColor>; 7] = [
    Labelled(BaseColor::Neutral),
    Labelled(BaseColor::Zinc),
    Labelled(BaseColor::Stone),
    Labelled(BaseColor::Mauve),
    Labelled(BaseColor::Mist),
    Labelled(BaseColor::Olive),
    Labelled(BaseColor::Taupe),
];

const ACCENTS: [AccentOpt; 18] = [
    AccentOpt::None,
    AccentOpt::Color(AccentColor::Amber),
    AccentOpt::Color(AccentColor::Blue),
    AccentOpt::Color(AccentColor::Cyan),
    AccentOpt::Color(AccentColor::Emerald),
    AccentOpt::Color(AccentColor::Fuchsia),
    AccentOpt::Color(AccentColor::Green),
    AccentOpt::Color(AccentColor::Indigo),
    AccentOpt::Color(AccentColor::Lime),
    AccentOpt::Color(AccentColor::Orange),
    AccentOpt::Color(AccentColor::Pink),
    AccentOpt::Color(AccentColor::Purple),
    AccentOpt::Color(AccentColor::Red),
    AccentOpt::Color(AccentColor::Rose),
    AccentOpt::Color(AccentColor::Sky),
    AccentOpt::Color(AccentColor::Teal),
    AccentOpt::Color(AccentColor::Violet),
    AccentOpt::Color(AccentColor::Yellow),
];

const MODES: [Labelled<ThemeMode>; 2] = [Labelled(ThemeMode::Light), Labelled(ThemeMode::Dark)];

const FONTS: [Labelled<FontId>; 5] = [
    Labelled(FontId::Geist),
    Labelled(FontId::Inter),
    Labelled(FontId::InstrumentSerif),
    Labelled(FontId::GeistMono),
    Labelled(FontId::JetBrainsMono),
];

const HEADINGS: [Labelled<FontHeading>; 6] = [
    Labelled(FontHeading::Inherit),
    Labelled(FontHeading::Font(FontId::Geist)),
    Labelled(FontHeading::Font(FontId::Inter)),
    Labelled(FontHeading::Font(FontId::InstrumentSerif)),
    Labelled(FontHeading::Font(FontId::GeistMono)),
    Labelled(FontHeading::Font(FontId::JetBrainsMono)),
];

const RADII: [Labelled<RadiusId>; 5] = [
    Labelled(RadiusId::Default),
    Labelled(RadiusId::None),
    Labelled(RadiusId::Small),
    Labelled(RadiusId::Medium),
    Labelled(RadiusId::Large),
];

const SELECTIONS: [Labelled<TreeSelectionMode>; 2] = [
    Labelled(TreeSelectionMode::None),
    Labelled(TreeSelectionMode::Single),
];

const RENDER_MODES: [Labelled<TreeViewRenderMode>; 2] = [
    Labelled(TreeViewRenderMode::Animated),
    Labelled(TreeViewRenderMode::Virtualized),
];

const NAVIGATION: [Labelled<TreeNavigationPolicy>; 2] = [
    Labelled(TreeNavigationPolicy::Basic),
    Labelled(TreeNavigationPolicy::Full),
];

const ORDERINGS: [Labelled<TreeOrdering>; 3] = [
    Labelled(TreeOrdering::Source),
    Labelled(TreeOrdering::FoldersFirst),
    Labelled(TreeOrdering::Label),
];
