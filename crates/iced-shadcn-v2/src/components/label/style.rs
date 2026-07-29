//! Style resolution for labels — thin iced adapter over shadcn-common recipes.

use crate::iced_compat::Color;
use shadcn_common::{LabelContext, LabelRecipe};

use crate::theme::Theme;

/// Sidecar footprint for icons beside labels (common SVG size-4).
pub(super) const SIDECAR_PX: f32 = 16.0;

pub(super) fn resolve_recipe(theme: &Theme, context: LabelContext) -> LabelRecipe {
    theme.style.label(context)
}

pub(super) fn resolve_color(theme: &Theme, color: Option<Color>, disabled: bool) -> Color {
    let recipe = theme.style.label(LabelContext::Field);
    let base = color.unwrap_or(theme.palette.foreground);
    if disabled {
        Color {
            a: base.a * recipe.disabled_opacity,
            ..base
        }
    } else {
        base
    }
}
