//! Builder-first responsive table component.
//!
//! This module mirrors shadcn-svelte's compositional table primitive:
//! [`Table`], [`TableHeader`], [`TableBody`], [`TableFooter`],
//! [`TableCaption`], [`TableRow`], [`TableHead`], and [`TableCell`]. The iced
//! API keeps the same slot boundaries while accepting arbitrary iced elements
//! as cell content.
//!
//! ```rust,no_run
//! use iced::Element;
//! use iced_shadcn_v2::{
//!     Table, TableBody, TableCaption, TableCell, TableFooter, TableHead,
//!     TableHeader, TableRow, Theme,
//! };
//!
//! fn view(theme: &Theme) -> Element<'_, ()> {
//!     Table::new(theme)
//!         .caption(TableCaption::text("Recent invoices", theme))
//!         .header(
//!             TableHeader::new(theme).push(
//!                 TableRow::new(theme)
//!                     .head(TableHead::text("Invoice", theme))
//!                     .head(TableHead::text("Status", theme)),
//!             ),
//!         )
//!         .body(TableBody::new(theme).push(
//!             TableRow::new(theme)
//!                 .cell(TableCell::text("INV001", theme))
//!                 .cell(TableCell::text("Paid", theme)),
//!         ))
//!         .footer(TableFooter::new(theme).push(
//!             TableRow::new(theme)
//!                 .cell(TableCell::text("Total", theme).span(1)),
//!         ))
//!         .into()
//! }
//! ```

mod render;
mod style;
mod types;

#[cfg(test)]
mod tests;

pub use types::TableCellAlignment;

use std::fmt;

use crate::iced_compat::alignment::{Horizontal, Vertical};
use crate::iced_compat::widget::container;
use crate::iced_compat::widget::text::{Fragment, IntoFragment};
use crate::iced_compat::{Color, Element, Font, Length, Padding};
use crate::theme::Theme;
use shadcn_common::FontWeight;

/// A private discriminator used while rendering typed table sections.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum SectionKind {
    Header,
    Body,
    Footer,
}

/// The content representation shared by text and arbitrary iced children.
enum TableContent<'a, Message> {
    Text(Fragment<'a>),
    Element(Element<'a, Message>),
}

impl<Message> TableContent<'_, Message> {
    fn kind(&self) -> &'static str {
        match self {
            Self::Text(_) => "text",
            Self::Element(_) => "element",
        }
    }
}

/// The shared configuration stored by [`TableCell`] and [`TableHead`].
struct TableCellConfig<'a, Message> {
    content: TableContent<'a, Message>,
    span: usize,
    width: Option<Length>,
    align_x: Horizontal,
    align_y: Vertical,
    padding: Option<Padding>,
    color: Option<Color>,
    font: Option<Font>,
    font_weight: Option<FontWeight>,
    text_size: Option<f32>,
    line_height: Option<f32>,
    style_override: Option<Box<dyn Fn(container::Style) -> container::Style + 'a>>,
}

fn new_cell_config<'a, Message>(
    content: TableContent<'a, Message>,
) -> TableCellConfig<'a, Message> {
    TableCellConfig {
        content,
        span: 1,
        width: None,
        align_x: Horizontal::Left,
        align_y: Vertical::Center,
        padding: None,
        color: None,
        font: None,
        font_weight: None,
        text_size: None,
        line_height: None,
        style_override: None,
    }
}

/// A regular body cell in a [`TableRow`].
#[must_use = "table builders do nothing unless turned into an iced Element"]
pub struct TableCell<'a, Message> {
    theme: &'a Theme,
    config: TableCellConfig<'a, Message>,
}

impl<Message> fmt::Debug for TableCell<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        debug_cell(formatter, "TableCell", self.theme, &self.config)
    }
}

/// A header cell in a [`TableRow`].
#[must_use = "table builders do nothing unless turned into an iced Element"]
pub struct TableHead<'a, Message> {
    theme: &'a Theme,
    config: TableCellConfig<'a, Message>,
}

impl<Message> fmt::Debug for TableHead<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        debug_cell(formatter, "TableHead", self.theme, &self.config)
    }
}

macro_rules! impl_cell_builder {
    ($name:ident, $is_header:expr) => {
        impl<'a, Message> $name<'a, Message> {
            /// Creates a cell from arbitrary iced content.
            #[must_use = "builder methods return modified table configuration"]
            pub fn new(content: impl Into<Element<'a, Message>>, theme: &'a Theme) -> Self {
                Self {
                    theme,
                    config: new_cell_config(TableContent::Element(content.into())),
                }
            }

            /// Creates a cell from a text fragment using table typography.
            #[must_use = "builder methods return modified table configuration"]
            pub fn text(content: impl IntoFragment<'a>, theme: &'a Theme) -> Self {
                Self {
                    theme,
                    config: new_cell_config(TableContent::Text(content.into_fragment())),
                }
            }

            /// Sets the number of logical columns occupied by this cell.
            ///
            /// A value of zero is normalized to one. This is the iced
            /// equivalent of the HTML `colspan` attribute.
            #[must_use = "builder methods return modified table configuration"]
            pub fn span(mut self, span: usize) -> Self {
                self.config.span = span.max(1);
                self
            }

            /// Sets an explicit cell width.
            #[must_use = "builder methods return modified table configuration"]
            pub fn width(mut self, width: impl Into<Length>) -> Self {
                self.config.width = Some(width.into());
                self
            }

            /// Sets the horizontal alignment of the cell content.
            #[must_use = "builder methods return modified table configuration"]
            pub fn align_x(mut self, alignment: impl Into<Horizontal>) -> Self {
                self.config.align_x = alignment.into();
                self
            }

            /// Sets the vertical alignment of the cell content.
            #[must_use = "builder methods return modified table configuration"]
            pub fn align_y(mut self, alignment: impl Into<Vertical>) -> Self {
                self.config.align_y = alignment.into();
                self
            }

            /// Sets custom cell padding.
            #[must_use = "builder methods return modified table configuration"]
            pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
                self.config.padding = Some(padding.into());
                self
            }

            /// Sets the cell foreground color.
            #[must_use = "builder methods return modified table configuration"]
            pub fn color(mut self, color: Color) -> Self {
                self.config.color = Some(color);
                self
            }

            /// Sets the font used for text content.
            #[must_use = "builder methods return modified table configuration"]
            pub fn font(mut self, font: Font) -> Self {
                self.config.font = Some(font);
                self
            }

            /// Sets the semantic font weight for text content.
            ///
            /// An explicit [`Self::font`] takes precedence when both are set.
            #[must_use = "builder methods return modified table configuration"]
            pub fn font_weight(mut self, weight: FontWeight) -> Self {
                self.config.font_weight = Some(weight);
                self
            }

            /// Sets the font size for text content in pixels.
            #[must_use = "builder methods return modified table configuration"]
            pub fn text_size(mut self, size: f32) -> Self {
                self.config.text_size = Some(style::normalize_min_px(size));
                self
            }

            /// Sets the line height for text content in pixels.
            #[must_use = "builder methods return modified table configuration"]
            pub fn line_height(mut self, line_height: f32) -> Self {
                self.config.line_height = Some(style::normalize_min_px(line_height));
                self
            }

            /// Applies an iced container-style override after semantic styling.
            #[must_use = "builder methods return modified table configuration"]
            pub fn style_override(
                mut self,
                style_override: impl Fn(container::Style) -> container::Style + 'a,
            ) -> Self {
                self.config.style_override = Some(Box::new(style_override));
                self
            }

            /// Builds this cell as a standalone iced element.
            #[must_use]
            pub fn into_element(self) -> Element<'a, Message>
            where
                Message: 'a,
            {
                render::build_standalone_cell(self.theme, self.config, $is_header)
            }
        }

        impl<'a, Message> From<$name<'a, Message>> for Element<'a, Message>
        where
            Message: 'a,
        {
            fn from(cell: $name<'a, Message>) -> Self {
                cell.into_element()
            }
        }
    };
}

impl_cell_builder!(TableCell, false);
impl_cell_builder!(TableHead, true);

/// A typed cell accepted by [`TableRow::push`].
#[non_exhaustive]
pub enum TableRowCell<'a, Message> {
    /// A body [`TableCell`].
    Cell(TableCell<'a, Message>),
    /// A header [`TableHead`].
    Head(TableHead<'a, Message>),
    /// Arbitrary content treated as one body cell.
    Element(Element<'a, Message>),
}

impl<Message> fmt::Debug for TableRowCell<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Cell(cell) => formatter.debug_tuple("Cell").field(cell).finish(),
            Self::Head(head) => formatter.debug_tuple("Head").field(head).finish(),
            Self::Element(_) => formatter.debug_tuple("Element").field(&"element").finish(),
        }
    }
}

impl<'a, Message> From<TableCell<'a, Message>> for TableRowCell<'a, Message> {
    fn from(cell: TableCell<'a, Message>) -> Self {
        Self::Cell(cell)
    }
}

impl<'a, Message> From<TableHead<'a, Message>> for TableRowCell<'a, Message> {
    fn from(head: TableHead<'a, Message>) -> Self {
        Self::Head(head)
    }
}

impl<'a, Message> From<Element<'a, Message>> for TableRowCell<'a, Message> {
    fn from(element: Element<'a, Message>) -> Self {
        Self::Element(element)
    }
}

/// A compositional table row containing body or header cells.
#[must_use = "table builders do nothing unless turned into an iced Element"]
pub struct TableRow<'a, Message> {
    theme: &'a Theme,
    cells: Vec<TableRowCell<'a, Message>>,
    selected: bool,
    hoverable: bool,
    height: Option<f32>,
    style_override: Option<Box<dyn Fn(container::Style) -> container::Style + 'a>>,
}

impl<Message> fmt::Debug for TableRow<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TableRow")
            .field("theme", &self.theme)
            .field("cells", &self.cells.len())
            .field("selected", &self.selected)
            .field("hoverable", &self.hoverable)
            .field("height", &self.height)
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

impl<'a, Message> TableRow<'a, Message> {
    /// Creates an empty table row.
    #[must_use = "builder methods return modified table configuration"]
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            cells: Vec::new(),
            selected: false,
            hoverable: true,
            height: None,
            style_override: None,
        }
    }

    /// Creates a row from an iterator of typed cells.
    #[must_use = "builder methods return modified table configuration"]
    pub fn with_cells<I, C>(theme: &'a Theme, cells: I) -> Self
    where
        I: IntoIterator<Item = C>,
        C: Into<TableRowCell<'a, Message>>,
    {
        Self::new(theme).extend(cells)
    }

    /// Appends a body or header cell.
    #[must_use = "builder methods return modified table configuration"]
    pub fn push(mut self, cell: impl Into<TableRowCell<'a, Message>>) -> Self {
        self.cells.push(cell.into());
        self
    }

    /// Appends a body cell.
    #[must_use = "builder methods return modified table configuration"]
    pub fn cell(self, cell: TableCell<'a, Message>) -> Self {
        self.push(cell)
    }

    /// Appends a header cell.
    #[must_use = "builder methods return modified table configuration"]
    pub fn head(self, head: TableHead<'a, Message>) -> Self {
        self.push(head)
    }

    /// Appends arbitrary content as a body cell.
    #[must_use = "builder methods return modified table configuration"]
    pub fn push_element(self, element: impl Into<Element<'a, Message>>) -> Self {
        self.push(TableRowCell::Element(element.into()))
    }

    /// Appends every cell from an iterator.
    #[must_use = "builder methods return modified table configuration"]
    pub fn extend<I, C>(mut self, cells: I) -> Self
    where
        I: IntoIterator<Item = C>,
        C: Into<TableRowCell<'a, Message>>,
    {
        self.cells.extend(cells.into_iter().map(Into::into));
        self
    }

    /// Marks the row as selected, using the semantic muted surface.
    #[must_use = "builder methods return modified table configuration"]
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    /// Enables or disables the row hover surface.
    #[must_use = "builder methods return modified table configuration"]
    pub fn hoverable(mut self, hoverable: bool) -> Self {
        self.hoverable = hoverable;
        self
    }

    /// Sets an explicit row height in pixels.
    #[must_use = "builder methods return modified table configuration"]
    pub fn height(mut self, height: f32) -> Self {
        self.height = Some(style::normalize_min_px(height));
        self
    }

    /// Applies an iced container-style override after row styling.
    #[must_use = "builder methods return modified table configuration"]
    pub fn style_override(
        mut self,
        style_override: impl Fn(container::Style) -> container::Style + 'a,
    ) -> Self {
        self.style_override = Some(Box::new(style_override));
        self
    }

    /// Returns the number of cells in the row.
    #[must_use]
    pub fn len(&self) -> usize {
        self.cells.len()
    }

    /// Returns whether the row has no cells.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.cells.is_empty()
    }
}

impl<'a, Message> From<TableRow<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(row: TableRow<'a, Message>) -> Self {
        render::build_standalone_row(row)
    }
}

/// A section builder used by [`TableHeader`], [`TableBody`], and
/// [`TableFooter`].
#[must_use = "table builders do nothing unless turned into an iced Element"]
pub struct TableSection<'a, Message> {
    theme: &'a Theme,
    rows: Vec<TableRow<'a, Message>>,
    style_override: Option<Box<dyn Fn(container::Style) -> container::Style + 'a>>,
}

impl<Message> fmt::Debug for TableSection<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TableSection")
            .field("theme", &self.theme)
            .field("rows", &self.rows.len())
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

impl<'a, Message> TableSection<'a, Message> {
    /// Creates an empty table section.
    #[must_use = "builder methods return modified table configuration"]
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            rows: Vec::new(),
            style_override: None,
        }
    }

    /// Creates a section from an iterator of rows.
    #[must_use = "builder methods return modified table configuration"]
    pub fn with_rows(
        theme: &'a Theme,
        rows: impl IntoIterator<Item = TableRow<'a, Message>>,
    ) -> Self {
        Self {
            theme,
            rows: rows.into_iter().collect(),
            style_override: None,
        }
    }

    /// Appends a row to the section.
    #[must_use = "builder methods return modified table configuration"]
    pub fn push(mut self, row: TableRow<'a, Message>) -> Self {
        self.rows.push(row);
        self
    }

    /// Appends every row from an iterator.
    #[must_use = "builder methods return modified table configuration"]
    pub fn extend(mut self, rows: impl IntoIterator<Item = TableRow<'a, Message>>) -> Self {
        self.rows.extend(rows);
        self
    }

    /// Applies an iced container-style override to the section wrapper.
    #[must_use = "builder methods return modified table configuration"]
    pub fn style_override(
        mut self,
        style_override: impl Fn(container::Style) -> container::Style + 'a,
    ) -> Self {
        self.style_override = Some(Box::new(style_override));
        self
    }

    /// Returns the number of rows in the section.
    #[must_use]
    pub fn len(&self) -> usize {
        self.rows.len()
    }

    /// Returns whether the section has no rows.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    /// Builds the section as a standalone body-style element.
    #[must_use]
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        render::build_standalone_section(self)
    }
}

impl<'a, Message> From<TableSection<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(section: TableSection<'a, Message>) -> Self {
        section.into_element()
    }
}

/// The table header slot. Use [`TableHeader::new`] and add [`TableRow`]s.
pub type TableHeader<'a, Message> = TableSection<'a, Message>;

/// The table body slot. Use [`TableBody::new`] and add [`TableRow`]s.
pub type TableBody<'a, Message> = TableSection<'a, Message>;

/// The table footer slot. Use [`TableFooter::new`] and add [`TableRow`]s.
pub type TableFooter<'a, Message> = TableSection<'a, Message>;

/// A caption rendered below the table, matching CSS `caption-side: bottom`.
#[must_use = "table builders do nothing unless turned into an iced Element"]
pub struct TableCaption<'a, Message> {
    theme: &'a Theme,
    content: TableContent<'a, Message>,
    width: Length,
    align_x: Horizontal,
    color: Option<Color>,
    font: Option<Font>,
    font_weight: Option<FontWeight>,
    text_size: Option<f32>,
    line_height: Option<f32>,
    margin_top: Option<f32>,
    style_override: Option<Box<dyn Fn(container::Style) -> container::Style + 'a>>,
}

impl<Message> fmt::Debug for TableCaption<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TableCaption")
            .field("theme", &self.theme)
            .field("content", &self.content.kind())
            .field("width", &self.width)
            .field("align_x", &self.align_x)
            .field("color", &self.color)
            .field("font", &self.font)
            .field("font_weight", &self.font_weight)
            .field("text_size", &self.text_size)
            .field("line_height", &self.line_height)
            .field("margin_top", &self.margin_top)
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

impl<'a, Message> TableCaption<'a, Message> {
    /// Creates a caption from arbitrary iced content.
    #[must_use = "builder methods return modified table configuration"]
    pub fn new(content: impl Into<Element<'a, Message>>, theme: &'a Theme) -> Self {
        Self {
            theme,
            content: TableContent::Element(content.into()),
            width: Length::Fill,
            align_x: Horizontal::Center,
            color: None,
            font: None,
            font_weight: None,
            text_size: None,
            line_height: None,
            margin_top: None,
            style_override: None,
        }
    }

    /// Creates a text caption using the table's typography.
    #[must_use = "builder methods return modified table configuration"]
    pub fn text(content: impl IntoFragment<'a>, theme: &'a Theme) -> Self {
        Self {
            theme,
            content: TableContent::Text(content.into_fragment()),
            width: Length::Fill,
            align_x: Horizontal::Center,
            color: None,
            font: None,
            font_weight: None,
            text_size: None,
            line_height: None,
            margin_top: None,
            style_override: None,
        }
    }

    /// Sets the caption width.
    #[must_use = "builder methods return modified table configuration"]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the horizontal caption alignment.
    #[must_use = "builder methods return modified table configuration"]
    pub fn align_x(mut self, alignment: impl Into<Horizontal>) -> Self {
        self.align_x = alignment.into();
        self
    }

    /// Sets the caption foreground color.
    #[must_use = "builder methods return modified table configuration"]
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Sets the caption font.
    #[must_use = "builder methods return modified table configuration"]
    pub fn font(mut self, font: Font) -> Self {
        self.font = Some(font);
        self
    }

    /// Sets the semantic font weight for a text caption.
    ///
    /// An explicit [`Self::font`] takes precedence when both are set.
    #[must_use = "builder methods return modified table configuration"]
    pub fn font_weight(mut self, weight: FontWeight) -> Self {
        self.font_weight = Some(weight);
        self
    }

    /// Sets the caption font size in pixels.
    #[must_use = "builder methods return modified table configuration"]
    pub fn text_size(mut self, size: f32) -> Self {
        self.text_size = Some(style::normalize_min_px(size));
        self
    }

    /// Sets the caption line height in pixels.
    #[must_use = "builder methods return modified table configuration"]
    pub fn line_height(mut self, line_height: f32) -> Self {
        self.line_height = Some(style::normalize_min_px(line_height));
        self
    }

    /// Overrides the default sixteen-pixel top margin.
    #[must_use = "builder methods return modified table configuration"]
    pub fn margin_top(mut self, margin_top: f32) -> Self {
        self.margin_top = Some(style::normalize_px(margin_top));
        self
    }

    /// Applies an iced container-style override after semantic styling.
    #[must_use = "builder methods return modified table configuration"]
    pub fn style_override(
        mut self,
        style_override: impl Fn(container::Style) -> container::Style + 'a,
    ) -> Self {
        self.style_override = Some(Box::new(style_override));
        self
    }

    /// Builds the caption as an iced element.
    #[must_use]
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        render::build_caption(self)
    }
}

impl<'a, Message> From<TableCaption<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(caption: TableCaption<'a, Message>) -> Self {
        caption.into_element()
    }
}

/// A complete responsive table assembled from typed slots.
#[must_use = "table builders do nothing unless turned into an iced Element"]
pub struct Table<'a, Message> {
    theme: &'a Theme,
    caption: Option<TableCaption<'a, Message>>,
    sections: Vec<TableRootSection<'a, Message>>,
    width: Length,
    min_width: f32,
    column_widths: Vec<Length>,
    style_override: Option<Box<dyn Fn(container::Style) -> container::Style + 'a>>,
}

impl<Message> fmt::Debug for Table<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Table")
            .field("theme", &self.theme)
            .field("caption", &self.caption.is_some())
            .field("sections", &self.sections.len())
            .field("width", &self.width)
            .field("min_width", &self.min_width)
            .field("column_widths", &self.column_widths)
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

/// A typed section owned by [`Table`].
enum TableRootSection<'a, Message> {
    Header(TableSection<'a, Message>),
    Body(TableSection<'a, Message>),
    Footer(TableSection<'a, Message>),
}

impl<'a, Message> Table<'a, Message> {
    /// Creates an empty table using the active style-pack defaults.
    #[must_use = "builder methods return modified table configuration"]
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            caption: None,
            sections: Vec::new(),
            width: Length::Fill,
            min_width: 0.0,
            column_widths: Vec::new(),
            style_override: None,
        }
    }

    /// Sets the caption slot. Captions are always rendered below the rows.
    #[must_use = "builder methods return modified table configuration"]
    pub fn caption(mut self, caption: TableCaption<'a, Message>) -> Self {
        self.caption = Some(caption);
        self
    }

    /// Adds a header section.
    #[must_use = "builder methods return modified table configuration"]
    pub fn header(mut self, header: TableHeader<'a, Message>) -> Self {
        self.sections.push(TableRootSection::Header(header));
        self
    }

    /// Adds a body section.
    #[must_use = "builder methods return modified table configuration"]
    pub fn body(mut self, body: TableBody<'a, Message>) -> Self {
        self.sections.push(TableRootSection::Body(body));
        self
    }

    /// Adds a footer section.
    #[must_use = "builder methods return modified table configuration"]
    pub fn footer(mut self, footer: TableFooter<'a, Message>) -> Self {
        self.sections.push(TableRootSection::Footer(footer));
        self
    }

    /// Sets the outer table width.
    #[must_use = "builder methods return modified table configuration"]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the minimum scrollable content width in pixels.
    ///
    /// A positive value enables horizontal scrolling when the table is wider
    /// than its outer width. The default zero keeps `Fill` columns responsive
    /// to the available layout width.
    #[must_use = "builder methods return modified table configuration"]
    pub fn min_width(mut self, width: f32) -> Self {
        self.min_width = style::normalize_px(width);
        self
    }

    /// Sets the logical width of each column.
    ///
    /// `Length::Fixed` is useful for a leading identifier column, while
    /// `Length::Fill` or `Length::FillPortion` distributes the remaining
    /// width. Spanning cells use the sum when all covered columns are fixed or
    /// fluid; mixed spans fall back to a fluid portion.
    #[must_use = "builder methods return modified table configuration"]
    pub fn column_widths<I>(mut self, widths: I) -> Self
    where
        I: IntoIterator<Item = Length>,
    {
        self.column_widths = widths.into_iter().collect();
        self
    }

    /// Sets one logical column width, filling omitted preceding columns.
    #[must_use = "builder methods return modified table configuration"]
    pub fn column_width(mut self, index: usize, width: impl Into<Length>) -> Self {
        let Some(required_len) = index.checked_add(1) else {
            return self;
        };
        if self.column_widths.len() < required_len {
            self.column_widths.resize(required_len, Length::Fill);
        }
        self.column_widths[index] = width.into();
        self
    }

    /// Applies an iced container-style override to the table root.
    #[must_use = "builder methods return modified table configuration"]
    pub fn style_override(
        mut self,
        style_override: impl Fn(container::Style) -> container::Style + 'a,
    ) -> Self {
        self.style_override = Some(Box::new(style_override));
        self
    }

    /// Builds the table as an iced element.
    #[must_use]
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        render::build_table(self)
    }
}

impl<'a, Message> From<Table<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(table: Table<'a, Message>) -> Self {
        table.into_element()
    }
}

fn debug_cell<Message>(
    formatter: &mut fmt::Formatter<'_>,
    name: &'static str,
    theme: &Theme,
    config: &TableCellConfig<'_, Message>,
) -> fmt::Result {
    formatter
        .debug_struct(name)
        .field("theme", theme)
        .field("content", &config.content.kind())
        .field("span", &config.span)
        .field("width", &config.width)
        .field("align_x", &config.align_x)
        .field("align_y", &config.align_y)
        .field("padding", &config.padding)
        .field("color", &config.color)
        .field("font", &config.font)
        .field("font_weight", &config.font_weight)
        .field("text_size", &config.text_size)
        .field("line_height", &config.line_height)
        .field("style_override", &config.style_override.is_some())
        .finish()
}
