//! Progress component - determinate and indeterminate progress bars.
//!
//! # Example
//! ```ignore
//! progress(ui, &theme, ProgressProps::new(Some(75.0)));
//! ```

use crate::theme::Theme;
use egui::{Color32, Ui, Vec2};

// =============================================================================
// ProgressSize / ProgressVariant
// =============================================================================

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ProgressSize {
    Size1,
    #[default]
    Size2,
    Size3,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ProgressVariant {
    Classic,
    #[default]
    Surface,
    Soft,
}

// =============================================================================
// ProgressProps
// =============================================================================

#[derive(Clone, Debug)]
pub struct ProgressProps {
    pub value: Option<f32>, // None = indeterminate
    pub max: f32,
    pub size: ProgressSize,
    pub variant: ProgressVariant,
    pub color: Option<Color32>,
    pub high_contrast: bool,
}

impl ProgressProps {
    pub fn new(value: Option<f32>) -> Self {
        Self {
            value,
            max: 100.0,
            size: ProgressSize::Size2,
            variant: ProgressVariant::Surface,
            color: None,
            high_contrast: false,
        }
    }

    pub fn max(mut self, max: f32) -> Self {
        self.max = max.max(0.01);
        self
    }

    pub fn size(mut self, size: ProgressSize) -> Self {
        self.size = size;
        self
    }

    pub fn variant(mut self, variant: ProgressVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn color(mut self, color: Color32) -> Self {
        self.color = Some(color);
        self
    }

    pub fn high_contrast(mut self, hc: bool) -> Self {
        self.high_contrast = hc;
        self
    }
}

// =============================================================================
// Main function
// =============================================================================

/// Render a progress bar.
pub fn progress(ui: &mut Ui, theme: &Theme, props: ProgressProps) {
    let accent = props.color.unwrap_or(theme.palette.primary);

    let (bg_color, fg_color) = match props.variant {
        ProgressVariant::Classic => (
            theme.palette.muted, 
            accent,
        ),
        ProgressVariant::Surface => (
            theme.palette.muted.gamma_multiply(0.5), 
            accent,
        ),
        ProgressVariant::Soft => (
            accent.gamma_multiply(0.2), 
            accent,
        ),
    };

    let height = match props.size {
        ProgressSize::Size1 => 4.0,
        ProgressSize::Size2 => 8.0,
        ProgressSize::Size3 => 12.0,
    };

    let available_width = ui.available_width();
    let rounding = height / 2.0;

    let (rect, _response) = ui.allocate_exact_size(Vec2::new(available_width, height), egui::Sense::hover());

    // Background
    ui.painter().rect_filled(rect, rounding, bg_color);

    // Foreground (progress)
    if let Some(value) = props.value {
        let progress = (value / props.max).clamp(0.0, 1.0);
        let progress_width = rect.width() * progress;

        if progress_width > 0.0 {
            let progress_rect = egui::Rect::from_min_size(
                rect.min,
                Vec2::new(progress_width, height),
            );
            ui.painter().rect_filled(progress_rect, rounding, fg_color);
        }
    } else {
        // Indeterminate animation
        let time = ui.ctx().input(|i| i.time);
        let anim_progress = ((time * 1.5).sin() as f32 + 1.0) / 2.0;
        let bar_width = rect.width() * 0.3;
        let offset = (rect.width() - bar_width) * anim_progress;

        let anim_rect = egui::Rect::from_min_size(
            rect.min + Vec2::new(offset, 0.0),
            Vec2::new(bar_width, height),
        );
        ui.painter().rect_filled(anim_rect, rounding, fg_color);
        ui.ctx().request_repaint();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn progress_size_default() {
        assert_eq!(ProgressSize::default(), ProgressSize::Size2);
    }

    #[test]
    fn progress_variant_default() {
        assert_eq!(ProgressVariant::default(), ProgressVariant::Surface);
    }

    #[test]
    fn progress_props_determinate() {
        let props = ProgressProps::new(Some(50.0));
        assert_eq!(props.value, Some(50.0));
        assert_eq!(props.max, 100.0);
    }

    #[test]
    fn progress_props_indeterminate() {
        let props = ProgressProps::new(None);
        assert!(props.value.is_none());
    }
}
