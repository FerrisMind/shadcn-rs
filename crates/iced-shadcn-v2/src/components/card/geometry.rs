//! Card spacing, radius, and typography geometry.

use shadcn_common::StyleId;

use super::types::{CardRadius, CardSize};
use crate::theme::Theme;

/// Default vertical gap and section inset for the active card style.
pub(super) fn default_spacing(theme: &Theme) -> f32 {
    theme.style.card_padding_px
}

/// Compact vertical gap and section inset from the source style CSS.
pub(super) fn small_spacing(theme: &Theme) -> f32 {
    match theme.style_id() {
        StyleId::Vega => 16.0,
        StyleId::Nova | StyleId::Lyra | StyleId::Mira => 12.0,
        StyleId::Maia | StyleId::Luma => 16.0,
        StyleId::Sera => 20.0,
        StyleId::Rhea => 16.0,
    }
}

pub(super) fn resolved_spacing(theme: &Theme, size: CardSize, custom_spacing: Option<f32>) -> f32 {
    custom_spacing.unwrap_or(match size {
        CardSize::Default => default_spacing(theme),
        CardSize::Sm => small_spacing(theme),
    })
}

/// Gap between title/description rows in a card header.
pub(super) fn header_gap(theme: &Theme) -> f32 {
    match theme.style_id() {
        StyleId::Maia => 8.0,
        StyleId::Luma | StyleId::Sera | StyleId::Rhea => 6.0,
        StyleId::Vega | StyleId::Nova | StyleId::Lyra | StyleId::Mira => 4.0,
    }
}

/// Whether the style-pack footer owns all of its vertical padding.
pub(super) fn footer_uses_full_padding(theme: &Theme) -> bool {
    matches!(theme.style_id(), StyleId::Nova | StyleId::Lyra)
}

/// Nova and Lyra remove the root's trailing padding when a footer is present.
pub(super) fn suppress_bottom_padding(theme: &Theme) -> bool {
    matches!(theme.style_id(), StyleId::Nova | StyleId::Lyra)
}

/// Resolved card radius in pixels.
pub(crate) fn radius_px(theme: &Theme, radius: CardRadius) -> f32 {
    let scale = theme.style.radius;

    let resolved = match radius {
        CardRadius::Theme => match theme.style_id() {
            // rounded-xl
            StyleId::Vega | StyleId::Nova => scale.xl_px,
            // rounded-2xl
            StyleId::Maia => scale.xxl_px,
            // rounded-none
            StyleId::Lyra | StyleId::Sera => 0.0,
            // rounded-lg
            StyleId::Mira => scale.lg_px,
            // rounded-4xl → `--radius-4xl` = base + 16
            StyleId::Luma => scale.xxxxl_px,
            // min(var(--radius-4xl), 24px)
            StyleId::Rhea => scale.xxxxl_px.min(24.0),
        },
        CardRadius::None => 0.0,
        CardRadius::Small => scale.sm_px,
        CardRadius::Medium => scale.md_px,
        CardRadius::Large => scale.lg_px,
        CardRadius::Xl => scale.xl_px,
        CardRadius::Full => 9999.0,
        CardRadius::Custom(value) if value.is_finite() => value.max(0.0),
        CardRadius::Custom(_) => 0.0,
    };

    resolved.max(0.0)
}

/// Source card title metrics for one style pack and density.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct TextMetrics {
    pub(super) size_px: f32,
    pub(super) line_height_px: f32,
    pub(super) semibold: bool,
    pub(super) uppercase: bool,
    pub(super) tracking_em: f32,
}

pub(super) fn title_metrics(theme: &Theme, size: CardSize) -> TextMetrics {
    match theme.style_id() {
        StyleId::Vega => TextMetrics {
            size_px: if size == CardSize::Sm { 14.0 } else { 16.0 },
            line_height_px: if size == CardSize::Sm { 20.0 } else { 24.0 },
            semibold: false,
            uppercase: false,
            tracking_em: 0.0,
        },
        StyleId::Nova => TextMetrics {
            size_px: if size == CardSize::Sm { 14.0 } else { 16.0 },
            line_height_px: if size == CardSize::Sm { 20.0 } else { 22.0 },
            semibold: false,
            uppercase: false,
            tracking_em: 0.0,
        },
        StyleId::Maia | StyleId::Luma | StyleId::Rhea => TextMetrics {
            size_px: 16.0,
            line_height_px: 24.0,
            semibold: false,
            uppercase: false,
            tracking_em: 0.0,
        },
        StyleId::Lyra | StyleId::Mira => TextMetrics {
            size_px: 14.0,
            line_height_px: 20.0,
            semibold: false,
            uppercase: false,
            tracking_em: 0.0,
        },
        StyleId::Sera => TextMetrics {
            size_px: 18.0,
            line_height_px: 28.0,
            semibold: true,
            uppercase: true,
            tracking_em: 0.05,
        },
    }
}

pub(super) fn description_metrics(theme: &Theme) -> TextMetrics {
    match theme.style_id() {
        StyleId::Lyra | StyleId::Mira => TextMetrics {
            size_px: 12.0,
            line_height_px: 18.0,
            semibold: false,
            uppercase: false,
            tracking_em: 0.0,
        },
        StyleId::Sera => TextMetrics {
            size_px: 14.0,
            line_height_px: 22.75,
            semibold: false,
            uppercase: false,
            tracking_em: 0.0,
        },
        StyleId::Vega | StyleId::Nova | StyleId::Maia | StyleId::Luma | StyleId::Rhea => {
            TextMetrics {
                size_px: 14.0,
                line_height_px: 20.0,
                semibold: false,
                uppercase: false,
                tracking_em: 0.0,
            }
        }
    }
}
