//! Normalized strip geometry shared by the track widget and the controls.
//!
//! Every length here is measured in *units*: one unit equals the viewport
//! main length plus one gap (the width of the web component's content box,
//! which its negative start margin widens by one gap). In these units all
//! slot positions, snap offsets, and the loop period scale linearly with the
//! pixel viewport, so the snap structure — including `trimSnaps` containment
//! — can be computed at view time before any layout has happened.

use shadcn_common::{CarouselAlign, carousel_slot_positions, carousel_snap_offsets_weighted};

/// Smallest accepted slide basis, so a slot always has positive length.
pub(super) const MIN_BASIS: f32 = 0.05;

/// Largest accepted slide basis (a slide never spans multiple viewports).
pub(super) const MAX_BASIS: f32 = 1.0;

/// Snap structure of a carousel strip in normalized units.
#[derive(Debug, Clone, PartialEq)]
pub(super) struct Strip {
    /// Sanitized basis of every slide.
    pub(super) bases: Vec<f32>,
    /// Slot start positions (prefix sums of `bases`).
    pub(super) starts: Vec<f32>,
    /// Loop period: total strip length.
    pub(super) period: f32,
    /// Scroll snap offsets (embla `scrollSnapList`).
    pub(super) snaps: Vec<f32>,
}

impl Strip {
    /// Builds the strip for per-slide bases.
    pub(super) fn new(bases: &[f32], align: CarouselAlign, looped: bool) -> Self {
        let bases: Vec<f32> = bases.iter().map(|basis| sanitize_basis(*basis)).collect();
        let starts = carousel_slot_positions(&bases);
        let period: f32 = bases.iter().sum();
        let snaps = carousel_snap_offsets_weighted(&bases, 0.0, 1.0, align, looped);

        Self {
            bases,
            starts,
            period,
            snaps,
        }
    }

    /// Number of scroll snaps (`scrollSnapList().length`).
    pub(super) fn snap_count(&self) -> usize {
        self.snaps.len()
    }

    /// Normalized offset of snap `index`, clamped into the snap list.
    pub(super) fn snap_offset(&self, index: usize) -> f32 {
        match self.snaps.get(index) {
            Some(offset) => *offset,
            None => self.snaps.last().copied().unwrap_or(0.0),
        }
    }
}

/// Clamps a slide basis into the supported range.
pub(super) fn sanitize_basis(basis: f32) -> f32 {
    if basis.is_finite() {
        basis.clamp(MIN_BASIS, MAX_BASIS)
    } else {
        MAX_BASIS
    }
}
