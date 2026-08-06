//! Configuration and value types used by the select component.

use std::fmt;

use shadcn_common::{SelectMode, next_multiple_values, next_single_value};

/// Preset control size for a [`super::Select`] trigger.
///
/// Matches the web `size="sm" | "default"` prop.
///
/// ```rust
/// use iced_shadcn_v2::SelectSize;
///
/// assert_eq!(SelectSize::default(), SelectSize::Default);
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SelectSize {
    /// Compact control height (`data-[size=sm]`).
    Sm,
    /// Default control height matching `.cn-select-trigger` (`h-9` on Vega).
    #[default]
    Default,
}

/// Border radius preset for a [`super::Select`] trigger.
///
/// When no radius is set, the active style pack decides.
///
/// ```rust
/// use iced_shadcn_v2::SelectRadius;
///
/// assert_eq!(SelectRadius::default(), SelectRadius::Medium);
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SelectRadius {
    /// No corner radius.
    None,
    /// Small corner radius.
    Small,
    /// Medium corner radius.
    #[default]
    Medium,
    /// Large corner radius.
    Large,
    /// Fully rounded (pill) corners.
    Full,
}

/// Selection mode of a [`super::Select`] (`type="single" | "multiple"`).
///
/// Alias of [`SelectMode`] from `shadcn-common` so iced consumers can import
/// it from this crate.
pub type SelectType = SelectMode;

/// Controlled selection value emitted by a [`super::Select`].
///
/// Mirrors the discriminated `value` prop from shadcn-svelte while keeping
/// the typed option payload of [`super::SelectItem`].
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(bound(
        serialize = "T: serde::Serialize",
        deserialize = "T: serde::Deserialize<'de>"
    ))
)]
pub enum SelectSelection<T> {
    /// The selected value, or `None` when empty.
    Single(Option<T>),
    /// The ordered selected values.
    Multiple(Vec<T>),
}

impl<T> Default for SelectSelection<T> {
    fn default() -> Self {
        Self::Single(None)
    }
}

impl<T> SelectSelection<T> {
    /// Creates a single-selection value.
    #[must_use]
    pub fn single(value: Option<T>) -> Self {
        Self::Single(value)
    }

    /// Creates a multiple-selection value, removing duplicates while
    /// preserving first-seen order.
    #[must_use]
    pub fn multiple(values: impl IntoIterator<Item = T>) -> Self
    where
        T: PartialEq,
    {
        let mut selected = Vec::new();

        for value in values {
            if !selected.iter().any(|existing| existing == &value) {
                selected.push(value);
            }
        }

        Self::Multiple(selected)
    }

    /// Returns the selected single value when this is [`Self::Single`].
    #[must_use]
    pub fn as_single(&self) -> Option<&T> {
        match self {
            Self::Single(value) => value.as_ref(),
            Self::Multiple(_) => None,
        }
    }

    /// Returns the selected values when this is [`Self::Multiple`].
    #[must_use]
    pub fn as_multiple(&self) -> &[T] {
        match self {
            Self::Single(_) => &[],
            Self::Multiple(values) => values,
        }
    }

    /// Whether `value` is currently selected.
    #[must_use]
    pub fn is_selected(&self, value: &T) -> bool
    where
        T: PartialEq,
    {
        match self {
            Self::Single(selected) => selected.as_ref() == Some(value),
            Self::Multiple(selected) => selected.iter().any(|item| item == value),
        }
    }

    /// Selection mode represented by this value.
    #[must_use]
    pub const fn selection_type(&self) -> SelectType {
        match self {
            Self::Single(_) => SelectMode::Single,
            Self::Multiple(_) => SelectMode::Multiple,
        }
    }

    /// Applies a pick of `value`, returning the next controlled selection.
    ///
    /// Single mode replaces (or clears when `deselectable` and already
    /// selected). Multiple mode toggles membership.
    #[must_use]
    pub fn toggled(self, mode: SelectType, value: &T, deselectable: bool) -> Self
    where
        T: Clone + PartialEq,
    {
        match mode {
            SelectMode::Single => {
                Self::Single(next_single_value(self.as_single(), value, deselectable))
            }
            SelectMode::Multiple => {
                let current = match self {
                    Self::Multiple(values) => values,
                    Self::Single(Some(single)) => vec![single],
                    Self::Single(None) => Vec::new(),
                };

                Self::Multiple(next_multiple_values(&current, value))
            }
            _ => self,
        }
    }

    /// Number of selected values.
    #[must_use]
    pub fn len(&self) -> usize {
        match self {
            Self::Single(value) => usize::from(value.is_some()),
            Self::Multiple(values) => values.len(),
        }
    }

    /// Whether nothing is selected.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// One selectable entry, mirroring the web `Select.Item`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[must_use = "items do nothing unless pushed into a Select"]
pub struct SelectItem<T> {
    pub(super) value: T,
    pub(super) label: String,
    pub(super) disabled: bool,
}

impl<T> SelectItem<T> {
    /// Creates an item with a value and its visible label.
    pub fn new(value: T, label: impl Into<String>) -> Self {
        Self {
            value,
            label: label.into(),
            disabled: false,
        }
    }

    /// Disables the item (`disabled` prop): grayed out, not selectable.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Whether the item was marked disabled.
    #[must_use]
    pub fn is_disabled(&self) -> bool {
        self.disabled
    }

    /// The visible label of the item.
    #[must_use]
    pub fn label(&self) -> &str {
        &self.label
    }

    /// The value produced when the item is selected.
    #[must_use]
    pub fn value(&self) -> &T {
        &self.value
    }
}

impl<T, S> From<(T, S)> for SelectItem<T>
where
    S: Into<String>,
{
    fn from((value, label): (T, S)) -> Self {
        Self::new(value, label)
    }
}

/// A labelled group of items, mirroring the web `Select.Group` + `Select.Label`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[must_use = "groups do nothing unless pushed into a Select"]
pub struct SelectGroup<T> {
    pub(super) label: Option<String>,
    pub(super) items: Vec<SelectItem<T>>,
}

impl<T> SelectGroup<T> {
    /// Creates an empty group with an optional heading label.
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: Some(label.into()),
            items: Vec::new(),
        }
    }

    /// Creates an empty group without a heading.
    pub fn unlabeled() -> Self {
        Self {
            label: None,
            items: Vec::new(),
        }
    }

    /// Appends one item to the group.
    pub fn item(mut self, item: impl Into<SelectItem<T>>) -> Self {
        self.items.push(item.into());
        self
    }

    /// Appends every item of the iterator to the group.
    pub fn items<I, O>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = O>,
        O: Into<SelectItem<T>>,
    {
        self.items.extend(items.into_iter().map(Into::into));
        self
    }

    /// The group heading text, if any.
    #[must_use]
    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    /// Number of items in the group.
    #[must_use]
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Whether the group holds no items.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

/// One flattened dropdown row: label, separator, or option.
#[derive(Debug, Clone)]
pub(super) enum Row<T> {
    /// Non-interactive `.cn-select-label` heading.
    Label { text: String },
    /// Non-interactive `.cn-select-separator` hairline.
    Separator,
    /// Selectable `.cn-select-item`.
    Option {
        value: T,
        label: String,
        disabled: bool,
    },
}

impl<T> Row<T> {
    /// The text painted for the row, if any.
    pub(super) fn label(&self) -> Option<&str> {
        match self {
            Self::Label { text } => Some(text),
            Self::Option { label, .. } => Some(label),
            Self::Separator => None,
        }
    }

    /// Whether hover / click / keyboard navigation may land on the row.
    pub(super) fn is_selectable(&self) -> bool {
        matches!(
            self,
            Self::Option {
                disabled: false,
                ..
            }
        )
    }
}

impl<T: fmt::Display> fmt::Display for Row<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.label() {
            Some(label) => formatter.write_str(label),
            None => formatter.write_str("—"),
        }
    }
}
