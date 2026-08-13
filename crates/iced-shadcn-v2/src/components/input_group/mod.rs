//! Builder-first input-group component.
//!
//! `InputGroup` ports shadcn-svelte's composable `Root`, `Addon`, `Button`,
//! `Input`, `Text`, and `Textarea` pieces to iced. Addons are grouped into
//! inline-start, inline-end, block-start, and block-end slots; the root owns
//! the shared border, fill, focus ring, invalid state, and disabled treatment.
//!
//! Iced has no DOM selectors, so arbitrary controls are accepted through
//! [`InputGroup::push_element`]. Controls created with [`Input`] or
//! [`InputGroupTextarea`] automatically make their own surface transparent so
//! the group remains the single visual control.
//!
//! ```rust,no_run
//! use iced::Element;
//! use iced_shadcn_v2::{
//!     ButtonVariant, Input, InputGroup, InputGroupAddon, InputGroupAddonAlign, InputGroupButton,
//!     InputGroupText, Theme,
//! };
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     QueryChanged(String),
//!     Search,
//! }
//!
//! fn search<'a>(theme: &'a Theme, query: &'a str) -> Element<'a, Message> {
//!     InputGroup::new(theme)
//!         .push(
//!             Input::new(theme)
//!                 .value(query)
//!                 .placeholder("Search...")
//!                 .id("search-input")
//!                 .on_input(Message::QueryChanged),
//!         )
//!         .push(
//!             InputGroupAddon::empty(theme)
//!                 .align(InputGroupAddonAlign::InlineEnd)
//!                 .focus_input("search-input")
//!                 .push(InputGroupButton::text("Search", theme)
//!                     .variant(ButtonVariant::Secondary)
//!                     .on_press(Message::Search))
//!                 .push(InputGroupText::text("⌘K", theme)),
//!         )
//!         .into()
//! }
//! ```

mod render;
mod style;
mod types;

#[cfg(test)]
mod tests;

pub use types::{
    InputGroupAddonAlign, InputGroupAddonProps, InputGroupButtonProps, InputGroupButtonSize,
    InputGroupInputProps, InputGroupProps, InputGroupRadius, InputGroupTextareaProps,
    InputGroupTextareaResize,
};

use std::fmt;

use crate::iced_compat::widget::container as container_widget;
use crate::iced_compat::widget::text::{Fragment, IntoFragment};
use crate::iced_compat::widget::text_editor;
use crate::iced_compat::{Element, Length, widget};

use crate::components::button::{Button, ButtonRadius, ButtonVariant};
use crate::components::input::{Input, InputSize};
use crate::theme::Theme;

/// The input-group input slot, backed by the full [`crate::Input`] builder.
pub type InputGroupInput<'a, Message> = Input<'a, Message>;

/// Builder for a composed input group.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct InputGroup<'a, Message> {
    pub(super) theme: &'a Theme,
    pub(super) items: Vec<InputGroupItem<'a, Message>>,
    pub(super) radius: Option<InputGroupRadius>,
    pub(super) invalid: bool,
    pub(super) disabled: bool,
    pub(super) width: Length,
    pub(super) height: Length,
    pub(super) aria_label: Option<String>,
    pub(super) style_override:
        Option<Box<dyn Fn(container_widget::Style) -> container_widget::Style + 'a>>,
}

impl<Message> fmt::Debug for InputGroup<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("InputGroup")
            .field("theme", &self.theme)
            .field("items", &self.items)
            .field("radius", &self.radius)
            .field("invalid", &self.invalid)
            .field("disabled", &self.disabled)
            .field("width", &self.width)
            .field("height", &self.height)
            .field("aria_label", &self.aria_label)
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

impl<'a, Message> InputGroup<'a, Message> {
    /// Creates an empty, full-width input group.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            items: Vec::new(),
            radius: None,
            invalid: false,
            disabled: false,
            width: Length::Fill,
            height: Length::Shrink,
            aria_label: None,
            style_override: None,
        }
    }

    /// Creates a group with the supplied controls and addons.
    pub fn with_children(
        theme: &'a Theme,
        children: impl IntoIterator<Item = InputGroupItem<'a, Message>>,
    ) -> Self {
        Self::new(theme).extend(children)
    }

    /// Creates a group from the compatibility options used by the v1 API.
    pub fn with_props(theme: &'a Theme, props: InputGroupProps) -> Self {
        Self::new(theme).apply_props(props)
    }

    /// Applies compatibility options to the root.
    pub fn apply_props(mut self, props: InputGroupProps) -> Self {
        self.radius = props.radius;
        self.invalid = props.invalid;
        self.disabled = props.disabled;
        self
    }

    /// Appends a control or addon.
    pub fn push(mut self, item: impl Into<InputGroupItem<'a, Message>>) -> Self {
        self.items.push(item.into());
        self
    }

    /// Appends an arbitrary control element.
    pub fn push_element(self, element: impl Into<Element<'a, Message>>) -> Self {
        self.push(InputGroupItem::control(element))
    }

    /// Appends an input control and lets the group own its border and fill.
    pub fn push_input(self, input: Input<'a, Message>) -> Self
    where
        Message: Clone + 'a,
    {
        self.push(input)
    }

    /// Appends an addon to one of the four alignment slots.
    pub fn push_addon(self, addon: InputGroupAddon<'a, Message>) -> Self {
        self.push(addon)
    }

    /// Appends every supplied item.
    pub fn extend(self, children: impl IntoIterator<Item = InputGroupItem<'a, Message>>) -> Self {
        children.into_iter().fold(self, Self::push)
    }

    /// Sets the outer radius. [`InputRadius`](crate::InputRadius) is accepted
    /// as a convenience conversion.
    pub fn radius(mut self, radius: impl Into<InputGroupRadius>) -> Self {
        self.radius = Some(radius.into());
        self
    }

    /// Marks the group invalid. Invalid styling outranks focus styling.
    pub fn invalid(mut self, invalid: bool) -> Self {
        self.invalid = invalid;
        self
    }

    /// Disables the group surface and addon text.
    ///
    /// Child controls should also be built with their own `disabled` option,
    /// matching the browser component's controlled disabled attributes.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Sets a custom group width. The default is [`Length::Fill`].
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets a custom group height. The default is [`Length::Shrink`].
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Carries the web `aria-label` for future iced accessibility support.
    pub fn aria_label(mut self, label: impl Into<String>) -> Self {
        self.aria_label = Some(label.into());
        self
    }

    /// Applies a container-style override after group state resolution.
    pub fn style_override(
        mut self,
        style_override: impl Fn(container_widget::Style) -> container_widget::Style + 'a,
    ) -> Self {
        self.style_override = Some(Box::new(style_override));
        self
    }

    /// Builds the group as an iced element.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        render::build_group(self)
    }
}

impl<'a, Message> From<InputGroup<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(group: InputGroup<'a, Message>) -> Self {
        group.into_element()
    }
}

/// One control or addon in an [`InputGroup`].
#[must_use = "items do nothing unless pushed into an InputGroup"]
pub struct InputGroupItem<'a, Message> {
    pub(super) kind: ItemKind<'a, Message>,
}

pub(super) enum ItemKind<'a, Message> {
    Control {
        element: Element<'a, Message>,
        invalid: bool,
        disabled: bool,
        focus_id: Option<widget::Id>,
    },
    Input(Input<'a, Message>),
    Textarea(InputGroupTextarea<'a, Message>),
    Addon(InputGroupAddon<'a, Message>),
}

impl<'a, Message> InputGroupItem<'a, Message> {
    /// Wraps an arbitrary element as the group control slot.
    pub fn control(element: impl Into<Element<'a, Message>>) -> Self {
        Self::control_with_state(element, false, false, None)
    }

    pub(super) fn control_with_state(
        element: impl Into<Element<'a, Message>>,
        invalid: bool,
        disabled: bool,
        focus_id: Option<widget::Id>,
    ) -> Self {
        Self {
            kind: ItemKind::Control {
                element: element.into(),
                invalid,
                disabled,
                focus_id,
            },
        }
    }

    /// Wraps an addon as a group item.
    pub fn addon(addon: InputGroupAddon<'a, Message>) -> Self {
        Self {
            kind: ItemKind::Addon(addon),
        }
    }

    pub(super) const fn kind_name(&self) -> &'static str {
        match self.kind {
            ItemKind::Control { .. } => "control",
            ItemKind::Input(_) => "input",
            ItemKind::Textarea(_) => "textarea",
            ItemKind::Addon(_) => "addon",
        }
    }
}

impl<Message> fmt::Debug for InputGroupItem<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("InputGroupItem")
            .field(&self.kind_name())
            .finish()
    }
}

impl<'a, Message> From<Element<'a, Message>> for InputGroupItem<'a, Message> {
    fn from(element: Element<'a, Message>) -> Self {
        Self::control(element)
    }
}

impl<'a, Message> From<Input<'a, Message>> for InputGroupItem<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(input: Input<'a, Message>) -> Self {
        Self {
            kind: ItemKind::Input(input),
        }
    }
}

impl<'a, Message> From<InputGroupAddon<'a, Message>> for InputGroupItem<'a, Message> {
    fn from(addon: InputGroupAddon<'a, Message>) -> Self {
        Self::addon(addon)
    }
}

impl<'a, Message> From<InputGroupTextarea<'a, Message>> for InputGroupItem<'a, Message>
where
    Message: 'a,
{
    fn from(textarea: InputGroupTextarea<'a, Message>) -> Self {
        Self {
            kind: ItemKind::Textarea(textarea),
        }
    }
}

/// An addon slot containing one or more elements.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct InputGroupAddon<'a, Message> {
    pub(super) theme: &'a Theme,
    pub(super) align: InputGroupAddonAlign,
    pub(super) children: Vec<Element<'a, Message>>,
    pub(super) width: Length,
    pub(super) padding: Option<crate::iced_compat::Padding>,
    pub(super) spacing: f32,
    pub(super) disabled: bool,
    pub(super) focus_id: Option<widget::Id>,
    pub(super) style_override:
        Option<Box<dyn Fn(container_widget::Style) -> container_widget::Style + 'a>>,
}

impl<Message> fmt::Debug for InputGroupAddon<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("InputGroupAddon")
            .field("theme", &self.theme)
            .field("align", &self.align)
            .field("children", &self.children.len())
            .field("width", &self.width)
            .field("padding", &self.padding)
            .field("spacing", &self.spacing)
            .field("disabled", &self.disabled)
            .field("focus_id", &self.focus_id)
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

impl<'a, Message: 'a> InputGroupAddon<'a, Message> {
    /// Creates an addon containing one element.
    pub fn new(content: impl Into<Element<'a, Message>>, theme: &'a Theme) -> Self {
        Self::empty(theme).push(content)
    }

    /// Creates an empty addon ready for chained [`Self::push`] calls.
    pub fn empty(theme: &'a Theme) -> Self {
        Self {
            theme,
            align: InputGroupAddonAlign::InlineStart,
            children: Vec::new(),
            width: Length::Shrink,
            padding: None,
            spacing: style::addon_spacing(theme),
            disabled: false,
            focus_id: None,
            style_override: None,
        }
    }

    /// Creates an addon from several elements.
    pub fn with_children(
        theme: &'a Theme,
        children: impl IntoIterator<Item = Element<'a, Message>>,
    ) -> Self {
        Self {
            children: children.into_iter().collect(),
            ..Self::empty(theme)
        }
    }

    /// Creates a text addon, equivalent to `Addon` + `Text` in Svelte.
    pub fn text(label: impl IntoFragment<'a>, theme: &'a Theme) -> Self {
        Self::new(InputGroupText::text(label, theme), theme)
    }

    /// Applies compatibility addon options.
    pub fn apply_props(mut self, props: InputGroupAddonProps) -> Self {
        self.align = props.align;
        self
    }

    /// Sets the addon alignment slot.
    pub fn align(mut self, align: InputGroupAddonAlign) -> Self {
        self.align = align;
        self
    }

    /// Appends an element to the addon row.
    pub fn push(mut self, child: impl Into<Element<'a, Message>>) -> Self {
        self.children.push(child.into());
        self
    }

    /// Appends multiple elements to the addon row.
    pub fn extend(self, children: impl IntoIterator<Item = Element<'a, Message>>) -> Self {
        children.into_iter().fold(self, Self::push)
    }

    /// Sets the gap between addon children in pixels.
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing.max(0.0);
        self
    }

    /// Sets an explicit addon width.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets all four addon padding sides.
    pub fn padding(mut self, padding: crate::iced_compat::Padding) -> Self {
        self.padding = Some(padding);
        self
    }

    /// Keeps the pack's horizontal inset and clears vertical pad.
    ///
    /// Use when the child already owns its vertical size (chrome icon buttons
    /// sized to the control height) so default addon `py` does not inflate the
    /// row past the input / group height.
    pub fn inline_padding_only(mut self) -> Self {
        let mut pad = style::addon_padding(self.theme, self.align);
        pad.top = 0.0;
        pad.bottom = 0.0;
        self.padding = Some(pad);
        self
    }

    /// Centers `child_height` in `slot_height` with the **same** inset on the
    /// outer edge and on top/bottom (inner edge toward the input stays 0).
    ///
    /// This keeps chrome adornments optically even: side gap == vertical gap.
    pub fn padding_uniform_around_child(mut self, child_height: f32, slot_height: f32) -> Self {
        let inset = ((slot_height - child_height).max(0.0) / 2.0).max(0.0);
        self.padding = Some(match self.align {
            InputGroupAddonAlign::InlineStart => crate::iced_compat::Padding {
                top: inset,
                right: 0.0,
                bottom: inset,
                left: inset,
            },
            InputGroupAddonAlign::InlineEnd => crate::iced_compat::Padding {
                top: inset,
                right: inset,
                bottom: inset,
                left: 0.0,
            },
            InputGroupAddonAlign::BlockStart => crate::iced_compat::Padding {
                top: inset,
                right: inset,
                bottom: 0.0,
                left: inset,
            },
            InputGroupAddonAlign::BlockEnd => crate::iced_compat::Padding {
                top: 0.0,
                right: inset,
                bottom: inset,
                left: inset,
            },
        });
        self
    }

    /// Centers `child_height` in `slot_height` with equal top/bottom inset; the
    /// outer inline edge uses that inset plus `inline_extra` (inner edge stays 0).
    pub fn padding_uniform_around_child_extra_inline(
        mut self,
        child_height: f32,
        slot_height: f32,
        inline_extra: f32,
    ) -> Self {
        let inset_y = ((slot_height - child_height).max(0.0) / 2.0).max(0.0);
        let inset_x = inset_y + inline_extra.max(0.0);
        self.padding = Some(match self.align {
            InputGroupAddonAlign::InlineStart => crate::iced_compat::Padding {
                top: inset_y,
                right: 0.0,
                bottom: inset_y,
                left: inset_x,
            },
            InputGroupAddonAlign::InlineEnd => crate::iced_compat::Padding {
                top: inset_y,
                right: inset_x,
                bottom: inset_y,
                left: 0.0,
            },
            InputGroupAddonAlign::BlockStart => crate::iced_compat::Padding {
                top: inset_x,
                right: inset_y,
                bottom: 0.0,
                left: inset_y,
            },
            InputGroupAddonAlign::BlockEnd => crate::iced_compat::Padding {
                top: 0.0,
                right: inset_y,
                bottom: inset_x,
                left: inset_y,
            },
        });
        self
    }

    /// Applies disabled addon text treatment independently of the root.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Focuses the supplied input/editor when the addon itself is clicked.
    ///
    /// This is the iced equivalent of shadcn-svelte's addon click behavior.
    /// The target control must be created with the same [`widget::Id`].
    pub fn focus_input(mut self, id: impl Into<widget::Id>) -> Self {
        self.focus_id = Some(id.into());
        self
    }

    /// Applies a container-style override after addon resolution.
    pub fn style_override(
        mut self,
        style_override: impl Fn(container_widget::Style) -> container_widget::Style + 'a,
    ) -> Self {
        self.style_override = Some(Box::new(style_override));
        self
    }

    /// Builds this addon as a standalone element.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        render::build_addon(self, false)
    }
}

impl<'a, Message> From<InputGroupAddon<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(addon: InputGroupAddon<'a, Message>) -> Self {
        addon.into_element()
    }
}

/// Muted supporting text or arbitrary inline content inside an addon.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct InputGroupText<'a, Message> {
    pub(super) content: InputGroupTextContent<'a, Message>,
    pub(super) theme: &'a Theme,
    pub(super) text_size: Option<f32>,
    pub(super) width: Length,
    pub(super) style_override:
        Option<Box<dyn Fn(container_widget::Style) -> container_widget::Style + 'a>>,
}

pub(super) enum InputGroupTextContent<'a, Message> {
    Label(Fragment<'a>),
    Element(Element<'a, Message>),
}

impl<Message> fmt::Debug for InputGroupText<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let content = match self.content {
            InputGroupTextContent::Label(_) => "label",
            InputGroupTextContent::Element(_) => "element",
        };

        formatter
            .debug_struct("InputGroupText")
            .field("content", &content)
            .field("theme", &self.theme)
            .field("text_size", &self.text_size)
            .field("width", &self.width)
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

impl<'a, Message> InputGroupText<'a, Message> {
    /// Creates text from arbitrary content, such as an icon plus a label.
    pub fn new(content: impl Into<Element<'a, Message>>, theme: &'a Theme) -> Self {
        Self {
            content: InputGroupTextContent::Element(content.into()),
            theme,
            text_size: None,
            width: Length::Shrink,
            style_override: None,
        }
    }

    /// Creates a text label.
    pub fn text(label: impl IntoFragment<'a>, theme: &'a Theme) -> Self {
        Self {
            content: InputGroupTextContent::Label(label.into_fragment()),
            theme,
            text_size: None,
            width: Length::Shrink,
            style_override: None,
        }
    }

    /// Sets the label size in pixels.
    pub fn text_size(mut self, text_size: f32) -> Self {
        self.text_size = Some(text_size.max(1.0));
        self
    }

    /// Sets the text cell width.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Applies a container-style override after the muted text style.
    pub fn style_override(
        mut self,
        style_override: impl Fn(container_widget::Style) -> container_widget::Style + 'a,
    ) -> Self {
        self.style_override = Some(Box::new(style_override));
        self
    }

    /// Builds the text cell as an element.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        render::build_text(self)
    }
}

impl<'a, Message> From<InputGroupText<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(text: InputGroupText<'a, Message>) -> Self {
        text.into_element()
    }
}

/// Compact button wrapper used in an input-group addon.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct InputGroupButton<'a, Message> {
    button: Button<'a, Message>,
    size: InputGroupButtonSize,
}

impl<Message> fmt::Debug for InputGroupButton<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("InputGroupButton")
            .field("button", &self.button)
            .field("size", &self.size)
            .finish()
    }
}

impl<'a, Message> InputGroupButton<'a, Message> {
    /// Creates a compact ghost button from arbitrary content.
    pub fn new(content: impl Into<Element<'a, Message>>, theme: &'a Theme) -> Self {
        Self {
            button: Button::new(content, theme)
                .variant(ButtonVariant::Ghost)
                .size(InputGroupButtonSize::Xs.button_size()),
            size: InputGroupButtonSize::Xs,
        }
    }

    /// Creates a compact text button.
    pub fn text(label: impl IntoFragment<'a>, theme: &'a Theme) -> Self {
        Self {
            button: Button::text(label, theme)
                .variant(ButtonVariant::Ghost)
                .size(InputGroupButtonSize::Xs.button_size()),
            size: InputGroupButtonSize::Xs,
        }
    }

    /// Creates a compact icon button.
    pub fn icon(content: impl Into<Element<'a, Message>>, theme: &'a Theme) -> Self {
        Self {
            button: Button::icon(content, theme)
                .variant(ButtonVariant::Ghost)
                .size(InputGroupButtonSize::IconXs.button_size()),
            size: InputGroupButtonSize::IconXs,
        }
    }

    /// Sets the underlying button variant.
    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.button = self.button.variant(variant);
        self
    }

    /// Sets the compact button size.
    pub fn size(mut self, size: InputGroupButtonSize) -> Self {
        self.size = size;
        self.button = self.button.size(size.button_size());
        self
    }

    /// Sets the button radius.
    pub fn radius(mut self, radius: ButtonRadius) -> Self {
        self.button = self.button.radius(radius);
        self
    }

    /// Applies a shadcn accent color.
    pub fn color(mut self, color: crate::AccentColor) -> Self {
        self.button = self.button.color(color);
        self
    }

    /// Disables the button.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.button = self.button.disabled(disabled);
        self
    }

    /// Shows the shared spinner and disables the button.
    pub fn loading(mut self, loading: bool) -> Self {
        self.button = self.button.loading(loading);
        self
    }

    /// Sets the message emitted on press.
    pub fn on_press(mut self, message: Message) -> Self {
        self.button = self.button.on_press(message);
        self
    }

    /// Sets or clears the message emitted on press.
    pub fn on_press_maybe(mut self, message: Option<Message>) -> Self {
        self.button = self.button.on_press_maybe(message);
        self
    }

    /// Sets an explicit button width.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.button = self.button.width(width);
        self
    }

    /// Builds the nested button and removes its optional elevation shadow.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        self.button
            .style_override(|mut resolved, _status| {
                resolved.shadow = Default::default();
                resolved
            })
            .into()
    }
}

impl<'a, Message> From<InputGroupButton<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(button: InputGroupButton<'a, Message>) -> Self {
        button.into_element()
    }
}

/// Builder for the multi-line input-group control.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct InputGroupTextarea<'a, Message> {
    pub(super) theme: &'a Theme,
    pub(super) content: &'a text_editor::Content,
    pub(super) placeholder: Fragment<'a>,
    pub(super) props: InputGroupTextareaProps,
    pub(super) id: Option<widget::Id>,
    pub(super) on_action: Option<Box<dyn Fn(text_editor::Action) -> Message + 'a>>,
    pub(super) style_override:
        Option<Box<dyn Fn(text_editor::Style, text_editor::Status) -> text_editor::Style + 'a>>,
}

impl<Message> fmt::Debug for InputGroupTextarea<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("InputGroupTextarea")
            .field("theme", &self.theme)
            .field("content", &self.content.text())
            .field("placeholder", &self.placeholder)
            .field("props", &self.props)
            .field("id", &self.id)
            .field("on_action", &self.on_action.is_some())
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

impl<'a, Message> InputGroupTextarea<'a, Message> {
    /// Creates a textarea backed by caller-owned iced editor content.
    pub fn new(content: &'a text_editor::Content, theme: &'a Theme) -> Self {
        Self {
            theme,
            content,
            placeholder: Fragment::default(),
            props: InputGroupTextareaProps::default(),
            id: None,
            on_action: None,
            style_override: None,
        }
    }

    /// Applies compatibility textarea options.
    pub fn apply_props(mut self, props: InputGroupTextareaProps) -> Self {
        self.props = props;
        self
    }

    /// Sets the placeholder.
    pub fn placeholder(mut self, placeholder: impl IntoFragment<'a>) -> Self {
        self.placeholder = placeholder.into_fragment();
        self
    }

    /// Sets the size ladder.
    pub fn size(mut self, size: InputSize) -> Self {
        self.props = self.props.size(size);
        self
    }

    /// Disables editing.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.props = self.props.disabled(disabled);
        self
    }

    /// Makes the textarea read-only.
    pub fn read_only(mut self, read_only: bool) -> Self {
        self.props = self.props.read_only(read_only);
        self
    }

    /// Marks the textarea invalid.
    pub fn invalid(mut self, invalid: bool) -> Self {
        self.props = self.props.invalid(invalid);
        self
    }

    /// Sets vertical and horizontal padding in pixels.
    pub fn padding(mut self, padding: [f32; 2]) -> Self {
        self.props = self.props.padding(padding);
        self
    }

    /// Sets the minimum row count.
    pub fn rows(mut self, rows: usize) -> Self {
        self.props = self.props.rows(rows);
        self
    }

    /// Sets the maximum row count.
    pub fn max_rows(mut self, rows: usize) -> Self {
        self.props = self.props.max_rows(rows);
        self
    }

    /// Sets the resize policy.
    ///
    /// Iced does not expose a browser-style resize handle. `None` fixes the
    /// minimum height; the other modes leave the editor height unconstrained.
    pub fn resize(mut self, resize: InputGroupTextareaResize) -> Self {
        self.props = self.props.resize(resize);
        self
    }

    /// Sets the iced text wrapping strategy.
    pub fn wrapping(mut self, wrapping: iced_core::text::Wrapping) -> Self {
        self.props = self.props.wrapping(wrapping);
        self
    }

    /// Sets a maximum character count enforced by the action helper.
    pub fn max_len(mut self, max_len: usize) -> Self {
        self.props = self.props.max_len(max_len);
        self
    }

    /// Sets the editor id for focus management.
    pub fn id(mut self, id: impl Into<widget::Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Sets the callback receiving editor actions.
    pub fn on_action(mut self, on_action: impl Fn(text_editor::Action) -> Message + 'a) -> Self {
        self.on_action = Some(Box::new(on_action));
        self
    }

    /// Sets or clears the editor action callback.
    pub fn on_action_maybe(
        mut self,
        on_action: Option<impl Fn(text_editor::Action) -> Message + 'a>,
    ) -> Self {
        self.on_action = on_action.map(|callback| Box::new(callback) as _);
        self
    }

    /// Applies an iced text-editor style override after group-neutral styling.
    pub fn style_override(
        mut self,
        style_override: impl Fn(text_editor::Style, text_editor::Status) -> text_editor::Style + 'a,
    ) -> Self {
        self.style_override = Some(Box::new(style_override));
        self
    }

    /// Builds the textarea as an iced element.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        render::build_textarea(self)
    }
}

impl<'a, Message> From<InputGroupTextarea<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(textarea: InputGroupTextarea<'a, Message>) -> Self {
        textarea.into_element()
    }
}

/// Creates a compatibility addon item.
pub fn input_group_addon<'a, Message: 'a>(
    content: impl Into<Element<'a, Message>>,
    props: InputGroupAddonProps,
    theme: &'a Theme,
) -> InputGroupItem<'a, Message> {
    InputGroupAddon::new(content, theme)
        .apply_props(props)
        .into()
}

/// Creates a compatibility control item.
pub fn input_group_control<'a, Message>(
    content: impl Into<Element<'a, Message>>,
) -> InputGroupItem<'a, Message> {
    InputGroupItem::control(content)
}

/// Creates a compact button element for an addon.
pub fn input_group_button<'a, Message>(
    content: impl Into<Element<'a, Message>>,
    on_press: Option<Message>,
    props: InputGroupButtonProps,
    theme: &'a Theme,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let mut button = InputGroupButton::new(content, theme)
        .variant(props.variant)
        .size(props.size)
        .disabled(props.disabled);

    if let Some(radius) = props.radius {
        button = button.radius(radius);
    }

    button.on_press_maybe(on_press).into()
}

/// Creates muted supporting text for an addon.
pub fn input_group_text<'a, Message>(
    value: impl IntoFragment<'a>,
    theme: &'a Theme,
) -> Element<'a, Message>
where
    Message: 'a,
{
    InputGroupText::text(value, theme).into()
}

/// Creates an input-group control from the full controlled input API.
pub fn input_group_input<'a, Message, F>(
    value: impl IntoFragment<'a>,
    placeholder: impl IntoFragment<'a>,
    on_input: Option<F>,
    props: InputGroupInputProps,
    theme: &'a Theme,
) -> InputGroupItem<'a, Message>
where
    Message: Clone + 'a,
    F: Fn(String) -> Message + 'a,
{
    let mut input = Input::new(theme)
        .value(value)
        .placeholder(placeholder)
        .size(props.size)
        .disabled(props.disabled)
        .invalid(props.invalid)
        .width(Length::Fill);

    if !props.disabled
        && !props.read_only
        && let Some(on_input) = on_input
    {
        input = input.on_input(on_input);
    }

    input.into()
}

/// Creates a textarea control item.
pub fn input_group_textarea<'a, Message, F>(
    content: &'a text_editor::Content,
    placeholder: impl IntoFragment<'a>,
    on_action: Option<F>,
    props: InputGroupTextareaProps,
    theme: &'a Theme,
) -> InputGroupItem<'a, Message>
where
    Message: 'a,
    F: Fn(text_editor::Action) -> Message + 'a,
{
    InputGroupTextarea::new(content, theme)
        .placeholder(placeholder)
        .apply_props(props)
        .on_action_maybe(on_action)
        .into()
}

/// Applies a textarea action while honoring read-only and maximum-length
/// options. The helper returns whether the content changed.
pub fn input_group_textarea_apply_action(
    content: &mut text_editor::Content,
    action: text_editor::Action,
    props: InputGroupTextareaProps,
) -> bool {
    if props.disabled || (props.read_only && action.is_edit()) {
        return false;
    }

    if let Some(max_len) = props.max_len
        && !can_apply_edit(content, &action, max_len)
    {
        return false;
    }

    content.perform(action);
    true
}

fn can_apply_edit(
    content: &text_editor::Content,
    action: &text_editor::Action,
    max_len: usize,
) -> bool {
    let text_editor::Action::Edit(edit) = action else {
        return true;
    };

    let current_len = content.text().chars().count();
    let selection_len = selection_len(content);
    let insert_len = match edit {
        text_editor::Edit::Insert(_) => 1,
        text_editor::Edit::Paste(text) => text.chars().count(),
        text_editor::Edit::Enter => content
            .line_ending()
            .unwrap_or_default()
            .as_str()
            .chars()
            .count(),
        text_editor::Edit::Indent
        | text_editor::Edit::Unindent
        | text_editor::Edit::Backspace
        | text_editor::Edit::Delete => 0,
    };

    insert_len == 0 || current_len.saturating_sub(selection_len) + insert_len <= max_len
}

fn selection_len(content: &text_editor::Content) -> usize {
    let cursor = content.cursor();
    let Some(selection) = cursor.selection else {
        return 0;
    };

    position_to_index(content, cursor.position).abs_diff(position_to_index(content, selection))
}

fn position_to_index(content: &text_editor::Content, position: text_editor::Position) -> usize {
    let mut index = 0;

    for (line_index, line) in content.lines().enumerate() {
        if line_index == position.line {
            return index + position.column.min(line.text.chars().count());
        }

        index += line.text.chars().count();
        index += line.ending.as_str().chars().count();
    }

    index
}
