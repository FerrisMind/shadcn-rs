//! Semantic color resolution for the slider component.
//!
//! Follows `.cn-slider-track` / `.cn-slider-range` / `.cn-slider-thumb`: the
//! track uses `muted` or a dimmed `input`, the range uses `primary`, and the
//! thumb keeps the pack's light fill with a `primary` / `ring` / hairline
//! border. `data-disabled:opacity-50` dims everything.

use crate::iced_compat::Color;
use shadcn_common::{SliderThumbBorder, SliderThumbFill, SliderTrackSurface};
use twill_core::prelude::theme::SemanticColor;

use super::Slider;
use super::geometry::Metrics;
use super::types::{SliderStatus, SliderStyle};

/// Alpha applied to a disabled slider (`data-disabled:opacity-50`).
const DISABLED_OPACITY: f32 = 0.5;
/// Alpha of the `ring-1 ring-black/10` hairline some packs put on the thumb.
const SUBTLE_BORDER_ALPHA: f32 = 0.1;

pub(super) fn resolve_style<Message>(
    slider: &Slider<'_, Message>,
    metrics: Metrics,
    status: SliderStatus,
    track_radius: f32,
    thumb_radius: f32,
) -> SliderStyle {
    let theme = slider.theme;
    let recipe = theme.style.slider();

    let range = slider.range_color.unwrap_or_else(|| match slider.color {
        Some(accent) => theme.color_with_accent(accent, SemanticColor::Primary),
        None => theme.palette.primary,
    });

    let track = slider.track_color.unwrap_or_else(|| {
        let surface = match recipe.track_surface {
            SliderTrackSurface::Input => theme.semantic_color(SemanticColor::Input),
            // `bg-muted` covers the remaining (and any future) surfaces.
            _ => theme.semantic_color(SemanticColor::Muted),
        };

        with_alpha(surface, recipe.track_opacity)
    });

    let thumb = slider.thumb_color.unwrap_or(match recipe.thumb_fill {
        SliderThumbFill::Primary => range,
        // The packs use a literal `bg-white` thumb in both color modes.
        _ => Color::WHITE,
    });

    let thumb_border = match recipe.thumb_border {
        SliderThumbBorder::None => Color::TRANSPARENT,
        SliderThumbBorder::Primary => range,
        SliderThumbBorder::Ring => theme.semantic_color(SemanticColor::Ring),
        // `ring-1 ring-black/10`.
        _ => Color::from_rgba(0.0, 0.0, 0.0, SUBTLE_BORDER_ALPHA),
    };

    let opacity = if status.disabled {
        DISABLED_OPACITY
    } else {
        1.0
    };

    SliderStyle {
        track: with_alpha(track, opacity),
        range: with_alpha(range, opacity),
        track_radius,
        thumb: with_alpha(thumb, opacity),
        thumb_border: with_alpha(thumb_border, opacity),
        thumb_border_width: recipe.thumb_border_px,
        thumb_radius,
        ring: with_alpha(
            theme.semantic_color(SemanticColor::Ring),
            recipe.ring_opacity * opacity,
        ),
        ring_width: metrics.ring_width,
    }
}

fn with_alpha(color: Color, alpha: f32) -> Color {
    Color {
        a: color.a * alpha.clamp(0.0, 1.0),
        ..color
    }
}
