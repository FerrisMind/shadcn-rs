//! Shared geometry tokens for the shadcn-svelte-extras phone-input family.
//!
//! Phone Input itself is pack-invariant in the upstream extras markup (the
//! same Tailwind join / flag / popover utilities for Vega…Rhea). Pack-specific
//! look comes from the **shared** [`crate::style::StylePack`] on the app theme:
//! Button, Input, Command, and Popover resolve Rhea/Nova/… through
//! `theme.style.button_*`, input pack tables, etc. Pass that same theme into
//! every phone-input part — do not invent a separate Phone Input style table
//! that overrides composed control height, radius, or fill.

use crate::style::StyleId;

/// Flag chip size (`h-4 w-6` → 16×24) from the extras country trigger.
pub const FLAG_HEIGHT_PX: f32 = 16.0;
/// Flag chip width (`w-6` → 24).
pub const FLAG_WIDTH_PX: f32 = 24.0;
/// Country-selector chevron size (`h-4 w-4`).
pub const CHEVRON_SIZE_PX: f32 = 16.0;
/// Gap inside the country trigger (`gap-1`).
pub const TRIGGER_GAP_PX: f32 = 4.0;
/// Horizontal padding on the country trigger (`px-3`).
pub const TRIGGER_PAD_X_PX: f32 = 12.0;
/// Country popover width (`w-[300px]`).
pub const POPOVER_WIDTH_PX: f32 = 300.0;
/// Country list scroll height (`h-72` → 288 px).
pub const LIST_HEIGHT_PX: f32 = 288.0;
/// Disabled / soft opacity (`opacity-50`).
pub const DISABLED_OPACITY: f32 = 0.5;
/// Merged 1 px border collapse between trigger and input (button-group pattern).
pub const JOINT_OVERLAP_PX: f32 = 1.0;

/// Extras-only layout tokens for one phone-input instance.
///
/// Control height, corner radii, fills, and typography are **not** stored here
/// — backends read those from the composed Button / Input / Command / Popover
/// recipes on the active [`StyleId`].
///
/// ```rust
/// use shadcn_common::{StyleId, phone_input_recipe};
///
/// let recipe = phone_input_recipe(StyleId::Rhea);
/// assert_eq!(recipe.popover_width_px, 300.0);
/// assert_eq!(recipe.flag_width_px, 24.0);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PhoneInputRecipe {
    /// Flag chip height.
    pub flag_height_px: f32,
    /// Flag chip width.
    pub flag_width_px: f32,
    /// Chevron icon footprint.
    pub chevron_size_px: f32,
    /// Gap between flag and chevron in the trigger.
    pub trigger_gap_px: f32,
    /// Horizontal padding inside the country trigger (`px-3`).
    pub trigger_pad_x_px: f32,
    /// Country-selector popover width.
    pub popover_width_px: f32,
    /// Scrollable country list height.
    pub list_height_px: f32,
    /// Opacity applied when the extras layer dims chrome (not control fills).
    pub disabled_opacity: f32,
    /// Pixel overlap used to collapse the shared vertical border.
    pub joint_overlap_px: f32,
}

impl Default for PhoneInputRecipe {
    fn default() -> Self {
        phone_input_recipe(StyleId::Vega)
    }
}

/// Returns the pack-invariant Phone Input layout tokens.
///
/// `style` is accepted for API symmetry but is unused: selecting Rhea (or any
/// pack) on the theme still styles Phone Input children because those children
/// call their own pack-aware recipes with `theme.style_id()`.
#[must_use]
pub const fn phone_input_recipe(style: StyleId) -> PhoneInputRecipe {
    let _ = style;
    PhoneInputRecipe {
        flag_height_px: FLAG_HEIGHT_PX,
        flag_width_px: FLAG_WIDTH_PX,
        chevron_size_px: CHEVRON_SIZE_PX,
        trigger_gap_px: TRIGGER_GAP_PX,
        trigger_pad_x_px: TRIGGER_PAD_X_PX,
        popover_width_px: POPOVER_WIDTH_PX,
        list_height_px: LIST_HEIGHT_PX,
        disabled_opacity: DISABLED_OPACITY,
        joint_overlap_px: JOINT_OVERLAP_PX,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recipe_is_pack_invariant_layout_only() {
        for style in StyleId::ALL {
            let recipe = phone_input_recipe(style);
            assert_eq!(recipe.flag_height_px, 16.0);
            assert_eq!(recipe.flag_width_px, 24.0);
            assert_eq!(recipe.popover_width_px, 300.0);
            assert_eq!(recipe.list_height_px, 288.0);
            assert_eq!(recipe.disabled_opacity, 0.5);
            assert_eq!(recipe.joint_overlap_px, 1.0);
        }
    }
}
