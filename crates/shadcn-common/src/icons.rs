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
    Eye,
    EyeOff,
    Github,
    Home,
    Info,
    Menu,
    Moon,
    Palette,
    Plus,
    Search,
    Settings,
    Star,
    Sun,
    Trash,
    User,
    X,
}

impl IconName {
    pub const ALL: [Self; 29] = [
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
        Self::Eye,
        Self::EyeOff,
        Self::Github,
        Self::Home,
        Self::Info,
        Self::Menu,
        Self::Moon,
        Self::Palette,
        Self::Plus,
        Self::Search,
        Self::Settings,
        Self::Star,
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
            Self::Eye => "eye",
            Self::EyeOff => "eye-off",
            Self::Github => "github",
            Self::Home => "home",
            Self::Info => "info",
            Self::Menu => "menu",
            Self::Moon => "moon",
            Self::Palette => "palette",
            Self::Plus => "plus",
            Self::Search => "search",
            Self::Settings => "settings",
            Self::Star => "star",
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
