//! Builder-first item component.
//!
//! This is the iced composition counterpart of shadcn-svelte's `Item` family:
//! a flexible row that groups media, content, and actions slots, plus the
//! `ItemGroup` list container and the `ItemSeparator` rule. Slots accept
//! arbitrary iced elements, while the typed title and description helpers
//! retain style-pack typography.
//!
//! Items are static containers by default. [`Item::on_press`] turns the row
//! into a pressable surface with the source's `[a]:hover:bg-muted` hover
//! treatment, mirroring the anchor-rendered items of shadcn-svelte.
//!
//! ```rust,no_run
//! use iced::Element;
//! use iced_shadcn_v2::{
//!     Button, ButtonSize, ButtonVariant, Item, ItemActions, ItemContent, ItemDescription,
//!     ItemTitle, ItemVariant, Theme,
//! };
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     Open,
//! }
//!
//! fn view(theme: &Theme) -> Element<'_, Message> {
//!     Item::new(theme)
//!         .variant(ItemVariant::Outline)
//!         .content(
//!             ItemContent::new(theme)
//!                 .title(ItemTitle::text("Basic Item", theme))
//!                 .description(ItemDescription::text(
//!                     "A simple item with title and description.",
//!                     theme,
//!                 )),
//!         )
//!         .actions(
//!             ItemActions::new(theme).push(
//!                 Button::text("Open", theme)
//!                     .variant(ButtonVariant::Outline)
//!                     .size(ButtonSize::Sm)
//!                     .on_press(Message::Open),
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

pub use types::{ItemMediaVariant, ItemRadius, ItemSize, ItemVariant};

use std::fmt;

use crate::iced_compat::widget::container;
use crate::iced_compat::widget::text::{Fragment, IntoFragment};
use crate::iced_compat::{Color, Element, Font, Length};

use crate::components::separator::Separator;
use crate::theme::Theme;

/// A style-pack-aware item row.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct Item<'a, Message> {
    pub(super) theme: &'a Theme,
    pub(super) variant: ItemVariant,
    pub(super) size: ItemSize,
    pub(super) radius: ItemRadius,
    pub(super) width: Length,
    pub(super) spacing: Option<f32>,
    pub(super) padding_x: Option<f32>,
    pub(super) padding_y: Option<f32>,
    pub(super) children: Vec<ItemRowChild<'a, Message>>,
    pub(super) on_press: Option<Message>,
    pub(super) style_override: Option<Box<dyn Fn(container::Style) -> container::Style + 'a>>,
}

pub(super) enum ItemRowChild<'a, Message> {
    Media(Box<ItemMedia<'a, Message>>),
    Content(Box<ItemContent<'a, Message>>),
    Actions(Box<ItemActions<'a, Message>>),
    Header(Box<ItemHeader<'a, Message>>),
    Footer(Box<ItemFooter<'a, Message>>),
    Element(Element<'a, Message>),
}

impl<Message> fmt::Debug for Item<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Item")
            .field("theme", &self.theme)
            .field("variant", &self.variant)
            .field("size", &self.size)
            .field("radius", &self.radius)
            .field("width", &self.width)
            .field("spacing", &self.spacing)
            .field("padding_x", &self.padding_x)
            .field("padding_y", &self.padding_y)
            .field("children", &self.children.len())
            .field("on_press", &self.on_press.is_some())
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

impl<'a, Message> Item<'a, Message> {
    /// Creates an empty item using the active style-pack defaults.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            variant: ItemVariant::Default,
            size: ItemSize::Default,
            radius: ItemRadius::Theme,
            width: Length::Fill,
            spacing: None,
            padding_x: None,
            padding_y: None,
            children: Vec::new(),
            on_press: None,
            style_override: None,
        }
    }

    /// Creates an item with arbitrary row children.
    pub fn with_children(
        theme: &'a Theme,
        children: impl IntoIterator<Item = Element<'a, Message>>,
    ) -> Self {
        Self {
            children: children.into_iter().map(ItemRowChild::Element).collect(),
            ..Self::new(theme)
        }
    }

    /// Sets the visual treatment of the item.
    pub fn variant(mut self, variant: ItemVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Sets the item density (`default`, `sm`, or `xs`).
    pub fn size(mut self, size: ItemSize) -> Self {
        self.size = size;
        self
    }

    /// Sets the item corner radius.
    pub fn radius(mut self, radius: ItemRadius) -> Self {
        self.radius = radius;
        self
    }

    /// Sets the item width. The default fills the parent (`w-full`).
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the gap between slots and stacked rows in pixels.
    ///
    /// Non-finite and negative values are normalized to `0.0`.
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = Some(normalize_px(spacing));
        self
    }

    /// Overrides the horizontal row inset in pixels.
    pub fn padding_x(mut self, padding: f32) -> Self {
        self.padding_x = Some(normalize_px(padding));
        self
    }

    /// Overrides the vertical row inset in pixels.
    pub fn padding_y(mut self, padding: f32) -> Self {
        self.padding_y = Some(normalize_px(padding));
        self
    }

    /// Adds a typed leading media slot.
    pub fn media(mut self, media: ItemMedia<'a, Message>) -> Self {
        self.children.push(ItemRowChild::Media(Box::new(media)));
        self
    }

    /// Adds a typed content slot.
    ///
    /// The first content slot fills the free row width; additional slots
    /// shrink to their intrinsic size, matching the source
    /// `[&+[data-slot=item-content]]:flex-none` rule.
    pub fn content(mut self, content: ItemContent<'a, Message>) -> Self {
        self.children.push(ItemRowChild::Content(Box::new(content)));
        self
    }

    /// Adds a typed trailing actions slot.
    pub fn actions(mut self, actions: ItemActions<'a, Message>) -> Self {
        self.children.push(ItemRowChild::Actions(Box::new(actions)));
        self
    }

    /// Adds a full-width header row above the main row.
    pub fn header(mut self, header: ItemHeader<'a, Message>) -> Self {
        self.children.push(ItemRowChild::Header(Box::new(header)));
        self
    }

    /// Adds a full-width footer row below the main row.
    pub fn footer(mut self, footer: ItemFooter<'a, Message>) -> Self {
        self.children.push(ItemRowChild::Footer(Box::new(footer)));
        self
    }

    /// Appends arbitrary content to the main row.
    pub fn push(mut self, child: impl Into<Element<'a, Message>>) -> Self {
        self.children.push(ItemRowChild::Element(child.into()));
        self
    }

    /// Appends every element from an iterator to the main row.
    pub fn extend(self, children: impl IntoIterator<Item = Element<'a, Message>>) -> Self {
        children.into_iter().fold(self, Self::push)
    }

    /// Makes the item pressable and sets the emitted message.
    ///
    /// Pressable items paint the `muted` surface on hover, like the
    /// anchor-rendered items of the source component.
    pub fn on_press(mut self, message: Message) -> Self {
        self.on_press = Some(message);
        self
    }

    /// Sets or clears the message emitted when the item is pressed.
    pub fn on_press_maybe(mut self, message: Option<Message>) -> Self {
        self.on_press = message;
        self
    }

    /// Applies an iced container-style override after semantic resolution.
    ///
    /// For pressable items the override runs on both the resting and the
    /// hovered style.
    pub fn style_override(
        mut self,
        style_override: impl Fn(container::Style) -> container::Style + 'a,
    ) -> Self {
        self.style_override = Some(Box::new(style_override));
        self
    }

    /// Builds the item as an iced [`Element`](iced_core::Element).
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        render::build_root(self)
    }
}

impl<'a, Message> From<Item<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(item: Item<'a, Message>) -> Self {
        item.into_element()
    }
}

/// A leading media slot: icon, thumbnail, or arbitrary decoration.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct ItemMedia<'a, Message> {
    pub(super) theme: &'a Theme,
    pub(super) variant: ItemMediaVariant,
    pub(super) children: Vec<Element<'a, Message>>,
    pub(super) spacing: Option<f32>,
    pub(super) image_size: Option<f32>,
    pub(super) image_radius: Option<f32>,
    pub(super) style_override: Option<Box<dyn Fn(container::Style) -> container::Style + 'a>>,
}

impl<Message> fmt::Debug for ItemMedia<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ItemMedia")
            .field("theme", &self.theme)
            .field("variant", &self.variant)
            .field("children", &self.children.len())
            .field("spacing", &self.spacing)
            .field("image_size", &self.image_size)
            .field("image_radius", &self.image_radius)
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

impl<'a, Message> ItemMedia<'a, Message> {
    /// Creates an empty media slot with the `default` variant.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            variant: ItemMediaVariant::Default,
            children: Vec::new(),
            spacing: None,
            image_size: None,
            image_radius: None,
            style_override: None,
        }
    }

    /// Creates an `icon` media slot from a glyph element.
    ///
    /// The source sizes plain icons to 16 px; pass a matching glyph size.
    pub fn icon(content: impl Into<Element<'a, Message>>, theme: &'a Theme) -> Self {
        Self::new(theme)
            .variant(ItemMediaVariant::Icon)
            .push(content)
    }

    /// Creates an `image` media slot: a clipped square thumbnail sized by the
    /// item density.
    ///
    /// iced clips to the rectangular bounds; the rounded-corner clipping of
    /// the source `overflow-hidden rounded-*` pair applies to the slot's own
    /// background and border only.
    pub fn image(content: impl Into<Element<'a, Message>>, theme: &'a Theme) -> Self {
        Self::new(theme)
            .variant(ItemMediaVariant::Image)
            .push(content)
    }

    /// Sets the media visual treatment.
    pub fn variant(mut self, variant: ItemMediaVariant) -> Self {
        self.variant = variant;
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

    /// Sets the gap between multiple media children.
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = Some(normalize_px(spacing));
        self
    }

    /// Overrides the square edge of the `image` variant in pixels.
    pub fn image_size(mut self, size: f32) -> Self {
        self.image_size = Some(normalize_px(size));
        self
    }

    /// Overrides the corner radius of the `image` variant in pixels.
    pub fn image_radius(mut self, radius: f32) -> Self {
        self.image_radius = Some(normalize_px(radius));
        self
    }

    /// Applies an iced container-style override to the media wrapper.
    pub fn style_override(
        mut self,
        style_override: impl Fn(container::Style) -> container::Style + 'a,
    ) -> Self {
        self.style_override = Some(Box::new(style_override));
        self
    }

    /// Builds the media slot with default-density geometry.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        render::build_media(self, ItemSize::Default, false)
    }
}

impl<'a, Message> From<ItemMedia<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(media: ItemMedia<'a, Message>) -> Self {
        media.into_element()
    }
}

/// The main content column with typed title and description slots.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct ItemContent<'a, Message> {
    pub(super) theme: &'a Theme,
    pub(super) title: Option<ItemTitle<'a, Message>>,
    pub(super) title_element: Option<Element<'a, Message>>,
    pub(super) description: Option<ItemDescription<'a, Message>>,
    pub(super) description_element: Option<Element<'a, Message>>,
    pub(super) children: Vec<Element<'a, Message>>,
    pub(super) spacing: Option<f32>,
    pub(super) style_override: Option<Box<dyn Fn(container::Style) -> container::Style + 'a>>,
}

impl<Message> fmt::Debug for ItemContent<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ItemContent")
            .field("theme", &self.theme)
            .field("title", &self.title.is_some())
            .field("title_element", &self.title_element.is_some())
            .field("description", &self.description.is_some())
            .field("description_element", &self.description_element.is_some())
            .field("children", &self.children.len())
            .field("spacing", &self.spacing)
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

impl<'a, Message> ItemContent<'a, Message> {
    /// Creates an empty content column.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            title: None,
            title_element: None,
            description: None,
            description_element: None,
            children: Vec::new(),
            spacing: None,
            style_override: None,
        }
    }

    /// Sets the style-pack-aware title.
    pub fn title(mut self, title: ItemTitle<'a, Message>) -> Self {
        self.title = Some(title);
        self.title_element = None;
        self
    }

    /// Sets an arbitrary title element. Use [`Self::title`] for automatic
    /// style-pack typography.
    pub fn title_element(mut self, title: impl Into<Element<'a, Message>>) -> Self {
        self.title = None;
        self.title_element = Some(title.into());
        self
    }

    /// Sets the style-pack-aware description.
    ///
    /// A description also top-aligns the sibling media slot with a small
    /// offset, matching the source `group-has-data-[slot=item-description]`
    /// rules.
    pub fn description(mut self, description: ItemDescription<'a, Message>) -> Self {
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

    /// Appends arbitrary content below the title and description.
    pub fn push(mut self, child: impl Into<Element<'a, Message>>) -> Self {
        self.children.push(child.into());
        self
    }

    /// Appends every element from an iterator.
    pub fn extend(self, children: impl IntoIterator<Item = Element<'a, Message>>) -> Self {
        children.into_iter().fold(self, Self::push)
    }

    /// Sets the gap between content rows in pixels.
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

    pub(super) fn has_description(&self) -> bool {
        self.description.is_some() || self.description_element.is_some()
    }

    /// Builds the content column with default-density geometry.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        render::build_content(self, ItemSize::Default, true)
    }
}

impl<'a, Message> From<ItemContent<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(content: ItemContent<'a, Message>) -> Self {
        content.into_element()
    }
}

/// Styled item title content.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct ItemTitle<'a, Message> {
    pub(super) content: ItemTextContent<'a, Message>,
    pub(super) theme: &'a Theme,
    pub(super) text_size: Option<f32>,
    pub(super) line_height: Option<f32>,
    pub(super) color: Option<Color>,
    pub(super) font: Option<Font>,
    pub(super) width: Length,
}

impl<Message> fmt::Debug for ItemTitle<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ItemTitle")
            .field("content", &self.content.kind())
            .field("theme", &self.theme)
            .field("text_size", &self.text_size)
            .field("line_height", &self.line_height)
            .field("color", &self.color)
            .field("font", &self.font)
            .field("width", &self.width)
            .finish()
    }
}

impl<'a, Message> ItemTitle<'a, Message> {
    /// Creates a title from arbitrary content.
    pub fn new(content: impl Into<Element<'a, Message>>, theme: &'a Theme) -> Self {
        Self::from_content(ItemTextContent::Element(content.into()), theme)
    }

    /// Creates a style-pack-aware text title.
    pub fn text(label: impl IntoFragment<'a>, theme: &'a Theme) -> Self {
        Self::from_content(ItemTextContent::Label(label.into_fragment()), theme)
    }

    fn from_content(content: ItemTextContent<'a, Message>, theme: &'a Theme) -> Self {
        Self {
            content,
            theme,
            text_size: None,
            line_height: None,
            color: None,
            font: None,
            width: Length::Fill,
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

    /// Sets the title font, replacing the style-pack heading face.
    pub fn font(mut self, font: Font) -> Self {
        self.font = Some(font);
        self
    }

    /// Sets the title width.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Builds the title with style-pack typography.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        render::build_title(self)
    }
}

impl<'a, Message> From<ItemTitle<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(title: ItemTitle<'a, Message>) -> Self {
        title.into_element()
    }
}

/// Styled item description content.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct ItemDescription<'a, Message> {
    pub(super) content: ItemTextContent<'a, Message>,
    pub(super) theme: &'a Theme,
    pub(super) text_size: Option<f32>,
    pub(super) line_height: Option<f32>,
    pub(super) color: Option<Color>,
    pub(super) font: Option<Font>,
    pub(super) width: Length,
}

impl<Message> fmt::Debug for ItemDescription<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ItemDescription")
            .field("content", &self.content.kind())
            .field("theme", &self.theme)
            .field("text_size", &self.text_size)
            .field("line_height", &self.line_height)
            .field("color", &self.color)
            .field("font", &self.font)
            .field("width", &self.width)
            .finish()
    }
}

impl<'a, Message> ItemDescription<'a, Message> {
    /// Creates a description from arbitrary content.
    pub fn new(content: impl Into<Element<'a, Message>>, theme: &'a Theme) -> Self {
        Self::from_content(ItemTextContent::Element(content.into()), theme)
    }

    /// Creates a style-pack-aware text description.
    pub fn text(label: impl IntoFragment<'a>, theme: &'a Theme) -> Self {
        Self::from_content(ItemTextContent::Label(label.into_fragment()), theme)
    }

    fn from_content(content: ItemTextContent<'a, Message>, theme: &'a Theme) -> Self {
        Self {
            content,
            theme,
            text_size: None,
            line_height: None,
            color: None,
            font: None,
            width: Length::Fill,
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

    /// Sets the description text color, overriding `muted-foreground`.
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

    /// Builds the description with default-density typography.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        render::build_description(self, ItemSize::Default)
    }
}

impl<'a, Message> From<ItemDescription<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(description: ItemDescription<'a, Message>) -> Self {
        description.into_element()
    }
}

/// A trailing actions row (buttons, switches, glyphs).
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct ItemActions<'a, Message> {
    pub(super) theme: &'a Theme,
    pub(super) children: Vec<Element<'a, Message>>,
    pub(super) spacing: Option<f32>,
}

impl<Message> fmt::Debug for ItemActions<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ItemActions")
            .field("theme", &self.theme)
            .field("children", &self.children.len())
            .field("spacing", &self.spacing)
            .finish()
    }
}

impl<'a, Message> ItemActions<'a, Message> {
    /// Creates an empty actions row.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            children: Vec::new(),
            spacing: None,
        }
    }

    /// Appends an action element.
    pub fn push(mut self, child: impl Into<Element<'a, Message>>) -> Self {
        self.children.push(child.into());
        self
    }

    /// Appends every element from an iterator.
    pub fn extend(self, children: impl IntoIterator<Item = Element<'a, Message>>) -> Self {
        children.into_iter().fold(self, Self::push)
    }

    /// Sets the gap between action children.
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = Some(normalize_px(spacing));
        self
    }

    /// Builds the actions row.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        render::build_actions(self)
    }
}

impl<'a, Message> From<ItemActions<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(actions: ItemActions<'a, Message>) -> Self {
        actions.into_element()
    }
}

/// A full-width header row with space-between distribution.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct ItemHeader<'a, Message> {
    pub(super) theme: &'a Theme,
    pub(super) children: Vec<Element<'a, Message>>,
    pub(super) spacing: Option<f32>,
}

impl<Message> fmt::Debug for ItemHeader<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ItemHeader")
            .field("theme", &self.theme)
            .field("children", &self.children.len())
            .field("spacing", &self.spacing)
            .finish()
    }
}

impl<'a, Message> ItemHeader<'a, Message> {
    /// Creates an empty header row.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            children: Vec::new(),
            spacing: None,
        }
    }

    /// Appends header content.
    pub fn push(mut self, child: impl Into<Element<'a, Message>>) -> Self {
        self.children.push(child.into());
        self
    }

    /// Appends every element from an iterator.
    pub fn extend(self, children: impl IntoIterator<Item = Element<'a, Message>>) -> Self {
        children.into_iter().fold(self, Self::push)
    }

    /// Sets the gap between header children.
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = Some(normalize_px(spacing));
        self
    }

    /// Builds the header row.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        render::build_section(self.children, self.spacing)
    }
}

impl<'a, Message> From<ItemHeader<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(header: ItemHeader<'a, Message>) -> Self {
        header.into_element()
    }
}

/// A full-width footer row with space-between distribution.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct ItemFooter<'a, Message> {
    pub(super) theme: &'a Theme,
    pub(super) children: Vec<Element<'a, Message>>,
    pub(super) spacing: Option<f32>,
}

impl<Message> fmt::Debug for ItemFooter<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ItemFooter")
            .field("theme", &self.theme)
            .field("children", &self.children.len())
            .field("spacing", &self.spacing)
            .finish()
    }
}

impl<'a, Message> ItemFooter<'a, Message> {
    /// Creates an empty footer row.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            children: Vec::new(),
            spacing: None,
        }
    }

    /// Appends footer content.
    pub fn push(mut self, child: impl Into<Element<'a, Message>>) -> Self {
        self.children.push(child.into());
        self
    }

    /// Appends every element from an iterator.
    pub fn extend(self, children: impl IntoIterator<Item = Element<'a, Message>>) -> Self {
        children.into_iter().fold(self, Self::push)
    }

    /// Sets the gap between footer children.
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = Some(normalize_px(spacing));
        self
    }

    /// Builds the footer row.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        render::build_section(self.children, self.spacing)
    }
}

impl<'a, Message> From<ItemFooter<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(footer: ItemFooter<'a, Message>) -> Self {
        footer.into_element()
    }
}

/// A vertical list of items with density-aware gaps.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct ItemGroup<'a, Message> {
    pub(super) theme: &'a Theme,
    pub(super) children: Vec<ItemGroupChild<'a, Message>>,
    pub(super) spacing: Option<f32>,
    pub(super) width: Length,
    pub(super) has_sm: bool,
    pub(super) has_xs: bool,
}

pub(super) enum ItemGroupChild<'a, Message> {
    Item(Box<Item<'a, Message>>),
    Element(Element<'a, Message>),
}

impl<Message> fmt::Debug for ItemGroup<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ItemGroup")
            .field("theme", &self.theme)
            .field("children", &self.children.len())
            .field("spacing", &self.spacing)
            .field("width", &self.width)
            .field("has_sm", &self.has_sm)
            .field("has_xs", &self.has_xs)
            .finish()
    }
}

impl<'a, Message> ItemGroup<'a, Message> {
    /// Creates an empty item group.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            children: Vec::new(),
            spacing: None,
            width: Length::Fill,
            has_sm: false,
            has_xs: false,
        }
    }

    /// Appends an item, letting the group derive its density-aware gap.
    pub fn push(mut self, item: Item<'a, Message>) -> Self {
        match item.size {
            ItemSize::Default => {}
            ItemSize::Sm => self.has_sm = true,
            ItemSize::Xs => self.has_xs = true,
        }
        self.children.push(ItemGroupChild::Item(Box::new(item)));
        self
    }

    /// Appends every item from an iterator.
    pub fn extend(self, items: impl IntoIterator<Item = Item<'a, Message>>) -> Self {
        items.into_iter().fold(self, Self::push)
    }

    /// Appends an arbitrary element (e.g. an [`ItemSeparator`]).
    pub fn push_element(mut self, child: impl Into<Element<'a, Message>>) -> Self {
        self.children.push(ItemGroupChild::Element(child.into()));
        self
    }

    /// Appends a default [`ItemSeparator`].
    pub fn separator(self) -> Self
    where
        Message: 'a,
    {
        let separator = ItemSeparator::new(self.theme);
        self.push_element(separator)
    }

    /// Overrides the vertical gap between grouped children in pixels.
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = Some(normalize_px(spacing));
        self
    }

    /// Sets the group width. The default fills the parent (`w-full`).
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
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

impl<'a, Message> From<ItemGroup<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(group: ItemGroup<'a, Message>) -> Self {
        group.into_element()
    }
}

/// A horizontal rule between grouped items with the source `my-2` margin.
#[derive(Debug, Clone, Copy, PartialEq)]
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct ItemSeparator {
    pub(super) separator: Separator,
    pub(super) margin_y: f32,
}

impl ItemSeparator {
    /// Creates a separator painted with the theme `border` token.
    pub fn new(theme: &Theme) -> Self {
        Self {
            separator: Separator::new(theme),
            margin_y: geometry::separator_margin_y(),
        }
    }

    /// Sets the rule color.
    pub fn color(mut self, color: Color) -> Self {
        self.separator = self.separator.color(color);
        self
    }

    /// Sets the rule thickness in pixels (clamped to at least 1 px).
    pub fn thickness(mut self, thickness: f32) -> Self {
        self.separator = self.separator.thickness(thickness);
        self
    }

    /// Sets the vertical margin around the rule in pixels.
    pub fn margin_y(mut self, margin: f32) -> Self {
        self.margin_y = normalize_px(margin);
        self
    }

    /// Builds the separator as an iced [`Element`](iced_core::Element).
    pub fn into_element<'a, Message>(self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        render::build_separator(self)
    }
}

impl<'a, Message> From<ItemSeparator> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(separator: ItemSeparator) -> Self {
        separator.into_element()
    }
}

pub(super) enum ItemTextContent<'a, Message> {
    Label(Fragment<'a>),
    Element(Element<'a, Message>),
}

impl<Message> ItemTextContent<'_, Message> {
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
