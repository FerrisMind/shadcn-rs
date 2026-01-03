//! Breadcrumb component - navigation chain with separators and ellipsis.

use crate::theme::Theme;
use egui::{
    Color32, FontId, Label, Response, Sense, Stroke, TextStyle, Ui, Vec2, WidgetText, pos2, vec2,
};

#[derive(Clone, Copy, Debug)]
pub struct BreadcrumbProps {
    pub text_size: f32,
    pub item_spacing: f32,
    pub line_spacing: f32,
    pub separator_size: f32,
    pub ellipsis_size: f32,
    pub wrap: bool,
}

impl Default for BreadcrumbProps {
    fn default() -> Self {
        Self {
            text_size: 12.0,
            item_spacing: 6.0,
            line_spacing: 4.0,
            separator_size: 12.0,
            ellipsis_size: 20.0,
            wrap: true,
        }
    }
}

impl BreadcrumbProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn text_size(mut self, text_size: f32) -> Self {
        self.text_size = text_size;
        self
    }

    pub fn item_spacing(mut self, item_spacing: f32) -> Self {
        self.item_spacing = item_spacing;
        self
    }

    pub fn line_spacing(mut self, line_spacing: f32) -> Self {
        self.line_spacing = line_spacing;
        self
    }

    pub fn separator_size(mut self, separator_size: f32) -> Self {
        self.separator_size = separator_size;
        self
    }

    pub fn ellipsis_size(mut self, ellipsis_size: f32) -> Self {
        self.ellipsis_size = ellipsis_size;
        self
    }

    pub fn wrap(mut self, wrap: bool) -> Self {
        self.wrap = wrap;
        self
    }
}

#[derive(Clone, Copy, Debug)]
pub struct BreadcrumbTokens {
    pub muted: Color32,
    pub foreground: Color32,
    pub separator: Color32,
}

#[derive(Clone, Copy, Debug)]
pub struct BreadcrumbMetrics {
    pub text_size: f32,
    pub item_spacing: f32,
    pub line_spacing: f32,
    pub separator_size: f32,
    pub ellipsis_size: f32,
    pub wrap: bool,
}

#[derive(Clone, Copy, Debug)]
pub struct BreadcrumbContext {
    pub tokens: BreadcrumbTokens,
    pub metrics: BreadcrumbMetrics,
}

pub fn breadcrumb<R>(
    ui: &mut Ui,
    theme: &Theme,
    props: BreadcrumbProps,
    add_contents: impl FnOnce(&mut Ui, &BreadcrumbContext) -> R,
) -> R {
    let tokens = BreadcrumbTokens {
        muted: theme.palette.muted_foreground,
        foreground: theme.palette.foreground,
        separator: theme.palette.muted_foreground,
    };
    let metrics = BreadcrumbMetrics {
        text_size: props.text_size,
        item_spacing: props.item_spacing,
        line_spacing: props.line_spacing,
        separator_size: props.separator_size,
        ellipsis_size: props.ellipsis_size,
        wrap: props.wrap,
    };
    let ctx = BreadcrumbContext { tokens, metrics };

    ui.scope(|ui| {
        let mut style = ui.style().as_ref().clone();
        style
            .text_styles
            .insert(TextStyle::Body, FontId::proportional(ctx.metrics.text_size));
        ui.set_style(style);
        add_contents(ui, &ctx)
    })
    .inner
}

pub fn breadcrumb_list<R>(
    ui: &mut Ui,
    ctx: &BreadcrumbContext,
    add_contents: impl FnOnce(&mut Ui, &BreadcrumbContext) -> R,
) -> R {
    let apply = |list_ui: &mut Ui| {
        list_ui.spacing_mut().item_spacing =
            vec2(ctx.metrics.item_spacing, ctx.metrics.line_spacing);
        list_ui.visuals_mut().override_text_color = Some(ctx.tokens.muted);
        add_contents(list_ui, ctx)
    };

    if ctx.metrics.wrap {
        ui.horizontal_wrapped(apply).inner
    } else {
        ui.horizontal(apply).inner
    }
}

pub fn breadcrumb_item<R>(
    ui: &mut Ui,
    ctx: &BreadcrumbContext,
    add_contents: impl FnOnce(&mut Ui) -> R,
) -> R {
    ui.horizontal(|item_ui| {
        item_ui.spacing_mut().item_spacing = vec2(ctx.metrics.item_spacing, 0.0);
        add_contents(item_ui)
    })
    .inner
}

pub fn breadcrumb_link(
    ui: &mut Ui,
    ctx: &BreadcrumbContext,
    text: impl Into<WidgetText>,
) -> Response {
    let label = Label::new(text.into().color(ctx.tokens.muted)).sense(Sense::click());
    ui.add(label)
        .on_hover_cursor(egui::CursorIcon::PointingHand)
}

pub fn breadcrumb_page(
    ui: &mut Ui,
    ctx: &BreadcrumbContext,
    text: impl Into<WidgetText>,
) -> Response {
    let label = Label::new(text.into().color(ctx.tokens.foreground)).sense(Sense::hover());
    ui.add(label)
}

pub fn breadcrumb_separator(
    ui: &mut Ui,
    ctx: &BreadcrumbContext,
    custom: Option<WidgetText>,
) -> Response {
    if let Some(text) = custom {
        return ui.add(Label::new(text.color(ctx.tokens.separator)).sense(Sense::hover()));
    }

    let size = ctx.metrics.separator_size;
    let (rect, response) = ui.allocate_exact_size(Vec2::splat(size), Sense::hover());
    let center = rect.center();
    icon_chevron_right(ui.painter(), center, size * 0.9, ctx.tokens.separator);
    response
}

pub fn breadcrumb_ellipsis(ui: &mut Ui, ctx: &BreadcrumbContext) -> Response {
    let size = ctx.metrics.ellipsis_size;
    let (rect, response) = ui.allocate_exact_size(Vec2::splat(size), Sense::hover());
    icon_more_horizontal(ui.painter(), rect.center(), size * 0.5, ctx.tokens.muted);
    response
}

fn stroke_width(size: f32) -> f32 {
    (size * 0.12).clamp(1.3, 2.2)
}

fn icon_chevron_right(painter: &egui::Painter, center: egui::Pos2, size: f32, color: Color32) {
    let stroke = Stroke::new(stroke_width(size), color);
    let half_x = size * 0.18;
    let half_y = size * 0.22;
    painter.line_segment(
        [
            pos2(center.x - half_x, center.y - half_y),
            pos2(center.x + half_x, center.y),
        ],
        stroke,
    );
    painter.line_segment(
        [
            pos2(center.x + half_x, center.y),
            pos2(center.x - half_x, center.y + half_y),
        ],
        stroke,
    );
}

fn icon_more_horizontal(painter: &egui::Painter, center: egui::Pos2, size: f32, color: Color32) {
    let radius = (size * 0.12).max(1.0);
    let gap = size * 0.28;
    painter.circle_filled(pos2(center.x - gap, center.y), radius, color);
    painter.circle_filled(center, radius, color);
    painter.circle_filled(pos2(center.x + gap, center.y), radius, color);
}
