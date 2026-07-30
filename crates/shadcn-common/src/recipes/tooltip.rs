//! Tooltip recipes from `.cn-tooltip-content` / `.cn-tooltip-arrow` across
//! style packs.

use crate::style::StyleId;

use super::{ComponentRadius, FontWeight, TypeRecipe};

/// Duration of the tooltip open/close animation (`animate-in` default).
pub const TOOLTIP_ANIMATION_MS: u64 = 150;

/// Distance covered by the `slide-in-from-*-2` entrance animation.
pub const TOOLTIP_SLIDE_PX: f32 = 8.0;

/// Initial scale of the `zoom-in-95` entrance animation.
pub const TOOLTIP_ZOOM_FROM: f32 = 0.95;

/// Geometry + typography recipe for `.cn-tooltip-content` and its arrow.
///
/// The tooltip surface uses the swapped theme pair `bg-foreground` /
/// `text-background`; colors stay with the backend palettes, only the
/// geometry lives here.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TooltipRecipe {
    /// Horizontal content padding (`px-3`).
    pub pad_x_px: f32,
    /// Vertical content padding (`py-1.5`).
    pub pad_y_px: f32,
    /// Maximum content width (`max-w-xs`).
    pub max_width_px: f32,
    /// Edge length of the square diamond arrow (`size-2.5`).
    pub arrow_size_px: f32,
    /// Corner radius of the arrow (`rounded-[2px]` / `rounded-none`).
    pub arrow_radius_px: f32,
    /// Content typography (`text-xs`).
    pub typography: TypeRecipe,
    /// Content corner radius intent.
    pub radius: ComponentRadius,
}

/// Resolves `.cn-tooltip-content` tokens for `style`.
pub const fn tooltip_recipe(style: StyleId) -> TooltipRecipe {
    match style {
        StyleId::Lyra | StyleId::Sera => TooltipRecipe {
            arrow_radius_px: 0.0,
            radius: ComponentRadius::None,
            ..base_recipe()
        },
        StyleId::Luma | StyleId::Rhea => TooltipRecipe {
            radius: ComponentRadius::Xl, // rounded-xl
            ..base_recipe()
        },
        StyleId::Maia => TooltipRecipe {
            radius: ComponentRadius::S2xl, // rounded-2xl
            ..base_recipe()
        },
        StyleId::Vega | StyleId::Nova | StyleId::Mira => base_recipe(),
    }
}

const fn base_recipe() -> TooltipRecipe {
    TooltipRecipe {
        pad_x_px: 12.0,
        pad_y_px: 6.0,
        max_width_px: 320.0,
        arrow_size_px: 10.0,
        arrow_radius_px: 2.0,
        typography: TypeRecipe {
            size_px: 12.0,
            weight: FontWeight::Normal,
            uppercase: false,
            tracking_em: 0.0,
            line_height_px: 16.0,
        },
        radius: ComponentRadius::Md,
    }
}
