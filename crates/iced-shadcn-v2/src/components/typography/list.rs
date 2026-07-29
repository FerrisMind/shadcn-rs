//! Bulleted list from the shadcn typography page (`typography-list`).

use std::fmt;

use crate::iced_compat::widget::text::{Fragment, IntoFragment, LineHeight};
use crate::iced_compat::widget::{column, row, text as iced_text};
use crate::iced_compat::{Color, Element, Length, Padding};

use super::style::{LIST_INDENT_PX, LIST_ITEM_GAP_PX, LIST_MARKER, LIST_MARKER_GAP_PX};
use super::types::TypographyVariant;
use crate::fonts::iced_font;
use crate::theme::Theme;

/// Builder-first disc list (`my-6 ms-6 list-disc [&>li]:mt-2`).
///
/// Items render at paragraph metrics ([`TypographyVariant::P`] size with the
/// web default `leading-normal`) behind a `•` marker; custom elements can be
/// mixed in via [`Self::item_element`].
///
/// ```rust,no_run
/// use iced::Element;
/// use iced_shadcn_v2::{Theme, TypographyList};
///
/// fn view(theme: &Theme) -> Element<'_, ()> {
///     TypographyList::new(theme)
///         .item("1st level of puns: 5 gold coins")
///         .item("2nd level of jokes: 10 gold coins")
///         .into()
/// }
/// ```
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct TypographyList<'a, Message> {
    items: Vec<ListItem<'a, Message>>,
    theme: &'a Theme,
    color: Option<Color>,
    width: Length,
    indent: f32,
}

enum ListItem<'a, Message> {
    Text(Fragment<'a>),
    Element(Element<'a, Message>),
}

impl<Message> fmt::Debug for TypographyList<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TypographyList")
            .field("items", &self.items.len())
            .field("theme", &self.theme)
            .field("color", &self.color)
            .field("width", &self.width)
            .field("indent", &self.indent)
            .finish()
    }
}

impl<'a, Message> TypographyList<'a, Message> {
    /// Creates an empty list.
    ///
    /// `theme` is required because typography and color resolve from
    /// `shadcn-common` theme tokens instead of `iced::Theme`.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            items: Vec::new(),
            theme,
            color: None,
            width: Length::Fill,
            indent: LIST_INDENT_PX,
        }
    }

    /// Appends a text item.
    pub fn item(mut self, item: impl IntoFragment<'a>) -> Self {
        self.items.push(ListItem::Text(item.into_fragment()));
        self
    }

    /// Appends an arbitrary element behind a disc marker.
    pub fn item_element(mut self, item: impl Into<Element<'a, Message>>) -> Self {
        self.items.push(ListItem::Element(item.into()));
        self
    }

    /// Appends several text items at once.
    pub fn items<I>(mut self, items: I) -> Self
    where
        I: IntoIterator,
        I::Item: IntoFragment<'a>,
    {
        for item in items {
            self.items.push(ListItem::Text(item.into_fragment()));
        }
        self
    }

    /// Overrides the item text color (defaults to theme `foreground`).
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Sets a custom list width (defaults to fill, like block-level `<ul>`).
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Overrides the leading indent in px (`ms-6` → 24 px by default).
    pub fn indent(mut self, indent: f32) -> Self {
        self.indent = indent.max(0.0);
        self
    }

    /// Number of items appended so far.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Whether no items have been appended.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Builds the list as an iced [`Element`](iced_core::Element).
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        let Self {
            items,
            theme,
            color,
            width,
            indent,
        } = self;

        // Web `<li>` text: paragraph size with the browser default line height.
        let recipe = TypographyVariant::P.type_recipe();
        let size = recipe.size_px;
        let line_height = LineHeight::Absolute((size * 1.5).into());
        let font = iced_font(theme.font_pack().sans);
        let text_color = color.unwrap_or(theme.palette.foreground);

        let rows = items.into_iter().map(|item| {
            let marker = iced_text(LIST_MARKER)
                .size(size)
                .line_height(line_height)
                .font(font)
                .color(text_color);

            let content: Element<'a, Message> = match item {
                ListItem::Text(fragment) => iced_text(fragment)
                    .size(size)
                    .line_height(line_height)
                    .font(font)
                    .color(text_color)
                    .width(Length::Fill)
                    .into(),
                ListItem::Element(element) => element,
            };

            row![marker, content]
                .spacing(LIST_MARKER_GAP_PX)
                .width(Length::Fill)
                .into()
        });

        column(rows)
            .spacing(LIST_ITEM_GAP_PX)
            .padding(Padding {
                left: indent,
                ..Padding::ZERO
            })
            .width(width)
            .into()
    }
}

impl<'a, Message> From<TypographyList<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(list: TypographyList<'a, Message>) -> Self {
        list.into_element()
    }
}
