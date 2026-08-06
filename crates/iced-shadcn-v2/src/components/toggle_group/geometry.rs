//! Layout and radius normalization for the toggle group.

use crate::components::toggle::{ToggleRadius, ToggleVariant};
use crate::recipes::component_radius_px;
use crate::theme::Theme;

/// The web component expresses `spacing` in style-pack spacing units. Keep a
/// finite upper bound so hostile values cannot overflow iced's layout math.
const MAX_SPACING_UNITS: f32 = 1024.0;

pub(super) fn normalize_spacing(value: f32) -> f32 {
    if value.is_finite() {
        value.clamp(0.0, MAX_SPACING_UNITS)
    } else {
        0.0
    }
}

pub(super) fn spacing_px(theme: &Theme, spacing: f32) -> f32 {
    let spacing = if spacing.is_finite() {
        spacing.max(0.0)
    } else {
        0.0
    };
    (theme.style.spacing_unit_px * spacing).min(f32::MAX / 2.0)
}

pub(super) fn merged_borders(variant: ToggleVariant, spacing: f32) -> bool {
    variant == ToggleVariant::Outline && spacing <= f32::EPSILON
}

pub(super) fn default_radius_px(theme: &Theme) -> f32 {
    component_radius_px(theme, theme.style.toggle().default_radius)
}

pub(super) fn default_radius(theme: &Theme) -> ToggleRadius {
    match theme.style.toggle().default_radius {
        shadcn_common::ComponentRadius::None => ToggleRadius::None,
        shadcn_common::ComponentRadius::Sm => ToggleRadius::Small,
        shadcn_common::ComponentRadius::Md => ToggleRadius::Medium,
        shadcn_common::ComponentRadius::Lg
        | shadcn_common::ComponentRadius::Xl
        | shadcn_common::ComponentRadius::S2xl
        | shadcn_common::ComponentRadius::S3xl
        | shadcn_common::ComponentRadius::S4xl => ToggleRadius::Large,
        shadcn_common::ComponentRadius::Full => ToggleRadius::Full,
        _ => ToggleRadius::Medium,
    }
}
