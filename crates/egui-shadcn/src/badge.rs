//! Badge component - status labels and tags.
//!
//! # Example
//! ```ignore
//! badge(ui, &theme, BadgeProps::new("New").variant(BadgeVariant::Solid));
//! ```

use crate::theme::Theme;
use egui::{Color32, RichText, Ui, Vec2};

// =============================================================================
// BadgeSize / BadgeVariant
// =============================================================================

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BadgeSize {
    #[default]
    Size1,
    Size2,
    Size3,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BadgeVariant {
    Solid,
    #[default]
    Soft,
    Surface,
    Outline,
}

// =============================================================================
// BadgeProps
// =============================================================================

#[derive(Clone, Debug)]
pub struct BadgeProps<'a> {
    pub label: &'a str,
    pub size: BadgeSize,
    pub variant: BadgeVariant,
    pub color: Option<Color32>,
    pub high_contrast: bool,
}

impl<'a> BadgeProps<'a> {
    pub fn new(label: &'a str) -> Self {
        Self {
            label,
            size: BadgeSize::Size1,
            variant: BadgeVariant::Soft,
            color: None,
            high_contrast: false,
        }
    }

    pub fn size(mut self, size: BadgeSize) -> Self {
        self.size = size;
        self
    }

    pub fn variant(mut self, variant: BadgeVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn color(mut self, color: Color32) -> Self {
        self.color = Some(color);
        self
    }

    pub fn high_contrast(mut self, high_contrast: bool) -> Self {
        self.high_contrast = high_contrast;
        self
    }
}

// =============================================================================
// Main function
// =============================================================================

/// Render a badge.
pub fn badge(ui: &mut Ui, theme: &Theme, props: BadgeProps<'_>) {
    let accent = props.color.unwrap_or(theme.palette.primary);

    let (bg_color, text_color, border_color) = match props.variant {
        BadgeVariant::Solid => (
            accent,
            Color32::WHITE,
            accent,
        ),
        BadgeVariant::Soft => (
            accent.gamma_multiply(0.15),
            accent,
            Color32::TRANSPARENT,
        ),
        BadgeVariant::Surface => (
            theme.palette.muted.gamma_multiply(0.5),
            accent,
            theme.palette.border,
        ),
        BadgeVariant::Outline => (
            Color32::TRANSPARENT,
            accent,
            accent,
        ),
    };

    let (font_size, padding) = match props.size {
        BadgeSize::Size1 => (11.0, Vec2::new(6.0, 2.0)),
        BadgeSize::Size2 => (12.0, Vec2::new(8.0, 3.0)),
        BadgeSize::Size3 => (13.0, Vec2::new(10.0, 4.0)),
    };

    let rounding = theme.radius.r6; // Use r6 as pill shape
    
    egui::Frame::NONE
        .fill(bg_color)
        .stroke(egui::Stroke::new(1.0, border_color))
        .corner_radius(rounding)
        .inner_margin(padding)
        .show(ui, |ui| {
            ui.label(
                RichText::new(props.label)
                    .size(font_size)
                    .color(text_color)
                    .strong(),
            );
        });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn badge_size_default() {
        assert_eq!(BadgeSize::default(), BadgeSize::Size1);
    }

    #[test]
    fn badge_variant_default() {
        assert_eq!(BadgeVariant::default(), BadgeVariant::Soft);
    }

    #[test]
    fn badge_props_builder() {
        let props = BadgeProps::new("Test")
            .size(BadgeSize::Size2)
            .variant(BadgeVariant::Solid)
            .high_contrast(true);

        assert_eq!(props.size, BadgeSize::Size2);
        assert_eq!(props.variant, BadgeVariant::Solid);
        assert!(props.high_contrast);
    }
}
