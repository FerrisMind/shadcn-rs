//! Backend-agnostic positioning for floating elements (tooltips, popovers).
//!
//! A minimal port of the floating-ui pipeline — offset → flip → shift →
//! arrow — shared by the iced and egui overlay components. All values are
//! logical pixels in the coordinate space of the anchor rectangle; no
//! iced/egui types are involved.
//!
//! ```rust
//! use shadcn_common::floating::{FloatingConfig, FloatingRect, FloatingSide, compute_floating};
//!
//! let anchor = FloatingRect::new(100.0, 100.0, 80.0, 32.0);
//! let boundary = FloatingRect::new(0.0, 0.0, 800.0, 600.0);
//! let config = FloatingConfig::default().side(FloatingSide::Top).side_offset(4.0);
//!
//! let placement = compute_floating(anchor, 120.0, 24.0, boundary, &config);
//! assert_eq!(placement.side, FloatingSide::Top);
//! assert_eq!(placement.y, 100.0 - 4.0 - 24.0);
//! ```

/// Side of the anchor on which the floating element is placed.
///
/// Matches the `side` prop of bits-ui floating content components.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum FloatingSide {
    /// Above the anchor.
    #[default]
    Top,
    /// To the right of the anchor.
    Right,
    /// Below the anchor.
    Bottom,
    /// To the left of the anchor.
    Left,
}

impl FloatingSide {
    /// The opposite side, used by the flip fallback.
    pub const fn opposite(self) -> Self {
        match self {
            Self::Top => Self::Bottom,
            Self::Bottom => Self::Top,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }

    /// Whether the floating element sits on the horizontal axis (left/right).
    pub const fn is_horizontal(self) -> bool {
        matches!(self, Self::Left | Self::Right)
    }

    /// Kebab-case token used by the web components (`data-side`).
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Top => "top",
            Self::Right => "right",
            Self::Bottom => "bottom",
            Self::Left => "left",
        }
    }
}

/// Alignment of the floating element along the anchor edge.
///
/// Matches the `align` prop of bits-ui floating content components.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum FloatingAlign {
    /// Aligned with the start of the anchor edge.
    Start,
    /// Centered on the anchor edge.
    #[default]
    Center,
    /// Aligned with the end of the anchor edge.
    End,
}

impl FloatingAlign {
    /// Kebab-case token used by the web components (`data-align`).
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::Center => "center",
            Self::End => "end",
        }
    }
}

/// Axis-aligned rectangle in logical pixels.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct FloatingRect {
    /// Left edge.
    pub x: f32,
    /// Top edge.
    pub y: f32,
    /// Horizontal extent.
    pub width: f32,
    /// Vertical extent.
    pub height: f32,
}

impl FloatingRect {
    /// Creates a rectangle from its top-left corner and size.
    pub const fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// Right edge.
    pub const fn right(self) -> f32 {
        self.x + self.width
    }

    /// Bottom edge.
    pub const fn bottom(self) -> f32 {
        self.y + self.height
    }

    /// Horizontal center.
    pub const fn center_x(self) -> f32 {
        self.x + self.width / 2.0
    }

    /// Vertical center.
    pub const fn center_y(self) -> f32 {
        self.y + self.height / 2.0
    }
}

/// Positioning options mirroring the bits-ui floating content props.
#[derive(Debug, Clone, Copy, PartialEq)]
#[must_use = "the config does nothing unless passed to compute_floating"]
pub struct FloatingConfig {
    /// Preferred side of the anchor.
    pub side: FloatingSide,
    /// Alignment along the anchor edge.
    pub align: FloatingAlign,
    /// Gap between the anchor edge and the floating element (`sideOffset`).
    pub side_offset: f32,
    /// Offset along the anchor edge from the alignment origin (`alignOffset`).
    pub align_offset: f32,
    /// Flip to the opposite side / shift into view on overflow
    /// (`avoidCollisions`).
    pub avoid_collisions: bool,
    /// Minimum distance kept from the boundary edges (`collisionPadding`).
    pub collision_padding: f32,
    /// Minimum distance kept between the arrow anchor and the floating
    /// element corners (`arrowPadding`).
    pub arrow_padding: f32,
}

impl Default for FloatingConfig {
    /// bits-ui defaults: top/center, no offsets, collisions avoided with an
    /// 8 px boundary padding.
    fn default() -> Self {
        Self {
            side: FloatingSide::Top,
            align: FloatingAlign::Center,
            side_offset: 0.0,
            align_offset: 0.0,
            avoid_collisions: true,
            collision_padding: 8.0,
            arrow_padding: 0.0,
        }
    }
}

impl FloatingConfig {
    /// Sets the preferred side.
    pub const fn side(mut self, side: FloatingSide) -> Self {
        self.side = side;
        self
    }

    /// Sets the alignment along the anchor edge.
    pub const fn align(mut self, align: FloatingAlign) -> Self {
        self.align = align;
        self
    }

    /// Sets the gap between anchor and floating element.
    pub const fn side_offset(mut self, offset: f32) -> Self {
        self.side_offset = offset;
        self
    }

    /// Sets the offset along the anchor edge.
    pub const fn align_offset(mut self, offset: f32) -> Self {
        self.align_offset = offset;
        self
    }

    /// Enables or disables collision handling (flip + shift).
    pub const fn avoid_collisions(mut self, avoid: bool) -> Self {
        self.avoid_collisions = avoid;
        self
    }

    /// Sets the minimum distance kept from the boundary edges.
    pub const fn collision_padding(mut self, padding: f32) -> Self {
        self.collision_padding = padding;
        self
    }

    /// Sets the minimum distance between arrow and floating corners.
    pub const fn arrow_padding(mut self, padding: f32) -> Self {
        self.arrow_padding = padding;
        self
    }
}

/// Resolved position of a floating element relative to its boundary space.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FloatingPlacement {
    /// Left edge of the floating element.
    pub x: f32,
    /// Top edge of the floating element.
    pub y: f32,
    /// Final side after collision handling (may differ from the preferred
    /// side when flipped).
    pub side: FloatingSide,
    /// Arrow anchor along the floating edge facing the anchor, relative to
    /// the floating element origin: an `x` offset for top/bottom sides, a
    /// `y` offset for left/right sides.
    pub arrow: f32,
}

/// Computes the position of a floating element of `width` × `height` around
/// `anchor`, constrained to `boundary`.
///
/// The pipeline mirrors floating-ui: the preferred side is offset by
/// `side_offset`; when it overflows the boundary the opposite side is tried
/// and the side with more available space wins (flip); the cross axis is
/// shifted into the boundary (shift); finally the arrow anchor is projected
/// onto the floating edge and clamped by `arrow_padding` (arrow).
pub fn compute_floating(
    anchor: FloatingRect,
    width: f32,
    height: f32,
    boundary: FloatingRect,
    config: &FloatingConfig,
) -> FloatingPlacement {
    let side = if config.avoid_collisions {
        resolve_side(anchor, width, height, boundary, config)
    } else {
        config.side
    };

    let main = main_axis_position(anchor, width, height, side, config.side_offset);
    let mut cross = cross_axis_position(anchor, width, height, side, config);

    if config.avoid_collisions {
        cross = shift_into_boundary(cross, width, height, side, boundary, config.collision_padding);
    }

    let (x, y) = if side.is_horizontal() {
        (main, cross)
    } else {
        (cross, main)
    };

    let arrow = arrow_offset(anchor, x, y, width, height, side, config.arrow_padding);

    FloatingPlacement { x, y, side, arrow }
}

/// Space available for the floating element on `side` of the anchor.
fn available_space(
    anchor: FloatingRect,
    side: FloatingSide,
    boundary: FloatingRect,
    padding: f32,
    side_offset: f32,
) -> f32 {
    let space = match side {
        FloatingSide::Top => anchor.y - boundary.y,
        FloatingSide::Bottom => boundary.bottom() - anchor.bottom(),
        FloatingSide::Left => anchor.x - boundary.x,
        FloatingSide::Right => boundary.right() - anchor.right(),
    };

    space - padding - side_offset
}

/// Picks the preferred side or its opposite, whichever has more room when
/// the preferred side cannot fit the floating element.
fn resolve_side(
    anchor: FloatingRect,
    width: f32,
    height: f32,
    boundary: FloatingRect,
    config: &FloatingConfig,
) -> FloatingSide {
    let extent = if config.side.is_horizontal() {
        width
    } else {
        height
    };

    let preferred = available_space(
        anchor,
        config.side,
        boundary,
        config.collision_padding,
        config.side_offset,
    );

    if preferred >= extent {
        return config.side;
    }

    let opposite = available_space(
        anchor,
        config.side.opposite(),
        boundary,
        config.collision_padding,
        config.side_offset,
    );

    if opposite > preferred {
        config.side.opposite()
    } else {
        config.side
    }
}

/// Main-axis coordinate: `y` for top/bottom, `x` for left/right.
fn main_axis_position(
    anchor: FloatingRect,
    width: f32,
    height: f32,
    side: FloatingSide,
    side_offset: f32,
) -> f32 {
    match side {
        FloatingSide::Top => anchor.y - side_offset - height,
        FloatingSide::Bottom => anchor.bottom() + side_offset,
        FloatingSide::Left => anchor.x - side_offset - width,
        FloatingSide::Right => anchor.right() + side_offset,
    }
}

/// Cross-axis coordinate: `x` for top/bottom, `y` for left/right.
fn cross_axis_position(
    anchor: FloatingRect,
    width: f32,
    height: f32,
    side: FloatingSide,
    config: &FloatingConfig,
) -> f32 {
    let (anchor_start, anchor_extent, extent) = if side.is_horizontal() {
        (anchor.y, anchor.height, height)
    } else {
        (anchor.x, anchor.width, width)
    };

    match config.align {
        FloatingAlign::Start => anchor_start + config.align_offset,
        FloatingAlign::Center => {
            anchor_start + (anchor_extent - extent) / 2.0 + config.align_offset
        }
        FloatingAlign::End => anchor_start + anchor_extent - extent - config.align_offset,
    }
}

/// Clamps the cross-axis coordinate so the floating element stays inside the
/// boundary, keeping `padding` from its edges.
fn shift_into_boundary(
    cross: f32,
    width: f32,
    height: f32,
    side: FloatingSide,
    boundary: FloatingRect,
    padding: f32,
) -> f32 {
    let (start, end, extent) = if side.is_horizontal() {
        (boundary.y, boundary.bottom(), height)
    } else {
        (boundary.x, boundary.right(), width)
    };

    let min = start + padding;
    let max = end - padding - extent;

    if max < min {
        // The element cannot fit with padding; center it in the boundary.
        start + (end - start - extent) / 2.0
    } else {
        cross.clamp(min, max)
    }
}

/// Projects the anchor center onto the floating edge facing it, clamped so
/// the arrow never reaches into the corners.
fn arrow_offset(
    anchor: FloatingRect,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    side: FloatingSide,
    padding: f32,
) -> f32 {
    let (target, origin, extent) = if side.is_horizontal() {
        (anchor.center_y(), y, height)
    } else {
        (anchor.center_x(), x, width)
    };

    let offset = target - origin;
    let min = padding;
    let max = extent - padding;

    if max < min {
        extent / 2.0
    } else {
        offset.clamp(min, max)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BOUNDARY: FloatingRect = FloatingRect::new(0.0, 0.0, 800.0, 600.0);
    const ANCHOR: FloatingRect = FloatingRect::new(360.0, 280.0, 80.0, 40.0);

    #[test]
    fn places_on_top_centered() {
        let config = FloatingConfig::default().side_offset(4.0);
        let placement = compute_floating(ANCHOR, 120.0, 24.0, BOUNDARY, &config);

        assert_eq!(placement.side, FloatingSide::Top);
        assert_eq!(placement.y, 280.0 - 4.0 - 24.0);
        assert_eq!(placement.x, 360.0 + (80.0 - 120.0) / 2.0);
        assert_eq!(placement.arrow, 60.0);
    }

    #[test]
    fn flips_to_bottom_when_top_overflows() {
        let anchor = FloatingRect::new(360.0, 4.0, 80.0, 40.0);
        let config = FloatingConfig::default();
        let placement = compute_floating(anchor, 120.0, 24.0, BOUNDARY, &config);

        assert_eq!(placement.side, FloatingSide::Bottom);
        assert_eq!(placement.y, anchor.bottom());
    }

    #[test]
    fn keeps_preferred_side_when_collisions_are_ignored() {
        let anchor = FloatingRect::new(360.0, 4.0, 80.0, 40.0);
        let config = FloatingConfig::default().avoid_collisions(false);
        let placement = compute_floating(anchor, 120.0, 24.0, BOUNDARY, &config);

        assert_eq!(placement.side, FloatingSide::Top);
        assert_eq!(placement.y, 4.0 - 24.0);
    }

    #[test]
    fn shifts_along_cross_axis_into_boundary() {
        let anchor = FloatingRect::new(4.0, 280.0, 40.0, 40.0);
        let config = FloatingConfig::default();
        let placement = compute_floating(anchor, 200.0, 24.0, BOUNDARY, &config);

        assert_eq!(placement.x, 8.0);
        // Arrow still points at the anchor center after shifting.
        assert_eq!(placement.arrow, anchor.center_x() - placement.x);
    }

    #[test]
    fn aligns_start_and_end() {
        let config = FloatingConfig::default().align(FloatingAlign::Start);
        let placement = compute_floating(ANCHOR, 120.0, 24.0, BOUNDARY, &config);
        assert_eq!(placement.x, ANCHOR.x);

        let config = FloatingConfig::default().align(FloatingAlign::End);
        let placement = compute_floating(ANCHOR, 120.0, 24.0, BOUNDARY, &config);
        assert_eq!(placement.x, ANCHOR.right() - 120.0);
    }

    #[test]
    fn side_placements_use_vertical_cross_axis() {
        let config = FloatingConfig::default()
            .side(FloatingSide::Right)
            .side_offset(6.0);
        let placement = compute_floating(ANCHOR, 100.0, 30.0, BOUNDARY, &config);

        assert_eq!(placement.side, FloatingSide::Right);
        assert_eq!(placement.x, ANCHOR.right() + 6.0);
        assert_eq!(placement.y, ANCHOR.y + (40.0 - 30.0) / 2.0);
        assert_eq!(placement.arrow, ANCHOR.center_y() - placement.y);
    }

    #[test]
    fn arrow_respects_padding() {
        let anchor = FloatingRect::new(4.0, 280.0, 10.0, 10.0);
        let config = FloatingConfig::default().arrow_padding(12.0);
        let placement = compute_floating(anchor, 200.0, 24.0, BOUNDARY, &config);

        assert!(placement.arrow >= 12.0);
    }

    #[test]
    fn opposite_sides_roundtrip() {
        for side in [
            FloatingSide::Top,
            FloatingSide::Right,
            FloatingSide::Bottom,
            FloatingSide::Left,
        ] {
            assert_eq!(side.opposite().opposite(), side);
        }
    }
}
