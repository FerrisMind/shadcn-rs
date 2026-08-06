//! Public configuration types for the stepper component.

use crate::iced_compat::widget::button;
use crate::iced_compat::widget::container;
use crate::iced_compat::widget::text::Fragment;
use crate::iced_compat::{Element, Font, Length, Padding};
use crate::theme::Theme;
use shadcn_common::AccentColor;

use super::super::button::{ButtonSize, ButtonVariant};

pub(super) enum StepperContent<'a, Message> {
    Label(Fragment<'a>),
    Element(Element<'a, Message>),
}

impl<Message> StepperContent<'_, Message> {
    pub(super) const fn kind(&self) -> &'static str {
        match self {
            Self::Label(_) => "label",
            Self::Element(_) => "element",
        }
    }
}

/// Axis used by the stepper navigation.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StepperOrientation {
    /// Place indicators and their labels in a horizontal rail.
    #[default]
    Horizontal,
    /// Place indicators and their labels in a vertical rail.
    Vertical,
}

impl StepperOrientation {
    /// Returns `true` when the navigation is vertical.
    #[must_use]
    pub const fn is_vertical(self) -> bool {
        matches!(self, Self::Vertical)
    }
}

/// Visual state derived from the controlled active step.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StepperItemState {
    /// The item is before the active step.
    Completed,
    /// The item is the active step.
    Active,
    /// The item is after the active step.
    Inactive,
}

/// The circular visual marker shown by a [`StepperTrigger`].
#[must_use = "a stepper indicator does nothing unless used by a StepperTrigger"]
pub struct StepperIndicator<'a, Message> {
    pub(super) content: StepperContent<'a, Message>,
    pub(super) theme: &'a Theme,
    pub(super) size: Option<f32>,
    pub(super) foreground: Option<crate::iced_compat::Color>,
    pub(super) background: Option<crate::iced_compat::Color>,
    pub(super) ring_color: Option<crate::iced_compat::Color>,
    pub(super) style_override: Option<Box<dyn Fn(container::Style) -> container::Style + 'a>>,
}

impl<Message> std::fmt::Debug for StepperIndicator<'_, Message> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("StepperIndicator")
            .field("content", &self.content.kind())
            .field("theme", &self.theme)
            .field("size", &self.size)
            .field("foreground", &self.foreground)
            .field("background", &self.background)
            .field("ring_color", &self.ring_color)
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

/// A title rendered beside or below a stepper indicator.
#[must_use = "a stepper title does nothing unless used by a StepperTrigger"]
pub struct StepperTitle<'a, Message> {
    pub(super) content: StepperContent<'a, Message>,
    pub(super) theme: &'a Theme,
    pub(super) color: Option<crate::iced_compat::Color>,
    pub(super) text_size: Option<f32>,
    pub(super) line_height: Option<f32>,
    pub(super) font: Option<Font>,
    pub(super) style_override: Option<Box<dyn Fn(container::Style) -> container::Style + 'a>>,
}

impl<Message> std::fmt::Debug for StepperTitle<'_, Message> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("StepperTitle")
            .field("content", &self.content.kind())
            .field("theme", &self.theme)
            .field("color", &self.color)
            .field("text_size", &self.text_size)
            .field("line_height", &self.line_height)
            .field("font", &self.font)
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

/// Supporting text rendered beside or below a stepper indicator.
#[must_use = "a stepper description does nothing unless used by a StepperTrigger"]
pub struct StepperDescription<'a, Message> {
    pub(super) content: StepperContent<'a, Message>,
    pub(super) theme: &'a Theme,
    pub(super) color: Option<crate::iced_compat::Color>,
    pub(super) text_size: Option<f32>,
    pub(super) line_height: Option<f32>,
    pub(super) font: Option<Font>,
    pub(super) style_override: Option<Box<dyn Fn(container::Style) -> container::Style + 'a>>,
}

impl<Message> std::fmt::Debug for StepperDescription<'_, Message> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("StepperDescription")
            .field("content", &self.content.kind())
            .field("theme", &self.theme)
            .field("color", &self.color)
            .field("text_size", &self.text_size)
            .field("line_height", &self.line_height)
            .field("font", &self.font)
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

/// A rail between two stepper items.
#[must_use = "a stepper separator does nothing unless used by a StepperItem"]
pub struct StepperSeparator<'a, Message> {
    pub(super) content: Option<StepperContent<'a, Message>>,
    pub(super) theme: &'a Theme,
    pub(super) offset: f32,
    pub(super) thickness: f32,
    pub(super) color: Option<crate::iced_compat::Color>,
    pub(super) completed_color: Option<crate::iced_compat::Color>,
    pub(super) style_override: Option<Box<dyn Fn(container::Style) -> container::Style + 'a>>,
}

impl<Message> std::fmt::Debug for StepperSeparator<'_, Message> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("StepperSeparator")
            .field("content", &self.content.as_ref().map(StepperContent::kind))
            .field("theme", &self.theme)
            .field("offset", &self.offset)
            .field("thickness", &self.thickness)
            .field("color", &self.color)
            .field("completed_color", &self.completed_color)
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

/// The clickable content for one step.
#[must_use = "a stepper trigger does nothing unless used by a StepperItem"]
pub struct StepperTrigger<'a, Message> {
    pub(super) theme: &'a Theme,
    pub(super) indicator: Option<StepperIndicator<'a, Message>>,
    pub(super) title: Option<StepperTitle<'a, Message>>,
    pub(super) description: Option<StepperDescription<'a, Message>>,
    pub(super) children: Vec<Element<'a, Message>>,
    pub(super) width: Length,
    pub(super) height: Length,
    pub(super) gap: Option<f32>,
    pub(super) disabled: bool,
    pub(super) on_press: Option<Message>,
    pub(super) style_override:
        Option<Box<dyn Fn(button::Style, button::Status) -> button::Style + 'a>>,
}

impl<Message> std::fmt::Debug for StepperTrigger<'_, Message> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("StepperTrigger")
            .field("theme", &self.theme)
            .field("indicator", &self.indicator)
            .field("title", &self.title)
            .field("description", &self.description)
            .field("children", &self.children.len())
            .field("width", &self.width)
            .field("height", &self.height)
            .field("gap", &self.gap)
            .field("disabled", &self.disabled)
            .field("on_press", &self.on_press.is_some())
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

/// One item in a [`Stepper`].
#[must_use = "a stepper item does nothing unless pushed into a Stepper"]
pub struct StepperItem<'a, Message> {
    pub(super) id: Option<String>,
    pub(super) trigger: StepperTrigger<'a, Message>,
    pub(super) separator: Option<StepperSeparator<'a, Message>>,
    pub(super) disabled: bool,
}

impl<Message> std::fmt::Debug for StepperItem<'_, Message> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("StepperItem")
            .field("id", &self.id)
            .field("trigger", &self.trigger)
            .field("separator", &self.separator)
            .field("disabled", &self.disabled)
            .finish()
    }
}

/// Configuration for the navigation rail inside a [`Stepper`].
#[must_use = "a stepper navigation configuration does nothing unless used by a Stepper"]
pub struct StepperNav {
    pub(super) orientation: StepperOrientation,
    pub(super) width: Length,
    pub(super) height: Length,
    pub(super) padding: Padding,
    pub(super) gap: Option<f32>,
    pub(super) style_override: Option<Box<dyn Fn(container::Style) -> container::Style>>,
}

impl std::fmt::Debug for StepperNav {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("StepperNav")
            .field("orientation", &self.orientation)
            .field("width", &self.width)
            .field("height", &self.height)
            .field("padding", &self.padding)
            .field("gap", &self.gap)
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

pub(super) enum StepperButtonContent<'a, Message> {
    Label(Fragment<'a>),
    Element(Element<'a, Message>),
}

impl<Message> StepperButtonContent<'_, Message> {
    pub(super) const fn kind(&self) -> &'static str {
        match self {
            Self::Label(_) => "label",
            Self::Element(_) => "element",
        }
    }
}

/// The automatic previous-step control.
#[must_use = "a stepper previous button does nothing unless used by a Stepper"]
pub struct StepperPrevious<'a, Message> {
    pub(super) theme: &'a Theme,
    pub(super) content: StepperButtonContent<'a, Message>,
    pub(super) variant: ButtonVariant,
    pub(super) size: ButtonSize,
    pub(super) color: Option<AccentColor>,
    pub(super) disabled: bool,
    pub(super) on_press: Option<Message>,
    pub(super) style_override:
        Option<Box<dyn Fn(button::Style, button::Status) -> button::Style + 'a>>,
}

impl<Message> std::fmt::Debug for StepperPrevious<'_, Message> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("StepperPrevious")
            .field("theme", &self.theme)
            .field("content", &self.content.kind())
            .field("variant", &self.variant)
            .field("size", &self.size)
            .field("color", &self.color)
            .field("disabled", &self.disabled)
            .field("on_press", &self.on_press.is_some())
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

/// The automatic next-step control.
#[must_use = "a stepper next button does nothing unless used by a Stepper"]
pub struct StepperNext<'a, Message> {
    pub(super) theme: &'a Theme,
    pub(super) content: StepperButtonContent<'a, Message>,
    pub(super) variant: ButtonVariant,
    pub(super) size: ButtonSize,
    pub(super) color: Option<AccentColor>,
    pub(super) disabled: bool,
    pub(super) on_press: Option<Message>,
    pub(super) style_override:
        Option<Box<dyn Fn(button::Style, button::Status) -> button::Style + 'a>>,
}

impl<Message> std::fmt::Debug for StepperNext<'_, Message> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("StepperNext")
            .field("theme", &self.theme)
            .field("content", &self.content.kind())
            .field("variant", &self.variant)
            .field("size", &self.size)
            .field("color", &self.color)
            .field("disabled", &self.disabled)
            .field("on_press", &self.on_press.is_some())
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

/// Controlled, composable step navigation.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct Stepper<'a, Message> {
    pub(super) theme: &'a Theme,
    pub(super) items: Vec<StepperItem<'a, Message>>,
    pub(super) nav: StepperNav,
    pub(super) step: usize,
    pub(super) previous: Option<StepperPrevious<'a, Message>>,
    pub(super) next: Option<StepperNext<'a, Message>>,
    pub(super) spacing: Option<f32>,
    pub(super) width: Length,
    pub(super) height: Length,
    pub(super) padding: Padding,
    pub(super) disabled: bool,
    pub(super) on_step_change: Option<Box<dyn Fn(usize) -> Message + 'a>>,
    pub(super) style_override: Option<Box<dyn Fn(container::Style) -> container::Style + 'a>>,
}

impl<Message> std::fmt::Debug for Stepper<'_, Message> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("Stepper")
            .field("theme", &self.theme)
            .field("items", &self.items)
            .field("nav", &self.nav)
            .field("step", &self.step)
            .field("previous", &self.previous)
            .field("next", &self.next)
            .field("spacing", &self.spacing)
            .field("width", &self.width)
            .field("height", &self.height)
            .field("padding", &self.padding)
            .field("disabled", &self.disabled)
            .field("on_step_change", &self.on_step_change.is_some())
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

impl<'a, Message> StepperIndicator<'a, Message> {
    pub(super) fn from_content(content: StepperContent<'a, Message>, theme: &'a Theme) -> Self {
        Self {
            content,
            theme,
            size: None,
            foreground: None,
            background: None,
            ring_color: None,
            style_override: None,
        }
    }
}

impl<'a, Message> StepperTitle<'a, Message> {
    pub(super) fn from_content(content: StepperContent<'a, Message>, theme: &'a Theme) -> Self {
        Self {
            content,
            theme,
            color: None,
            text_size: None,
            line_height: None,
            font: None,
            style_override: None,
        }
    }
}

impl<'a, Message> StepperDescription<'a, Message> {
    pub(super) fn from_content(content: StepperContent<'a, Message>, theme: &'a Theme) -> Self {
        Self {
            content,
            theme,
            color: None,
            text_size: None,
            line_height: None,
            font: None,
            style_override: None,
        }
    }
}

impl<'a, Message> StepperSeparator<'a, Message> {
    pub(super) fn empty(theme: &'a Theme) -> Self {
        Self {
            content: None,
            theme,
            offset: 0.0,
            thickness: 4.0,
            color: None,
            completed_color: None,
            style_override: None,
        }
    }
}

impl<'a, Message> StepperTrigger<'a, Message> {
    pub(super) fn empty(theme: &'a Theme) -> Self {
        Self {
            theme,
            indicator: None,
            title: None,
            description: None,
            children: Vec::new(),
            width: Length::Shrink,
            height: Length::Shrink,
            gap: None,
            disabled: false,
            on_press: None,
            style_override: None,
        }
    }
}

impl<'a, Message> StepperNext<'a, Message> {
    pub(super) fn from_content(
        content: StepperButtonContent<'a, Message>,
        theme: &'a Theme,
    ) -> Self {
        Self {
            theme,
            content,
            variant: ButtonVariant::Default,
            size: ButtonSize::Default,
            color: None,
            disabled: false,
            on_press: None,
            style_override: None,
        }
    }
}

impl<'a, Message> StepperPrevious<'a, Message> {
    pub(super) fn from_content(
        content: StepperButtonContent<'a, Message>,
        theme: &'a Theme,
    ) -> Self {
        Self {
            theme,
            content,
            variant: ButtonVariant::Outline,
            size: ButtonSize::Default,
            color: None,
            disabled: false,
            on_press: None,
            style_override: None,
        }
    }
}

impl<'a, Message> StepperButtonContent<'a, Message> {
    pub(super) fn label(label: Fragment<'a>) -> Self {
        Self::Label(label)
    }
}

impl<'a, Message> From<StepperIndicator<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(indicator: StepperIndicator<'a, Message>) -> Self {
        super::render::build_standalone_indicator(indicator)
    }
}

impl<'a, Message> From<StepperTitle<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(title: StepperTitle<'a, Message>) -> Self {
        super::render::build_title(title, StepperOrientation::Horizontal)
    }
}

impl<'a, Message> From<StepperDescription<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(description: StepperDescription<'a, Message>) -> Self {
        super::render::build_description(description, StepperOrientation::Horizontal)
    }
}

impl<'a, Message> From<StepperSeparator<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(separator: StepperSeparator<'a, Message>) -> Self {
        super::render::build_standalone_separator(separator)
    }
}

impl<'a, Message> From<StepperNext<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(next: StepperNext<'a, Message>) -> Self {
        super::render::build_standalone_next(next)
    }
}

impl<'a, Message> From<StepperPrevious<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(previous: StepperPrevious<'a, Message>) -> Self {
        super::render::build_standalone_previous(previous)
    }
}

impl<'a, Message> From<StepperTrigger<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(trigger: StepperTrigger<'a, Message>) -> Self {
        super::render::build_standalone_trigger(trigger)
    }
}
