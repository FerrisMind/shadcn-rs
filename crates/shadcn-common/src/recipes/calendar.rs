//! Calendar recipes from `.cn-calendar` across style packs.

use crate::style::StyleId;

use super::ComponentRadius;

/// Geometry recipe for `.cn-calendar`.
///
/// Captures the per-style CSS custom properties: `--cell-size`,
/// `--cell-radius`, and the root padding (`p-3` / `p-2`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CalendarRecipe {
    /// Day/nav cell footprint (`--cell-size`).
    pub cell_size_px: f32,
    /// Root padding (`p-3` → 12, `p-2` → 8).
    pub pad_px: f32,
    /// Day cell corner radius (`--cell-radius`).
    pub cell_radius: ComponentRadius,
}

/// Resolves `.cn-calendar` tokens for `style`.
pub const fn calendar_recipe(style: StyleId) -> CalendarRecipe {
    match style {
        StyleId::Vega => CalendarRecipe {
            cell_size_px: 32.0,
            pad_px: 12.0,
            cell_radius: ComponentRadius::Md,
        },
        StyleId::Nova => CalendarRecipe {
            cell_size_px: 28.0,
            pad_px: 8.0,
            cell_radius: ComponentRadius::Md,
        },
        StyleId::Maia | StyleId::Luma => CalendarRecipe {
            cell_size_px: 32.0,
            pad_px: 12.0,
            cell_radius: ComponentRadius::S4xl,
        },
        StyleId::Lyra => CalendarRecipe {
            cell_size_px: 28.0,
            pad_px: 8.0,
            cell_radius: ComponentRadius::None,
        },
        StyleId::Mira => CalendarRecipe {
            cell_size_px: 24.0,
            pad_px: 12.0,
            cell_radius: ComponentRadius::Md,
        },
        StyleId::Sera => CalendarRecipe {
            cell_size_px: 32.0,
            pad_px: 12.0,
            cell_radius: ComponentRadius::None,
        },
        StyleId::Rhea => CalendarRecipe {
            cell_size_px: 32.0,
            pad_px: 12.0,
            cell_radius: ComponentRadius::S2xl,
        },
    }
}
