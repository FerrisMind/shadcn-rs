//! Toggle recipes from `.cn-toggle` + `.cn-toggle-variant-*` / `.cn-toggle-size-*`.
//!
//! shadcn-svelte's toggle is a stateful button with two variants (`default` /
//! `outline`) and three footprints (`sm` / `default` / `lg`). Every style pack
//! redefines typography, radius, heights, and horizontal padding; the tables
//! below are transcribed from the Tailwind utilities of each pack
//! (`1rem` = `16px`, `h-9` = `36px`, `px-2.5` = `10px`, …).

use crate::style::StyleId;

use super::{ComponentRadius, ControlSize, FontWeight, TypeRecipe};

/// Size-independent `.cn-toggle` tokens of a style pack.
///
/// ```rust
/// use shadcn_common::{StyleId, toggle_recipe};
///
/// let sera = toggle_recipe(StyleId::Sera);
/// assert!(sera.typography.uppercase);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ToggleRecipe {
    /// Label typography (`text-sm font-medium`, Sera goes uppercase).
    pub typography: TypeRecipe,
    /// Corner radius from the pack's Toggle CSS.
    pub default_radius: ComponentRadius,
    /// Whether the `outline` variant carries a `shadow-xs` (Vega only).
    pub outline_shadow: bool,
    /// Gap between an icon slot and the label (`gap-1`, Sera `gap-1.5`).
    pub gap_px: f32,
}

/// Geometry of one toggle footprint (`sm` / `default` / `lg`) under a style pack.
///
/// ```rust
/// use shadcn_common::{ControlSize, StyleId, toggle_size};
///
/// let vega = toggle_size(StyleId::Vega, ControlSize::Md);
/// assert_eq!(vega.height_px, 36.0);
/// // Icon-only toggles are square: `min-w-*` matches `h-*`.
/// assert_eq!(vega.min_width_px, vega.height_px);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ToggleSizeRecipe {
    /// Control height in logical pixels.
    pub height_px: f32,
    /// Minimum control width (`min-w-*`, the square icon footprint).
    pub min_width_px: f32,
    /// Default horizontal padding (`px-*`).
    pub pad_x_px: f32,
    /// Horizontal padding on a side that holds an icon slot
    /// (`has-data-[icon=inline-start]:pl-*` / `…inline-end]:pr-*`; the packs
    /// use symmetric values, so one token covers both sides).
    pub pad_x_icon_px: f32,
    /// Label text size for this slot (may override the base typography).
    pub text_size_px: f32,
    /// Default SVG / icon footprint (`[&_svg]:size-*`).
    pub icon_px: f32,
}

/// Resolves size-independent `.cn-toggle` tokens for `style`.
///
/// ```rust
/// use shadcn_common::{ComponentRadius, StyleId, toggle_recipe};
///
/// assert_eq!(
///     toggle_recipe(StyleId::Lyra).default_radius,
///     ComponentRadius::None,
/// );
/// ```
pub const fn toggle_recipe(style: StyleId) -> ToggleRecipe {
    match style {
        // `rounded-md text-sm font-medium`, outline adds `shadow-xs`.
        StyleId::Vega => ToggleRecipe {
            typography: text_sm_medium(),
            default_radius: ComponentRadius::Md,
            outline_shadow: true,
            gap_px: 4.0,
        },
        // `rounded-lg text-sm font-medium`.
        StyleId::Nova => ToggleRecipe {
            typography: text_sm_medium(),
            default_radius: ComponentRadius::Lg,
            outline_shadow: false,
            gap_px: 4.0,
        },
        // `rounded-4xl text-sm font-medium`.
        StyleId::Maia => ToggleRecipe {
            typography: text_sm_medium(),
            default_radius: ComponentRadius::S4xl,
            outline_shadow: false,
            gap_px: 4.0,
        },
        // `rounded-none text-xs font-medium`.
        StyleId::Lyra => ToggleRecipe {
            typography: text_xs_medium(),
            default_radius: ComponentRadius::None,
            outline_shadow: false,
            gap_px: 4.0,
        },
        // `rounded-md text-xs font-medium`.
        StyleId::Mira => ToggleRecipe {
            typography: text_xs_medium(),
            default_radius: ComponentRadius::Md,
            outline_shadow: false,
            gap_px: 4.0,
        },
        // `rounded-3xl text-sm font-medium`.
        // Soft panel: `rounded-3xl text-sm font-medium`.
        StyleId::Luma => ToggleRecipe {
            typography: text_sm_medium(),
            default_radius: ComponentRadius::S3xl,
            outline_shadow: false,
            gap_px: 4.0,
        },
        // `rounded-none text-xs font-semibold tracking-widest uppercase`.
        StyleId::Sera => ToggleRecipe {
            typography: TypeRecipe {
                size_px: 12.0,
                weight: FontWeight::Semibold,
                uppercase: true,
                tracking_em: 0.1,
                line_height_px: 16.0,
            },
            default_radius: ComponentRadius::None,
            outline_shadow: false,
            gap_px: 6.0,
        },
        // `rounded-2xl text-sm font-medium`.
        StyleId::Rhea => ToggleRecipe {
            typography: text_sm_medium(),
            default_radius: ComponentRadius::S2xl,
            outline_shadow: false,
            gap_px: 4.0,
        },
    }
}

/// Resolves `.cn-toggle-size-*` geometry for `style` + `size`.
///
/// shadcn-svelte ships `sm` / `default` / `lg` footprints:
/// [`ControlSize::Xs`] and [`ControlSize::Sm`] resolve to `sm`,
/// [`ControlSize::Md`] to `default`, and [`ControlSize::Lg`] to `lg`.
///
/// ```rust
/// use shadcn_common::{ControlSize, StyleId, toggle_size};
///
/// let sm = toggle_size(StyleId::Vega, ControlSize::Sm);
/// let lg = toggle_size(StyleId::Vega, ControlSize::Lg);
/// assert!(sm.height_px < lg.height_px);
/// assert_eq!(sm, toggle_size(StyleId::Vega, ControlSize::Xs));
/// ```
pub const fn toggle_size(style: StyleId, size: ControlSize) -> ToggleSizeRecipe {
    match style {
        // `h-9 / h-8 / h-10`, `px-2.5`, icon sides `pl-2` / `pr-2` (`sm`: 1.5).
        StyleId::Vega => match size {
            ControlSize::Xs | ControlSize::Sm => slot(32.0, 10.0, 6.0, 14.0, 16.0),
            ControlSize::Md => slot(36.0, 10.0, 8.0, 14.0, 16.0),
            ControlSize::Lg => slot(40.0, 10.0, 8.0, 14.0, 16.0),
        },
        // `h-8 / h-7 / h-9`; `sm` drops to `text-[0.8rem]` + `size-3.5` icons.
        StyleId::Nova => match size {
            ControlSize::Xs | ControlSize::Sm => slot(28.0, 10.0, 6.0, 12.8, 14.0),
            ControlSize::Md => slot(32.0, 10.0, 8.0, 14.0, 16.0),
            ControlSize::Lg => slot(36.0, 10.0, 8.0, 14.0, 16.0),
        },
        // `h-9 / h-8 / h-10`, generous `px-3` / `px-4`.
        StyleId::Maia => match size {
            ControlSize::Xs | ControlSize::Sm => slot(32.0, 12.0, 8.0, 14.0, 16.0),
            ControlSize::Md => slot(36.0, 12.0, 10.0, 14.0, 16.0),
            ControlSize::Lg => slot(40.0, 16.0, 12.0, 14.0, 16.0),
        },
        // `h-8 / h-7 / h-9`, `px-2.5`.
        StyleId::Lyra => match size {
            ControlSize::Xs | ControlSize::Sm => slot(28.0, 10.0, 6.0, 12.0, 16.0),
            ControlSize::Md => slot(32.0, 10.0, 8.0, 12.0, 16.0),
            ControlSize::Lg => slot(36.0, 10.0, 8.0, 12.0, 16.0),
        },
        // `h-7 / h-6 / h-8`; `sm` drops to `text-[0.625rem]` + `size-3` icons.
        StyleId::Mira => match size {
            ControlSize::Xs | ControlSize::Sm => slot(24.0, 8.0, 6.0, 10.0, 12.0),
            ControlSize::Md => slot(28.0, 8.0, 6.0, 12.0, 16.0),
            ControlSize::Lg => slot(32.0, 10.0, 8.0, 12.0, 16.0),
        },
        // `h-9 / h-8 / h-10`, generous `px-3` / `px-4`.
        StyleId::Luma => match size {
            ControlSize::Xs | ControlSize::Sm => slot(32.0, 12.0, 8.0, 14.0, 16.0),
            ControlSize::Md => slot(36.0, 12.0, 10.0, 14.0, 16.0),
            ControlSize::Lg => slot(40.0, 16.0, 12.0, 14.0, 16.0),
        },
        // `h-10 / h-9 / h-11`, editorial `px-6` / `px-4` / `px-8`, `size-3.5` icons.
        StyleId::Sera => match size {
            ControlSize::Xs | ControlSize::Sm => slot(36.0, 16.0, 12.0, 12.0, 14.0),
            ControlSize::Md => slot(40.0, 24.0, 16.0, 12.0, 14.0),
            ControlSize::Lg => slot(44.0, 32.0, 20.0, 12.0, 14.0),
        },
        // `h-8 / h-7 / h-9`, `px-2.5`.
        StyleId::Rhea => match size {
            ControlSize::Xs | ControlSize::Sm => slot(28.0, 10.0, 6.0, 14.0, 16.0),
            ControlSize::Md => slot(32.0, 10.0, 8.0, 14.0, 16.0),
            ControlSize::Lg => slot(36.0, 10.0, 8.0, 14.0, 16.0),
        },
    }
}

/// `min-w-*` always mirrors `h-*` in the pack CSS, so it is derived here.
const fn slot(
    height_px: f32,
    pad_x_px: f32,
    pad_x_icon_px: f32,
    text_size_px: f32,
    icon_px: f32,
) -> ToggleSizeRecipe {
    ToggleSizeRecipe {
        height_px,
        min_width_px: height_px,
        pad_x_px,
        pad_x_icon_px,
        text_size_px,
        icon_px,
    }
}

const fn text_sm_medium() -> TypeRecipe {
    TypeRecipe {
        size_px: 14.0,
        weight: FontWeight::Medium,
        uppercase: false,
        tracking_em: 0.0,
        line_height_px: 20.0,
    }
}

const fn text_xs_medium() -> TypeRecipe {
    TypeRecipe {
        size_px: 12.0,
        weight: FontWeight::Medium,
        uppercase: false,
        tracking_em: 0.0,
        line_height_px: 16.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn footprints_grow_monotonically_within_every_pack() {
        for style in StyleId::ALL {
            let sm = toggle_size(style, ControlSize::Sm);
            let md = toggle_size(style, ControlSize::Md);
            let lg = toggle_size(style, ControlSize::Lg);

            assert!(sm.height_px < md.height_px, "{style:?} sm >= default");
            assert!(md.height_px < lg.height_px, "{style:?} default >= lg");
            assert!(sm.pad_x_px <= lg.pad_x_px, "{style:?} padding shrinks");
        }
    }

    #[test]
    fn icon_footprints_are_square() {
        for style in StyleId::ALL {
            for size in [ControlSize::Sm, ControlSize::Md, ControlSize::Lg] {
                let recipe = toggle_size(style, size);

                assert_eq!(recipe.min_width_px, recipe.height_px);
                assert!(recipe.icon_px < recipe.height_px);
                assert!(recipe.text_size_px > 0.0);
                assert!(
                    recipe.pad_x_icon_px <= recipe.pad_x_px,
                    "{style:?} {size:?} icon padding exceeds the default one",
                );
            }
        }
    }

    #[test]
    fn only_vega_outline_carries_a_shadow() {
        for style in StyleId::ALL {
            assert_eq!(
                toggle_recipe(style).outline_shadow,
                matches!(style, StyleId::Vega),
            );
        }
    }
}
