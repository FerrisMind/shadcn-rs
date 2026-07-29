//! Builder-first form-field composition for `iced-shadcn-v2`.
//!
//! The family mirrors shadcn-svelte's `Field` exports: [`Field`],
//! [`FieldSet`], [`FieldLegend`], [`FieldGroup`], [`FieldContent`],
//! [`FieldLabel`], [`FieldTitle`], [`FieldDescription`], [`FieldSeparator`],
//! and [`FieldError`]. Every part accepts arbitrary [`iced_core::Element`]
//! content,
//! so controls such as [`crate::Input`], [`crate::Checkbox`], radio groups,
//! and application-owned widgets can be composed without a second API.
//!
//! ```rust,no_run
//! use iced::Element;
//! use iced_shadcn_v2::{Field, FieldDescription, FieldLabel, Input, Theme};
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     EmailChanged(String),
//! }
//!
//! fn email_field<'a>(theme: &'a Theme, value: &'a str) -> Element<'a, Message> {
//!     Field::new(theme)
//!         .push(FieldLabel::text("Email", theme))
//!         .push(
//!             Input::new(theme)
//!                 .value(value)
//!                 .placeholder("you@example.com")
//!                 .on_input(Message::EmailChanged),
//!         )
//!         .push(FieldDescription::text(
//!             "We will never share your address.",
//!             theme,
//!         ))
//!         .into()
//! }
//! ```

mod geometry;
mod render;
mod types;

#[cfg(test)]
mod tests;

pub use types::{FieldErrorItem, FieldLegendVariant, FieldOrientation};

use std::borrow::Cow;
use std::fmt;

use crate::iced_compat::widget::container;
use crate::iced_compat::widget::text::{Fragment, IntoFragment};
use crate::iced_compat::{Background, Border, Color, Element, Length, Padding};

use crate::theme::Theme;

use super::label::{Label, LabelContext};

pub(crate) enum FieldTextContent<'a, Message> {
    Text(Fragment<'a>),
    Element(Element<'a, Message>),
}

impl<'a, Message> FieldTextContent<'a, Message> {
    fn kind(&self) -> &'static str {
        match self {
            Self::Text(_) => "text",
            Self::Element(_) => "element",
        }
    }
}

/// Breakpoint used by [`FieldOrientation::Responsive`] when no custom value
/// is supplied.
pub const DEFAULT_FIELD_RESPONSIVE_BREAKPOINT: f32 = geometry::DEFAULT_RESPONSIVE_BREAKPOINT;

/// A single form field composed from a label, control, descriptions, and
/// validation messages.
///
/// The builder deliberately does not prescribe a control type. Add any iced
/// element with [`Self::push`], or use [`Self::with_children`] for an existing
/// collection. `orientation=responsive` uses the available layout width, not
/// a hard-coded viewport guess.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct Field<'a, Message> {
    theme: &'a Theme,
    children: Vec<Element<'a, Message>>,
    orientation: FieldOrientation,
    width: Length,
    spacing: Option<f32>,
    responsive_breakpoint: f32,
    invalid: bool,
    disabled: bool,
}

impl<Message> fmt::Debug for Field<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Field")
            .field("theme", &self.theme)
            .field("children", &self.children.len())
            .field("orientation", &self.orientation)
            .field("width", &self.width)
            .field("spacing", &self.spacing)
            .field("responsive_breakpoint", &self.responsive_breakpoint)
            .field("invalid", &self.invalid)
            .field("disabled", &self.disabled)
            .finish()
    }
}

impl<'a, Message> Field<'a, Message> {
    /// Creates an empty vertical field.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            children: Vec::new(),
            orientation: FieldOrientation::default(),
            width: Length::Fill,
            spacing: None,
            responsive_breakpoint: geometry::DEFAULT_RESPONSIVE_BREAKPOINT,
            invalid: false,
            disabled: false,
        }
    }

    /// Creates a field and appends all supplied children.
    pub fn with_children<I, E>(theme: &'a Theme, children: I) -> Self
    where
        I: IntoIterator<Item = E>,
        E: Into<Element<'a, Message>>,
    {
        let mut field = Self::new(theme);
        field.children.extend(children.into_iter().map(Into::into));
        field
    }

    /// Appends one child to the field.
    pub fn push(mut self, child: impl Into<Element<'a, Message>>) -> Self {
        self.children.push(child.into());
        self
    }

    /// Appends a collection of children to the field.
    pub fn extend<I>(mut self, children: I) -> Self
    where
        I: IntoIterator<Item = Element<'a, Message>>,
    {
        self.children.extend(children);
        self
    }

    /// Sets the field orientation.
    pub fn orientation(mut self, orientation: FieldOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Sets the gap between the field's direct children in pixels.
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = Some(geometry::normalize_px(spacing));
        self
    }

    /// Sets the field width.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the width at which a responsive field switches to horizontal.
    pub fn responsive_breakpoint(mut self, breakpoint: f32) -> Self {
        self.responsive_breakpoint = geometry::normalize_px(breakpoint);
        self
    }

    /// Marks the field invalid, matching `data-invalid="true"`.
    pub fn invalid(mut self, invalid: bool) -> Self {
        self.invalid = invalid;
        self
    }

    /// Marks the field disabled, matching `data-disabled="true"`.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Builds the field as an iced element.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        render::build_field(
            self.theme,
            self.children,
            render::FieldOptions {
                orientation: self.orientation,
                width: self.width,
                spacing: self.spacing,
                responsive_breakpoint: self.responsive_breakpoint,
                invalid: self.invalid,
                disabled: self.disabled,
            },
        )
    }
}

impl<'a, Message> From<Field<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(field: Field<'a, Message>) -> Self {
        field.into_element()
    }
}

/// A vertical collection of related fields.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct FieldGroup<'a, Message> {
    children: Vec<Element<'a, Message>>,
    width: Length,
    spacing: Option<f32>,
    checkbox_group: bool,
}

impl<Message> fmt::Debug for FieldGroup<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("FieldGroup")
            .field("children", &self.children.len())
            .field("width", &self.width)
            .field("spacing", &self.spacing)
            .field("checkbox_group", &self.checkbox_group)
            .finish()
    }
}

impl<'a, Message> FieldGroup<'a, Message> {
    /// Creates an empty field group.
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            width: Length::Fill,
            spacing: None,
            checkbox_group: false,
        }
    }

    /// Creates a field group and appends all supplied children.
    pub fn with_children<I, E>(children: I) -> Self
    where
        I: IntoIterator<Item = E>,
        E: Into<Element<'a, Message>>,
    {
        let mut group = Self::new();
        group.children.extend(children.into_iter().map(Into::into));
        group
    }

    /// Appends one child to the group.
    pub fn push(mut self, child: impl Into<Element<'a, Message>>) -> Self {
        self.children.push(child.into());
        self
    }

    /// Appends a collection of children to the group.
    pub fn extend<I>(mut self, children: I) -> Self
    where
        I: IntoIterator<Item = Element<'a, Message>>,
    {
        self.children.extend(children);
        self
    }

    /// Sets the gap between group children in pixels.
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = Some(geometry::normalize_px(spacing));
        self
    }

    /// Uses the tighter spacing intended for checkbox/radio groups.
    pub fn checkbox_group(mut self, checkbox_group: bool) -> Self {
        self.checkbox_group = checkbox_group;
        self
    }

    /// Sets the group width.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Builds the group as an iced element.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        render::build_group(self.children, self.width, self.spacing, self.checkbox_group)
    }
}

impl<'a, Message> Default for FieldGroup<'a, Message> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, Message> From<FieldGroup<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(group: FieldGroup<'a, Message>) -> Self {
        group.into_element()
    }
}

/// A semantic fieldset-like container for related controls.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct FieldSet<'a, Message> {
    children: Vec<Element<'a, Message>>,
    width: Length,
    spacing: Option<f32>,
}

impl<Message> fmt::Debug for FieldSet<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("FieldSet")
            .field("children", &self.children.len())
            .field("width", &self.width)
            .field("spacing", &self.spacing)
            .finish()
    }
}

impl<'a, Message> FieldSet<'a, Message> {
    /// Creates an empty fieldset.
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            width: Length::Fill,
            spacing: None,
        }
    }

    /// Creates a fieldset and appends all supplied children.
    pub fn with_children<I, E>(children: I) -> Self
    where
        I: IntoIterator<Item = E>,
        E: Into<Element<'a, Message>>,
    {
        let mut set = Self::new();
        set.children.extend(children.into_iter().map(Into::into));
        set
    }

    /// Appends one child to the fieldset.
    pub fn push(mut self, child: impl Into<Element<'a, Message>>) -> Self {
        self.children.push(child.into());
        self
    }

    /// Appends a collection of children to the fieldset.
    pub fn extend<I>(mut self, children: I) -> Self
    where
        I: IntoIterator<Item = Element<'a, Message>>,
    {
        self.children.extend(children);
        self
    }

    /// Sets the gap between fieldset children in pixels.
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = Some(geometry::normalize_px(spacing));
        self
    }

    /// Sets the fieldset width.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Builds the fieldset as an iced element.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        render::build_set(self.children, self.width, self.spacing)
    }
}

impl<'a, Message> Default for FieldSet<'a, Message> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, Message> From<FieldSet<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(set: FieldSet<'a, Message>) -> Self {
        set.into_element()
    }
}

/// A flex column for a field label and its supporting text.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct FieldContent<'a, Message> {
    children: Vec<Element<'a, Message>>,
    width: Length,
    spacing: Option<f32>,
}

impl<Message> fmt::Debug for FieldContent<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("FieldContent")
            .field("children", &self.children.len())
            .field("width", &self.width)
            .field("spacing", &self.spacing)
            .finish()
    }
}

impl<'a, Message> FieldContent<'a, Message> {
    /// Creates an empty field content column.
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            width: Length::Fill,
            spacing: None,
        }
    }

    /// Creates field content and appends all supplied children.
    pub fn with_children<I, E>(children: I) -> Self
    where
        I: IntoIterator<Item = E>,
        E: Into<Element<'a, Message>>,
    {
        let mut content = Self::new();
        content
            .children
            .extend(children.into_iter().map(Into::into));
        content
    }

    /// Appends one child to the content column.
    pub fn push(mut self, child: impl Into<Element<'a, Message>>) -> Self {
        self.children.push(child.into());
        self
    }

    /// Appends a collection of children to the content column.
    pub fn extend<I>(mut self, children: I) -> Self
    where
        I: IntoIterator<Item = Element<'a, Message>>,
    {
        self.children.extend(children);
        self
    }

    /// Sets the gap between content children in pixels.
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = Some(geometry::normalize_px(spacing));
        self
    }

    /// Sets the content column width.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Builds the content column as an iced element.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        render::build_content(self.children, self.width, self.spacing)
    }
}

impl<'a, Message> Default for FieldContent<'a, Message> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, Message> From<FieldContent<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(content: FieldContent<'a, Message>) -> Self {
        content.into_element()
    }
}

/// A legend for a [`FieldSet`].
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct FieldLegend<'a, Message> {
    content: FieldTextContent<'a, Message>,
    theme: &'a Theme,
    variant: FieldLegendVariant,
    width: Length,
    color: Option<Color>,
}

impl<Message> fmt::Debug for FieldLegend<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("FieldLegend")
            .field("content", &self.content.kind())
            .field("theme", &self.theme)
            .field("variant", &self.variant)
            .field("width", &self.width)
            .field("color", &self.color)
            .finish()
    }
}

impl<'a, Message> FieldLegend<'a, Message> {
    /// Creates a legend from arbitrary iced content.
    pub fn new(content: impl Into<Element<'a, Message>>, theme: &'a Theme) -> Self {
        Self {
            content: FieldTextContent::Element(content.into()),
            theme,
            variant: FieldLegendVariant::default(),
            width: Length::Fill,
            color: None,
        }
    }

    /// Creates a text legend.
    pub fn text(content: impl IntoFragment<'a>, theme: &'a Theme) -> Self {
        Self {
            content: FieldTextContent::Text(content.into_fragment()),
            theme,
            variant: FieldLegendVariant::default(),
            width: Length::Fill,
            color: None,
        }
    }

    /// Sets the legend variant.
    pub fn variant(mut self, variant: FieldLegendVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Sets the legend width.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Overrides the legend color.
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Builds the legend as an iced element.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        render::build_legend(
            self.content,
            self.theme,
            self.variant,
            self.width,
            self.color,
        )
    }
}

impl<'a, Message> From<FieldLegend<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(legend: FieldLegend<'a, Message>) -> Self {
        legend.into_element()
    }
}

/// A label styled for use in a field or beside a control.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct FieldLabel<'a, Message> {
    label: Label<'a, Message>,
    theme: &'a Theme,
    choice_card: bool,
    selected: bool,
    padding: Padding,
}

impl<Message> fmt::Debug for FieldLabel<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("FieldLabel")
            .field("label", &self.label)
            .field("theme", &self.theme)
            .field("choice_card", &self.choice_card)
            .field("selected", &self.selected)
            .field("padding", &self.padding)
            .finish()
    }
}

impl<'a, Message> FieldLabel<'a, Message> {
    /// Creates a field label from arbitrary iced content.
    pub fn new(content: impl Into<Element<'a, Message>>, theme: &'a Theme) -> Self {
        Self {
            label: Label::new(content, theme)
                .context(LabelContext::Field)
                .width(Length::Fill),
            theme,
            choice_card: false,
            selected: false,
            padding: Padding::from(16.0),
        }
    }

    /// Creates a text field label.
    pub fn text(content: impl IntoFragment<'a>, theme: &'a Theme) -> Self {
        Self {
            label: Label::text(content, theme)
                .context(LabelContext::Field)
                .width(Length::Fill),
            theme,
            choice_card: false,
            selected: false,
            padding: Padding::from(16.0),
        }
    }

    /// Sets the label context used by the active style pack.
    pub fn context(mut self, context: LabelContext) -> Self {
        self.label = self.label.context(context);
        self
    }

    /// Overrides the label color.
    pub fn color(mut self, color: Color) -> Self {
        self.label = self.label.color(color);
        self
    }

    /// Sets the label width.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.label = self.label.width(width);
        self
    }

    /// Applies the disabled label treatment.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.label = self.label.disabled(disabled);
        self
    }

    /// Associates the label with a control identifier.
    pub fn for_id(mut self, id: impl Into<Cow<'a, str>>) -> Self {
        self.label = self.label.for_id(id);
        self
    }

    /// Sets the message emitted when the label is pressed.
    pub fn on_press(mut self, message: Message) -> Self {
        self.label = self.label.on_press(message);
        self
    }

    /// Sets or clears the message emitted when the label is pressed.
    pub fn on_press_maybe(mut self, message: Option<Message>) -> Self {
        self.label = self.label.on_press_maybe(message);
        self
    }

    /// Enables the bordered choice-card treatment used for nested fields.
    pub fn choice_card(mut self, choice_card: bool) -> Self {
        self.choice_card = choice_card;
        self
    }

    /// Marks a choice card as selected.
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    /// Sets the choice-card padding.
    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = padding.into();
        self
    }

    /// Builds the field label as an iced element.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        let label: Element<'a, Message> = self.label.into_element();
        if !self.choice_card {
            return label;
        }

        let theme = self.theme;
        let border = theme.palette.border;
        let background = if self.selected {
            Some(Background::Color(with_alpha(theme.palette.primary, 0.05)))
        } else {
            None
        };
        container(label)
            .padding(self.padding)
            .width(Length::Fill)
            .style(move |_| container::Style {
                background,
                border: Border {
                    color: border,
                    width: 1.0,
                    radius: crate::components::field::geometry::choice_card_radius(theme).into(),
                },
                ..container::Style::default()
            })
            .into()
    }
}

impl<'a, Message> From<FieldLabel<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(label: FieldLabel<'a, Message>) -> Self {
        label.into_element()
    }
}

/// A compact title used inside [`FieldContent`].
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct FieldTitle<'a, Message> {
    content: FieldTextContent<'a, Message>,
    theme: &'a Theme,
    width: Length,
    disabled: bool,
    color: Option<Color>,
}

impl<Message> fmt::Debug for FieldTitle<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("FieldTitle")
            .field("content", &self.content.kind())
            .field("theme", &self.theme)
            .field("width", &self.width)
            .field("disabled", &self.disabled)
            .field("color", &self.color)
            .finish()
    }
}

impl<'a, Message> FieldTitle<'a, Message> {
    /// Creates a title from arbitrary iced content.
    pub fn new(content: impl Into<Element<'a, Message>>, theme: &'a Theme) -> Self {
        Self {
            content: FieldTextContent::Element(content.into()),
            theme,
            width: Length::Fill,
            disabled: false,
            color: None,
        }
    }

    /// Creates a text title.
    pub fn text(content: impl IntoFragment<'a>, theme: &'a Theme) -> Self {
        Self {
            content: FieldTextContent::Text(content.into_fragment()),
            theme,
            width: Length::Fill,
            disabled: false,
            color: None,
        }
    }

    /// Sets the title width.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Applies the disabled title treatment.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Overrides the title color.
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Builds the title as an iced element.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        render::build_title(
            self.content,
            self.theme,
            self.width,
            self.disabled,
            self.color,
        )
    }
}

impl<'a, Message> From<FieldTitle<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(title: FieldTitle<'a, Message>) -> Self {
        title.into_element()
    }
}

/// Muted supporting text for a field.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct FieldDescription<'a, Message> {
    content: FieldTextContent<'a, Message>,
    theme: &'a Theme,
    width: Length,
    color: Option<Color>,
}

impl<Message> fmt::Debug for FieldDescription<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("FieldDescription")
            .field("content", &self.content.kind())
            .field("theme", &self.theme)
            .field("width", &self.width)
            .field("color", &self.color)
            .finish()
    }
}

impl<'a, Message> FieldDescription<'a, Message> {
    /// Creates a description from arbitrary iced content.
    pub fn new(content: impl Into<Element<'a, Message>>, theme: &'a Theme) -> Self {
        Self {
            content: FieldTextContent::Element(content.into()),
            theme,
            width: Length::Fill,
            color: None,
        }
    }

    /// Creates a text description.
    pub fn text(content: impl IntoFragment<'a>, theme: &'a Theme) -> Self {
        Self {
            content: FieldTextContent::Text(content.into_fragment()),
            theme,
            width: Length::Fill,
            color: None,
        }
    }

    /// Sets the description width.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Overrides the description color.
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Builds the description as an iced element.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        render::build_description(self.content, self.theme, self.width, self.color)
    }
}

impl<'a, Message> From<FieldDescription<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(description: FieldDescription<'a, Message>) -> Self {
        description.into_element()
    }
}

/// Validation content for a field.
///
/// A custom content element takes precedence over [`Self::errors`]. A single
/// message is rendered plainly; multiple messages become an indented bullet
/// list, matching shadcn-svelte's `Field.Error` behavior.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct FieldError<'a, Message> {
    theme: &'a Theme,
    content: Option<FieldTextContent<'a, Message>>,
    errors: Vec<FieldErrorItem>,
    width: Length,
}

impl<Message> fmt::Debug for FieldError<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("FieldError")
            .field("theme", &self.theme)
            .field(
                "content",
                &self.content.as_ref().map(FieldTextContent::kind),
            )
            .field("errors", &self.errors)
            .field("width", &self.width)
            .finish()
    }
}

impl<'a, Message> FieldError<'a, Message> {
    /// Creates an empty error renderer.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            content: None,
            errors: Vec::new(),
            width: Length::Fill,
        }
    }

    /// Creates an error renderer containing arbitrary content.
    pub fn with_content(content: impl Into<Element<'a, Message>>, theme: &'a Theme) -> Self {
        Self::new(theme).content(content)
    }

    /// Creates an error renderer containing text content.
    pub fn text(content: impl IntoFragment<'a>, theme: &'a Theme) -> Self {
        Self {
            theme,
            content: Some(FieldTextContent::Text(content.into_fragment())),
            errors: Vec::new(),
            width: Length::Fill,
        }
    }

    /// Replaces the custom error content.
    pub fn content(mut self, content: impl Into<Element<'a, Message>>) -> Self {
        self.content = Some(FieldTextContent::Element(content.into()));
        self
    }

    /// Replaces the error list.
    pub fn errors<I, E>(mut self, errors: I) -> Self
    where
        I: IntoIterator<Item = E>,
        E: Into<FieldErrorItem>,
    {
        self.errors = errors.into_iter().map(Into::into).collect();
        self
    }

    /// Appends one error item.
    pub fn push_error(mut self, error: impl Into<FieldErrorItem>) -> Self {
        self.errors.push(error.into());
        self
    }

    /// Sets the error width.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Builds the error content as an iced element.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        render::build_error(self.content, self.errors, self.theme, self.width)
    }
}

impl<'a, Message> From<FieldError<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(error: FieldError<'a, Message>) -> Self {
        error.into_element()
    }
}

/// A horizontal rule separating field sections.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct FieldSeparator<'a, Message> {
    content: Option<FieldTextContent<'a, Message>>,
    theme: &'a Theme,
    width: Length,
}

impl<Message> fmt::Debug for FieldSeparator<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("FieldSeparator")
            .field(
                "content",
                &self.content.as_ref().map(FieldTextContent::kind),
            )
            .field("theme", &self.theme)
            .field("width", &self.width)
            .finish()
    }
}

impl<'a, Message> FieldSeparator<'a, Message> {
    /// Creates an unlabeled separator.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            content: None,
            theme,
            width: Length::Fill,
        }
    }

    /// Creates a separator with arbitrary centered content.
    pub fn with_content(content: impl Into<Element<'a, Message>>, theme: &'a Theme) -> Self {
        Self::new(theme).content(content)
    }

    /// Creates a separator with centered text.
    pub fn text(content: impl IntoFragment<'a>, theme: &'a Theme) -> Self {
        Self {
            content: Some(FieldTextContent::Text(content.into_fragment())),
            theme,
            width: Length::Fill,
        }
    }

    /// Replaces the separator's centered content.
    pub fn content(mut self, content: impl Into<Element<'a, Message>>) -> Self {
        self.content = Some(FieldTextContent::Element(content.into()));
        self
    }

    /// Sets the separator width.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Builds the separator as an iced element.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        render::build_separator(self.content, self.theme, self.width)
    }
}

impl<'a, Message> From<FieldSeparator<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(separator: FieldSeparator<'a, Message>) -> Self {
        separator.into_element()
    }
}

fn with_alpha(color: Color, alpha: f32) -> Color {
    Color {
        a: (color.a * alpha).clamp(0.0, 1.0),
        ..color
    }
}
