//! Layout and rendering for the table component.

use crate::iced_compat::advanced::layout::{self, Layout};
use crate::iced_compat::advanced::widget::{Operation, Tree, tree};
use crate::iced_compat::advanced::{Clipboard, Shell, Widget, overlay, renderer};
use crate::iced_compat::widget::text::LineHeight;
use crate::iced_compat::widget::{Space, column, container, row, scrollable, text as iced_text};
use crate::iced_compat::{
    Background, Color, Element, Event, Length, Padding, Rectangle, Size, Vector, mouse, window,
};
use crate::theme::Theme;
use iced_core::Renderer as _;
use iced_core::text::Wrapping;
use shadcn_common::FontWeight;

use super::{
    SectionKind, Table, TableCaption, TableCellConfig, TableContent, TableRootSection, TableRow,
    TableRowCell, TableSection, style,
};

/// Builds the complete table root, including the optional horizontal overflow
/// wrapper and the bottom caption.
pub(super) fn build_table<'a, Message>(table: Table<'a, Message>) -> Element<'a, Message>
where
    Message: 'a,
{
    let Table {
        theme,
        caption,
        sections,
        width,
        min_width,
        column_widths,
        style_override,
    } = table;

    let columns = sections
        .iter()
        .map(root_section_column_count)
        .max()
        .unwrap_or(1);

    let content_width = if min_width > 0.0 {
        Length::Fixed(min_width)
    } else {
        Length::Fill
    };

    let mut section_elements = Vec::with_capacity(sections.len());
    for section in sections {
        let element = match section {
            TableRootSection::Header(section) => {
                build_section(section, SectionKind::Header, &column_widths, columns)
            }
            TableRootSection::Body(section) => {
                build_section(section, SectionKind::Body, &column_widths, columns)
            }
            TableRootSection::Footer(section) => {
                build_section(section, SectionKind::Footer, &column_widths, columns)
            }
        };
        section_elements.push(element);
    }

    let table_content =
        container(column(section_elements).spacing(0).width(Length::Fill)).width(content_width);
    // A horizontal `scrollable` lays out its child with an infinite width.
    // Keep the responsive/default path as a normal bounded layout so `Fill`
    // columns resolve to the available viewport; an explicit minimum width is
    // the opt-in native equivalent of the web component's overflow wrapper.
    let table_content: Element<'a, Message> = if min_width > 0.0 {
        scrollable(table_content).horizontal().width(width).into()
    } else {
        table_content.width(width).into()
    };

    let mut children = vec![table_content];
    if let Some(caption) = caption {
        children.push(build_caption(caption));
    }

    let body = column(children).spacing(0).width(width);
    let mut resolved = container::Style {
        text_color: Some(theme.palette.foreground),
        ..container::Style::default()
    };
    if let Some(style_override) = style_override {
        resolved = style_override(resolved);
    }

    container(body).width(width).style(move |_| resolved).into()
}

/// Builds a section with row-aware border and footer handling.
pub(super) fn build_section<'a, Message>(
    section: TableSection<'a, Message>,
    kind: SectionKind,
    column_widths: &[Length],
    columns: usize,
) -> Element<'a, Message>
where
    Message: 'a,
{
    let TableSection {
        theme,
        rows,
        style_override,
    } = section;

    let row_count = rows.len();
    let resolved_widths = if column_widths.is_empty() {
        vec![Length::Fill; columns.max(1)]
    } else {
        column_widths.to_vec()
    };
    let mut rendered_rows = Vec::with_capacity(row_count);

    for (index, row) in rows.into_iter().enumerate() {
        rendered_rows.push(build_row(
            theme,
            row,
            kind,
            index + 1 == row_count,
            &resolved_widths,
        ));
    }

    let section_body = column(rendered_rows).spacing(0).width(Length::Fill);
    let section_body: Element<'a, Message> = if kind == SectionKind::Footer {
        column![horizontal_rule(theme.palette.border), section_body]
            .spacing(0)
            .width(Length::Fill)
            .into()
    } else {
        section_body.into()
    };

    let mut resolved = container::Style {
        text_color: Some(theme.palette.foreground),
        ..container::Style::default()
    };
    if let Some(style_override) = style_override {
        resolved = style_override(resolved);
    }

    container(section_body)
        .width(Length::Fill)
        .style(move |_| resolved)
        .into()
}

/// Builds one row with equal/fill portions or explicit root column widths.
fn build_row<'a, Message>(
    theme: &'a Theme,
    table_row: TableRow<'a, Message>,
    kind: SectionKind,
    is_last: bool,
    column_widths: &[Length],
) -> Element<'a, Message>
where
    Message: 'a,
{
    let TableRow {
        theme: _row_theme,
        cells,
        selected,
        hoverable,
        height,
        style_override,
    } = table_row;

    let has_bottom_border = match kind {
        SectionKind::Header => true,
        SectionKind::Body => !is_last,
        SectionKind::Footer => false,
    };

    let mut rendered_cells = Vec::with_capacity(cells.len());
    let mut column = 0usize;

    for cell in cells {
        match cell {
            TableRowCell::Cell(cell) => {
                let span = cell.config.span.max(1);
                let width = resolve_cell_width(cell.config.width, column_widths, column, span);
                rendered_cells.push(build_cell(
                    theme,
                    cell.theme,
                    cell.config,
                    false,
                    kind,
                    width,
                ));
                column = column.saturating_add(span);
            }
            TableRowCell::Head(head) => {
                let span = head.config.span.max(1);
                let width = resolve_cell_width(head.config.width, column_widths, column, span);
                rendered_cells.push(build_cell(
                    theme,
                    head.theme,
                    head.config,
                    true,
                    kind,
                    width,
                ));
                column = column.saturating_add(span);
            }
            TableRowCell::Element(element) => {
                let width = resolve_cell_width(None, column_widths, column, 1);
                rendered_cells.push(build_cell(
                    theme,
                    theme,
                    super::new_cell_config(TableContent::Element(element)),
                    false,
                    kind,
                    width,
                ));
                column = column.saturating_add(1);
            }
        }
    }

    let mut body = row(rendered_cells)
        .spacing(0)
        .align_y(crate::iced_compat::alignment::Vertical::Center)
        .width(Length::Fill);
    if let Some(height) = height {
        body = body.height(Length::Fixed(height));
    }

    let mut base = style::row_style(theme, kind, selected, has_bottom_border);
    let mut hovered = style::hover_row_style(theme, kind, selected, has_bottom_border);
    if let Some(style_override) = style_override {
        base = style_override(base);
        hovered = style_override(hovered);
    }

    Element::new(HoverRow {
        content: body.into(),
        base,
        hovered,
        hoverable,
    })
}

/// Builds a cell wrapper and applies the source style-pack typography.
fn build_cell<'a, Message>(
    table_theme: &'a Theme,
    cell_theme: &'a Theme,
    config: TableCellConfig<'a, Message>,
    is_header: bool,
    section: SectionKind,
    width: Length,
) -> Element<'a, Message>
where
    Message: 'a,
{
    let TableCellConfig {
        content,
        span: _,
        width: _,
        align_x,
        align_y,
        padding,
        color,
        font,
        font_weight,
        text_size,
        line_height,
        style_override,
    } = config;

    let metrics = style::metrics(cell_theme);
    let footer = section == SectionKind::Footer && !is_header;
    let default_weight = if is_header || footer {
        FontWeight::Medium
    } else {
        FontWeight::Normal
    };
    let resolved_font =
        font.unwrap_or_else(|| style::font(cell_theme, font_weight.unwrap_or(default_weight)));
    let (default_size, default_line_height, uppercase) = if is_header {
        (
            metrics.header_text_size,
            metrics.header_line_height,
            metrics.header_uppercase,
        )
    } else {
        (metrics.text_size, metrics.line_height, false)
    };
    let size = text_size.unwrap_or(default_size);
    let line_height = line_height.unwrap_or(default_line_height);
    let foreground = color.unwrap_or(if is_header && metrics.header_is_muted {
        cell_theme.palette.muted_foreground
    } else {
        table_theme.palette.foreground
    });

    let content: Element<'a, Message> = match content {
        TableContent::Text(fragment) => {
            let fragment = if uppercase {
                fragment.as_ref().to_uppercase().into()
            } else {
                fragment
            };
            iced_text(fragment)
                .size(size)
                .line_height(LineHeight::Absolute(line_height.into()))
                .wrapping(Wrapping::None)
                .font(resolved_font)
                .color(foreground)
                .width(Length::Fill)
                .align_x(align_x)
                .into()
        }
        TableContent::Element(element) => container(element)
            .width(Length::Fill)
            .align_x(align_x)
            .into(),
    };

    let mut resolved = container::Style {
        text_color: Some(foreground),
        ..container::Style::default()
    };
    if let Some(style_override) = style_override {
        resolved = style_override(resolved);
    }

    let mut cell = container(content)
        .width(width)
        .padding(padding.unwrap_or_else(|| style::cell_padding(cell_theme, is_header)))
        .align_x(align_x)
        .align_y(align_y)
        .style(move |_| resolved);

    if is_header {
        cell = cell.height(Length::Fixed(metrics.header_height));
    }

    cell.into()
}

/// Builds a standalone cell without a row/grid context.
pub(super) fn build_standalone_cell<'a, Message>(
    theme: &'a Theme,
    config: TableCellConfig<'a, Message>,
    is_header: bool,
) -> Element<'a, Message>
where
    Message: 'a,
{
    build_cell(
        theme,
        theme,
        config,
        is_header,
        SectionKind::Body,
        Length::Fill,
    )
}

/// Builds a standalone row with the normal body treatment.
pub(super) fn build_standalone_row<'a, Message>(row: TableRow<'a, Message>) -> Element<'a, Message>
where
    Message: 'a,
{
    let theme = row.theme;
    build_row(theme, row, SectionKind::Body, true, &[])
}

/// Builds a standalone body-style section.
pub(super) fn build_standalone_section<'a, Message>(
    section: TableSection<'a, Message>,
) -> Element<'a, Message>
where
    Message: 'a,
{
    let columns = section.rows.iter().map(row_column_count).max().unwrap_or(1);
    build_section(section, SectionKind::Body, &[], columns)
}

/// Builds the caption after the table rows, matching `caption-bottom`.
pub(super) fn build_caption<'a, Message>(caption: TableCaption<'a, Message>) -> Element<'a, Message>
where
    Message: 'a,
{
    let TableCaption {
        theme,
        content,
        width,
        align_x,
        color,
        font,
        font_weight,
        text_size,
        line_height,
        margin_top,
        style_override,
    } = caption;

    let metrics = style::metrics(theme);
    let foreground = color.unwrap_or(theme.palette.muted_foreground);
    let resolved_font =
        font.unwrap_or_else(|| style::font(theme, font_weight.unwrap_or(FontWeight::Normal)));
    let size = text_size.unwrap_or(metrics.text_size);
    let line_height = line_height.unwrap_or(metrics.line_height);

    let content: Element<'a, Message> = match content {
        TableContent::Text(fragment) => iced_text(fragment)
            .size(size)
            .line_height(LineHeight::Absolute(line_height.into()))
            .wrapping(Wrapping::None)
            .font(resolved_font)
            .color(foreground)
            .width(Length::Fill)
            .align_x(align_x)
            .into(),
        TableContent::Element(element) => container(element)
            .width(Length::Fill)
            .align_x(align_x)
            .into(),
    };

    let mut resolved = container::Style {
        text_color: Some(foreground),
        ..container::Style::default()
    };
    if let Some(style_override) = style_override {
        resolved = style_override(resolved);
    }

    container(content)
        .width(width)
        .padding(Padding {
            top: margin_top.unwrap_or(metrics.caption_margin_top),
            ..Padding::ZERO
        })
        .style(move |_| resolved)
        .into()
}

fn row_column_count<Message>(row: &TableRow<'_, Message>) -> usize {
    row.cells
        .iter()
        .map(|cell| match cell {
            TableRowCell::Cell(cell) => cell.config.span.max(1),
            TableRowCell::Head(head) => head.config.span.max(1),
            TableRowCell::Element(_) => 1,
        })
        .fold(0usize, usize::saturating_add)
        .max(1)
}

fn root_section_column_count<Message>(section: &TableRootSection<'_, Message>) -> usize {
    match section {
        TableRootSection::Header(section)
        | TableRootSection::Body(section)
        | TableRootSection::Footer(section) => {
            section.rows.iter().map(row_column_count).max().unwrap_or(1)
        }
    }
}

fn resolve_cell_width(
    explicit: Option<Length>,
    column_widths: &[Length],
    start: usize,
    span: usize,
) -> Length {
    if let Some(width) = explicit {
        return width;
    }

    if column_widths.is_empty() {
        return Length::FillPortion(portion(span));
    }

    let end = start.saturating_add(span).min(column_widths.len());
    let widths = &column_widths[start.min(column_widths.len())..end];
    if widths.is_empty() {
        return Length::FillPortion(portion(span));
    }

    if widths.iter().all(|width| matches!(width, Length::Fixed(_))) {
        return Length::Fixed(
            widths
                .iter()
                .map(|width| match width {
                    Length::Fixed(value) => *value,
                    _ => 0.0,
                })
                .sum(),
        );
    }

    if span == 1 {
        return widths[0];
    }

    if widths.iter().all(Length::is_fill) {
        let portions = widths
            .iter()
            .map(Length::fill_factor)
            .fold(0u16, u16::saturating_add)
            .max(1);
        return Length::FillPortion(portions);
    }

    Length::FillPortion(portion(span))
}

fn portion(span: usize) -> u16 {
    u16::try_from(span.max(1)).unwrap_or(u16::MAX)
}

fn horizontal_rule<'a, Message: 'a>(color: Color) -> Element<'a, Message> {
    container(Space::new())
        .width(Length::Fill)
        .height(Length::Fixed(1.0))
        .style(move |_| container::Style {
            background: Some(Background::Color(color)),
            ..container::Style::default()
        })
        .into()
}

/// A pass-through widget that paints a row background from the current cursor.
///
/// `container` styles are static in iced. This small wrapper preserves the
/// source table's CSS hover state without requiring application-owned hover
/// state or a synthetic message.
struct HoverRow<'a, Message> {
    content: Element<'a, Message>,
    base: container::Style,
    hovered: container::Style,
    hoverable: bool,
}

#[derive(Debug, Default)]
struct HoverRowState {
    is_hovered: bool,
}

impl<Message> Widget<Message, crate::iced_compat::Theme, crate::iced_compat::Renderer>
    for HoverRow<'_, Message>
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<HoverRowState>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(HoverRowState::default())
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.content));
    }

    fn size(&self) -> Size<Length> {
        self.content.as_widget().size()
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &crate::iced_compat::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let child = self
            .content
            .as_widget_mut()
            .layout(&mut tree.children[0], renderer, limits);
        layout::Node::with_children(child.size(), vec![child])
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &crate::iced_compat::Renderer,
        operation: &mut dyn Operation,
    ) {
        self.content.as_widget_mut().operate(
            &mut tree.children[0],
            layout.children().next().expect("table row child layout"),
            renderer,
            operation,
        );
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &crate::iced_compat::Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();

        self.content.as_widget_mut().update(
            &mut tree.children[0],
            event,
            layout.children().next().expect("table row child layout"),
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );

        let is_hovered = self.hoverable && cursor.is_over(bounds);
        let state = tree.state.downcast_mut::<HoverRowState>();
        match event {
            Event::Mouse(mouse::Event::CursorMoved { .. }) if state.is_hovered != is_hovered => {
                state.is_hovered = is_hovered;
                shell.request_redraw();
            }
            Event::Window(window::Event::RedrawRequested(_)) => {
                state.is_hovered = is_hovered;
            }
            _ => {}
        }
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &crate::iced_compat::Renderer,
    ) -> mouse::Interaction {
        self.content.as_widget().mouse_interaction(
            &tree.children[0],
            layout.children().next().expect("table row child layout"),
            cursor,
            viewport,
            renderer,
        )
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut crate::iced_compat::Renderer,
        _theme: &crate::iced_compat::Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        if !bounds.intersects(viewport) {
            return;
        }

        let is_hovered = tree.state.downcast_ref::<HoverRowState>().is_hovered;
        let style = if self.hoverable && is_hovered {
            self.hovered
        } else {
            self.base
        };

        renderer.fill_quad(
            renderer::Quad {
                bounds,
                shadow: style.shadow,
                snap: style.snap,
                ..renderer::Quad::default()
            },
            style
                .background
                .unwrap_or(Background::Color(Color::TRANSPARENT)),
        );

        let border = style.border;
        if border.width > 0.0 && border.color.a > f32::EPSILON {
            let thickness = border.width.min(bounds.height);
            renderer.fill_quad(
                renderer::Quad {
                    bounds: Rectangle {
                        x: bounds.x,
                        y: bounds.y + bounds.height - thickness,
                        width: bounds.width,
                        height: thickness,
                    },
                    ..renderer::Quad::default()
                },
                Background::Color(border.color),
            );
        }

        self.content.as_widget().draw(
            &tree.children[0],
            renderer,
            _theme,
            _style,
            layout.children().next().expect("table row child layout"),
            cursor,
            viewport,
        );
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'b>,
        renderer: &crate::iced_compat::Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<
        overlay::Element<'b, Message, crate::iced_compat::Theme, crate::iced_compat::Renderer>,
    > {
        self.content.as_widget_mut().overlay(
            &mut tree.children[0],
            layout.children().next().expect("table row child layout"),
            renderer,
            viewport,
            translation,
        )
    }
}
