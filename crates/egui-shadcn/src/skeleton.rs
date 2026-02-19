//! Skeleton component - placeholder for loading content.
//!
//! # Example
//! ```ignore
//! skeleton(ui, &theme, SkeletonProps::new().width(200.0).height(20.0));
//! ```

use crate::theme::Theme;
use egui::{Ui, Vec2};

// =============================================================================
// SkeletonProps
// =============================================================================

#[derive(Clone, Debug)]
pub struct SkeletonProps {
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub circle: bool,
    pub duration_ms: u32,
}

impl Default for SkeletonProps {
    fn default() -> Self {
        Self {
            width: None,
            height: None,
            circle: false,
            duration_ms: 2000,
        }
    }
}

impl SkeletonProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.height = Some(height);
        self
    }

    pub fn circle(mut self, circle: bool) -> Self {
        self.circle = circle;
        self
    }

    pub fn duration_ms(mut self, duration_ms: u32) -> Self {
        self.duration_ms = duration_ms;
        self
    }
}

// =============================================================================
// Main function
// =============================================================================

/// Render a skeleton placeholder with shimmer animation.
pub fn skeleton(ui: &mut Ui, theme: &Theme, props: SkeletonProps) {
    let width = props.width.unwrap_or(ui.available_width());
    let height = props.height.unwrap_or(20.0);
    let size = if props.circle {
        let s = width.min(height);
        Vec2::splat(s)
    } else {
        Vec2::new(width, height)
    };

    let (rect, _response) = ui.allocate_exact_size(size, egui::Sense::hover());

    // Pulse animation (Shadcn pattern: 2s duration, 0.5 to 1.0 opacity)
    let time = ui.ctx().input(|i| i.time) as f32;
    let duration_secs = props.duration_ms as f32 / 1000.0;

    // Smooth oscillation from 0.0 to 1.0
    let wave = (time * std::f32::consts::PI * 2.0 / duration_secs).cos() * 0.5 + 0.5;

    // Base color from theme
    let base_color = theme.palette.muted;

    // Apply pulsing opacity (0.5 to 1.0 range)
    let alpha_factor = 0.5 + 0.5 * wave;
    let pulsing_color = base_color.gamma_multiply(alpha_factor);

    let rounding = if props.circle {
        size.x / 2.0
    } else {
        theme.radius.r2
    };

    // Draw pulsing placeholder
    ui.painter().rect_filled(rect, rounding, pulsing_color);

    ui.ctx().request_repaint();
}

/// Helper for text skeleton with multiple lines.
pub fn skeleton_text(ui: &mut Ui, theme: &Theme, lines: usize, line_height: f32) {
    ui.vertical(|ui| {
        ui.spacing_mut().item_spacing.y = 8.0;
        for i in 0..lines {
            // Last line is shorter
            let width = if i == lines - 1 {
                ui.available_width() * 0.6
            } else {
                ui.available_width()
            };
            skeleton(
                ui,
                theme,
                SkeletonProps::new().width(width).height(line_height),
            );
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skeleton_props_default() {
        let props = SkeletonProps::new();
        assert!(props.width.is_none());
        assert!(props.height.is_none());
        assert!(!props.circle);
    }

    #[test]
    fn skeleton_props_builder() {
        let props = SkeletonProps::new().width(100.0).height(50.0).circle(true);

        assert_eq!(props.width, Some(100.0));
        assert_eq!(props.height, Some(50.0));
        assert!(props.circle);
    }
}
