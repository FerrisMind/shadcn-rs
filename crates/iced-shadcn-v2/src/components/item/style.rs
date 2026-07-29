//! Semantic item style resolution.

use crate::iced_compat::border::Border;
use crate::iced_compat::widget::{button as button_widget, container};
use crate::iced_compat::{Background, Color};

use super::geometry;
use super::types::{ItemRadius, ItemVariant};
use crate::theme::Theme;

/// Resolves the resting `cn-item-variant-*` container style.
pub(super) fn resolve_root_style(
    theme: &Theme,
    variant: ItemVariant,
    radius: ItemRadius,
) -> container::Style {
    let (background, border_color) = match variant {
        ItemVariant::Default => (None, Color::TRANSPARENT),
        ItemVariant::Outline => (None, theme.palette.border),
        ItemVariant::Muted => (Some(muted_background(theme)), Color::TRANSPARENT),
    };

    container::Style {
        background: background.map(Background::Color),
        text_color: Some(theme.palette.foreground),
        border: Border {
            radius: geometry::radius_px(theme, radius).into(),
            width: 1.0,
            color: border_color,
        },
        snap: true,
        ..container::Style::default()
    }
}

/// Resolves the hovered/pressed style of a pressable item, mirroring the
/// source `[a]:hover:bg-muted` rule.
pub(super) fn resolve_hover_style(
    theme: &Theme,
    variant: ItemVariant,
    radius: ItemRadius,
) -> container::Style {
    container::Style {
        background: Some(Background::Color(theme.palette.muted)),
        ..resolve_root_style(theme, variant, radius)
    }
}

/// Adapts a resolved container style onto iced's button style.
pub(super) fn to_button_style(
    style: container::Style,
    fallback_text: Color,
) -> button_widget::Style {
    button_widget::Style {
        background: style.background.filter(|background| match background {
            Background::Color(color) => color.a > f32::EPSILON,
            Background::Gradient(_) => true,
        }),
        text_color: style.text_color.unwrap_or(fallback_text),
        border: style.border,
        shadow: style.shadow,
        snap: style.snap,
    }
}

/// The `bg-muted/50` surface of the muted variant.
pub(super) fn muted_background(theme: &Theme) -> Color {
    with_alpha(theme.palette.muted, 0.5)
}

fn with_alpha(mut color: Color, alpha: f32) -> Color {
    color.a *= alpha.clamp(0.0, 1.0);
    color
}
