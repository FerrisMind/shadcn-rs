//! Avatar dimensions and typography metrics.

use shadcn_common::StyleId;

use super::types::{AvatarRadius, AvatarSize};
use crate::theme::Theme;

#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct TextMetrics {
    pub(super) size_px: f32,
    pub(super) line_height_px: f32,
}

/// The badge footprint encoded by the source `data-size` selectors.
pub(super) fn badge_size(size: AvatarSize) -> f32 {
    match size {
        AvatarSize::Sm => 8.0,
        AvatarSize::Default => 10.0,
        AvatarSize::Lg => 12.0,
        AvatarSize::Custom(_) => (size.pixels() * 0.3125).clamp(6.0, 16.0),
    }
}

/// Returns the source icon footprint for an explicitly icon-marked badge.
pub(super) fn badge_icon_size(size: AvatarSize) -> Option<f32> {
    match size {
        AvatarSize::Sm => None,
        AvatarSize::Default | AvatarSize::Lg | AvatarSize::Custom(_) => Some(8.0),
    }
}

/// Returns the source icon footprint for an explicitly icon-marked group count.
pub(super) fn group_count_icon_size(size: AvatarSize) -> f32 {
    match size {
        AvatarSize::Sm => 12.0,
        AvatarSize::Default => 16.0,
        AvatarSize::Lg => 20.0,
        AvatarSize::Custom(value) if value.is_finite() && value <= 24.0 => 12.0,
        AvatarSize::Custom(value) if value.is_finite() && value <= 32.0 => 16.0,
        AvatarSize::Custom(_) => 20.0,
    }
}

/// Fallback initials use `text-sm`, except for the compact `sm` root.
pub(super) fn fallback_metrics(size: AvatarSize) -> TextMetrics {
    if matches!(size, AvatarSize::Sm)
        || matches!(size, AvatarSize::Custom(value) if value.is_finite() && value <= 24.0)
    {
        TextMetrics {
            size_px: 12.0,
            line_height_px: 16.0,
        }
    } else {
        TextMetrics {
            size_px: 14.0,
            line_height_px: 20.0,
        }
    }
}

/// Group counts follow the compact Mira type recipe, otherwise `text-sm`.
pub(super) fn group_count_metrics(theme: &Theme, size: AvatarSize) -> TextMetrics {
    if theme.style_id() == StyleId::Mira {
        TextMetrics {
            size_px: 12.0,
            line_height_px: 19.5,
        }
    } else if matches!(size, AvatarSize::Sm)
        || matches!(size, AvatarSize::Custom(value) if value.is_finite() && value <= 24.0)
    {
        TextMetrics {
            size_px: 12.0,
            line_height_px: 16.0,
        }
    } else {
        TextMetrics {
            size_px: 14.0,
            line_height_px: 20.0,
        }
    }
}

/// Resolves the root radius. `Theme` deliberately stays full-round: the
/// source avatar uses `rounded-full` instead of the pack's component radius.
pub(super) fn radius_px(theme: &Theme, radius: AvatarRadius) -> f32 {
    let scale = theme.style.radius;
    let value = match radius {
        AvatarRadius::Theme | AvatarRadius::Full => 9999.0,
        AvatarRadius::None => 0.0,
        AvatarRadius::Small => scale.sm_px,
        AvatarRadius::Medium => scale.md_px,
        AvatarRadius::Large => scale.lg_px,
        AvatarRadius::Xl => scale.xl_px,
        AvatarRadius::Custom(value) if value.is_finite() => value.max(0.0),
        AvatarRadius::Custom(_) => 0.0,
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

pub(super) fn normalize_opacity(value: f32) -> f32 {
    if value.is_finite() {
        value.clamp(0.0, 1.0)
    } else {
        1.0
    }
}

pub(super) fn normalize_scale(value: f32) -> f32 {
    if value.is_finite() {
        value.max(0.0)
    } else {
        1.0
    }
}
