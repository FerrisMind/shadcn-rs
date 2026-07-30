//! Public configuration and option types for the native-select component.

use std::fmt;

use crate::theme::Theme;

/// Size of a [`super::NativeSelect`].
///
/// The values mirror shadcn-svelte's `size="sm"` and `size="default"`
/// attributes. The actual height comes from the active style pack so the
/// control stays aligned with other form controls.
///
/// ```rust
/// use iced_shadcn_v2::NativeSelectSize;
///
/// assert_eq!(NativeSelectSize::default(), NativeSelectSize::Default);
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum NativeSelectSize {
    /// Compact `h-*` control.
    Sm,
    /// Default control matching the active pack's native-select recipe.
    #[default]
    Default,
}

impl NativeSelectSize {
    /// Returns the control height in pixels for `theme`.
    pub(crate) fn control_height(self, theme: &Theme) -> f32 {
        match self {
            Self::Sm => theme.style.control_height_sm_px,
            Self::Default => theme.style.control_height_md_px,
        }
    }
}

/// Corner-radius override for a [`super::NativeSelect`].
///
/// Without an override, the active shadcn style pack supplies the radius, just
/// as the web component's CSS recipe does.
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
    /// Square corners.
    None,
    /// Small corners.
    Small,
    /// Medium corners.
    #[default]
    Medium,
    /// Large corners.
    Large,
    /// Fully rounded corners.
    Full,
}

/// One selectable option in a [`super::NativeSelect`].
///
/// `NativeSelectOption` is the typed equivalent of the web component's
/// `NativeSelect.Option`: `value` is returned to the selection callback while
/// `label` is rendered to the user.
///
/// ```rust
/// use iced_shadcn_v2::NativeSelectOption;
///
/// let option = NativeSelectOption::new("apple", "Apple").disabled(true);
/// assert_eq!(option.value(), &"apple");
/// assert!(option.is_disabled());
/// ```
#[must_use = "options do nothing unless added to a NativeSelect"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NativeSelectOption<T> {
    value: T,
    label: String,
    disabled: bool,
}

impl<T> NativeSelectOption<T> {
    /// Creates an enabled option.
    pub fn new(value: T, label: impl Into<String>) -> Self {
        Self {
            value,
            label: label.into(),
            disabled: false,
        }
    }

    /// Returns the typed value carried by this option.
    #[must_use]
    pub fn value(&self) -> &T {
        &self.value
    }

    /// Returns the text displayed for this option.
    #[must_use]
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Returns whether the option is disabled.
    #[must_use]
    pub const fn is_disabled(&self) -> bool {
        self.disabled
    }

    /// Sets the option's disabled state.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Consumes the option and returns its typed value.
    pub fn into_value(self) -> T {
        self.value
    }
}

/// A labelled group of options, equivalent to HTML `optgroup`.
///
/// Groups remain visible in the iced dropdown menu as inert headings. A
/// disabled group disables every option it contains, while an option can also
/// be disabled individually.
///
/// ```rust
/// use iced_shadcn_v2::{NativeSelectOptGroup, NativeSelectOption};
///
/// let group = NativeSelectOptGroup::new("Fruit")
///     .push(NativeSelectOption::new("apple", "Apple"));
/// assert_eq!(group.label(), "Fruit");
/// assert_eq!(group.options().len(), 1);
/// ```
#[must_use = "groups do nothing unless added to a NativeSelect"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NativeSelectOptGroup<T> {
    label: String,
    options: Vec<NativeSelectOption<T>>,
    disabled: bool,
}

impl<T> NativeSelectOptGroup<T> {
    /// Creates an empty enabled option group.
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            options: Vec::new(),
            disabled: false,
        }
    }

    /// Creates a group containing all options from `options`.
    pub fn with_options<I>(label: impl Into<String>, options: I) -> Self
    where
        I: IntoIterator<Item = NativeSelectOption<T>>,
    {
        Self {
            label: label.into(),
            options: options.into_iter().collect(),
            disabled: false,
        }
    }

    /// Appends one option.
    pub fn push(mut self, option: NativeSelectOption<T>) -> Self {
        self.options.push(option);
        self
    }

    /// Appends all options from an iterator.
    pub fn extend<I>(mut self, options: I) -> Self
    where
        I: IntoIterator<Item = NativeSelectOption<T>>,
    {
        self.options.extend(options);
        self
    }

    /// Returns the group heading.
    #[must_use]
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Returns the options in layout order.
    #[must_use = "inspect or iterate the group's options"]
    pub fn options(&self) -> &[NativeSelectOption<T>] {
        &self.options
    }

    /// Returns whether the whole group is disabled.
    #[must_use]
    pub const fn is_disabled(&self) -> bool {
        self.disabled
    }

    /// Sets the group's disabled state.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

/// One entry accepted by [`super::NativeSelect::with_items`].
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum NativeSelectItem<T> {
    /// A single option.
    Option(NativeSelectOption<T>),
    /// A labelled option group.
    OptGroup(NativeSelectOptGroup<T>),
}

impl<T> NativeSelectItem<T> {
    /// Wraps one option as a select item.
    pub fn option(option: NativeSelectOption<T>) -> Self {
        Self::Option(option)
    }

    /// Wraps one option group as a select item.
    pub fn opt_group(group: NativeSelectOptGroup<T>) -> Self {
        Self::OptGroup(group)
    }
}

impl<T> From<NativeSelectOption<T>> for NativeSelectItem<T> {
    fn from(option: NativeSelectOption<T>) -> Self {
        Self::Option(option)
    }
}

impl<T> From<NativeSelectOptGroup<T>> for NativeSelectItem<T> {
    fn from(group: NativeSelectOptGroup<T>) -> Self {
        Self::OptGroup(group)
    }
}

impl<T> fmt::Display for NativeSelectOption<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.label)
    }
}
