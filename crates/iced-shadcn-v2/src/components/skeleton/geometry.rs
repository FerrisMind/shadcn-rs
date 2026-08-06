//! Shape geometry for skeleton placeholders.

use crate::iced_compat::Size;

use super::types::{SkeletonRadius, SkeletonShape};
use crate::theme::Theme;

/// Resolves a semantic shape preset to a clamped pixel radius.
pub(super) fn radius_px(theme: &Theme, shape: SkeletonShape, size: Size) -> f32 {
    let max_radius = (size.width.min(size.height) * 0.5).max(0.0);
    let radius = match shape {
        SkeletonShape::Circle => max_radius,
        SkeletonShape::Rounded(SkeletonRadius::None) => 0.0,
        SkeletonShape::Rounded(SkeletonRadius::Small) => theme.style.twill_radius_sm.px_value(),
        SkeletonShape::Rounded(SkeletonRadius::Medium) => theme.style.twill_radius_md.px_value(),
        SkeletonShape::Rounded(SkeletonRadius::Large) => theme.style.twill_radius_lg.px_value(),
        SkeletonShape::Rounded(SkeletonRadius::Full) => max_radius,
        SkeletonShape::Rounded(SkeletonRadius::Custom(radius)) => {
            if radius.is_finite() {
                radius.max(0.0)
            } else {
                0.0
            }
        }
    };

    radius.clamp(0.0, max_radius)
}
