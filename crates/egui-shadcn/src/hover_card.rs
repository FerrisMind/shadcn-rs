//! Hover Card component - popover-like card shown on hover.

use crate::popover::{
    PopoverAlign, PopoverCollisionPadding, PopoverSide, PopoverSticky,
    compute_popover_rect_with_collision,
};
use crate::theme::Theme;
use egui::{CornerRadius, Frame, Id, Margin, Order, Response, Stroke, Ui, vec2};
use std::time::Duration;

const DEFAULT_OPEN_DELAY_MS: u64 = 700;
const DEFAULT_CLOSE_DELAY_MS: u64 = 300;
const DEFAULT_WIDTH: f32 = 256.0;
const DEFAULT_MAX_HEIGHT: f32 = 320.0;

#[derive(Clone, Copy, Debug)]
pub struct HoverCardProps {
    pub id_source: Id,
    pub open_delay_ms: u64,
    pub close_delay_ms: u64,
    pub side: PopoverSide,
    pub align: PopoverAlign,
    pub side_offset: f32,
    pub align_offset: f32,
    pub width: Option<f32>,
    pub max_height: Option<f32>,
    pub content_padding: Margin,
}

impl HoverCardProps {
    pub fn new(id_source: Id) -> Self {
        Self {
            id_source,
            open_delay_ms: DEFAULT_OPEN_DELAY_MS,
            close_delay_ms: DEFAULT_CLOSE_DELAY_MS,
            side: PopoverSide::Top,
            align: PopoverAlign::Center,
            side_offset: 4.0,
            align_offset: 0.0,
            width: None,
            max_height: None,
            content_padding: Margin::same(16),
        }
    }

    pub fn open_delay_ms(mut self, delay_ms: u64) -> Self {
        self.open_delay_ms = delay_ms;
        self
    }

    pub fn close_delay_ms(mut self, delay_ms: u64) -> Self {
        self.close_delay_ms = delay_ms;
        self
    }

    pub fn side(mut self, side: PopoverSide) -> Self {
        self.side = side;
        self
    }

    pub fn align(mut self, align: PopoverAlign) -> Self {
        self.align = align;
        self
    }

    pub fn side_offset(mut self, offset: f32) -> Self {
        self.side_offset = offset;
        self
    }

    pub fn align_offset(mut self, offset: f32) -> Self {
        self.align_offset = offset;
        self
    }

    pub fn with_width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn with_max_height(mut self, max_height: f32) -> Self {
        self.max_height = Some(max_height);
        self
    }

    pub fn with_content_padding(mut self, padding: Margin) -> Self {
        self.content_padding = padding;
        self
    }
}

fn hover_card_open_id(id_source: Id) -> Id {
    id_source.with("hover-card-open")
}

fn hover_card_hover_start_id(id_source: Id) -> Id {
    id_source.with("hover-card-hover-start")
}

fn hover_card_close_start_id(id_source: Id) -> Id {
    id_source.with("hover-card-close-start")
}

fn hover_card_content_hover_id(id_source: Id) -> Id {
    id_source.with("hover-card-content-hover")
}

fn update_hover_card_open_state(
    ctx: &egui::Context,
    id_source: Id,
    want_open: bool,
    open_delay_ms: u64,
    close_delay_ms: u64,
) -> bool {
    let now = ctx.input(|i| i.time);
    let open_key = hover_card_open_id(id_source);
    let hover_start_key = hover_card_hover_start_id(id_source);
    let close_start_key = hover_card_close_start_id(id_source);

    let was_open = ctx.data(|d| d.get_temp::<bool>(open_key)).unwrap_or(false);
    let open;
    let mut request_after: Option<f64> = None;

    if want_open {
        ctx.data_mut(|d| d.remove::<f64>(close_start_key));
        if !was_open {
            let start = ctx
                .data(|d| d.get_temp::<f64>(hover_start_key))
                .unwrap_or_else(|| {
                    ctx.data_mut(|d| d.insert_temp(hover_start_key, now));
                    now
                });
            let elapsed = now - start;
            let delay_secs = open_delay_ms as f64 / 1000.0;
            if delay_secs <= 0.0 || elapsed >= delay_secs {
                open = true;
                ctx.data_mut(|d| d.remove::<f64>(hover_start_key));
            } else {
                open = false;
                request_after = Some(delay_secs - elapsed);
            }
        } else {
            open = true;
            ctx.data_mut(|d| d.remove::<f64>(hover_start_key));
        }
    } else {
        ctx.data_mut(|d| d.remove::<f64>(hover_start_key));
        if was_open {
            let start = ctx
                .data(|d| d.get_temp::<f64>(close_start_key))
                .unwrap_or_else(|| {
                    ctx.data_mut(|d| d.insert_temp(close_start_key, now));
                    now
                });
            let elapsed = now - start;
            let delay_secs = close_delay_ms as f64 / 1000.0;
            if delay_secs <= 0.0 || elapsed >= delay_secs {
                open = false;
                ctx.data_mut(|d| d.remove::<f64>(close_start_key));
            } else {
                open = true;
                request_after = Some(delay_secs - elapsed);
            }
        } else {
            open = false;
            ctx.data_mut(|d| d.remove::<f64>(close_start_key));
        }
    }

    ctx.data_mut(|d| d.insert_temp(open_key, open));
    if let Some(wait) = request_after {
        ctx.request_repaint_after(Duration::from_secs_f64(wait));
    }
    open
}

pub fn hover_card_trigger(
    ui: &mut Ui,
    id_source: Id,
    render_trigger: impl FnOnce(&mut Ui) -> Response,
) -> Response {
    ui.push_id(id_source.with("trigger"), |ui| render_trigger(ui))
        .inner
}

pub fn hover_card<R>(
    ui: &mut Ui,
    theme: &Theme,
    props: HoverCardProps,
    render_trigger: impl FnOnce(&mut Ui) -> Response,
    render_content: impl FnOnce(&mut Ui) -> R,
) -> (Response, Option<R>) {
    let id_source = props.id_source;
    let trigger_response = hover_card_trigger(ui, id_source, render_trigger);

    let content_hovered = ui
        .ctx()
        .data(|d| d.get_temp::<bool>(hover_card_content_hover_id(id_source)))
        .unwrap_or(false);
    let want_open = trigger_response.hovered() || trigger_response.has_focus() || content_hovered;

    let open = update_hover_card_open_state(
        ui.ctx(),
        id_source,
        want_open,
        props.open_delay_ms,
        props.close_delay_ms,
    );

    let inner = hover_card_content(ui, theme, props, &trigger_response, open, render_content);
    (trigger_response, inner)
}

pub fn hover_card_content<R>(
    ui: &mut Ui,
    theme: &Theme,
    props: HoverCardProps,
    trigger_response: &Response,
    open: bool,
    add_contents: impl FnOnce(&mut Ui) -> R,
) -> Option<R> {
    let ctx = ui.ctx();
    let anim_t = ctx.animate_bool(props.id_source.with("open-anim"), open);
    let is_mounted = open || anim_t > 0.0;
    if !is_mounted {
        ctx.data_mut(|d| d.insert_temp(hover_card_content_hover_id(props.id_source), false));
        return None;
    }

    let palette = &theme.palette;
    let bg = palette.popover.gamma_multiply(anim_t);
    let border = palette.border.gamma_multiply(anim_t);
    let rounding = CornerRadius::same(theme.radius.r3.round() as u8);

    let width = props.width.unwrap_or(DEFAULT_WIDTH).max(160.0);
    let max_height = props.max_height.unwrap_or(DEFAULT_MAX_HEIGHT);

    let boundary = ctx.available_rect();
    let (position_rect, computed_side) = compute_popover_rect_with_collision(
        trigger_response.rect,
        boundary,
        props.side,
        props.align,
        props.side_offset,
        props.align_offset,
        width,
        max_height,
        true,
        PopoverCollisionPadding::default(),
        PopoverSticky::default(),
    );

    let slide_offset = match computed_side {
        PopoverSide::Bottom => vec2(0.0, -8.0),
        PopoverSide::Top => vec2(0.0, 8.0),
        PopoverSide::Left => vec2(8.0, 0.0),
        PopoverSide::Right => vec2(-8.0, 0.0),
    };
    let animated_origin = position_rect.min + slide_offset * (1.0 - anim_t);

    let mut popup_rect = position_rect;
    let mut inner: Option<R> = None;
    let content_id = props.id_source.with("content");

    egui::Area::new(content_id)
        .order(Order::Tooltip)
        .interactable(is_mounted)
        .movable(false)
        .fixed_pos(animated_origin)
        .show(ctx, |popup_ui| {
            popup_ui.visuals_mut().override_text_color = Some(palette.popover_foreground);
            popup_ui.set_min_width(width);
            popup_ui.set_max_height(position_rect.height());

            let frame = Frame::popup(popup_ui.style())
                .fill(bg)
                .stroke(Stroke::new(1.0, border))
                .corner_radius(rounding)
                .inner_margin(props.content_padding);

            let frame_resp = frame.show(popup_ui, |content_ui| {
                inner = Some(add_contents(content_ui));
            });
            popup_rect = frame_resp.response.rect;
        });

    let expanded_rect = popup_rect.expand(4.0);
    let content_hovered = ctx
        .input(|i| i.pointer.hover_pos())
        .is_some_and(|pos| expanded_rect.contains(pos));
    ctx.data_mut(|d| {
        d.insert_temp(
            hover_card_content_hover_id(props.id_source),
            content_hovered,
        )
    });

    inner
}
