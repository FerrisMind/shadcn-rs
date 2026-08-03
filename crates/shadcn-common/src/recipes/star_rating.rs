//! Star-rating geometry from shadcn-svelte-extras `StarRating` / `Star`.
//!
//! The extras component hard-codes Tailwind utilities (`size-5`, `gap-1`,
//! `rounded-md`, `ring-2`, `ring-offset-2`, `group-aria-disabled:opacity-50`)
//! rather than pack-specific `.cn-*` tables, so the recipe is intentionally
//! pack-invariant — [`StyleId`] is accepted for API symmetry with other
//! recipes.

use crate::style::StyleId;

use super::ComponentRadius;

/// Default star footprint (`size-5` → 20 px).
pub const STAR_SIZE_PX: f32 = 20.0;
/// Gap between stars (`gap-1` → 4 px).
pub const STAR_GAP_PX: f32 = 4.0;
/// Disabled / group-disabled opacity (`opacity-50`).
pub const DISABLED_OPACITY: f32 = 0.5;
/// Focus ring width (`focus-visible:ring-2`).
pub const RING_WIDTH_PX: f32 = 2.0;
/// Focus ring offset (`ring-offset-2`).
pub const RING_OFFSET_PX: f32 = 2.0;
/// Lucide viewBox size used when stroking the star path.
pub const STAR_VIEWBOX: f32 = 24.0;
/// Lucide default stroke width inside the 24×24 viewBox.
pub const STAR_STROKE_VIEWBOX: f32 = 2.0;

/// Geometry and interaction tokens for one star-rating instance.
///
/// ```rust
/// use shadcn_common::{StyleId, star_rating_recipe};
///
/// let recipe = star_rating_recipe(StyleId::Vega);
/// assert_eq!(recipe.star_size_px, 20.0);
/// assert_eq!(recipe.gap_px, 4.0);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StarRatingRecipe {
    /// Default star edge length in logical pixels (`size-5`).
    pub star_size_px: f32,
    /// Gap between adjacent stars (`gap-1`).
    pub gap_px: f32,
    /// Corner radius of each star hit-target (`rounded-md`).
    pub item_radius: ComponentRadius,
    /// Focus-visible ring width.
    pub ring_width_px: f32,
    /// Gap between the star and its focus ring.
    pub ring_offset_px: f32,
    /// Opacity applied when the group is disabled.
    pub disabled_opacity: f32,
}

impl Default for StarRatingRecipe {
    fn default() -> Self {
        star_rating_recipe(StyleId::Vega)
    }
}

/// Resolves star-rating tokens.
///
/// `style` is accepted for API symmetry but unused — the extras markup is the
/// same across Vega…Rhea packs.
#[must_use]
pub const fn star_rating_recipe(style: StyleId) -> StarRatingRecipe {
    let _ = style;
    StarRatingRecipe {
        star_size_px: STAR_SIZE_PX,
        gap_px: STAR_GAP_PX,
        item_radius: ComponentRadius::Md,
        ring_width_px: RING_WIDTH_PX,
        ring_offset_px: RING_OFFSET_PX,
        disabled_opacity: DISABLED_OPACITY,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recipe_matches_extras_tailwind() {
        for style in StyleId::ALL {
            let recipe = star_rating_recipe(style);
            assert_eq!(recipe.star_size_px, 20.0);
            assert_eq!(recipe.gap_px, 4.0);
            assert_eq!(recipe.ring_width_px, 2.0);
            assert_eq!(recipe.ring_offset_px, 2.0);
            assert_eq!(recipe.disabled_opacity, 0.5);
            assert_eq!(recipe.item_radius, ComponentRadius::Md);
        }
    }
}
