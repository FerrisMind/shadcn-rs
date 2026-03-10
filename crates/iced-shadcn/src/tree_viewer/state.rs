use iced::Color;
use lucide_icons::Icon as LucideIcon;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FolderState {
    Unloaded,
    Loading,
    Loaded,
}

#[derive(Clone, Debug)]
pub struct FlatNode {
    pub id: String,
    pub path: String,
    pub name: String,
    pub depth: usize,
    pub is_folder: bool,
    pub is_expanded: bool,
    pub folder_state: FolderState,
    pub icon_open: Option<LucideIcon>,
    pub icon_closed: Option<LucideIcon>,
    pub icon_file: Option<LucideIcon>,
    pub icon_glyph: Option<char>,
    pub icon_font_family: Option<&'static str>,
    pub icon_color: Option<Color>,
}

impl FlatNode {
    pub fn folder(
        id: impl Into<String>,
        path: impl Into<String>,
        name: impl Into<String>,
        depth: usize,
        is_expanded: bool,
        folder_state: FolderState,
    ) -> Self {
        Self {
            id: id.into(),
            path: path.into(),
            name: name.into(),
            depth,
            is_folder: true,
            is_expanded,
            folder_state,
            icon_open: None,
            icon_closed: None,
            icon_file: None,
            icon_glyph: None,
            icon_font_family: None,
            icon_color: None,
        }
    }

    pub fn file(
        id: impl Into<String>,
        path: impl Into<String>,
        name: impl Into<String>,
        depth: usize,
    ) -> Self {
        Self {
            id: id.into(),
            path: path.into(),
            name: name.into(),
            depth,
            is_folder: false,
            is_expanded: false,
            folder_state: FolderState::Loaded,
            icon_open: None,
            icon_closed: None,
            icon_file: None,
            icon_glyph: None,
            icon_font_family: None,
            icon_color: None,
        }
    }

    pub fn with_icon(mut self, icon: LucideIcon) -> Self {
        self.icon_file = Some(icon);
        self
    }

    pub fn with_folder_icons(mut self, open: LucideIcon, closed: LucideIcon) -> Self {
        self.icon_open = Some(open);
        self.icon_closed = Some(closed);
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct TreeViewerState {
    /// The flattened list of ALL nodes currently available.
    /// Only visible ones (where parents are expanded) should be placed here,
    /// OR the widget can filter them. For best performance, `nodes` should
    /// ONLY contain nodes that are not hidden by a collapsed parent.
    /// This makes rendering O(1) just by slicing.
    pub nodes: Vec<FlatNode>,
    pub selected_path: Option<String>,
}

impl TreeViewerState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn select(&mut self, path: &str) {
        self.selected_path = Some(path.to_string());
    }

    pub fn is_selected(&self, path: &str) -> bool {
        self.selected_path.as_deref() == Some(path)
    }
}
