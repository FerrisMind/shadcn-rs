use crate::theme::Theme;
use crate::tokens::{ColorPalette, DEFAULT_FOCUS, ease_out_cubic, mix};
use egui::scroll_area::{
    ScrollArea as EguiScrollArea, ScrollBarVisibility, State as EguiScrollState,
};
use egui::{Color32, CornerRadius, Id, Pos2, Rect, Sense, Stroke, StrokeKind, Ui, Vec2};
use log::trace;
use std::time::Duration;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScrollDirection {
    Horizontal,
    Vertical,
    Both,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ScrollAreaType {
    #[default]
    Hover,

    Scroll,

    Always,
    Auto,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ScrollAreaSize {
    #[default]
    Size1,
    Size2,
    Size3,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScrollAreaDir {
    Ltr,
    Rtl,
}

#[derive(Clone, Copy, Debug)]
pub struct ScrollAreaMetrics {
    pub bar_thickness: f32,
    pub handle_min_length: f32,
    pub bar_margin: f32,
    pub bar_padding: f32,
}

impl ScrollAreaSize {
    pub fn metrics(self) -> ScrollAreaMetrics {
        match self {
            ScrollAreaSize::Size1 => ScrollAreaMetrics {
                bar_thickness: 4.0,
                handle_min_length: 16.0,
                bar_margin: 4.0,
                bar_padding: 1.0,
            },
            ScrollAreaSize::Size2 => ScrollAreaMetrics {
                bar_thickness: 8.0,
                handle_min_length: 18.0,
                bar_margin: 4.0,
                bar_padding: 1.0,
            },
            ScrollAreaSize::Size3 => ScrollAreaMetrics {
                bar_thickness: 12.0,
                handle_min_length: 20.0,
                bar_margin: 4.0,
                bar_padding: 1.0,
            },
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ScrollAreaRadius {
    None,
    Small,
    Medium,
    Large,
    #[default]
    Full,
}

impl ScrollAreaRadius {
    pub fn corner_radius(self) -> CornerRadius {
        match self {
            ScrollAreaRadius::None => CornerRadius::same(0),
            ScrollAreaRadius::Small => CornerRadius::same(2),
            ScrollAreaRadius::Medium => CornerRadius::same(4),
            ScrollAreaRadius::Large => CornerRadius::same(6),
            ScrollAreaRadius::Full => CornerRadius::same(255),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ScrollAreaColors {
    pub track: Color32,
    pub thumb: Color32,
    pub thumb_hover: Color32,
    pub thumb_active: Color32,
    pub focus_ring: Stroke,
    pub corner: Color32,
}

impl ScrollAreaColors {
    pub fn from_palette(
        palette: &ColorPalette,
        accent_override: Option<Color32>,
        high_contrast: bool,
    ) -> Self {
        let accent = accent_override.unwrap_or(palette.accent);
        let focus_color = Color32::from_rgba_unmultiplied(
            palette.ring.r(),
            palette.ring.g(),
            palette.ring.b(),
            128,
        );
        let focus_ring = DEFAULT_FOCUS.stroke(focus_color);

        let track = mix(
            palette.muted,
            palette.background,
            if high_contrast { 0.6 } else { 0.7 },
        );
        let mut thumb = mix(
            palette.border,
            palette.foreground,
            if high_contrast { 0.45 } else { 0.35 },
        );
        let mut thumb_hover = mix(
            thumb,
            palette.foreground,
            if high_contrast { 0.4 } else { 0.25 },
        );
        let mut thumb_active = mix(thumb, accent, if high_contrast { 0.35 } else { 0.2 });

        if let Some(accent) = accent_override {
            thumb = mix(thumb, accent, if high_contrast { 0.35 } else { 0.25 });
            thumb_hover = mix(thumb_hover, accent, if high_contrast { 0.45 } else { 0.35 });
            thumb_active = mix(
                thumb_active,
                accent,
                if high_contrast { 0.55 } else { 0.45 },
            );
        }

        ScrollAreaColors {
            track,
            thumb,
            thumb_hover,
            thumb_active,
            focus_ring,
            corner: track,
        }
    }
}

struct ScrollBarContext<'a, R> {
    ui: &'a mut Ui,
    theme: &'a Theme,
    id: Id,
    props: &'a ScrollAreaProps,
    colors: &'a ScrollAreaColors,
    metrics: ScrollAreaMetrics,
    rounding: CornerRadius,
    output: &'a egui::scroll_area::ScrollAreaOutput<R>,
}

#[derive(Clone, Copy)]
struct ThumbInteractionGeometry {
    track: Rect,
    thumb: Rect,
    max_scroll: f32,
    vertical: bool,
}

#[derive(Clone, Debug)]
pub struct ScrollAreaProps {
    pub as_child: bool,
    pub id_source: Option<Id>,
    pub dir: Option<ScrollAreaDir>,
    pub nonce: Option<String>,
    pub direction: ScrollDirection,
    pub size: ScrollAreaSize,
    pub radius: ScrollAreaRadius,
    pub scroll_type: ScrollAreaType,
    pub scroll_hide_delay_ms: Option<f32>,
    pub force_mount: [bool; 2],
    pub auto_shrink: [bool; 2],
    pub max_size: Option<Vec2>,
    pub bar_visibility: ScrollBarVisibility,
    pub accent_color: Option<Color32>,
    pub high_contrast: bool,
    pub colors_override: Option<ScrollAreaColors>,
}

impl Default for ScrollAreaProps {
    fn default() -> Self {
        Self {
            as_child: false,
            id_source: None,
            dir: None,
            nonce: None,
            direction: ScrollDirection::Both,
            size: ScrollAreaSize::Size1,
            radius: ScrollAreaRadius::Full,
            scroll_type: ScrollAreaType::Hover,
            scroll_hide_delay_ms: Some(600.0),
            force_mount: [false; 2],
            auto_shrink: [true; 2],
            max_size: None,
            bar_visibility: ScrollBarVisibility::VisibleWhenNeeded,
            accent_color: None,
            high_contrast: false,
            colors_override: None,
        }
    }
}

impl ScrollAreaProps {
    pub fn as_child(mut self, as_child: bool) -> Self {
        self.as_child = as_child;
        self
    }

    pub fn with_id(mut self, id: Id) -> Self {
        self.id_source = Some(id);
        self
    }

    pub fn with_dir(mut self, dir: ScrollAreaDir) -> Self {
        self.dir = Some(dir);
        self
    }

    pub fn with_nonce(mut self, nonce: impl Into<String>) -> Self {
        self.nonce = Some(nonce.into());
        self
    }

    pub fn with_direction(mut self, direction: ScrollDirection) -> Self {
        self.direction = direction;
        self
    }

    pub fn with_size(mut self, size: ScrollAreaSize) -> Self {
        self.size = size;
        self
    }

    pub fn with_radius(mut self, radius: ScrollAreaRadius) -> Self {
        self.radius = radius;
        self
    }

    pub fn with_type(mut self, scroll_type: ScrollAreaType) -> Self {
        self.scroll_type = scroll_type;
        self
    }

    pub fn with_scroll_hide_delay(mut self, delay_ms: f32) -> Self {
        self.scroll_hide_delay_ms = Some(delay_ms);
        self
    }

    pub fn with_hide_delay_ms(mut self, delay_ms: f32) -> Self {
        self.scroll_hide_delay_ms = Some(delay_ms);
        self
    }

    pub fn with_force_mount(mut self, force_mount: [bool; 2]) -> Self {
        self.force_mount = force_mount;
        self
    }

    pub fn with_auto_shrink(mut self, auto_shrink: [bool; 2]) -> Self {
        self.auto_shrink = auto_shrink;
        self
    }

    pub fn with_max_size(mut self, max_size: Vec2) -> Self {
        self.max_size = Some(max_size);
        self
    }

    pub fn with_bar_visibility(mut self, visibility: ScrollBarVisibility) -> Self {
        self.bar_visibility = visibility;
        self
    }

    pub fn with_accent_color(mut self, accent: Color32) -> Self {
        self.accent_color = Some(accent);
        self
    }

    pub fn high_contrast(mut self, high_contrast: bool) -> Self {
        self.high_contrast = high_contrast;
        self
    }

    pub fn with_colors(mut self, colors: ScrollAreaColors) -> Self {
        self.colors_override = Some(colors);
        self
    }
}

pub fn scroll_area<R>(
    ui: &mut Ui,
    theme: &Theme,
    props: ScrollAreaProps,
    add_contents: impl FnOnce(&mut Ui) -> R,
) -> R {
    let id = props
        .id_source
        .unwrap_or_else(|| ui.make_persistent_id("scroll-area"));
    trace!("render scroll_area {:?}", id);

    let metrics = props.size.metrics();
    let rounding = props.radius.corner_radius();
    let colors = props.colors_override.unwrap_or_else(|| {
        ScrollAreaColors::from_palette(&theme.palette, props.accent_color, props.high_contrast)
    });

    let mut scroll = match props.direction {
        ScrollDirection::Horizontal => EguiScrollArea::horizontal(),
        ScrollDirection::Vertical => EguiScrollArea::vertical(),
        ScrollDirection::Both => EguiScrollArea::both(),
    };
    scroll = scroll
        .id_salt(id)
        .auto_shrink(props.auto_shrink)
        .scroll_bar_visibility(ScrollBarVisibility::AlwaysHidden);
    if let Some(max) = props.max_size {
        scroll = scroll.max_width(max.x).max_height(max.y);
    }

    let output = scroll.show(ui, add_contents);

    paint_scrollbars(ScrollBarContext {
        ui,
        theme,
        id,
        props: &props,
        colors: &colors,
        metrics,
        rounding,
        output: &output,
    });

    output.inner
}

fn paint_scrollbars<R>(ctx: ScrollBarContext<'_, R>) {
    let ScrollBarContext {
        ui,
        theme,
        id,
        props,
        colors,
        metrics,
        rounding,
        output,
    } = ctx;

    let egui_ctx = ui.ctx().clone();
    let now = egui_ctx.input(|i| i.time);
    let viewport_rect = output.inner_rect;
    let viewport_size = viewport_rect.size();
    let content_size = output.content_size;

    let needs_h = content_size.x > viewport_size.x + 1.0;
    let needs_v = content_size.y > viewport_size.y + 1.0;

    let hovered = egui_ctx
        .input(|i| i.pointer.hover_pos())
        .is_some_and(|pos| viewport_rect.contains(pos));

    let last_offset_key = id.with("last-offset");
    let last_scroll_key = id.with("last-scroll-time");
    let last_hover_key = id.with("last-hover-time");

    let last_offset = egui_ctx
        .data(|d| d.get_temp::<Vec2>(last_offset_key))
        .unwrap_or(output.state.offset);

    let scrolling_now = (output.state.offset - last_offset).length_sq() > 0.5;

    egui_ctx.data_mut(|d| {
        d.insert_temp(last_offset_key, output.state.offset);
        if hovered {
            d.insert_temp(last_hover_key, now);
        }
        if scrolling_now {
            d.insert_temp(last_scroll_key, now);
        }
    });

    let last_scroll_time = egui_ctx
        .data(|d| d.get_temp::<f64>(last_scroll_key))
        .unwrap_or(0.0);
    let last_hover_time = egui_ctx
        .data(|d| d.get_temp::<f64>(last_hover_key))
        .unwrap_or(0.0);

    let fade_duration = theme.motion.fast_ms.max(1.0) / 1000.0;
    let hide_delay_ms = props.scroll_hide_delay_ms.unwrap_or(600.0);
    let hide_delay_secs = hide_delay_ms as f64 / 1000.0;

    let base_visible_h = props.force_mount[0]
        || match props.bar_visibility {
            ScrollBarVisibility::AlwaysHidden => false,
            ScrollBarVisibility::AlwaysVisible => true,
            ScrollBarVisibility::VisibleWhenNeeded => needs_h,
        };
    let base_visible_v = props.force_mount[1]
        || match props.bar_visibility {
            ScrollBarVisibility::AlwaysHidden => false,
            ScrollBarVisibility::AlwaysVisible => true,
            ScrollBarVisibility::VisibleWhenNeeded => needs_v,
        };

    let scroll_recent = now - last_scroll_time <= hide_delay_secs;
    let hover_recent = now - last_hover_time <= hide_delay_secs;

    let want_visible_h = base_visible_h
        && matches!(
            props.direction,
            ScrollDirection::Horizontal | ScrollDirection::Both
        )
        && match props.scroll_type {
            ScrollAreaType::Always => true,
            ScrollAreaType::Hover => hovered || hover_recent,
            ScrollAreaType::Scroll => scrolling_now || scroll_recent,
            ScrollAreaType::Auto => hovered || scrolling_now || scroll_recent || hover_recent,
        };
    let want_visible_v = base_visible_v
        && matches!(
            props.direction,
            ScrollDirection::Vertical | ScrollDirection::Both
        )
        && match props.scroll_type {
            ScrollAreaType::Always => true,
            ScrollAreaType::Hover => hovered || hover_recent,
            ScrollAreaType::Scroll => scrolling_now || scroll_recent,
            ScrollAreaType::Auto => hovered || scrolling_now || scroll_recent || hover_recent,
        };

    if props.scroll_type == ScrollAreaType::Scroll
        && hide_delay_secs > 0.0
        && (base_visible_h || base_visible_v)
        && !scroll_recent
        && !scrolling_now
    {
        let elapsed = now - last_scroll_time;
        if elapsed < hide_delay_secs {
            egui_ctx.request_repaint_after(Duration::from_secs_f64(hide_delay_secs - elapsed));
        }
    }

    let alpha_h = egui_ctx.animate_bool_with_time_and_easing(
        id.with("bar-alpha-h"),
        want_visible_h,
        fade_duration,
        ease_out_cubic,
    );
    let alpha_v = egui_ctx.animate_bool_with_time_and_easing(
        id.with("bar-alpha-v"),
        want_visible_v,
        fade_duration,
        ease_out_cubic,
    );

    if alpha_h <= 0.0 && alpha_v <= 0.0 {
        return;
    }

    let bar_thickness = metrics.bar_thickness;
    let margin = metrics.bar_margin;
    let padding = metrics.bar_padding;

    let show_h = alpha_h > 0.0
        && matches!(
            props.direction,
            ScrollDirection::Horizontal | ScrollDirection::Both
        );
    let show_v = alpha_v > 0.0
        && matches!(
            props.direction,
            ScrollDirection::Vertical | ScrollDirection::Both
        );

    let mut h_track_rect = None;
    let mut v_track_rect = None;
    let mut corner_rect = None;

    if show_v {
        let track_height = viewport_rect.height()
            - margin * 2.0
            - if show_h { bar_thickness + margin } else { 0.0 };
        if track_height > 0.0 {
            let x = viewport_rect.right() - margin - bar_thickness;
            let y = viewport_rect.top() + margin;
            v_track_rect = Some(Rect::from_min_size(
                Pos2::new(x, y),
                Vec2::new(bar_thickness, track_height),
            ));
        }
    }

    if show_h {
        let track_width = viewport_rect.width()
            - margin * 2.0
            - if show_v { bar_thickness + margin } else { 0.0 };
        if track_width > 0.0 {
            let x = viewport_rect.left() + margin;
            let y = viewport_rect.bottom() - margin - bar_thickness;
            h_track_rect = Some(Rect::from_min_size(
                Pos2::new(x, y),
                Vec2::new(track_width, bar_thickness),
            ));
        }
    }

    if show_h && show_v {
        let x = viewport_rect.right() - margin - bar_thickness;
        let y = viewport_rect.bottom() - margin - bar_thickness;
        corner_rect = Some(Rect::from_min_size(
            Pos2::new(x, y),
            Vec2::splat(bar_thickness),
        ));
    }

    let pointer_pos = egui_ctx.input(|i| i.pointer.interact_pos());

    if let Some(track) = v_track_rect {
        let alpha = alpha_v;
        let track_color = apply_opacity(colors.track, alpha);
        let track_fill_rect = track.shrink(padding);
        ui.painter()
            .rect_filled(track_fill_rect, rounding, track_color);

        let (thumb_rect, max_scroll) = compute_thumb_rect(
            track_fill_rect,
            content_size.y,
            viewport_size.y,
            output.state.offset.y,
            metrics.handle_min_length,
            true,
        );

        let thumb_hovered = pointer_pos.is_some_and(|pos| thumb_rect.contains(pos));
        let dragging = handle_thumb_interaction(
            ui,
            &egui_ctx,
            id.with("v-thumb"),
            output.id,
            ThumbInteractionGeometry {
                track: track_fill_rect,
                thumb: thumb_rect,
                max_scroll,
                vertical: true,
            },
        );

        let thumb_color = if dragging {
            colors.thumb_active
        } else if thumb_hovered {
            colors.thumb_hover
        } else {
            colors.thumb
        };

        ui.painter()
            .rect_filled(thumb_rect, rounding, apply_opacity(thumb_color, alpha));
    }

    if let Some(track) = h_track_rect {
        let alpha = alpha_h;
        let track_color = apply_opacity(colors.track, alpha);
        let track_fill_rect = track.shrink(padding);
        ui.painter()
            .rect_filled(track_fill_rect, rounding, track_color);

        let (thumb_rect, max_scroll) = compute_thumb_rect(
            track_fill_rect,
            content_size.x,
            viewport_size.x,
            output.state.offset.x,
            metrics.handle_min_length,
            false,
        );

        let thumb_hovered = pointer_pos.is_some_and(|pos| thumb_rect.contains(pos));
        let dragging = handle_thumb_interaction(
            ui,
            &egui_ctx,
            id.with("h-thumb"),
            output.id,
            ThumbInteractionGeometry {
                track: track_fill_rect,
                thumb: thumb_rect,
                max_scroll,
                vertical: false,
            },
        );

        let thumb_color = if dragging {
            colors.thumb_active
        } else if thumb_hovered {
            colors.thumb_hover
        } else {
            colors.thumb
        };

        ui.painter()
            .rect_filled(thumb_rect, rounding, apply_opacity(thumb_color, alpha));
    }

    if let Some(corner) = corner_rect {
        let corner_color = apply_opacity(colors.corner, alpha_h.max(alpha_v));
        ui.painter()
            .rect_filled(corner, CornerRadius::same(0), corner_color);
    }

    if egui_ctx.memory(|m| m.has_focus(output.id)) {
        ui.painter().rect_stroke(
            viewport_rect,
            CornerRadius::same(0),
            colors.focus_ring,
            StrokeKind::Outside,
        );
    }
}

fn compute_thumb_rect(
    track: Rect,
    content_len: f32,
    viewport_len: f32,
    offset: f32,
    min_len: f32,
    vertical: bool,
) -> (Rect, f32) {
    let track_len = if vertical {
        track.height()
    } else {
        track.width()
    };
    let max_scroll = (content_len - viewport_len).max(0.0);

    let ratio = if content_len > 0.0 {
        (viewport_len / content_len).clamp(0.0, 1.0)
    } else {
        1.0
    };
    let thumb_len = (track_len * ratio).clamp(min_len, track_len.max(min_len));
    let available = (track_len - thumb_len).max(0.0);

    let pos = if max_scroll > 0.0 {
        (offset / max_scroll).clamp(0.0, 1.0) * available
    } else {
        0.0
    };

    if vertical {
        let min = Pos2::new(track.left(), track.top() + pos);
        (
            Rect::from_min_size(min, Vec2::new(track.width(), thumb_len)),
            max_scroll,
        )
    } else {
        let min = Pos2::new(track.left() + pos, track.top());
        (
            Rect::from_min_size(min, Vec2::new(thumb_len, track.height())),
            max_scroll,
        )
    }
}

fn handle_thumb_interaction(
    ui: &mut Ui,
    ctx: &egui::Context,
    thumb_id: Id,
    scroll_area_id: Id,
    geometry: ThumbInteractionGeometry,
) -> bool {
    let ThumbInteractionGeometry {
        track,
        thumb,
        max_scroll,
        vertical,
    } = geometry;
    let sense = Sense::click_and_drag();
    let thumb_resp = ui.interact(thumb, thumb_id, sense);
    let track_resp = ui.interact(track, thumb_id.with("track"), Sense::click());

    let drag_offset_key = thumb_id.with("drag-offset");

    if thumb_resp.drag_started()
        && let Some(pos) = thumb_resp.interact_pointer_pos()
    {
        let drag_offset = if vertical {
            pos.y - thumb.top()
        } else {
            pos.x - thumb.left()
        };
        ctx.data_mut(|d| d.insert_temp(drag_offset_key, drag_offset as f64));
    }

    if thumb_resp.dragged()
        && let Some(pos) = thumb_resp.interact_pointer_pos()
    {
        let stored_offset = ctx
            .data(|d| d.get_temp::<f64>(drag_offset_key))
            .unwrap_or(0.0) as f32;

        let pointer_axis = if vertical { pos.y } else { pos.x };
        let track_start = if vertical { track.top() } else { track.left() };
        let track_len = if vertical {
            track.height()
        } else {
            track.width()
        };
        let thumb_len = if vertical {
            thumb.height()
        } else {
            thumb.width()
        };
        let available = (track_len - thumb_len).max(0.0);

        let new_thumb_pos = (pointer_axis - stored_offset - track_start).clamp(0.0, available);
        let new_offset = if available > 0.0 && max_scroll > 0.0 {
            (new_thumb_pos / available) * max_scroll
        } else {
            0.0
        };

        if let Some(mut state) = EguiScrollState::load(ctx, scroll_area_id) {
            if vertical {
                state.offset.y = new_offset;
            } else {
                state.offset.x = new_offset;
            }
            state.store(ctx, scroll_area_id);
        }
        return true;
    }

    if thumb_resp.drag_stopped() {
        ctx.data_mut(|d| d.remove::<f64>(drag_offset_key));
    }

    if track_resp.clicked()
        && let Some(pos) = track_resp.interact_pointer_pos()
    {
        let pointer_axis = if vertical { pos.y } else { pos.x };
        let track_start = if vertical { track.top() } else { track.left() };
        let track_len = if vertical {
            track.height()
        } else {
            track.width()
        };
        let thumb_len = if vertical {
            thumb.height()
        } else {
            thumb.width()
        };
        let available = (track_len - thumb_len).max(0.0);

        let new_thumb_pos = (pointer_axis - track_start - thumb_len / 2.0).clamp(0.0, available);
        let new_offset = if available > 0.0 && max_scroll > 0.0 {
            (new_thumb_pos / available) * max_scroll
        } else {
            0.0
        };

        if let Some(mut state) = EguiScrollState::load(ctx, scroll_area_id) {
            if vertical {
                state.offset.y = new_offset;
            } else {
                state.offset.x = new_offset;
            }
            state.store(ctx, scroll_area_id);
        }
    }

    false
}

fn apply_opacity(color: Color32, opacity: f32) -> Color32 {
    let alpha = (color.a() as f32 * opacity.clamp(0.0, 1.0)).round() as u8;
    Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), alpha)
}
