//! Backend-agnostic navigation-menu timing and motion helpers.
//!
//! Ports the bits-ui / Radix Navigation Menu delay rules so egui and iced
//! share one behaviour layer. Geometry types stay as plain floats —
//! backends map them onto their native rects.

use crate::recipes::{
    NAVIGATION_MENU_CLOSE_DELAY_MS, NAVIGATION_MENU_DELAY_DURATION_MS,
    NAVIGATION_MENU_FAST_DELAY_MS, NAVIGATION_MENU_MOTION_ANIM_MS,
    NAVIGATION_MENU_MOTION_DISTANCE_CONTENT_PX, NAVIGATION_MENU_MOTION_DISTANCE_VIEWPORT_PX,
    NAVIGATION_MENU_SKIP_DELAY_DURATION_MS,
};

/// Timing knobs that mirror bits-ui `NavigationMenu.Root` props.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NavigationMenuTiming {
    /// Hover delay before opening (`delayDuration`).
    pub delay_duration_ms: u64,
    /// After-close window that shortens the next open delay
    /// (`skipDelayDuration`).
    pub skip_delay_duration_ms: u64,
    /// Delay before closing once the pointer left both trigger and content
    /// (`closeDelay`). Zero means reuse [`Self::derived_open_delay_ms`].
    pub close_delay_ms: u64,
}

impl Default for NavigationMenuTiming {
    fn default() -> Self {
        Self {
            delay_duration_ms: NAVIGATION_MENU_DELAY_DURATION_MS,
            skip_delay_duration_ms: NAVIGATION_MENU_SKIP_DELAY_DURATION_MS,
            close_delay_ms: NAVIGATION_MENU_CLOSE_DELAY_MS,
        }
    }
}

impl NavigationMenuTiming {
    /// Creates timing with bits-ui defaults.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            delay_duration_ms: NAVIGATION_MENU_DELAY_DURATION_MS,
            skip_delay_duration_ms: NAVIGATION_MENU_SKIP_DELAY_DURATION_MS,
            close_delay_ms: NAVIGATION_MENU_CLOSE_DELAY_MS,
        }
    }

    /// Whether `elapsed_since_close_ms` falls inside the skip-delay window.
    #[must_use]
    pub const fn should_skip_delay(self, elapsed_since_close_ms: Option<u64>) -> bool {
        if self.skip_delay_duration_ms == 0 {
            return false;
        }

        match elapsed_since_close_ms {
            Some(elapsed) => elapsed <= self.skip_delay_duration_ms,
            None => false,
        }
    }

    /// Open delay for the current interaction context.
    ///
    /// Uses the fast delay while another item is already open, or while
    /// still inside the skip-delay window after a close.
    #[must_use]
    pub const fn derived_open_delay_ms(
        self,
        item_already_open: bool,
        elapsed_since_close_ms: Option<u64>,
    ) -> u64 {
        if item_already_open || self.should_skip_delay(elapsed_since_close_ms) {
            NAVIGATION_MENU_FAST_DELAY_MS
        } else {
            self.delay_duration_ms
        }
    }

    /// Close delay for the current interaction context.
    #[must_use]
    pub const fn derived_close_delay_ms(
        self,
        item_already_open: bool,
        elapsed_since_close_ms: Option<u64>,
    ) -> u64 {
        if self.close_delay_ms > 0 {
            self.close_delay_ms
        } else {
            self.derived_open_delay_ms(item_already_open, elapsed_since_close_ms)
        }
    }
}

/// Horizontal content slide while switching between open triggers.
///
/// `progress` is `0..=1` over [`NAVIGATION_MENU_MOTION_ANIM_MS`].
/// `direction` is `+1` when moving to a later item, `-1` to an earlier one.
#[must_use]
pub fn motion_offset_x(progress: f32, direction: i8, viewport: bool) -> f32 {
    let t = progress.clamp(0.0, 1.0);
    let distance = if viewport {
        NAVIGATION_MENU_MOTION_DISTANCE_VIEWPORT_PX
    } else {
        NAVIGATION_MENU_MOTION_DISTANCE_CONTENT_PX
    };
    (1.0 - t) * distance * f32::from(direction)
}

/// Motion animation duration in milliseconds.
#[must_use]
pub const fn motion_duration_ms() -> u64 {
    NAVIGATION_MENU_MOTION_ANIM_MS
}

/// Axis-aligned float rectangle used by placement helpers.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NavRect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl NavRect {
    /// Creates a rectangle from origin and size.
    #[must_use]
    pub const fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// Linearly interpolates between two rectangles.
    #[must_use]
    pub fn lerp(self, to: Self, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        Self {
            x: self.x + (to.x - self.x) * t,
            y: self.y + (to.y - self.y) * t,
            width: self.width + (to.width - self.width) * t,
            height: self.height + (to.height - self.height) * t,
        }
    }
}

/// Horizontal alignment of the floating panel relative to the trigger.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum NavigationMenuAlign {
    /// Align with the start of the trigger edge.
    #[default]
    Start,
    /// Center on the trigger edge.
    Center,
    /// Align with the end of the trigger edge.
    End,
}

/// Preferred side of the trigger for floating content when `viewport` is off.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum NavigationMenuSide {
    /// Above the trigger.
    Top,
    /// To the right of the trigger.
    Right,
    /// Below the trigger.
    #[default]
    Bottom,
    /// To the left of the trigger.
    Left,
}

/// Places the shared viewport below the active trigger, clamping into the
/// window with `collision_padding`.
#[must_use]
#[allow(clippy::too_many_arguments)]
pub fn place_viewport(
    trigger: NavRect,
    content_width: f32,
    content_height: f32,
    align: NavigationMenuAlign,
    align_offset: f32,
    side_offset: f32,
    collision_padding: f32,
    window_width: f32,
    window_height: f32,
) -> (f32, f32) {
    let base_x = match align {
        NavigationMenuAlign::Start => trigger.x,
        NavigationMenuAlign::Center => trigger.x + (trigger.width - content_width) / 2.0,
        NavigationMenuAlign::End => trigger.x + trigger.width - content_width,
    } + align_offset;

    let x = base_x.clamp(
        collision_padding,
        (window_width - content_width - collision_padding).max(collision_padding),
    );
    let y = (trigger.y + trigger.height + side_offset).clamp(
        collision_padding,
        (window_height - content_height - collision_padding).max(collision_padding),
    );

    (x, y)
}

/// Places a per-item floating panel relative to its trigger (`viewport=false`).
#[must_use]
#[allow(clippy::too_many_arguments)]
pub fn place_content(
    trigger: NavRect,
    content_width: f32,
    content_height: f32,
    side: NavigationMenuSide,
    align: NavigationMenuAlign,
    side_offset: f32,
    align_offset: f32,
    collision_padding: f32,
    window_width: f32,
    window_height: f32,
) -> (f32, f32) {
    let (mut x, mut y) = match side {
        NavigationMenuSide::Top => (trigger.x, trigger.y - content_height - side_offset),
        NavigationMenuSide::Bottom => (trigger.x, trigger.y + trigger.height + side_offset),
        NavigationMenuSide::Left => (trigger.x - content_width - side_offset, trigger.y),
        NavigationMenuSide::Right => (trigger.x + trigger.width + side_offset, trigger.y),
    };

    match side {
        NavigationMenuSide::Top | NavigationMenuSide::Bottom => {
            x = match align {
                NavigationMenuAlign::Start => trigger.x,
                NavigationMenuAlign::Center => trigger.x + (trigger.width - content_width) / 2.0,
                NavigationMenuAlign::End => trigger.x + trigger.width - content_width,
            } + align_offset;
        }
        NavigationMenuSide::Left | NavigationMenuSide::Right => {
            y = match align {
                NavigationMenuAlign::Start => trigger.y,
                NavigationMenuAlign::Center => trigger.y + (trigger.height - content_height) / 2.0,
                NavigationMenuAlign::End => trigger.y + trigger.height - content_height,
            } + align_offset;
        }
    }

    x = x.clamp(
        collision_padding,
        (window_width - content_width - collision_padding).max(collision_padding),
    );
    y = y.clamp(
        collision_padding,
        (window_height - content_height - collision_padding).max(collision_padding),
    );

    (x, y)
}

/// Diamond indicator rect centred under a trigger bounds.
#[must_use]
pub fn indicator_diamond(trigger: NavRect, size: f32, offset_y: f32) -> NavRect {
    NavRect::new(
        trigger.x + (trigger.width - size) / 2.0,
        trigger.y + trigger.height + offset_y,
        size,
        size,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skip_delay_shortens_open_delay() {
        let timing = NavigationMenuTiming::default();
        assert_eq!(
            timing.derived_open_delay_ms(false, None),
            NAVIGATION_MENU_DELAY_DURATION_MS
        );
        assert_eq!(
            timing.derived_open_delay_ms(true, None),
            NAVIGATION_MENU_FAST_DELAY_MS
        );
        assert_eq!(
            timing.derived_open_delay_ms(false, Some(100)),
            NAVIGATION_MENU_FAST_DELAY_MS
        );
        assert_eq!(
            timing.derived_open_delay_ms(false, Some(400)),
            NAVIGATION_MENU_DELAY_DURATION_MS
        );
    }

    #[test]
    fn place_viewport_clamps_into_window() {
        let trigger = NavRect::new(900.0, 10.0, 80.0, 32.0);
        let (x, y) = place_viewport(
            trigger,
            200.0,
            120.0,
            NavigationMenuAlign::Start,
            0.0,
            6.0,
            8.0,
            1000.0,
            800.0,
        );
        assert!((x - 792.0).abs() < 0.1);
        assert!((y - 48.0).abs() < 0.1);
    }
}
