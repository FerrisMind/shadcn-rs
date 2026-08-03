//! Geometry helpers for the star-rating canvas.

use crate::iced_compat::{Length, Point, Rectangle, Size};

use super::StarRating;
use super::types::StarRatingOrientation;
use crate::theme::Theme;

/// Resolved pixel metrics for one star-rating instance.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct Metrics {
    pub(super) star_size: f32,
    pub(super) gap: f32,
    pub(super) ring_width: f32,
    pub(super) ring_offset: f32,
    pub(super) count: usize,
}

impl Metrics {
    /// Space reserved around each star for the focus ring.
    pub(super) fn ring_reserve(self) -> f32 {
        self.ring_width + self.ring_offset
    }

    /// Cross-axis widget size (star + ring reserve on both sides).
    pub(super) fn cross_size(self) -> f32 {
        self.star_size + self.ring_reserve() * 2.0
    }

    /// Main-axis widget size for `count` stars.
    pub(super) fn main_size(self) -> f32 {
        if self.count == 0 {
            return self.ring_reserve() * 2.0;
        }
        let stars = self.count as f32 * self.star_size;
        let gaps = (self.count.saturating_sub(1)) as f32 * self.gap;
        stars + gaps + self.ring_reserve() * 2.0
    }
}

pub(super) fn resolve_metrics<Message>(rating: &StarRating<'_, Message>) -> Metrics {
    let recipe = rating.theme.style.star_rating();
    Metrics {
        star_size: rating.star_size.pixels(),
        gap: recipe.gap_px,
        ring_width: recipe.ring_width_px,
        ring_offset: recipe.ring_offset_px,
        count: rating.config().star_count(),
    }
}

/// Widget dimensions, honouring explicit overrides.
pub(super) fn resolved_dimensions<Message>(rating: &StarRating<'_, Message>) -> (Length, Length) {
    let metrics = resolve_metrics(rating);
    match rating.orientation {
        StarRatingOrientation::Horizontal => (
            rating.width.unwrap_or(Length::Fixed(metrics.main_size())),
            rating.height.unwrap_or(Length::Fixed(metrics.cross_size())),
        ),
        StarRatingOrientation::Vertical => (
            rating.width.unwrap_or(Length::Fixed(metrics.cross_size())),
            rating.height.unwrap_or(Length::Fixed(metrics.main_size())),
        ),
    }
}

/// Local rectangle of the star at `index`.
pub(super) fn star_rect(
    metrics: Metrics,
    orientation: StarRatingOrientation,
    index: usize,
) -> Rectangle {
    let reserve = metrics.ring_reserve();
    match orientation {
        StarRatingOrientation::Horizontal => Rectangle {
            x: reserve + index as f32 * (metrics.star_size + metrics.gap),
            y: reserve,
            width: metrics.star_size,
            height: metrics.star_size,
        },
        StarRatingOrientation::Vertical => Rectangle {
            x: reserve,
            y: reserve + index as f32 * (metrics.star_size + metrics.gap),
            width: metrics.star_size,
            height: metrics.star_size,
        },
    }
}

/// Paint rectangle of the star at `index` (does not include gaps).
///
/// Prefer [`hit_star`] for pointer interaction — gaps between stars are dead
/// zones for painting but must stay interactive so hover preview does not
/// flicker when the cursor crosses a `gap-1` gutter.
#[cfg_attr(not(test), allow(dead_code))]
pub(super) fn star_at(
    metrics: Metrics,
    orientation: StarRatingOrientation,
    position: Point,
) -> Option<usize> {
    (0..metrics.count).find(|&index| star_rect(metrics, orientation, index).contains(position))
}

/// Hit rectangle for the star at `index`, absorbing half the gap on each side.
///
/// Neighbouring stars meet in the middle of the gutter so the cursor never
/// leaves every hit target while still over the control.
pub(super) fn hit_rect(
    metrics: Metrics,
    orientation: StarRatingOrientation,
    index: usize,
) -> Rectangle {
    let paint = star_rect(metrics, orientation, index);
    let half_gap = metrics.gap * 0.5;
    let leading = if index == 0 { 0.0 } else { half_gap };
    let trailing = if index + 1 >= metrics.count {
        0.0
    } else {
        half_gap
    };

    match orientation {
        StarRatingOrientation::Horizontal => Rectangle {
            x: paint.x - leading,
            y: paint.y,
            width: paint.width + leading + trailing,
            height: paint.height,
        },
        StarRatingOrientation::Vertical => Rectangle {
            x: paint.x,
            y: paint.y - leading,
            width: paint.width,
            height: paint.height + leading + trailing,
        },
    }
}

/// Star under the pointer for interaction, including gap gutters.
pub(super) fn hit_star(
    metrics: Metrics,
    orientation: StarRatingOrientation,
    position: Point,
) -> Option<usize> {
    (0..metrics.count).find(|&index| hit_rect(metrics, orientation, index).contains(position))
}

/// Normalised fraction of `position` along the **paint** star (`0..=1`).
///
/// Positions in the leading/trailing gap gutters clamp to `0` / `1` so half
/// ratings still resolve against the visible star, not the expanded hit box.
pub(super) fn fraction_in_star(
    rect: Rectangle,
    orientation: StarRatingOrientation,
    position: Point,
) -> f32 {
    match orientation {
        StarRatingOrientation::Horizontal => {
            if rect.width <= f32::EPSILON {
                0.0
            } else {
                ((position.x - rect.x) / rect.width).clamp(0.0, 1.0)
            }
        }
        StarRatingOrientation::Vertical => {
            if rect.height <= f32::EPSILON {
                0.0
            } else {
                ((position.y - rect.y) / rect.height).clamp(0.0, 1.0)
            }
        }
    }
}

/// Content size used when laying out without overrides.
#[allow(dead_code)]
pub(super) fn content_size(
    theme: &Theme,
    star_size: f32,
    count: usize,
    orientation: StarRatingOrientation,
) -> Size {
    let recipe = theme.style.star_rating();
    let metrics = Metrics {
        star_size,
        gap: recipe.gap_px,
        ring_width: recipe.ring_width_px,
        ring_offset: recipe.ring_offset_px,
        count,
    };
    match orientation {
        StarRatingOrientation::Horizontal => Size::new(metrics.main_size(), metrics.cross_size()),
        StarRatingOrientation::Vertical => Size::new(metrics.cross_size(), metrics.main_size()),
    }
}
