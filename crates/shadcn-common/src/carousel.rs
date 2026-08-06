//! Backend-agnostic carousel scroll-snap engine.
//!
//! Ports the slice of embla-carousel behaviour that the shadcn carousel
//! relies on: snap-point computation with `trimSnaps` containment, slide
//! alignment, loop wrapping, drag settling, and prev/next enablement. The
//! module works in logical pixels along one axis, so a GUI backend maps its
//! own orientation onto `main`-axis values and owns rendering, animation
//! frames, and input plumbing.
//!
//! Geometry model: slide *slots* repeat every `slot_px` and each slot leads
//! with `gap_px` of spacing (the web component's `-ms-4` container margin and
//! `ps-4` item padding). Slide *content* of slot `k` therefore spans
//! `k * slot_px .. (k + 1) * slot_px - gap_px` in strip coordinates, and an
//! offset of `0` shows slide `0` flush with the viewport start.

/// Alignment of a snapped slide inside the viewport (embla `align`).
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum CarouselAlign {
    /// Slide content starts at the viewport start.
    #[default]
    Start,
    /// Slide content is centered inside the viewport.
    Center,
    /// Slide content ends at the viewport end.
    End,
}

/// Spacing between slides and leading strip inset (`-ms-4` / `ps-4`).
pub const CAROUSEL_GAP_PX: f32 = 16.0;

/// Distance from the content edge to the outer edge of a prev/next control
/// (`-start-12` / `-end-12`).
pub const CAROUSEL_CONTROL_OFFSET_PX: f32 = 48.0;

/// Fraction of one slot a drag must travel before it commits a slide change.
pub const CAROUSEL_DRAG_THRESHOLD_FRACTION: f32 = 0.25;

/// Duration of the settle animation between snap points.
pub const CAROUSEL_ANIMATION_MS: f32 = 300.0;

/// Default delay between autoplay advances (embla-carousel-autoplay).
pub const CAROUSEL_AUTOPLAY_DELAY_MS: f32 = 4000.0;

/// One-axis geometry of a carousel strip.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CarouselLayout {
    /// Visible length of the viewport along the scroll axis.
    pub viewport_px: f32,
    /// Distance between the starts of two adjacent slots (content + gap).
    pub slot_px: f32,
    /// Leading spacing inside each slot.
    pub gap_px: f32,
    /// Number of slides.
    pub count: usize,
    /// Whether the strip wraps around (embla `loop`).
    pub looped: bool,
    /// Snap alignment of slides inside the viewport.
    pub align: CarouselAlign,
}

impl CarouselLayout {
    /// Length of the content of one slide (`slot` without its leading gap).
    #[must_use]
    pub fn content_px(&self) -> f32 {
        (self.slot_px - self.gap_px).max(0.0)
    }

    /// Period of the strip: the distance after which slot positions repeat.
    #[must_use]
    pub fn period_px(&self) -> f32 {
        self.slot_px * self.count as f32
    }

    /// Largest reachable offset of a non-looped strip.
    #[must_use]
    pub fn max_offset_px(&self) -> f32 {
        (self.period_px() - self.gap_px - self.viewport_px).max(0.0)
    }

    /// Unclamped offset that aligns slide `index` per [`Self::align`].
    #[must_use]
    pub fn raw_snap_px(&self, index: usize) -> f32 {
        let start = self.slot_px * index as f32;

        match self.align {
            CarouselAlign::Start => start,
            CarouselAlign::Center => start + (self.content_px() - self.viewport_px) / 2.0,
            CarouselAlign::End => start + self.content_px() - self.viewport_px,
        }
    }
}

/// Offsets of every scroll snap (embla `scrollSnapList`).
///
/// Without looping the offsets are contained in the reachable scroll range
/// and consecutive duplicates are removed (embla `containScroll: "trimSnaps"`,
/// the shadcn default), so the list can be shorter than the slide count. With
/// looping every slide keeps its own snap and nothing is clamped.
///
/// ```rust
/// use shadcn_common::{CarouselAlign, CarouselLayout, carousel_snap_offsets};
///
/// let layout = CarouselLayout {
///     viewport_px: 320.0,
///     slot_px: 336.0,
///     gap_px: 16.0,
///     count: 5,
///     looped: false,
///     align: CarouselAlign::Start,
/// };
///
/// assert_eq!(carousel_snap_offsets(&layout).len(), 5);
/// ```
#[must_use]
pub fn carousel_snap_offsets(layout: &CarouselLayout) -> Vec<f32> {
    if layout.count == 0 || !layout.slot_px.is_finite() || layout.slot_px <= 0.0 {
        return Vec::new();
    }

    let slots = vec![layout.slot_px; layout.count];

    carousel_snap_offsets_weighted(
        &slots,
        layout.gap_px,
        layout.viewport_px,
        layout.align,
        layout.looped,
    )
}

/// Start positions of every slot: prefix sums of `slot_lengths`.
///
/// ```rust
/// use shadcn_common::carousel_slot_positions;
///
/// assert_eq!(carousel_slot_positions(&[100.0, 50.0, 50.0]), [0.0, 100.0, 150.0]);
/// ```
#[must_use]
pub fn carousel_slot_positions(slot_lengths: &[f32]) -> Vec<f32> {
    let mut positions = Vec::with_capacity(slot_lengths.len());
    let mut acc = 0.0_f32;

    for length in slot_lengths {
        positions.push(acc);
        acc += if length.is_finite() {
            length.max(0.0)
        } else {
            0.0
        };
    }

    positions
}

/// [`carousel_snap_offsets`] generalized to per-slide slot lengths
/// (the web component's per-item `basis-*` overrides).
#[must_use]
pub fn carousel_snap_offsets_weighted(
    slot_lengths: &[f32],
    gap_px: f32,
    viewport_px: f32,
    align: CarouselAlign,
    looped: bool,
) -> Vec<f32> {
    if slot_lengths.is_empty() {
        return Vec::new();
    }

    let positions = carousel_slot_positions(slot_lengths);
    let period: f32 = slot_lengths
        .iter()
        .map(|length| {
            if length.is_finite() {
                length.max(0.0)
            } else {
                0.0
            }
        })
        .sum();
    let raw = positions.iter().zip(slot_lengths).map(|(start, slot)| {
        let content = (slot - gap_px).max(0.0);

        match align {
            CarouselAlign::Start => *start,
            CarouselAlign::Center => start + (content - viewport_px) / 2.0,
            CarouselAlign::End => start + content - viewport_px,
        }
    });

    if looped {
        return raw.collect();
    }

    let max_offset = (period - gap_px - viewport_px).max(0.0);
    let mut snaps: Vec<f32> = Vec::with_capacity(slot_lengths.len());

    for offset in raw {
        let contained = offset.clamp(0.0, max_offset);

        match snaps.last() {
            Some(last) if (last - contained).abs() <= f32::EPSILON => {}
            _ => snaps.push(contained),
        }
    }

    snaps
}

/// Index of the snap closest to `offset_px`, honoring loop wrapping.
///
/// With looping the distance to every snap is measured modulo `period_px`,
/// so an offset just past the last slide correctly settles on the first one.
#[must_use]
pub fn carousel_nearest_snap(
    snaps: &[f32],
    offset_px: f32,
    period_px: f32,
    looped: bool,
) -> Option<usize> {
    if snaps.is_empty() || !offset_px.is_finite() {
        return None;
    }

    let distance = |snap: f32| -> f32 {
        let direct = (snap - offset_px).abs();

        if looped && period_px.is_finite() && period_px > 0.0 {
            let wrapped = (snap - offset_px).rem_euclid(period_px);
            direct.min(wrapped.min(period_px - wrapped))
        } else {
            direct
        }
    };

    snaps
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| distance(**a).total_cmp(&distance(**b)))
        .map(|(index, _)| index)
}

/// Whether a "previous" control is enabled (embla `canScrollPrev`).
#[must_use]
pub fn carousel_can_scroll_prev(selected: usize, snap_count: usize, looped: bool) -> bool {
    if snap_count <= 1 {
        return false;
    }

    looped || selected > 0
}

/// Whether a "next" control is enabled (embla `canScrollNext`).
#[must_use]
pub fn carousel_can_scroll_next(selected: usize, snap_count: usize, looped: bool) -> bool {
    if snap_count <= 1 {
        return false;
    }

    looped || selected + 1 < snap_count
}

/// Snap reached by scrolling one step backward, if any.
#[must_use]
pub fn carousel_previous_snap(selected: usize, snap_count: usize, looped: bool) -> Option<usize> {
    if !carousel_can_scroll_prev(selected, snap_count, looped) {
        return None;
    }

    Some(if selected == 0 {
        snap_count - 1
    } else {
        selected - 1
    })
}

/// Snap reached by scrolling one step forward, if any.
#[must_use]
pub fn carousel_next_snap(selected: usize, snap_count: usize, looped: bool) -> Option<usize> {
    if !carousel_can_scroll_next(selected, snap_count, looped) {
        return None;
    }

    Some(if selected + 1 >= snap_count {
        0
    } else {
        selected + 1
    })
}

/// Snap reached by stepping `steps` snaps from `selected`.
///
/// Positive steps move forward. Without looping the result saturates at the
/// edges; with looping it wraps. Returns `selected` when the list is empty.
#[must_use]
pub fn carousel_step_snap(selected: usize, snap_count: usize, looped: bool, steps: isize) -> usize {
    if snap_count == 0 {
        return selected;
    }

    let count = snap_count as isize;
    let target = selected.min(snap_count - 1) as isize + steps;

    if looped {
        target.rem_euclid(count) as usize
    } else {
        target.clamp(0, count - 1) as usize
    }
}

/// Normalizes a strip position into `0.0..period` for loop rendering.
#[must_use]
pub fn carousel_wrap_position(position: f32, period_px: f32) -> f32 {
    if !period_px.is_finite() || period_px <= 0.0 || !position.is_finite() {
        return 0.0;
    }

    position.rem_euclid(period_px)
}

/// Representative of `snap_px` (modulo `period_px`) closest to `current_px`.
///
/// A looped strip animates along an unbounded offset axis; this picks the
/// equivalent snap target that produces the shortest travel, so "next" from
/// the last slide keeps moving forward instead of rewinding the whole strip.
#[must_use]
pub fn carousel_loop_target(current_px: f32, snap_px: f32, period_px: f32) -> f32 {
    if !period_px.is_finite() || period_px <= 0.0 {
        return snap_px;
    }

    let current = if current_px.is_finite() {
        current_px
    } else {
        0.0
    };
    let snap = if snap_px.is_finite() { snap_px } else { 0.0 };
    let turns = ((current - snap) / period_px).round();

    snap + turns * period_px
}

/// Snap steps a finished drag settles to (embla drag commit).
///
/// `drag_px` is positive when the pointer moved toward the strip start (i.e.
/// the user pulls the next slide into view). Drags shorter than
/// `threshold_fraction` of one slot settle back to the current snap; longer
/// drags commit to the nearest whole number of slots, at least one.
#[must_use]
pub fn carousel_drag_steps(drag_px: f32, slot_px: f32, threshold_fraction: f32) -> isize {
    if !drag_px.is_finite() || !slot_px.is_finite() || slot_px <= 0.0 {
        return 0;
    }

    if drag_px.abs() < slot_px * threshold_fraction.clamp(0.0, 1.0) {
        return 0;
    }

    let steps = (drag_px / slot_px).round() as isize;

    if steps == 0 {
        drag_px.signum() as isize
    } else {
        steps
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn layout(count: usize, looped: bool, align: CarouselAlign) -> CarouselLayout {
        CarouselLayout {
            viewport_px: 300.0,
            slot_px: 100.0,
            gap_px: 16.0,
            count,
            looped,
            align,
        }
    }

    #[test]
    fn start_snaps_are_trimmed_into_the_scroll_range() {
        // 5 slides of 100 in a 300 viewport: strip = 484, max offset = 184.
        let snaps = carousel_snap_offsets(&layout(5, false, CarouselAlign::Start));

        assert_eq!(snaps, vec![0.0, 100.0, 184.0]);
    }

    #[test]
    fn full_width_slides_keep_one_snap_per_slide() {
        let layout = CarouselLayout {
            viewport_px: 320.0,
            slot_px: 336.0,
            gap_px: 16.0,
            count: 5,
            looped: false,
            align: CarouselAlign::Start,
        };

        assert_eq!(
            carousel_snap_offsets(&layout),
            vec![0.0, 336.0, 672.0, 1008.0, 1344.0]
        );
    }

    #[test]
    fn looped_snaps_are_neither_clamped_nor_deduped() {
        let snaps = carousel_snap_offsets(&layout(5, true, CarouselAlign::Start));

        assert_eq!(snaps, vec![0.0, 100.0, 200.0, 300.0, 400.0]);
    }

    #[test]
    fn center_alignment_offsets_by_half_the_leftover_viewport() {
        let layout = CarouselLayout {
            looped: true,
            ..layout(5, true, CarouselAlign::Center)
        };

        // content = 84; (84 - 300) / 2 = -108.
        assert_eq!(layout.raw_snap_px(1), 100.0 - 108.0);
    }

    #[test]
    fn edges_disable_controls_without_looping() {
        assert!(!carousel_can_scroll_prev(0, 3, false));
        assert!(carousel_can_scroll_next(0, 3, false));
        assert!(carousel_can_scroll_prev(2, 3, false));
        assert!(!carousel_can_scroll_next(2, 3, false));
        assert!(!carousel_can_scroll_prev(0, 1, true));
        assert!(carousel_can_scroll_prev(0, 3, true));
    }

    #[test]
    fn stepping_wraps_only_when_looped() {
        assert_eq!(carousel_previous_snap(0, 3, false), None);
        assert_eq!(carousel_previous_snap(0, 3, true), Some(2));
        assert_eq!(carousel_next_snap(2, 3, false), None);
        assert_eq!(carousel_next_snap(2, 3, true), Some(0));
        assert_eq!(carousel_step_snap(1, 5, false, 10), 4);
        assert_eq!(carousel_step_snap(1, 5, true, -3), 3);
    }

    #[test]
    fn loop_target_takes_the_shortest_path() {
        // From the last of five 100 px slots, snap 0 continues forward.
        assert_eq!(carousel_loop_target(400.0, 0.0, 500.0), 500.0);
        // From the first slot, the previous snap rewinds backward.
        assert_eq!(carousel_loop_target(0.0, 400.0, 500.0), -100.0);
        assert_eq!(carousel_wrap_position(-100.0, 500.0), 400.0);
    }

    #[test]
    fn short_drags_settle_back_and_long_drags_commit() {
        assert_eq!(carousel_drag_steps(10.0, 100.0, 0.25), 0);
        assert_eq!(carousel_drag_steps(30.0, 100.0, 0.25), 1);
        assert_eq!(carousel_drag_steps(-30.0, 100.0, 0.25), -1);
        assert_eq!(carousel_drag_steps(260.0, 100.0, 0.25), 3);
        assert_eq!(carousel_drag_steps(30.0, 0.0, 0.25), 0);
    }

    #[test]
    fn weighted_snaps_follow_prefix_sums() {
        let snaps = carousel_snap_offsets_weighted(
            &[200.0, 100.0, 100.0],
            16.0,
            300.0,
            CarouselAlign::Start,
            false,
        );

        // strip = 400 - 16 = 384; max offset = 84.
        assert_eq!(snaps, vec![0.0, 84.0]);
    }

    #[test]
    fn nearest_snap_wraps_around_the_period() {
        let snaps = [0.0, 100.0, 200.0, 300.0, 400.0];

        assert_eq!(carousel_nearest_snap(&snaps, 90.0, 500.0, false), Some(1));
        assert_eq!(carousel_nearest_snap(&snaps, 470.0, 500.0, true), Some(0));
        assert_eq!(carousel_nearest_snap(&snaps, 470.0, 500.0, false), Some(4));
        assert_eq!(carousel_nearest_snap(&[], 0.0, 500.0, true), None);
    }
}
