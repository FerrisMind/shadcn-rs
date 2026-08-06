//! Builder-first avatar primitives.
//!
//! This is the iced composition counterpart of shadcn-svelte's
//! `Avatar.Root`, `Avatar.Image`, `Avatar.Fallback`, `Avatar.Badge`,
//! `Avatar.Group`, and `Avatar.GroupCount`. Images use iced's native
//! [`iced::widget::image::Handle`](iced_widget::image::Handle) sources (paths, encoded bytes, or decoded
//! RGBA pixels). A fallback is rendered underneath the image, so it remains
//! visible when the renderer cannot decode the image handle.
//!
//! ```rust,no_run
//! use iced::{Element, widget::text};
//! use iced_shadcn_v2::{Avatar, AvatarBadge, AvatarFallback, AvatarSize, Theme};
//!
//! #[derive(Debug, Clone)]
//! enum Message {}
//!
//! fn view(theme: &Theme) -> Element<'_, Message> {
//!     Avatar::new(theme)
//!         .size(AvatarSize::Lg)
//!         .fallback(AvatarFallback::text("CN", theme))
//!         .badge(AvatarBadge::icon(text("+"), theme))
//!         .into()
//! }
//! ```

mod geometry;
mod render;
mod style;
mod types;

#[cfg(test)]
mod tests;

pub use types::{AvatarRadius, AvatarSize};

use std::fmt;
use std::path::PathBuf;

use crate::iced_compat::widget::container;
use crate::iced_compat::widget::image as image_widget;
use crate::iced_compat::widget::text::{Fragment, IntoFragment};
use crate::iced_compat::{Color, ContentFit, Element, Font, Length};

use crate::theme::Theme;

/// A source image for an [`Avatar`].
///
/// The handle is intentionally owned so callers can use a path, encoded image
/// bytes, or already-decoded RGBA pixels without an additional wrapper type.
#[derive(Clone, Debug)]
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct AvatarImage {
    handle: image_widget::Handle,
    content_fit: ContentFit,
    filter_method: image_widget::FilterMethod,
    opacity: f32,
    scale: f32,
}

impl AvatarImage {
    /// Creates an image source from an iced image handle.
    pub fn new(handle: impl Into<image_widget::Handle>) -> Self {
        Self {
            handle: handle.into(),
            content_fit: ContentFit::Cover,
            filter_method: image_widget::FilterMethod::Linear,
            opacity: 1.0,
            scale: 1.0,
        }
    }

    /// Creates an image source whose bytes are read by iced from `path`.
    pub fn from_path(path: impl Into<PathBuf>) -> Self {
        Self::new(image_widget::Handle::from_path(path))
    }

    /// Creates an image source from encoded image bytes.
    pub fn from_bytes(bytes: impl AsRef<[u8]>) -> Self {
        Self::new(image_widget::Handle::from_bytes(bytes.as_ref().to_vec()))
    }

    /// Creates an image source from decoded RGBA pixels.
    pub fn from_rgba(width: u32, height: u32, pixels: impl AsRef<[u8]>) -> Self {
        Self::new(image_widget::Handle::from_rgba(
            width,
            height,
            pixels.as_ref().to_vec(),
        ))
    }

    /// Returns the underlying iced handle.
    pub fn handle(&self) -> &image_widget::Handle {
        &self.handle
    }

    /// Consumes the source and returns the underlying iced handle.
    pub fn into_handle(self) -> image_widget::Handle {
        self.handle
    }

    /// Sets how the source image fits the square avatar bounds.
    ///
    /// The default is [`ContentFit::Cover`], matching the source component's
    /// `object-cover` behavior.
    pub fn content_fit(mut self, content_fit: ContentFit) -> Self {
        self.content_fit = content_fit;
        self
    }

    /// Sets the raster filtering strategy.
    pub fn filter_method(mut self, filter_method: image_widget::FilterMethod) -> Self {
        self.filter_method = filter_method;
        self
    }

    /// Sets image opacity, clamped to `0.0..=1.0`.
    pub fn opacity(mut self, opacity: f32) -> Self {
        self.opacity = geometry::normalize_opacity(opacity);
        self
    }

    /// Sets the image scale, clamped to a non-negative finite value.
    pub fn scale(mut self, scale: f32) -> Self {
        self.scale = geometry::normalize_scale(scale);
        self
    }
}

impl From<image_widget::Handle> for AvatarImage {
    fn from(handle: image_widget::Handle) -> Self {
        Self::new(handle)
    }
}

impl From<&image_widget::Handle> for AvatarImage {
    fn from(handle: &image_widget::Handle) -> Self {
        Self::new(handle.clone())
    }
}

impl From<PathBuf> for AvatarImage {
    fn from(path: PathBuf) -> Self {
        Self::from_path(path)
    }
}

enum AvatarTextContent<'a, Message> {
    Label(Fragment<'a>),
    Element(Element<'a, Message>),
    Icon(Element<'a, Message>),
}

impl<Message> AvatarTextContent<'_, Message> {
    fn kind(&self) -> &'static str {
        match self {
            Self::Label(_) => "label",
            Self::Element(_) => "element",
            Self::Icon(_) => "icon",
        }
    }
}

enum AvatarBadgeContent<'a, Message> {
    Element(Element<'a, Message>),
    Icon(Element<'a, Message>),
}

impl<Message> AvatarBadgeContent<'_, Message> {
    fn kind(&self) -> &'static str {
        match self {
            Self::Element(_) => "element",
            Self::Icon(_) => "icon",
        }
    }
}

/// Content displayed when an [`AvatarImage`] is not available.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct AvatarFallback<'a, Message> {
    content: AvatarTextContent<'a, Message>,
    theme: &'a Theme,
    text_size: Option<f32>,
    line_height: Option<f32>,
    color: Option<Color>,
    background: Option<Color>,
    font: Option<Font>,
    style_override: Option<Box<dyn Fn(container::Style) -> container::Style + 'a>>,
}

impl<Message> fmt::Debug for AvatarFallback<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AvatarFallback")
            .field("content", &self.content.kind())
            .field("theme", &self.theme)
            .field("text_size", &self.text_size)
            .field("line_height", &self.line_height)
            .field("color", &self.color)
            .field("background", &self.background)
            .field("font", &self.font)
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

impl<'a, Message> AvatarFallback<'a, Message> {
    /// Creates a fallback from arbitrary iced content.
    pub fn new(content: impl Into<Element<'a, Message>>, theme: &'a Theme) -> Self {
        Self::from_content(AvatarTextContent::Element(content.into()), theme)
    }

    /// Creates a fallback from text using the active style-pack font.
    pub fn text(label: impl IntoFragment<'a>, theme: &'a Theme) -> Self {
        Self::from_content(AvatarTextContent::Label(label.into_fragment()), theme)
    }

    fn from_content(content: AvatarTextContent<'a, Message>, theme: &'a Theme) -> Self {
        Self {
            content,
            theme,
            text_size: None,
            line_height: None,
            color: None,
            background: None,
            font: None,
            style_override: None,
        }
    }

    /// Sets the fallback text size in pixels.
    pub fn text_size(mut self, text_size: f32) -> Self {
        self.text_size = Some(geometry::normalize_min_px(text_size));
        self
    }

    /// Sets the fallback line height in pixels.
    pub fn line_height(mut self, line_height: f32) -> Self {
        self.line_height = Some(geometry::normalize_min_px(line_height));
        self
    }

    /// Sets the fallback foreground color.
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Sets the fallback background color.
    pub fn background(mut self, background: Color) -> Self {
        self.background = Some(background);
        self
    }

    /// Sets the fallback font.
    pub fn font(mut self, font: Font) -> Self {
        self.font = Some(font);
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

    /// Builds the fallback with the default avatar footprint.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        render::build_fallback(
            self,
            AvatarSize::Default,
            Length::Fixed(32.0),
            Length::Fixed(32.0),
            9999.0,
        )
    }
}

impl<'a, Message> From<AvatarFallback<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(fallback: AvatarFallback<'a, Message>) -> Self {
        fallback.into_element()
    }
}

/// A primary-colored status dot or icon positioned at an avatar's bottom-right.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct AvatarBadge<'a, Message> {
    content: Option<AvatarBadgeContent<'a, Message>>,
    theme: &'a Theme,
    width: Option<Length>,
    height: Option<Length>,
    style_override: Option<Box<dyn Fn(container::Style) -> container::Style + 'a>>,
}

impl<Message> fmt::Debug for AvatarBadge<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AvatarBadge")
            .field(
                "content",
                &self.content.as_ref().map(AvatarBadgeContent::kind),
            )
            .field("theme", &self.theme)
            .field("width", &self.width)
            .field("height", &self.height)
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

impl<'a, Message> AvatarBadge<'a, Message> {
    /// Creates a badge with arbitrary content.
    pub fn new(content: impl Into<Element<'a, Message>>, theme: &'a Theme) -> Self {
        Self {
            content: Some(AvatarBadgeContent::Element(content.into())),
            theme,
            width: None,
            height: None,
            style_override: None,
        }
    }

    /// Creates an empty status dot.
    pub fn dot(theme: &'a Theme) -> Self {
        Self {
            content: None,
            theme,
            width: None,
            height: None,
            style_override: None,
        }
    }

    /// Creates a text badge.
    pub fn text(label: impl IntoFragment<'a>, theme: &'a Theme) -> Self {
        Self::new(crate::iced_compat::widget::text(label), theme)
    }

    /// Creates a badge containing an icon-sized element.
    ///
    /// This follows the source component's direct-`svg` selectors: the icon
    /// is hidden for [`AvatarSize::Sm`] and constrained to 8px for the other
    /// preset avatar sizes. Use [`Self::new`] when the badge should preserve
    /// arbitrary child layout instead.
    pub fn icon(content: impl Into<Element<'a, Message>>, theme: &'a Theme) -> Self {
        Self {
            content: Some(AvatarBadgeContent::Icon(content.into())),
            theme,
            width: None,
            height: None,
            style_override: None,
        }
    }

    /// Sets an explicit badge width.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = Some(width.into());
        self
    }

    /// Sets an explicit badge height.
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = Some(height.into());
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

    /// Builds the badge with the default avatar footprint.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        render::build_badge(self, AvatarSize::Default)
    }
}

impl<'a, Message> From<AvatarBadge<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(badge: AvatarBadge<'a, Message>) -> Self {
        badge.into_element()
    }
}

/// A count or action item appended to an [`AvatarGroup`].
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct AvatarGroupCount<'a, Message> {
    content: AvatarTextContent<'a, Message>,
    theme: &'a Theme,
    width: Option<Length>,
    height: Option<Length>,
    style_override: Option<Box<dyn Fn(container::Style) -> container::Style + 'a>>,
}

impl<Message> fmt::Debug for AvatarGroupCount<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AvatarGroupCount")
            .field("theme", &self.theme)
            .field("width", &self.width)
            .field("height", &self.height)
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

impl<'a, Message> AvatarGroupCount<'a, Message> {
    /// Creates a group count from arbitrary content.
    pub fn new(content: impl Into<Element<'a, Message>>, theme: &'a Theme) -> Self {
        Self {
            content: AvatarTextContent::Element(content.into()),
            theme,
            width: None,
            height: None,
            style_override: None,
        }
    }

    /// Creates a text group count.
    pub fn text(label: impl IntoFragment<'a>, theme: &'a Theme) -> Self {
        Self {
            content: AvatarTextContent::Label(label.into_fragment()),
            theme,
            width: None,
            height: None,
            style_override: None,
        }
    }

    /// Creates a group count containing an icon-sized element.
    ///
    /// The icon follows the source component's size selectors: 12px for a
    /// small group, 16px for the default group, and 20px for a large group.
    /// Use [`Self::new`] when the count should preserve arbitrary child
    /// layout instead.
    pub fn icon(content: impl Into<Element<'a, Message>>, theme: &'a Theme) -> Self {
        Self {
            content: AvatarTextContent::Icon(content.into()),
            theme,
            width: None,
            height: None,
            style_override: None,
        }
    }

    /// Sets an explicit count width.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = Some(width.into());
        self
    }

    /// Sets an explicit count height.
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = Some(height.into());
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

    /// Builds the count with the default group footprint.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        render::build_group_count(self, AvatarSize::Default)
    }
}

impl<'a, Message> From<AvatarGroupCount<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(count: AvatarGroupCount<'a, Message>) -> Self {
        count.into_element()
    }
}

/// A composable avatar root with image, fallback, and badge slots.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct Avatar<'a, Message> {
    theme: &'a Theme,
    size: AvatarSize,
    radius: AvatarRadius,
    width: Option<Length>,
    height: Option<Length>,
    image: Option<AvatarImage>,
    fallback: Option<AvatarFallback<'a, Message>>,
    badge: Option<AvatarBadge<'a, Message>>,
    style_override: Option<Box<dyn Fn(container::Style) -> container::Style + 'a>>,
}

impl<Message> fmt::Debug for Avatar<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Avatar")
            .field("theme", &self.theme)
            .field("size", &self.size)
            .field("radius", &self.radius)
            .field("width", &self.width)
            .field("height", &self.height)
            .field("image", &self.image.is_some())
            .field("fallback", &self.fallback.is_some())
            .field("badge", &self.badge.is_some())
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

impl<'a, Message> Avatar<'a, Message> {
    /// Creates an avatar with shadcn-svelte's default 32px full-round root.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            size: AvatarSize::Default,
            radius: AvatarRadius::Theme,
            width: None,
            height: None,
            image: None,
            fallback: None,
            badge: None,
            style_override: None,
        }
    }

    /// Creates an avatar with an image source already attached.
    pub fn with_image(theme: &'a Theme, image: impl Into<AvatarImage>) -> Self {
        Self::new(theme).image(image)
    }

    /// Sets the root size preset or custom square footprint.
    pub fn size(mut self, size: AvatarSize) -> Self {
        self.size = size;
        self
    }

    /// Sets the root corner radius.
    pub fn radius(mut self, radius: AvatarRadius) -> Self {
        self.radius = radius;
        self
    }

    /// Sets an explicit root width.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = Some(width.into());
        self
    }

    /// Sets an explicit root height.
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = Some(height.into());
        self
    }

    /// Sets the image slot. The fallback remains underneath it for decode
    /// failures or intentionally missing image data.
    pub fn image(mut self, image: impl Into<AvatarImage>) -> Self {
        self.image = Some(image.into());
        self
    }

    /// Sets the fallback slot.
    pub fn fallback(mut self, fallback: AvatarFallback<'a, Message>) -> Self {
        self.fallback = Some(fallback);
        self
    }

    /// Sets a text fallback using the avatar's theme.
    pub fn fallback_text(mut self, label: impl IntoFragment<'a>) -> Self {
        self.fallback = Some(AvatarFallback::text(label, self.theme));
        self
    }

    /// Sets the bottom-right badge slot.
    pub fn badge(mut self, badge: AvatarBadge<'a, Message>) -> Self {
        self.badge = Some(badge);
        self
    }

    /// Adds a dot badge using the active theme.
    pub fn badge_dot(self) -> Self {
        let theme = self.theme;
        self.badge(AvatarBadge::dot(theme))
    }

    /// Adds an arbitrary badge element using the active theme.
    pub fn push_badge(self, content: impl Into<Element<'a, Message>>) -> Self {
        let theme = self.theme;
        self.badge(AvatarBadge::new(content, theme))
    }

    /// Applies an iced container-style override after semantic resolution.
    pub fn style_override(
        mut self,
        style_override: impl Fn(container::Style) -> container::Style + 'a,
    ) -> Self {
        self.style_override = Some(Box::new(style_override));
        self
    }

    pub(super) fn nominal_size(&self) -> f32 {
        self.size.pixels()
    }

    pub(super) fn nominal_radius(&self) -> AvatarRadius {
        self.radius
    }

    pub(super) fn into_group_element(self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        render::build_avatar(self)
    }

    /// Builds the avatar as an iced [`Element`](iced_core::Element).
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        render::build_avatar(self)
    }
}

impl<'a, Message> From<Avatar<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(avatar: Avatar<'a, Message>) -> Self {
        avatar.into_element()
    }
}

enum AvatarGroupItem<'a, Message> {
    Avatar(Box<Avatar<'a, Message>>),
    Element {
        element: Element<'a, Message>,
        size: AvatarSize,
        radius: AvatarRadius,
    },
    Count(AvatarGroupCount<'a, Message>),
}

/// A horizontally-overlapping collection of avatars and an optional count.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct AvatarGroup<'a, Message> {
    theme: &'a Theme,
    items: Vec<AvatarGroupItem<'a, Message>>,
    overlap: f32,
    style_override: Option<Box<dyn Fn(container::Style) -> container::Style + 'a>>,
}

impl<Message> fmt::Debug for AvatarGroup<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AvatarGroup")
            .field("theme", &self.theme)
            .field("items", &self.items.len())
            .field("overlap", &self.overlap)
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

impl<'a, Message> AvatarGroup<'a, Message> {
    /// Creates a group with the source component's 8px overlap.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            items: Vec::new(),
            overlap: 8.0,
            style_override: None,
        }
    }

    /// Creates a group from avatar roots.
    pub fn with_children(
        theme: &'a Theme,
        children: impl IntoIterator<Item = Avatar<'a, Message>>,
    ) -> Self {
        Self {
            items: children
                .into_iter()
                .map(|avatar| AvatarGroupItem::Avatar(Box::new(avatar)))
                .collect(),
            ..Self::new(theme)
        }
    }

    /// Appends an avatar root to the group.
    pub fn push(mut self, avatar: Avatar<'a, Message>) -> Self {
        self.items.push(AvatarGroupItem::Avatar(Box::new(avatar)));
        self
    }

    /// Appends arbitrary content with an explicit avatar footprint.
    pub fn push_element(
        mut self,
        element: impl Into<Element<'a, Message>>,
        size: AvatarSize,
    ) -> Self {
        self.items.push(AvatarGroupItem::Element {
            element: element.into(),
            size,
            radius: AvatarRadius::Theme,
        });
        self
    }

    /// Appends arbitrary content with an explicit footprint and corner radius.
    ///
    /// The source group applies its `ring-2` to arbitrary children, so the
    /// ring follows the child's shape. Use this method when the child is not
    /// a fully rounded avatar root.
    pub fn push_element_with_radius(
        mut self,
        element: impl Into<Element<'a, Message>>,
        size: AvatarSize,
        radius: AvatarRadius,
    ) -> Self {
        self.items.push(AvatarGroupItem::Element {
            element: element.into(),
            size,
            radius,
        });
        self
    }

    /// Appends every avatar root from an iterator.
    pub fn extend(self, children: impl IntoIterator<Item = Avatar<'a, Message>>) -> Self {
        children.into_iter().fold(self, Self::push)
    }

    /// Appends a group count/action item.
    pub fn count(mut self, count: AvatarGroupCount<'a, Message>) -> Self {
        self.items.push(AvatarGroupItem::Count(count));
        self
    }

    /// Sets the positive overlap distance between adjacent items.
    ///
    /// Negative and non-finite values resolve to zero, while the default is
    /// 8px (`-space-x-2`).
    pub fn overlap(mut self, overlap: f32) -> Self {
        self.overlap = geometry::normalize_px(overlap);
        self
    }

    /// Applies an iced container-style override to the group wrapper.
    pub fn style_override(
        mut self,
        style_override: impl Fn(container::Style) -> container::Style + 'a,
    ) -> Self {
        self.style_override = Some(Box::new(style_override));
        self
    }

    /// Builds the group as an iced [`Element`](iced_core::Element).
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        render::build_group(self)
    }
}

impl<'a, Message> From<AvatarGroup<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(group: AvatarGroup<'a, Message>) -> Self {
        group.into_element()
    }
}
