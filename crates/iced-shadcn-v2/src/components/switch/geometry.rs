//! Track, thumb, and radius geometry for the switch component.

use crate::iced_compat::{Length, Size};
use shadcn_common::{ControlSize, SwitchSizeRecipe};

use super::Switch;
use super::types::{SwitchRadius, SwitchSize};
use crate::recipes::component_radius_px;
use crate::theme::Theme;

/// Resolved pixel geometry of one switch instance.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct Metrics {
    pub(super) track: Size,
    pub(super) thumb: Size,
    pub(super) thumb_inset: f32,
    pub(super) thumb_travel: f32,
    pub(super) border_width: f32,
    pub(super) ring_width: f32,
}

impl Metrics {
    /// Widget footprint, including the space reserved for the ring.
    ///
    /// The reserve is constant so focusing or invalidating a switch never
    /// reflows the surrounding layout; it doubles as the hit-area slop of the
    /// web component's `after:-inset-*` pseudo-element.
    pub(super) fn bounds(self) -> Size {
        Size::new(
            self.track.width + self.ring_width * 2.0,
            self.track.height + self.ring_width * 2.0,
        )
    }
}

pub(super) fn resolve_metrics(theme: &Theme, size: SwitchSize) -> Metrics {
    let recipe = size_recipe(theme, size);
    let scale = custom_scale(theme, size);
    let switch = theme.style.switch();

    Metrics {
        track: Size::new(
            recipe.track_width_px * scale,
            recipe.track_height_px * scale,
        ),
        thumb: Size::new(
            recipe.thumb_width_px * scale,
            recipe.thumb_height_px * scale,
        ),
        thumb_inset: recipe.thumb_inset_px * scale,
        thumb_travel: recipe.thumb_travel_px * scale,
        border_width: switch.border_width_px * scale,
        ring_width: switch.ring_width_px,
    }
}

pub(super) fn resolved_dimensions(theme: &Theme, size: SwitchSize) -> (Length, Length) {
    let bounds = resolve_metrics(theme, size).bounds();

    (Length::Fixed(bounds.width), Length::Fixed(bounds.height))
}

fn size_recipe(theme: &Theme, size: SwitchSize) -> SwitchSizeRecipe {
    let control = match size {
        SwitchSize::Sm => ControlSize::Sm,
        SwitchSize::Default | SwitchSize::Custom(_) => ControlSize::Md,
    };

    theme.style.switch_size(control)
}

/// Multiplier applied to the pack's `default` footprint for custom heights.
fn custom_scale(theme: &Theme, size: SwitchSize) -> f32 {
    let SwitchSize::Custom(height) = size else {
        return 1.0;
    };

    let height = if height.is_finite() {
        height.max(1.0)
    } else {
        1.0
    };
    let reference = size_recipe(theme, size).track_height_px;

    if reference > 0.0 {
        height / reference
    } else {
        1.0
    }
}

pub(super) fn default_radius(theme: &Theme) -> SwitchRadius {
    pack_radius(theme, theme.style.switch().default_radius)
}

fn pack_radius(theme: &Theme, radius: shadcn_common::ComponentRadius) -> SwitchRadius {
    match radius {
        shadcn_common::ComponentRadius::None => SwitchRadius::None,
        shadcn_common::ComponentRadius::Sm => SwitchRadius::Small,
        shadcn_common::ComponentRadius::Md => SwitchRadius::Medium,
        shadcn_common::ComponentRadius::Lg => SwitchRadius::Large,
        shadcn_common::ComponentRadius::Xl
        | shadcn_common::ComponentRadius::S2xl
        | shadcn_common::ComponentRadius::S3xl
        | shadcn_common::ComponentRadius::S4xl => {
            SwitchRadius::Custom(component_radius_px(theme, radius))
        }
        shadcn_common::ComponentRadius::Full => SwitchRadius::Full,
        _ => SwitchRadius::Medium,
    }
}

/// Resolves a radius preset against `size`, capping it to a valid pill radius.
pub(super) fn radius_px<Message>(switch: &Switch<'_, Message>, size: Size) -> f32 {
    let theme = switch.theme;
    let max_radius = (size.width.min(size.height) / 2.0).max(0.0);

    match switch.radius.unwrap_or_else(|| default_radius(theme)) {
        SwitchRadius::None => 0.0,
        SwitchRadius::Small => theme.style.twill_radius_sm.px_value().min(max_radius),
        SwitchRadius::Medium => theme.style.twill_radius_md.px_value().min(max_radius),
        SwitchRadius::Large => theme.style.twill_radius_lg.px_value().min(max_radius),
        SwitchRadius::Full => max_radius,
        SwitchRadius::Custom(radius) if radius.is_finite() => radius.max(0.0).min(max_radius),
        SwitchRadius::Custom(_) => 0.0,
    }
}

/// Thumb offset from the leading edge of the track for a `0.0..=1.0` position.
pub(super) fn thumb_offset(metrics: Metrics, position: f32) -> f32 {
    metrics.thumb_inset + metrics.thumb_travel * position.clamp(0.0, 1.0)
}
