//! Style resolution for checkbox using theme tokens.

use crate::iced_compat::widget::checkbox;
use crate::iced_compat::{Background, Border, Color};

use super::geometry;
use super::types::{CheckboxSize, CheckboxVariant};
use crate::theme::Theme;

/// Resolves a native iced checkbox style from shadcn semantic tokens.
pub fn resolve_style(
    theme: &Theme,
    variant: CheckboxVariant,
    size: CheckboxSize,
    status: checkbox::Status,
) -> checkbox::Style {
    let palette = theme.palette;
    let (is_checked, hovered, disabled) = match status {
        checkbox::Status::Active { is_checked } => (is_checked, false, false),
        checkbox::Status::Hovered { is_checked } => (is_checked, true, false),
        checkbox::Status::Disabled { is_checked } => (is_checked, false, true),
    };

    let unchecked_background = match variant {
        CheckboxVariant::Surface => palette.background,
        CheckboxVariant::Classic => palette.card,
        CheckboxVariant::Soft => palette.muted,
    };
    let unchecked_border = match variant {
        CheckboxVariant::Surface => palette.input,
        CheckboxVariant::Classic => palette.border,
        CheckboxVariant::Soft => with_alpha(palette.muted_foreground, 0.35),
    };

    let (background, border, icon_color) = if is_checked {
        (palette.primary, palette.primary, palette.primary_foreground)
    } else if hovered {
        (
            with_alpha(palette.accent, 0.45),
            palette.ring,
            palette.foreground,
        )
    } else {
        (unchecked_background, unchecked_border, palette.foreground)
    };

    let opacity = if disabled { 0.5 } else { 1.0 };
    checkbox::Style {
        background: Background::Color(with_alpha(background, opacity)),
        icon_color: with_alpha(icon_color, opacity),
        border: Border {
            color: with_alpha(border, opacity),
            width: 1.0,
            radius: geometry::track_radius(theme, size).into(),
        },
        text_color: Some(with_alpha(
            if disabled {
                palette.muted_foreground
            } else {
                palette.foreground
            },
            opacity,
        )),
    }
}

fn with_alpha(color: Color, alpha: f32) -> Color {
    Color::from_rgba(color.r, color.g, color.b, color.a * alpha)
}
