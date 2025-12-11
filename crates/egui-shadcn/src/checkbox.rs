use crate::theme::{Theme, widget_visuals};
use crate::tokens::{
    ControlSize, ControlVariant, checkbox_metrics, checkbox_tokens_with_high_contrast, ease_out_cubic,
    mix,
};
use egui::style::Widgets;
use egui::{
    Color32, CornerRadius, CursorIcon, Id, Pos2, Response, Sense, Stroke, StrokeKind, TextStyle,
    Ui, Vec2, WidgetText,
};
use log::trace;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CheckboxState {
    Unchecked,
    Checked,
    Indeterminate,
}

impl CheckboxState {
    pub fn is_checked(self) -> bool {
        matches!(self, CheckboxState::Checked)
    }

    pub fn is_active(self) -> bool {
        matches!(self, CheckboxState::Checked | CheckboxState::Indeterminate)
    }

    pub fn is_indeterminate(self) -> bool {
        matches!(self, CheckboxState::Indeterminate)
    }

    pub fn toggle(&mut self, cycle: CheckboxCycle) {
        *self = match (cycle, *self) {
            (CheckboxCycle::Binary, CheckboxState::Unchecked) => CheckboxState::Checked,
            (CheckboxCycle::Binary, _) => CheckboxState::Unchecked,
            (CheckboxCycle::TriState, CheckboxState::Unchecked) => CheckboxState::Checked,
            (CheckboxCycle::TriState, CheckboxState::Checked) => CheckboxState::Indeterminate,
            (CheckboxCycle::TriState, CheckboxState::Indeterminate) => CheckboxState::Unchecked,
        };
    }
}

impl From<bool> for CheckboxState {
    fn from(value: bool) -> Self {
        if value {
            CheckboxState::Checked
        } else {
            CheckboxState::Unchecked
        }
    }
}

impl From<CheckboxState> for bool {
    fn from(value: CheckboxState) -> Self {
        value.is_checked()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CheckboxCycle {
    Binary,
    TriState,
}

#[derive(Clone, Copy, Debug)]
pub struct CheckboxOptions {
    pub variant: ControlVariant,
    pub size: ControlSize,
    pub enabled: bool,
    pub invalid: bool,
    pub cycle: CheckboxCycle,
    pub animate: bool,
    pub high_contrast: bool,
}

impl Default for CheckboxOptions {
    fn default() -> Self {
        Self {
            variant: ControlVariant::Secondary,
            size: ControlSize::Md,
            enabled: true,
            invalid: false,
            cycle: CheckboxCycle::Binary,
            animate: true,
            high_contrast: false,
        }
    }
}

pub fn checkbox(
    ui: &mut Ui,
    theme: &Theme,
    checked: &mut bool,
    label: impl Into<WidgetText>,
    variant: ControlVariant,
    size: ControlSize,
    enabled: bool,
) -> Response {
    trace!(
        "Rendering checkbox variant={:?} size={:?} enabled={}",
        variant, size, enabled
    );
    let mut state = CheckboxState::from(*checked);
    let response = checkbox_state(
        ui,
        theme,
        &mut state,
        label,
        CheckboxOptions {
            variant,
            size,
            enabled,
            ..CheckboxOptions::default()
        },
    );
    *checked = bool::from(state);
    response
}

pub fn checkbox_state(
    ui: &mut Ui,
    theme: &Theme,
    state: &mut CheckboxState,
    label: impl Into<WidgetText>,
    options: CheckboxOptions,
) -> Response {
    let label_text: WidgetText = label.into();
    trace!(
        "Rendering checkbox state={:?} variant={:?} size={:?} enabled={} invalid={} animate={}",
        state, options.variant, options.size, options.enabled, options.invalid, options.animate
    );

    let CheckboxOptions {
        variant,
        size,
        enabled,
        invalid,
        cycle,
        animate,
        high_contrast,
    } = options;
    let visuals = theme.control(variant, size);
    let metrics = checkbox_metrics(size);
    let toggle_tokens = checkbox_tokens_with_high_contrast(&theme.palette, variant, high_contrast);
    let rounding = CornerRadius::same((metrics.track_size.x * 0.25).round() as u8);
    let icon_spacing = visuals.padding.x * 0.35;
    let focus_ring = Stroke::new(
        3.0,
        mix(
            toggle_tokens.on.idle.bg_fill,
            theme.palette.foreground,
            if high_contrast { 0.35 } else { 0.15 },
        ),
    );
    let invalid_ring = Stroke::new(3.0, theme.palette.destructive);
    let expansion = size.expansion();
    let widgets = Widgets {
        noninteractive: widget_visuals(&toggle_tokens.disabled, rounding, expansion),
        inactive: widget_visuals(&toggle_tokens.off.idle, rounding, expansion),
        hovered: widget_visuals(&toggle_tokens.off.hovered, rounding, expansion),
        active: widget_visuals(&toggle_tokens.on.active, rounding, expansion),
        open: widget_visuals(&toggle_tokens.off.hovered, rounding, expansion),
    };

    theme.scoped(ui, widgets, |scoped_ui| {
        let mut style = scoped_ui.style().as_ref().clone();
        style.spacing.icon_width = metrics.track_size.x;
        style.spacing.icon_width_inner = metrics.thumb_size.x;
        style.spacing.icon_spacing = icon_spacing;
        style.spacing.item_spacing.x = icon_spacing;
        style.spacing.item_spacing.y = visuals.padding.y * 0.25;
        style
            .text_styles
            .insert(TextStyle::Body, visuals.text_style.clone());
        scoped_ui.set_style(style);

        scoped_ui
            .horizontal(|row| {
                let sense = if enabled {
                    Sense::click()
                } else {
                    Sense::hover()
                };
                let (icon_rect, icon_response) = row.allocate_exact_size(metrics.track_size, sense);
                let label_response =
                    row.add_enabled(enabled, egui::Label::new(label_text.clone()).wrap());

                let clicked_icon = icon_response.clicked();
                let clicked_label = label_response.clicked();
                let clicked = enabled && (clicked_icon || clicked_label);
                if clicked {
                    state.toggle(cycle);
                }

                let anim_id: Id = icon_response.id.with("checkbox");
                let anim_duration = theme.motion.base_ms / 1000.0;
                let on_t = if animate {
                    row.ctx().animate_bool_with_time_and_easing(
                        anim_id,
                        state.is_active(),
                        anim_duration,
                        ease_out_cubic,
                    )
                } else if state.is_active() {
                    1.0
                } else {
                    0.0
                };
                let indeterminate_t = if animate {
                    row.ctx().animate_bool_with_time_and_easing(
                        anim_id.with("indeterminate"),
                        state.is_indeterminate(),
                        anim_duration,
                        ease_out_cubic,
                    )
                } else if state.is_indeterminate() {
                    1.0
                } else {
                    0.0
                };

                let pointer_down = icon_response.is_pointer_button_down_on();
                let hovered_icon = icon_response.hovered();
                let has_focus = icon_response.has_focus();

                let mut off_state = toggle_tokens.off.idle;
                let mut on_state = toggle_tokens.on.idle;
                if !enabled {
                    off_state = toggle_tokens.disabled;
                    on_state = toggle_tokens.disabled;
                } else if pointer_down {
                    off_state = toggle_tokens.off.active;
                    on_state = toggle_tokens.on.active;
                } else if hovered_icon {
                    off_state = toggle_tokens.off.hovered;
                    on_state = toggle_tokens.on.hovered;
                }
                let track_state = lerp_state(off_state, on_state, on_t);

                let painter = row.painter_at(icon_rect.expand(focus_ring.width));
                let track_rect =
                    egui::Rect::from_center_size(icon_rect.center(), metrics.track_size);

                painter.rect_filled(track_rect, rounding, track_state.bg_fill);
                if track_state.border != Stroke::NONE {
                    painter.rect_stroke(
                        track_rect,
                        rounding,
                        track_state.border,
                        StrokeKind::Outside,
                    );
                }

                paint_indicator(
                    &painter,
                    track_rect,
                    track_state.fg_stroke.color,
                    on_t,
                    indeterminate_t,
                    metrics.thumb_size,
                );

                if invalid && enabled {
                    let ring_rect = track_rect.expand(invalid_ring.width * 0.5 + 1.0);
                    painter.rect_stroke(ring_rect, rounding, invalid_ring, StrokeKind::Outside);
                } else if has_focus && enabled {
                    let ring_rect = track_rect.expand(focus_ring.width * 0.5 + 0.5);
                    painter.rect_stroke(ring_rect, rounding, focus_ring, StrokeKind::Outside);
                }

                let mut response = icon_response | label_response;
                if clicked {
                    response.mark_changed();
                }
                if enabled {
                    response = response.on_hover_cursor(CursorIcon::PointingHand);
                }
                response
            })
            .inner
    })
}

fn lerp_state(
    off: crate::tokens::StateColors,
    on: crate::tokens::StateColors,
    t: f32,
) -> crate::tokens::StateColors {
    crate::tokens::StateColors {
        bg_fill: mix(off.bg_fill, on.bg_fill, t),
        fg_stroke: lerp_stroke(off.fg_stroke, on.fg_stroke, t),
        border: lerp_stroke(off.border, on.border, t),
    }
}

fn lerp_stroke(a: Stroke, b: Stroke, t: f32) -> Stroke {
    Stroke {
        width: a.width + (b.width - a.width) * t,
        color: mix(a.color, b.color, t),
    }
}

fn paint_indicator(
    painter: &egui::Painter,
    track_rect: egui::Rect,
    color: Color32,
    on_t: f32,
    indeterminate_t: f32,
    thumb_size: Vec2,
) {
    let check_weight = thumb_size.x.mul_add(0.22, 1.2).clamp(1.5, 2.4);
    let check_alpha = on_t * (1.0 - indeterminate_t);
    if check_alpha > 0.0 {
        let points = [
            Pos2::new(
                track_rect.left() + track_rect.width() * 0.26,
                track_rect.center().y,
            ),
            Pos2::new(
                track_rect.left() + track_rect.width() * 0.45,
                track_rect.bottom() - track_rect.height() * 0.28,
            ),
            Pos2::new(
                track_rect.right() - track_rect.width() * 0.2,
                track_rect.top() + track_rect.height() * 0.3,
            ),
        ];
        painter.add(egui::Shape::line(
            vec![points[0], points[1], points[2]],
            Stroke::new(check_weight, scale_alpha(color, check_alpha)),
        ));
    }

    let dash_alpha = on_t * indeterminate_t;
    if dash_alpha > 0.0 {
        let dash_height = check_weight;
        let dash_rect = egui::Rect::from_center_size(
            track_rect.center(),
            Vec2::new(track_rect.width() * 0.52, dash_height),
        );
        painter.rect_filled(
            dash_rect,
            CornerRadius::same((dash_height * 0.5) as u8),
            scale_alpha(color, dash_alpha),
        );
    }
}

fn scale_alpha(color: Color32, factor: f32) -> Color32 {
    let clamped = factor.clamp(0.0, 1.0);
    let [r, g, b, a] = color.to_array();
    let alpha = ((a as f32) * clamped).round() as u8;
    Color32::from_rgba_unmultiplied(r, g, b, alpha)
}
