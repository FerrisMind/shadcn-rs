//! Shared geometry and typography tokens for the shadcn-svelte form family.
//!
//! Form itself is pack-invariant in the upstream registry (`form.json` is the
//! same for Vega…Rhea). There is no Form style variant to pick — choosing a
//! pack on the app [`crate::style::StylePack`] / theme means every **composed**
//! control (Label, Button, Input, …) resolves that pack through
//! `theme.style.label(...)`, `theme.style.button_*`, `theme.style_id()`, etc.
//! Pass that same theme into every form part; do not invent a separate Form
//! style table.

use crate::style::StyleId;

use super::{FontWeight, TypeRecipe};

/// Backend-neutral form layout and text tokens.
///
/// These gaps/type sizes match the shared Form markup (`space-y-6`,
/// `text-sm`, …). They intentionally do **not** branch on [`StyleId`]. Style
/// packs affect Form only via composed component recipes on the same theme.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FormRecipe {
    /// Default gap between top-level form children (`space-y-6`).
    pub form_gap_px: f32,
    /// Gap between direct children of a form field (`space-y-2`).
    pub field_gap_px: f32,
    /// Gap between direct children of a fieldset (`space-y-2`).
    pub fieldset_gap_px: f32,
    /// Typography for muted descriptions (`text-sm`).
    pub description: TypeRecipe,
    /// Typography for validation errors (`text-sm font-medium`).
    pub error: TypeRecipe,
    /// Typography for compact legends (`text-sm leading-none font-medium`).
    pub legend: TypeRecipe,
}

impl Default for FormRecipe {
    fn default() -> Self {
        form_recipe(StyleId::Vega)
    }
}

/// Returns the pack-invariant Form layout/typography tokens.
///
/// `style` is accepted for API symmetry with other recipes but is unused:
/// selecting Rhea (or any pack) on the theme still styles Form children
/// because those children call their own pack-aware recipes with
/// `theme.style_id()`.
#[must_use]
pub const fn form_recipe(style: StyleId) -> FormRecipe {
    let _ = style;
    FormRecipe {
        form_gap_px: 24.0,
        field_gap_px: 8.0,
        fieldset_gap_px: 8.0,
        description: TypeRecipe {
            size_px: 14.0,
            weight: FontWeight::Normal,
            uppercase: false,
            tracking_em: 0.0,
            line_height_px: 20.0,
        },
        error: TypeRecipe {
            size_px: 14.0,
            weight: FontWeight::Medium,
            uppercase: false,
            tracking_em: 0.0,
            line_height_px: 20.0,
        },
        legend: TypeRecipe {
            size_px: 14.0,
            weight: FontWeight::Medium,
            uppercase: false,
            tracking_em: 0.0,
            line_height_px: 14.0,
        },
    }
}
