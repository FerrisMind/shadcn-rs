//! Size, radius, and value normalization for the meter component.

use crate::iced_compat::Length;
use shadcn_common::meter_ratio;

use super::Meter;
use super::types::{MeterOrientation, MeterRadius, MeterSize, MeterState};
use crate::recipes::component_radius_px;
use crate::theme::Theme;

pub(super) fn resolved_dimensions(
    theme: &Theme,
    size: MeterSize,
    orientation: MeterOrientation,
    width: Option<Length>,
    height: Option<Length>,
) -> (Length, Length) {
    let thickness = Length::Fixed(size.pixels(theme));

    match orientation {
        MeterOrientation::Horizontal => {
            (width.unwrap_or(Length::Fill), height.unwrap_or(thickness))
        }
        MeterOrientation::Vertical => (width.unwrap_or(thickness), height.unwrap_or(Length::Fill)),
    }
}

pub(super) fn default_height(theme: &Theme) -> f32 {
    theme.style.meter().height_px
}

pub(super) fn default_radius(theme: &Theme) -> MeterRadius {
    pack_radius(theme, theme.style.meter().default_radius)
}

fn pack_radius(theme: &Theme, radius: shadcn_common::ComponentRadius) -> MeterRadius {
    match radius {
        shadcn_common::ComponentRadius::None => MeterRadius::None,
        shadcn_common::ComponentRadius::Sm => MeterRadius::Small,
        shadcn_common::ComponentRadius::Md => MeterRadius::Medium,
        shadcn_common::ComponentRadius::Lg => MeterRadius::Large,
        shadcn_common::ComponentRadius::Xl
        | shadcn_common::ComponentRadius::S2xl
        | shadcn_common::ComponentRadius::S3xl
        | shadcn_common::ComponentRadius::S4xl => {
            MeterRadius::Custom(component_radius_px(theme, radius))
        }
        shadcn_common::ComponentRadius::Full => MeterRadius::Full,
        _ => MeterRadius::Full,
    }
}

pub(super) fn radius_px(theme: &Theme, meter: &Meter<'_>, width: f32, height: f32) -> f32 {
    let max_radius = (width.min(height) / 2.0).max(0.0);
    match meter.radius.unwrap_or_else(|| default_radius(theme)) {
        MeterRadius::None => 0.0,
        MeterRadius::Small => theme.style.twill_radius_sm.px_value().min(max_radius),
        MeterRadius::Medium => theme.style.twill_radius_md.px_value().min(max_radius),
        MeterRadius::Large => theme.style.twill_radius_lg.px_value().min(max_radius),
        MeterRadius::Full => max_radius,
        MeterRadius::Custom(radius) if radius.is_finite() => radius.max(0.0).min(max_radius),
        MeterRadius::Custom(_) => 0.0,
    }
}

pub(super) fn target_ratio(meter: &Meter<'_>) -> f32 {
    meter_ratio(meter.config())
}

/// Ratio the canvas should paint.
///
/// While a value transition is running, the shared scalar transition drives the
/// animation. At rest the ratio comes straight from `value`/`min`/`max`.
pub(super) fn display_ratio(state: &MeterState, meter: &Meter<'_>) -> f32 {
    if state.transition.is_running() {
        state.transition.current().clamp(0.0, 1.0)
    } else {
        target_ratio(meter)
    }
}

pub(super) fn sync_transition(
    state: &mut MeterState,
    meter: &Meter<'_>,
    animated: bool,
    now: crate::iced_compat::time::Instant,
) {
    let target = target_ratio(meter);
    if !state.initialized {
        state.initialized = true;
        state.target_ratio = target;
        state.transition.reset(target);
        return;
    }

    if (state.target_ratio - target).abs() > f32::EPSILON {
        state.target_ratio = target;
    }

    state.transition.advance(
        target,
        animated,
        meter.transition_duration,
        shadcn_common::Easing::EaseInOut,
        now,
    );
}

impl MeterSize {
    pub(super) fn pixels(self, theme: &Theme) -> f32 {
        match self {
            Self::Xs => 2.0,
            Self::Sm => 4.0,
            Self::Default => default_height(theme),
            Self::Lg => 12.0,
            Self::Xl => 16.0,
            Self::Custom(value) if value.is_finite() => value.max(1.0),
            Self::Custom(_) => 1.0,
        }
    }
}
