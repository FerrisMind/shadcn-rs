//! Slot, separator, and ring geometry for the input-otp widget.

use crate::iced_compat::{Point, Rectangle, Size};

/// Root `.cn-input-otp` flex gap between groups and separators (`gap-2`).
pub(super) const GROUP_GAP: f32 = 8.0;
/// Separator icon footprint (`size-4` on every pack, `size-3.5` on Sera).
pub(super) const SEPARATOR_SIZE: f32 = 16.0;
/// Shared 1 px border between adjacent slots (`border-y border-r`).
pub(super) const SLOT_BORDER_WIDTH: f32 = 1.0;

/// Resolved footprint numbers of one input-otp instance.
#[derive(Debug, Clone, Copy)]
pub(super) struct OtpMetrics {
    /// Square slot side in px.
    pub(super) slot_size: f32,
    /// Gap between slots inside one group (Sera only).
    pub(super) slot_gap: f32,
    /// Ring width reserved around the widget so the halo never clips.
    pub(super) ring_width: f32,
    /// Whether a minus separator is drawn between groups.
    pub(super) separator: bool,
}

impl OtpMetrics {
    /// Total size of the widget, ring reserve included.
    pub(super) fn total_size(&self, groups: &[usize]) -> Size {
        let width = self.content_width(groups) + self.ring_width * 2.0;
        let height = self.slot_size + self.ring_width * 2.0;
        Size::new(width, height)
    }

    fn group_width(&self, slots: usize) -> f32 {
        let slots = slots as f32;
        self.slot_size * slots + self.slot_gap * (slots - 1.0).max(0.0)
    }

    fn content_width(&self, groups: &[usize]) -> f32 {
        let slots: f32 = groups.iter().map(|count| self.group_width(*count)).sum();
        let joints = groups.len().saturating_sub(1) as f32;
        let separators = if self.separator {
            joints * (SEPARATOR_SIZE + GROUP_GAP * 2.0)
        } else {
            joints * GROUP_GAP
        };
        slots + separators
    }

    /// Group bounds followed by separator bounds, laid out inside `bounds`.
    ///
    /// The content is centered when `bounds` is larger than the natural
    /// footprint (never happens with the shrink-sized widget, but keeps the
    /// draw code robust against custom layouts).
    pub(super) fn regions(&self, bounds: Rectangle, groups: &[usize]) -> OtpRegions {
        let total = self.total_size(groups);
        let mut x = bounds.x + (bounds.width - total.width).max(0.0) / 2.0 + self.ring_width;
        let y = bounds.y + (bounds.height - total.height).max(0.0) / 2.0 + self.ring_width;

        let mut group_bounds = Vec::with_capacity(groups.len());
        let mut separator_bounds = Vec::with_capacity(groups.len().saturating_sub(1));

        for (index, count) in groups.iter().enumerate() {
            if index > 0 {
                if self.separator {
                    separator_bounds.push(Rectangle::new(
                        Point::new(x + GROUP_GAP, y + (self.slot_size - SEPARATOR_SIZE) / 2.0),
                        Size::new(SEPARATOR_SIZE, SEPARATOR_SIZE),
                    ));
                    x += GROUP_GAP * 2.0 + SEPARATOR_SIZE;
                } else {
                    x += GROUP_GAP;
                }
            }

            let width = self.group_width(*count);
            group_bounds.push(Rectangle::new(
                Point::new(x, y),
                Size::new(width, self.slot_size),
            ));
            x += width;
        }

        OtpRegions {
            group_bounds,
            separator_bounds,
        }
    }

    /// Bounds of one slot inside its group.
    pub(super) fn slot_bounds(&self, group: Rectangle, index_in_group: usize) -> Rectangle {
        let offset = (self.slot_size + self.slot_gap) * index_in_group as f32;
        Rectangle::new(
            Point::new(group.x + offset, group.y),
            Size::new(self.slot_size, self.slot_size),
        )
    }
}

/// Group and separator bounds of one laid-out input-otp.
#[derive(Debug, Clone)]
pub(super) struct OtpRegions {
    pub(super) group_bounds: Vec<Rectangle>,
    pub(super) separator_bounds: Vec<Rectangle>,
}

/// Normalizes builder-provided group sizes against `max_length`.
///
/// Zero-sized groups are dropped, oversized layouts are truncated, and any
/// remaining slots are appended as a trailing group, so every slot is always
/// visible and the widget never panics on inconsistent input.
pub(super) fn normalize_groups(max_length: usize, groups: &[usize]) -> Vec<usize> {
    let mut normalized = Vec::with_capacity(groups.len().max(1));
    let mut remaining = max_length;

    for &count in groups {
        if remaining == 0 {
            break;
        }
        let take = count.min(remaining);
        if take > 0 {
            normalized.push(take);
            remaining -= take;
        }
    }

    if remaining > 0 {
        normalized.push(remaining);
    }

    if normalized.is_empty() {
        normalized.push(max_length.max(1));
    }

    normalized
}
