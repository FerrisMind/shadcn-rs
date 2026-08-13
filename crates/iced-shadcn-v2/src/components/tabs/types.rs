//! Public configuration and builder state for [`super::Tabs`].

use crate::iced_compat::widget::text::Fragment;
use crate::iced_compat::widget::{button, container};
use crate::iced_compat::{Element, Length, Padding};
use crate::theme::Theme;

/// Axis used by a [`super::Tabs`] root and its tab list.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TabsOrientation {
    /// Place the list above the active panel.
    #[default]
    Horizontal,
    /// Place the list beside the active panel.
    Vertical,
}

impl TabsOrientation {
    /// Returns `true` when the list is vertical.
    #[must_use]
    pub const fn is_vertical(self) -> bool {
        matches!(self, Self::Vertical)
    }
}

/// Alias retained for callers that describe the root axis as a direction.
pub type TabsDirection = TabsOrientation;

/// Keyboard activation policy for a tab list.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TabsActivationMode {
    /// Arrow keys move focus and activate the next tab immediately.
    #[default]
    Automatic,
    /// Arrow keys move focus; Enter or Space activates the focused tab.
    Manual,
}

/// Whether arrow-key navigation wraps from one end of a list to the other.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TabsListLoop {
    /// Move from the last enabled trigger to the first, and vice versa.
    Enabled,
    /// Stop at the first and last enabled trigger.
    #[default]
    Disabled,
}

/// Surface treatment of a [`super::TabsList`].
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TabsListVariant {
    /// A muted segmented surface with an active surface trigger.
    #[default]
    Default,
    /// A transparent list with an active underline or side indicator.
    Line,
}

/// Size ladder for tab triggers.
///
/// The web component normally inherits the active style pack's default
/// footprint. The other sizes are useful when tabs are embedded in compact
/// toolbars or larger settings surfaces.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TabsSize {
    /// Compact trigger footprint.
    Sm,
    /// The active style pack's default footprint.
    #[default]
    Default,
    /// Larger trigger footprint.
    Lg,
}

/// Wrapping policy for a horizontal tab list.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TabsWrap {
    /// Keep all triggers on one row and allow the list to overflow its bounds.
    #[default]
    NoWrap,
    /// Wrap triggers onto additional rows when the list has a bounded width.
    Wrap,
    /// Wrap triggers and anchor the rows to the bottom of the list.
    WrapReverse,
}

/// Horizontal alignment of triggers inside a list row.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TabsJustify {
    /// Align triggers to the leading edge.
    #[default]
    Start,
    /// Center triggers in the available row.
    Center,
    /// Align triggers to the trailing edge.
    End,
}

/// Hover treatment for inactive tab triggers.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TabsHover {
    /// Keep inactive trigger text at its resting color.
    None,
    /// Use a restrained hover surface.
    #[default]
    Subtle,
    /// Use the strongest supported hover surface.
    Soft,
}

/// Text or arbitrary widget content used by a tab trigger.
pub(super) enum TabsTriggerContent<'a, Message> {
    Label(Fragment<'a>),
    Element(Element<'a, Message>),
}

impl<Message> TabsTriggerContent<'_, Message> {
    pub(super) const fn kind(&self) -> &'static str {
        match self {
            Self::Label(_) => "label",
            Self::Element(_) => "element",
        }
    }
}

/// One selectable trigger in a [`super::TabsList`].
#[must_use = "a tab trigger does nothing unless pushed into a TabsList"]
pub struct TabsTrigger<'a, Message> {
    pub(super) theme: &'a Theme,
    pub(super) value: String,
    pub(super) content: TabsTriggerContent<'a, Message>,
    pub(super) disabled: bool,
    pub(super) width: Option<Length>,
    pub(super) height: Option<Length>,
    pub(super) padding: Option<Padding>,
    pub(super) style_override:
        Option<Box<dyn Fn(button::Style, button::Status) -> button::Style + 'a>>,
}

impl<Message> std::fmt::Debug for TabsTrigger<'_, Message> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("TabsTrigger")
            .field("theme", &self.theme)
            .field("value", &self.value)
            .field("content", &self.content.kind())
            .field("disabled", &self.disabled)
            .field("width", &self.width)
            .field("height", &self.height)
            .field("padding", &self.padding)
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

/// One content panel associated with a trigger value.
#[must_use = "a tab content panel does nothing unless pushed into Tabs"]
pub struct TabsContent<'a, Message> {
    pub(super) theme: &'a Theme,
    pub(super) value: String,
    pub(super) content: TabsContentValue<'a, Message>,
    pub(super) width: Length,
    pub(super) height: Length,
    pub(super) padding: Padding,
    pub(super) style_override: Option<Box<dyn Fn(container::Style) -> container::Style + 'a>>,
}

pub(super) enum TabsContentValue<'a, Message> {
    Label(Fragment<'a>),
    Element(Element<'a, Message>),
}

impl<Message> TabsContentValue<'_, Message> {
    pub(super) const fn kind(&self) -> &'static str {
        match self {
            Self::Label(_) => "label",
            Self::Element(_) => "element",
        }
    }
}

impl<Message> std::fmt::Debug for TabsContent<'_, Message> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("TabsContent")
            .field("theme", &self.theme)
            .field("value", &self.value)
            .field("content", &self.content.kind())
            .field("width", &self.width)
            .field("height", &self.height)
            .field("padding", &self.padding)
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

/// A list of tab triggers, equivalent to shadcn-svelte `Tabs.List`.
#[must_use = "a tabs list does nothing unless turned into an iced Element"]
pub struct TabsList<'a, Message> {
    pub(super) theme: &'a Theme,
    pub(super) triggers: Vec<TabsTrigger<'a, Message>>,
    pub(super) variant: TabsListVariant,
    pub(super) size: TabsSize,
    pub(super) wrap: TabsWrap,
    pub(super) justify: TabsJustify,
    pub(super) hover: TabsHover,
    pub(super) full_width: bool,
    pub(super) width: Length,
    pub(super) height: Length,
    /// Overrides style-pack trigger gap when set.
    pub(super) gap: Option<f32>,
    /// Overrides style-pack list padding when set.
    pub(super) list_padding: Option<f32>,
    pub(super) style_override: Option<Box<dyn Fn(container::Style) -> container::Style + 'a>>,
}

impl<Message> std::fmt::Debug for TabsList<'_, Message> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("TabsList")
            .field("theme", &self.theme)
            .field("triggers", &self.triggers)
            .field("variant", &self.variant)
            .field("size", &self.size)
            .field("wrap", &self.wrap)
            .field("justify", &self.justify)
            .field("hover", &self.hover)
            .field("full_width", &self.full_width)
            .field("width", &self.width)
            .field("height", &self.height)
            .field("gap", &self.gap)
            .field("list_padding", &self.list_padding)
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

/// Root builder for a controlled set of tab panels.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct Tabs<'a, Message> {
    pub(super) theme: &'a Theme,
    pub(super) list: TabsList<'a, Message>,
    pub(super) contents: Vec<TabsContent<'a, Message>>,
    pub(super) value: String,
    pub(super) orientation: TabsOrientation,
    pub(super) activation_mode: TabsActivationMode,
    pub(super) list_loop: TabsListLoop,
    pub(super) spacing: f32,
    pub(super) width: Length,
    pub(super) height: Length,
    pub(super) padding: Padding,
    pub(super) disabled: bool,
    pub(super) on_value_change: Option<Box<dyn Fn(String) -> Message + 'a>>,
    pub(super) style_override: Option<Box<dyn Fn(container::Style) -> container::Style + 'a>>,
}

impl<Message> std::fmt::Debug for Tabs<'_, Message> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("Tabs")
            .field("theme", &self.theme)
            .field("list", &self.list)
            .field("contents", &self.contents)
            .field("value", &self.value)
            .field("orientation", &self.orientation)
            .field("activation_mode", &self.activation_mode)
            .field("list_loop", &self.list_loop)
            .field("spacing", &self.spacing)
            .field("width", &self.width)
            .field("height", &self.height)
            .field("padding", &self.padding)
            .field("disabled", &self.disabled)
            .field("on_value_change", &self.on_value_change.is_some())
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}
