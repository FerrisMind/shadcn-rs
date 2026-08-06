//! Label recipes from `.cn-label` across style packs.

use crate::style::StyleId;

use super::{FontWeight, TypeRecipe};

/// Layout role of a label relative to its control.
///
/// iced/egui stand-in for Sera’s CSS `peer-data-[slot=checkbox|radio-group-item|switch]:*`
/// rules. Other style packs ignore the distinction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum LabelContext {
    /// Standalone / field label (`cn-label` defaults).
    #[default]
    Field,
    /// Label adjacent to a checkbox, radio, or switch control.
    AdjacentControl,
}

/// Full label recipe for one style pack + context.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LabelRecipe {
    pub typography: TypeRecipe,
    /// Flex gap between text and sidecar icons (`gap-2` → 8 px).
    pub gap_px: f32,
    /// Opacity when `group-data-[disabled=true]` / `peer-disabled`.
    pub disabled_opacity: f32,
}

/// Resolves `.cn-label` tokens for `style` + `context`.
pub const fn label_recipe(style: StyleId, context: LabelContext) -> LabelRecipe {
    LabelRecipe {
        typography: typography(style, context),
        gap_px: 8.0,
        disabled_opacity: 0.5,
    }
}

const fn typography(style: StyleId, context: LabelContext) -> TypeRecipe {
    match (style, context) {
        // Sera peer: `text-sm font-normal tracking-normal normal-case`
        // (`text-sm` line-height 1.25rem wins over base `leading-relaxed`).
        (StyleId::Sera, LabelContext::AdjacentControl) => TypeRecipe {
            size_px: 14.0,
            weight: FontWeight::Normal,
            uppercase: false,
            tracking_em: 0.0,
            line_height_px: 20.0,
        },
        // Sera field: `text-xs leading-relaxed font-semibold tracking-wide uppercase`.
        (StyleId::Sera, LabelContext::Field) => TypeRecipe {
            size_px: 12.0,
            weight: FontWeight::Semibold,
            uppercase: true,
            tracking_em: 0.025,
            line_height_px: 12.0 * 1.625,
        },
        // Lyra: `text-xs leading-none` (no font-medium).
        (StyleId::Lyra, _) => TypeRecipe {
            size_px: 12.0,
            weight: FontWeight::Normal,
            uppercase: false,
            tracking_em: 0.0,
            line_height_px: 12.0,
        },
        // Mira: `text-xs/relaxed leading-none font-medium` — `leading-none` wins.
        (StyleId::Mira, _) => TypeRecipe {
            size_px: 12.0,
            weight: FontWeight::Medium,
            uppercase: false,
            tracking_em: 0.0,
            line_height_px: 12.0,
        },
        // Vega / Nova / Maia / Luma / Rhea: `text-sm leading-none font-medium`.
        (StyleId::Vega | StyleId::Nova | StyleId::Maia | StyleId::Luma | StyleId::Rhea, _) => {
            TypeRecipe {
                size_px: 14.0,
                weight: FontWeight::Medium,
                uppercase: false,
                tracking_em: 0.0,
                line_height_px: 14.0,
            }
        }
    }
}
