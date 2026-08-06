//! Configuration types for the input-group component.

use crate::components::button::{ButtonRadius, ButtonSize, ButtonVariant};
use crate::components::input::{InputRadius, InputSize};

use iced_core::text::Wrapping;

#[cfg(feature = "serde")]
mod wrapping_serde {
    use iced_core::text::Wrapping;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(value: &Wrapping, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(match value {
            Wrapping::None => "none",
            Wrapping::Word => "word",
            Wrapping::Glyph => "glyph",
            Wrapping::WordOrGlyph => "word-or-glyph",
        })
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Wrapping, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;

        match value.as_str() {
            "none" => Ok(Wrapping::None),
            "word" => Ok(Wrapping::Word),
            "glyph" => Ok(Wrapping::Glyph),
            "word-or-glyph" => Ok(Wrapping::WordOrGlyph),
            _ => Err(serde::de::Error::unknown_variant(
                &value,
                &["none", "word", "glyph", "word-or-glyph"],
            )),
        }
    }
}

/// Alignment slot for an [`super::InputGroupAddon`].
///
/// The four values mirror shadcn-svelte's `inline-start`, `inline-end`,
/// `block-start`, and `block-end` addon variants. Inline addons share a row
/// with the control; block addons create a full-width row above or below it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InputGroupAddonAlign {
    /// Place the addon before the control in the inline direction.
    #[default]
    InlineStart,
    /// Place the addon after the control in the inline direction.
    InlineEnd,
    /// Place the addon above the control in the block direction.
    BlockStart,
    /// Place the addon below the control in the block direction.
    BlockEnd,
}

impl InputGroupAddonAlign {
    /// Whether this slot is laid out on the block axis.
    pub const fn is_block(self) -> bool {
        matches!(self, Self::BlockStart | Self::BlockEnd)
    }
}

/// Corner-radius preset for an [`super::InputGroup`].
///
/// This is deliberately distinct from [`InputRadius`]: the value styles the
/// outer group surface, not one of the controls inside it.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InputGroupRadius {
    /// No corner radius.
    None,
    /// Small corner radius.
    Small,
    /// Medium corner radius.
    #[default]
    Medium,
    /// Large corner radius.
    Large,
    /// Fully rounded corners.
    Full,
}

impl From<InputRadius> for InputGroupRadius {
    fn from(radius: InputRadius) -> Self {
        match radius {
            InputRadius::None => Self::None,
            InputRadius::Small => Self::Small,
            InputRadius::Medium => Self::Medium,
            InputRadius::Large => Self::Large,
            InputRadius::Full => Self::Full,
        }
    }
}

impl From<InputGroupRadius> for InputRadius {
    fn from(radius: InputGroupRadius) -> Self {
        match radius {
            InputGroupRadius::None => Self::None,
            InputGroupRadius::Small => Self::Small,
            InputGroupRadius::Medium => Self::Medium,
            InputGroupRadius::Large => Self::Large,
            InputGroupRadius::Full => Self::Full,
        }
    }
}

/// Compact button sizes supported by [`super::InputGroupButton`].
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InputGroupButtonSize {
    /// A compact text button (`h-6`).
    #[default]
    Xs,
    /// A small text button (`h-8`).
    Sm,
    /// A compact square icon button (`size-6`).
    IconXs,
    /// A small square icon button (`size-8`).
    IconSm,
}

impl InputGroupButtonSize {
    /// Whether this size is square and intended for an icon.
    pub const fn is_icon(self) -> bool {
        matches!(self, Self::IconXs | Self::IconSm)
    }

    pub(crate) const fn button_size(self) -> ButtonSize {
        match self {
            Self::Xs => ButtonSize::Xs,
            Self::Sm => ButtonSize::Sm,
            Self::IconXs => ButtonSize::IconXs,
            Self::IconSm => ButtonSize::IconSm,
        }
    }
}

/// Root options compatible with shadcn-svelte's input-group state.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InputGroupProps {
    pub(super) radius: Option<InputGroupRadius>,
    pub(super) invalid: bool,
    pub(super) disabled: bool,
}

impl InputGroupProps {
    /// Creates the default enabled, valid group options.
    pub const fn new() -> Self {
        Self {
            radius: None,
            invalid: false,
            disabled: false,
        }
    }

    /// Sets the outer group radius.
    pub fn radius(mut self, radius: impl Into<InputGroupRadius>) -> Self {
        self.radius = Some(radius.into());
        self
    }

    /// Marks the group as invalid.
    pub fn invalid(mut self, invalid: bool) -> Self {
        self.invalid = invalid;
        self
    }

    /// Disables the group surface and addon affordances.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

/// Options for an input-group addon.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InputGroupAddonProps {
    pub(super) align: InputGroupAddonAlign,
}

impl InputGroupAddonProps {
    /// Creates an inline-start addon configuration.
    pub const fn new() -> Self {
        Self {
            align: InputGroupAddonAlign::InlineStart,
        }
    }

    /// Sets the addon alignment slot.
    pub fn align(mut self, align: InputGroupAddonAlign) -> Self {
        self.align = align;
        self
    }
}

/// Options for the compact button nested in an input-group addon.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InputGroupButtonProps {
    pub(super) variant: ButtonVariant,
    pub(super) size: InputGroupButtonSize,
    pub(super) radius: Option<ButtonRadius>,
    pub(super) disabled: bool,
}

impl Default for InputGroupButtonProps {
    fn default() -> Self {
        Self {
            variant: ButtonVariant::Ghost,
            size: InputGroupButtonSize::default(),
            radius: None,
            disabled: false,
        }
    }
}

impl InputGroupButtonProps {
    /// Creates the default ghost `xs` button options.
    pub const fn new() -> Self {
        Self {
            variant: ButtonVariant::Ghost,
            size: InputGroupButtonSize::Xs,
            radius: None,
            disabled: false,
        }
    }

    /// Sets the underlying shadcn button variant.
    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Sets the compact input-group button size.
    pub fn size(mut self, size: InputGroupButtonSize) -> Self {
        self.size = size;
        self
    }

    /// Sets an explicit button radius.
    pub fn radius(mut self, radius: ButtonRadius) -> Self {
        self.radius = Some(radius);
        self
    }

    /// Disables the button.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

/// Options for the controlled input inside an input group.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InputGroupInputProps {
    pub(super) size: InputSize,
    pub(super) disabled: bool,
    pub(super) read_only: bool,
    pub(super) invalid: bool,
}

impl Default for InputGroupInputProps {
    fn default() -> Self {
        Self {
            size: InputSize::Default,
            disabled: false,
            read_only: false,
            invalid: false,
        }
    }
}

impl InputGroupInputProps {
    /// Creates default input options.
    pub const fn new() -> Self {
        Self {
            size: InputSize::Default,
            disabled: false,
            read_only: false,
            invalid: false,
        }
    }

    /// Sets the input size.
    pub fn size(mut self, size: InputSize) -> Self {
        self.size = size;
        self
    }

    /// Disables editing while retaining the value.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Makes the input read-only.
    pub fn read_only(mut self, read_only: bool) -> Self {
        self.read_only = read_only;
        self
    }

    /// Marks the input as invalid.
    pub fn invalid(mut self, invalid: bool) -> Self {
        self.invalid = invalid;
        self
    }
}

/// Resize policy for an [`super::InputGroupTextarea`].
///
/// Iced's `text_editor` widget has no browser-style pointer resize handle.
/// [`Self::None`] keeps the control at its minimum height; the other values
/// leave the height unconstrained while preserving the source component's
/// intent for applications that provide their own layout policy.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InputGroupTextareaResize {
    /// Keep the textarea at its minimum height.
    #[default]
    None,
    /// Allow vertical resizing.
    Vertical,
    /// Allow horizontal resizing.
    Horizontal,
    /// Allow resizing in both directions.
    Both,
}

/// Options for the multi-line control inside an input group.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InputGroupTextareaProps {
    pub(super) size: InputSize,
    pub(super) disabled: bool,
    pub(super) read_only: bool,
    pub(super) invalid: bool,
    pub(super) padding: Option<[f32; 2]>,
    pub(super) rows: Option<usize>,
    pub(super) max_rows: Option<usize>,
    pub(super) resize: InputGroupTextareaResize,
    #[cfg_attr(feature = "serde", serde(with = "wrapping_serde"))]
    pub(super) wrapping: Wrapping,
    pub(super) max_len: Option<usize>,
}

impl Default for InputGroupTextareaProps {
    fn default() -> Self {
        Self {
            size: InputSize::Default,
            disabled: false,
            read_only: false,
            invalid: false,
            padding: None,
            rows: None,
            max_rows: None,
            resize: InputGroupTextareaResize::None,
            wrapping: Wrapping::WordOrGlyph,
            max_len: None,
        }
    }
}

impl InputGroupTextareaProps {
    /// Creates default textarea options.
    pub const fn new() -> Self {
        Self {
            size: InputSize::Default,
            disabled: false,
            read_only: false,
            invalid: false,
            padding: None,
            rows: None,
            max_rows: None,
            resize: InputGroupTextareaResize::None,
            wrapping: Wrapping::WordOrGlyph,
            max_len: None,
        }
    }

    /// Sets the textarea size ladder.
    pub fn size(mut self, size: InputSize) -> Self {
        self.size = size;
        self
    }

    /// Disables editing.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Makes the textarea read-only.
    pub fn read_only(mut self, read_only: bool) -> Self {
        self.read_only = read_only;
        self
    }

    /// Marks the textarea as invalid.
    pub fn invalid(mut self, invalid: bool) -> Self {
        self.invalid = invalid;
        self
    }

    /// Sets vertical and horizontal padding in pixels.
    pub fn padding(mut self, padding: [f32; 2]) -> Self {
        self.padding = Some([padding[0].max(0.0), padding[1].max(0.0)]);
        self
    }

    /// Sets the minimum number of rows, clamped to one.
    pub fn rows(mut self, rows: usize) -> Self {
        self.rows = Some(rows.max(1));
        self
    }

    /// Sets the maximum number of rows, clamped to one.
    pub fn max_rows(mut self, rows: usize) -> Self {
        self.max_rows = Some(rows.max(1));
        self
    }

    /// Sets the resize policy.
    pub fn resize(mut self, resize: InputGroupTextareaResize) -> Self {
        self.resize = resize;
        self
    }

    /// Sets the text wrapping strategy used by iced.
    pub fn wrapping(mut self, wrapping: Wrapping) -> Self {
        self.wrapping = wrapping;
        self
    }

    /// Sets a maximum character count for [`super::input_group_textarea_apply_action`].
    pub fn max_len(mut self, max_len: usize) -> Self {
        self.max_len = Some(max_len);
        self
    }
}
