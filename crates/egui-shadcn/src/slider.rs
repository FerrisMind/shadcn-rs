use crate::theme::Theme;
use crate::tokens::{ColorPalette, ControlSize, ease_out_cubic, mix};
use crate::tooltip::{TooltipProps, TooltipSide, tooltip};
use egui::{
    Color32, CornerRadius, CursorIcon, Key, Rect, Response, Sense, Stroke, StrokeKind, Ui, Vec2,
    pos2, vec2,
};
use log::trace;
use std::cmp::Ordering;
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SliderSize {
    Size1,

    #[default]
    Size2,

    Size3,
}

impl SliderSize {
    pub fn track_size(self) -> f32 {
        match self {
            SliderSize::Size1 => 4.0,
            SliderSize::Size2 => 6.0,
            SliderSize::Size3 => 8.0,
        }
    }

    pub fn thumb_size(self) -> f32 {
        self.track_size() * 2.0 + 4.0
    }

    pub fn thumb_hit_size(self) -> f32 {
        self.thumb_size() * 3.0
    }
}

impl From<ControlSize> for SliderSize {
    fn from(size: ControlSize) -> Self {
        match size {
            ControlSize::Sm | ControlSize::IconSm => SliderSize::Size1,
            ControlSize::Md | ControlSize::Icon => SliderSize::Size2,
            ControlSize::Lg | ControlSize::IconLg => SliderSize::Size3,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SliderVariant {
    Classic,

    #[default]
    Surface,

    Soft,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SliderRadius {
    None,

    Small,

    Medium,

    Large,

    #[default]
    Full,
}

impl SliderRadius {
    pub fn corner_radius(self, track_size: f32) -> CornerRadius {
        let radius = match self {
            SliderRadius::None => 0.0,
            SliderRadius::Small => 4.0,
            SliderRadius::Medium => (track_size / 3.0).max(4.0),
            SliderRadius::Large => 8.0,
            SliderRadius::Full => track_size * 0.5,
        };
        CornerRadius::same(radius.round().clamp(0.0, u8::MAX as f32) as u8)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SliderTokens {
    pub track_bg: Color32,
    pub track_border: Stroke,
    pub range_bg: Color32,
    pub range_border: Stroke,
    pub thumb_bg: Color32,
    pub thumb_border: Stroke,
    pub thumb_disabled_bg: Color32,
    pub thumb_disabled_border: Stroke,
    pub ring_color: Color32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SliderTokenOptions {
    pub variant: SliderVariant,
    pub high_contrast: bool,
    pub accent: Color32,
}

pub fn slider_tokens(palette: &ColorPalette, options: SliderTokenOptions) -> SliderTokens {
    let track_bg = match options.variant {
        SliderVariant::Surface => mix(palette.muted, palette.background, 0.3),
        SliderVariant::Classic => mix(palette.muted, palette.background, 0.3),
        SliderVariant::Soft => mix(palette.muted, palette.background, 0.4),
    };

    let track_border = match options.variant {
        SliderVariant::Surface => Stroke::new(1.0, mix(palette.border, palette.foreground, 0.1)),
        SliderVariant::Classic => Stroke::NONE,
        SliderVariant::Soft => Stroke::NONE,
    };

    let range_bg = if options.high_contrast {
        mix(options.accent, Color32::BLACK, 0.2)
    } else {
        options.accent
    };

    let range_border = match options.variant {
        SliderVariant::Surface => Stroke::new(1.0, mix(palette.border, palette.foreground, 0.1)),
        SliderVariant::Classic => Stroke::new(
            1.0,
            mix(
                mix(palette.border, palette.foreground, 0.1),
                options.accent,
                0.3,
            ),
        ),
        SliderVariant::Soft => Stroke::NONE,
    };

    let thumb_bg = Color32::WHITE;
    let accent_border = if options.high_contrast {
        mix(options.accent, Color32::BLACK, 0.2)
    } else {
        options.accent
    };
    let thumb_border = match options.variant {
        SliderVariant::Surface => Stroke::new(1.0, accent_border),
        SliderVariant::Classic => Stroke::new(1.0, Color32::from_rgba_unmultiplied(0, 0, 0, 76)),
        SliderVariant::Soft => Stroke::new(1.0, Color32::from_rgba_unmultiplied(0, 0, 0, 76)),
    };

    let thumb_disabled_bg = mix(palette.muted, palette.background, 0.6);
    let thumb_disabled_border =
        Stroke::new(1.0, mix(palette.border, palette.muted_foreground, 0.5));

    let ring_color = Color32::from_rgba_unmultiplied(
        palette.ring.r(),
        palette.ring.g(),
        palette.ring.b(),
        128,
    );

    SliderTokens {
        track_bg,
        track_border,
        range_bg,
        range_border,
        thumb_bg,
        thumb_border,
        thumb_disabled_bg,
        thumb_disabled_border,
        ring_color,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SliderOrientation {
    #[default]
    Horizontal,
    Vertical,
}

pub struct SliderProps<'a, Id> {
    pub id_source: Id,
    pub values: &'a mut Vec<f32>,
    pub min: f32,
    pub max: f32,
    pub step: Option<f32>,
    pub min_steps_between_thumbs: Option<usize>,
    pub disabled: bool,
    pub size: SliderSize,
    pub variant: SliderVariant,
    pub radius: SliderRadius,
    pub high_contrast: bool,
    pub accent: Option<Color32>,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub orientation: SliderOrientation,
    pub animate: bool,
    pub show_value_tooltip: bool,
    pub on_value_change: Option<SliderValueChangeCallback<'a>>,
}

type SliderValueChangeCallback<'a> = Box<dyn FnMut(&[f32]) + 'a>;

impl<'a, Id: Hash + Debug> SliderProps<'a, Id> {
    pub fn new(id_source: Id, values: &'a mut Vec<f32>) -> Self {
        Self {
            id_source,
            values,
            min: 0.0,
            max: 100.0,
            step: None,
            min_steps_between_thumbs: None,
            disabled: false,
            size: SliderSize::Size2,
            variant: SliderVariant::Surface,
            radius: SliderRadius::Full,
            high_contrast: false,
            accent: None,
            width: None,
            height: None,
            orientation: SliderOrientation::Horizontal,
            animate: true,
            show_value_tooltip: false,
            on_value_change: None,
        }
    }

    pub fn min(mut self, min: f32) -> Self {
        self.min = min;
        self
    }

    pub fn max(mut self, max: f32) -> Self {
        self.max = max;
        self
    }

    pub fn step(mut self, step: f32) -> Self {
        self.step = Some(step);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn size(mut self, size: SliderSize) -> Self {
        self.size = size;
        self
    }

    pub fn variant(mut self, variant: SliderVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn radius(mut self, radius: SliderRadius) -> Self {
        self.radius = radius;
        self
    }

    pub fn high_contrast(mut self, high_contrast: bool) -> Self {
        self.high_contrast = high_contrast;
        self
    }

    pub fn accent(mut self, accent: Color32) -> Self {
        self.accent = Some(accent);
        self
    }

    pub fn color(mut self, color: Color32) -> Self {
        self.accent = Some(color);
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.height = Some(height);
        self
    }

    pub fn orientation(mut self, orientation: SliderOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn animate(mut self, animate: bool) -> Self {
        self.animate = animate;
        self
    }

    pub fn show_value_tooltip(mut self, show: bool) -> Self {
        self.show_value_tooltip = show;
        self
    }

    pub fn on_value_change<F>(mut self, callback: F) -> Self
    where
        F: FnMut(&[f32]) + 'a,
    {
        self.on_value_change = Some(Box::new(callback));
        self
    }

    pub fn min_steps_between_thumbs(mut self, steps: usize) -> Self {
        self.min_steps_between_thumbs = Some(steps);
        self
    }
}

fn snap_to_step(value: f32, min: f32, max: f32, step: Option<f32>) -> f32 {
    if let Some(step) = step {
        let steps = ((value - min) / step).round();
        (min + steps * step).clamp(min, max)
    } else {
        value.clamp(min, max)
    }
}

fn value_to_position(value: f32, min: f32, max: f32, track_length: f32) -> f32 {
    if max <= min {
        return 0.0;
    }
    let normalized = (value - min) / (max - min);
    normalized * track_length
}

fn position_to_value(pos: f32, min: f32, max: f32, track_length: f32, step: Option<f32>) -> f32 {
    if track_length <= 0.0 || max <= min {
        return min;
    }
    let normalized = (pos / track_length).clamp(0.0, 1.0);
    let value = min + normalized * (max - min);
    snap_to_step(value, min, max, step)
}

fn min_steps_gap(step: Option<f32>, min_steps: Option<usize>) -> f32 {
    let steps = min_steps.unwrap_or(0);
    if steps == 0 {
        return 0.0;
    }
    let step_value = step.unwrap_or(1.0).max(1e-6);
    (steps as f32) * step_value
}

fn thumb_value_bounds(values: &[f32], idx: usize, min: f32, max: f32, min_gap: f32) -> (f32, f32) {
    if values.len() <= 1 || min_gap <= 0.0 || min >= max || idx >= values.len() {
        return (min, max);
    }

    let mut sorted: Vec<(usize, f32)> = values.iter().cloned().enumerate().collect();
    sorted.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal));

    let pos = match sorted.iter().position(|(i, _)| *i == idx) {
        Some(p) => p,
        None => return (min, max),
    };

    let lower = if pos == 0 {
        min
    } else {
        sorted[pos - 1].1 + min_gap
    };

    let upper = if pos + 1 == sorted.len() {
        max
    } else {
        sorted[pos + 1].1 - min_gap
    };

    let lower = lower.clamp(min, max);
    let upper = upper.clamp(min, max);

    if lower > upper {
        (min, max)
    } else {
        (lower, upper)
    }
}

fn clamp_thumb_value(
    values: &[f32],
    idx: usize,
    desired_value: f32,
    min: f32,
    max: f32,
    min_gap: f32,
) -> f32 {
    let (lower, upper) = thumb_value_bounds(values, idx, min, max, min_gap);
    desired_value.clamp(lower, upper)
}

pub fn slider_with_props<Id>(ui: &mut Ui, theme: &Theme, mut props: SliderProps<'_, Id>) -> Response
where
    Id: Hash + Debug,
{
    trace!(
        "Rendering slider size={:?} variant={:?} disabled={} values={:?} orientation={:?} animate={}",
        props.size, props.variant, props.disabled, props.values, props.orientation, props.animate
    );

    if props.values.is_empty() {
        props.values.push(props.min);
    }

    for value in props.values.iter_mut() {
        *value = value.clamp(props.min, props.max);
        if let Some(step) = props.step {
            *value = snap_to_step(*value, props.min, props.max, Some(step));
        }
    }

    let track_size = props.size.track_size();
    let thumb_size = props.size.thumb_size();
    let thumb_hit_size = props.size.thumb_hit_size();

    let is_vertical = props.orientation == SliderOrientation::Vertical;
    let default_width = if is_vertical { thumb_size } else { 200.0 };
    let default_height = if is_vertical { 176.0 } else { thumb_size };

    let width = props.width.unwrap_or(default_width);
    let height = props.height.unwrap_or(default_height);

    let id = ui.make_persistent_id(&props.id_source);
    let desired_size = Vec2::new(width, height);
    let (rect, mut response) = ui.allocate_exact_size(desired_size, Sense::click_and_drag());

    let track_rect = if is_vertical {
        Rect::from_min_size(
            pos2(rect.center().x - track_size * 0.5, rect.top()),
            vec2(track_size, rect.height()),
        )
    } else {
        Rect::from_min_size(
            pos2(rect.left(), rect.center().y - track_size * 0.5),
            vec2(rect.width(), track_size),
        )
    };

    let track_length = if is_vertical {
        track_rect.height()
    } else {
        track_rect.width()
    };
    let rounding = props.radius.corner_radius(track_size);

    let accent = props.accent.unwrap_or(theme.palette.primary);
    let tokens = slider_tokens(
        &theme.palette,
        SliderTokenOptions {
            variant: props.variant,
            high_contrast: props.high_contrast,
            accent,
        },
    );
    let anim_duration = theme.motion.fast_ms / 1000.0;
    let min_gap = min_steps_gap(props.step, props.min_steps_between_thumbs);

    let focused_thumb_id = id.with("focused_thumb");
    let focused_thumb_idx = ui
        .ctx()
        .memory_mut(|m| m.data.get_persisted::<usize>(focused_thumb_id).unwrap_or(0));

    let mut thumb_responses = Vec::new();
    for (idx, value) in props.values.iter().enumerate() {
        let thumb_pos = value_to_position(*value, props.min, props.max, track_length);
        let thumb_center = if is_vertical {
            pos2(track_rect.center().x, track_rect.bottom() - thumb_pos)
        } else {
            pos2(track_rect.left() + thumb_pos, track_rect.center().y)
        };
        let thumb_hit_rect = Rect::from_center_size(thumb_center, Vec2::splat(thumb_hit_size));
        let is_focused = focused_thumb_idx == idx;
        let show_tooltip = props.show_value_tooltip && is_focused;
        if show_tooltip {
            let _ = ui.allocate_response(thumb_hit_rect.size(), Sense::hover());
        }
        thumb_responses.push((idx, thumb_hit_rect, thumb_center));
    }

    let painter = ui.painter();

    painter.rect_filled(track_rect, rounding, tokens.track_bg);
    if tokens.track_border != Stroke::NONE {
        painter.rect_stroke(
            track_rect,
            rounding,
            tokens.track_border,
            StrokeKind::Inside,
        );
    }

    let mut values_sorted = props.values.clone();
    values_sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

    if !values_sorted.is_empty() {
        let range_start = value_to_position(values_sorted[0], props.min, props.max, track_length);
        let range_end = if values_sorted.len() > 1 {
            value_to_position(
                values_sorted[values_sorted.len() - 1],
                props.min,
                props.max,
                track_length,
            )
        } else {
            range_start
        };

        let range_rect = if is_vertical {
            Rect::from_min_size(
                pos2(
                    track_rect.left(),
                    track_rect.top() + (track_length - range_end),
                ),
                vec2(track_rect.width(), (range_end - range_start).max(0.0)),
            )
        } else {
            Rect::from_min_size(
                pos2(track_rect.left() + range_start, track_rect.top()),
                vec2((range_end - range_start).max(0.0), track_rect.height()),
            )
        };

        painter.rect_filled(range_rect, rounding, tokens.range_bg);
        if tokens.range_border != Stroke::NONE {
            painter.rect_stroke(
                range_rect,
                rounding,
                tokens.range_border,
                StrokeKind::Inside,
            );
        }
    }

    let focused_thumb_id = id.with("focused_thumb");
    let mut focused_thumb_idx = ui
        .ctx()
        .memory_mut(|m| m.data.get_persisted::<usize>(focused_thumb_id).unwrap_or(0));
    let dragged_thumb_id = id.with("dragged_thumb");
    let mut dragged_thumb_idx = ui
        .ctx()
        .memory_mut(|m| m.data.get_persisted::<Option<usize>>(dragged_thumb_id).unwrap_or(None));

    if response.clicked() && !props.disabled {
        let pointer_pos = ui.input(|i| i.pointer.hover_pos());
        if let Some(pos) = pointer_pos {
            let local_pos = if is_vertical {
                track_length - (pos.y - track_rect.top())
            } else {
                pos.x - track_rect.left()
            };
            let new_value =
                position_to_value(local_pos, props.min, props.max, track_length, props.step);

            if props.values.len() == 1 {
                props.values[0] = new_value;
            } else {
                let mut closest_idx = 0;
                let mut closest_dist = f32::MAX;
                for (idx, value) in props.values.iter().enumerate() {
                    let thumb_pos = value_to_position(*value, props.min, props.max, track_length);
                    let dist = (local_pos - thumb_pos).abs();
                    if dist < closest_dist {
                        closest_dist = dist;
                        closest_idx = idx;
                    }
                }
                if closest_dist < thumb_hit_size * 0.5 {
                    focused_thumb_idx = closest_idx;
                } else {
                    let mut sorted_indices: Vec<(usize, f32)> = props
                        .values
                        .iter()
                        .enumerate()
                        .map(|(i, v)| (i, *v))
                        .collect();
                    sorted_indices
                        .sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

                    let insert_idx = sorted_indices
                        .iter()
                        .position(|(_, v)| *v > new_value)
                        .unwrap_or(sorted_indices.len());
                    if insert_idx < props.values.len() {
                        focused_thumb_idx = sorted_indices[insert_idx].0;
                    }
                }
            }

            ui.memory_mut(|m| m.data.insert_persisted(focused_thumb_id, focused_thumb_idx));
            response.request_focus();
        }
    }

    if response.drag_started() && !props.disabled {
        let pointer_pos = ui.input(|i| i.pointer.hover_pos());
        if let Some(pos) = pointer_pos {
            let local_pos = if is_vertical {
                track_length - (pos.y - track_rect.top())
            } else {
                pos.x - track_rect.left()
            };
            let mut closest_idx = 0;
            let mut closest_dist = f32::MAX;

            for (idx, value) in props.values.iter().enumerate() {
                let thumb_pos = value_to_position(*value, props.min, props.max, track_length);
                let dist = (local_pos - thumb_pos).abs();
                if dist < closest_dist {
                    closest_dist = dist;
                    closest_idx = idx;
                }
            }

            if closest_dist < thumb_hit_size * 0.5 {
                dragged_thumb_idx = Some(closest_idx);
                ui.memory_mut(|m| m.data.insert_persisted(focused_thumb_id, closest_idx));
                ui.memory_mut(|m| m.data.insert_persisted(dragged_thumb_id, dragged_thumb_idx));
            }
        }
    }

    let pointer_released = ui.input(|i| i.pointer.any_released());
    if pointer_released {
        dragged_thumb_idx = None;
        ui.memory_mut(|m| m.data.insert_persisted(dragged_thumb_id, dragged_thumb_idx));
    }

    if let Some(thumb_idx) = dragged_thumb_idx
        && response.dragged()
        && response.is_pointer_button_down_on()
    {
        let pointer_pos = ui.input(|i| i.pointer.hover_pos());
        if let Some(pos) = pointer_pos {
            let local_pos = if is_vertical {
                (track_length - (pos.y - track_rect.top())).clamp(0.0, track_length)
            } else {
                (pos.x - track_rect.left()).clamp(0.0, track_length)
            };
            let new_value =
                position_to_value(local_pos, props.min, props.max, track_length, props.step);

            if thumb_idx < props.values.len() {
                let old_value = props.values[thumb_idx];
                let clamped_value = clamp_thumb_value(
                    props.values.as_slice(),
                    thumb_idx,
                    new_value,
                    props.min,
                    props.max,
                    min_gap,
                );
                props.values[thumb_idx] = clamped_value;

                if props.values[thumb_idx] != old_value {
                    if let Some(ref mut cb) = props.on_value_change {
                        cb(props.values.as_slice());
                    }
                    response.mark_changed();
                }
            }
        }
    }

    if response.has_focus() && !props.disabled && focused_thumb_idx < props.values.len() {
        let mut candidate = props.values[focused_thumb_idx];
        let step_size = props.step.unwrap_or((props.max - props.min) / 100.0);
        let mut value_changed = false;

        if ui.input(|i| i.key_pressed(Key::ArrowLeft) || i.key_pressed(Key::ArrowDown)) {
            let next = (candidate - step_size).max(props.min);
            if (next - candidate).abs() > f32::EPSILON {
                candidate = next;
                value_changed = true;
            }
        }

        if ui.input(|i| i.key_pressed(Key::ArrowRight) || i.key_pressed(Key::ArrowUp)) {
            let next = (candidate + step_size).min(props.max);
            if (next - candidate).abs() > f32::EPSILON {
                candidate = next;
                value_changed = true;
            }
        }

        if value_changed {
            if let Some(step) = props.step {
                candidate = snap_to_step(candidate, props.min, props.max, Some(step));
            }
            candidate = clamp_thumb_value(
                props.values.as_slice(),
                focused_thumb_idx,
                candidate,
                props.min,
                props.max,
                min_gap,
            );

            if (props.values[focused_thumb_idx] - candidate).abs() > f32::EPSILON {
                props.values[focused_thumb_idx] = candidate;
                if let Some(ref mut cb) = props.on_value_change {
                    cb(props.values.as_slice());
                }
                response.mark_changed();
            }
        }
    }

    for (idx, thumb_hit_rect, thumb_center) in thumb_responses {
        let value = props.values[idx];
        let thumb_rect = Rect::from_center_size(thumb_center, Vec2::splat(thumb_size));
        let thumb_rounding = props.radius.corner_radius(thumb_size);

        let thumb_bg = if props.disabled {
            tokens.thumb_disabled_bg
        } else {
            tokens.thumb_bg
        };

        let thumb_border = if props.disabled {
            tokens.thumb_disabled_border
        } else {
            tokens.thumb_border
        };

        let is_focused = response.has_focus() && !props.disabled && focused_thumb_idx == idx;
        let thumb_response = ui.interact(thumb_hit_rect, id.with(("thumb", idx)), Sense::hover());
        let show_tooltip =
            props.show_value_tooltip && !props.disabled && (thumb_response.hovered() || is_focused);

        let thumb_painter = ui.painter();
        thumb_painter.rect_filled(thumb_rect, thumb_rounding, thumb_bg);
        if thumb_border != Stroke::NONE {
            thumb_painter.rect_stroke(
                thumb_rect,
                thumb_rounding,
                thumb_border,
                StrokeKind::Outside,
            );
        }

        let show_ring = !props.disabled && (thumb_response.hovered() || is_focused);
        let ring_t = if props.animate {
            ui.ctx().animate_bool_with_time_and_easing(
                id.with(("thumb-ring", idx)),
                show_ring,
                anim_duration,
                ease_out_cubic,
            )
        } else if show_ring {
            1.0
        } else {
            0.0
        };

        if ring_t > 0.0 {
            thumb_painter.rect_stroke(
                thumb_rect,
                thumb_rounding,
                Stroke::new(4.0 * ring_t, tokens.ring_color),
                StrokeKind::Outside,
            );
        }

        if show_tooltip {
            let value_text = format!("{:.0}", value);
            let _ = tooltip(
                &thumb_response,
                ui,
                theme,
                TooltipProps::new(value_text)
                    .side(if is_vertical {
                        TooltipSide::Left
                    } else {
                        TooltipSide::Top
                    })
                    .delay_ms(0)
                    .skip_delay_ms(0),
            );
        }
    }

    if !props.disabled {
        response = response.on_hover_cursor(CursorIcon::Grab);
        if response.is_pointer_button_down_on() {
            response = response.on_hover_cursor(CursorIcon::Grabbing);
        }
    } else {
        response = response.on_hover_cursor(CursorIcon::NotAllowed);
    }

    response
}

pub fn slider<Id>(
    ui: &mut Ui,
    theme: &Theme,
    id_source: Id,
    values: &mut Vec<f32>,
    min: f32,
    max: f32,
) -> Response
where
    Id: Hash + Debug,
{
    slider_with_props(
        ui,
        theme,
        SliderProps::new(id_source, values).min(min).max(max),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clamp_thumb_value_without_gap_returns_candidate() {
        let values = vec![10.0];
        let clamped = clamp_thumb_value(&values, 0, 15.0, 0.0, 30.0, 0.0);
        assert_eq!(clamped, 15.0);
    }

    #[test]
    fn clamp_thumb_value_with_gap_respects_neighbor() {
        let values = vec![10.0, 25.0];
        let clamped = clamp_thumb_value(&values, 0, 22.0, 0.0, 40.0, 5.0);
        assert_eq!(clamped, 20.0);
    }

    #[test]
    fn min_steps_gap_scales_with_step() {
        let gap = min_steps_gap(Some(2.5), Some(3));
        assert!((gap - 7.5).abs() < f32::EPSILON);
    }

    #[test]
    fn slider_size_metrics_match_reference() {
        assert_eq!(SliderSize::Size2.track_size(), 6.0);
        assert_eq!(SliderSize::Size2.thumb_size(), 16.0);
        assert_eq!(SliderSize::Size2.thumb_hit_size(), 48.0);
    }

    #[test]
    fn slider_tokens_use_ring_color() {
        let palette = ColorPalette::default();
        let tokens = slider_tokens(
            &palette,
            SliderTokenOptions {
                variant: SliderVariant::Surface,
                high_contrast: false,
                accent: palette.primary,
            },
        );
        let expected = Color32::from_rgba_unmultiplied(
            palette.ring.r(),
            palette.ring.g(),
            palette.ring.b(),
            128,
        );
        assert_eq!(tokens.ring_color, expected);
    }

    #[test]
    fn thumb_value_bounds_out_of_bounds_returns_limits() {
        let bounds = thumb_value_bounds(&[10.0], 1, 0.0, 20.0, 5.0);
        assert_eq!(bounds, (0.0, 20.0));
    }
}