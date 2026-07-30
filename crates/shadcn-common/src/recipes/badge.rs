//! Badge recipes from `.cn-badge` across style packs.

use crate::style::StyleId;

use super::{ComponentRadius, FontWeight, TypeRecipe};

/// Geometry + typography recipe for `.cn-badge`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BadgeRecipe {
    /// Fixed height (`h-5` → 20). `None` for Sera (no fixed height).
    pub height_px: Option<f32>,
    pub typography: TypeRecipe,
    pub gap_px: f32,
    pub pad_x_px: f32,
    pub pad_x_icon_px: f32,
    pub icon_px: f32,
    pub default_radius: ComponentRadius,
}

/// Resolves `.cn-badge` tokens for `style`.
pub const fn badge_recipe(style: StyleId) -> BadgeRecipe {
    match style {
        StyleId::Sera => BadgeRecipe {
            height_px: None,
            typography: TypeRecipe {
                size_px: 10.0, // 0.625rem
                weight: FontWeight::Semibold,
                uppercase: true,
                tracking_em: 0.1,
                line_height_px: 10.0 * 1.25,
            },
            gap_px: 6.0, // gap-1.5
            pad_x_px: 0.0,
            pad_x_icon_px: 0.0,
            icon_px: 12.0,
            default_radius: ComponentRadius::None,
        },
        StyleId::Mira => BadgeRecipe {
            height_px: Some(20.0),
            typography: TypeRecipe {
                size_px: 10.0,
                weight: FontWeight::Medium,
                uppercase: false,
                tracking_em: 0.0,
                line_height_px: 10.0 * 1.25,
            },
            gap_px: 4.0,
            pad_x_px: 8.0,
            pad_x_icon_px: 6.0,
            icon_px: 10.0, // size-2.5
            default_radius: ComponentRadius::Full,
        },
        StyleId::Lyra => BadgeRecipe {
            height_px: Some(20.0),
            typography: TypeRecipe {
                size_px: 12.0,
                weight: FontWeight::Medium,
                uppercase: false,
                tracking_em: 0.0,
                line_height_px: 16.0,
            },
            gap_px: 4.0,
            pad_x_px: 8.0,
            pad_x_icon_px: 6.0,
            icon_px: 12.0,
            default_radius: ComponentRadius::None,
        },
        StyleId::Rhea => BadgeRecipe {
            height_px: Some(20.0),
            typography: TypeRecipe {
                size_px: 12.0,
                weight: FontWeight::Medium,
                uppercase: false,
                tracking_em: 0.0,
                line_height_px: 16.0,
            },
            gap_px: 4.0,
            pad_x_px: 8.0,
            pad_x_icon_px: 6.0,
            icon_px: 12.0,
            default_radius: ComponentRadius::S2xl, // rounded-2xl
        },
        StyleId::Luma => BadgeRecipe {
            height_px: Some(20.0),
            typography: TypeRecipe {
                size_px: 12.0,
                weight: FontWeight::Medium,
                uppercase: false,
                tracking_em: 0.0,
                line_height_px: 16.0,
            },
            gap_px: 4.0,
            pad_x_px: 8.0,
            pad_x_icon_px: 6.0,
            icon_px: 12.0,
            default_radius: ComponentRadius::S3xl, // rounded-3xl
        },
        StyleId::Vega | StyleId::Nova | StyleId::Maia => BadgeRecipe {
            height_px: Some(20.0),
            typography: TypeRecipe {
                size_px: 12.0,
                weight: FontWeight::Medium,
                uppercase: false,
                tracking_em: 0.0,
                line_height_px: 16.0,
            },
            gap_px: 4.0,
            pad_x_px: 8.0,
            pad_x_icon_px: 6.0,
            icon_px: 12.0,
            default_radius: ComponentRadius::S4xl, // rounded-4xl
        },
    }
}
