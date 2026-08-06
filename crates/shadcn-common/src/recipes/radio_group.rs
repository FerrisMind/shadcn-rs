//! Radio-group recipes from `.cn-radio-group*` across style packs.
//!
//! shadcn-svelte exposes a single radio footprint per style pack, so the table
//! below carries one indicator diameter (`size-4` = `16px`, Sera's `size-4.5`
//! = `18px`) instead of a `sm` / `default` pair. Colors stay backend-agnostic:
//! the recipe only names the semantic slot and its opacity, and each GUI
//! backend resolves it against its own palette.

use crate::style::StyleId;

use super::ComponentRadius;

/// Surface treatment of an unchecked radio indicator.
///
/// ```rust
/// use shadcn_common::{RadioSurface, StyleId, radio_group_recipe};
///
/// assert_eq!(
///     radio_group_recipe(StyleId::Vega).unchecked_surface,
///     RadioSurface::Outline,
/// );
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum RadioSurface {
    /// `border-input` over the page background (`dark:bg-input/30`).
    #[default]
    Outline,
    /// `bg-input/90 border-transparent` — the fill carries the whole shape.
    Filled,
    /// `border-input bg-transparent` in both modes.
    Transparent,
}

/// Fill treatment of a checked radio indicator.
///
/// ```rust
/// use shadcn_common::{RadioCheckedFill, StyleId, radio_group_recipe};
///
/// assert_eq!(
///     radio_group_recipe(StyleId::Sera).checked_fill,
///     RadioCheckedFill::Foreground,
/// );
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum RadioCheckedFill {
    /// `data-checked:bg-primary data-checked:border-primary` with a
    /// `bg-primary-foreground` dot.
    #[default]
    Primary,
    /// `data-checked:border-foreground` with no fill and a `bg-foreground` dot.
    Foreground,
}

/// Geometry and semantic tokens of `.cn-radio-group*` for one style pack.
///
/// The dot has two diameters because Luma and Rhea grow it in dark mode
/// (`size-2 dark:size-2.5`); every other pack repeats the same value.
///
/// ```rust
/// use shadcn_common::{StyleId, radio_group_recipe};
///
/// let vega = radio_group_recipe(StyleId::Vega);
/// assert_eq!(vega.indicator_px, 16.0);
/// assert_eq!(vega.gap_px, 12.0);
/// // The dot always fits inside the indicator, borders included.
/// assert!(vega.dot_px + vega.border_width_px * 2.0 < vega.indicator_px);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RadioGroupRecipe {
    /// Indicator diameter in logical pixels (`size-4` → 16).
    pub indicator_px: f32,
    /// Indicator corner radius (`rounded-full` for every pack).
    pub radius: ComponentRadius,
    /// Dot diameter in light mode (`size-2` → 8).
    pub dot_px: f32,
    /// Dot diameter in dark mode (`dark:size-2.5` → 10).
    pub dark_dot_px: f32,
    /// Indicator border width in logical pixels.
    pub border_width_px: f32,
    /// Surface treatment while unchecked.
    pub unchecked_surface: RadioSurface,
    /// Opacity of the unchecked fill in light mode (`0` = no fill).
    pub unchecked_opacity: f32,
    /// Opacity of the unchecked fill in dark mode (`dark:bg-input/30` → `0.3`).
    pub dark_unchecked_opacity: f32,
    /// Fill treatment while checked.
    pub checked_fill: RadioCheckedFill,
    /// `focus-visible:ring-*` width in logical pixels.
    pub ring_width_px: f32,
    /// Alpha of the ring color (`ring-ring/50` → `0.5`).
    pub ring_opacity: f32,
    /// `.cn-radio-group` gap between items (`gap-3` → 12).
    pub gap_px: f32,
    /// Gap between one indicator and its label (`space-x-2` → 8).
    pub label_gap_px: f32,
    /// Opacity of a disabled item (`disabled:opacity-50`).
    pub disabled_opacity: f32,
}

/// `space-x-2` between indicator and label is shared by every pack.
const LABEL_GAP_PX: f32 = 8.0;
/// `disabled:opacity-50` is shared by every pack.
const DISABLED_OPACITY: f32 = 0.5;

/// Resolves `.cn-radio-group*` tokens for `style`.
///
/// ```rust
/// use shadcn_common::{RadioSurface, StyleId, radio_group_recipe};
///
/// // Sera grows the indicator and drops the fill entirely.
/// let sera = radio_group_recipe(StyleId::Sera);
/// assert_eq!(sera.indicator_px, 18.0);
/// assert_eq!(sera.unchecked_surface, RadioSurface::Transparent);
///
/// // Nova tightens the gap between items.
/// assert_eq!(radio_group_recipe(StyleId::Nova).gap_px, 8.0);
/// ```
pub const fn radio_group_recipe(style: StyleId) -> RadioGroupRecipe {
    match style {
        // `border-input dark:bg-input/30 size-4 rounded-full
        // focus-visible:ring-3 ring-ring/50`, group `grid gap-3`.
        StyleId::Vega | StyleId::Maia | StyleId::Mira => RadioGroupRecipe {
            indicator_px: 16.0,
            radius: ComponentRadius::Full,
            dot_px: 8.0,
            dark_dot_px: 8.0,
            border_width_px: 1.0,
            unchecked_surface: RadioSurface::Outline,
            unchecked_opacity: 0.0,
            dark_unchecked_opacity: 0.3,
            checked_fill: RadioCheckedFill::Primary,
            ring_width_px: 3.0,
            ring_opacity: 0.5,
            gap_px: 12.0,
            label_gap_px: LABEL_GAP_PX,
            disabled_opacity: DISABLED_OPACITY,
        },
        // Same indicator as Vega, group `grid gap-2`.
        StyleId::Nova | StyleId::Lyra => RadioGroupRecipe {
            indicator_px: 16.0,
            radius: ComponentRadius::Full,
            dot_px: 8.0,
            dark_dot_px: 8.0,
            border_width_px: 1.0,
            unchecked_surface: RadioSurface::Outline,
            unchecked_opacity: 0.0,
            dark_unchecked_opacity: 0.3,
            checked_fill: RadioCheckedFill::Primary,
            ring_width_px: 3.0,
            ring_opacity: 0.5,
            gap_px: 8.0,
            label_gap_px: LABEL_GAP_PX,
            disabled_opacity: DISABLED_OPACITY,
        },
        // `bg-input/90 border-transparent rounded-full` + `dark:size-2.5` dot,
        // `focus-visible:ring-3 ring-ring/30`.
        StyleId::Luma => RadioGroupRecipe {
            indicator_px: 16.0,
            radius: ComponentRadius::Full,
            dot_px: 8.0,
            dark_dot_px: 10.0,
            border_width_px: 1.0,
            unchecked_surface: RadioSurface::Filled,
            unchecked_opacity: 0.9,
            dark_unchecked_opacity: 0.9,
            checked_fill: RadioCheckedFill::Primary,
            ring_width_px: 3.0,
            ring_opacity: 0.3,
            gap_px: 12.0,
            label_gap_px: LABEL_GAP_PX,
            disabled_opacity: DISABLED_OPACITY,
        },
        // `border-input bg-transparent size-4.5 data-checked:border-foreground`
        // with a `bg-foreground` dot, `focus-visible:ring-2 ring-ring/30`.
        StyleId::Sera => RadioGroupRecipe {
            indicator_px: 18.0,
            radius: ComponentRadius::Full,
            dot_px: 8.0,
            dark_dot_px: 8.0,
            border_width_px: 1.0,
            unchecked_surface: RadioSurface::Transparent,
            unchecked_opacity: 0.0,
            dark_unchecked_opacity: 0.0,
            checked_fill: RadioCheckedFill::Foreground,
            ring_width_px: 2.0,
            ring_opacity: 0.3,
            gap_px: 12.0,
            label_gap_px: LABEL_GAP_PX,
            disabled_opacity: DISABLED_OPACITY,
        },
        // `bg-input/90 border-transparent rounded-2xl` plus a `dark:size-2.5` dot.
        StyleId::Rhea => RadioGroupRecipe {
            indicator_px: 16.0,
            radius: ComponentRadius::S2xl,
            dot_px: 8.0,
            dark_dot_px: 10.0,
            border_width_px: 1.0,
            unchecked_surface: RadioSurface::Filled,
            unchecked_opacity: 0.9,
            dark_unchecked_opacity: 0.9,
            checked_fill: RadioCheckedFill::Primary,
            ring_width_px: 3.0,
            ring_opacity: 0.3,
            gap_px: 12.0,
            label_gap_px: LABEL_GAP_PX,
            disabled_opacity: DISABLED_OPACITY,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_pack_keeps_the_dot_inside_the_indicator() {
        for style in StyleId::ALL {
            let recipe = radio_group_recipe(style);
            let widest_dot = recipe.dot_px.max(recipe.dark_dot_px);

            assert!(
                widest_dot + recipe.border_width_px * 2.0 < recipe.indicator_px,
                "{style:?} dot {widest_dot} does not fit in {}",
                recipe.indicator_px,
            );
        }
    }

    #[test]
    fn every_pack_paints_a_visible_ring_and_gap() {
        for style in StyleId::ALL {
            let recipe = radio_group_recipe(style);

            assert!(recipe.ring_width_px > 0.0);
            assert!(recipe.ring_opacity > 0.0 && recipe.ring_opacity <= 1.0);
            assert!(recipe.gap_px > 0.0);
            assert!(recipe.label_gap_px > 0.0);
        }
    }

    #[test]
    fn only_filled_packs_carry_an_unchecked_fill() {
        for style in StyleId::ALL {
            let recipe = radio_group_recipe(style);
            let filled = recipe.unchecked_surface == RadioSurface::Filled;

            assert_eq!(recipe.unchecked_opacity > 0.0, filled, "{style:?}");
            assert!(recipe.disabled_opacity > 0.0 && recipe.disabled_opacity < 1.0);
        }
    }
}
