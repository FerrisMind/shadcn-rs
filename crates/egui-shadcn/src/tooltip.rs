use crate::theme::Theme;
use crate::tokens::{ColorPalette, DEFAULT_MOTION, ease_out_cubic, mix};
use egui::epaint::Shadow;
use egui::{
    Color32, CornerRadius, Frame, Id, Order, Pos2, Rect, Response, Stroke, Ui, Vec2, WidgetText,
    vec2,
};
use log::trace;
use std::time::Duration;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum TooltipPosition {
    Cursor,
    #[default]
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum TooltipSide {
    #[default]
    Top,
    Right,
    Bottom,
    Left,
}

impl TooltipSide {
    pub fn from_position(pos: TooltipPosition) -> Self {
        match pos {
            TooltipPosition::Top => TooltipSide::Top,
            TooltipPosition::Bottom => TooltipSide::Bottom,
            TooltipPosition::Left => TooltipSide::Left,
            TooltipPosition::Right => TooltipSide::Right,
            TooltipPosition::Cursor => TooltipSide::Top,
        }
    }

    pub fn offset_direction(&self) -> Vec2 {
        match self {
            TooltipSide::Top => vec2(0.0, -1.0),
            TooltipSide::Bottom => vec2(0.0, 1.0),
            TooltipSide::Left => vec2(-1.0, 0.0),
            TooltipSide::Right => vec2(1.0, 0.0),
        }
    }

    pub fn flip(&self) -> Self {
        match self {
            TooltipSide::Top => TooltipSide::Bottom,
            TooltipSide::Bottom => TooltipSide::Top,
            TooltipSide::Left => TooltipSide::Right,
            TooltipSide::Right => TooltipSide::Left,
        }
    }

    pub fn is_vertical(&self) -> bool {
        matches!(self, TooltipSide::Top | TooltipSide::Bottom)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum TooltipAlign {
    #[default]
    Center,
    Start,
    End,
}

impl TooltipAlign {
    pub fn factor(&self) -> f32 {
        match self {
            TooltipAlign::Center => 0.0,
            TooltipAlign::Start => -1.0,
            TooltipAlign::End => 1.0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TooltipAnimationState {
    Closed,

    DelayedOpen,

    InstantOpen,
}

#[derive(Clone, Debug, Default)]
pub struct TooltipOpenState {
    pub is_open: bool,

    pub animation_progress: f32,

    pub hover_start_time: Option<f64>,

    pub last_close_time: Option<f64>,
}

impl TooltipOpenState {
    pub fn is_visible(&self) -> bool {
        self.is_open || self.animation_progress > 0.0
    }

    pub fn is_animating(&self) -> bool {
        if self.is_open {
            self.animation_progress < 1.0
        } else {
            self.animation_progress > 0.0
        }
    }

    pub fn should_skip_delay(&self, current_time: f64, skip_delay_ms: u64) -> bool {
        if let Some(close_time) = self.last_close_time {
            let elapsed = current_time - close_time;
            let skip_delay_secs = skip_delay_ms as f64 / 1000.0;
            elapsed < skip_delay_secs
        } else {
            false
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct TooltipState {
    pub open_state: TooltipOpenState,

    pub computed_side: Option<TooltipSide>,

    pub computed_align: Option<TooltipAlign>,
}

impl TooltipState {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Clone, Debug)]
pub struct TooltipStyle {
    pub bg: Color32,
    pub border: Color32,
    pub border_width: f32,
    pub text: Color32,
    pub rounding: CornerRadius,
    pub shadow: Shadow,

    pub arrow_fill: Color32,
}

impl TooltipStyle {
    pub fn from_palette(palette: &ColorPalette, high_contrast: bool) -> Self {
        let bg = if high_contrast {
            palette.foreground
        } else {
            mix(palette.foreground, palette.background, 0.1)
        };

        let border = if high_contrast {
            palette.foreground
        } else {
            mix(palette.border, palette.foreground, 0.2)
        };

        let text = palette.background;

        let rounding = CornerRadius::same(6);
        let shadow = Shadow::default();
        Self {
            bg,
            border,
            border_width: if high_contrast { 0.0 } else { 1.0 },
            text,
            rounding,
            shadow,
            arrow_fill: bg,
        }
    }
}

#[derive(Clone, Debug)]
pub struct TooltipProps {
    pub text: WidgetText,

    pub delay_ms: u64,

    pub max_width: f32,

    pub position: TooltipPosition,

    pub side: TooltipSide,

    pub align: TooltipAlign,

    pub offset: Vec2,

    pub side_offset: f32,

    pub align_offset: f32,

    pub collision_padding: f32,
    pub high_contrast: bool,
    pub persistent_id: Option<Id>,
    pub style: Option<TooltipStyle>,
    pub show_when_disabled: bool,

    pub show_arrow: bool,

    pub arrow_width: f32,

    pub arrow_height: f32,

    pub arrow_padding: f32,

    pub force_mount: bool,

    pub skip_delay_ms: u64,

    pub disable_hoverable_content: bool,

    pub animation_duration_ms: u64,

    pub open: Option<bool>,

    pub default_open: bool,

    pub sticky: bool,

    pub avoid_collisions: bool,
}

impl TooltipProps {
    pub fn new(text: impl Into<WidgetText>) -> Self {
        Self {
            text: text.into(),
            delay_ms: 700,
            max_width: 360.0,
            position: TooltipPosition::Top,
            side: TooltipSide::Top,
            align: TooltipAlign::Center,
            offset: vec2(0.0, 8.0),
            side_offset: 4.0,
            align_offset: 0.0,
            collision_padding: 10.0,
            high_contrast: false,
            persistent_id: None,
            style: None,
            show_when_disabled: false,
            show_arrow: false,
            arrow_width: 11.0,
            arrow_height: 5.0,
            arrow_padding: 0.0,
            force_mount: false,
            skip_delay_ms: 300,
            disable_hoverable_content: false,
            animation_duration_ms: DEFAULT_MOTION.base_ms as u64,
            open: None,
            default_open: false,
            sticky: false,
            avoid_collisions: true,
        }
    }

    pub fn delay_ms(mut self, delay_ms: u64) -> Self {
        self.delay_ms = delay_ms;
        self
    }

    pub fn max_width(mut self, max_width: f32) -> Self {
        self.max_width = max_width;
        self
    }

    pub fn position(mut self, position: TooltipPosition) -> Self {
        self.position = position;
        self.side = TooltipSide::from_position(position);
        self
    }

    pub fn side(mut self, side: TooltipSide) -> Self {
        self.side = side;
        self
    }

    pub fn align(mut self, align: TooltipAlign) -> Self {
        self.align = align;
        self
    }

    pub fn offset(mut self, offset: Vec2) -> Self {
        self.offset = offset;
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

    pub fn collision_padding(mut self, padding: f32) -> Self {
        self.collision_padding = padding;
        self
    }

    pub fn high_contrast(mut self, high_contrast: bool) -> Self {
        self.high_contrast = high_contrast;
        self
    }

    pub fn persistent_id(mut self, id: Id) -> Self {
        self.persistent_id = Some(id);
        self
    }

    pub fn style(mut self, style: TooltipStyle) -> Self {
        self.style = Some(style);
        self
    }

    pub fn show_when_disabled(mut self, show: bool) -> Self {
        self.show_when_disabled = show;
        self
    }

    pub fn show_arrow(mut self, show: bool) -> Self {
        self.show_arrow = show;
        self
    }

    pub fn arrow_size(mut self, width: f32, height: f32) -> Self {
        self.arrow_width = width;
        self.arrow_height = height;
        self
    }

    pub fn arrow_padding(mut self, padding: f32) -> Self {
        self.arrow_padding = padding;
        self
    }

    pub fn force_mount(mut self, force: bool) -> Self {
        self.force_mount = force;
        self
    }

    pub fn skip_delay_ms(mut self, skip_delay: u64) -> Self {
        self.skip_delay_ms = skip_delay;
        self
    }

    pub fn disable_hoverable_content(mut self, disable: bool) -> Self {
        self.disable_hoverable_content = disable;
        self
    }

    pub fn animation_duration_ms(mut self, duration: u64) -> Self {
        self.animation_duration_ms = duration;
        self
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = Some(open);
        self
    }

    pub fn default_open(mut self, default_open: bool) -> Self {
        self.default_open = default_open;
        self
    }

    pub fn sticky(mut self, sticky: bool) -> Self {
        self.sticky = sticky;
        self
    }

    pub fn avoid_collisions(mut self, avoid: bool) -> Self {
        self.avoid_collisions = avoid;
        self
    }
}

#[allow(clippy::too_many_arguments)]
fn calculate_tooltip_pos(
    anchor_rect: Rect,
    tooltip_size: Vec2,
    side: TooltipSide,
    align: TooltipAlign,
    side_offset: f32,
    align_offset: f32,
    collision_padding: f32,
    viewport_size: Vec2,
    avoid_collisions: bool,
    arrow_height: f32,
    show_arrow: bool,
) -> (Pos2, TooltipSide) {
    let effective_side_offset = if show_arrow {
        side_offset + arrow_height
    } else {
        side_offset
    };

    let mut current_side = side;
    let mut pos = calculate_position_for_side(
        anchor_rect,
        tooltip_size,
        current_side,
        align,
        effective_side_offset,
        align_offset,
    );

    if avoid_collisions {
        let viewport_rect = Rect::from_min_size(Pos2::ZERO, viewport_size);
        let tooltip_rect = Rect::from_min_size(pos, tooltip_size);

        if !viewport_rect
            .shrink(collision_padding)
            .contains_rect(tooltip_rect)
        {
            let flipped_side = current_side.flip();
            let flipped_pos = calculate_position_for_side(
                anchor_rect,
                tooltip_size,
                flipped_side,
                align,
                effective_side_offset,
                align_offset,
            );
            let flipped_rect = Rect::from_min_size(flipped_pos, tooltip_size);

            if viewport_rect
                .shrink(collision_padding)
                .contains_rect(flipped_rect)
            {
                current_side = flipped_side;
                pos = flipped_pos;
            }
        }
    }

    let min_x = collision_padding;
    let max_x = (viewport_size.x - tooltip_size.x - collision_padding).max(min_x);
    let min_y = collision_padding;
    let max_y = (viewport_size.y - tooltip_size.y - collision_padding).max(min_y);

    pos.x = pos.x.clamp(min_x, max_x);
    pos.y = pos.y.clamp(min_y, max_y);

    (pos, current_side)
}

fn calculate_position_for_side(
    anchor_rect: Rect,
    tooltip_size: Vec2,
    side: TooltipSide,
    align: TooltipAlign,
    side_offset: f32,
    align_offset: f32,
) -> Pos2 {
    let anchor_center = anchor_rect.center();

    match side {
        TooltipSide::Top => {
            let x = calculate_aligned_pos(
                anchor_center.x,
                anchor_rect.width(),
                tooltip_size.x,
                align,
                align_offset,
            );
            let y = anchor_rect.top() - tooltip_size.y - side_offset;
            Pos2::new(x, y)
        }
        TooltipSide::Bottom => {
            let x = calculate_aligned_pos(
                anchor_center.x,
                anchor_rect.width(),
                tooltip_size.x,
                align,
                align_offset,
            );
            let y = anchor_rect.bottom() + side_offset;
            Pos2::new(x, y)
        }
        TooltipSide::Left => {
            let x = anchor_rect.left() - tooltip_size.x - side_offset;
            let y = calculate_aligned_pos(
                anchor_center.y,
                anchor_rect.height(),
                tooltip_size.y,
                align,
                align_offset,
            );
            Pos2::new(x, y)
        }
        TooltipSide::Right => {
            let x = anchor_rect.right() + side_offset;
            let y = calculate_aligned_pos(
                anchor_center.y,
                anchor_rect.height(),
                tooltip_size.y,
                align,
                align_offset,
            );
            Pos2::new(x, y)
        }
    }
}

fn calculate_aligned_pos(
    anchor_center: f32,
    anchor_size: f32,
    tooltip_size: f32,
    align: TooltipAlign,
    align_offset: f32,
) -> f32 {
    match align {
        TooltipAlign::Center => anchor_center - tooltip_size / 2.0 + align_offset,
        TooltipAlign::Start => anchor_center - anchor_size / 2.0 + align_offset,
        TooltipAlign::End => anchor_center + anchor_size / 2.0 - tooltip_size + align_offset,
    }
}

#[allow(clippy::too_many_arguments)]
fn draw_arrow(
    painter: &egui::Painter,
    content_rect: Rect,
    side: TooltipSide,
    arrow_width: f32,
    arrow_height: f32,
    fill: Color32,
    anchor_rect: Rect,
    arrow_padding: f32,
) {
    let arrow_center = match side {
        TooltipSide::Top | TooltipSide::Bottom => {
            let min_x = content_rect.left() + arrow_padding + arrow_width / 2.0;
            let max_x = content_rect.right() - arrow_padding - arrow_width / 2.0;
            anchor_rect.center().x.clamp(min_x, max_x)
        }
        TooltipSide::Left | TooltipSide::Right => {
            let min_y = content_rect.top() + arrow_padding + arrow_width / 2.0;
            let max_y = content_rect.bottom() - arrow_padding - arrow_width / 2.0;
            anchor_rect.center().y.clamp(min_y, max_y)
        }
    };

    let points = match side {
        TooltipSide::Top => {
            let tip_y = content_rect.bottom() + arrow_height;
            vec![
                Pos2::new(arrow_center - arrow_width / 2.0, content_rect.bottom()),
                Pos2::new(arrow_center + arrow_width / 2.0, content_rect.bottom()),
                Pos2::new(arrow_center, tip_y),
            ]
        }
        TooltipSide::Bottom => {
            let tip_y = content_rect.top() - arrow_height;
            vec![
                Pos2::new(arrow_center - arrow_width / 2.0, content_rect.top()),
                Pos2::new(arrow_center + arrow_width / 2.0, content_rect.top()),
                Pos2::new(arrow_center, tip_y),
            ]
        }
        TooltipSide::Left => {
            let tip_x = content_rect.right() + arrow_height;
            vec![
                Pos2::new(content_rect.right(), arrow_center - arrow_width / 2.0),
                Pos2::new(content_rect.right(), arrow_center + arrow_width / 2.0),
                Pos2::new(tip_x, arrow_center),
            ]
        }
        TooltipSide::Right => {
            let tip_x = content_rect.left() - arrow_height;
            vec![
                Pos2::new(content_rect.left(), arrow_center - arrow_width / 2.0),
                Pos2::new(content_rect.left(), arrow_center + arrow_width / 2.0),
                Pos2::new(tip_x, arrow_center),
            ]
        }
    };

    let shape = egui::epaint::PathShape::convex_polygon(points, fill, Stroke::NONE);
    painter.add(shape);
}

fn get_global_last_close_time(ctx: &egui::Context) -> Option<f64> {
    ctx.data(|d| d.get_temp::<f64>(Id::new("__tooltip_global_last_close__")))
}

fn set_global_last_close_time(ctx: &egui::Context, time: f64) {
    ctx.data_mut(|d| d.insert_temp(Id::new("__tooltip_global_last_close__"), time));
}

pub fn tooltip(anchor: &Response, ui: &mut Ui, theme: &Theme, props: TooltipProps) -> bool {
    let ctx = ui.ctx();
    let now = ctx.input(|i| i.time);

    let anchor_hovered = anchor.hovered() || anchor.has_focus();
    let disabled = !anchor.enabled();

    if disabled && !props.show_when_disabled && !props.force_mount {
        return false;
    }

    let id = props
        .persistent_id
        .unwrap_or_else(|| anchor.id.with("tooltip"));

    let delay_secs = props.delay_ms as f64 / 1000.0;
    let animation_duration = (props.animation_duration_ms as f32).max(1.0) / 1000.0;

    let global_last_close = get_global_last_close_time(ctx);
    let should_skip_delay = global_last_close.is_some_and(|close_time| {
        let elapsed = now - close_time;
        elapsed < (props.skip_delay_ms as f64 / 1000.0)
    });

    let tooltip_area_id = id.with("area");
    let tooltip_hovered = if !props.disable_hoverable_content {
        ctx.data(|d| d.get_temp::<bool>(tooltip_area_id))
            .unwrap_or(false)
    } else {
        false
    };

    let should_be_open = props.open.unwrap_or(anchor_hovered || tooltip_hovered);

    let (elapsed_hover, is_open, should_record_close) = ctx.data_mut(|d| {
        let hover_start_key = id.with("hover-start");
        let open_key = id.with("is-open");

        if should_be_open {
            let start = d.get_temp::<f64>(hover_start_key).unwrap_or(now);
            if d.get_temp::<f64>(hover_start_key).is_none() {
                d.insert_temp(hover_start_key, now);
            }
            let elapsed = now - start;

            let effective_delay = if should_skip_delay || props.default_open {
                0.0
            } else {
                delay_secs
            };
            let is_open = elapsed >= effective_delay || props.force_mount;

            if is_open {
                d.insert_temp(open_key, true);
            }

            (
                elapsed,
                d.get_temp::<bool>(open_key).unwrap_or(false),
                false,
            )
        } else {
            d.remove::<f64>(hover_start_key);
            let was_open = d.get_temp::<bool>(open_key).unwrap_or(false);
            d.remove::<bool>(open_key);
            (0.0, false, was_open)
        }
    });

    if should_record_close {
        set_global_last_close_time(ctx, now);
    }

    let animation_progress = ctx.animate_bool_with_time_and_easing(
        id.with("animation"),
        is_open || props.force_mount,
        animation_duration,
        ease_out_cubic,
    );

    if animation_progress <= 0.0 && !props.force_mount {
        if should_be_open && elapsed_hover < delay_secs {
            ctx.request_repaint_after(Duration::from_secs_f64(delay_secs - elapsed_hover));
        }
        return false;
    }

    let _animation_state = if !is_open && !props.force_mount {
        TooltipAnimationState::Closed
    } else if should_skip_delay || props.default_open || elapsed_hover == 0.0 {
        TooltipAnimationState::InstantOpen
    } else {
        TooltipAnimationState::DelayedOpen
    };

    let style = props
        .style
        .clone()
        .unwrap_or_else(|| TooltipStyle::from_palette(&theme.palette, props.high_contrast));

    let anchor_rect = anchor.rect;
    let viewport_size = ctx.viewport_rect().size();

    let (measured_size, text_galley) = {
        let text_str = props.text.text().to_string();
        let available_width = props.max_width - 24.0;

        let galley = ctx.fonts_mut(|fonts| {
            fonts.layout(
                text_str,
                egui::FontId::default(),
                style.text,
                available_width,
            )
        });

        let text_size = galley.size();

        let size = Vec2::new(text_size.x + 24.0, text_size.y + 12.0);

        (size, galley)
    };

    let _ = text_galley;

    let (tooltip_pos, computed_side) = calculate_tooltip_pos(
        anchor_rect,
        measured_size,
        props.side,
        props.align,
        props.side_offset,
        props.align_offset,
        props.collision_padding,
        viewport_size,
        props.avoid_collisions,
        props.arrow_height,
        props.show_arrow,
    );

    let slide_offset = match computed_side {
        TooltipSide::Top => vec2(0.0, 4.0),
        TooltipSide::Bottom => vec2(0.0, -4.0),
        TooltipSide::Left => vec2(4.0, 0.0),
        TooltipSide::Right => vec2(-4.0, 0.0),
    };

    let scale = 0.96 + 0.04 * animation_progress;
    let scaled_size = measured_size * scale;
    let scale_offset = (measured_size - scaled_size) * 0.5;

    let animated_offset = slide_offset * (1.0 - animation_progress);
    let final_pos = tooltip_pos + animated_offset + scale_offset;

    let opacity = animation_progress;

    trace!(
        "Showing tooltip at {:?}, side={:?}, progress={:.2}",
        final_pos, computed_side, animation_progress
    );

    let area_response = egui::Area::new(id)
        .order(Order::Tooltip)
        .fixed_pos(final_pos)
        .show(ctx, |tooltip_ui| {
            tooltip_ui.set_max_width(props.max_width);

            let mut visuals = tooltip_ui.visuals().clone();
            visuals.widgets.noninteractive.bg_fill = style.bg.gamma_multiply(opacity);
            tooltip_ui.ctx().set_visuals(visuals);

            let mut frame = Frame::popup(tooltip_ui.style());
            frame.fill = style.bg.gamma_multiply(opacity);
            frame.stroke = Stroke::new(style.border_width, style.border.gamma_multiply(opacity));
            frame.corner_radius = style.rounding;
            frame.shadow = Shadow {
                offset: style.shadow.offset,
                blur: style.shadow.blur,
                spread: style.shadow.spread,
                color: style.shadow.color.gamma_multiply(opacity),
            };
            frame.inner_margin = egui::Margin::symmetric(12, 6);

            let frame_response = frame.show(tooltip_ui, |content_ui| {
                content_ui.style_mut().visuals.override_text_color =
                    Some(style.text.gamma_multiply(opacity));

                content_ui.label(props.text.clone().color(style.text.gamma_multiply(opacity)));
            });

            if props.show_arrow {
                let painter = tooltip_ui.painter();
                draw_arrow(
                    painter,
                    frame_response.response.rect,
                    computed_side,
                    props.arrow_width,
                    props.arrow_height,
                    style.arrow_fill.gamma_multiply(opacity),
                    anchor_rect,
                    props.arrow_padding,
                );
            }
        });

    if !props.disable_hoverable_content {
        let tooltip_rect = area_response.response.rect;

        let expanded_rect = tooltip_rect.expand(4.0);
        let mouse_pos = ctx.input(|i| i.pointer.hover_pos());
        let content_hovered = mouse_pos.is_some_and(|pos| expanded_rect.contains(pos));
        ctx.data_mut(|d| d.insert_temp(tooltip_area_id, content_hovered));
    }

    ctx.request_repaint();
    true
}
