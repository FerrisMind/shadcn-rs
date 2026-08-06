//! Checkbox recipes from the shadcn-svelte `.cn-checkbox` component.
//!
//! Corner radii are absolute pixels from `style-*.css` (`rounded-[4px]` /
//! `rounded-[5px]` / `rounded-[6px]` / `rounded-none`), shared by iced and egui.

use crate::style::StyleId;

/// Geometry recipe for a checkbox track.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CheckboxRecipe {
    /// Track corner radius in px (style-locked; does not scale with size).
    pub radius_px: f32,
}

/// Resolves the checkbox track tokens for `style`.
pub const fn checkbox_recipe(style: StyleId) -> CheckboxRecipe {
    CheckboxRecipe {
        radius_px: match style {
            // rounded-[4px]
            StyleId::Vega | StyleId::Nova | StyleId::Mira => 4.0,
            // rounded-[6px]
            StyleId::Maia => 6.0,
            // rounded-[5px]
            StyleId::Luma | StyleId::Rhea => 5.0,
            // rounded-none
            StyleId::Lyra | StyleId::Sera => 0.0,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lyra_and_sera_are_sharp() {
        assert_eq!(checkbox_recipe(StyleId::Lyra).radius_px, 0.0);
        assert_eq!(checkbox_recipe(StyleId::Sera).radius_px, 0.0);
    }

    #[test]
    fn maia_is_six_px() {
        assert_eq!(checkbox_recipe(StyleId::Maia).radius_px, 6.0);
    }
}
