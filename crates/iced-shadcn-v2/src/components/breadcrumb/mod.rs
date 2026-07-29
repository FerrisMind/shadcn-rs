//! Builder-first breadcrumb component.
//!
//! Port of shadcn-svelte `Breadcrumb.Root`, `Breadcrumb.List`,
//! `Breadcrumb.Item`, `Breadcrumb.Link`, `Breadcrumb.Page`,
//! `Breadcrumb.Separator`, and `Breadcrumb.Ellipsis`. Every web part has a
//! builder here, so the DOM tree of the source component maps one-to-one onto
//! the iced tree.
//!
//! The public API lives in this module; style-pack metrics, semantic colors,
//! the canvas glyphs, and layout assembly are kept in focused private
//! submodules.
//!
//! iced exposes no accessibility tree yet, so the web `aria-label="breadcrumb"`,
//! `aria-current="page"`, `role="presentation"`, and the ellipsis screen-reader
//! label are carried as inert builder values ([`Breadcrumb::aria_label`],
//! [`BreadcrumbEllipsis::screen_reader_label`]) and take effect once
//! accessibility support lands.
//!
//! ```rust,no_run
//! use iced::Element;
//! use iced_shadcn_v2::{Breadcrumb, BreadcrumbLink, BreadcrumbPage, Theme};
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     Navigate(&'static str),
//! }
//!
//! fn trail(theme: &Theme) -> Element<'_, Message> {
//!     Breadcrumb::new(theme)
//!         .push(BreadcrumbLink::text("Home", theme).on_press(Message::Navigate("/")))
//!         .push_separator()
//!         .push(
//!             BreadcrumbLink::text("Components", theme)
//!                 .on_press(Message::Navigate("/components")),
//!         )
//!         .push_separator()
//!         .push(BreadcrumbPage::text("Breadcrumb", theme))
//!         .into()
//! }
//! ```

mod geometry;
mod icon;
mod render;
mod style;

#[cfg(test)]
mod tests;

use std::borrow::Cow;
use std::fmt;

use crate::iced_compat::widget::button as button_widget;
use crate::iced_compat::widget::container;
use crate::iced_compat::widget::text::{Fragment, IntoFragment};
use crate::iced_compat::{Color, Element, Font, Length, Padding};

use crate::theme::Theme;

/// Text or arbitrary content of a breadcrumb text part.
enum TextContent<'a, Message> {
    Label(Fragment<'a>),
    Element(Element<'a, Message>),
}

impl<Message> TextContent<'_, Message> {
    fn kind(&self) -> &'static str {
        match self {
            Self::Label(_) => "label",
            Self::Element(_) => "element",
        }
    }
}

/// Navigation trail showing the path to the current page.
///
/// Port of shadcn-svelte `Breadcrumb.Root`: a wrapper around a single
/// [`BreadcrumbList`]. Entries pushed on the root are forwarded to that list,
/// so the common case needs no explicit list; call [`Self::list`] when the list
/// itself has to be configured.
///
/// ```rust,no_run
/// use iced::Element;
/// use iced_shadcn_v2::{Breadcrumb, BreadcrumbList, BreadcrumbPage, Theme};
///
/// #[derive(Debug, Clone)]
/// enum Message {}
///
/// fn trail(theme: &Theme) -> Element<'_, Message> {
///     Breadcrumb::new(theme)
///         .list(BreadcrumbList::new(theme).spacing(10.0))
///         .push(BreadcrumbPage::text("Breadcrumb", theme))
///         .into()
/// }
/// ```
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct Breadcrumb<'a, Message> {
    theme: &'a Theme,
    list: BreadcrumbList<'a, Message>,
    width: Length,
    height: Length,
    padding: Padding,
    aria_label: Cow<'a, str>,
    style_override: Option<Box<dyn Fn(container::Style) -> container::Style + 'a>>,
}

impl<Message> fmt::Debug for Breadcrumb<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Breadcrumb")
            .field("theme", &self.theme)
            .field("list", &self.list)
            .field("width", &self.width)
            .field("height", &self.height)
            .field("padding", &self.padding)
            .field("aria_label", &self.aria_label.as_ref())
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

impl<'a, Message> Breadcrumb<'a, Message> {
    /// Creates an empty trail using the active style-pack defaults.
    ///
    /// `theme` is required because spacing, typography, and colors resolve
    /// from `shadcn-common` theme tokens instead of `iced::Theme`.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            list: BreadcrumbList::new(theme),
            width: Length::Shrink,
            height: Length::Shrink,
            padding: Padding::ZERO,
            aria_label: Cow::Borrowed("breadcrumb"),
            style_override: None,
        }
    }

    /// Creates a trail populated from an iterator of entries.
    pub fn with_children(
        theme: &'a Theme,
        children: impl IntoIterator<Item = BreadcrumbEntry<'a, Message>>,
    ) -> Self {
        Self::new(theme).extend(children)
    }

    /// Replaces the wrapped list, keeping any entries already pushed.
    pub fn list(mut self, list: BreadcrumbList<'a, Message>) -> Self {
        let entries = std::mem::take(&mut self.list.entries);
        self.list = list.extend(entries);
        self
    }

    /// Appends an entry: a [`BreadcrumbItem`], a [`BreadcrumbSeparator`], a
    /// bare [`BreadcrumbLink`] / [`BreadcrumbPage`] / [`BreadcrumbEllipsis`],
    /// or an arbitrary widget.
    pub fn push(mut self, entry: impl Into<BreadcrumbEntry<'a, Message>>) -> Self {
        self.list = self.list.push(entry);
        self
    }

    /// Appends the default chevron separator — port of the web
    /// `Breadcrumb.Separator` with no children.
    pub fn push_separator(mut self) -> Self {
        self.list = self.list.push_separator();
        self
    }

    /// Appends an arbitrary widget (a button, a pick list, …) as an entry.
    pub fn push_element(mut self, element: impl Into<Element<'a, Message>>) -> Self {
        self.list = self.list.push_element(element);
        self
    }

    /// Appends every entry of the given iterator.
    pub fn extend(
        mut self,
        children: impl IntoIterator<Item = BreadcrumbEntry<'a, Message>>,
    ) -> Self {
        self.list = self.list.extend(children);
        self
    }

    /// Sets the trail width (defaults to [`Length::Shrink`], like the web `nav`
    /// hugging its list).
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the trail height (defaults to [`Length::Shrink`]).
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Sets the padding around the list. Negative and non-finite sides are
    /// normalized to zero.
    pub fn padding(mut self, padding: Padding) -> Self {
        self.padding = Padding {
            top: geometry::normalize_px(padding.top),
            right: geometry::normalize_px(padding.right),
            bottom: geometry::normalize_px(padding.bottom),
            left: geometry::normalize_px(padding.left),
        };
        self
    }

    /// Sets the accessible label of the trail (defaults to `"breadcrumb"`).
    ///
    /// Carried for parity with the web `aria-label`; see the module docs.
    pub fn aria_label(mut self, aria_label: impl Into<Cow<'a, str>>) -> Self {
        self.aria_label = aria_label.into();
        self
    }

    /// Returns the accessible label of the trail.
    pub fn accessible_label(&self) -> &str {
        self.aria_label.as_ref()
    }

    /// Applies a narrow iced-style escape hatch to the trail wrapper.
    pub fn style_override(
        mut self,
        style_override: impl Fn(container::Style) -> container::Style + 'a,
    ) -> Self {
        self.style_override = Some(Box::new(style_override));
        self
    }

    /// Builds the trail as an iced [`Element`](iced_core::Element).
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        render::build_breadcrumb(self)
    }
}

impl<'a, Message> From<Breadcrumb<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(breadcrumb: Breadcrumb<'a, Message>) -> Self {
        breadcrumb.into_element()
    }
}

/// Ordered list of breadcrumb entries — port of the web `Breadcrumb.List`.
///
/// Lays entries out as a wrapping, vertically centered row (`flex flex-wrap
/// items-center`) and owns the tokens the web `<ol>` passes down by CSS
/// inheritance: [`Self::color`], [`Self::text_size`], and
/// [`Self::line_height`] become the defaults of every entry that does not set
/// its own.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct BreadcrumbList<'a, Message> {
    theme: &'a Theme,
    entries: Vec<BreadcrumbEntry<'a, Message>>,
    spacing: Option<f32>,
    line_spacing: Option<f32>,
    wrap: bool,
    width: Length,
    color: Option<Color>,
    text_size: Option<f32>,
    line_height: Option<f32>,
    style_override: Option<Box<dyn Fn(container::Style) -> container::Style + 'a>>,
}

impl<Message> fmt::Debug for BreadcrumbList<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("BreadcrumbList")
            .field("theme", &self.theme)
            .field("entries", &self.entries)
            .field("spacing", &self.spacing)
            .field("line_spacing", &self.line_spacing)
            .field("wrap", &self.wrap)
            .field("width", &self.width)
            .field("color", &self.color)
            .field("text_size", &self.text_size)
            .field("line_height", &self.line_height)
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

impl<'a, Message> BreadcrumbList<'a, Message> {
    /// Creates an empty list using the active style-pack defaults.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            entries: Vec::new(),
            spacing: None,
            line_spacing: None,
            wrap: true,
            width: Length::Shrink,
            color: None,
            text_size: None,
            line_height: None,
            style_override: None,
        }
    }

    /// Creates a list populated from an iterator of entries.
    pub fn with_children(
        theme: &'a Theme,
        children: impl IntoIterator<Item = BreadcrumbEntry<'a, Message>>,
    ) -> Self {
        Self::new(theme).extend(children)
    }

    /// Appends an entry.
    pub fn push(mut self, entry: impl Into<BreadcrumbEntry<'a, Message>>) -> Self {
        self.entries.push(entry.into());
        self
    }

    /// Appends the default chevron separator.
    pub fn push_separator(self) -> Self {
        let separator = BreadcrumbSeparator::new(self.theme);
        self.push(separator)
    }

    /// Appends an arbitrary widget (a button, a pick list, …) as an entry.
    pub fn push_element(self, element: impl Into<Element<'a, Message>>) -> Self {
        self.push(BreadcrumbEntry::element(element))
    }

    /// Appends every entry of the given iterator.
    pub fn extend(self, children: impl IntoIterator<Item = BreadcrumbEntry<'a, Message>>) -> Self {
        children.into_iter().fold(self, Self::push)
    }

    /// Sets the gap between entries in px (defaults to the style-pack
    /// `gap-1.5`). Negative and non-finite values resolve to zero.
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = Some(geometry::normalize_px(spacing));
        self
    }

    /// Sets the gap between wrapped lines in px (defaults to
    /// [`Self::spacing`]). Negative and non-finite values resolve to zero.
    pub fn line_spacing(mut self, line_spacing: f32) -> Self {
        self.line_spacing = Some(geometry::normalize_px(line_spacing));
        self
    }

    /// Enables or disables line wrapping (the web `flex-wrap`, enabled by
    /// default).
    pub fn wrap(mut self, wrap: bool) -> Self {
        self.wrap = wrap;
        self
    }

    /// Sets the list width (defaults to [`Length::Shrink`]).
    ///
    /// Wrapping needs a bounded width: a shrinking list wraps against the
    /// width its parent offers.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Overrides the inherited entry color (defaults to the theme
    /// `muted-foreground`).
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Overrides the inherited text size in px (defaults to the style-pack
    /// `text-sm` / `text-xs`). Values are clamped to at least 1 px.
    pub fn text_size(mut self, text_size: f32) -> Self {
        self.text_size = Some(geometry::normalize_min_px(text_size));
        self
    }

    /// Overrides the inherited line height in px. Values are clamped to at
    /// least 1 px.
    pub fn line_height(mut self, line_height: f32) -> Self {
        self.line_height = Some(geometry::normalize_min_px(line_height));
        self
    }

    /// Applies a narrow iced-style escape hatch to the list surface.
    ///
    /// Entries inherit `container::Style::text_color`, so changing it here
    /// recolors every entry that does not set its own color.
    pub fn style_override(
        mut self,
        style_override: impl Fn(container::Style) -> container::Style + 'a,
    ) -> Self {
        self.style_override = Some(Box::new(style_override));
        self
    }

    /// Number of entries in the list.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether the list has no entries.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Builds the list as an iced [`Element`](iced_core::Element).
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        render::build_list(self)
    }
}

impl<'a, Message> From<BreadcrumbList<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(list: BreadcrumbList<'a, Message>) -> Self {
        list.into_element()
    }
}

/// A single child of a [`BreadcrumbList`] or of a [`BreadcrumbItem`].
///
/// Usually created implicitly through the [`From`] conversions accepted by
/// [`BreadcrumbList::push`] and [`BreadcrumbItem::push`]; use [`Self::element`]
/// for arbitrary widgets.
///
/// A bare [`BreadcrumbLink`], [`BreadcrumbPage`], or [`BreadcrumbEllipsis`]
/// pushed straight onto a list renders exactly as if it were wrapped in a
/// single-child [`BreadcrumbItem`], which keeps the source markup's `<li>`
/// nesting optional.
#[must_use = "entries do nothing unless pushed into a breadcrumb"]
pub struct BreadcrumbEntry<'a, Message> {
    kind: EntryKind<'a, Message>,
}

enum EntryKind<'a, Message> {
    Item(Box<BreadcrumbItem<'a, Message>>),
    Separator(Box<BreadcrumbSeparator<'a, Message>>),
    Link(Box<BreadcrumbLink<'a, Message>>),
    Page(Box<BreadcrumbPage<'a, Message>>),
    Ellipsis(Box<BreadcrumbEllipsis<'a, Message>>),
    Element(Element<'a, Message>),
}

impl<'a, Message> BreadcrumbEntry<'a, Message> {
    /// Wraps an arbitrary widget (a button, a dropdown trigger, …) as an entry.
    pub fn element(element: impl Into<Element<'a, Message>>) -> Self {
        Self {
            kind: EntryKind::Element(element.into()),
        }
    }

    /// Kebab-case name of the wrapped part, matching the web `data-slot`
    /// values (`"breadcrumb-item"`, `"breadcrumb-separator"`, …).
    pub const fn slot(&self) -> &'static str {
        match self.kind {
            EntryKind::Item(_) => "breadcrumb-item",
            EntryKind::Separator(_) => "breadcrumb-separator",
            EntryKind::Link(_) => "breadcrumb-link",
            EntryKind::Page(_) => "breadcrumb-page",
            EntryKind::Ellipsis(_) => "breadcrumb-ellipsis",
            EntryKind::Element(_) => "element",
        }
    }
}

impl<Message> fmt::Debug for BreadcrumbEntry<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("BreadcrumbEntry")
            .field(&self.slot())
            .finish()
    }
}

impl<'a, Message> From<BreadcrumbItem<'a, Message>> for BreadcrumbEntry<'a, Message> {
    fn from(item: BreadcrumbItem<'a, Message>) -> Self {
        Self {
            kind: EntryKind::Item(Box::new(item)),
        }
    }
}

impl<'a, Message> From<BreadcrumbSeparator<'a, Message>> for BreadcrumbEntry<'a, Message> {
    fn from(separator: BreadcrumbSeparator<'a, Message>) -> Self {
        Self {
            kind: EntryKind::Separator(Box::new(separator)),
        }
    }
}

impl<'a, Message> From<BreadcrumbLink<'a, Message>> for BreadcrumbEntry<'a, Message> {
    fn from(link: BreadcrumbLink<'a, Message>) -> Self {
        Self {
            kind: EntryKind::Link(Box::new(link)),
        }
    }
}

impl<'a, Message> From<BreadcrumbPage<'a, Message>> for BreadcrumbEntry<'a, Message> {
    fn from(page: BreadcrumbPage<'a, Message>) -> Self {
        Self {
            kind: EntryKind::Page(Box::new(page)),
        }
    }
}

impl<'a, Message> From<BreadcrumbEllipsis<'a, Message>> for BreadcrumbEntry<'a, Message> {
    fn from(ellipsis: BreadcrumbEllipsis<'a, Message>) -> Self {
        Self {
            kind: EntryKind::Ellipsis(Box::new(ellipsis)),
        }
    }
}

impl<'a, Message> From<Element<'a, Message>> for BreadcrumbEntry<'a, Message> {
    fn from(element: Element<'a, Message>) -> Self {
        Self::element(element)
    }
}

/// One step of the trail — port of the web `Breadcrumb.Item` (`<li>`).
///
/// Lays its children out as a vertically centered row with the style-pack
/// item gap, which is what the web component uses to sit a label next to an
/// icon or a dropdown trigger.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct BreadcrumbItem<'a, Message> {
    theme: &'a Theme,
    children: Vec<BreadcrumbEntry<'a, Message>>,
    spacing: Option<f32>,
    width: Length,
    style_override: Option<Box<dyn Fn(container::Style) -> container::Style + 'a>>,
}

impl<Message> fmt::Debug for BreadcrumbItem<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("BreadcrumbItem")
            .field("theme", &self.theme)
            .field("children", &self.children)
            .field("spacing", &self.spacing)
            .field("width", &self.width)
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

impl<'a, Message> BreadcrumbItem<'a, Message> {
    /// Creates an empty item using the active style-pack defaults.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            children: Vec::new(),
            spacing: None,
            width: Length::Shrink,
            style_override: None,
        }
    }

    /// Creates an item populated from an iterator of children.
    pub fn with_children(
        theme: &'a Theme,
        children: impl IntoIterator<Item = BreadcrumbEntry<'a, Message>>,
    ) -> Self {
        Self::new(theme).extend(children)
    }

    /// Appends a child: a [`BreadcrumbLink`], a [`BreadcrumbPage`], a
    /// [`BreadcrumbEllipsis`], or an arbitrary widget.
    pub fn push(mut self, child: impl Into<BreadcrumbEntry<'a, Message>>) -> Self {
        self.children.push(child.into());
        self
    }

    /// Appends an arbitrary widget (a dropdown trigger, an icon, …) as a child.
    pub fn push_element(self, element: impl Into<Element<'a, Message>>) -> Self {
        self.push(BreadcrumbEntry::element(element))
    }

    /// Appends every child of the given iterator.
    pub fn extend(self, children: impl IntoIterator<Item = BreadcrumbEntry<'a, Message>>) -> Self {
        children.into_iter().fold(self, Self::push)
    }

    /// Sets the gap between children in px (defaults to the style-pack
    /// `gap-1.5` / `gap-1`). Negative and non-finite values resolve to zero.
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = Some(geometry::normalize_px(spacing));
        self
    }

    /// Sets the item width (defaults to [`Length::Shrink`]).
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Applies a narrow iced-style escape hatch to the item surface.
    pub fn style_override(
        mut self,
        style_override: impl Fn(container::Style) -> container::Style + 'a,
    ) -> Self {
        self.style_override = Some(Box::new(style_override));
        self
    }

    /// Number of children in the item.
    pub fn len(&self) -> usize {
        self.children.len()
    }

    /// Whether the item has no children.
    pub fn is_empty(&self) -> bool {
        self.children.is_empty()
    }

    /// Builds the item as an iced [`Element`](iced_core::Element).
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        let inherited = render::inherited(self.theme);
        render::build_item(self, inherited)
    }
}

impl<'a, Message> From<BreadcrumbItem<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(item: BreadcrumbItem<'a, Message>) -> Self {
        item.into_element()
    }
}

/// Navigable step of the trail — port of the web `Breadcrumb.Link` (`<a>`).
///
/// Renders as a transparent iced button so the source
/// `hover:text-foreground transition-colors` treatment and the pointer cursor
/// work. Without [`Self::on_press`] the link is inert and keeps its resting
/// color.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct BreadcrumbLink<'a, Message> {
    content: TextContent<'a, Message>,
    theme: &'a Theme,
    href: Option<Cow<'a, str>>,
    color: Option<Color>,
    hover_color: Option<Color>,
    text_size: Option<f32>,
    line_height: Option<f32>,
    font: Option<Font>,
    width: Length,
    disabled: bool,
    on_press: Option<Message>,
    style_override: Option<
        Box<dyn Fn(button_widget::Style, button_widget::Status) -> button_widget::Style + 'a>,
    >,
}

impl<Message> fmt::Debug for BreadcrumbLink<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("BreadcrumbLink")
            .field("content", &self.content.kind())
            .field("theme", &self.theme)
            .field("href", &self.href.as_deref())
            .field("color", &self.color)
            .field("hover_color", &self.hover_color)
            .field("text_size", &self.text_size)
            .field("line_height", &self.line_height)
            .field("font", &self.font)
            .field("width", &self.width)
            .field("disabled", &self.disabled)
            .field("on_press", &self.on_press.is_some())
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

impl<'a, Message> BreadcrumbLink<'a, Message> {
    /// Creates a link from arbitrary iced content.
    pub fn new(content: impl Into<Element<'a, Message>>, theme: &'a Theme) -> Self {
        Self::from_content(TextContent::Element(content.into()), theme)
    }

    /// Creates a style-pack-aware text link.
    pub fn text(label: impl IntoFragment<'a>, theme: &'a Theme) -> Self {
        Self::from_content(TextContent::Label(label.into_fragment()), theme)
    }

    fn from_content(content: TextContent<'a, Message>, theme: &'a Theme) -> Self {
        Self {
            content,
            theme,
            href: None,
            color: None,
            hover_color: None,
            text_size: None,
            line_height: None,
            font: None,
            width: Length::Shrink,
            disabled: false,
            on_press: None,
            style_override: None,
        }
    }

    /// Records the target of the link (the web `href`).
    ///
    /// iced has no navigation model, so the value is carried for parity and
    /// for apps that route manually from [`Self::on_press`].
    pub fn href(mut self, href: impl Into<Cow<'a, str>>) -> Self {
        self.href = Some(href.into());
        self
    }

    /// Returns the recorded target of the link, if any.
    pub fn associated_href(&self) -> Option<&str> {
        self.href.as_deref()
    }

    /// Overrides the resting text color (defaults to the color inherited from
    /// the list).
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Overrides the hovered text color (defaults to the theme `foreground`).
    pub fn hover_color(mut self, hover_color: Color) -> Self {
        self.hover_color = Some(hover_color);
        self
    }

    /// Overrides the text size in px. Values are clamped to at least 1 px.
    pub fn text_size(mut self, text_size: f32) -> Self {
        self.text_size = Some(geometry::normalize_min_px(text_size));
        self
    }

    /// Overrides the line height in px. Values are clamped to at least 1 px.
    pub fn line_height(mut self, line_height: f32) -> Self {
        self.line_height = Some(geometry::normalize_min_px(line_height));
        self
    }

    /// Overrides the font (defaults to the theme sans face at `font-normal`).
    pub fn font(mut self, font: Font) -> Self {
        self.font = Some(font);
        self
    }

    /// Sets the link width (defaults to [`Length::Shrink`]).
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Suppresses [`Self::on_press`] while keeping the link's appearance.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Sets the message emitted when the link is pressed.
    pub fn on_press(mut self, message: Message) -> Self {
        self.on_press = Some(message);
        self
    }

    /// Sets or clears the message emitted when the link is pressed.
    pub fn on_press_maybe(mut self, message: Option<Message>) -> Self {
        self.on_press = message;
        self
    }

    /// Applies a narrow iced-style escape hatch after color resolution.
    pub fn style_override(
        mut self,
        style_override: impl Fn(button_widget::Style, button_widget::Status) -> button_widget::Style
        + 'a,
    ) -> Self {
        self.style_override = Some(Box::new(style_override));
        self
    }

    /// Builds the link as an iced [`Element`](iced_core::Element).
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        let inherited = render::inherited(self.theme);
        render::build_link(self, inherited)
    }
}

impl<'a, Message> From<BreadcrumbLink<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(link: BreadcrumbLink<'a, Message>) -> Self {
        link.into_element()
    }
}

/// Current page of the trail — port of the web `Breadcrumb.Page`.
///
/// Non-interactive text in the theme `foreground` at `font-normal`, matching
/// the web `role="link" aria-disabled="true" aria-current="page"` span.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct BreadcrumbPage<'a, Message> {
    content: TextContent<'a, Message>,
    theme: &'a Theme,
    color: Option<Color>,
    text_size: Option<f32>,
    line_height: Option<f32>,
    font: Option<Font>,
    width: Length,
    style_override: Option<Box<dyn Fn(container::Style) -> container::Style + 'a>>,
}

impl<Message> fmt::Debug for BreadcrumbPage<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("BreadcrumbPage")
            .field("content", &self.content.kind())
            .field("theme", &self.theme)
            .field("color", &self.color)
            .field("text_size", &self.text_size)
            .field("line_height", &self.line_height)
            .field("font", &self.font)
            .field("width", &self.width)
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

impl<'a, Message> BreadcrumbPage<'a, Message> {
    /// Creates a page marker from arbitrary iced content.
    pub fn new(content: impl Into<Element<'a, Message>>, theme: &'a Theme) -> Self {
        Self::from_content(TextContent::Element(content.into()), theme)
    }

    /// Creates a style-pack-aware text page marker.
    pub fn text(label: impl IntoFragment<'a>, theme: &'a Theme) -> Self {
        Self::from_content(TextContent::Label(label.into_fragment()), theme)
    }

    fn from_content(content: TextContent<'a, Message>, theme: &'a Theme) -> Self {
        Self {
            content,
            theme,
            color: None,
            text_size: None,
            line_height: None,
            font: None,
            width: Length::Shrink,
            style_override: None,
        }
    }

    /// Overrides the text color (defaults to the theme `foreground`).
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Overrides the text size in px. Values are clamped to at least 1 px.
    pub fn text_size(mut self, text_size: f32) -> Self {
        self.text_size = Some(geometry::normalize_min_px(text_size));
        self
    }

    /// Overrides the line height in px. Values are clamped to at least 1 px.
    pub fn line_height(mut self, line_height: f32) -> Self {
        self.line_height = Some(geometry::normalize_min_px(line_height));
        self
    }

    /// Overrides the font (defaults to the theme sans face at `font-normal`).
    pub fn font(mut self, font: Font) -> Self {
        self.font = Some(font);
        self
    }

    /// Sets the page-marker width (defaults to [`Length::Shrink`]).
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Applies a narrow iced-style escape hatch to the page surface.
    pub fn style_override(
        mut self,
        style_override: impl Fn(container::Style) -> container::Style + 'a,
    ) -> Self {
        self.style_override = Some(Box::new(style_override));
        self
    }

    /// Builds the page marker as an iced [`Element`](iced_core::Element).
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        let inherited = render::inherited(self.theme);
        render::build_page(self, inherited)
    }
}

impl<'a, Message> From<BreadcrumbPage<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(page: BreadcrumbPage<'a, Message>) -> Self {
        page.into_element()
    }
}

/// Divider between two steps — port of the web `Breadcrumb.Separator`.
///
/// Defaults to the Lucide `chevron-right` glyph drawn on a canvas at the
/// style-pack `size-3.5` footprint, mirroring the source component's icon
/// slot. Supply [`Self::text`] or [`Self::with_content`] for the web's
/// "children override the icon" behavior.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct BreadcrumbSeparator<'a, Message> {
    content: Option<TextContent<'a, Message>>,
    theme: &'a Theme,
    color: Option<Color>,
    icon_size: Option<f32>,
    text_size: Option<f32>,
    line_height: Option<f32>,
    font: Option<Font>,
    style_override: Option<Box<dyn Fn(container::Style) -> container::Style + 'a>>,
}

impl<Message> fmt::Debug for BreadcrumbSeparator<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let content = match &self.content {
            None => "chevron",
            Some(content) => content.kind(),
        };

        formatter
            .debug_struct("BreadcrumbSeparator")
            .field("content", &content)
            .field("theme", &self.theme)
            .field("color", &self.color)
            .field("icon_size", &self.icon_size)
            .field("text_size", &self.text_size)
            .field("line_height", &self.line_height)
            .field("font", &self.font)
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

impl<'a, Message> BreadcrumbSeparator<'a, Message> {
    /// Creates a separator drawing the default chevron glyph.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            content: None,
            theme,
            color: None,
            icon_size: None,
            text_size: None,
            line_height: None,
            font: None,
            style_override: None,
        }
    }

    /// Creates a separator from arbitrary iced content, replacing the glyph.
    pub fn with_content(content: impl Into<Element<'a, Message>>, theme: &'a Theme) -> Self {
        Self {
            content: Some(TextContent::Element(content.into())),
            ..Self::new(theme)
        }
    }

    /// Creates a text separator (for example `"/"`), replacing the glyph.
    pub fn text(label: impl IntoFragment<'a>, theme: &'a Theme) -> Self {
        Self {
            content: Some(TextContent::Label(label.into_fragment())),
            ..Self::new(theme)
        }
    }

    /// Whether the separator paints the default chevron glyph.
    pub const fn is_default_glyph(&self) -> bool {
        self.content.is_none()
    }

    /// Overrides the separator color (defaults to the color inherited from the
    /// list).
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Overrides the glyph footprint in px (defaults to the style-pack
    /// `size-3.5`). Values are clamped to at least 1 px and only apply to the
    /// default glyph.
    pub fn icon_size(mut self, icon_size: f32) -> Self {
        self.icon_size = Some(geometry::normalize_min_px(icon_size));
        self
    }

    /// Overrides the text size in px for text separators. Values are clamped
    /// to at least 1 px.
    pub fn text_size(mut self, text_size: f32) -> Self {
        self.text_size = Some(geometry::normalize_min_px(text_size));
        self
    }

    /// Overrides the line height in px for text separators. Values are clamped
    /// to at least 1 px.
    pub fn line_height(mut self, line_height: f32) -> Self {
        self.line_height = Some(geometry::normalize_min_px(line_height));
        self
    }

    /// Overrides the font for text separators.
    pub fn font(mut self, font: Font) -> Self {
        self.font = Some(font);
        self
    }

    /// Applies a narrow iced-style escape hatch to the separator surface.
    pub fn style_override(
        mut self,
        style_override: impl Fn(container::Style) -> container::Style + 'a,
    ) -> Self {
        self.style_override = Some(Box::new(style_override));
        self
    }

    /// Builds the separator as an iced [`Element`](iced_core::Element).
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        let inherited = render::inherited(self.theme);
        render::build_separator(self, inherited)
    }
}

impl<'a, Message> From<BreadcrumbSeparator<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(separator: BreadcrumbSeparator<'a, Message>) -> Self {
        separator.into_element()
    }
}

/// Collapsed-steps marker — port of the web `Breadcrumb.Ellipsis`.
///
/// Draws the Lucide `more-horizontal` glyph centered in the style-pack
/// `size-5` box (`size-4` for Mira). The web component is inert and is wrapped
/// in a dropdown trigger by the caller; [`Self::on_press`] is the iced
/// stand-in until the overlay-based menus land, and composing it inside a
/// [`crate::Button`] keeps working either way.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct BreadcrumbEllipsis<'a, Message> {
    theme: &'a Theme,
    color: Option<Color>,
    size: Option<f32>,
    icon_size: Option<f32>,
    screen_reader_label: Cow<'a, str>,
    on_press: Option<Message>,
    style_override: Option<EllipsisStyleOverride<'a>>,
}

enum EllipsisStyleOverride<'a> {
    Container(Box<dyn Fn(container::Style) -> container::Style + 'a>),
    Button(Box<dyn Fn(button_widget::Style, button_widget::Status) -> button_widget::Style + 'a>),
}

impl<Message> fmt::Debug for BreadcrumbEllipsis<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("BreadcrumbEllipsis")
            .field("theme", &self.theme)
            .field("color", &self.color)
            .field("size", &self.size)
            .field("icon_size", &self.icon_size)
            .field("screen_reader_label", &self.screen_reader_label.as_ref())
            .field("on_press", &self.on_press.is_some())
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

impl<'a, Message> BreadcrumbEllipsis<'a, Message> {
    /// Creates an ellipsis using the active style-pack defaults.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            color: None,
            size: None,
            icon_size: None,
            screen_reader_label: Cow::Borrowed("More"),
            on_press: None,
            style_override: None,
        }
    }

    /// Overrides the glyph color (defaults to the color inherited from the
    /// list).
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Overrides the box footprint in px (defaults to the style-pack `size-5`
    /// / `size-4`). Values are clamped to at least 1 px.
    pub fn size(mut self, size: f32) -> Self {
        self.size = Some(geometry::normalize_min_px(size));
        self
    }

    /// Overrides the glyph footprint in px (defaults to the style-pack
    /// `size-4` / `size-3.5`). Values are clamped to at least 1 px.
    pub fn icon_size(mut self, icon_size: f32) -> Self {
        self.icon_size = Some(geometry::normalize_min_px(icon_size));
        self
    }

    /// Sets the screen-reader-only label (defaults to `"More"`).
    ///
    /// Carried for parity with the web `sr-only` span; see the module docs.
    pub fn sr_label(mut self, sr_label: impl Into<Cow<'a, str>>) -> Self {
        self.screen_reader_label = sr_label.into();
        self
    }

    /// Returns the screen-reader-only label.
    pub fn screen_reader_label(&self) -> &str {
        self.screen_reader_label.as_ref()
    }

    /// Sets the message emitted when the ellipsis is pressed.
    pub fn on_press(mut self, message: Message) -> Self {
        self.on_press = Some(message);
        self
    }

    /// Sets or clears the message emitted when the ellipsis is pressed.
    pub fn on_press_maybe(mut self, message: Option<Message>) -> Self {
        self.on_press = message;
        self
    }

    /// Applies a narrow iced-style escape hatch for an inert ellipsis.
    ///
    /// Prefer [`Self::button_style_override`] once [`Self::on_press`] is set.
    pub fn style_override(
        mut self,
        style_override: impl Fn(container::Style) -> container::Style + 'a,
    ) -> Self {
        self.style_override = Some(EllipsisStyleOverride::Container(Box::new(style_override)));
        self
    }

    /// Applies a narrow iced-style escape hatch for a pressable ellipsis.
    pub fn button_style_override(
        mut self,
        style_override: impl Fn(button_widget::Style, button_widget::Status) -> button_widget::Style
        + 'a,
    ) -> Self {
        self.style_override = Some(EllipsisStyleOverride::Button(Box::new(style_override)));
        self
    }

    /// Builds the ellipsis as an iced [`Element`](iced_core::Element).
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        let inherited = render::inherited(self.theme);
        render::build_ellipsis(self, inherited)
    }
}

impl<'a, Message> From<BreadcrumbEllipsis<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(ellipsis: BreadcrumbEllipsis<'a, Message>) -> Self {
        ellipsis.into_element()
    }
}
