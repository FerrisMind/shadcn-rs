//! Navigation-menu recipes from `.cn-navigation-menu-*` across style packs.
//!
//! Timing constants match bits-ui / Radix Navigation Menu defaults
//! (`delayDuration`, `skipDelayDuration`). Surface tokens mirror the
//! popover viewport (`bg-popover`, `ring-foreground/N`, shadow).

use crate::style::StyleId;

use super::popover::PopoverShadow;
use super::{ComponentRadius, FontWeight, TypeRecipe};

/// Default hover open delay (`delayDuration`) in milliseconds.
pub const NAVIGATION_MENU_DELAY_DURATION_MS: u64 = 200;

/// Window after close during which the open delay is shortened
/// (`skipDelayDuration`).
pub const NAVIGATION_MENU_SKIP_DELAY_DURATION_MS: u64 = 300;

/// Open delay used while another item is already open, or within the
/// skip-delay window after a close.
pub const NAVIGATION_MENU_FAST_DELAY_MS: u64 = 100;

/// Default close delay (`closeDelay`); `0` means reuse the derived open delay.
pub const NAVIGATION_MENU_CLOSE_DELAY_MS: u64 = 0;

/// Indicator cross-fade / slide duration.
pub const NAVIGATION_MENU_INDICATOR_ANIM_MS: u64 = 160;

/// Horizontal content motion when switching triggers.
pub const NAVIGATION_MENU_MOTION_ANIM_MS: u64 = 200;

/// Viewport / popup entrance duration (`duration-100`).
pub const NAVIGATION_MENU_VIEWPORT_ANIM_MS: u64 = 100;

/// Without-viewport content entrance duration (`duration-300`).
pub const NAVIGATION_MENU_CONTENT_ANIM_MS: u64 = 300;

/// Gap between the trigger list and the viewport (`mt-1.5`).
pub const NAVIGATION_MENU_SIDE_OFFSET_PX: f32 = 6.0;

/// Extra padding baked into the viewport size CSS
/// (`+1rem` on width/height).
pub const NAVIGATION_MENU_VIEWPORT_PAD_PX: f32 = 16.0;

/// Slide distance while swapping content inside a shared viewport.
pub const NAVIGATION_MENU_MOTION_DISTANCE_VIEWPORT_PX: f32 = 48.0;

/// Slide distance when each item owns its own floating panel.
pub const NAVIGATION_MENU_MOTION_DISTANCE_CONTENT_PX: f32 = 32.0;

/// Initial scale of the viewport `zoom-in-90` entrance.
pub const NAVIGATION_MENU_VIEWPORT_ZOOM_FROM: f32 = 0.90;

/// Initial scale of the without-viewport `zoom-in-95` entrance.
pub const NAVIGATION_MENU_CONTENT_ZOOM_FROM: f32 = 0.95;

/// Chevron icon size (`size-3`).
pub const NAVIGATION_MENU_CHEVRON_SIZE_PX: f32 = 12.0;

/// Chevron open rotation duration (`duration-300`).
pub const NAVIGATION_MENU_CHEVRON_ROTATE_MS: u64 = 300;

/// Disabled trigger / link opacity (`disabled:opacity-50`).
pub const NAVIGATION_MENU_DISABLED_OPACITY: f32 = 0.50;

/// Open trigger fill alpha against `muted` (`data-open:bg-muted/50`).
pub const NAVIGATION_MENU_OPEN_MUTED_ALPHA: f32 = 0.50;

/// Geometry + typography recipe for navigation-menu triggers, links, and
/// the shared viewport / popup surface.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NavigationMenuRecipe {
    /// Trigger / top-level link vertical padding (`py-2` / `py-1.5` / …).
    pub trigger_pad_y_px: f32,
    /// Trigger / top-level link horizontal padding (`px-4` / `px-2.5` / …).
    pub trigger_pad_x_px: f32,
    /// Trigger / link corner radius intent.
    pub trigger_radius: ComponentRadius,
    /// Trigger / link typography (`text-sm font-medium` / `text-xs`).
    pub trigger_typography: TypeRecipe,
    /// Content-panel padding when `viewport=false` (`p-2` / `p-1` / …).
    pub content_pad_px: f32,
    /// Content-panel corner radius when `viewport=false`.
    pub content_radius: ComponentRadius,
    /// Shared viewport corner radius (`rounded-lg` / `rounded-2xl` / …).
    pub viewport_radius: ComponentRadius,
    /// `ring-foreground/N` alpha in light mode.
    pub ring_alpha: f32,
    /// `ring-foreground/N` alpha in dark mode.
    pub ring_alpha_dark: f32,
    /// Viewport / popup drop shadow.
    pub shadow: PopoverShadow,
    /// In-content link padding (`p-2` / `p-3`).
    pub link_pad_px: f32,
    /// In-content link corner radius.
    pub link_radius: ComponentRadius,
    /// Gap between list items (`gap-0` in CSS — kept for layout flexibility).
    pub list_gap_px: f32,
}

/// Resolves `.cn-navigation-menu-*` tokens for `style`.
pub const fn navigation_menu_recipe(style: StyleId) -> NavigationMenuRecipe {
    let base = base_recipe();

    match style {
        StyleId::Vega => base,
        StyleId::Nova => NavigationMenuRecipe {
            trigger_pad_y_px: 6.0,
            trigger_pad_x_px: 10.0,
            trigger_radius: ComponentRadius::Lg,
            content_pad_px: 4.0,
            content_radius: ComponentRadius::Lg,
            viewport_radius: ComponentRadius::Lg,
            link_radius: ComponentRadius::Md,
            ..base
        },
        StyleId::Mira => NavigationMenuRecipe {
            trigger_pad_y_px: 6.0,
            trigger_pad_x_px: 10.0,
            trigger_radius: ComponentRadius::Lg,
            trigger_typography: text_xs_relaxed(FontWeight::Medium),
            content_radius: ComponentRadius::Lg,
            viewport_radius: ComponentRadius::Lg,
            link_radius: ComponentRadius::Md,
            ..base
        },
        StyleId::Lyra => NavigationMenuRecipe {
            trigger_pad_y_px: 6.0,
            trigger_pad_x_px: 10.0,
            trigger_radius: ComponentRadius::None,
            trigger_typography: text_xs(FontWeight::Medium),
            content_radius: ComponentRadius::None,
            viewport_radius: ComponentRadius::None,
            link_radius: ComponentRadius::None,
            ..base
        },
        StyleId::Maia => NavigationMenuRecipe {
            trigger_pad_y_px: 10.0,
            trigger_pad_x_px: 18.0,
            trigger_radius: ComponentRadius::S2xl,
            content_pad_px: 10.0,
            content_radius: ComponentRadius::S2xl,
            viewport_radius: ComponentRadius::S2xl,
            ring_alpha: 0.05,
            ring_alpha_dark: 0.05,
            shadow: PopoverShadow::XXL,
            link_pad_px: 12.0,
            link_radius: ComponentRadius::Xl,
            ..base
        },
        StyleId::Luma => NavigationMenuRecipe {
            trigger_pad_y_px: 10.0,
            trigger_pad_x_px: 18.0,
            trigger_radius: ComponentRadius::S3xl,
            content_radius: ComponentRadius::S3xl,
            viewport_radius: ComponentRadius::S3xl,
            ring_alpha: 0.05,
            ring_alpha_dark: 0.10,
            shadow: PopoverShadow::LG,
            link_radius: ComponentRadius::Lg,
            ..base
        },
        StyleId::Rhea => NavigationMenuRecipe {
            trigger_pad_y_px: 6.0,
            trigger_pad_x_px: 10.0,
            trigger_radius: ComponentRadius::S2xl,
            content_radius: ComponentRadius::S2xl,
            viewport_radius: ComponentRadius::S2xl,
            ring_alpha: 0.05,
            ring_alpha_dark: 0.10,
            shadow: PopoverShadow::LG,
            link_radius: ComponentRadius::Lg,
            ..base
        },
        StyleId::Sera => NavigationMenuRecipe {
            trigger_pad_y_px: 10.0,
            trigger_pad_x_px: 18.0,
            trigger_radius: ComponentRadius::None,
            content_radius: ComponentRadius::None,
            viewport_radius: ComponentRadius::None,
            link_radius: ComponentRadius::None,
            ..base
        },
    }
}

const fn base_recipe() -> NavigationMenuRecipe {
    NavigationMenuRecipe {
        trigger_pad_y_px: 8.0,
        trigger_pad_x_px: 16.0,
        trigger_radius: ComponentRadius::Md,
        trigger_typography: TypeRecipe {
            size_px: 14.0,
            weight: FontWeight::Medium,
            uppercase: false,
            tracking_em: 0.0,
            line_height_px: 20.0,
        },
        content_pad_px: 8.0,
        content_radius: ComponentRadius::Md,
        viewport_radius: ComponentRadius::Lg,
        ring_alpha: 0.10,
        ring_alpha_dark: 0.10,
        shadow: PopoverShadow::MD,
        link_pad_px: 8.0,
        link_radius: ComponentRadius::Md,
        list_gap_px: 0.0,
    }
}

const fn text_xs(weight: FontWeight) -> TypeRecipe {
    TypeRecipe {
        size_px: 12.0,
        weight,
        uppercase: false,
        tracking_em: 0.0,
        line_height_px: 16.0,
    }
}

const fn text_xs_relaxed(weight: FontWeight) -> TypeRecipe {
    TypeRecipe {
        size_px: 12.0,
        weight,
        uppercase: false,
        tracking_em: 0.0,
        line_height_px: 19.5,
    }
}
