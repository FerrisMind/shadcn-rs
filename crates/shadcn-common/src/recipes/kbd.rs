//! Kbd recipes from `.cn-kbd` across style packs.

use crate::style::StyleId;

use super::{ComponentRadius, FontWeight, TypeRecipe};

/// Geometry + typography recipe for `.cn-kbd`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KbdRecipe {
    pub height_px: f32,
    pub min_width_px: f32,
    pub pad_x_px: f32,
    pub gap_px: f32,
    pub icon_px: f32,
    pub typography: TypeRecipe,
    pub default_radius: ComponentRadius,
}

/// Resolves `.cn-kbd` tokens for `style`.
pub const fn kbd_recipe(style: StyleId) -> KbdRecipe {
    match style {
        StyleId::Sera => KbdRecipe {
            height_px: 22.0,
            min_width_px: 22.0,
            pad_x_px: 6.0,
            gap_px: 4.0,
            icon_px: 12.0,
            typography: medium_xs(),
            default_radius: ComponentRadius::None,
        },
        StyleId::Luma => KbdRecipe {
            height_px: 22.0,
            min_width_px: 22.0,
            pad_x_px: 6.0,
            gap_px: 4.0,
            icon_px: 12.0,
            typography: medium_xs(),
            default_radius: ComponentRadius::Lg,
        },
        StyleId::Mira => KbdRecipe {
            height_px: 20.0,
            min_width_px: 20.0,
            pad_x_px: 4.0,
            gap_px: 4.0,
            icon_px: 12.0,
            typography: TypeRecipe {
                size_px: 10.0,
                weight: FontWeight::Medium,
                uppercase: false,
                tracking_em: 0.0,
                line_height_px: 12.5,
            },
            default_radius: ComponentRadius::Sm,
        },
        StyleId::Lyra => KbdRecipe {
            height_px: 20.0,
            min_width_px: 20.0,
            pad_x_px: 4.0,
            gap_px: 4.0,
            icon_px: 12.0,
            typography: medium_xs(),
            default_radius: ComponentRadius::None,
        },
        StyleId::Rhea => KbdRecipe {
            height_px: 20.0,
            min_width_px: 20.0,
            pad_x_px: 4.0,
            gap_px: 4.0,
            icon_px: 12.0,
            typography: medium_xs(),
            default_radius: ComponentRadius::Lg,
        },
        StyleId::Vega | StyleId::Nova | StyleId::Maia => KbdRecipe {
            height_px: 20.0,
            min_width_px: 20.0,
            pad_x_px: 4.0,
            gap_px: 4.0,
            icon_px: 12.0,
            typography: medium_xs(),
            default_radius: ComponentRadius::Sm,
        },
    }
}

const fn medium_xs() -> TypeRecipe {
    TypeRecipe {
        size_px: 12.0,
        weight: FontWeight::Medium,
        uppercase: false,
        tracking_em: 0.0,
        line_height_px: 16.0,
    }
}
