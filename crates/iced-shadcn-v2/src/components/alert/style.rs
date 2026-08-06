//! Semantic style resolution for the alert component.

use crate::iced_compat::border::Border;
use crate::iced_compat::widget::container;
use crate::iced_compat::{Background, Color};
use shadcn_common::StyleId;
use twill_core::prelude::theme::SemanticColor;

use super::geometry;
use super::types::{AlertRadius, AlertVariant};
use crate::theme::Theme;

/// Resolves the root container style for one alert variant.
pub(super) fn resolve_root_style(
    theme: &Theme,
    variant: AlertVariant,
    radius: AlertRadius,
) -> container::Style {
    let text_color = foreground_color(theme, variant);

    container::Style {
        background: Some(Background::Color(theme.palette.card)),
        text_color: Some(text_color),
        border: Border {
            color: theme.palette.border,
            width: 1.0,
            radius: geometry::radius_px(theme, radius).into(),
        },
        snap: true,
        ..container::Style::default()
    }
}

/// Foreground used by the root and by default title/icon content.
pub(super) fn foreground_color(theme: &Theme, variant: AlertVariant) -> Color {
    match variant {
        AlertVariant::Default => theme.palette.card_foreground,
        AlertVariant::Destructive => theme.semantic_color(SemanticColor::Destructive),
    }
}

/// Description color, including shadcn's destructive `/90` treatment.
pub(super) fn description_color(theme: &Theme, variant: AlertVariant) -> Color {
    match variant {
        AlertVariant::Default => theme.palette.muted_foreground,
        AlertVariant::Destructive => {
            with_alpha(theme.semantic_color(SemanticColor::Destructive), 0.90)
        }
    }
}

/// Sera adds a two-pixel semantic rail on the root's leading edge.
pub(super) fn accent_bar_color(theme: &Theme, variant: AlertVariant) -> Option<Color> {
    if theme.style_id() != StyleId::Sera {
        return None;
    }

    Some(match variant {
        AlertVariant::Default => theme.palette.foreground,
        AlertVariant::Destructive => theme.semantic_color(SemanticColor::Destructive),
    })
}

fn with_alpha(mut color: Color, alpha: f32) -> Color {
    color.a *= alpha.clamp(0.0, 1.0);
    color
}
