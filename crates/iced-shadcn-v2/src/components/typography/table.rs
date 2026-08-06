//! Prose table from the shadcn typography page (`typography-table`).

use std::fmt;

use crate::iced_compat::alignment::Horizontal;
use crate::iced_compat::widget::text::{Fragment, IntoFragment, LineHeight};
use crate::iced_compat::widget::{column, container, row, text as iced_text};
use crate::iced_compat::{Background, Border, Color, Element, Length, Padding};

use super::render::{horizontal_rule, vertical_rule};
use super::style::{RULE_PX, TABLE_CELL_PADDING_X_PX, TABLE_CELL_PADDING_Y_PX};
use super::types::TypographyVariant;
use crate::fonts::iced_font;
use crate::recipes::iced_font_weight;
use crate::theme::Theme;
use shadcn_common::FontWeight;

/// Builder-first prose table (bordered grid, bold header, striped rows).
///
/// Mirrors the `typography-table` example: 1 px grid in the theme border
/// color, `px-4 py-2` cells, `font-bold` header, and `even:bg-muted` striping
/// on body rows. Columns share the width evenly and default to start
/// alignment (`text-start`), like the web version. The column count follows
/// the widest row (header or body); shorter rows are padded with empty cells.
///
/// ```rust,no_run
/// use iced::Element;
/// use iced_shadcn_v2::{Theme, TypographyTable};
///
/// fn view(theme: &Theme) -> Element<'_, ()> {
///     TypographyTable::new(theme)
///         .header(["King's Treasury", "People's happiness"])
///         .row(["Empty", "Overflowing"])
///         .row(["Modest", "Satisfied"])
///         .into()
/// }
/// ```
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct TypographyTable<'a, Message> {
    header: Option<Vec<Fragment<'a>>>,
    rows: Vec<Vec<Fragment<'a>>>,
    theme: &'a Theme,
    color: Option<Color>,
    width: Length,
    align_columns: Vec<Horizontal>,
    striped: bool,
    _message: std::marker::PhantomData<Message>,
}

impl<Message> fmt::Debug for TypographyTable<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TypographyTable")
            .field("header", &self.header.is_some())
            .field("rows", &self.rows.len())
            .field("theme", &self.theme)
            .field("color", &self.color)
            .field("width", &self.width)
            .field("align_columns", &self.align_columns)
            .field("striped", &self.striped)
            .finish()
    }
}

impl<'a, Message> TypographyTable<'a, Message> {
    /// Creates an empty table.
    ///
    /// `theme` is required because typography and color resolve from
    /// `shadcn-common` theme tokens instead of `iced::Theme`.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            header: None,
            rows: Vec::new(),
            theme,
            color: None,
            width: Length::Fill,
            align_columns: Vec::new(),
            striped: true,
            _message: std::marker::PhantomData,
        }
    }

    /// Sets the bold header row (`<thead>`).
    pub fn header<I>(mut self, cells: I) -> Self
    where
        I: IntoIterator,
        I::Item: IntoFragment<'a>,
    {
        self.header = Some(cells.into_iter().map(IntoFragment::into_fragment).collect());
        self
    }

    /// Appends a body row (`<tbody><tr>`).
    pub fn row<I>(mut self, cells: I) -> Self
    where
        I: IntoIterator,
        I::Item: IntoFragment<'a>,
    {
        self.rows
            .push(cells.into_iter().map(IntoFragment::into_fragment).collect());
        self
    }

    /// Overrides the cell text color (defaults to theme `foreground`).
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Sets a custom table width (defaults to fill, like `w-full`).
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets per-column text alignment (the `[&[align=…]]` escape hatch).
    ///
    /// Missing trailing columns keep the default start alignment.
    pub fn align_columns<I>(mut self, alignments: I) -> Self
    where
        I: IntoIterator<Item = Horizontal>,
    {
        self.align_columns = alignments.into_iter().collect();
        self
    }

    /// Enables or disables `even:bg-muted` striping on body rows.
    pub fn striped(mut self, striped: bool) -> Self {
        self.striped = striped;
        self
    }

    /// Number of body rows appended so far.
    pub fn len(&self) -> usize {
        self.rows.len()
    }

    /// Whether no body rows have been appended.
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    /// Builds the table as an iced [`Element`](iced_core::Element).
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        let Self {
            header,
            rows,
            theme,
            color,
            width,
            align_columns,
            striped,
            _message,
        } = self;

        let border_color = theme.palette.border;
        let muted = theme.palette.muted;
        let text_color = color.unwrap_or(theme.palette.foreground);
        let font = iced_font(theme.font_pack().sans);
        let mut bold_font = font;
        bold_font.weight = iced_font_weight(FontWeight::Bold);
        let size = TypographyVariant::P.type_recipe().size_px;

        let columns = column_count(header.as_ref().map(Vec::len), rows.iter().map(Vec::len));

        let mut lines: Vec<Element<'a, Message>> = Vec::new();

        if let Some(cells) = header {
            lines.push(table_row(
                cells,
                columns,
                &align_columns,
                bold_font,
                size,
                text_color,
                None,
                border_color,
            ));
        }

        for (index, cells) in rows.into_iter().enumerate() {
            if !lines.is_empty() || index > 0 {
                lines.push(horizontal_rule(border_color));
            }
            // Web `even:bg-muted` is 1-based: the 2nd, 4th, … body rows.
            let background = (striped && index % 2 == 1).then_some(muted);
            lines.push(table_row(
                cells,
                columns,
                &align_columns,
                font,
                size,
                text_color,
                background,
                border_color,
            ));
        }

        container(column(lines).width(Length::Fill))
            .width(width)
            .style(move |_| container::Style {
                border: Border {
                    color: border_color,
                    width: RULE_PX,
                    ..Border::default()
                },
                ..container::Style::default()
            })
            .into()
    }
}

pub(super) fn column_count(
    header_len: Option<usize>,
    row_lengths: impl IntoIterator<Item = usize>,
) -> usize {
    header_len
        .into_iter()
        .chain(row_lengths)
        .max()
        .unwrap_or(0)
        .max(1)
}

#[allow(clippy::too_many_arguments)]
fn table_row<'a, Message: 'a>(
    cells: Vec<Fragment<'a>>,
    columns: usize,
    align_columns: &[Horizontal],
    font: crate::iced_compat::Font,
    size: f32,
    text_color: Color,
    background: Option<Color>,
    border_color: Color,
) -> Element<'a, Message> {
    let mut cells = cells;
    cells.resize_with(columns, || Fragment::from(""));

    let mut children: Vec<Element<'a, Message>> = Vec::with_capacity(columns * 2);

    for (index, cell) in cells.into_iter().enumerate() {
        if index > 0 {
            children.push(vertical_rule(border_color));
        }

        let align = align_columns
            .get(index)
            .copied()
            .unwrap_or(Horizontal::Left);
        let text = iced_text(cell)
            .size(size)
            .line_height(LineHeight::Absolute((size * 1.5).into()))
            .font(font)
            .color(text_color)
            .width(Length::Fill)
            .align_x(align);

        children.push(
            container(text)
                .width(Length::FillPortion(1))
                .padding(Padding {
                    top: TABLE_CELL_PADDING_Y_PX,
                    bottom: TABLE_CELL_PADDING_Y_PX,
                    left: TABLE_CELL_PADDING_X_PX,
                    right: TABLE_CELL_PADDING_X_PX,
                })
                .into(),
        );
    }

    let body = row(children).width(Length::Fill);

    if let Some(background) = background {
        container(body)
            .width(Length::Fill)
            .style(move |_| container::Style {
                background: Some(Background::Color(background)),
                ..container::Style::default()
            })
            .into()
    } else {
        body.into()
    }
}

impl<'a, Message> From<TypographyTable<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(table: TypographyTable<'a, Message>) -> Self {
        table.into_element()
    }
}
