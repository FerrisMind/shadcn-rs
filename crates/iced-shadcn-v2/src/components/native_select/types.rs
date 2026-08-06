//! Configuration types used by the native-select component.

use std::fmt;

/// Preset control size for a [`super::NativeSelect`].
///
/// The web component ships `size="sm" | "default"`; [`NativeSelectSize::Lg`]
/// is an iced extension reusing the pack's control-height ladder so a select
/// can line up with [`crate::Button`] rows of any size.
///
/// ```rust
/// use iced_shadcn_v2::NativeSelectSize;
///
/// assert_eq!(NativeSelectSize::default(), NativeSelectSize::Default);
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum NativeSelectSize {
    /// Compact control height (`data-[size=sm]`, pack `control_height_sm`).
    Sm,
    /// Default control height matching `.cn-native-select` (`h-9` on Vega).
    #[default]
    Default,
    /// Tall control height (pack `control_height_lg`; iced extension).
    Lg,
}

/// Border radius preset for a [`super::NativeSelect`].
///
/// When no radius is set, the default follows the active style pack's
/// `.cn-native-select` corner treatment (`rounded-md` on Vega, pill on
/// Maia/Luma, square on Lyra/Sera, …).
///
/// ```rust
/// use iced_shadcn_v2::NativeSelectRadius;
///
/// assert!(NativeSelectRadius::None < NativeSelectRadius::Full);
/// assert_eq!(NativeSelectRadius::default(), NativeSelectRadius::Medium);
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum NativeSelectRadius {
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

/// One selectable entry, mirroring the web `NativeSelect.Option`.
///
/// The label is explicit instead of `Display`-derived so values stay plain
/// data (ids, enums) while labels remain presentation.
///
/// ```rust
/// use iced_shadcn_v2::NativeSelectOption;
///
/// let option = NativeSelectOption::new("apple", "Apple").disabled(true);
/// assert!(option.is_disabled());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[must_use = "options do nothing unless pushed into a NativeSelect"]
pub struct NativeSelectOption<T> {
    pub(super) value: T,
    pub(super) label: String,
    pub(super) disabled: bool,
}

impl<T> NativeSelectOption<T> {
    /// Creates an option with a value and its visible label.
    pub fn new(value: T, label: impl Into<String>) -> Self {
        Self {
            value,
            label: label.into(),
            disabled: false,
        }
    }

    /// Disables the option (`<option disabled>`): grayed out, not selectable.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Whether the option was marked disabled.
    pub fn is_disabled(&self) -> bool {
        self.disabled
    }

    /// The visible label of the option.
    pub fn label(&self) -> &str {
        &self.label
    }

    /// The value produced when the option is selected.
    pub fn value(&self) -> &T {
        &self.value
    }
}

impl<T, S> From<(T, S)> for NativeSelectOption<T>
where
    S: Into<String>,
{
    fn from((value, label): (T, S)) -> Self {
        Self::new(value, label)
    }
}

/// A labelled group of options, mirroring the web `NativeSelect.OptGroup`.
///
/// ```rust
/// use iced_shadcn_v2::{NativeSelectGroup, NativeSelectOption};
///
/// let group = NativeSelectGroup::new("Fruits")
///     .option(NativeSelectOption::new("apple", "Apple"))
///     .option(("banana", "Banana"));
/// assert_eq!(group.len(), 2);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[must_use = "groups do nothing unless pushed into a NativeSelect"]
pub struct NativeSelectGroup<T> {
    pub(super) label: String,
    pub(super) disabled: bool,
    pub(super) options: Vec<NativeSelectOption<T>>,
}

impl<T> NativeSelectGroup<T> {
    /// Creates an empty group with the given `<optgroup label>`.
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            disabled: false,
            options: Vec::new(),
        }
    }

    /// Appends one option to the group.
    pub fn option(mut self, option: impl Into<NativeSelectOption<T>>) -> Self {
        self.options.push(option.into());
        self
    }

    /// Appends every option of the iterator to the group.
    pub fn options<I, O>(mut self, options: I) -> Self
    where
        I: IntoIterator<Item = O>,
        O: Into<NativeSelectOption<T>>,
    {
        self.options.extend(options.into_iter().map(Into::into));
        self
    }

    /// Disables the group (`<optgroup disabled>`): every nested option is
    /// grayed out and not selectable.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Whether the group was marked disabled.
    pub fn is_disabled(&self) -> bool {
        self.disabled
    }

    /// The `<optgroup label>` text.
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Number of options in the group.
    pub fn len(&self) -> usize {
        self.options.len()
    }

    /// Whether the group holds no options.
    pub fn is_empty(&self) -> bool {
        self.options.is_empty()
    }
}

/// One flattened dropdown row: either a group heading or an option.
#[derive(Debug, Clone)]
pub(super) enum Row<T> {
    /// Non-interactive `<optgroup>` heading.
    GroupLabel { label: String },
    /// Selectable `<option>`; `disabled` folds in the parent group state.
    Option {
        value: T,
        label: String,
        disabled: bool,
        indented: bool,
    },
}

impl<T> Row<T> {
    /// The text painted for the row.
    pub(super) fn label(&self) -> &str {
        match self {
            Self::GroupLabel { label } | Self::Option { label, .. } => label,
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

    /// Whether the row text is inset under a group heading.
    pub(super) fn is_indented(&self) -> bool {
        matches!(self, Self::Option { indented: true, .. })
    }
}

impl<T: fmt::Display> fmt::Display for Row<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.label())
    }
}
