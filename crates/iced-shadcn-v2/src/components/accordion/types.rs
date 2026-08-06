//! Public configuration and builder state for the accordion component.

use std::time::Duration;

use crate::iced_compat::widget::text::Fragment;
use crate::iced_compat::widget::{button, container};
use crate::iced_compat::{Element, Length};
use crate::theme::Theme;

use shadcn_common::AccentColor;
use twill_core::prelude::theme::SemanticColor;

use crate::components::button::{ButtonRadius, ButtonSize, ButtonVariant};

/// Selection mode of an [`super::Accordion`].
///
/// `Single` keeps zero or one open item. `Multiple` allows any number of items
/// to be open at the same time.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AccordionType {
    /// At most one item can be open.
    #[default]
    Single,
    /// Any number of items can be open.
    Multiple,
}

impl AccordionType {
    /// Returns `true` when the accordion accepts multiple open items.
    #[must_use]
    pub const fn is_multiple(self) -> bool {
        matches!(self, Self::Multiple)
    }
}

/// Alias for [`AccordionType`] using the shorter mode terminology.
pub type AccordionMode = AccordionType;

/// Axis used for accordion keyboard navigation.
///
/// The visual root remains a vertical stack, matching the shadcn-svelte
/// component. The value controls the directional navigation contract for an
/// application that routes keyboard events to its iced view.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AccordionOrientation {
    /// Navigate items from top to bottom.
    #[default]
    Vertical,
    /// Navigate items from left to right.
    Horizontal,
}

impl AccordionOrientation {
    /// Returns `true` when the accordion uses vertical navigation.
    #[must_use]
    pub const fn is_vertical(self) -> bool {
        matches!(self, Self::Vertical)
    }
}

/// Keyboard navigation wrapping policy of an [`super::Accordion`].
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AccordionLoop {
    /// Move from the last item back to the first item.
    #[default]
    Enabled,
    /// Stop at the first and last item.
    Disabled,
}

/// Controlled open value emitted by an [`super::Accordion`].
///
/// The enum mirrors the discriminated `value` prop from bits-ui while keeping
/// single and multiple values distinct in Rust. Multiple values preserve the
/// first-seen order and never contain duplicates.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AccordionValue {
    /// The open item value, or `None` when every item is closed.
    Single(Option<String>),
    /// The ordered values of all open items.
    Multiple(Vec<String>),
}

/// Alias for [`AccordionValue`] using the selection terminology used by the
/// other controlled groups in this crate.
pub type AccordionSelection = AccordionValue;

impl Default for AccordionValue {
    fn default() -> Self {
        Self::Single(None)
    }
}

impl AccordionValue {
    /// Creates a controlled single-item value.
    #[must_use]
    pub fn single(value: Option<impl Into<String>>) -> Self {
        Self::Single(value.map(Into::into))
    }

    /// Creates a controlled multiple-item value, removing duplicate values.
    #[must_use]
    pub fn multiple(values: impl IntoIterator<Item = impl Into<String>>) -> Self {
        let mut open = Vec::new();

        for value in values {
            let value = value.into();
            if !open.iter().any(|existing| existing == &value) {
                open.push(value);
            }
        }

        Self::Multiple(open)
    }

    /// Returns the selected single value, if this is a single value.
    #[must_use]
    pub fn as_single(&self) -> Option<&str> {
        match self {
            Self::Single(value) => value.as_deref(),
            Self::Multiple(_) => None,
        }
    }

    /// Returns the selected multiple values, if this is a multiple value.
    #[must_use]
    pub fn as_multiple(&self) -> &[String] {
        match self {
            Self::Single(_) => &[],
            Self::Multiple(values) => values,
        }
    }

    /// Returns whether `value` is currently open.
    #[must_use]
    pub fn is_open(&self, value: &str) -> bool {
        match self {
            Self::Single(open) => open.as_deref() == Some(value),
            Self::Multiple(open) => open.iter().any(|item| item == value),
        }
    }

    /// Returns the selection mode represented by this value.
    #[must_use]
    pub const fn value_type(&self) -> AccordionType {
        match self {
            Self::Single(_) => AccordionType::Single,
            Self::Multiple(_) => AccordionType::Multiple,
        }
    }

    pub(super) fn for_type(&self, accordion_type: AccordionType) -> Self {
        match (accordion_type, self) {
            (AccordionType::Single, Self::Single(value)) => Self::Single(value.clone()),
            (AccordionType::Single, Self::Multiple(values)) => {
                Self::Single(values.first().cloned())
            }
            (AccordionType::Multiple, Self::Single(value)) => Self::multiple(value.iter().cloned()),
            (AccordionType::Multiple, Self::Multiple(values)) => {
                Self::multiple(values.iter().cloned())
            }
        }
    }

    pub(super) fn toggled(&self, accordion_type: AccordionType, value: &str) -> Self {
        match self.for_type(accordion_type) {
            Self::Single(open) => {
                if open.as_deref() == Some(value) {
                    Self::Single(None)
                } else {
                    Self::Single(Some(value.to_owned()))
                }
            }
            Self::Multiple(mut open) => {
                if let Some(index) = open.iter().position(|item| item == value) {
                    open.remove(index);
                } else {
                    open.push(value.to_owned());
                }

                Self::Multiple(open)
            }
        }
    }
}

impl From<String> for AccordionValue {
    fn from(value: String) -> Self {
        Self::Single(Some(value))
    }
}

impl From<&str> for AccordionValue {
    fn from(value: &str) -> Self {
        Self::Single(Some(value.to_owned()))
    }
}

impl From<Option<String>> for AccordionValue {
    fn from(value: Option<String>) -> Self {
        Self::Single(value)
    }
}

impl<'a> From<Option<&'a str>> for AccordionValue {
    fn from(value: Option<&'a str>) -> Self {
        Self::Single(value.map(str::to_owned))
    }
}

impl From<Vec<String>> for AccordionValue {
    fn from(value: Vec<String>) -> Self {
        Self::multiple(value)
    }
}

impl<'a> From<Vec<&'a str>> for AccordionValue {
    fn from(value: Vec<&'a str>) -> Self {
        Self::multiple(value)
    }
}

/// Semantic heading level of an accordion item trigger.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AccordionHeaderLevel {
    /// Heading level one.
    One,
    /// Heading level two.
    Two,
    /// Heading level three.
    #[default]
    Three,
    /// Heading level four.
    Four,
    /// Heading level five.
    Five,
    /// Heading level six.
    Six,
}

impl AccordionHeaderLevel {
    /// Returns the numeric heading level used by the source component.
    #[must_use]
    pub const fn number(self) -> u8 {
        match self {
            Self::One => 1,
            Self::Two => 2,
            Self::Three => 3,
            Self::Four => 4,
            Self::Five => 5,
            Self::Six => 6,
        }
    }

    /// Converts a numeric level, clamping values outside `1..=6`.
    #[must_use]
    pub const fn from_number(level: u8) -> Self {
        match level {
            0 | 1 => Self::One,
            2 => Self::Two,
            3 => Self::Three,
            4 => Self::Four,
            5 => Self::Five,
            _ => Self::Six,
        }
    }
}

/// Text or arbitrary widget content used by an accordion trigger.
pub(super) enum AccordionTriggerContent<'a, Message> {
    Label(Fragment<'a>),
    Element(Element<'a, Message>),
    Icon(Element<'a, Message>),
}

impl<Message> AccordionTriggerContent<'_, Message> {
    pub(super) const fn kind(&self) -> &'static str {
        match self {
            Self::Label(_) => "label",
            Self::Element(_) => "element",
            Self::Icon(_) => "icon",
        }
    }
}

/// Trigger builder for one [`super::AccordionItem`].
#[must_use = "a trigger does nothing unless added to an AccordionItem"]
pub struct AccordionTrigger<'a, Message> {
    pub(super) theme: &'a Theme,
    pub(super) content: AccordionTriggerContent<'a, Message>,
    pub(super) variant: ButtonVariant,
    pub(super) size: ButtonSize,
    pub(super) radius: Option<ButtonRadius>,
    pub(super) color: Option<AccentColor>,
    pub(super) width: Length,
    pub(super) height: Length,
    pub(super) full_width: bool,
    pub(super) disabled: bool,
    pub(super) level: AccordionHeaderLevel,
    pub(super) padding: Option<twill_core::prelude::Padding>,
    pub(super) gap: Option<f32>,
    pub(super) on_press: Option<Message>,
    pub(super) style_override:
        Option<Box<dyn Fn(button::Style, button::Status) -> button::Style + 'a>>,
}

/// Content builder for one [`super::AccordionItem`].
#[must_use = "content does nothing unless added to an AccordionItem"]
pub struct AccordionContent<'a, Message> {
    pub(super) theme: &'a Theme,
    pub(super) children: Vec<Element<'a, Message>>,
    pub(super) spacing: f32,
    pub(super) padding: Option<crate::iced_compat::Padding>,
    pub(super) width: Length,
    pub(super) height: Length,
    pub(super) background: Option<SemanticColor>,
    pub(super) bordered: bool,
    pub(super) radius: Option<f32>,
    pub(super) force_mount: bool,
    pub(super) hidden_until_found: bool,
    pub(super) style_override: Option<Box<dyn Fn(container::Style) -> container::Style + 'a>>,
}

/// One value-bearing accordion item containing a trigger and content panel.
#[must_use = "an item does nothing unless added to an Accordion"]
pub struct AccordionItem<'a, Message> {
    pub(super) theme: &'a Theme,
    pub(super) value: Option<String>,
    pub(super) trigger: Option<AccordionTrigger<'a, Message>>,
    pub(super) content: Option<AccordionContent<'a, Message>>,
    pub(super) disabled: bool,
    pub(super) padding: Option<crate::iced_compat::Padding>,
    pub(super) background: Option<SemanticColor>,
    pub(super) bordered: bool,
    pub(super) radius: Option<f32>,
    pub(super) style_override: Option<Box<dyn Fn(container::Style) -> container::Style + 'a>>,
}

/// Builder-first root for a controlled accordion.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct Accordion<'a, Message> {
    pub(super) theme: &'a Theme,
    pub(super) items: Vec<AccordionItem<'a, Message>>,
    pub(super) accordion_type: AccordionType,
    pub(super) value: AccordionValue,
    pub(super) orientation: AccordionOrientation,
    pub(super) loop_navigation: AccordionLoop,
    pub(super) spacing: f32,
    pub(super) disabled: bool,
    pub(super) animated: bool,
    pub(super) duration: Duration,
    pub(super) width: Length,
    pub(super) height: Length,
    pub(super) padding: Option<crate::iced_compat::Padding>,
    pub(super) background: Option<SemanticColor>,
    pub(super) bordered: Option<bool>,
    pub(super) radius: Option<f32>,
    pub(super) on_value_change: Option<std::rc::Rc<dyn Fn(AccordionValue) -> Message + 'a>>,
    pub(super) on_press: Option<Message>,
    pub(super) style_override: Option<Box<dyn Fn(container::Style) -> container::Style + 'a>>,
}
