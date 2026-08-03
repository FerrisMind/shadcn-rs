//! Meter geometry from shadcn-svelte-extras `Meter`.
//!
//! Meter itself is pack-invariant in the upstream extras markup (`h-2`,
//! `rounded-full`, `bg-(--meter-background)/20` — no per-pack `.cn-meter`
//! table). There is no Meter style variant to pick — same rule as Form:
//! choosing a pack on the app [`crate::style::StylePack`] / theme means every
//! **theme-driven** surface around and inside Meter resolves that pack
//! (Button recipes, fonts, radius slots) through `theme.style_id()` /
//! `theme.style.button_*`, while fill colors come from the shared Theme
//! palette (Base / Accent / Mode). Pass that same theme into every meter;
//! do not invent a separate Meter style-pack table.

use crate::style::StyleId;

use super::ComponentRadius;

/// Default track thickness (`h-2` → 8 px).
pub const HEIGHT_PX: f32 = 8.0;
/// Track fill uses the indicator color at this alpha (`/20`).
pub const TRACK_ALPHA: f32 = 0.20;
/// Default determinate transition (`transition-[color,transform]`, 150 ms).
pub const TRANSITION_MS: u32 = 150;
/// Warning band used by the extras Tokens demo (`> LIMIT * 0.75`).
pub const WARNING_RATIO: f32 = 0.75;

/// Geometry tokens for one meter instance.
///
/// These match the shared extras markup. They intentionally do **not** branch
/// on [`StyleId`]. Style packs affect Meter only via theme palette / accents
/// on the same [`crate::style::StylePack`].
///
/// ```rust
/// use shadcn_common::{StyleId, meter_recipe};
///
/// let recipe = meter_recipe(StyleId::Vega);
/// assert_eq!(recipe.height_px, 8.0);
/// assert_eq!(recipe.track_alpha, 0.20);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MeterRecipe {
    /// Track and indicator height in logical pixels (`h-2`).
    pub height_px: f32,
    /// Default track radius (`rounded-full`).
    pub default_radius: ComponentRadius,
    /// Alpha applied to the indicator color for the track backdrop.
    pub track_alpha: f32,
    /// Default value-transition duration in milliseconds.
    pub transition_ms: u32,
    /// Default warning threshold as a fraction of the `[min, max]` span.
    pub warning_ratio: f32,
}

impl Default for MeterRecipe {
    fn default() -> Self {
        meter_recipe(StyleId::Vega)
    }
}

/// Returns the pack-invariant Meter geometry tokens.
///
/// `style` is accepted for API symmetry but is unused: selecting Rhea (or any
/// pack) on the theme still styles Meter fills because indicator / track
/// colors come from `theme.palette` / accents resolved with that pack.
#[must_use]
pub const fn meter_recipe(style: StyleId) -> MeterRecipe {
    let _ = style;
    MeterRecipe {
        height_px: HEIGHT_PX,
        default_radius: ComponentRadius::Full,
        track_alpha: TRACK_ALPHA,
        transition_ms: TRANSITION_MS,
        warning_ratio: WARNING_RATIO,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recipe_matches_extras_tailwind() {
        for style in StyleId::ALL {
            let recipe = meter_recipe(style);
            assert_eq!(recipe.height_px, 8.0);
            assert_eq!(recipe.track_alpha, 0.20);
            assert_eq!(recipe.transition_ms, 150);
            assert_eq!(recipe.warning_ratio, 0.75);
            assert_eq!(recipe.default_radius, ComponentRadius::Full);
        }
    }
}
