//! Builder-first alert component.
//!
//! This is the iced composition counterpart of shadcn-svelte's
//! `Alert.Root`, `Alert.Title`, `Alert.Description`, and `Alert.Action`.
//! The root supports arbitrary iced elements, an explicit icon slot, typed
//! title/description typography, an absolutely-positioned action layer, and
//! the `default` / `destructive` variants from the source component.
//!
//! Iced does not currently expose a DOM role attribute, so the visual root is
//! the portable stand-in for the web component's `role="alert"`. Applications
//! should still place alerts in their own announcement/accessibility model.
//!
//! ```rust,no_run
//! use iced::{Element, widget::text};
//! use iced_shadcn_v2::{Alert, AlertDescription, AlertTitle, Theme};
//!
//! #[derive(Debug, Clone)]
//! enum Message {}
//!
//! fn view(theme: &Theme) -> Element<'_, Message> {
//!     Alert::new(theme)
//!         .title(AlertTitle::text("Heads up!", theme))
//!         .description(AlertDescription::text(
//!             "You can add components to your app using the CLI.",
//!             theme,
//!         ))
//!         .push(text("Additional content"))
//!         .into()
//! }
//! ```

mod geometry;
mod render;
mod style;
mod types;

#[cfg(test)]
mod tests;

pub use types::{AlertRadius, AlertVariant};

use std::fmt;

use crate::iced_compat::widget::container;
use crate::iced_compat::widget::text::{Fragment, IntoFragment};
use crate::iced_compat::{Color, Element, Font, Length, Padding};

use crate::theme::Theme;

/// A composable callout that draws attention to information, success, or an
/// error state.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct Alert<'a, Message> {
    theme: &'a Theme,
    variant: AlertVariant,
    radius: AlertRadius,
    width: Length,
    height: Length,
    padding: Option<Padding>,
    spacing: Option<f32>,
    icon: Option<Element<'a, Message>>,
    items: Vec<AlertItem<'a, Message>>,
    action: Option<AlertAction<'a, Message>>,
    style_override: Option<Box<dyn Fn(container::Style) -> container::Style + 'a>>,
}

enum AlertItem<'a, Message> {
    Element(Element<'a, Message>),
    Title(AlertTitle<'a, Message>),
    Description(AlertDescription<'a, Message>),
}

enum AlertTextContent<'a, Message> {
    Label(Fragment<'a>),
    Element(Element<'a, Message>),
}

impl<Message> fmt::Debug for Alert<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Alert")
            .field("theme", &self.theme)
            .field("variant", &self.variant)
            .field("radius", &self.radius)
            .field("width", &self.width)
            .field("height", &self.height)
            .field("padding", &self.padding)
            .field("spacing", &self.spacing)
            .field("icon", &self.icon.is_some())
            .field("items", &self.items.len())
            .field("action", &self.action.is_some())
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

impl<'a, Message> Alert<'a, Message> {
    /// Creates an empty alert using the active style-pack defaults.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            variant: AlertVariant::Default,
            radius: AlertRadius::Theme,
            width: Length::Fill,
            height: Length::Shrink,
            padding: None,
            spacing: None,
            icon: None,
            items: Vec::new(),
            action: None,
            style_override: None,
        }
    }

    /// Creates an alert whose body is populated from an iterator.
    pub fn with_children(
        theme: &'a Theme,
        children: impl IntoIterator<Item = Element<'a, Message>>,
    ) -> Self {
        Self {
            items: children.into_iter().map(AlertItem::Element).collect(),
            ..Self::new(theme)
        }
    }

    /// Sets the alert's visual variant.
    pub fn variant(mut self, variant: AlertVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Sets the alert corner radius.
    pub fn radius(mut self, radius: AlertRadius) -> Self {
        self.radius = radius;
        self
    }

    /// Sets the alert width.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the alert height.
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Overrides all four root padding sides in pixels.
    ///
    /// Negative and non-finite sides are normalized to zero so an invalid
    /// layout value cannot escape into iced's layout engine.
    pub fn padding(mut self, padding: Padding) -> Self {
        self.padding = Some(geometry::normalize_padding(padding));
        self
    }

    /// Sets the vertical gap between body items in pixels.
    ///
    /// The default is style-pack specific (`gap-0.5`, or `gap-1` for Sera).
    /// Negative and non-finite values resolve to zero.
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = Some(geometry::normalize_px(spacing));
        self
    }

    /// Sets an arbitrary leading icon element.
    ///
    /// The icon is constrained to the source component's default icon
    /// footprint. Supply an already-sized element when a custom glyph needs a
    /// different visual size.
    pub fn icon(mut self, icon: impl Into<Element<'a, Message>>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Alias for [`Self::icon`] that makes the slot direction explicit.
    pub fn icon_start(self, icon: impl Into<Element<'a, Message>>) -> Self {
        self.icon(icon)
    }

    /// Adds a typed title slot.
    pub fn title(mut self, title: AlertTitle<'a, Message>) -> Self {
        self.items.push(AlertItem::Title(title));
        self
    }

    /// Adds a typed description slot.
    pub fn description(mut self, description: AlertDescription<'a, Message>) -> Self {
        self.items.push(AlertItem::Description(description));
        self
    }

    /// Adds a typed action slot rendered at the root's top-right edge.
    pub fn action(mut self, action: AlertAction<'a, Message>) -> Self {
        self.action = Some(action);
        self
    }

    /// Appends arbitrary content to the alert body.
    pub fn push(mut self, child: impl Into<Element<'a, Message>>) -> Self {
        self.items.push(AlertItem::Element(child.into()));
        self
    }

    /// Appends every element from an iterator to the alert body.
    pub fn extend(self, children: impl IntoIterator<Item = Element<'a, Message>>) -> Self {
        children.into_iter().fold(self, Self::push)
    }

    /// Applies an iced container-style override after semantic resolution.
    pub fn style_override(
        mut self,
        style_override: impl Fn(container::Style) -> container::Style + 'a,
    ) -> Self {
        self.style_override = Some(Box::new(style_override));
        self
    }

    /// Builds the alert as an iced [`Element`](iced_core::Element).
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        render::build_alert(self)
    }
}

impl<'a, Message> From<Alert<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(alert: Alert<'a, Message>) -> Self {
        alert.into_element()
    }
}

/// Styled title content for an [`Alert`].
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct AlertTitle<'a, Message> {
    content: AlertTextContent<'a, Message>,
    theme: &'a Theme,
    text_size: Option<f32>,
    line_height: Option<f32>,
    color: Option<Color>,
    font: Option<Font>,
    width: Length,
    style_override: Option<Box<dyn Fn(container::Style) -> container::Style + 'a>>,
}

impl<Message> fmt::Debug for AlertTitle<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AlertTitle")
            .field("content", &self.content.kind())
            .field("theme", &self.theme)
            .field("text_size", &self.text_size)
            .field("line_height", &self.line_height)
            .field("color", &self.color)
            .field("font", &self.font)
            .field("width", &self.width)
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

impl<'a, Message> AlertTitle<'a, Message> {
    /// Creates a title from arbitrary iced content.
    pub fn new(content: impl Into<Element<'a, Message>>, theme: &'a Theme) -> Self {
        Self::from_content(AlertTextContent::Element(content.into()), theme)
    }

    /// Creates a style-pack-aware text title.
    pub fn text(label: impl IntoFragment<'a>, theme: &'a Theme) -> Self {
        Self::from_content(AlertTextContent::Label(label.into_fragment()), theme)
    }

    fn from_content(content: AlertTextContent<'a, Message>, theme: &'a Theme) -> Self {
        Self {
            content,
            theme,
            text_size: None,
            line_height: None,
            color: None,
            font: None,
            width: Length::Fill,
            style_override: None,
        }
    }

    /// Sets the title text size in pixels.
    pub fn text_size(mut self, text_size: f32) -> Self {
        self.text_size = Some(geometry::normalize_min_px(text_size));
        self
    }

    /// Sets the title line height in pixels.
    pub fn line_height(mut self, line_height: f32) -> Self {
        self.line_height = Some(geometry::normalize_min_px(line_height));
        self
    }

    /// Sets the title text color.
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Sets the title font.
    pub fn font(mut self, font: Font) -> Self {
        self.font = Some(font);
        self
    }

    /// Sets the title width.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Applies an iced container-style override to the title.
    pub fn style_override(
        mut self,
        style_override: impl Fn(container::Style) -> container::Style + 'a,
    ) -> Self {
        self.style_override = Some(Box::new(style_override));
        self
    }

    /// Builds the title as an iced [`Element`](iced_core::Element).
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        render::build_title(self, AlertVariant::Default)
    }
}

impl<'a, Message> From<AlertTitle<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(title: AlertTitle<'a, Message>) -> Self {
        title.into_element()
    }
}

/// Styled description content for an [`Alert`].
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct AlertDescription<'a, Message> {
    content: AlertTextContent<'a, Message>,
    theme: &'a Theme,
    text_size: Option<f32>,
    line_height: Option<f32>,
    color: Option<Color>,
    font: Option<Font>,
    width: Length,
    style_override: Option<Box<dyn Fn(container::Style) -> container::Style + 'a>>,
}

impl<Message> fmt::Debug for AlertDescription<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AlertDescription")
            .field("content", &self.content.kind())
            .field("theme", &self.theme)
            .field("text_size", &self.text_size)
            .field("line_height", &self.line_height)
            .field("color", &self.color)
            .field("font", &self.font)
            .field("width", &self.width)
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

impl<'a, Message> AlertDescription<'a, Message> {
    /// Creates a description from arbitrary iced content.
    pub fn new(content: impl Into<Element<'a, Message>>, theme: &'a Theme) -> Self {
        Self::from_content(AlertTextContent::Element(content.into()), theme)
    }

    /// Creates a style-pack-aware text description.
    pub fn text(label: impl IntoFragment<'a>, theme: &'a Theme) -> Self {
        Self::from_content(AlertTextContent::Label(label.into_fragment()), theme)
    }

    fn from_content(content: AlertTextContent<'a, Message>, theme: &'a Theme) -> Self {
        Self {
            content,
            theme,
            text_size: None,
            line_height: None,
            color: None,
            font: None,
            width: Length::Fill,
            style_override: None,
        }
    }

    /// Sets the description text size in pixels.
    pub fn text_size(mut self, text_size: f32) -> Self {
        self.text_size = Some(geometry::normalize_min_px(text_size));
        self
    }

    /// Sets the description line height in pixels.
    pub fn line_height(mut self, line_height: f32) -> Self {
        self.line_height = Some(geometry::normalize_min_px(line_height));
        self
    }

    /// Sets the description text color.
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Sets the description font.
    pub fn font(mut self, font: Font) -> Self {
        self.font = Some(font);
        self
    }

    /// Sets the description width.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Applies an iced container-style override to the description.
    pub fn style_override(
        mut self,
        style_override: impl Fn(container::Style) -> container::Style + 'a,
    ) -> Self {
        self.style_override = Some(Box::new(style_override));
        self
    }

    /// Builds the description as an iced [`Element`](iced_core::Element).
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        render::build_description(self, AlertVariant::Default)
    }
}

impl<'a, Message> From<AlertDescription<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(description: AlertDescription<'a, Message>) -> Self {
        description.into_element()
    }
}

/// A top-right action slot for an [`Alert`].
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct AlertAction<'a, Message> {
    content: Element<'a, Message>,
    width: Length,
    height: Length,
    style_override: Option<Box<dyn Fn(container::Style) -> container::Style + 'a>>,
}

impl<Message> fmt::Debug for AlertAction<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AlertAction")
            .field("width", &self.width)
            .field("height", &self.height)
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

impl<'a, Message> AlertAction<'a, Message> {
    /// Creates an action from arbitrary iced content.
    pub fn new(content: impl Into<Element<'a, Message>>) -> Self {
        Self {
            content: content.into(),
            width: Length::Shrink,
            height: Length::Shrink,
            style_override: None,
        }
    }

    /// Sets the action width.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the action height.
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Applies an iced container-style override to the action wrapper.
    pub fn style_override(
        mut self,
        style_override: impl Fn(container::Style) -> container::Style + 'a,
    ) -> Self {
        self.style_override = Some(Box::new(style_override));
        self
    }

    /// Builds the action as an iced [`Element`](iced_core::Element).
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        render::build_action(self)
    }
}

impl<'a, Message> From<AlertAction<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(action: AlertAction<'a, Message>) -> Self {
        action.into_element()
    }
}

impl<Message> AlertTextContent<'_, Message> {
    fn kind(&self) -> &'static str {
        match self {
            Self::Label(_) => "label",
            Self::Element(_) => "element",
        }
    }
}
