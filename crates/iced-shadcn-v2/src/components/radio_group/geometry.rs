//! Indicator, dot, ring, and gap geometry for the radio-group component.

use shadcn_common::RadioGroupRecipe;

use super::types::{RadioGroupRadius, RadioGroupSize};
use crate::recipes::component_radius_px;
use crate::theme::Theme;

/// `RadioGroupSize::Sm` next to `Default` is `size-3.5` next to `size-4`.
const SMALL_SCALE: f32 = 0.875;
/// `RadioGroupSize::Lg` next to `Default` is `size-5` next to `size-4`.
const LARGE_SCALE: f32 = 1.25;
/// The web component expresses gaps in style-pack spacing units. Keep a finite
/// upper bound so hostile values cannot overflow iced's layout math.
const MAX_SPACING_UNITS: f32 = 1024.0;

/// Resolved pixel geometry of one radio indicator.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct Metrics {
    pub(super) indicator: f32,
    pub(super) dot: f32,
    pub(super) border_width: f32,
    pub(super) ring_width: f32,
    pub(super) radius: f32,
}

impl Metrics {
    /// Indicator footprint including the space reserved for the ring.
    ///
    /// The reserve is constant so focusing or invalidating an item never
    /// reflows the surrounding layout; it doubles as the hit-area slop the web
    /// component adds with its `after:-inset-*` pseudo-element.
    pub(super) fn footprint(self) -> f32 {
        self.indicator + self.ring_width * 2.0
    }
}

pub(super) fn resolve_metrics(
    theme: &Theme,
    size: RadioGroupSize,
    radius: Option<RadioGroupRadius>,
) -> Metrics {
    let recipe = theme.style.radio_group();
    let scale = scale(recipe, size);
    let indicator = (recipe.indicator_px * scale).max(1.0);
    let dot_px = if theme.is_dark() {
        recipe.dark_dot_px
    } else {
        recipe.dot_px
    };
    let border_width = recipe.border_width_px * scale;
    // Keep the dot clear of the border at every scale, mirroring the way the
    // web component centers a `size-2` circle inside a bordered `size-4` box.
    let dot = (dot_px * scale).clamp(0.0, (indicator - border_width * 2.0).max(0.0));

    Metrics {
        indicator,
        dot,
        border_width,
        ring_width: recipe.ring_width_px,
        radius: radius_px(theme, radius, indicator),
    }
}

/// Multiplier applied to the pack footprint for one size preset.
fn scale(recipe: RadioGroupRecipe, size: RadioGroupSize) -> f32 {
    match size {
        RadioGroupSize::Sm => SMALL_SCALE,
        RadioGroupSize::Default => 1.0,
        RadioGroupSize::Lg => LARGE_SCALE,
        RadioGroupSize::Custom(diameter) => {
            let diameter = if diameter.is_finite() {
                diameter.max(1.0)
            } else {
                1.0
            };

            if recipe.indicator_px > 0.0 {
                diameter / recipe.indicator_px
            } else {
                1.0
            }
        }
    }
}

/// Radius preset of the active style pack.
pub(super) fn default_radius(theme: &Theme) -> RadioGroupRadius {
    pack_radius(theme, theme.style.radio_group().radius)
}

fn pack_radius(theme: &Theme, radius: shadcn_common::ComponentRadius) -> RadioGroupRadius {
    match radius {
        shadcn_common::ComponentRadius::None => RadioGroupRadius::None,
        shadcn_common::ComponentRadius::Sm => RadioGroupRadius::Small,
        shadcn_common::ComponentRadius::Md => RadioGroupRadius::Medium,
        shadcn_common::ComponentRadius::Lg => RadioGroupRadius::Large,
        shadcn_common::ComponentRadius::Xl
        | shadcn_common::ComponentRadius::S2xl
        | shadcn_common::ComponentRadius::S3xl
        | shadcn_common::ComponentRadius::S4xl => {
            RadioGroupRadius::Custom(component_radius_px(theme, radius))
        }
        shadcn_common::ComponentRadius::Full => RadioGroupRadius::Full,
        _ => RadioGroupRadius::Full,
    }
}

/// Resolves a radius preset against `indicator`, capping it to a valid circle.
fn radius_px(theme: &Theme, radius: Option<RadioGroupRadius>, indicator: f32) -> f32 {
    let max_radius = (indicator / 2.0).max(0.0);

    match radius.unwrap_or_else(|| default_radius(theme)) {
        RadioGroupRadius::None => 0.0,
        RadioGroupRadius::Small => theme.style.twill_radius_sm.px_value().min(max_radius),
        RadioGroupRadius::Medium => theme.style.twill_radius_md.px_value().min(max_radius),
        RadioGroupRadius::Large => theme.style.twill_radius_lg.px_value().min(max_radius),
        RadioGroupRadius::Full => max_radius,
        RadioGroupRadius::Custom(radius) if radius.is_finite() => radius.max(0.0).min(max_radius),
        RadioGroupRadius::Custom(_) => 0.0,
    }
}

pub(super) fn normalize_spacing(value: f32) -> f32 {
    if value.is_finite() {
        value.clamp(0.0, MAX_SPACING_UNITS)
    } else {
        0.0
    }
}

/// Gap between items: the pack's `.cn-radio-group` gap unless overridden.
pub(super) fn gap_px(theme: &Theme, spacing: Option<f32>) -> f32 {
    match spacing {
        Some(units) => (theme.style.spacing_unit_px * units).min(f32::MAX / 2.0),
        None => theme.style.radio_group().gap_px,
    }
}

/// Gap between one indicator and its label, minus the reserved ring width.
///
/// The ring reserve already separates the indicator from the text, so it is
/// subtracted to keep the optical gap equal to the web component's `space-x-2`.
pub(super) fn label_gap_px(theme: &Theme, metrics: Metrics, spacing: Option<f32>) -> f32 {
    let requested = match spacing {
        Some(units) => (theme.style.spacing_unit_px * units).min(f32::MAX / 2.0),
        None => theme.style.radio_group().label_gap_px,
    };

    (requested - metrics.ring_width).max(0.0)
}
