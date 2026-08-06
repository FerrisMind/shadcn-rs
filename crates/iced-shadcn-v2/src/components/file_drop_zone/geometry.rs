//! Layout helpers for the default file-drop-zone trigger.
//!
//! Numeric recipe tokens are pack-invariant; [`Metrics::radius_px`] resolves
//! `rounded-lg` against the active theme's radius scale so Rhea/Sera/… differ.

use crate::recipes::component_radius_px;
use crate::theme::Theme;

use shadcn_common::FileDropZoneRecipe;

/// Resolved geometry for one theme (shared extras tokens + pack radius).
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct Metrics {
    pub(super) recipe: FileDropZoneRecipe,
    pub(super) radius_px: f32,
}

pub(super) fn metrics(theme: &Theme) -> Metrics {
    let recipe = theme.style.file_drop_zone();
    Metrics {
        radius_px: component_radius_px(theme, recipe.radius),
        recipe,
    }
}
