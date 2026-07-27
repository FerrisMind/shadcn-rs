//! Shared Lucide icon name catalog (backend loads `lucide-icons` separately).

/// Stable Lucide icon identifiers used across iced/egui demos and components.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IconName {
    Activity,
    ArrowLeft,
    ArrowRight,
    Bell,
    BookOpen,
    Calendar,
    Check,
    ChevronDown,
    ChevronRight,
    Circle,
    Copy,
    Download,
    Ellipsis,
    Github,
    Home,
    Info,
    Menu,
    Moon,
    Palette,
    Plus,
    Search,
    Settings,
    Sun,
    Trash,
    User,
    X,
}

impl IconName {
    pub const ALL: [Self; 26] = [
        Self::Activity,
        Self::ArrowLeft,
        Self::ArrowRight,
        Self::Bell,
        Self::BookOpen,
        Self::Calendar,
        Self::Check,
        Self::ChevronDown,
        Self::ChevronRight,
        Self::Circle,
        Self::Copy,
        Self::Download,
        Self::Ellipsis,
        Self::Github,
        Self::Home,
        Self::Info,
        Self::Menu,
        Self::Moon,
        Self::Palette,
        Self::Plus,
        Self::Search,
        Self::Settings,
        Self::Sun,
        Self::Trash,
        Self::User,
        Self::X,
    ];

    /// Lucide kebab-case name.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Activity => "activity",
            Self::ArrowLeft => "arrow-left",
            Self::ArrowRight => "arrow-right",
            Self::Bell => "bell",
            Self::BookOpen => "book-open",
            Self::Calendar => "calendar",
            Self::Check => "check",
            Self::ChevronDown => "chevron-down",
            Self::ChevronRight => "chevron-right",
            Self::Circle => "circle",
            Self::Copy => "copy",
            Self::Download => "download",
            Self::Ellipsis => "ellipsis",
            Self::Github => "github",
            Self::Home => "home",
            Self::Info => "info",
            Self::Menu => "menu",
            Self::Moon => "moon",
            Self::Palette => "palette",
            Self::Plus => "plus",
            Self::Search => "search",
            Self::Settings => "settings",
            Self::Sun => "sun",
            Self::Trash => "trash",
            Self::User => "user",
            Self::X => "x",
        }
    }
}

/// Default icon set for shadcn-rs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct IconSet;

impl IconSet {
    pub const fn names(self) -> &'static [IconName] {
        &IconName::ALL
    }
}
