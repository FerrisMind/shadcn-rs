//! Empty-state spacing, media, radius, and typography geometry.
//!
//! Values mirror the `cn-empty*` rules in the eight shadcn-svelte style
//! packs. Keeping them in one resolver lets the builder stay style-pack
//! aware without baking Nova's measurements into the rendering code.

use shadcn_common::{FontWeight, StyleId};

use crate::theme::Theme;

use super::types::EmptyRadius;

/// Resolved layout metrics for one style pack.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct Metrics {
    /// Gap between root children.
    pub(super) root_gap_px: f32,
    /// Equal root inset on all sides.
    pub(super) root_padding_px: f32,
    /// Root corner radius.
    pub(super) root_radius_px: f32,
    /// Gap between header children.
    pub(super) header_gap_px: f32,
    /// Maximum width of header and content sections.
    pub(super) section_max_width_px: f32,
    /// Bottom margin below media.
    pub(super) media_margin_bottom_px: f32,
    /// Default icon media edge.
    pub(super) media_size_px: f32,
    /// Icon media corner radius.
    pub(super) media_radius_px: f32,
    /// Gap between content children.
    pub(super) content_gap_px: f32,
}

/// Text measurements for a typed title or description.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct TextMetrics {
    /// Font size in pixels.
    pub(super) size_px: f32,
    /// Absolute line height in pixels.
    pub(super) line_height_px: f32,
    /// Default font weight.
    pub(super) weight: FontWeight,
    /// Whether source CSS transforms the text to uppercase.
    pub(super) uppercase: bool,
    /// Top inset applied before the text (Sera description only).
    pub(super) top_padding_px: f32,
}

/// Returns all non-typographic metrics for the active style pack.
pub(super) fn metrics(theme: &Theme) -> Metrics {
    let scale = theme.radius_scale();
    let (root_padding_px, root_radius_px, media_size_px, media_radius_px, content_gap_px) =
        match theme.style_id() {
            StyleId::Vega => (48.0, scale.lg_px, 40.0, scale.lg_px, 16.0),
            StyleId::Nova => (24.0, scale.xl_px, 32.0, scale.lg_px, 10.0),
            StyleId::Maia => (48.0, scale.lg_px, 40.0, scale.lg_px, 16.0),
            StyleId::Lyra => (24.0, 0.0, 32.0, 0.0, 10.0),
            StyleId::Mira => (24.0, scale.xl_px, 32.0, scale.md_px, 8.0),
            StyleId::Luma => (48.0, scale.xxl_px, 40.0, scale.xl_px, 16.0),
            StyleId::Sera => (48.0, 0.0, 40.0, 0.0, 16.0),
            StyleId::Rhea => (48.0, scale.xxl_px + 4.0, 40.0, scale.xl_px, 16.0),
        };

    Metrics {
        root_gap_px: 16.0,
        root_padding_px,
        root_radius_px,
        header_gap_px: if theme.style_id() == StyleId::Mira {
            4.0
        } else {
            8.0
        },
        section_max_width_px: 384.0,
        media_margin_bottom_px: 8.0,
        media_size_px,
        media_radius_px,
        content_gap_px,
    }
}

/// Resolves the explicit radius choices against the active theme.
pub(super) fn radius_px(theme: &Theme, radius: EmptyRadius) -> f32 {
    let scale = theme.radius_scale();

    let resolved = match radius {
        EmptyRadius::Theme => metrics(theme).root_radius_px,
        EmptyRadius::None => 0.0,
        EmptyRadius::Small => scale.sm_px,
        EmptyRadius::Medium => scale.md_px,
        EmptyRadius::Large => scale.lg_px,
        EmptyRadius::Xl => scale.xl_px,
        EmptyRadius::Xxl => scale.xxl_px,
        EmptyRadius::Full => 9999.0,
        EmptyRadius::Custom(value) if value.is_finite() => value.max(0.0),
        EmptyRadius::Custom(_) => 0.0,
    };

    resolved.max(0.0)
}

/// Resolves an explicit media radius. `Theme` uses the media tile radius,
/// which differs from the root radius in several style packs.
pub(super) fn media_radius_px(theme: &Theme, radius: EmptyRadius) -> f32 {
    if radius == EmptyRadius::Theme {
        return metrics(theme).media_radius_px;
    }

    radius_px(theme, radius)
}

/// Resolves typed title typography for the active style pack.
pub(super) fn title_metrics(theme: &Theme) -> TextMetrics {
    match theme.style_id() {
        StyleId::Vega | StyleId::Maia | StyleId::Luma | StyleId::Rhea => TextMetrics {
            size_px: 18.0,
            line_height_px: 28.0,
            weight: FontWeight::Medium,
            uppercase: false,
            top_padding_px: 0.0,
        },
        StyleId::Nova | StyleId::Lyra | StyleId::Mira => TextMetrics {
            size_px: 14.0,
            line_height_px: 20.0,
            weight: FontWeight::Medium,
            uppercase: false,
            top_padding_px: 0.0,
        },
        StyleId::Sera => TextMetrics {
            size_px: 18.0,
            line_height_px: 28.0,
            weight: FontWeight::Semibold,
            uppercase: true,
            top_padding_px: 0.0,
        },
    }
}

/// Resolves typed description typography for the active style pack.
pub(super) fn description_metrics(theme: &Theme) -> TextMetrics {
    match theme.style_id() {
        StyleId::Lyra | StyleId::Mira => TextMetrics {
            size_px: 12.0,
            line_height_px: 19.5,
            weight: FontWeight::Normal,
            uppercase: false,
            top_padding_px: 0.0,
        },
        StyleId::Sera => TextMetrics {
            size_px: 14.0,
            line_height_px: 22.75,
            weight: FontWeight::Normal,
            uppercase: false,
            top_padding_px: 2.0,
        },
        StyleId::Vega | StyleId::Nova | StyleId::Maia | StyleId::Luma | StyleId::Rhea => {
            TextMetrics {
                size_px: 14.0,
                line_height_px: 22.75,
                weight: FontWeight::Normal,
                uppercase: false,
                top_padding_px: 0.0,
            }
        }
    }
}
