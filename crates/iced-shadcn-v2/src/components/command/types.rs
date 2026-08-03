//! Public configuration types for [`super::Command`].

use std::borrow::Cow;

/// Built-in lucide-style glyphs used by command rows and the search field.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum CommandGlyph {
    /// Magnifying glass (`.cn-command-input-icon`).
    #[default]
    Search,
    /// Trailing check indicator on checked items.
    Check,
    /// Calendar suggestion icon.
    Calendar,
    /// Emoji / smile suggestion icon.
    Smile,
    /// Calculator suggestion icon.
    Calculator,
    /// Profile / user settings icon.
    User,
    /// Billing / credit-card icon.
    CreditCard,
    /// Settings / gear icon.
    Settings,
    /// Documentation / book icon.
    BookOpen,
}

/// A selectable command row (`Command.Item` / `Command.LinkItem`).
#[derive(Debug, Clone)]
pub struct CommandItem<T> {
    /// Stable value emitted on select.
    pub value: T,
    /// Visible label.
    pub label: String,
    /// Optional secondary line rendered below the label.
    pub description: Option<String>,
    /// Extra search keywords (cmdk `keywords`).
    pub keywords: Vec<String>,
    /// Optional shortcut chip text (`.cn-command-shortcut`).
    pub shortcut: Option<String>,
    /// Optional href for link items (`Command.LinkItem`).
    pub href: Option<String>,
    /// Optional leading glyph.
    pub icon: Option<CommandGlyph>,
    /// Soft-disabled rows stay visible but are not selectable.
    pub disabled: bool,
    /// Keep mounted even when filtered out (`forceMount`).
    pub force_mount: bool,
    /// Marks the item as checked (shows the trailing check indicator).
    pub checked: bool,
    /// Reserves a leading check-indicator slot.
    ///
    /// When enabled, the check glyph is painted before the label and becomes
    /// transparent while [`Self::checked`] is false. This is useful for
    /// composed controls such as comboboxes whose selected indicator lives
    /// before the option label in the source component.
    pub leading_check: bool,
}

impl<T> CommandItem<T> {
    /// Creates an item with `value` and `label`.
    #[must_use]
    pub fn new(value: T, label: impl Into<String>) -> Self {
        Self {
            value,
            label: label.into(),
            description: None,
            keywords: Vec::new(),
            shortcut: None,
            href: None,
            icon: None,
            disabled: false,
            force_mount: false,
            checked: false,
            leading_check: false,
        }
    }

    /// Appends search keywords.
    #[must_use]
    pub fn keywords(mut self, keywords: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.keywords = keywords.into_iter().map(Into::into).collect();
        self
    }

    /// Sets the optional secondary line shown below the label.
    #[must_use]
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets the trailing shortcut label.
    #[must_use]
    pub fn shortcut(mut self, shortcut: impl Into<String>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }

    /// Marks the item as a link with `href`.
    #[must_use]
    pub fn href(mut self, href: impl Into<String>) -> Self {
        self.href = Some(href.into());
        self
    }

    /// Sets the leading glyph.
    #[must_use]
    pub fn icon(mut self, icon: CommandGlyph) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Disables selection while keeping the row visible.
    #[must_use]
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Keeps the item mounted when filtered out.
    #[must_use]
    pub fn force_mount(mut self, force_mount: bool) -> Self {
        self.force_mount = force_mount;
        self
    }

    /// Shows the trailing check indicator (`data-checked`).
    #[must_use]
    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    /// Shows a leading check-indicator slot, including when unchecked.
    #[must_use]
    pub fn leading_check(mut self, leading_check: bool) -> Self {
        self.leading_check = leading_check;
        self
    }
}

impl<T> From<(T, &str)> for CommandItem<T> {
    fn from((value, label): (T, &str)) -> Self {
        Self::new(value, label)
    }
}

impl<T> From<(T, String)> for CommandItem<T> {
    fn from((value, label): (T, String)) -> Self {
        Self::new(value, label)
    }
}

/// A labelled group of command entries (`Command.Group`).
#[derive(Debug, Clone)]
pub struct CommandGroup<T> {
    /// Optional heading (`Command.GroupHeading`).
    pub heading: Option<String>,
    /// Optional group filter value; defaults to the heading.
    pub value: Option<String>,
    /// Keep the group mounted when every child is filtered out.
    pub force_mount: bool,
    /// Nested entries (items, separators, loading).
    pub entries: Vec<CommandEntry<T>>,
}

impl<T> CommandGroup<T> {
    /// Creates an empty group with a heading.
    #[must_use]
    pub fn new(heading: impl Into<String>) -> Self {
        Self {
            heading: Some(heading.into()),
            value: None,
            force_mount: false,
            entries: Vec::new(),
        }
    }

    /// Creates a group without a heading.
    #[must_use]
    pub fn untitled() -> Self {
        Self {
            heading: None,
            value: None,
            force_mount: false,
            entries: Vec::new(),
        }
    }

    /// Sets the group filter value.
    #[must_use]
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Keeps the group mounted when filtered out.
    #[must_use]
    pub fn force_mount(mut self, force_mount: bool) -> Self {
        self.force_mount = force_mount;
        self
    }

    /// Appends one item.
    #[must_use]
    pub fn item(mut self, item: impl Into<CommandItem<T>>) -> Self {
        self.entries.push(CommandEntry::Item(item.into()));
        self
    }

    /// Appends every item of the iterator.
    #[must_use]
    pub fn items<I, O>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = O>,
        O: Into<CommandItem<T>>,
    {
        for item in items {
            self = self.item(item);
        }
        self
    }

    /// Appends a separator inside the group.
    #[must_use]
    pub fn separator(mut self) -> Self {
        self.entries
            .push(CommandEntry::Separator { force_mount: false });
        self
    }
}

/// Loading placeholder row (`Command.Loading`).
#[derive(Debug, Clone)]
pub struct CommandLoading {
    /// Accessible / visible label.
    pub label: String,
    /// Optional determinate progress in `[0.0, 1.0]`.
    pub progress: Option<f32>,
}

impl CommandLoading {
    /// Creates a loading row.
    #[must_use]
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            progress: None,
        }
    }

    /// Sets determinate progress.
    #[must_use]
    pub fn progress(mut self, progress: f32) -> Self {
        self.progress = Some(progress.clamp(0.0, 1.0));
        self
    }
}

/// One top-level or nested list entry.
#[derive(Debug, Clone)]
pub enum CommandEntry<T> {
    /// Selectable item.
    Item(CommandItem<T>),
    /// Group with optional heading.
    Group(CommandGroup<T>),
    /// Horizontal rule (`Command.Separator`).
    Separator {
        /// Keep mounted when filtered out.
        force_mount: bool,
    },
    /// Loading placeholder.
    Loading(CommandLoading),
}

/// Empty-state copy shown when nothing matches (`Command.Empty`).
#[derive(Debug, Clone)]
pub struct CommandEmpty {
    /// Visible text.
    pub text: String,
    /// Keep mounted even when results exist.
    pub force_mount: bool,
}

impl CommandEmpty {
    /// Creates an empty-state message.
    #[must_use]
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            force_mount: false,
        }
    }

    /// Keeps the empty slot mounted.
    #[must_use]
    pub fn force_mount(mut self, force_mount: bool) -> Self {
        self.force_mount = force_mount;
        self
    }
}

impl From<&str> for CommandEmpty {
    fn from(text: &str) -> Self {
        Self::new(text)
    }
}

impl From<String> for CommandEmpty {
    fn from(text: String) -> Self {
        Self::new(text)
    }
}

impl From<Cow<'_, str>> for CommandEmpty {
    fn from(text: Cow<'_, str>) -> Self {
        Self::new(text.into_owned())
    }
}

/// Corner-radius override for the command surface.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[non_exhaustive]
pub enum CommandRadius {
    /// Style-pack default.
    #[default]
    Default,
    /// `rounded-none`.
    None,
    /// `rounded-sm`.
    Sm,
    /// `rounded-md`.
    Md,
    /// `rounded-lg`.
    Lg,
    /// `rounded-xl`.
    Xl,
    /// `rounded-2xl`.
    S2xl,
    /// `rounded-3xl`.
    S3xl,
    /// `rounded-4xl`.
    S4xl,
}

impl CommandRadius {
    pub(super) fn to_component(self) -> Option<shadcn_common::ComponentRadius> {
        use shadcn_common::ComponentRadius;
        match self {
            Self::Default => None,
            Self::None => Some(ComponentRadius::None),
            Self::Sm => Some(ComponentRadius::Sm),
            Self::Md => Some(ComponentRadius::Md),
            Self::Lg => Some(ComponentRadius::Lg),
            Self::Xl => Some(ComponentRadius::Xl),
            Self::S2xl => Some(ComponentRadius::S2xl),
            Self::S3xl => Some(ComponentRadius::S3xl),
            Self::S4xl => Some(ComponentRadius::S4xl),
        }
    }
}
