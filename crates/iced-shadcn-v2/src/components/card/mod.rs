//! Builder-first card component.
//!
//! This is the iced composition counterpart of shadcn-svelte's `Card.Root`
//! family: a styled container with header, title, description, action,
//! content, and footer slots. All slots accept arbitrary iced elements, while
//! the typed title and description helpers retain style-pack typography.
//!
//! ```rust,no_run
//! use iced::{Element, Length};
//! use iced_shadcn_v2::{
//!     Button, Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle, Theme,
//! };
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     Save,
//! }
//!
//! fn view(theme: &Theme) -> Element<'_, Message> {
//!     Card::new(theme)
//!         .width(Length::Fixed(360.0))
//!         .header(
//!             CardHeader::new(theme)
//!                 .title(CardTitle::text("Create project", theme))
//!                 .description(CardDescription::text(
//!                     "Deploy your new project in one click.",
//!                     theme,
//!                 )),
//!         )
//!         .content(CardContent::new(theme).push(iced::widget::text("Project details")))
//!         .footer(
//!             CardFooter::new(theme)
//!                 .push(Button::text("Save", theme).on_press(Message::Save)),
//!         )
//!         .into()
//! }
//! ```

pub(crate) mod geometry;
mod render;
mod style;
mod types;

#[cfg(test)]
mod tests;

pub use types::{CardBorder, CardFooterAlignment, CardFooterDirection, CardRadius, CardSize};

use std::fmt;

use crate::iced_compat::widget::container;
use crate::iced_compat::widget::text::{Fragment, IntoFragment};
use crate::iced_compat::{Color, Element, Font, Length};

use crate::theme::Theme;

/// A style-pack-aware card root.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct Card<'a, Message> {
    theme: &'a Theme,
    size: CardSize,
    spacing: Option<f32>,
    radius: CardRadius,
    width: Length,
    height: Length,
    top_padding: Option<f32>,
    bottom_padding: Option<f32>,
    children: Vec<CardItem<'a, Message>>,
    footer_present: bool,
    style_override: Option<Box<dyn Fn(container::Style) -> container::Style + 'a>>,
}

enum CardItem<'a, Message> {
    Element(Element<'a, Message>),
    Header(Box<CardHeader<'a, Message>>),
    Content(Box<CardContent<'a, Message>>),
    Footer(Box<CardFooter<'a, Message>>),
}

impl<Message> fmt::Debug for Card<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Card")
            .field("theme", &self.theme)
            .field("size", &self.size)
            .field("spacing", &self.spacing)
            .field("radius", &self.radius)
            .field("width", &self.width)
            .field("height", &self.height)
            .field("top_padding", &self.top_padding)
            .field("bottom_padding", &self.bottom_padding)
            .field("children", &self.children.len())
            .field("footer_present", &self.footer_present)
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

impl<'a, Message> Card<'a, Message> {
    /// Creates an empty card using the active style-pack defaults.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            size: CardSize::Default,
            spacing: None,
            radius: CardRadius::Theme,
            width: Length::Fill,
            height: Length::Shrink,
            top_padding: None,
            bottom_padding: None,
            children: Vec::new(),
            footer_present: false,
            style_override: None,
        }
    }

    /// Creates a card with arbitrary root children.
    pub fn with_children(
        theme: &'a Theme,
        children: impl IntoIterator<Item = Element<'a, Message>>,
    ) -> Self {
        Self {
            children: children.into_iter().map(CardItem::Element).collect(),
            ..Self::new(theme)
        }
    }

    /// Appends arbitrary content to the root card column.
    pub fn push(mut self, child: impl Into<Element<'a, Message>>) -> Self {
        self.children.push(CardItem::Element(child.into()));
        self
    }

    /// Appends every element from an iterator to the root card column.
    pub fn extend(self, children: impl IntoIterator<Item = Element<'a, Message>>) -> Self {
        children.into_iter().fold(self, Self::push)
    }

    /// Adds a typed header slot.
    pub fn header(mut self, header: CardHeader<'a, Message>) -> Self {
        self.children.push(CardItem::Header(Box::new(header)));
        self
    }

    /// Adds a typed content slot.
    pub fn content(mut self, content: CardContent<'a, Message>) -> Self {
        self.children.push(CardItem::Content(Box::new(content)));
        self
    }

    /// Adds a typed footer slot and enables style-pack footer edge handling.
    pub fn footer(mut self, footer: CardFooter<'a, Message>) -> Self {
        self.footer_present = true;
        self.children.push(CardItem::Footer(Box::new(footer)));
        self
    }

    /// Sets the card density (`default` or `sm`).
    pub fn size(mut self, size: CardSize) -> Self {
        self.size = size;
        self
    }

    /// Sets card gap and section inset in pixels, analogous to
    /// shadcn-svelte's `--card-spacing` override.
    ///
    /// Non-finite and negative values are normalized to `0.0`.
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = Some(normalize_px(spacing));
        self
    }

    /// Sets the card corner radius.
    pub fn radius(mut self, radius: CardRadius) -> Self {
        self.radius = radius;
        self
    }

    /// Sets the card width.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the card height.
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Overrides the root's top padding in pixels.
    ///
    /// Use `0.0` for an edge-to-edge first child such as an image. The default
    /// is the resolved card spacing.
    pub fn top_padding(mut self, padding: f32) -> Self {
        self.top_padding = Some(normalize_px(padding));
        self
    }

    /// Overrides the root's bottom padding in pixels.
    pub fn bottom_padding(mut self, padding: f32) -> Self {
        self.bottom_padding = Some(normalize_px(padding));
        self
    }

    /// Applies an iced container-style override after semantic resolution.
    pub fn style_override(
        mut self,
        style_override: impl Fn(container::Style) -> container::Style + 'a,
    ) -> Self {
        self.style_override = Some(Box::new(style_override));
        self
    }

    /// Builds the card as an iced [`Element`](iced_core::Element).
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        let Card {
            theme,
            size,
            spacing: custom_spacing,
            radius,
            width,
            height,
            top_padding,
            bottom_padding,
            children,
            footer_present,
            style_override,
        } = self;

        let spacing = geometry::resolved_spacing(theme, size, custom_spacing);
        let root_children = children
            .into_iter()
            .map(|child| match child {
                CardItem::Element(element) => render::fill_child(element),
                CardItem::Header(header) => render::build_header(*header, spacing, size, radius),
                CardItem::Content(content) => render::build_content(*content, spacing),
                CardItem::Footer(footer) => render::build_footer(*footer, spacing, radius),
            })
            .collect::<Vec<_>>();

        let body = crate::iced_compat::widget::column(root_children)
            .spacing(spacing)
            .width(Length::Fill);
        let bottom = bottom_padding.unwrap_or_else(|| {
            if footer_present && geometry::suppress_bottom_padding(theme) {
                0.0
            } else {
                spacing
            }
        });

        let mut resolved = style::resolve_root_style(theme, radius);
        if let Some(override_fn) = style_override.as_ref() {
            resolved = override_fn(resolved);
        }

        // CSS `ring-1` is outside; keep any override color/width as the ring
        // source, then clear the inset border so opaque children cannot cover it.
        let (default_ring, default_width) = style::root_ring(theme);
        let ring_width = if resolved.border.width > 0.0 {
            resolved.border.width
        } else {
            default_width
        };
        let ring_color = if resolved.border.width > 0.0 {
            resolved.border.color
        } else {
            default_ring
        };
        let ring_radius = geometry::radius_px(theme, radius);
        resolved.border.width = 0.0;
        resolved.border.color = Color::TRANSPARENT;

        let inner = container(body)
            .padding(crate::iced_compat::Padding {
                top: top_padding.unwrap_or(spacing),
                right: 0.0,
                bottom,
                left: 0.0,
            })
            .width(width)
            .height(height)
            .clip(true)
            .style(move |_| resolved);

        render::with_outside_ring(inner.into(), ring_color, ring_width, ring_radius)
    }
}

impl<'a, Message> From<Card<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(card: Card<'a, Message>) -> Self {
        card.into_element()
    }
}

/// A card header with typed title, description, and top-right action slots.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct CardHeader<'a, Message> {
    pub(super) theme: &'a Theme,
    pub(super) title: Option<CardTitle<'a, Message>>,
    pub(super) description: Option<CardDescription<'a, Message>>,
    pub(super) title_element: Option<Element<'a, Message>>,
    pub(super) description_element: Option<Element<'a, Message>>,
    pub(super) children: Vec<Element<'a, Message>>,
    pub(super) action: Option<Element<'a, Message>>,
    pub(super) spacing: Option<f32>,
    pub(super) border: CardBorder,
    pub(super) radius: CardRadius,
    pub(super) style_override: Option<Box<dyn Fn(container::Style) -> container::Style + 'a>>,
}

impl<Message> fmt::Debug for CardHeader<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CardHeader")
            .field("theme", &self.theme)
            .field("title", &self.title.is_some())
            .field("description", &self.description.is_some())
            .field("title_element", &self.title_element.is_some())
            .field("description_element", &self.description_element.is_some())
            .field("children", &self.children.len())
            .field("action", &self.action.is_some())
            .field("spacing", &self.spacing)
            .field("border", &self.border)
            .field("radius", &self.radius)
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

impl<'a, Message> CardHeader<'a, Message> {
    /// Creates an empty header.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            title: None,
            description: None,
            title_element: None,
            description_element: None,
            children: Vec::new(),
            action: None,
            spacing: None,
            border: CardBorder::Theme,
            radius: CardRadius::Theme,
            style_override: None,
        }
    }

    /// Sets the style-pack-aware title.
    pub fn title(mut self, title: CardTitle<'a, Message>) -> Self {
        self.title = Some(title);
        self.title_element = None;
        self
    }

    /// Sets an arbitrary title element. Use [`Self::title`] for automatic
    /// style-pack typography and small-card title sizing.
    pub fn title_element(mut self, title: impl Into<Element<'a, Message>>) -> Self {
        self.title = None;
        self.title_element = Some(title.into());
        self
    }

    /// Sets the style-pack-aware description.
    pub fn description(mut self, description: CardDescription<'a, Message>) -> Self {
        self.description = Some(description);
        self.description_element = None;
        self
    }

    /// Sets an arbitrary description element.
    pub fn description_element(mut self, description: impl Into<Element<'a, Message>>) -> Self {
        self.description = None;
        self.description_element = Some(description.into());
        self
    }

    /// Appends arbitrary content to the header's main column.
    pub fn push(mut self, child: impl Into<Element<'a, Message>>) -> Self {
        self.children.push(child.into());
        self
    }

    /// Sets the action aligned to the header's top-right corner.
    pub fn action(mut self, action: impl Into<Element<'a, Message>>) -> Self {
        self.action = Some(action.into());
        self
    }

    /// Sets the gap between header content rows in pixels.
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = Some(normalize_px(spacing));
        self
    }

    /// Sets an explicit header bottom border.
    pub fn border_bottom(mut self) -> Self {
        self.border = CardBorder::Present;
        self
    }

    /// Removes the header bottom border, including a style-pack default.
    pub fn without_border(mut self) -> Self {
        self.border = CardBorder::None;
        self
    }

    /// Sets the header corner-radius treatment.
    pub fn radius(mut self, radius: CardRadius) -> Self {
        self.radius = radius;
        self
    }

    /// Applies an iced container-style override to the header wrapper.
    pub fn style_override(
        mut self,
        style_override: impl Fn(container::Style) -> container::Style + 'a,
    ) -> Self {
        self.style_override = Some(Box::new(style_override));
        self
    }

    /// Builds the header using the active style-pack spacing.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        let spacing = geometry::default_spacing(self.theme);
        render::build_header(self, spacing, CardSize::Default, CardRadius::Theme)
    }
}

impl<'a, Message> From<CardHeader<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(header: CardHeader<'a, Message>) -> Self {
        header.into_element()
    }
}

/// A content section with automatic horizontal card insets.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct CardContent<'a, Message> {
    pub(super) theme: &'a Theme,
    pub(super) children: Vec<Element<'a, Message>>,
    pub(super) spacing: Option<f32>,
    pub(super) style_override: Option<Box<dyn Fn(container::Style) -> container::Style + 'a>>,
}

impl<Message> fmt::Debug for CardContent<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CardContent")
            .field("theme", &self.theme)
            .field("children", &self.children.len())
            .field("spacing", &self.spacing)
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

impl<'a, Message> CardContent<'a, Message> {
    /// Creates an empty content section.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            children: Vec::new(),
            spacing: None,
            style_override: None,
        }
    }

    /// Creates a content section containing one arbitrary element.
    pub fn with_content(content: impl Into<Element<'a, Message>>, theme: &'a Theme) -> Self {
        Self::new(theme).push(content)
    }

    /// Creates a content section from arbitrary elements.
    pub fn with_children(
        theme: &'a Theme,
        children: impl IntoIterator<Item = Element<'a, Message>>,
    ) -> Self {
        Self {
            theme,
            children: children.into_iter().collect(),
            spacing: None,
            style_override: None,
        }
    }

    /// Appends arbitrary content.
    pub fn push(mut self, child: impl Into<Element<'a, Message>>) -> Self {
        self.children.push(child.into());
        self
    }

    /// Appends every element from an iterator.
    pub fn extend(self, children: impl IntoIterator<Item = Element<'a, Message>>) -> Self {
        children.into_iter().fold(self, Self::push)
    }

    /// Sets the gap between multiple content children.
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = Some(normalize_px(spacing));
        self
    }

    /// Applies an iced container-style override to the content wrapper.
    pub fn style_override(
        mut self,
        style_override: impl Fn(container::Style) -> container::Style + 'a,
    ) -> Self {
        self.style_override = Some(Box::new(style_override));
        self
    }

    /// Builds the content section using the active style-pack spacing.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        let spacing = geometry::default_spacing(self.theme);
        render::build_content(self, spacing)
    }
}

impl<'a, Message> From<CardContent<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(content: CardContent<'a, Message>) -> Self {
        content.into_element()
    }
}

/// A card footer with row/column layout and optional top border.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct CardFooter<'a, Message> {
    pub(super) theme: &'a Theme,
    pub(super) children: Vec<Element<'a, Message>>,
    pub(super) direction: CardFooterDirection,
    pub(super) alignment: CardFooterAlignment,
    pub(super) spacing: Option<f32>,
    pub(super) border: CardBorder,
    pub(super) background: Option<Color>,
    pub(super) style_override: Option<Box<dyn Fn(container::Style) -> container::Style + 'a>>,
}

impl<Message> fmt::Debug for CardFooter<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CardFooter")
            .field("theme", &self.theme)
            .field("children", &self.children.len())
            .field("direction", &self.direction)
            .field("alignment", &self.alignment)
            .field("spacing", &self.spacing)
            .field("border", &self.border)
            .field("background", &self.background)
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

impl<'a, Message> CardFooter<'a, Message> {
    /// Creates an empty horizontal footer.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            children: Vec::new(),
            direction: CardFooterDirection::Row,
            alignment: CardFooterAlignment::Start,
            spacing: None,
            border: CardBorder::Theme,
            background: None,
            style_override: None,
        }
    }

    /// Appends arbitrary footer content.
    pub fn push(mut self, child: impl Into<Element<'a, Message>>) -> Self {
        self.children.push(child.into());
        self
    }

    /// Appends every element from an iterator.
    pub fn extend(self, children: impl IntoIterator<Item = Element<'a, Message>>) -> Self {
        children.into_iter().fold(self, Self::push)
    }

    /// Sets the footer layout direction.
    pub fn direction(mut self, direction: CardFooterDirection) -> Self {
        self.direction = direction;
        self
    }

    /// Sets horizontal child alignment.
    pub fn align(mut self, alignment: CardFooterAlignment) -> Self {
        self.alignment = alignment;
        self
    }

    /// Packs row children at the trailing edge.
    pub fn justify_end(mut self) -> Self {
        self.alignment = CardFooterAlignment::End;
        self
    }

    /// Centers footer children.
    pub fn justify_center(mut self) -> Self {
        self.alignment = CardFooterAlignment::Center;
        self
    }

    /// Distributes row children across the available width.
    pub fn space_between(mut self) -> Self {
        self.alignment = CardFooterAlignment::SpaceBetween;
        self
    }

    /// Uses a vertical footer layout.
    pub fn column(mut self) -> Self {
        self.direction = CardFooterDirection::Column;
        self
    }

    /// Uses a horizontal footer layout.
    pub fn row(mut self) -> Self {
        self.direction = CardFooterDirection::Row;
        self
    }

    /// Sets the gap between footer children.
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = Some(normalize_px(spacing));
        self
    }

    /// Sets an explicit footer top border.
    pub fn border_top(mut self) -> Self {
        self.border = CardBorder::Present;
        self
    }

    /// Removes the footer top border, including a style-pack default.
    pub fn without_border(mut self) -> Self {
        self.border = CardBorder::None;
        self
    }

    /// Sets the footer background color.
    pub fn background(mut self, color: Color) -> Self {
        self.background = Some(color);
        self
    }

    /// Makes the footer background transparent.
    pub fn transparent(mut self) -> Self {
        self.background = Some(Color::TRANSPARENT);
        self
    }

    /// Applies an iced container-style override to the footer wrapper.
    pub fn style_override(
        mut self,
        style_override: impl Fn(container::Style) -> container::Style + 'a,
    ) -> Self {
        self.style_override = Some(Box::new(style_override));
        self
    }

    /// Builds the footer using the active style-pack spacing.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        let spacing = geometry::default_spacing(self.theme);
        render::build_footer(self, spacing, CardRadius::Theme)
    }
}

impl<'a, Message> From<CardFooter<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(footer: CardFooter<'a, Message>) -> Self {
        footer.into_element()
    }
}

/// A header action wrapper for an arbitrary element.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct CardAction<'a, Message> {
    pub(super) content: Element<'a, Message>,
    pub(super) width: Length,
    pub(super) height: Length,
    pub(super) style_override: Option<Box<dyn Fn(container::Style) -> container::Style + 'a>>,
}

impl<Message> fmt::Debug for CardAction<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CardAction")
            .field("width", &self.width)
            .field("height", &self.height)
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

impl<'a, Message> CardAction<'a, Message> {
    /// Creates an action from arbitrary content.
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

    /// Applies an iced container-style override.
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

impl<'a, Message> From<CardAction<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(action: CardAction<'a, Message>) -> Self {
        action.into_element()
    }
}

/// Styled card title content.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct CardTitle<'a, Message> {
    pub(super) content: CardTextContent<'a, Message>,
    pub(super) theme: &'a Theme,
    pub(super) text_size: Option<f32>,
    pub(super) line_height: Option<f32>,
    pub(super) color: Option<Color>,
    pub(super) font: Option<Font>,
    pub(super) width: Length,
    pub(super) style_override: Option<Box<dyn Fn(container::Style) -> container::Style + 'a>>,
}

impl<Message> fmt::Debug for CardTitle<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CardTitle")
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

impl<'a, Message> CardTitle<'a, Message> {
    /// Creates a title from arbitrary content.
    pub fn new(content: impl Into<Element<'a, Message>>, theme: &'a Theme) -> Self {
        Self::from_content(CardTextContent::Element(content.into()), theme)
    }

    /// Creates a style-pack-aware text title.
    pub fn text(label: impl IntoFragment<'a>, theme: &'a Theme) -> Self {
        Self::from_content(CardTextContent::Label(label.into_fragment()), theme)
    }

    fn from_content(content: CardTextContent<'a, Message>, theme: &'a Theme) -> Self {
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
        self.text_size = Some(normalize_min_px(text_size));
        self
    }

    /// Sets the title line height in pixels.
    pub fn line_height(mut self, line_height: f32) -> Self {
        self.line_height = Some(normalize_min_px(line_height));
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

    /// Applies an iced container-style override.
    pub fn style_override(
        mut self,
        style_override: impl Fn(container::Style) -> container::Style + 'a,
    ) -> Self {
        self.style_override = Some(Box::new(style_override));
        self
    }

    /// Builds the title with default-card typography.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        render::build_title(self, CardSize::Default)
    }
}

impl<'a, Message> From<CardTitle<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(title: CardTitle<'a, Message>) -> Self {
        title.into_element()
    }
}

/// Styled card description content.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct CardDescription<'a, Message> {
    pub(super) content: CardTextContent<'a, Message>,
    pub(super) theme: &'a Theme,
    pub(super) text_size: Option<f32>,
    pub(super) line_height: Option<f32>,
    pub(super) color: Option<Color>,
    pub(super) font: Option<Font>,
    pub(super) width: Length,
    pub(super) style_override: Option<Box<dyn Fn(container::Style) -> container::Style + 'a>>,
}

impl<Message> fmt::Debug for CardDescription<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CardDescription")
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

impl<'a, Message> CardDescription<'a, Message> {
    /// Creates a description from arbitrary content.
    pub fn new(content: impl Into<Element<'a, Message>>, theme: &'a Theme) -> Self {
        Self::from_content(CardTextContent::Element(content.into()), theme)
    }

    /// Creates a style-pack-aware text description.
    pub fn text(label: impl IntoFragment<'a>, theme: &'a Theme) -> Self {
        Self::from_content(CardTextContent::Label(label.into_fragment()), theme)
    }

    fn from_content(content: CardTextContent<'a, Message>, theme: &'a Theme) -> Self {
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
        self.text_size = Some(normalize_min_px(text_size));
        self
    }

    /// Sets the description line height in pixels.
    pub fn line_height(mut self, line_height: f32) -> Self {
        self.line_height = Some(normalize_min_px(line_height));
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

    /// Applies an iced container-style override.
    pub fn style_override(
        mut self,
        style_override: impl Fn(container::Style) -> container::Style + 'a,
    ) -> Self {
        self.style_override = Some(Box::new(style_override));
        self
    }

    /// Builds the description with default style-pack typography.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        render::build_description(self)
    }
}

impl<'a, Message> From<CardDescription<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(description: CardDescription<'a, Message>) -> Self {
        description.into_element()
    }
}

pub(super) enum CardTextContent<'a, Message> {
    Label(Fragment<'a>),
    Element(Element<'a, Message>),
}

impl<Message> CardTextContent<'_, Message> {
    fn kind(&self) -> &'static str {
        match self {
            Self::Label(_) => "label",
            Self::Element(_) => "element",
        }
    }
}

fn normalize_px(value: f32) -> f32 {
    if value.is_finite() {
        value.max(0.0)
    } else {
        0.0
    }
}

fn normalize_min_px(value: f32) -> f32 {
    if value.is_finite() {
        value.max(1.0)
    } else {
        1.0
    }
}
