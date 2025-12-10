use crate::theme::Theme;
use crate::tokens::{ControlSize, ControlVariant, mix, switch_metrics, switch_tokens};
use egui::{CornerRadius, Id, Response, Sense, Stroke, StrokeKind, Ui, Vec2, WidgetText};
use log::trace;

fn lerp_stroke(a: Stroke, b: Stroke, t: f32) -> Stroke {
    Stroke {
        width: a.width + (b.width - a.width) * t,
        color: mix(a.color, b.color, t),
    }
}

fn lerp_state(
    off: &crate::tokens::StateColors,
    on: &crate::tokens::StateColors,
    t: f32,
) -> crate::tokens::StateColors {
    crate::tokens::StateColors {
        bg_fill: mix(off.bg_fill, on.bg_fill, t),
        fg_stroke: lerp_stroke(off.fg_stroke, on.fg_stroke, t),
        border: lerp_stroke(off.border, on.border, t),
    }
}

pub fn switch(
    ui: &mut Ui,
    theme: &Theme,
    on: &mut bool,
    label: impl Into<WidgetText>,
    variant: ControlVariant,
    size: ControlSize,
    enabled: bool,
) -> Response {
    trace!(
        "Rendering switch variant={:?} size={:?} enabled={}",
        variant, size, enabled
    );
    let metrics = switch_metrics(size);
    let tokens = switch_tokens(&theme.palette, variant);
    let rounding = CornerRadius::same((metrics.track_size.y * 0.5).round() as u8);
    let track_size = metrics.track_size;
    let thumb_size = metrics.thumb_size;
    let visuals = theme.input(size);

    let desired_size = Vec2::new(track_size.x, track_size.y).max(ui.spacing().interact_size);
    let (hit_rect, mut response) = ui.allocate_exact_size(desired_size, Sense::click());
    let track_rect = egui::Rect::from_center_size(hit_rect.center(), track_size);

    let anim_id: Id = response.id;
    let t = ui.ctx().animate_bool(anim_id, *on);

    let mut off_state = tokens.off.idle;
    let mut on_state = tokens.on.idle;
    if !enabled {
        off_state = tokens.disabled;
        on_state = tokens.disabled;
    } else if response.is_pointer_button_down_on() {
        off_state = tokens.off.active;
        on_state = tokens.on.active;
    } else if response.hovered() {
        off_state = tokens.off.hovered;
        on_state = tokens.on.hovered;
    }

    let track_state = lerp_state(&off_state, &on_state, t);

    if response.clicked() && enabled {
        *on = !*on;
        response.mark_changed();
    }

    let painter = ui.painter();
    painter.rect_filled(track_rect, rounding, track_state.bg_fill);
    if track_state.border != Stroke::NONE {
        painter.rect_stroke(
            track_rect,
            rounding,
            track_state.border,
            StrokeKind::Outside,
        );
    }

    let padding = (track_size.y - thumb_size.y) * 0.5;
    let off_x = track_rect.left() + padding;
    let on_x = track_rect.right() - padding - thumb_size.x;
    let thumb_x = off_x + (on_x - off_x) * t;
    let thumb_rect =
        egui::Rect::from_min_size(egui::pos2(thumb_x, track_rect.top() + padding), thumb_size);
    let thumb_rounding = CornerRadius::same((thumb_size.y * 0.5) as u8);

    let thumb_color = mix(tokens.thumb_off, tokens.thumb_on, t);
    painter.rect_filled(thumb_rect, thumb_rounding, thumb_color);

    if response.has_focus() {
        let ring_rect = track_rect.expand(visuals.focus_stroke.width * 0.5);
        painter.rect_stroke(
            ring_rect,
            rounding,
            visuals.focus_stroke,
            StrokeKind::Outside,
        );
    }

    let label_resp = ui.add_enabled(enabled, egui::Label::new(label).wrap());
    if label_resp.clicked() && enabled {
        *on = !*on;
        response.mark_changed();
    }

    response | label_resp
}
