//! Geometry, layout, and value mapping for the slider component.

use crate::iced_compat::{Length, Point, Rectangle, Size};

use super::Slider;
use super::types::{SliderOrientation, SliderRadius};
use crate::recipes::component_radius_px;
use crate::theme::Theme;

/// Resolved pixel geometry of one slider instance.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct Metrics {
    pub(super) track_thickness: f32,
    pub(super) thumb_length: f32,
    pub(super) thumb_thickness: f32,
    pub(super) ring_width: f32,
}

impl Metrics {
    /// Widget size across the axis: the thumb plus the room its ring needs.
    ///
    /// The reserve is constant, so hovering or focusing a thumb never reflows
    /// the surrounding layout and the ring is never clipped.
    pub(super) fn cross_size(self) -> f32 {
        self.thumb_thickness + self.ring_width * 2.0
    }
}

pub(super) fn resolve_metrics(theme: &Theme) -> Metrics {
    let recipe = theme.style.slider();

    Metrics {
        track_thickness: recipe.track_thickness_px,
        thumb_length: recipe.thumb_length_px,
        thumb_thickness: recipe.thumb_thickness_px,
        ring_width: recipe.ring_width_px,
    }
}

/// Size of a painted thumb in the widget's local axes.
pub(super) fn thumb_size(metrics: Metrics, orientation: SliderOrientation) -> Size {
    match orientation {
        SliderOrientation::Horizontal => Size::new(metrics.thumb_length, metrics.thumb_thickness),
        SliderOrientation::Vertical => Size::new(metrics.thumb_thickness, metrics.thumb_length),
    }
}

/// Widget dimensions, honoring explicit overrides from the builder.
pub(super) fn resolved_dimensions<Message>(slider: &Slider<'_, Message>) -> (Length, Length) {
    let metrics = resolve_metrics(slider.theme);
    let cross = Length::Fixed(metrics.cross_size());

    match slider.orientation {
        SliderOrientation::Horizontal => (
            slider.width.unwrap_or(Length::Fill),
            slider.height.unwrap_or(cross),
        ),
        SliderOrientation::Vertical => (
            slider.width.unwrap_or(cross),
            slider.height.unwrap_or(Length::Fill),
        ),
    }
}

/// Track rectangle inside `bounds`, in canvas-local coordinates.
///
/// The track is inset along the axis by the ring width so a ring painted around
/// a thumb at either end still fits inside the widget.
pub(super) fn track_rect(
    bounds: Size,
    metrics: Metrics,
    orientation: SliderOrientation,
) -> Rectangle {
    match orientation {
        SliderOrientation::Horizontal => {
            let length = (bounds.width - metrics.ring_width * 2.0).max(0.0);
            Rectangle {
                x: metrics.ring_width,
                y: ((bounds.height - metrics.track_thickness) / 2.0).max(0.0),
                width: length,
                height: metrics.track_thickness.min(bounds.height),
            }
        }
        SliderOrientation::Vertical => {
            let length = (bounds.height - metrics.ring_width * 2.0).max(0.0);
            Rectangle {
                x: ((bounds.width - metrics.track_thickness) / 2.0).max(0.0),
                y: metrics.ring_width,
                width: metrics.track_thickness.min(bounds.width),
                height: length,
            }
        }
    }
}

/// Length of the track along the slider axis.
pub(super) fn track_length(track: Rectangle, orientation: SliderOrientation) -> f32 {
    match orientation {
        SliderOrientation::Horizontal => track.width,
        SliderOrientation::Vertical => track.height,
    }
}

/// Distance a thumb center can travel along the track.
///
/// Thumbs stay fully inside the track, matching the `contain` thumb positioning
/// of the web component.
pub(super) fn travel(track: Rectangle, metrics: Metrics, orientation: SliderOrientation) -> f32 {
    (track_length(track, orientation) - metrics.thumb_length).max(0.0)
}

/// Center of the thumb painted for `fraction` (`0.0..=1.0`).
pub(super) fn thumb_center(
    track: Rectangle,
    metrics: Metrics,
    orientation: SliderOrientation,
    fraction: f32,
) -> Point {
    let fraction = fraction.clamp(0.0, 1.0);
    let leading = metrics.thumb_length / 2.0;
    let offset = leading + travel(track, metrics, orientation) * fraction;

    match orientation {
        SliderOrientation::Horizontal => Point::new(track.x + offset, track.y + track.height / 2.0),
        // Vertical sliders grow upwards: the maximum sits at the top edge.
        SliderOrientation::Vertical => Point::new(
            track.x + track.width / 2.0,
            track.y + track_length(track, orientation) - offset,
        ),
    }
}

/// Fraction of the range a value occupies, clamped to `0.0..=1.0`.
pub(super) fn fraction(value: f32, min: f32, max: f32) -> f32 {
    shadcn_common::fraction(value, min, max)
}

/// Fraction of a controlled value after applying the configured step grid.
pub(super) fn snapped_fraction(value: f32, min: f32, max: f32, step: f32) -> f32 {
    shadcn_common::snapped_fraction(value, min, max, step)
}

/// Value a cursor position maps to, snapped to `step` and clamped to the range.
pub(super) fn value_at<Message>(
    slider: &Slider<'_, Message>,
    track: Rectangle,
    metrics: Metrics,
    cursor: Point,
) -> f32 {
    let travel = travel(track, metrics, slider.orientation);
    let leading = metrics.thumb_length / 2.0;

    let raw = if travel <= f32::EPSILON {
        0.0
    } else {
        match slider.orientation {
            SliderOrientation::Horizontal => (cursor.x - track.x - leading) / travel,
            SliderOrientation::Vertical => {
                let length = track_length(track, slider.orientation);
                (track.y + length - leading - cursor.y) / travel
            }
        }
    };

    shadcn_common::value_at_fraction(raw, slider.min, slider.max, slider.step)
}

/// Rounds `value` onto the step grid anchored at `min`.
///
/// A non-positive or non-finite step keeps the slider continuous.
#[cfg(test)]
pub(super) fn snap(value: f32, min: f32, max: f32, step: f32) -> f32 {
    shadcn_common::snap(value, min, max, step)
}

/// Index of the thumb closest to `cursor` along the slider axis.
///
/// Ties resolve to the thumb that can still move towards the cursor, so
/// stacked thumbs at the same value never lock each other in place.
pub(super) fn closest_thumb<Message>(
    slider: &Slider<'_, Message>,
    track: Rectangle,
    metrics: Metrics,
    cursor: Point,
) -> Option<usize> {
    if slider.values.is_empty() {
        return None;
    }

    let target = value_at(slider, track, metrics, cursor);
    shadcn_common::closest_index(&slider.values, target)
}

/// Index of the thumb under `cursor`, if any.
pub(super) fn thumb_at<Message>(
    slider: &Slider<'_, Message>,
    track: Rectangle,
    metrics: Metrics,
    cursor: Point,
) -> Option<usize> {
    slider
        .values
        .iter()
        .copied()
        .enumerate()
        .find(|(_, value)| {
            let center = thumb_center(
                track,
                metrics,
                slider.orientation,
                fraction(*value, slider.min, slider.max),
            );
            let half_length = metrics.thumb_length / 2.0 + metrics.ring_width;
            let half_thickness = metrics.thumb_thickness / 2.0 + metrics.ring_width;

            let (along, across) = match slider.orientation {
                SliderOrientation::Horizontal => (cursor.x - center.x, cursor.y - center.y),
                SliderOrientation::Vertical => (cursor.y - center.y, cursor.x - center.x),
            };

            along.abs() <= half_length && across.abs() <= half_thickness
        })
        .map(|(index, _)| index)
}

pub(super) fn default_track_radius(theme: &Theme) -> SliderRadius {
    pack_radius(theme, theme.style.slider().track_radius)
}

pub(super) fn default_thumb_radius(theme: &Theme) -> SliderRadius {
    pack_radius(theme, theme.style.slider().thumb_radius)
}

fn pack_radius(theme: &Theme, radius: shadcn_common::ComponentRadius) -> SliderRadius {
    match radius {
        shadcn_common::ComponentRadius::None => SliderRadius::None,
        shadcn_common::ComponentRadius::Sm => SliderRadius::Small,
        shadcn_common::ComponentRadius::Md => SliderRadius::Medium,
        shadcn_common::ComponentRadius::Lg => SliderRadius::Large,
        shadcn_common::ComponentRadius::Xl
        | shadcn_common::ComponentRadius::S2xl
        | shadcn_common::ComponentRadius::S3xl
        | shadcn_common::ComponentRadius::S4xl => {
            SliderRadius::Custom(component_radius_px(theme, radius))
        }
        shadcn_common::ComponentRadius::Full => SliderRadius::Full,
        _ => SliderRadius::Medium,
    }
}

/// Resolves a radius preset against a box, capping it to a valid pill radius.
pub(super) fn radius_px(theme: &Theme, radius: SliderRadius, size: Size) -> f32 {
    let max_radius = (size.width.min(size.height) / 2.0).max(0.0);

    match radius {
        SliderRadius::None => 0.0,
        SliderRadius::Small => theme.style.twill_radius_sm.px_value().min(max_radius),
        SliderRadius::Medium => theme.style.twill_radius_md.px_value().min(max_radius),
        SliderRadius::Large => theme.style.twill_radius_lg.px_value().min(max_radius),
        SliderRadius::Full => max_radius,
        SliderRadius::Custom(radius) if radius.is_finite() => radius.max(0.0).min(max_radius),
        SliderRadius::Custom(_) => 0.0,
    }
}
