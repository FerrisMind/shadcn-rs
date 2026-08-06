//! Footprint and spacing helpers for pagination controls.

use shadcn_common::{ButtonSizeRecipe, ControlSize};

use crate::components::button::ButtonSize;
use crate::theme::Theme;

/// Spacing is expressed in style-pack spacing units, mirroring the web
/// component. Keep a finite upper bound so hostile values cannot overflow
/// iced's layout math.
const MAX_SPACING_UNITS: f32 = 1024.0;

pub(super) fn normalize_spacing(value: f32) -> f32 {
    if value.is_finite() {
        value.clamp(0.0, MAX_SPACING_UNITS)
    } else {
        0.0
    }
}

/// Gap between pagination items in px. `None` falls back to one spacing
/// unit — the `gap-1` used by shadcn's `pagination-content` list.
pub(super) fn spacing_px(theme: &Theme, units: Option<f32>) -> f32 {
    let units = units.map_or(1.0, normalize_spacing);
    (theme.style.spacing_unit_px * units).min(f32::MAX / 2.0)
}

/// Style-pack size ladder entry backing a [`ButtonSize`], mirroring the
/// mapping used by the button component itself.
pub(super) fn size_recipe(theme: &Theme, size: ButtonSize) -> ButtonSizeRecipe {
    let control_size = match size {
        ButtonSize::Xs | ButtonSize::IconXs => ControlSize::Xs,
        ButtonSize::Sm | ButtonSize::IconSm => ControlSize::Sm,
        ButtonSize::Default | ButtonSize::Icon => ControlSize::Md,
        ButtonSize::Lg | ButtonSize::IconLg => ControlSize::Lg,
    };

    theme.style.button_size(control_size)
}
