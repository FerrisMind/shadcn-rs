//! Public configuration and value types for [`super::ToggleGroup`].

/// Selection mode of a [`super::ToggleGroup`].
///
/// `Single` keeps zero or one selected item. `Multiple` keeps an ordered set
/// of selected item values.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ToggleGroupType {
    /// At most one item can be selected at a time.
    #[default]
    Single,
    /// Any number of items can be selected at the same time.
    Multiple,
}

impl ToggleGroupType {
    /// Returns `true` when this group accepts multiple selected values.
    #[must_use]
    pub const fn is_multiple(self) -> bool {
        matches!(self, Self::Multiple)
    }
}

/// Axis used to lay out a [`super::ToggleGroup`].
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ToggleGroupOrientation {
    /// Lay items out from left to right.
    #[default]
    Horizontal,
    /// Lay items out from top to bottom.
    Vertical,
}

impl ToggleGroupOrientation {
    /// Returns `true` when items are laid out vertically.
    #[must_use]
    pub const fn is_vertical(self) -> bool {
        matches!(self, Self::Vertical)
    }
}

/// Controlled selection value emitted by a [`super::ToggleGroup`].
///
/// The enum mirrors the discriminated `value` prop from shadcn-svelte while
/// retaining Rust's type safety. Values in `Multiple` preserve insertion
/// order and never contain duplicates.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ToggleGroupSelection {
    /// The selected value, or `None` when the group is empty.
    Single(Option<String>),
    /// The ordered selected values.
    Multiple(Vec<String>),
}

impl Default for ToggleGroupSelection {
    fn default() -> Self {
        Self::Single(None)
    }
}

impl ToggleGroupSelection {
    /// Creates a single-selection value.
    #[must_use]
    pub fn single(value: Option<impl Into<String>>) -> Self {
        Self::Single(value.map(Into::into))
    }

    /// Creates a multiple-selection value, removing duplicate values while
    /// preserving the order of their first occurrence.
    #[must_use]
    pub fn multiple(values: impl IntoIterator<Item = impl Into<String>>) -> Self {
        let mut selected = Vec::new();

        for value in values {
            let value = value.into();
            if !selected.iter().any(|existing| existing == &value) {
                selected.push(value);
            }
        }

        Self::Multiple(selected)
    }

    /// Returns the selected single value, if this is a `Single` selection.
    #[must_use]
    pub fn as_single(&self) -> Option<&str> {
        match self {
            Self::Single(value) => value.as_deref(),
            Self::Multiple(_) => None,
        }
    }

    /// Returns the selected values, if this is a `Multiple` selection.
    #[must_use]
    pub fn as_multiple(&self) -> &[String] {
        match self {
            Self::Single(_) => &[],
            Self::Multiple(values) => values,
        }
    }

    /// Returns whether `value` is selected.
    #[must_use]
    pub fn is_selected(&self, value: &str) -> bool {
        match self {
            Self::Single(selected) => selected.as_deref() == Some(value),
            Self::Multiple(selected) => selected.iter().any(|item| item == value),
        }
    }

    /// Returns the selection mode represented by this value.
    #[must_use]
    pub const fn selection_type(&self) -> super::ToggleGroupType {
        match self {
            Self::Single(_) => super::ToggleGroupType::Single,
            Self::Multiple(_) => super::ToggleGroupType::Multiple,
        }
    }

    pub(super) fn for_type(&self, group_type: super::ToggleGroupType) -> Self {
        match (group_type, self) {
            (super::ToggleGroupType::Single, Self::Single(value)) => Self::Single(value.clone()),
            (super::ToggleGroupType::Single, Self::Multiple(values)) => {
                Self::Single(values.first().cloned())
            }
            (super::ToggleGroupType::Multiple, Self::Single(value)) => {
                Self::multiple(value.iter().cloned())
            }
            (super::ToggleGroupType::Multiple, Self::Multiple(values)) => {
                Self::multiple(values.iter().cloned())
            }
        }
    }

    pub(super) fn toggled(&self, group_type: super::ToggleGroupType, value: &str) -> Self {
        let current = self.for_type(group_type);

        match current {
            Self::Single(selected) => {
                if selected.as_deref() == Some(value) {
                    Self::Single(None)
                } else {
                    Self::Single(Some(value.to_owned()))
                }
            }
            Self::Multiple(mut selected) => {
                if let Some(index) = selected.iter().position(|item| item == value) {
                    selected.remove(index);
                } else {
                    selected.push(value.to_owned());
                }

                Self::Multiple(selected)
            }
        }
    }
}

impl From<String> for ToggleGroupSelection {
    fn from(value: String) -> Self {
        Self::Single(Some(value))
    }
}

impl From<&str> for ToggleGroupSelection {
    fn from(value: &str) -> Self {
        Self::Single(Some(value.to_owned()))
    }
}

impl From<Option<String>> for ToggleGroupSelection {
    fn from(value: Option<String>) -> Self {
        Self::Single(value)
    }
}

impl<'a> From<Option<&'a str>> for ToggleGroupSelection {
    fn from(value: Option<&'a str>) -> Self {
        Self::Single(value.map(str::to_owned))
    }
}

impl From<Vec<String>> for ToggleGroupSelection {
    fn from(value: Vec<String>) -> Self {
        Self::multiple(value)
    }
}

impl<'a> From<Vec<&'a str>> for ToggleGroupSelection {
    fn from(value: Vec<&'a str>) -> Self {
        Self::multiple(value)
    }
}
