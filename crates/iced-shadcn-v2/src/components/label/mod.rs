//! Builder-first label component.
//!
//! Port of the shadcn-svelte `Label` (bits-ui `Label.Root`). The public API
//! lives in this module; typography resolution and content composition are
//! kept in focused private submodules.

mod render;
mod style;
mod types;

#[cfg(test)]
mod tests;

pub use types::LabelContext;

use std::borrow::Cow;
use std::fmt;

use crate::iced_compat::widget::button as button_widget;
use crate::iced_compat::widget::container;
use crate::iced_compat::widget::text::{Fragment, IntoFragment};
use crate::iced_compat::{Color, Element, Length};

use crate::theme::Theme;

/// Builder-first form label styled directly with iced types.
///
/// Mirrors shadcn-svelte `Label`: theme-aware typography (`text-sm` /
/// `font-medium`, with style-pack overrides for Lyra / Mira / Sera), flex row
/// with `gap-2` for optional icons, disabled opacity, and an optional
/// `for` / click association for focusing a control.
///
/// Theme tokens come from `shadcn-common` via [`Theme`]. Non-interactive
/// labels render as styled text (optionally in a `container`); interactive
/// ones (`on_press`) use a transparent iced `button` so clicking the label
/// can focus or toggle the associated control in app code.
///
/// ```rust,no_run
/// use iced::Element;
/// use iced_shadcn_v2::{Label, Theme};
///
/// #[derive(Debug, Clone)]
/// enum Message {
///     FocusEmail,
/// }
///
/// fn email_label(theme: &Theme) -> Element<'_, Message> {
///     Label::text("Email", theme)
///         .for_id("email")
///         .on_press(Message::FocusEmail)
///         .into()
/// }
/// ```
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct Label<'a, Message> {
    content: LabelContent<'a, Message>,
    theme: &'a Theme,
    context: LabelContext,
    color: Option<Color>,
    width: Length,
    disabled: bool,
    icon_start: Option<Element<'a, Message>>,
    icon_end: Option<Element<'a, Message>>,
    for_id: Option<Cow<'a, str>>,
    on_press: Option<Message>,
    style_override: Option<LabelStyleOverride<'a>>,
}

enum LabelContent<'a, Message> {
    Text(Fragment<'a>),
    Element(Element<'a, Message>),
}

enum LabelStyleOverride<'a> {
    Container(Box<dyn Fn(container::Style) -> container::Style + 'a>),
    Button(Box<dyn Fn(button_widget::Style, button_widget::Status) -> button_widget::Style + 'a>),
}

impl<Message> fmt::Debug for Label<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let content = match &self.content {
            LabelContent::Text(_) => "text",
            LabelContent::Element(_) => "element",
        };

        formatter
            .debug_struct("Label")
            .field("content", &content)
            .field("theme", &self.theme)
            .field("context", &self.context)
            .field("color", &self.color)
            .field("width", &self.width)
            .field("disabled", &self.disabled)
            .field("icon_start", &self.icon_start.is_some())
            .field("icon_end", &self.icon_end.is_some())
            .field("for_id", &self.for_id.as_deref())
            .field("on_press", &self.on_press.is_some())
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

impl<'a, Message> Label<'a, Message> {
    /// Creates a new label from arbitrary content.
    ///
    /// `theme` is required because typography and color resolve from
    /// `shadcn-common` theme tokens instead of `iced::Theme`.
    pub fn new(content: impl Into<Element<'a, Message>>, theme: &'a Theme) -> Self {
        Self::from_content(LabelContent::Element(content.into()), theme)
    }

    /// Creates a text label.
    pub fn text(label: impl IntoFragment<'a>, theme: &'a Theme) -> Self {
        Self::from_content(LabelContent::Text(label.into_fragment()), theme)
    }

    fn from_content(content: LabelContent<'a, Message>, theme: &'a Theme) -> Self {
        Self {
            content,
            theme,
            context: LabelContext::default(),
            color: None,
            width: Length::Shrink,
            disabled: false,
            icon_start: None,
            icon_end: None,
            for_id: None,
            on_press: None,
            style_override: None,
        }
    }

    /// Sets the layout role relative to the associated control.
    ///
    /// iced has no CSS `peer-*` selectors. This is the explicit stand-in for
    /// Sera’s `peer-data-[slot=checkbox|radio-group-item|switch]:*` rules:
    /// use [`LabelContext::AdjacentControl`] when the label sits next to a
    /// checkbox, radio, or switch. Other style packs ignore the distinction.
    pub fn context(mut self, context: LabelContext) -> Self {
        self.context = context;
        self
    }

    /// Overrides the label foreground color (defaults to theme `foreground`).
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Sets a custom label width.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Applies the disabled treatment (`opacity-50` / peer-disabled).
    ///
    /// Mirrors `group-data-[disabled=true]` and `peer-disabled` on the web
    /// component. Also suppresses `on_press` while disabled.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Sets a leading icon (shadcn flex row with `gap-2`).
    pub fn icon_start(mut self, icon: impl Into<Element<'a, Message>>) -> Self {
        self.icon_start = Some(icon.into());
        self
    }

    /// Sets a trailing icon (shadcn flex row with `gap-2`).
    pub fn icon_end(mut self, icon: impl Into<Element<'a, Message>>) -> Self {
        self.icon_end = Some(icon.into());
        self
    }

    /// Associates the label with a control id (`htmlFor` / `for` on the web).
    ///
    /// iced does not yet expose an accessibility tree, so the id is carried
    /// for API parity and for apps that wire focus manually via
    /// [`Self::on_press`]. It will take effect once accessibility support
    /// lands (same pattern as [`crate::Separator::decorative`]).
    pub fn for_id(mut self, id: impl Into<Cow<'a, str>>) -> Self {
        self.for_id = Some(id.into());
        self
    }

    /// Sets the message emitted when the label is pressed.
    ///
    /// Use this to focus or toggle the associated control — the iced stand-in
    /// for native `htmlFor` click-to-focus behavior.
    pub fn on_press(mut self, message: Message) -> Self {
        self.on_press = Some(message);
        self
    }

    /// Sets or clears the message emitted when the label is pressed.
    pub fn on_press_maybe(mut self, message: Option<Message>) -> Self {
        self.on_press = message;
        self
    }

    /// Applies a narrow iced-style escape hatch for non-interactive labels.
    ///
    /// Prefer [`Self::button_style_override`] when the label is interactive.
    pub fn style_override(
        mut self,
        style_override: impl Fn(container::Style) -> container::Style + 'a,
    ) -> Self {
        self.style_override = Some(LabelStyleOverride::Container(Box::new(style_override)));
        self
    }

    /// Applies a narrow iced-style escape hatch for interactive labels.
    pub fn button_style_override(
        mut self,
        style_override: impl Fn(button_widget::Style, button_widget::Status) -> button_widget::Style
        + 'a,
    ) -> Self {
        self.style_override = Some(LabelStyleOverride::Button(Box::new(style_override)));
        self
    }

    /// Returns the associated control id, if any.
    pub fn associated_id(&self) -> Option<&str> {
        self.for_id.as_deref()
    }

    /// Builds the label as an iced [`Element`](iced_core::Element).
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        let Label {
            content,
            theme,
            context,
            color,
            width,
            disabled,
            icon_start,
            icon_end,
            for_id: _,
            on_press,
            style_override,
        } = self;

        let recipe = style::resolve_recipe(theme, context);
        let text_color = style::resolve_color(theme, color, disabled);
        let body = render::build_content(content, icon_start, icon_end, recipe, text_color, theme);

        if let Some(message) = on_press {
            render::wrap_interactive(body, width, text_color, message, disabled, style_override)
        } else {
            render::wrap_static(body, width, style_override)
        }
    }
}

impl<'a, Message> From<Label<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(label: Label<'a, Message>) -> Self {
        label.into_element()
    }
}
