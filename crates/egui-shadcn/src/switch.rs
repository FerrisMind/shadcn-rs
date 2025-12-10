use crate::theme::Theme;
use crate::tokens::{
    ControlSize, ControlVariant, SwitchSize, SwitchTokenOptions, SwitchVariant, mix,
    switch_metrics, switch_tokens_with_options,
};
use egui::{
    Color32, CornerRadius, CursorIcon, Id, Response, Sense, Stroke, StrokeKind, Ui, Vec2,
    WidgetText,
};
use log::trace;

#[derive(Clone, Debug)]
pub struct SwitchOptions {
    pub size: SwitchSize,
    pub style: SwitchVariant,
    pub enabled: bool,
    pub high_contrast: bool,
    pub animate: bool,
    pub accent: Option<Color32>,
    pub corner_radius: Option<CornerRadius>,
    pub thumb_color: Option<Color32>,
}

impl Default for SwitchOptions {
    fn default() -> Self {
        Self {
            size: SwitchSize::Two,
            style: SwitchVariant::Surface,
            enabled: true,
            high_contrast: false,
            animate: true,
            accent: None,
            corner_radius: None,
            thumb_color: None,
        }
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
    let accent = accent_from_control_variant(&theme.palette, variant);
    switch_with_options(
        ui,
        theme,
        on,
        label,
        SwitchOptions {
            size: SwitchSize::from(size),
            enabled,
            accent: Some(accent),
            ..SwitchOptions::default()
        },
    )
}

pub fn switch_with_options(
    ui: &mut Ui,
    theme: &Theme,
    on: &mut bool,
    label: impl Into<WidgetText>,
    options: SwitchOptions,
) -> Response {
    let label_text: WidgetText = label.into();
    trace!(
        "Rendering switch style={:?} size={:?} enabled={} high_contrast={} animate={}",
        options.style, options.size, options.enabled, options.high_contrast, options.animate
    );

    let metrics = switch_metrics(options.size);
    let rounding = options
        .corner_radius
        .unwrap_or_else(|| CornerRadius::same((metrics.track_size.y * 0.5).round() as u8));
    let focus_size: ControlSize = options.size.into();
    let _visuals = theme.input(focus_size);
    let tokens = switch_tokens_with_options(
        &theme.palette,
        SwitchTokenOptions {
            variant: options.style,
            high_contrast: options.high_contrast,
            accent: options.accent.unwrap_or(theme.palette.primary),
            thumb_color: options.thumb_color,
        },
    );
    let desired_size =
        Vec2::new(metrics.track_size.x, metrics.track_size.y).max(ui.spacing().interact_size);
    let (hit_rect, mut response) = ui.allocate_exact_size(desired_size, Sense::click());
    let track_rect = egui::Rect::from_center_size(hit_rect.center(), metrics.track_size);

    let anim_id: Id = response.id;
    let t = if options.animate {
        ui.ctx().animate_bool(anim_id, *on)
    } else if *on {
        1.0
    } else {
        0.0
    };

    let mut off_state = tokens.toggle.off.idle;
    let mut on_state = tokens.toggle.on.idle;
    if !options.enabled {
        off_state = tokens.toggle.disabled;
        on_state = tokens.toggle.disabled;
    } else if response.is_pointer_button_down_on() {
        off_state = tokens.toggle.off.active;
        on_state = tokens.toggle.on.active;
    } else if response.hovered() {
        off_state = tokens.toggle.off.hovered;
        on_state = tokens.toggle.on.hovered;
    }

    if response.clicked() && options.enabled {
        *on = !*on;
        response.mark_changed();
    }

    let track_state = lerp_state(&off_state, &on_state, t);
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

    let padding = (metrics.track_size.y - metrics.thumb_size.y) * 0.5;
    let off_x = track_rect.left() + padding;
    let on_x = track_rect.right() - padding - metrics.thumb_size.x;
    let thumb_x = off_x + (on_x - off_x) * t;
    let thumb_rect = egui::Rect::from_min_size(
        egui::pos2(thumb_x, track_rect.top() + padding),
        metrics.thumb_size,
    );
    let thumb_rounding = options
        .corner_radius
        .unwrap_or_else(|| CornerRadius::same((metrics.thumb_size.y * 0.5) as u8));
    let thumb_color = mix(tokens.toggle.thumb_off, tokens.toggle.thumb_on, t);
    painter.rect_filled(thumb_rect, thumb_rounding, thumb_color);

    if response.has_focus() && options.enabled {
        let ring_rect = track_rect.expand(tokens.focus_ring.width * 0.5 + 0.5);
        painter.rect_stroke(ring_rect, rounding, tokens.focus_ring, StrokeKind::Outside);
    }

    let label_resp = ui.add_enabled(options.enabled, egui::Label::new(label_text).wrap());
    if label_resp.clicked() && options.enabled {
        *on = !*on;
        response.mark_changed();
    }

    let mut combined = response | label_resp;
    if options.enabled {
        combined = combined.on_hover_cursor(CursorIcon::PointingHand);
    }
    combined
}

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

fn accent_from_control_variant(
    palette: &crate::tokens::ColorPalette,
    variant: ControlVariant,
) -> Color32 {
    match variant {
        ControlVariant::Primary | ControlVariant::Link => palette.primary,
        ControlVariant::Destructive => palette.destructive,
        ControlVariant::Secondary => palette.secondary,
        ControlVariant::Ghost | ControlVariant::Outline => palette.accent,
    }
}
