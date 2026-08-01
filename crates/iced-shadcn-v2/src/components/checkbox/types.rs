//! Configuration types used by the checkbox component.

/// Visual state of a [`super::Checkbox`].
///
/// ```rust
/// use iced_shadcn_v2::CheckboxState;
///
/// assert_eq!(CheckboxState::default(), CheckboxState::Unchecked);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CheckboxState {
    /// Not checked.
    #[default]
    Unchecked,
    /// Checked.
    Checked,
    /// Partially checked (indeterminate).
    Indeterminate,
}

impl CheckboxState {
    /// Cycles the state: Unchecked -> Checked -> Indeterminate -> Unchecked.
    pub fn cycle(self) -> Self {
        match self {
            CheckboxState::Unchecked => CheckboxState::Checked,
            CheckboxState::Checked => CheckboxState::Indeterminate,
            CheckboxState::Indeterminate => CheckboxState::Unchecked,
        }
    }
}

/// Visual variant of a [`super::Checkbox`].
///
/// ```rust
/// use iced_shadcn_v2::CheckboxVariant;
///
/// assert_eq!(CheckboxVariant::default(), CheckboxVariant::Surface);
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CheckboxVariant {
    /// Surface style (elevated, rounded).
    #[default]
    Surface,
    /// Classic style (flat).
    Classic,
    /// Soft style (subtle).
    Soft,
}

/// Preset control size for a [`super::Checkbox`].
///
/// ```rust
/// use iced_shadcn_v2::CheckboxSize;
///
/// assert_eq!(CheckboxSize::default(), CheckboxSize::Lg);
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CheckboxSize {
    /// Extra-small size (16 px), matching shadcn-svelte's `size-4` control.
    Xs,
    /// Small size (20 px).
    Sm,
    /// Medium size (24 px).
    Md,
    /// Large size (28 px, default).
    #[default]
    Lg,
}

impl CheckboxSize {
    /// Returns the size in pixels or rem for geometry.
    pub const fn size_px(self) -> f32 {
        match self {
            CheckboxSize::Xs => 16.0,
            CheckboxSize::Sm => 20.0,
            CheckboxSize::Md => 24.0,
            CheckboxSize::Lg => 28.0,
        }
    }
}

/// Visual configuration for the checkbox.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CheckboxConfig {
    /// Current controlled state.
    pub state: CheckboxState,
    /// Visual variant.
    pub variant: CheckboxVariant,
    /// Indicator size.
    pub size: CheckboxSize,
    /// Optional text shown next to the indicator.
    pub label: Option<String>,
    /// Whether interaction is suppressed.
    pub disabled: bool,
}

impl Default for CheckboxConfig {
    fn default() -> Self {
        Self {
            state: CheckboxState::Unchecked,
            variant: CheckboxVariant::Surface,
            size: CheckboxSize::Lg,
            label: None,
            disabled: false,
        }
    }
}
