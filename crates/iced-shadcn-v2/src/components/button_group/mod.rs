//! Builder-first button-group component.
//!
//! Port of shadcn-svelte `ButtonGroup` / `ButtonGroupSeparator` /
//! `ButtonGroupText`. The public API lives in this module; layout assembly,
//! corner flattening, and border merging are kept in the private `render`
//! submodule.

mod render;
mod types;

#[cfg(test)]
mod tests;

pub use types::ButtonGroupOrientation;

use std::fmt;

use crate::iced_compat::widget::container as container_widget;
use crate::iced_compat::widget::text::{Fragment, IntoFragment};
use crate::iced_compat::{Element, Length};

use super::button::Button;
use super::separator::Separator;
use crate::theme::Theme;

/// Container that groups related buttons into one visual control.
///
/// Port of shadcn-svelte `ButtonGroup`: children are laid out along
/// [`ButtonGroupOrientation`] with merged edges — outer corners keep the
/// button radius of the active style pack, inner corners are flattened, and
/// adjacent 1 px borders collapse into a single divider (the web
/// `rounded-*-none` / `border-*-0` rules).
///
/// Children are pushed as [`ButtonGroupItem`]s: [`Button`]s and
/// [`ButtonGroupText`] cells participate in corner merging, separators map to
/// the web `ButtonGroupSeparator`, arbitrary elements (inputs, pickers, …)
/// are laid out as-is, and nested groups reproduce the web
/// `has-[>[data-slot=button-group]]:gap-2` spacing — a group containing
/// groups spaces them by [`Self::nested_gap`] instead of merging edges.
///
/// Like the web `w-fit` container, the group hugs its content by default.
/// For [`ButtonGroupOrientation::Vertical`] groups, setting an explicit
/// [`Self::width`] additionally stretches buttons and text cells to fill it
/// (the `items-stretch` behavior of the flex column).
///
/// ```rust,no_run
/// use iced::Element;
/// use iced_shadcn_v2::{Button, ButtonGroup, ButtonVariant, Theme};
///
/// #[derive(Debug, Clone)]
/// enum Message {
///     Prev,
///     Next,
/// }
///
/// fn pager(theme: &Theme) -> Element<'_, Message> {
///     ButtonGroup::new(theme)
///         .push(
///             Button::text("Previous", theme)
///                 .variant(ButtonVariant::Outline)
///                 .on_press(Message::Prev),
///         )
///         .push(
///             Button::text("Next", theme)
///                 .variant(ButtonVariant::Outline)
///                 .on_press(Message::Next),
///         )
///         .into()
/// }
/// ```
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct ButtonGroup<'a, Message> {
    theme: &'a Theme,
    orientation: ButtonGroupOrientation,
    items: Vec<ButtonGroupItem<'a, Message>>,
    width: Length,
    height: Length,
    nested_gap: f32,
    aria_label: Option<String>,
}

impl<Message> fmt::Debug for ButtonGroup<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ButtonGroup")
            .field("theme", &self.theme)
            .field("orientation", &self.orientation)
            .field("items", &self.items)
            .field("width", &self.width)
            .field("height", &self.height)
            .field("nested_gap", &self.nested_gap)
            .field("aria_label", &self.aria_label)
            .finish()
    }
}

impl<'a, Message> ButtonGroup<'a, Message> {
    /// Creates an empty horizontal group.
    ///
    /// `theme` is required because default separators and text cells derive
    /// their styling from `shadcn-common` theme tokens.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            orientation: ButtonGroupOrientation::default(),
            items: Vec::new(),
            width: Length::Shrink,
            height: Length::Shrink,
            nested_gap: render::DEFAULT_NESTED_GAP,
            aria_label: None,
        }
    }

    /// Creates a group with the given children.
    pub fn with_children(
        theme: &'a Theme,
        children: impl IntoIterator<Item = ButtonGroupItem<'a, Message>>,
    ) -> Self {
        Self::new(theme).extend(children)
    }

    /// Sets the layout axis of the group.
    pub fn orientation(mut self, orientation: ButtonGroupOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Appends a child: a [`Button`], a [`ButtonGroupText`], a [`Separator`],
    /// a nested [`ButtonGroup`], or a prebuilt [`ButtonGroupItem`].
    pub fn push(mut self, item: impl Into<ButtonGroupItem<'a, Message>>) -> Self {
        self.items.push(item.into());
        self
    }

    /// Appends an arbitrary widget (input, pick list, …).
    ///
    /// Opaque elements keep their own styling — the group cannot flatten
    /// their corners, but adjacent borders still merge into one divider.
    pub fn push_element(self, element: impl Into<Element<'a, Message>>) -> Self {
        self.push(ButtonGroupItem::element(element))
    }

    /// Appends a divider — port of the web `ButtonGroupSeparator`.
    ///
    /// The rule is painted with the theme `input` token, spans the cross
    /// axis, and is oriented perpendicular to the group automatically. Push
    /// a custom [`Separator`] to change its color or thickness.
    pub fn push_separator(self) -> Self {
        let rule = Separator::from_color(self.theme.palette.input);
        self.push(rule)
    }

    /// Appends every child of the given iterator.
    pub fn extend(self, children: impl IntoIterator<Item = ButtonGroupItem<'a, Message>>) -> Self {
        children.into_iter().fold(self, Self::push)
    }

    /// Sets a custom group width (defaults to `w-fit` / [`Length::Shrink`]).
    ///
    /// Vertical groups with an explicit width stretch their buttons and text
    /// cells to fill it, mirroring the `items-stretch` flex behavior.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets a custom group height (defaults to [`Length::Shrink`]).
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Sets the gap in px between children of a nesting group (a group that
    /// contains other groups). Defaults to 8 px (`gap-2`); clamped to at
    /// least 0 px. Groups without nested groups merge edges instead.
    pub fn nested_gap(mut self, nested_gap: f32) -> Self {
        self.nested_gap = nested_gap.max(0.0);
        self
    }

    /// Sets the accessible label of the group.
    ///
    /// Mirrors `aria-label` on the web `role="group"` element. iced does not
    /// expose an accessibility tree yet, so the label is carried for API
    /// parity and will take effect once accessibility support lands.
    pub fn aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    /// Builds the group as an iced [`Element`](iced_core::Element).
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        render::build_group(self)
    }
}

impl<'a, Message> From<ButtonGroup<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(group: ButtonGroup<'a, Message>) -> Self {
        group.into_element()
    }
}

/// A single child of a [`ButtonGroup`].
///
/// Usually created implicitly through the [`From`] conversions accepted by
/// [`ButtonGroup::push`]; use [`Self::element`] for arbitrary widgets.
#[must_use = "items do nothing unless pushed into a ButtonGroup"]
pub struct ButtonGroupItem<'a, Message> {
    pub(super) kind: ItemKind<'a, Message>,
}

pub(super) enum ItemKind<'a, Message> {
    Button(Box<Button<'a, Message>>),
    Text(Box<ButtonGroupText<'a, Message>>),
    Separator(Separator),
    Element(Element<'a, Message>),
    Group(Box<ButtonGroup<'a, Message>>),
}

impl<'a, Message> ButtonGroupItem<'a, Message> {
    /// Wraps an arbitrary widget (input, pick list, …) as a group child.
    pub fn element(element: impl Into<Element<'a, Message>>) -> Self {
        Self {
            kind: ItemKind::Element(element.into()),
        }
    }

    pub(super) const fn kind_name(&self) -> &'static str {
        match self.kind {
            ItemKind::Button(_) => "button",
            ItemKind::Text(_) => "text",
            ItemKind::Separator(_) => "separator",
            ItemKind::Element(_) => "element",
            ItemKind::Group(_) => "group",
        }
    }
}

impl<Message> fmt::Debug for ButtonGroupItem<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("ButtonGroupItem")
            .field(&self.kind_name())
            .finish()
    }
}

impl<'a, Message> From<Button<'a, Message>> for ButtonGroupItem<'a, Message> {
    fn from(button: Button<'a, Message>) -> Self {
        Self {
            kind: ItemKind::Button(Box::new(button)),
        }
    }
}

impl<'a, Message> From<ButtonGroupText<'a, Message>> for ButtonGroupItem<'a, Message> {
    fn from(text: ButtonGroupText<'a, Message>) -> Self {
        Self {
            kind: ItemKind::Text(Box::new(text)),
        }
    }
}

impl<Message> From<Separator> for ButtonGroupItem<'_, Message> {
    fn from(separator: Separator) -> Self {
        Self {
            kind: ItemKind::Separator(separator),
        }
    }
}

impl<'a, Message> From<ButtonGroup<'a, Message>> for ButtonGroupItem<'a, Message> {
    fn from(group: ButtonGroup<'a, Message>) -> Self {
        Self {
            kind: ItemKind::Group(Box::new(group)),
        }
    }
}

impl<'a, Message> From<Element<'a, Message>> for ButtonGroupItem<'a, Message> {
    fn from(element: Element<'a, Message>) -> Self {
        Self::element(element)
    }
}

/// Static text cell inside a [`ButtonGroup`] — port of the web
/// `ButtonGroupText`.
///
/// A non-interactive chip on the theme `muted` surface with a 1 px `border`
/// outline, horizontal `px-4` padding, and the button typography of the
/// active style pack. Inside a group its corners are flattened like any
/// button; standalone it keeps the full button radius.
///
/// ```rust,no_run
/// use iced::Element;
/// use iced_shadcn_v2::{Button, ButtonGroup, ButtonGroupText, ButtonVariant, Theme};
///
/// #[derive(Debug, Clone)]
/// enum Message {
///     Copy,
/// }
///
/// fn labelled(theme: &Theme) -> Element<'_, Message> {
///     ButtonGroup::new(theme)
///         .push(ButtonGroupText::text("https://", theme))
///         .push(
///             Button::text("Copy", theme)
///                 .variant(ButtonVariant::Outline)
///                 .on_press(Message::Copy),
///         )
///         .into()
/// }
/// ```
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct ButtonGroupText<'a, Message> {
    pub(super) content: TextContent<'a, Message>,
    pub(super) theme: &'a Theme,
    pub(super) padding_x: Option<f32>,
    pub(super) text_size: Option<f32>,
    pub(super) style_override:
        Option<Box<dyn Fn(container_widget::Style) -> container_widget::Style + 'a>>,
}

pub(super) enum TextContent<'a, Message> {
    Label(Fragment<'a>),
    Element(Element<'a, Message>),
}

impl<Message> fmt::Debug for ButtonGroupText<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let content = match &self.content {
            TextContent::Label(_) => "label",
            TextContent::Element(_) => "element",
        };

        formatter
            .debug_struct("ButtonGroupText")
            .field("content", &content)
            .field("theme", &self.theme)
            .field("padding_x", &self.padding_x)
            .field("text_size", &self.text_size)
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

impl<'a, Message> ButtonGroupText<'a, Message> {
    /// Creates a text cell from arbitrary content (e.g. an icon with a
    /// label).
    pub fn new(content: impl Into<Element<'a, Message>>, theme: &'a Theme) -> Self {
        Self::from_content(TextContent::Element(content.into()), theme)
    }

    /// Creates a text cell from a label.
    pub fn text(label: impl IntoFragment<'a>, theme: &'a Theme) -> Self {
        Self::from_content(TextContent::Label(label.into_fragment()), theme)
    }

    fn from_content(content: TextContent<'a, Message>, theme: &'a Theme) -> Self {
        Self {
            content,
            theme,
            padding_x: None,
            text_size: None,
            style_override: None,
        }
    }

    /// Sets the horizontal padding in px (defaults to 16 px / `px-4`).
    ///
    /// Negative values are clamped to `0.0`.
    pub fn padding_x(mut self, padding_x: f32) -> Self {
        self.padding_x = Some(padding_x.max(0.0));
        self
    }

    /// Sets the label text size in px (defaults to the style-pack button
    /// text size). Values are clamped to at least 1 px; element content is
    /// unaffected.
    pub fn text_size(mut self, text_size: f32) -> Self {
        self.text_size = Some(text_size.max(1.0));
        self
    }

    /// Applies a narrow iced-style escape hatch after internal style
    /// resolution (the equivalent of the svelte `class` override).
    ///
    /// Text labels inherit `container::Style::text_color`, so changing it
    /// here recolors the label as well.
    pub fn style_override(
        mut self,
        style_override: impl Fn(container_widget::Style) -> container_widget::Style + 'a,
    ) -> Self {
        self.style_override = Some(Box::new(style_override));
        self
    }

    /// Builds the text cell as a standalone iced
    /// [`Element`](iced_core::Element) with the full button radius.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        render::build_standalone_text(self)
    }
}

impl<'a, Message> From<ButtonGroupText<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(text: ButtonGroupText<'a, Message>) -> Self {
        text.into_element()
    }
}
