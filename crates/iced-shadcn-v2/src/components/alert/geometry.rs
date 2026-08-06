//! Alert geometry and typography metrics.

use shadcn_common::StyleId;

use super::types::AlertRadius;
use crate::theme::Theme;

/// Resolved dimensions for one shadcn style pack.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct Metrics {
    pub(super) gap_px: f32,
    pub(super) padding_x_px: f32,
    pub(super) padding_y_px: f32,
    pub(super) icon_gap_px: f32,
    pub(super) icon_size_px: f32,
    pub(super) icon_offset_y_px: f32,
    pub(super) title_size_px: f32,
    pub(super) title_line_height_px: f32,
    pub(super) description_size_px: f32,
    pub(super) description_line_height_px: f32,
    pub(super) title_is_semibold: bool,
    pub(super) action_top_px: f32,
    pub(super) action_right_px: f32,
}

/// Resolves the dimensions encoded by the source style CSS.
pub(super) fn metrics(theme: &Theme) -> Metrics {
    match theme.style_id() {
        StyleId::Vega | StyleId::Maia | StyleId::Luma | StyleId::Rhea => Metrics {
            gap_px: 2.0,
            padding_x_px: 16.0,
            padding_y_px: 12.0,
            icon_gap_px: 10.0,
            icon_size_px: 16.0,
            icon_offset_y_px: 2.0,
            title_size_px: 14.0,
            title_line_height_px: 20.0,
            description_size_px: 14.0,
            description_line_height_px: 20.0,
            title_is_semibold: false,
            action_top_px: 10.0,
            action_right_px: 12.0,
        },
        StyleId::Nova => Metrics {
            gap_px: 2.0,
            padding_x_px: 10.0,
            padding_y_px: 8.0,
            icon_gap_px: 8.0,
            icon_size_px: 16.0,
            icon_offset_y_px: 2.0,
            title_size_px: 14.0,
            title_line_height_px: 20.0,
            description_size_px: 14.0,
            description_line_height_px: 20.0,
            title_is_semibold: false,
            action_top_px: 8.0,
            action_right_px: 8.0,
        },
        StyleId::Lyra => Metrics {
            gap_px: 2.0,
            padding_x_px: 10.0,
            padding_y_px: 8.0,
            icon_gap_px: 8.0,
            icon_size_px: 16.0,
            icon_offset_y_px: 0.0,
            title_size_px: 12.0,
            title_line_height_px: 16.0,
            description_size_px: 12.0,
            description_line_height_px: 18.0,
            title_is_semibold: false,
            action_top_px: 10.0,
            action_right_px: 10.0,
        },
        StyleId::Mira => Metrics {
            gap_px: 2.0,
            padding_x_px: 8.0,
            padding_y_px: 6.0,
            icon_gap_px: 6.0,
            icon_size_px: 14.0,
            icon_offset_y_px: 2.0,
            title_size_px: 12.0,
            title_line_height_px: 19.5,
            description_size_px: 12.0,
            description_line_height_px: 19.5,
            title_is_semibold: false,
            action_top_px: 6.0,
            action_right_px: 8.0,
        },
        StyleId::Sera => Metrics {
            gap_px: 4.0,
            padding_x_px: 16.0,
            padding_y_px: 12.0,
            icon_gap_px: 10.0,
            icon_size_px: 16.0,
            icon_offset_y_px: 2.0,
            title_size_px: 14.0,
            title_line_height_px: 20.0,
            description_size_px: 14.0,
            description_line_height_px: 20.0,
            title_is_semibold: true,
            action_top_px: 10.0,
            action_right_px: 12.0,
        },
    }
}

/// Resolves the style-pack radius used by the source alert CSS.
pub(super) fn radius_px(theme: &Theme, radius: AlertRadius) -> f32 {
    let scale = theme.style.radius;

    let value = match radius {
        AlertRadius::Theme => match theme.style_id() {
            StyleId::Vega | StyleId::Nova | StyleId::Maia | StyleId::Mira => scale.lg_px,
            StyleId::Lyra | StyleId::Sera => 0.0,
            StyleId::Luma | StyleId::Rhea => scale.xxl_px,
        },
        AlertRadius::None => 0.0,
        AlertRadius::Small => scale.sm_px,
        AlertRadius::Medium => scale.md_px,
        AlertRadius::Large => scale.lg_px,
        AlertRadius::Xl => scale.xl_px,
        AlertRadius::Full => 9999.0,
        AlertRadius::Custom(value) if value.is_finite() => value.max(0.0),
        AlertRadius::Custom(_) => 0.0,
    };

    value.max(0.0)
}

pub(super) fn normalize_px(value: f32) -> f32 {
    if value.is_finite() {
        value.max(0.0)
    } else {
        0.0
    }
}

pub(super) fn normalize_min_px(value: f32) -> f32 {
    if value.is_finite() {
        value.max(1.0)
    } else {
        1.0
    }
}

pub(super) fn normalize_padding(
    mut padding: crate::iced_compat::Padding,
) -> crate::iced_compat::Padding {
    padding.top = normalize_px(padding.top);
    padding.right = normalize_px(padding.right);
    padding.bottom = normalize_px(padding.bottom);
    padding.left = normalize_px(padding.left);
    padding
}
