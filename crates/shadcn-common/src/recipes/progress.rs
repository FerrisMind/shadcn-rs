//! Progress geometry recipes from `.cn-progress` across style packs.

use crate::style::StyleId;

use super::ComponentRadius;

/// Geometry tokens for the default [`iced-shadcn-v2`](https://docs.rs/iced-shadcn-v2)
/// progress bar treatment of a style pack.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ProgressRecipe {
    /// Track and indicator height in logical pixels.
    pub height_px: f32,
    /// Default track radius from the style pack's Progress CSS.
    pub default_radius: ComponentRadius,
}

/// Resolves `.cn-progress` tokens for `style`.
pub const fn progress_recipe(style: StyleId) -> ProgressRecipe {
    match style {
        StyleId::Vega => ProgressRecipe {
            height_px: 6.0,
            default_radius: ComponentRadius::Full,
        },
        StyleId::Nova => ProgressRecipe {
            height_px: 4.0,
            default_radius: ComponentRadius::Full,
        },
        StyleId::Maia => ProgressRecipe {
            height_px: 12.0,
            default_radius: ComponentRadius::S4xl, // rounded-4xl
        },
        StyleId::Lyra => ProgressRecipe {
            height_px: 4.0,
            default_radius: ComponentRadius::None,
        },
        StyleId::Mira => ProgressRecipe {
            height_px: 4.0,
            default_radius: ComponentRadius::Md,
        },
        StyleId::Luma => ProgressRecipe {
            height_px: 12.0,
            default_radius: ComponentRadius::Full,
        },
        StyleId::Sera => ProgressRecipe {
            height_px: 2.0,
            default_radius: ComponentRadius::None,
        },
        StyleId::Rhea => ProgressRecipe {
            height_px: 8.0,
            default_radius: ComponentRadius::S2xl,
        },
    }
}
