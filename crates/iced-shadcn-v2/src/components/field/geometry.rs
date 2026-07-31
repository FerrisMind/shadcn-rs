//! Field layout and typography metrics.

use crate::components::card::{CardRadius, geometry as card_geometry};
use crate::theme::Theme;

/// The `@md/field-group` container query used by shadcn-svelte is 28 rem.
pub(super) const DEFAULT_RESPONSIVE_BREAKPOINT: f32 = 448.0;

/// The gap between direct children of a field (`gap-2`).
pub(super) const FIELD_GAP_PX: f32 = 8.0;

/// The default gap between fields in a group (`gap-5`).
pub(super) const FIELD_GROUP_GAP_PX: f32 = 20.0;

/// The default gap between children in a field set (`gap-4`).
pub(super) const FIELD_SET_GAP_PX: f32 = 16.0;

/// The compact gap used by checkbox groups (`gap-3`).
pub(super) const CHECKBOX_GROUP_GAP_PX: f32 = 12.0;

/// The gap between label and description inside [`super::FieldContent`].
pub(super) const FIELD_CONTENT_GAP_PX: f32 = 2.0;

/// The default title/legend line height.
pub(super) const TITLE_LINE_HEIGHT_PX: f32 = 20.0;

/// Normalizes a user-provided pixel value without allowing NaN or infinity to
/// leak into Iced's layout solver.
pub(super) fn normalize_px(value: f32) -> f32 {
    if value.is_finite() {
        value.max(0.0)
    } else {
        0.0
    }
}

/// Returns the choice-card radius used by the active style pack.
///
/// Choice cards are card-like surfaces without their own style tokens, so the
/// corner radius follows the card pack ([`CardRadius::Theme`]).
pub(super) fn choice_card_radius(theme: &Theme) -> f32 {
    card_geometry::radius_px(theme, CardRadius::Theme)
}
