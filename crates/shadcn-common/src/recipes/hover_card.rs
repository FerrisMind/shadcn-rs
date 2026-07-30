//! Hover-card recipes from `.cn-hover-card-content` across style packs.
//!
//! The shadcn-svelte hover card is the bits-ui link preview: a popover-like
//! surface that opens on hover after `openDelay` and closes after
//! `closeDelay` once the pointer leaves both the trigger and the content.

use crate::style::StyleId;

use super::popover::PopoverShadow;
use super::{ComponentRadius, FontWeight, TypeRecipe};

/// Default delay before the card opens (`openDelay`, bits-ui link preview).
pub const HOVER_CARD_OPEN_DELAY_MS: u64 = 700;

/// Default delay before the card closes (`closeDelay`, bits-ui link
/// preview).
pub const HOVER_CARD_CLOSE_DELAY_MS: u64 = 300;

/// Duration of the hover-card open/close animation (`duration-100`).
pub const HOVER_CARD_ANIMATION_MS: u64 = 100;

/// Distance covered by the `slide-in-from-*-2` entrance animation.
pub const HOVER_CARD_SLIDE_PX: f32 = 8.0;

/// Initial scale of the `zoom-in-95` entrance animation.
pub const HOVER_CARD_ZOOM_FROM: f32 = 0.95;

/// Geometry + typography recipe for `.cn-hover-card-content`.
///
/// The surface uses the `bg-popover` / `text-popover-foreground` pair with
/// a `ring-1 ring-foreground/N` hairline; colors stay with the backend
/// palettes, only geometry and alphas live here.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HoverCardRecipe {
    /// Uniform content padding (`p-4` / `p-2.5`).
    pub pad_px: f32,
    /// Default content width (`w-64` / `w-72`).
    pub width_px: f32,
    /// Surface corner radius intent (`rounded-lg` / `rounded-none` / …).
    pub radius: ComponentRadius,
    /// `ring-foreground/N` alpha in light mode.
    pub ring_alpha: f32,
    /// `ring-foreground/N` alpha in dark mode (`dark:ring-foreground/N`).
    pub ring_alpha_dark: f32,
    /// Surface drop shadow.
    pub shadow: PopoverShadow,
    /// Body typography (`text-sm` / `text-xs/relaxed`).
    pub typography: TypeRecipe,
}

/// Resolves `.cn-hover-card-content` tokens for `style`.
pub const fn hover_card_recipe(style: StyleId) -> HoverCardRecipe {
    let base = base_recipe();

    match style {
        StyleId::Vega => base,
        StyleId::Nova => HoverCardRecipe {
            pad_px: 10.0,
            ..base
        },
        StyleId::Mira => HoverCardRecipe {
            pad_px: 10.0,
            width_px: 288.0,
            typography: text_xs_relaxed(),
            ..base
        },
        StyleId::Lyra => HoverCardRecipe {
            pad_px: 10.0,
            radius: ComponentRadius::None,
            typography: text_xs_relaxed(),
            ..base
        },
        StyleId::Maia => HoverCardRecipe {
            width_px: 288.0,
            radius: ComponentRadius::S2xl, // rounded-2xl
            ring_alpha: 0.05,
            ring_alpha_dark: 0.05,
            shadow: PopoverShadow::XXL,
            ..base
        },
        StyleId::Luma | StyleId::Rhea => HoverCardRecipe {
            width_px: 288.0,
            radius: ComponentRadius::S3xl,
            ring_alpha: 0.05,
            ring_alpha_dark: 0.10,
            shadow: PopoverShadow::LG,
            ..base
        },
        StyleId::Sera => HoverCardRecipe {
            width_px: 288.0,
            radius: ComponentRadius::None,
            ..base
        },
    }
}

const fn base_recipe() -> HoverCardRecipe {
    HoverCardRecipe {
        pad_px: 16.0,
        width_px: 256.0,
        radius: ComponentRadius::Lg,
        ring_alpha: 0.10,
        ring_alpha_dark: 0.10,
        shadow: PopoverShadow::MD,
        typography: TypeRecipe {
            size_px: 14.0,
            weight: FontWeight::Normal,
            uppercase: false,
            tracking_em: 0.0,
            line_height_px: 20.0,
        },
    }
}

const fn text_xs_relaxed() -> TypeRecipe {
    TypeRecipe {
        size_px: 12.0,
        weight: FontWeight::Normal,
        uppercase: false,
        tracking_em: 0.0,
        line_height_px: 19.5,
    }
}
