//! Configuration types for the navigation-menu component.

use shadcn_common::{NavigationMenuAlign, NavigationMenuSide, NavigationMenuTiming, Orientation};

use crate::iced_compat::time::Instant;
use crate::iced_compat::{Length, Padding, Rectangle, Size};

/// Orientation of the navigation-menu list.
///
/// Matches the bits-ui `orientation` prop on `NavigationMenu.Root`.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum NavigationMenuOrientation {
    /// Triggers sit in a horizontal row (default).
    #[default]
    Horizontal,
    /// Triggers stack vertically.
    Vertical,
}

impl NavigationMenuOrientation {
    /// Maps onto the shared [`Orientation`] used by keyboard helpers.
    pub const fn to_orientation(self) -> Orientation {
        match self {
            Self::Horizontal => Orientation::Horizontal,
            Self::Vertical => Orientation::Vertical,
        }
    }
}

/// How list items wrap when they no longer fit on one line.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum NavigationMenuWrap {
    /// Keep items on a single line (default).
    #[default]
    NoWrap,
    /// Wrap onto additional lines.
    Wrap,
    /// Wrap in reverse order.
    WrapReverse,
}

/// Horizontal justification of items within a line.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum NavigationMenuJustify {
    /// Pack toward the start.
    Start,
    /// Center within the available width (default).
    #[default]
    Center,
    /// Pack toward the end.
    End,
}

/// Visual size of top-level triggers and links.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum NavigationMenuSize {
    /// Compact trigger (`Size1`).
    Size1,
    /// Default trigger (`Size2`).
    #[default]
    Size2,
}

impl NavigationMenuSize {
    pub(super) const fn padding(self) -> [f32; 2] {
        match self {
            Self::Size1 => [6.0, 10.0],
            Self::Size2 => [8.0, 14.0],
        }
    }

    pub(super) const fn text_size(self) -> f32 {
        match self {
            Self::Size1 => 12.0,
            Self::Size2 => 14.0,
        }
    }

    pub(super) const fn icon_size(self) -> f32 {
        match self {
            Self::Size1 => 10.0,
            Self::Size2 => 12.0,
        }
    }
}

/// Link surface treatment for top-level and in-content links.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum NavigationMenuLinkVariant {
    /// In-content / default link (`NavigationMenu.Link`).
    #[default]
    Default,
    /// Trigger-styled top-level link (no chevron).
    Trigger,
}

/// Root configuration for [`super::NavigationMenu`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NavigationMenuProps {
    /// List orientation.
    pub orientation: NavigationMenuOrientation,
    /// Hover / skip / close delay knobs (bits-ui defaults).
    pub timing: NavigationMenuTiming,
    /// Whether content is shown in a shared viewport panel.
    pub viewport: bool,
    /// Whether the diamond indicator under the open trigger is drawn.
    pub indicator: bool,
    /// Uncontrolled initial open value.
    pub default_value: Option<&'static str>,
}

impl Default for NavigationMenuProps {
    fn default() -> Self {
        Self {
            orientation: NavigationMenuOrientation::Horizontal,
            timing: NavigationMenuTiming::default(),
            viewport: true,
            indicator: false,
            default_value: None,
        }
    }
}

impl NavigationMenuProps {
    /// Creates props with shadcn / bits-ui defaults.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the list orientation.
    #[must_use]
    pub fn orientation(mut self, orientation: NavigationMenuOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Sets the full timing bundle.
    #[must_use]
    pub fn timing(mut self, timing: NavigationMenuTiming) -> Self {
        self.timing = timing;
        self
    }

    /// Sets `delayDuration`.
    #[must_use]
    pub fn delay_duration_ms(mut self, delay: u64) -> Self {
        self.timing.delay_duration_ms = delay;
        self
    }

    /// Sets `skipDelayDuration`.
    #[must_use]
    pub fn skip_delay_duration_ms(mut self, delay: u64) -> Self {
        self.timing.skip_delay_duration_ms = delay;
        self
    }

    /// Sets `closeDelay`.
    #[must_use]
    pub fn close_delay_ms(mut self, delay: u64) -> Self {
        self.timing.close_delay_ms = delay;
        self
    }

    /// Enables or disables the shared viewport (`viewport` on Root).
    #[must_use]
    pub fn viewport(mut self, viewport: bool) -> Self {
        self.viewport = viewport;
        self
    }

    /// Enables or disables the indicator under the open trigger.
    #[must_use]
    pub fn indicator(mut self, indicator: bool) -> Self {
        self.indicator = indicator;
        self
    }

    /// Sets the uncontrolled initial open value.
    #[must_use]
    pub fn default_value(mut self, value: &'static str) -> Self {
        self.default_value = Some(value);
        self
    }
}

/// Layout knobs for the trigger list (`NavigationMenu.List`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NavigationMenuListProps {
    /// Trigger size.
    pub size: NavigationMenuSize,
    /// Wrap behaviour.
    pub wrap: NavigationMenuWrap,
    /// Line justification.
    pub justify: NavigationMenuJustify,
    /// Gap between items (CSS defaults to `0`).
    pub gap: f32,
    /// Stretch items across the available width.
    pub full_width: bool,
    /// Outer list padding.
    pub padding: f32,
}

impl Default for NavigationMenuListProps {
    fn default() -> Self {
        Self {
            size: NavigationMenuSize::Size2,
            wrap: NavigationMenuWrap::NoWrap,
            justify: NavigationMenuJustify::Center,
            gap: 0.0,
            full_width: false,
            padding: 0.0,
        }
    }
}

impl NavigationMenuListProps {
    /// Creates list props with CSS defaults (`gap-0`, centered).
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the trigger size.
    #[must_use]
    pub fn size(mut self, size: NavigationMenuSize) -> Self {
        self.size = size;
        self
    }

    /// Sets wrap behaviour.
    #[must_use]
    pub fn wrap(mut self, wrap: NavigationMenuWrap) -> Self {
        self.wrap = wrap;
        self
    }

    /// Sets line justification.
    #[must_use]
    pub fn justify(mut self, justify: NavigationMenuJustify) -> Self {
        self.justify = justify;
        self
    }

    /// Sets the gap between items.
    #[must_use]
    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap.max(0.0);
        self
    }

    /// Stretches items across the available width.
    #[must_use]
    pub fn full_width(mut self, full_width: bool) -> Self {
        self.full_width = full_width;
        self
    }

    /// Sets outer list padding.
    #[must_use]
    pub fn padding(mut self, padding: f32) -> Self {
        self.padding = padding.max(0.0);
        self
    }
}

/// Floating content configuration (`NavigationMenu.Content`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NavigationMenuContentProps {
    /// Fixed content width.
    pub width: Option<f32>,
    /// Maximum content height.
    pub max_height: Option<f32>,
    /// Preferred side when `viewport` is off.
    pub side: NavigationMenuSide,
    /// Alignment along the trigger edge.
    pub align: NavigationMenuAlign,
    /// Gap between the trigger and the panel (`mt-1.5` → 6).
    pub side_offset: f32,
    /// Offset along the trigger edge.
    pub align_offset: f32,
    /// Inner content padding.
    pub padding: f32,
    /// Minimum distance from window edges.
    pub collision_padding: f32,
}

impl Default for NavigationMenuContentProps {
    fn default() -> Self {
        Self {
            width: None,
            max_height: None,
            side: NavigationMenuSide::Bottom,
            align: NavigationMenuAlign::Start,
            side_offset: shadcn_common::NAVIGATION_MENU_SIDE_OFFSET_PX,
            align_offset: 0.0,
            padding: 8.0,
            collision_padding: 8.0,
        }
    }
}

impl NavigationMenuContentProps {
    /// Creates content props with shadcn defaults.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets a fixed content width.
    #[must_use]
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width.max(0.0));
        self
    }

    /// Sets a maximum content height.
    #[must_use]
    pub fn max_height(mut self, max_height: f32) -> Self {
        self.max_height = Some(max_height.max(0.0));
        self
    }

    /// Sets the preferred side (`viewport=false`).
    #[must_use]
    pub fn side(mut self, side: NavigationMenuSide) -> Self {
        self.side = side;
        self
    }

    /// Sets alignment along the trigger edge.
    #[must_use]
    pub fn align(mut self, align: NavigationMenuAlign) -> Self {
        self.align = align;
        self
    }

    /// Sets the gap between the trigger and the panel.
    #[must_use]
    pub fn side_offset(mut self, offset: f32) -> Self {
        self.side_offset = offset;
        self
    }

    /// Sets the offset along the trigger edge.
    #[must_use]
    pub fn align_offset(mut self, offset: f32) -> Self {
        self.align_offset = offset;
        self
    }

    /// Sets inner content padding.
    #[must_use]
    pub fn padding(mut self, padding: f32) -> Self {
        self.padding = padding.max(0.0);
        self
    }

    /// Sets the collision padding against window edges.
    #[must_use]
    pub fn collision_padding(mut self, padding: f32) -> Self {
        self.collision_padding = padding.max(0.0);
        self
    }
}

/// Link configuration (`NavigationMenu.Link`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NavigationMenuLinkProps {
    /// Surface variant.
    pub variant: NavigationMenuLinkVariant,
    /// Size (affects padding / type when used as a top-level trigger-style link).
    pub size: NavigationMenuSize,
    /// Uniform padding override.
    pub padding: Option<f32>,
    /// Stretch to the available width.
    pub full_width: bool,
    /// Marks the link as the current page (`data-active`).
    pub active: bool,
    /// Disables interaction (`disabled`).
    pub disabled: bool,
    /// Explicit width.
    pub width: Length,
    /// Explicit height.
    pub height: Length,
}

impl Default for NavigationMenuLinkProps {
    fn default() -> Self {
        Self {
            variant: NavigationMenuLinkVariant::Default,
            size: NavigationMenuSize::Size2,
            padding: None,
            full_width: false,
            active: false,
            disabled: false,
            width: Length::Shrink,
            height: Length::Shrink,
        }
    }
}

impl NavigationMenuLinkProps {
    /// Creates link props with defaults.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the surface variant.
    #[must_use]
    pub fn variant(mut self, variant: NavigationMenuLinkVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Sets the size.
    #[must_use]
    pub fn size(mut self, size: NavigationMenuSize) -> Self {
        self.size = size;
        self
    }

    /// Overrides uniform padding.
    #[must_use]
    pub fn padding(mut self, padding: f32) -> Self {
        self.padding = Some(padding.max(0.0));
        self
    }

    /// Stretches the link to the available width.
    #[must_use]
    pub fn full_width(mut self, full_width: bool) -> Self {
        self.full_width = full_width;
        self
    }

    /// Marks the link active.
    #[must_use]
    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    /// Disables the link.
    #[must_use]
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl NavigationMenuLinkProps {
    pub(super) fn resolved_padding(self) -> Padding {
        let pad = self.padding.unwrap_or(match self.variant {
            NavigationMenuLinkVariant::Trigger => self.size.padding()[0],
            NavigationMenuLinkVariant::Default => 8.0,
        });
        let pad_x = self.padding.unwrap_or(match self.variant {
            NavigationMenuLinkVariant::Trigger => self.size.padding()[1],
            NavigationMenuLinkVariant::Default => 8.0,
        });
        Padding {
            top: pad,
            bottom: pad,
            left: pad_x,
            right: pad_x,
        }
    }
}

/// Layout metrics derived from list props + recipe.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct NavigationMenuMetrics {
    pub(super) list_padding: f32,
    pub(super) gap: f32,
    pub(super) line_gap: f32,
    pub(super) indicator_size: f32,
    pub(super) indicator_offset: f32,
    pub(super) radius: f32,
}

/// Pending hover-open timer.
#[derive(Clone, Copy, Debug)]
pub(super) struct PendingOpen {
    pub(super) index: usize,
    pub(super) started_at: Instant,
}

/// Horizontal content motion while switching triggers.
#[derive(Clone, Copy, Debug)]
pub(super) struct Motion {
    pub(super) direction: i8,
    pub(super) started_at: Instant,
}

/// Widget-tree state for the navigation menu root.
#[derive(Debug, Default)]
pub(super) struct NavigationMenuState {
    pub(super) open_value: Option<String>,
    pub(super) open_index: Option<usize>,
    pub(super) focused: bool,
    pub(super) focus_visible: bool,
    pub(super) focused_index: Option<usize>,
    pub(super) hovered_index: Option<usize>,
    pub(super) trigger_bounds: Vec<Rectangle>,
    pub(super) indicator_from: Option<Rectangle>,
    pub(super) indicator_to: Option<Rectangle>,
    pub(super) indicator_started: Option<Instant>,
    pub(super) motion: Option<Motion>,
    pub(super) pending_open: Option<PendingOpen>,
    pub(super) pending_close: Option<Instant>,
    pub(super) last_close_at: Option<Instant>,
    pub(super) viewport_bounds: Option<Rectangle>,
    pub(super) viewport_size: Option<Size>,
    pub(super) viewport_hovered: bool,
    pub(super) last_redraw: Option<Instant>,
    pub(super) initialized: bool,
}

#[derive(Debug, Default)]
pub(super) struct NavigationMenuLinkState {
    pub(super) is_pressed: bool,
}

#[derive(Debug, Default)]
pub(super) struct NavigationMenuTriggerState {
    pub(super) is_pressed: bool,
    pub(super) is_open: bool,
}
