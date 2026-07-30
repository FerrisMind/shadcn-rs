//! Avatar component - user avatar with fallback.
//!
//! # Example
//! ```ignore
//! avatar(ui, &theme, AvatarProps::new("JD"));
//! ```

use crate::theme::Theme;
use egui::{Color32, CornerRadius, ImageSource, Stroke, Ui, Vec2};

// =============================================================================
// AvatarSize / AvatarVariant
// =============================================================================

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AvatarSize {
    Size1,
    Size2,
    #[default]
    Size3,
    Size4,
    Size5,
    Size6,
    Size7,
    Size8,
    Size9,
}

impl AvatarSize {
    fn to_pixels(self) -> f32 {
        match self {
            AvatarSize::Size1 => 16.0,
            AvatarSize::Size2 => 20.0,
            AvatarSize::Size3 => 24.0,
            AvatarSize::Size4 => 32.0,
            AvatarSize::Size5 => 40.0,
            AvatarSize::Size6 => 48.0,
            AvatarSize::Size7 => 64.0,
            AvatarSize::Size8 => 80.0,
            AvatarSize::Size9 => 96.0,
        }
    }

    fn font_size(self) -> f32 {
        self.to_pixels() * 0.4
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AvatarVariant {
    Solid,
    #[default]
    Soft,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AvatarShape {
    #[default]
    Circle,
    Rounded,
}

impl AvatarShape {
    fn corner_radius(self, size: f32) -> CornerRadius {
        match self {
            AvatarShape::Circle => {
                CornerRadius::same((size * 0.5).round().clamp(0.0, u8::MAX as f32) as u8)
            }
            AvatarShape::Rounded => CornerRadius::same(8),
        }
    }
}

// =============================================================================
// AvatarProps
// =============================================================================

#[derive(Clone, Debug)]
pub struct AvatarProps<'a> {
    pub fallback: &'a str,
    pub size: AvatarSize,
    pub variant: AvatarVariant,
    pub color: Option<Color32>,
    pub shape: AvatarShape,
    pub image: Option<ImageSource<'a>>,
}

impl<'a> AvatarProps<'a> {
    pub fn new(fallback: &'a str) -> Self {
        Self {
            fallback,
            size: AvatarSize::Size3,
            variant: AvatarVariant::Soft,
            color: None,
            shape: AvatarShape::Circle,
            image: None,
        }
    }

    pub fn size(mut self, size: AvatarSize) -> Self {
        self.size = size;
        self
    }

    pub fn variant(mut self, variant: AvatarVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn color(mut self, color: Color32) -> Self {
        self.color = Some(color);
        self
    }

    pub fn shape(mut self, shape: AvatarShape) -> Self {
        self.shape = shape;
        self
    }

    pub fn image(mut self, image: ImageSource<'a>) -> Self {
        self.image = Some(image);
        self
    }
}

// =============================================================================
// Main function
// =============================================================================

/// Render an avatar with fallback text (typically initials).
pub fn avatar(ui: &mut Ui, theme: &Theme, props: AvatarProps<'_>) {
    let size = props.size.to_pixels();
    let font_size = props.size.font_size();
    let accent = props.color.unwrap_or(theme.palette.primary);
    let rounding = props.shape.corner_radius(size);

    let (bg_color, text_color) = match props.variant {
        AvatarVariant::Solid => (accent, Color32::WHITE),
        AvatarVariant::Soft => (accent.gamma_multiply(0.2), accent),
    };

    if let Some(image) = props.image {
        egui::Frame::new()
            .fill(theme.palette.muted)
            .stroke(Stroke::new(1.0_f32, theme.palette.border))
            .corner_radius(rounding)
            .show(ui, |frame_ui| {
                let _ = frame_ui.add_sized(
                    Vec2::splat(size),
                    egui::Image::new(image)
                        .fit_to_exact_size(Vec2::splat(size))
                        .corner_radius(rounding),
                );
            });
        return;
    }

    let (rect, _response) = ui.allocate_exact_size(Vec2::splat(size), egui::Sense::hover());

    ui.painter().rect_filled(rect, rounding, bg_color);

    // Fallback text (centered)
    let text = props
        .fallback
        .chars()
        .take(2)
        .collect::<String>()
        .to_uppercase();
    let galley =
        ui.painter()
            .layout_no_wrap(text, egui::FontId::proportional(font_size), text_color);
    let center = rect.center();
    let text_pos = center - galley.size() / 2.0;
    ui.painter().galley(text_pos, galley, text_color);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn avatar_size_default() {
        assert_eq!(AvatarSize::default(), AvatarSize::Size3);
    }

    #[test]
    fn avatar_size_pixels() {
        assert_eq!(AvatarSize::Size1.to_pixels(), 16.0);
        assert_eq!(AvatarSize::Size9.to_pixels(), 96.0);
    }

    #[test]
    fn avatar_props_builder() {
        let props = AvatarProps::new("JD")
            .size(AvatarSize::Size5)
            .variant(AvatarVariant::Solid)
            .shape(AvatarShape::Rounded);

        assert_eq!(props.fallback, "JD");
        assert_eq!(props.size, AvatarSize::Size5);
        assert_eq!(props.variant, AvatarVariant::Solid);
        assert_eq!(props.shape, AvatarShape::Rounded);
    }
}
