//! Size, radius, and value normalization for the progress component.

use crate::iced_compat::Length;

use super::Progress;
use super::types::{ProgressOrientation, ProgressRadius, ProgressSize, ProgressState};
use crate::recipes::component_radius_px;
use crate::theme::Theme;

pub(super) fn resolved_dimensions(
    theme: &Theme,
    size: ProgressSize,
    orientation: ProgressOrientation,
    width: Option<Length>,
    height: Option<Length>,
) -> (Length, Length) {
    let thickness = Length::Fixed(size.pixels(theme));

    match orientation {
        ProgressOrientation::Horizontal => {
            (width.unwrap_or(Length::Fill), height.unwrap_or(thickness))
        }
        ProgressOrientation::Vertical => {
            (width.unwrap_or(thickness), height.unwrap_or(Length::Fill))
        }
    }
}

pub(super) fn default_height(theme: &Theme) -> f32 {
    theme.style.progress().height_px
}

pub(super) fn default_radius(theme: &Theme) -> ProgressRadius {
    pack_radius(theme, theme.style.progress().default_radius)
}

fn pack_radius(theme: &Theme, radius: shadcn_common::ComponentRadius) -> ProgressRadius {
    match radius {
        shadcn_common::ComponentRadius::None => ProgressRadius::None,
        shadcn_common::ComponentRadius::Sm => ProgressRadius::Small,
        shadcn_common::ComponentRadius::Md => ProgressRadius::Medium,
        shadcn_common::ComponentRadius::Lg => ProgressRadius::Large,
        shadcn_common::ComponentRadius::Xl
        | shadcn_common::ComponentRadius::S2xl
        | shadcn_common::ComponentRadius::S3xl
        | shadcn_common::ComponentRadius::S4xl => {
            ProgressRadius::Custom(component_radius_px(theme, radius))
        }
        shadcn_common::ComponentRadius::Full => ProgressRadius::Full,
        _ => ProgressRadius::Medium,
    }
}

pub(super) fn radius_px(theme: &Theme, progress: &Progress<'_>, width: f32, height: f32) -> f32 {
    let max_radius = (width.min(height) / 2.0).max(0.0);
    match progress.radius.unwrap_or_else(|| default_radius(theme)) {
        ProgressRadius::None => 0.0,
        ProgressRadius::Small => theme.style.twill_radius_sm.px_value().min(max_radius),
        ProgressRadius::Medium => theme.style.twill_radius_md.px_value().min(max_radius),
        ProgressRadius::Large => theme.style.twill_radius_lg.px_value().min(max_radius),
        ProgressRadius::Full => max_radius,
        ProgressRadius::Custom(radius) if radius.is_finite() => radius.max(0.0).min(max_radius),
        ProgressRadius::Custom(_) => 0.0,
    }
}

pub(super) fn normalized_ratio(value: Option<f32>, max: f32) -> f32 {
    let Some(value) = value else {
        return 0.0;
    };

    if !max.is_finite() || max <= 0.0 || !value.is_finite() {
        return 0.0;
    }

    (value / max).clamp(0.0, 1.0)
}

/// Ratio the canvas should paint for a determinate bar.
///
/// While a value transition is running, the shared scalar transition drives the
/// animation. At rest the ratio is derived straight from `value`/`max`, so a
/// value that changes while the bar is idle (and therefore not pumping redraws
/// into [`super::render`]'s `update`) is reflected immediately instead of
/// leaving the bar stuck on the last animated ratio.
pub(super) fn display_ratio(state: &ProgressState, value: Option<f32>, max: f32) -> f32 {
    if state.transition.is_running() {
        state.transition.current().clamp(0.0, 1.0)
    } else {
        normalized_ratio(value, max)
    }
}

impl ProgressSize {
    pub(super) fn pixels(self, theme: &Theme) -> f32 {
        match self {
            Self::Xs => 2.0,
            Self::Sm => 4.0,
            Self::Default => default_height(theme),
            Self::Lg => 8.0,
            Self::Xl => 12.0,
            Self::Custom(value) if value.is_finite() => value.max(1.0),
            Self::Custom(_) => 1.0,
        }
    }
}
