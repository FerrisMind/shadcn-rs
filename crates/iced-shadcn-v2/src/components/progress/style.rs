//! Semantic color resolution for the progress component.

use crate::iced_compat::Color;
use twill_core::prelude::theme::SemanticColor;

use super::types::{Progress, ProgressVariant};
use crate::theme::Theme;

#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct Visual {
    pub(super) track: Color,
    pub(super) indicator: Color,
}

pub(super) fn resolve_visual(progress: &Progress<'_>, theme: &Theme) -> Visual {
    let indicator = progress
        .custom_indicator_color
        .unwrap_or_else(|| match progress.color {
            Some(color) => theme.color_with_accent(color, SemanticColor::Primary),
            None => theme.palette.primary,
        });

    let mut indicator = if progress.high_contrast {
        with_alpha(indicator, 1.0)
    } else {
        indicator
    };

    if !indicator.a.is_finite() {
        indicator = theme.palette.primary;
    }

    let track = progress
        .track_color
        .unwrap_or_else(|| match progress.variant {
            ProgressVariant::Default | ProgressVariant::Classic | ProgressVariant::Surface => {
                theme.semantic_color(SemanticColor::Muted)
            }
            ProgressVariant::Soft => mix_color(
                theme.semantic_color(SemanticColor::Background),
                indicator,
                if theme.is_dark() { 0.20 } else { 0.12 },
            ),
        });

    Visual { track, indicator }
}

fn mix_color(a: Color, b: Color, amount: f32) -> Color {
    let amount = amount.clamp(0.0, 1.0);
    Color {
        r: a.r + (b.r - a.r) * amount,
        g: a.g + (b.g - a.g) * amount,
        b: a.b + (b.b - a.b) * amount,
        a: a.a + (b.a - a.a) * amount,
    }
}

fn with_alpha(mut color: Color, alpha: f32) -> Color {
    color.a = alpha.clamp(0.0, 1.0);
    color
}
