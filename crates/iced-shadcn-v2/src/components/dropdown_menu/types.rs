//! Configuration and entry types used by the dropdown-menu component.

use std::fmt;

use shadcn_common::{MenuActivateKind, MenuItemVariant};

/// Visual variant of a dropdown-menu item (`data-variant`).
///
/// Alias of [`MenuItemVariant`] from `shadcn-common` so iced consumers can
/// import it from this crate.
pub type DropdownMenuItemVariant = MenuItemVariant;

/// One plain menu action (`DropdownMenu.Item`).
#[derive(Clone)]
#[must_use = "items do nothing unless pushed into a DropdownMenu"]
pub struct DropdownMenuItem<Message> {
    pub(super) label: String,
    pub(super) shortcut: Option<String>,
    pub(super) variant: MenuItemVariant,
    pub(super) disabled: bool,
    pub(super) inset: bool,
    pub(super) close_on_select: bool,
    pub(super) on_select: Option<Message>,
}

impl<Message> fmt::Debug for DropdownMenuItem<Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DropdownMenuItem")
            .field("label", &self.label)
            .field("shortcut", &self.shortcut)
            .field("variant", &self.variant)
            .field("disabled", &self.disabled)
            .field("inset", &self.inset)
            .field("close_on_select", &self.close_on_select)
            .field("on_select", &self.on_select.is_some())
            .finish()
    }
}

impl<Message> DropdownMenuItem<Message> {
    /// Creates an item with the given visible label.
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            shortcut: None,
            variant: MenuItemVariant::Default,
            disabled: false,
            inset: false,
            close_on_select: MenuActivateKind::Item.closes_menu_by_default(),
            on_select: None,
        }
    }

    /// Sets the keyboard shortcut hint (`DropdownMenu.Shortcut`).
    pub fn shortcut(mut self, shortcut: impl Into<String>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }

    /// Sets the visual variant (`default` | `destructive`).
    pub fn variant(mut self, variant: MenuItemVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Disables the item (`data-disabled`).
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Indents the item to align with checkbox/radio rows (`data-inset`).
    pub fn inset(mut self, inset: bool) -> Self {
        self.inset = inset;
        self
    }

    /// Whether activating the item should close the menu.
    ///
    /// Defaults to `true`, matching bits-ui's plain item behaviour.
    pub fn close_on_select(mut self, close_on_select: bool) -> Self {
        self.close_on_select = close_on_select;
        self
    }

    /// Sets the message published when the item is activated.
    pub fn on_select(mut self, message: Message) -> Self {
        self.on_select = Some(message);
        self
    }

    /// Sets or clears the activation message.
    pub fn on_select_maybe(mut self, message: Option<Message>) -> Self {
        self.on_select = message;
        self
    }

    /// The visible label.
    #[must_use]
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Whether the item is disabled.
    #[must_use]
    pub fn is_disabled(&self) -> bool {
        self.disabled
    }
}

/// One checkbox row (`DropdownMenu.CheckboxItem`).
#[derive(Clone)]
#[must_use = "checkbox items do nothing unless pushed into a DropdownMenu"]
pub struct DropdownMenuCheckboxItem<Message> {
    pub(super) label: String,
    pub(super) checked: bool,
    pub(super) disabled: bool,
    pub(super) inset: bool,
    pub(super) on_toggle: Option<Message>,
}

impl<Message> fmt::Debug for DropdownMenuCheckboxItem<Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DropdownMenuCheckboxItem")
            .field("label", &self.label)
            .field("checked", &self.checked)
            .field("disabled", &self.disabled)
            .field("inset", &self.inset)
            .field("on_toggle", &self.on_toggle.is_some())
            .finish()
    }
}

impl<Message> DropdownMenuCheckboxItem<Message> {
    /// Creates a checkbox item with the given label and checked state.
    pub fn new(label: impl Into<String>, checked: bool) -> Self {
        Self {
            label: label.into(),
            checked,
            disabled: false,
            inset: false,
            on_toggle: None,
        }
    }

    /// Disables the item.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Indents the item (`data-inset`).
    pub fn inset(mut self, inset: bool) -> Self {
        self.inset = inset;
        self
    }

    /// Sets the message published when the checkbox is toggled.
    ///
    /// The application owns `checked` and feeds the next value back on the
    /// following frame (controlled, like `bind:checked`).
    pub fn on_toggle(mut self, message: Message) -> Self {
        self.on_toggle = Some(message);
        self
    }

    /// Sets or clears the toggle message.
    pub fn on_toggle_maybe(mut self, message: Option<Message>) -> Self {
        self.on_toggle = message;
        self
    }

    /// The visible label.
    #[must_use]
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Whether the checkbox is currently checked.
    #[must_use]
    pub fn is_checked(&self) -> bool {
        self.checked
    }
}

/// One radio row (`DropdownMenu.RadioItem`).
#[derive(Clone)]
#[must_use = "radio items do nothing unless pushed into a DropdownMenu"]
pub struct DropdownMenuRadioItem<Message> {
    pub(super) label: String,
    pub(super) selected: bool,
    pub(super) disabled: bool,
    pub(super) inset: bool,
    pub(super) close_on_select: bool,
    pub(super) on_select: Option<Message>,
}

impl<Message> fmt::Debug for DropdownMenuRadioItem<Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DropdownMenuRadioItem")
            .field("label", &self.label)
            .field("selected", &self.selected)
            .field("disabled", &self.disabled)
            .field("inset", &self.inset)
            .field("close_on_select", &self.close_on_select)
            .field("on_select", &self.on_select.is_some())
            .finish()
    }
}

impl<Message> DropdownMenuRadioItem<Message> {
    /// Creates a radio item with the given label and selected state.
    pub fn new(label: impl Into<String>, selected: bool) -> Self {
        Self {
            label: label.into(),
            selected,
            disabled: false,
            inset: false,
            close_on_select: MenuActivateKind::Radio.closes_menu_by_default(),
            on_select: None,
        }
    }

    /// Disables the item.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Indents the item (`data-inset`).
    pub fn inset(mut self, inset: bool) -> Self {
        self.inset = inset;
        self
    }

    /// Whether activating the radio should close the menu.
    pub fn close_on_select(mut self, close_on_select: bool) -> Self {
        self.close_on_select = close_on_select;
        self
    }

    /// Sets the message published when the radio is chosen.
    pub fn on_select(mut self, message: Message) -> Self {
        self.on_select = Some(message);
        self
    }

    /// Sets or clears the selection message.
    pub fn on_select_maybe(mut self, message: Option<Message>) -> Self {
        self.on_select = message;
        self
    }

    /// The visible label.
    #[must_use]
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Whether this radio is the selected value.
    #[must_use]
    pub fn is_selected(&self) -> bool {
        self.selected
    }
}

/// Non-interactive section heading (`DropdownMenu.Label` / `GroupHeading`).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[must_use = "labels do nothing unless pushed into a DropdownMenu"]
pub struct DropdownMenuLabel {
    pub(super) text: String,
    pub(super) inset: bool,
}

impl DropdownMenuLabel {
    /// Creates a section label.
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            inset: false,
        }
    }

    /// Indents the label (`data-inset`).
    pub fn inset(mut self, inset: bool) -> Self {
        self.inset = inset;
        self
    }

    /// The visible text.
    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }
}

/// Nested submenu (`DropdownMenu.Sub` + `SubTrigger` + `SubContent`).
#[derive(Clone)]
#[must_use = "submenus do nothing unless pushed into a DropdownMenu"]
pub struct DropdownMenuSub<Message> {
    pub(super) label: String,
    pub(super) disabled: bool,
    pub(super) inset: bool,
    pub(super) entries: Vec<Entry<Message>>,
}

impl<Message> fmt::Debug for DropdownMenuSub<Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DropdownMenuSub")
            .field("label", &self.label)
            .field("disabled", &self.disabled)
            .field("inset", &self.inset)
            .field("entries", &self.entries.len())
            .finish()
    }
}

impl<Message> DropdownMenuSub<Message> {
    /// Creates an empty submenu with the given trigger label.
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            disabled: false,
            inset: false,
            entries: Vec::new(),
        }
    }

    /// Disables the submenu trigger.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Indents the submenu trigger (`data-inset`).
    pub fn inset(mut self, inset: bool) -> Self {
        self.inset = inset;
        self
    }

    /// Appends a plain item.
    pub fn item(mut self, item: DropdownMenuItem<Message>) -> Self {
        self.entries.push(Entry::Item(item));
        self
    }

    /// Appends a checkbox item.
    pub fn checkbox_item(mut self, item: DropdownMenuCheckboxItem<Message>) -> Self {
        self.entries.push(Entry::Checkbox(item));
        self
    }

    /// Appends a radio item.
    pub fn radio_item(mut self, item: DropdownMenuRadioItem<Message>) -> Self {
        self.entries.push(Entry::Radio(item));
        self
    }

    /// Appends a section label.
    pub fn label(mut self, label: impl Into<DropdownMenuLabel>) -> Self {
        self.entries.push(Entry::Label(label.into()));
        self
    }

    /// Appends a hairline separator.
    pub fn separator(mut self) -> Self {
        self.entries.push(Entry::Separator);
        self
    }

    /// Appends a nested submenu.
    pub fn submenu(mut self, submenu: DropdownMenuSub<Message>) -> Self {
        self.entries.push(Entry::Sub(submenu));
        self
    }

    /// The trigger label.
    #[must_use]
    pub fn label_text(&self) -> &str {
        &self.label
    }

    /// Number of child entries.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether the submenu has no child entries.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

impl From<&str> for DropdownMenuLabel {
    fn from(text: &str) -> Self {
        Self::new(text)
    }
}

impl From<String> for DropdownMenuLabel {
    fn from(text: String) -> Self {
        Self::new(text)
    }
}

/// One top-level or nested menu entry owned by the builder.
#[derive(Clone)]
pub(super) enum Entry<Message> {
    Label(DropdownMenuLabel),
    Separator,
    Item(DropdownMenuItem<Message>),
    Checkbox(DropdownMenuCheckboxItem<Message>),
    Radio(DropdownMenuRadioItem<Message>),
    Sub(DropdownMenuSub<Message>),
}

impl<Message> fmt::Debug for Entry<Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Label(label) => formatter.debug_tuple("Label").field(label).finish(),
            Self::Separator => formatter.write_str("Separator"),
            Self::Item(item) => formatter.debug_tuple("Item").field(item).finish(),
            Self::Checkbox(item) => formatter.debug_tuple("Checkbox").field(item).finish(),
            Self::Radio(item) => formatter.debug_tuple("Radio").field(item).finish(),
            Self::Sub(sub) => formatter.debug_tuple("Sub").field(sub).finish(),
        }
    }
}

impl<Message> Entry<Message> {
    /// Whether hover / click / keyboard navigation may land on the entry.
    pub(super) fn is_selectable(&self) -> bool {
        match self {
            Self::Item(item) => !item.disabled,
            Self::Checkbox(item) => !item.disabled,
            Self::Radio(item) => !item.disabled,
            Self::Sub(sub) => !sub.disabled,
            Self::Label(_) | Self::Separator => false,
        }
    }

    /// Whether this entry opens a nested menu.
    pub(super) fn is_submenu(&self) -> bool {
        matches!(self, Self::Sub(_))
    }
}
