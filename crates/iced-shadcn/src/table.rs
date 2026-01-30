use iced::border::Border;
use iced::widget::{column, container, row, rule, text};
use iced::{Alignment, Background, Color, Element, Length};
use std::hash::Hash;

use crate::theme::Theme;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum TableSize {
    Size1,
    #[default]
    Size2,
    Size3,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum TableVariant {
    #[default]
    Default,
    Muted,
}

#[derive(Clone, Debug)]
pub struct TableProps {
    pub size: TableSize,
    pub variant: TableVariant,
}

impl Default for TableProps {
    fn default() -> Self {
        Self {
            size: TableSize::Size2,
            variant: TableVariant::Default,
        }
    }
}

impl TableProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn size(mut self, size: TableSize) -> Self {
        self.size = size;
        self
    }

    pub fn variant(mut self, variant: TableVariant) -> Self {
        self.variant = variant;
        self
    }
}

#[derive(Clone, Copy, Debug)]
pub struct TableRowProps<IdSource> {
    pub id_source: IdSource,
    pub selected: bool,
    pub hoverable: bool,
}

impl<IdSource> TableRowProps<IdSource> {
    pub fn new(id_source: IdSource) -> Self {
        Self {
            id_source,
            selected: false,
            hoverable: true,
        }
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn hoverable(mut self, hoverable: bool) -> Self {
        self.hoverable = hoverable;
        self
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct TableCellProps {
    pub checkbox: bool,
    pub fill: bool,
}

impl TableCellProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn checkbox(mut self, checkbox: bool) -> Self {
        self.checkbox = checkbox;
        self
    }

    pub fn fill(mut self, fill: bool) -> Self {
        self.fill = fill;
        self
    }
}

#[derive(Clone, Copy, Debug)]
pub struct TableContext {
    pub size: TableSize,
    pub variant: TableVariant,
    tokens: TableTokens,
    metrics: TableMetrics,
}

#[derive(Clone, Copy, Debug)]
struct TableTokens {
    border: Color,
    text: Color,
    text_muted: Color,
    selected_bg: Color,
    footer_bg: Color,
    container_bg: Color,
}

#[derive(Clone, Copy, Debug)]
struct TableMetrics {
    row_height: f32,
    cell_padding: [f32; 2],
    checkbox_padding: [f32; 2],
    caption_gap: f32,
}

fn table_tokens(theme: &Theme, variant: TableVariant) -> TableTokens {
    let palette = theme.palette;
    let container_bg = match variant {
        TableVariant::Default => Color::TRANSPARENT,
        TableVariant::Muted => apply_opacity(palette.muted, 0.2),
    };
    TableTokens {
        border: palette.border,
        text: palette.foreground,
        text_muted: palette.muted_foreground,
        selected_bg: apply_opacity(palette.muted, 0.7),
        footer_bg: apply_opacity(palette.muted, 0.5),
        container_bg,
    }
}

fn table_metrics(size: TableSize) -> TableMetrics {
    match size {
        TableSize::Size1 => TableMetrics {
            row_height: 32.0,
            cell_padding: [6.0, 4.0],
            checkbox_padding: [6.0, 4.0],
            caption_gap: 12.0,
        },
        TableSize::Size2 => TableMetrics {
            row_height: 40.0,
            cell_padding: [8.0, 6.0],
            checkbox_padding: [8.0, 6.0],
            caption_gap: 16.0,
        },
        TableSize::Size3 => TableMetrics {
            row_height: 48.0,
            cell_padding: [10.0, 8.0],
            checkbox_padding: [10.0, 8.0],
            caption_gap: 20.0,
        },
    }
}

pub fn table<'a, Message: Clone + 'a>(
    props: TableProps,
    theme: &Theme,
    add_contents: impl FnOnce(&TableContext) -> Element<'a, Message>,
) -> Element<'a, Message> {
    let tokens = table_tokens(theme, props.variant);
    let metrics = table_metrics(props.size);
    let ctx = TableContext {
        size: props.size,
        variant: props.variant,
        tokens,
        metrics,
    };

    container(add_contents(&ctx))
        .style(move |_t| iced::widget::container::Style {
            background: Some(Background::Color(tokens.container_bg)),
            text_color: Some(tokens.text),
            ..Default::default()
        })
        .into()
}

pub fn table_header<'a, Message: Clone + 'a>(
    _ctx: &TableContext,
    content: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    container(content).into()
}

pub fn table_body<'a, Message: Clone + 'a>(
    _ctx: &TableContext,
    content: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    container(content).into()
}

pub fn table_footer<'a, Message: Clone + 'a>(
    ctx: &TableContext,
    content: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    let footer_bg = ctx.tokens.footer_bg;
    container(content)
        .style(move |_t| iced::widget::container::Style {
            background: Some(Background::Color(footer_bg)),
            ..Default::default()
        })
        .into()
}

pub fn table_row<'a, Message: Clone + 'a, IdSource: Hash>(
    ctx: &TableContext,
    props: TableRowProps<IdSource>,
    cells: Vec<Element<'a, Message>>,
) -> Element<'a, Message> {
    let background = if props.selected {
        ctx.tokens.selected_bg
    } else {
        Color::TRANSPARENT
    };
    let row_height = ctx.metrics.row_height;
    let border_color = ctx.tokens.border;

    container(row(cells).spacing(0).align_y(Alignment::Center))
        .height(Length::Fixed(row_height))
        .style(move |_t| iced::widget::container::Style {
            background: Some(Background::Color(background)),
            border: Border {
                radius: 0.0.into(),
                width: 1.0,
                color: border_color,
            },
            ..Default::default()
        })
        .into()
}

pub fn table_head<'a, Message: Clone + 'a>(
    ctx: &TableContext,
    props: TableCellProps,
    content: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    let padding = if props.checkbox {
        ctx.metrics.checkbox_padding
    } else {
        ctx.metrics.cell_padding
    };
    let text_muted = ctx.tokens.text_muted;
    let element =
        container(content)
            .padding(padding)
            .style(move |_t| iced::widget::container::Style {
                text_color: Some(text_muted),
                ..Default::default()
            });

    if props.fill {
        element.width(Length::Fill).into()
    } else {
        element.into()
    }
}

pub fn table_cell<'a, Message: Clone + 'a>(
    ctx: &TableContext,
    props: TableCellProps,
    content: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    let padding = if props.checkbox {
        ctx.metrics.checkbox_padding
    } else {
        ctx.metrics.cell_padding
    };
    let element = container(content).padding(padding);
    if props.fill {
        element.width(Length::Fill).into()
    } else {
        element.into()
    }
}

pub fn table_caption<'a, Message: Clone + 'a>(
    ctx: &TableContext,
    text_value: &'a str,
) -> Element<'a, Message> {
    let text_muted = ctx.tokens.text_muted;
    let caption_gap = ctx.metrics.caption_gap;
    column![
        rule::horizontal(1),
        text(text_value)
            .size(12)
            .style(move |_t| iced::widget::text::Style {
                color: Some(text_muted),
            })
    ]
    .spacing(caption_gap)
    .into()
}

fn apply_opacity(color: Color, opacity: f32) -> Color {
    Color {
        a: color.a * opacity,
        ..color
    }
}
