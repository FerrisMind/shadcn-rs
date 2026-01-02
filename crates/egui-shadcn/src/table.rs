//! Table component - display tabular data with shadcn styling.
//!
//! # Example
//! ```ignore
//! table(ui, &theme, TableProps::default(), |ui, ctx| {
//!     table_header(ui, ctx, |ui| {
//!         table_row(ui, ctx, TableRowProps::new("head"), |ui| {
//!             table_head(ui, ctx, TableCellProps::default(), |ui| {
//!                 ui.label("Invoice");
//!             });
//!         });
//!     });
//! });
//! ```

use crate::theme::Theme;
use egui::{
    Align, Color32, CornerRadius, Frame, Layout, Margin, Response, Sense, Stroke, Ui, Vec2, vec2,
};
use std::hash::Hash;

// =============================================================================
// Table size + variant
// =============================================================================

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

// =============================================================================
// Props
// =============================================================================

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

// =============================================================================
// Table context + tokens
// =============================================================================

#[derive(Clone, Copy, Debug)]
pub struct TableContext {
    pub size: TableSize,
    pub variant: TableVariant,
    tokens: TableTokens,
    metrics: TableMetrics,
}

#[derive(Clone, Copy, Debug)]
struct TableTokens {
    border: Color32,
    text: Color32,
    text_muted: Color32,
    hover_bg: Color32,
    selected_bg: Color32,
    footer_bg: Color32,
    container_bg: Color32,
}

#[derive(Clone, Copy, Debug)]
struct TableMetrics {
    row_height: f32,
    cell_padding: Margin,
    checkbox_padding: Margin,
    caption_gap: f32,
}

fn table_tokens(theme: &Theme, variant: TableVariant) -> TableTokens {
    let palette = &theme.palette;
    let container_bg = match variant {
        TableVariant::Default => Color32::TRANSPARENT,
        TableVariant::Muted => palette.muted.gamma_multiply(0.2),
    };
    TableTokens {
        border: palette.border,
        text: palette.foreground,
        text_muted: palette.muted_foreground,
        hover_bg: palette.muted.gamma_multiply(0.5),
        selected_bg: palette.muted.gamma_multiply(0.7),
        footer_bg: palette.muted.gamma_multiply(0.5),
        container_bg,
    }
}

fn table_metrics(size: TableSize) -> TableMetrics {
    match size {
        TableSize::Size1 => TableMetrics {
            row_height: 32.0,
            cell_padding: Margin::symmetric(6, 4),
            checkbox_padding: Margin::symmetric(6, 4),
            caption_gap: 12.0,
        },
        TableSize::Size2 => TableMetrics {
            row_height: 40.0,
            cell_padding: Margin::symmetric(8, 6),
            checkbox_padding: Margin::symmetric(8, 6),
            caption_gap: 16.0,
        },
        TableSize::Size3 => TableMetrics {
            row_height: 48.0,
            cell_padding: Margin::symmetric(10, 8),
            checkbox_padding: Margin::symmetric(10, 8),
            caption_gap: 20.0,
        },
    }
}

// =============================================================================
// Table API
// =============================================================================

pub fn table<R>(
    ui: &mut Ui,
    theme: &Theme,
    props: TableProps,
    add_contents: impl FnOnce(&mut Ui, &TableContext) -> R,
) -> R {
    let tokens = table_tokens(theme, props.variant);
    let metrics = table_metrics(props.size);
    let ctx = TableContext {
        size: props.size,
        variant: props.variant,
        tokens,
        metrics,
    };

    Frame::NONE
        .fill(tokens.container_bg)
        .show(ui, |table_ui| {
            table_ui.visuals_mut().override_text_color = Some(tokens.text);
            table_ui.spacing_mut().item_spacing = vec2(0.0, 0.0);
            add_contents(table_ui, &ctx)
        })
        .inner
}

pub fn table_header<R>(
    ui: &mut Ui,
    _ctx: &TableContext,
    add_contents: impl FnOnce(&mut Ui) -> R,
) -> R {
    ui.vertical(|header_ui| {
        header_ui.spacing_mut().item_spacing = vec2(0.0, 0.0);
        add_contents(header_ui)
    })
    .inner
}

pub fn table_body<R>(
    ui: &mut Ui,
    _ctx: &TableContext,
    add_contents: impl FnOnce(&mut Ui) -> R,
) -> R {
    ui.vertical(|body_ui| {
        body_ui.spacing_mut().item_spacing = vec2(0.0, 0.0);
        add_contents(body_ui)
    })
    .inner
}

pub fn table_footer<R>(
    ui: &mut Ui,
    ctx: &TableContext,
    add_contents: impl FnOnce(&mut Ui) -> R,
) -> R {
    Frame::NONE
        .fill(ctx.tokens.footer_bg)
        .show(ui, |footer_ui| {
            footer_ui.spacing_mut().item_spacing = vec2(0.0, 0.0);
            add_contents(footer_ui)
        })
        .inner
}

pub fn table_row<R, IdSource: Hash>(
    ui: &mut Ui,
    ctx: &TableContext,
    props: TableRowProps<IdSource>,
    add_contents: impl FnOnce(&mut Ui) -> R,
) -> TableRowResponse<R> {
    let row_height = ctx.metrics.row_height;
    let desired_size = Vec2::new(ui.available_width(), row_height);
    let row_id = ui.make_persistent_id(props.id_source);

    let inner = ui.allocate_ui_with_layout(
        desired_size,
        Layout::left_to_right(Align::Center),
        |row_ui| {
            row_ui.spacing_mut().item_spacing = vec2(0.0, 0.0);
            let rect = row_ui.max_rect();
            let response = row_ui.interact(rect, row_id, Sense::hover());
            let hover = props.hoverable && response.hovered();
            let fill = if props.selected {
                ctx.tokens.selected_bg
            } else if hover {
                ctx.tokens.hover_bg
            } else {
                Color32::TRANSPARENT
            };

            if fill != Color32::TRANSPARENT {
                row_ui
                    .painter()
                    .rect_filled(rect, CornerRadius::same(0), fill);
            }
            row_ui.painter().line_segment(
                [rect.left_bottom(), rect.right_bottom()],
                Stroke::new(1.0, ctx.tokens.border),
            );

            let contents = add_contents(row_ui);
            (contents, response)
        },
    );

    TableRowResponse {
        inner: inner.inner.0,
        response: inner.inner.1,
    }
}

pub struct TableRowResponse<R> {
    pub inner: R,
    pub response: Response,
}

pub fn table_head<R>(
    ui: &mut Ui,
    ctx: &TableContext,
    props: TableCellProps,
    add_contents: impl FnOnce(&mut Ui) -> R,
) -> R {
    let padding = if props.checkbox {
        ctx.metrics.checkbox_padding
    } else {
        ctx.metrics.cell_padding
    };
    let render = |cell_ui: &mut Ui| {
        Frame::NONE
            .inner_margin(padding)
            .show(cell_ui, |inner_ui| {
                inner_ui.visuals_mut().override_text_color = Some(ctx.tokens.text_muted);
                inner_ui
                    .with_layout(Layout::left_to_right(Align::Center), |inner_ui| {
                        add_contents(inner_ui)
                    })
                    .inner
            })
            .inner
    };

    if props.fill {
        let desired = vec2(ui.available_width(), ui.available_height());
        ui.allocate_ui_with_layout(desired, Layout::left_to_right(Align::Center), |cell_ui| {
            render(cell_ui)
        })
        .inner
    } else {
        render(ui)
    }
}

pub fn table_cell<R>(
    ui: &mut Ui,
    ctx: &TableContext,
    props: TableCellProps,
    add_contents: impl FnOnce(&mut Ui) -> R,
) -> R {
    let padding = if props.checkbox {
        ctx.metrics.checkbox_padding
    } else {
        ctx.metrics.cell_padding
    };
    let render = |cell_ui: &mut Ui| {
        Frame::NONE
            .inner_margin(padding)
            .show(cell_ui, |inner_ui| {
                inner_ui
                    .with_layout(Layout::left_to_right(Align::Center), |inner_ui| {
                        add_contents(inner_ui)
                    })
                    .inner
            })
            .inner
    };

    if props.fill {
        let desired = vec2(ui.available_width(), ui.available_height());
        ui.allocate_ui_with_layout(desired, Layout::left_to_right(Align::Center), |cell_ui| {
            render(cell_ui)
        })
        .inner
    } else {
        render(ui)
    }
}

pub fn table_caption(ui: &mut Ui, ctx: &TableContext, text: &str) -> Response {
    ui.add_space(ctx.metrics.caption_gap);
    ui.label(egui::RichText::new(text).color(ctx.tokens.text_muted))
}
