//! Chart recipes across style packs.
//!
//! The web charts restyle little per pack — marks keep their demo geometry —
//! so the recipe carries the pack-dependent surfaces: the tooltip card
//! (`rounded-lg border px-2.5 py-1.5 text-xs` in the base packs) and the
//! default bar rounding used when a chart does not override it.

use crate::style::StyleId;

use super::{ComponentRadius, FontWeight, TypeRecipe};

/// Style-pack tokens for the chart component.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ChartRecipe {
    /// Default bar corner radius intent when the builder sets none.
    pub bar_radius: ComponentRadius,
    /// Tooltip surface corner radius (`rounded-lg`, squared/softened per pack).
    pub tooltip_radius: ComponentRadius,
    /// Tooltip horizontal padding (`px-2.5`).
    pub tooltip_pad_x_px: f32,
    /// Tooltip vertical padding (`py-1.5`).
    pub tooltip_pad_y_px: f32,
    /// Tick, legend, and tooltip typography (`text-xs`).
    pub typography: TypeRecipe,
}

/// Resolves chart tokens for `style`.
///
/// ```rust
/// use shadcn_common::{ComponentRadius, StyleId, chart_recipe};
///
/// assert_eq!(chart_recipe(StyleId::Vega).tooltip_radius, ComponentRadius::Lg);
/// assert_eq!(chart_recipe(StyleId::Lyra).bar_radius, ComponentRadius::None);
/// ```
pub const fn chart_recipe(style: StyleId) -> ChartRecipe {
    match style {
        // Squared packs: no rounding anywhere.
        StyleId::Lyra | StyleId::Sera => ChartRecipe {
            bar_radius: ComponentRadius::None,
            tooltip_radius: ComponentRadius::None,
            ..base_recipe()
        },
        // Softer packs round the tooltip one step further.
        StyleId::Luma | StyleId::Rhea => ChartRecipe {
            bar_radius: ComponentRadius::Md,
            tooltip_radius: ComponentRadius::Xl,
            ..base_recipe()
        },
        StyleId::Maia => ChartRecipe {
            bar_radius: ComponentRadius::Md,
            tooltip_radius: ComponentRadius::S2xl,
            ..base_recipe()
        },
        StyleId::Vega | StyleId::Nova | StyleId::Mira => base_recipe(),
    }
}

const fn base_recipe() -> ChartRecipe {
    ChartRecipe {
        bar_radius: ComponentRadius::Sm,
        tooltip_radius: ComponentRadius::Lg,
        tooltip_pad_x_px: 10.0,
        tooltip_pad_y_px: 6.0,
        typography: TypeRecipe {
            size_px: 12.0,
            weight: FontWeight::Normal,
            uppercase: false,
            tracking_em: 0.0,
            line_height_px: 16.0,
        },
    }
}
