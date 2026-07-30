//! Skeleton recipes from `.cn-skeleton` across style packs.

use crate::style::StyleId;

use super::ComponentRadius;

/// Default corner radius for `.cn-skeleton`.
pub const fn skeleton_default_radius(style: StyleId) -> ComponentRadius {
    match style {
        StyleId::Lyra | StyleId::Sera => ComponentRadius::None,
        StyleId::Maia => ComponentRadius::Xl, // rounded-xl
        StyleId::Luma | StyleId::Rhea => ComponentRadius::S2xl, // rounded-2xl
        StyleId::Vega | StyleId::Nova | StyleId::Mira => ComponentRadius::Md,
    }
}

/// Thin wrapper kept for API symmetry with other recipes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SkeletonRecipe {
    pub default_radius: ComponentRadius,
}

/// Resolves `.cn-skeleton` tokens for `style`.
pub const fn skeleton_recipe(style: StyleId) -> SkeletonRecipe {
    SkeletonRecipe {
        default_radius: skeleton_default_radius(style),
    }
}
