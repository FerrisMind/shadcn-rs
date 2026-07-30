//! LightSwitch component - light/dark mode toggle.
//!
//! # Example
//! ```ignore
//! light_switch(ui, &theme, LightSwitchProps::new(dark_mode), |new_dark| { /* handle */ });
//! ```

use crate::theme::Theme;
use egui::{Color32, CornerRadius, Response, Sense, Ui};

/// Properties for the LightSwitch component.
#[derive(Clone, Copy, Debug)]
pub struct LightSwitchProps {
    pub dark_mode: bool,
    pub disabled: bool,
}

impl LightSwitchProps {
    pub fn new(dark_mode: bool) -> Self {
        Self {
            dark_mode,
            disabled: false,
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

/// Render a light/dark mode toggle switch.
///
/// Returns `Response`. Check `response.clicked()` to detect toggle.
pub fn light_switch(ui: &mut Ui, theme: &Theme, props: LightSwitchProps) -> Response {
    let track_width = 44.0_f32;
    let track_height = 24.0_f32;
    let thumb_size = 20.0_f32;
    let padding = 2.0_f32;

    let desired_size = egui::vec2(track_width, track_height);
    let sense = if props.disabled {
        Sense::hover()
    } else {
        Sense::click()
    };
    let (rect, resp) = ui.allocate_exact_size(desired_size, sense);

    let track_color = if props.dark_mode {
        theme.palette.primary
    } else {
        theme.palette.muted
    };

    let track_color = if props.disabled {
        Color32::from_rgba_unmultiplied(track_color.r(), track_color.g(), track_color.b(), 128)
    } else {
        track_color
    };

    // Draw track
    ui.painter()
        .rect_filled(rect, CornerRadius::same(12), track_color);

    // Animate thumb position
    let anim_t = ui.ctx().animate_bool(resp.id, props.dark_mode);
    let thumb_x = rect.left() + padding + anim_t * (track_width - thumb_size - padding * 2.0);
    let thumb_rect = egui::Rect::from_min_size(
        egui::pos2(thumb_x, rect.top() + padding),
        egui::vec2(thumb_size, thumb_size),
    );

    // Draw thumb
    ui.painter()
        .rect_filled(thumb_rect, CornerRadius::same(10), Color32::WHITE);

    // Draw icon
    let icon = if props.dark_mode { "🌙" } else { "☀" };
    ui.painter().text(
        thumb_rect.center(),
        egui::Align2::CENTER_CENTER,
        icon,
        egui::FontId::proportional(11.0),
        Color32::from_gray(60),
    );

    // Focus ring
    if resp.has_focus() {
        ui.painter().rect_stroke(
            rect,
            CornerRadius::same(12),
            egui::Stroke::new(2.0_f32, theme.palette.ring),
            egui::StrokeKind::Outside,
        );
    }

    resp
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn light_switch_props_builder() {
        let props = LightSwitchProps::new(true).disabled(false);
        assert!(props.dark_mode);
        assert!(!props.disabled);
    }

    #[test]
    fn light_switch_props_light_mode() {
        let props = LightSwitchProps::new(false);
        assert!(!props.dark_mode);
    }
}
