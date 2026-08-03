//! Semantic color resolution for the meter component.

use crate::iced_compat::Color;
use shadcn_common::MeterFillTone;
use twill_core::prelude::theme::SemanticColor;

use super::types::Meter;
use crate::theme::Theme;

#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct Visual {
    pub(super) track: Color,
    pub(super) indicator: Color,
}

pub(super) fn resolve_visual(meter: &Meter<'_>, theme: &Theme) -> Visual {
    let mut indicator = meter
        .custom_indicator_color
        .unwrap_or_else(|| resolve_indicator(meter, theme));

    if meter.high_contrast {
        indicator = with_alpha(indicator, 1.0);
    }

    if !indicator.a.is_finite() {
        indicator = theme.palette.primary;
    }

    let track = meter
        .track_color
        .unwrap_or_else(|| with_alpha(indicator, theme.style.meter().track_alpha));

    Visual { track, indicator }
}

fn resolve_indicator(meter: &Meter<'_>, theme: &Theme) -> Color {
    // Threshold tones from the extras Tokens demo win over the default
    // `--meter-background` (primary / accent) once the measurement crosses
    // the warning or danger bands.
    match meter.resolved_tone() {
        MeterFillTone::Warning => {
            return theme
                .color_with_accent(shadcn_common::AccentColor::Orange, SemanticColor::Primary);
        }
        MeterFillTone::Danger => return theme.palette.destructive,
        MeterFillTone::Default | _ => {}
    }

    if let Some(color) = meter.color {
        return theme.color_with_accent(color, SemanticColor::Primary);
    }

    theme.palette.primary
}

fn with_alpha(mut color: Color, alpha: f32) -> Color {
    color.a = alpha.clamp(0.0, 1.0);
    color
}
