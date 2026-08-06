//! Builder-first empty-state component.
//!
//! This is the iced composition counterpart of shadcn-svelte's
//! `Empty.Root`, `Empty.Header`, `Empty.Media`, `Empty.Title`,
//! `Empty.Description`, and `Empty.Content` family. Every slot accepts
//! arbitrary iced elements; typed title and description helpers retain the
//! active style-pack typography.
//!
//! ```rust,no_run
//! use iced::{Element, Length};
//! use iced_shadcn_v2::{
//!     Button, ButtonVariant, Empty, EmptyContent, EmptyDescription, EmptyHeader,
//!     EmptyMedia, EmptyTitle, Theme,
//! };
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     CreateProject,
//! }
//!
//! fn view(theme: &Theme) -> Element<'_, Message> {
//!     Empty::new(theme)
//!         .width(Length::Fill)
//!         .header(
//!             EmptyHeader::new(theme)
//!                 .media(EmptyMedia::icon(iced::widget::text("□"), theme))
//!                 .title(EmptyTitle::text("No projects yet", theme))
//!                 .description(EmptyDescription::text(
//!                     "Create your first project to get started.",
//!                     theme,
//!                 )),
//!         )
//!         .content(
//!             EmptyContent::new(theme).push(
//!                 Button::text("Create project", theme)
//!                     .variant(ButtonVariant::Default)
//!                     .on_press(Message::CreateProject),
//!             ),
//!         )
//!         .into()
//! }
//! ```

mod geometry;
mod render;
mod style;
mod types;

#[cfg(test)]
mod tests;

pub use types::{EmptyBorderStyle, EmptyMediaVariant, EmptyRadius};

use std::fmt;

use crate::iced_compat::alignment::{Horizontal, Vertical};
use crate::iced_compat::widget::{container, text::Fragment, text::IntoFragment};
use crate::iced_compat::{Background, Color, Element, Font, Length, Padding};

use crate::theme::Theme;

/// A style-pack-aware empty-state root.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct Empty<'a, Message> {
    pub(super) theme: &'a Theme,
    pub(super) width: Length,
    pub(super) height: Length,
    pub(super) padding: Option<Padding>,
    pub(super) spacing: Option<f32>,
    pub(super) radius: EmptyRadius,
    pub(super) border: EmptyBorderStyle,
    pub(super) border_width: f32,
    pub(super) border_color: Color,
    pub(super) background: Option<Background>,
    pub(super) align_x: Horizontal,
    pub(super) align_y: Vertical,
    pub(super) children: Vec<EmptyChild<'a, Message>>,
    pub(super) style_override: Option<Box<dyn Fn(container::Style) -> container::Style + 'a>>,
}

pub(super) enum EmptyChild<'a, Message> {
    Element(Element<'a, Message>),
    Header(Box<EmptyHeader<'a, Message>>),
    Content(Box<EmptyContent<'a, Message>>),
}

impl<Message> fmt::Debug for Empty<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Empty")
            .field("theme", &self.theme)
            .field("width", &self.width)
            .field("height", &self.height)
            .field("padding", &self.padding)
            .field("spacing", &self.spacing)
            .field("radius", &self.radius)
            .field("border", &self.border)
            .field("border_width", &self.border_width)
            .field("border_color", &self.border_color)
            .field("background", &self.background)
            .field("align_x", &self.align_x)
            .field("align_y", &self.align_y)
            .field("children", &self.children.len())
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

impl<'a, Message> Empty<'a, Message> {
    /// Creates an empty state using the active style-pack defaults.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            width: Length::Fill,
            height: Length::Shrink,
            padding: None,
            spacing: None,
            radius: EmptyRadius::Theme,
            border: EmptyBorderStyle::None,
            border_width: style::DEFAULT_BORDER_WIDTH_PX,
            border_color: theme.palette.border,
            background: None,
            align_x: Horizontal::Center,
            align_y: Vertical::Center,
            children: Vec::new(),
            style_override: None,
        }
    }

    /// Creates an empty state with arbitrary root children.
    pub fn with_children(
        theme: &'a Theme,
        children: impl IntoIterator<Item = Element<'a, Message>>,
    ) -> Self {
        Self {
            children: children.into_iter().map(EmptyChild::Element).collect(),
            ..Self::new(theme)
        }
    }

    /// Appends arbitrary content to the root column.
    pub fn push(mut self, child: impl Into<Element<'a, Message>>) -> Self {
        self.children.push(EmptyChild::Element(child.into()));
        self
    }

    /// Appends every element from an iterator to the root column.
    pub fn extend(self, children: impl IntoIterator<Item = Element<'a, Message>>) -> Self {
        children.into_iter().fold(self, Self::push)
    }

    /// Adds a typed header slot.
    pub fn header(mut self, header: EmptyHeader<'a, Message>) -> Self {
        self.children.push(EmptyChild::Header(Box::new(header)));
        self
    }

    /// Adds a typed content slot.
    pub fn content(mut self, content: EmptyContent<'a, Message>) -> Self {
        self.children.push(EmptyChild::Content(Box::new(content)));
        self
    }

    /// Sets the root width.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the root height.
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Sets the root padding.
    ///
    /// Non-finite and negative sides are normalized to zero before layout.
    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = Some(normalize_padding(padding.into()));
        self
    }

    /// Sets the gap between root children in pixels.
    ///
    /// Non-finite and negative values are normalized to zero.
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = Some(normalize_px(spacing));
        self
    }

    /// Sets the root corner-radius treatment.
    pub fn radius(mut self, radius: EmptyRadius) -> Self {
        self.radius = radius;
        self
    }

    /// Sets the root border treatment.
    pub fn border(mut self, border: EmptyBorderStyle) -> Self {
        self.border = border;
        self
    }

    /// Enables the source component's visible dashed outline.
    pub fn outline(mut self) -> Self {
        self.border = EmptyBorderStyle::Dashed;
        self
    }

    /// Removes the root border.
    pub fn without_border(mut self) -> Self {
        self.border = EmptyBorderStyle::None;
        self
    }

    /// Sets the root border color.
    pub fn border_color(mut self, color: Color) -> Self {
        self.border_color = color;
        self
    }

    /// Sets the root border width in pixels.
    ///
    /// Non-finite and negative values are normalized to zero.
    pub fn border_width(mut self, width: f32) -> Self {
        self.border_width = normalize_px(width);
        self
    }

    /// Sets the root background, including solid colors and iced gradients.
    pub fn background(mut self, background: impl Into<Background>) -> Self {
        self.background = Some(background.into());
        self
    }

    /// Makes the root background transparent.
    pub fn transparent(mut self) -> Self {
        self.background = Some(Background::Color(Color::TRANSPARENT));
        self
    }

    /// Sets the horizontal alignment used by root children.
    pub fn align_x(mut self, alignment: impl Into<Horizontal>) -> Self {
        self.align_x = alignment.into();
        self
    }

    /// Sets the vertical alignment used by the root container.
    pub fn align_y(mut self, alignment: impl Into<Vertical>) -> Self {
        self.align_y = alignment.into();
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

    /// Builds the root as an iced [`Element`](iced_core::Element).
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        render::build_root(self)
    }
}

impl<'a, Message> From<Empty<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(empty: Empty<'a, Message>) -> Self {
        empty.into_element()
    }
}

/// A centered header section containing media, title, description, and custom
/// children.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct EmptyHeader<'a, Message> {
    pub(super) theme: &'a Theme,
    pub(super) media: Option<EmptyMedia<'a, Message>>,
    pub(super) title: Option<EmptyTitle<'a, Message>>,
    pub(super) description: Option<EmptyDescription<'a, Message>>,
    pub(super) title_element: Option<Element<'a, Message>>,
    pub(super) description_element: Option<Element<'a, Message>>,
    pub(super) children: Vec<Element<'a, Message>>,
    pub(super) width: Length,
    pub(super) max_width: f32,
    pub(super) spacing: Option<f32>,
    pub(super) style_override: Option<Box<dyn Fn(container::Style) -> container::Style + 'a>>,
}

impl<Message> fmt::Debug for EmptyHeader<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("EmptyHeader")
            .field("theme", &self.theme)
            .field("media", &self.media.is_some())
            .field("title", &self.title.is_some())
            .field("description", &self.description.is_some())
            .field("title_element", &self.title_element.is_some())
            .field("description_element", &self.description_element.is_some())
            .field("children", &self.children.len())
            .field("width", &self.width)
            .field("max_width", &self.max_width)
            .field("spacing", &self.spacing)
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

impl<'a, Message> EmptyHeader<'a, Message> {
    /// Creates an empty header using the active style-pack defaults.
    pub fn new(theme: &'a Theme) -> Self {
        let metrics = geometry::metrics(theme);
        Self {
            theme,
            media: None,
            title: None,
            description: None,
            title_element: None,
            description_element: None,
            children: Vec::new(),
            width: Length::Fill,
            max_width: metrics.section_max_width_px,
            spacing: None,
            style_override: None,
        }
    }

    /// Creates a header from arbitrary children.
    pub fn with_children(
        theme: &'a Theme,
        children: impl IntoIterator<Item = Element<'a, Message>>,
    ) -> Self {
        Self {
            children: children.into_iter().collect(),
            ..Self::new(theme)
        }
    }

    /// Sets the media slot.
    pub fn media(mut self, media: EmptyMedia<'a, Message>) -> Self {
        self.media = Some(media);
        self
    }

    /// Sets the style-pack-aware title slot.
    pub fn title(mut self, title: EmptyTitle<'a, Message>) -> Self {
        self.title = Some(title);
        self.title_element = None;
        self
    }

    /// Sets an arbitrary title element.
    pub fn title_element(mut self, title: impl Into<Element<'a, Message>>) -> Self {
        self.title = None;
        self.title_element = Some(title.into());
        self
    }

    /// Sets the style-pack-aware description slot.
    pub fn description(mut self, description: EmptyDescription<'a, Message>) -> Self {
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

    /// Appends arbitrary content after the typed header slots.
    pub fn push(mut self, child: impl Into<Element<'a, Message>>) -> Self {
        self.children.push(child.into());
        self
    }

    /// Appends every element from an iterator.
    pub fn extend(self, children: impl IntoIterator<Item = Element<'a, Message>>) -> Self {
        children.into_iter().fold(self, Self::push)
    }

    /// Sets the header width.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the header maximum width in pixels.
    ///
    /// Non-finite and negative values are normalized to zero.
    pub fn max_width(mut self, width: f32) -> Self {
        self.max_width = normalize_px(width);
        self
    }

    /// Sets the gap between header children in pixels.
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = Some(normalize_px(spacing));
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

    /// Builds the header as an iced element.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        render::build_header(self)
    }
}

impl<'a, Message> From<EmptyHeader<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(header: EmptyHeader<'a, Message>) -> Self {
        header.into_element()
    }
}

/// The media slot for an empty state.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct EmptyMedia<'a, Message> {
    pub(super) theme: &'a Theme,
    pub(super) variant: EmptyMediaVariant,
    pub(super) children: Vec<Element<'a, Message>>,
    pub(super) width: Length,
    pub(super) height: Length,
    pub(super) size: Option<f32>,
    pub(super) radius: Option<EmptyRadius>,
    pub(super) spacing: Option<f32>,
    pub(super) style_override: Option<Box<dyn Fn(container::Style) -> container::Style + 'a>>,
}

impl<Message> fmt::Debug for EmptyMedia<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("EmptyMedia")
            .field("theme", &self.theme)
            .field("variant", &self.variant)
            .field("children", &self.children.len())
            .field("width", &self.width)
            .field("height", &self.height)
            .field("size", &self.size)
            .field("radius", &self.radius)
            .field("spacing", &self.spacing)
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

impl<'a, Message> EmptyMedia<'a, Message> {
    /// Creates an unboxed media slot.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            variant: EmptyMediaVariant::Default,
            children: Vec::new(),
            width: Length::Shrink,
            height: Length::Shrink,
            size: None,
            radius: None,
            spacing: None,
            style_override: None,
        }
    }

    /// Creates a media slot containing one arbitrary element.
    pub fn with_content(content: impl Into<Element<'a, Message>>, theme: &'a Theme) -> Self {
        Self::new(theme).push(content)
    }

    /// Creates an icon media tile containing one arbitrary element.
    pub fn icon(content: impl Into<Element<'a, Message>>, theme: &'a Theme) -> Self {
        Self::with_content(content, theme).variant(EmptyMediaVariant::Icon)
    }

    /// Sets the media visual variant.
    pub fn variant(mut self, variant: EmptyMediaVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Sets both media dimensions in pixels.
    ///
    /// Non-finite and non-positive values resolve to a one-pixel footprint.
    pub fn size(mut self, size: f32) -> Self {
        self.size = Some(normalize_min_px(size));
        self
    }

    /// Sets the media width.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the media height.
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Sets the icon tile corner-radius treatment.
    pub fn radius(mut self, radius: EmptyRadius) -> Self {
        self.radius = Some(radius);
        self
    }

    /// Sets the gap between multiple media children in pixels.
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = Some(normalize_px(spacing));
        self
    }

    /// Appends arbitrary media content.
    pub fn push(mut self, child: impl Into<Element<'a, Message>>) -> Self {
        self.children.push(child.into());
        self
    }

    /// Appends every element from an iterator.
    pub fn extend(self, children: impl IntoIterator<Item = Element<'a, Message>>) -> Self {
        children.into_iter().fold(self, Self::push)
    }

    /// Applies an iced container-style override to the media wrapper.
    pub fn style_override(
        mut self,
        style_override: impl Fn(container::Style) -> container::Style + 'a,
    ) -> Self {
        self.style_override = Some(Box::new(style_override));
        self
    }

    /// Builds the media slot as an iced element.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        render::build_media(self)
    }
}

impl<'a, Message> From<EmptyMedia<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(media: EmptyMedia<'a, Message>) -> Self {
        media.into_element()
    }
}

/// A centered content section for actions, links, and custom elements.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct EmptyContent<'a, Message> {
    pub(super) theme: &'a Theme,
    pub(super) children: Vec<Element<'a, Message>>,
    pub(super) width: Length,
    pub(super) max_width: f32,
    pub(super) spacing: Option<f32>,
    pub(super) style_override: Option<Box<dyn Fn(container::Style) -> container::Style + 'a>>,
}

impl<Message> fmt::Debug for EmptyContent<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("EmptyContent")
            .field("theme", &self.theme)
            .field("children", &self.children.len())
            .field("width", &self.width)
            .field("max_width", &self.max_width)
            .field("spacing", &self.spacing)
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

impl<'a, Message> EmptyContent<'a, Message> {
    /// Creates an empty content section using the active style-pack defaults.
    pub fn new(theme: &'a Theme) -> Self {
        let metrics = geometry::metrics(theme);
        Self {
            theme,
            children: Vec::new(),
            width: Length::Fill,
            max_width: metrics.section_max_width_px,
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
            children: children.into_iter().collect(),
            ..Self::new(theme)
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

    /// Sets the content width.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the content maximum width in pixels.
    pub fn max_width(mut self, width: f32) -> Self {
        self.max_width = normalize_px(width);
        self
    }

    /// Sets the gap between content children in pixels.
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

    /// Builds the content section as an iced element.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        render::build_content(self)
    }
}

impl<'a, Message> From<EmptyContent<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(content: EmptyContent<'a, Message>) -> Self {
        content.into_element()
    }
}

/// Typed or arbitrary title content.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct EmptyTitle<'a, Message> {
    pub(super) content: EmptyTextContent<'a, Message>,
    pub(super) theme: &'a Theme,
    pub(super) text_size: Option<f32>,
    pub(super) line_height: Option<f32>,
    pub(super) color: Option<Color>,
    pub(super) font: Option<Font>,
    pub(super) width: Length,
    pub(super) align_x: Horizontal,
    pub(super) style_override: Option<Box<dyn Fn(container::Style) -> container::Style + 'a>>,
}

impl<Message> fmt::Debug for EmptyTitle<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("EmptyTitle")
            .field("content", &self.content.kind())
            .field("theme", &self.theme)
            .field("text_size", &self.text_size)
            .field("line_height", &self.line_height)
            .field("color", &self.color)
            .field("font", &self.font)
            .field("width", &self.width)
            .field("align_x", &self.align_x)
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

impl<'a, Message> EmptyTitle<'a, Message> {
    /// Creates a title from arbitrary content.
    pub fn new(content: impl Into<Element<'a, Message>>, theme: &'a Theme) -> Self {
        Self::from_content(EmptyTextContent::Element(content.into()), theme)
    }

    /// Creates a style-pack-aware text title.
    pub fn text(label: impl IntoFragment<'a>, theme: &'a Theme) -> Self {
        Self::from_content(EmptyTextContent::Label(label.into_fragment()), theme)
    }

    fn from_content(content: EmptyTextContent<'a, Message>, theme: &'a Theme) -> Self {
        Self {
            content,
            theme,
            text_size: None,
            line_height: None,
            color: None,
            font: None,
            width: Length::Fill,
            align_x: Horizontal::Center,
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

    /// Sets the title's horizontal text alignment.
    pub fn align_x(mut self, alignment: impl Into<Horizontal>) -> Self {
        self.align_x = alignment.into();
        self
    }

    /// Applies an iced container-style override to the title wrapper.
    pub fn style_override(
        mut self,
        style_override: impl Fn(container::Style) -> container::Style + 'a,
    ) -> Self {
        self.style_override = Some(Box::new(style_override));
        self
    }

    /// Builds the title using the active style-pack typography.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        render::build_title(self)
    }
}

impl<'a, Message> From<EmptyTitle<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(title: EmptyTitle<'a, Message>) -> Self {
        title.into_element()
    }
}

/// Typed or arbitrary description content.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct EmptyDescription<'a, Message> {
    pub(super) content: EmptyTextContent<'a, Message>,
    pub(super) theme: &'a Theme,
    pub(super) text_size: Option<f32>,
    pub(super) line_height: Option<f32>,
    pub(super) color: Option<Color>,
    pub(super) font: Option<Font>,
    pub(super) width: Length,
    pub(super) align_x: Horizontal,
    pub(super) style_override: Option<Box<dyn Fn(container::Style) -> container::Style + 'a>>,
}

impl<Message> fmt::Debug for EmptyDescription<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("EmptyDescription")
            .field("content", &self.content.kind())
            .field("theme", &self.theme)
            .field("text_size", &self.text_size)
            .field("line_height", &self.line_height)
            .field("color", &self.color)
            .field("font", &self.font)
            .field("width", &self.width)
            .field("align_x", &self.align_x)
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

impl<'a, Message> EmptyDescription<'a, Message> {
    /// Creates a description from arbitrary content.
    pub fn new(content: impl Into<Element<'a, Message>>, theme: &'a Theme) -> Self {
        Self::from_content(EmptyTextContent::Element(content.into()), theme)
    }

    /// Creates a style-pack-aware text description.
    pub fn text(label: impl IntoFragment<'a>, theme: &'a Theme) -> Self {
        Self::from_content(EmptyTextContent::Label(label.into_fragment()), theme)
    }

    fn from_content(content: EmptyTextContent<'a, Message>, theme: &'a Theme) -> Self {
        Self {
            content,
            theme,
            text_size: None,
            line_height: None,
            color: None,
            font: None,
            width: Length::Fill,
            align_x: Horizontal::Center,
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

    /// Sets the description's horizontal text alignment.
    pub fn align_x(mut self, alignment: impl Into<Horizontal>) -> Self {
        self.align_x = alignment.into();
        self
    }

    /// Applies an iced container-style override to the description wrapper.
    pub fn style_override(
        mut self,
        style_override: impl Fn(container::Style) -> container::Style + 'a,
    ) -> Self {
        self.style_override = Some(Box::new(style_override));
        self
    }

    /// Builds the description using the active style-pack typography.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        render::build_description(self)
    }
}

impl<'a, Message> From<EmptyDescription<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(description: EmptyDescription<'a, Message>) -> Self {
        description.into_element()
    }
}

pub(super) enum EmptyTextContent<'a, Message> {
    Label(Fragment<'a>),
    Element(Element<'a, Message>),
}

impl<Message> EmptyTextContent<'_, Message> {
    pub(super) fn kind(&self) -> &'static str {
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

fn normalize_padding(padding: Padding) -> Padding {
    Padding {
        top: normalize_px(padding.top),
        right: normalize_px(padding.right),
        bottom: normalize_px(padding.bottom),
        left: normalize_px(padding.left),
    }
}
