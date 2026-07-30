//! Switch geometry recipes from `.cn-switch` / `.cn-switch-thumb`.
//!
//! shadcn-svelte exposes two switch footprints (`data-size="sm"` and
//! `data-size="default"`); every style pack redefines the track, the thumb, and
//! the distance the thumb travels. The tables below are transcribed from the
//! Tailwind utilities of each pack (`1rem` = `16px`, `size-4` = `16px`, …).

use crate::style::StyleId;

use super::{ComponentRadius, ControlSize};

/// Track-independent `.cn-switch` tokens of a style pack.
///
/// ```rust
/// use shadcn_common::{StyleId, switch_recipe};
///
/// let sera = switch_recipe(StyleId::Sera);
/// assert_eq!(sera.border_width_px, 1.0);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SwitchRecipe {
    /// Track and thumb radius from the pack's Switch CSS.
    pub default_radius: ComponentRadius,
    /// Track border width in logical pixels.
    pub border_width_px: f32,
    /// `focus-visible:ring-*` width in logical pixels.
    pub ring_width_px: f32,
    /// Alpha of the focus ring color (`ring/50` → `0.5`).
    pub ring_opacity: f32,
}

/// Geometry of one switch footprint (`sm` / `default`) under a style pack.
///
/// The thumb is positioned from the leading edge of the track: it starts at
/// [`Self::thumb_inset_px`] and moves [`Self::thumb_travel_px`] further when the
/// switch is checked, which mirrors the `translate-x-*` utilities of the web
/// component.
///
/// ```rust
/// use shadcn_common::{ControlSize, StyleId, switch_size};
///
/// let vega = switch_size(StyleId::Vega, ControlSize::Md);
/// assert_eq!(vega.track_width_px, 32.0);
/// // The thumb keeps a symmetric gap on both ends of the track.
/// assert_eq!(
///     vega.track_width_px - vega.checked_thumb_inset_px() - vega.thumb_width_px,
///     vega.thumb_inset_px,
/// );
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SwitchSizeRecipe {
    /// Track width in logical pixels.
    pub track_width_px: f32,
    /// Track height in logical pixels.
    pub track_height_px: f32,
    /// Thumb width in logical pixels.
    pub thumb_width_px: f32,
    /// Thumb height in logical pixels.
    pub thumb_height_px: f32,
    /// Distance between the track's leading edge and the unchecked thumb.
    pub thumb_inset_px: f32,
    /// Distance the thumb travels when the switch becomes checked.
    pub thumb_travel_px: f32,
}

impl SwitchSizeRecipe {
    /// Distance between the track's leading edge and the checked thumb.
    pub const fn checked_thumb_inset_px(self) -> f32 {
        self.thumb_inset_px + self.thumb_travel_px
    }
}

/// Resolves `.cn-switch` tokens for `style`.
///
/// ```rust
/// use shadcn_common::{ComponentRadius, StyleId, switch_recipe};
///
/// assert_eq!(
///     switch_recipe(StyleId::Rhea).default_radius,
///     ComponentRadius::S2xl,
/// );
/// ```
pub const fn switch_recipe(style: StyleId) -> SwitchRecipe {
    match style {
        // `rounded-full border border-transparent focus-visible:ring-3 ring/50`
        StyleId::Vega | StyleId::Nova => SwitchRecipe {
            default_radius: ComponentRadius::Full,
            border_width_px: 1.0,
            ring_width_px: 3.0,
            ring_opacity: 0.5,
        },
        // Same as Vega, but `focus-visible:ring-[3px]`.
        StyleId::Maia => SwitchRecipe {
            default_radius: ComponentRadius::Full,
            border_width_px: 1.0,
            ring_width_px: 3.0,
            ring_opacity: 0.5,
        },
        // `focus-visible:ring-1`
        StyleId::Lyra => SwitchRecipe {
            default_radius: ComponentRadius::Full,
            border_width_px: 1.0,
            ring_width_px: 1.0,
            ring_opacity: 0.5,
        },
        // `focus-visible:ring-2 ring/30`
        StyleId::Mira => SwitchRecipe {
            default_radius: ComponentRadius::Full,
            border_width_px: 1.0,
            ring_width_px: 2.0,
            ring_opacity: 0.3,
        },
        // `rounded-full border-2 focus-visible:ring-3 ring/30`
        StyleId::Luma => SwitchRecipe {
            default_radius: ComponentRadius::Full,
            border_width_px: 2.0,
            ring_width_px: 3.0,
            ring_opacity: 0.3,
        },
        // `rounded-none border focus-visible:ring-2 ring/30`
        StyleId::Sera => SwitchRecipe {
            default_radius: ComponentRadius::None,
            border_width_px: 1.0,
            ring_width_px: 2.0,
            ring_opacity: 0.3,
        },
        // `rounded-2xl border-2 focus-visible:ring-3 ring/30`
        StyleId::Rhea => SwitchRecipe {
            default_radius: ComponentRadius::S2xl,
            border_width_px: 2.0,
            ring_width_px: 3.0,
            ring_opacity: 0.3,
        },
    }
}

/// Resolves `.cn-switch` / `.cn-switch-thumb` geometry for `style` + `size`.
///
/// shadcn-svelte only ships `sm` and `default` footprints:
/// [`ControlSize::Xs`] and [`ControlSize::Sm`] resolve to `sm`, while
/// [`ControlSize::Md`] and [`ControlSize::Lg`] resolve to `default`.
///
/// ```rust
/// use shadcn_common::{ControlSize, StyleId, switch_size};
///
/// let small = switch_size(StyleId::Vega, ControlSize::Sm);
/// let default = switch_size(StyleId::Vega, ControlSize::Md);
/// assert!(small.track_height_px < default.track_height_px);
/// assert_eq!(small, switch_size(StyleId::Vega, ControlSize::Xs));
/// ```
pub const fn switch_size(style: StyleId, size: ControlSize) -> SwitchSizeRecipe {
    let small = matches!(size, ControlSize::Xs | ControlSize::Sm);

    match style {
        // `h-[18.4px] w-[32px]` + `size-4`, thumb `translate-x-[calc(100%-2px)]`.
        StyleId::Vega | StyleId::Nova | StyleId::Maia | StyleId::Lyra => {
            if small {
                SwitchSizeRecipe {
                    track_width_px: 24.0,
                    track_height_px: 14.0,
                    thumb_width_px: 12.0,
                    thumb_height_px: 12.0,
                    thumb_inset_px: 1.0,
                    thumb_travel_px: 10.0,
                }
            } else {
                SwitchSizeRecipe {
                    track_width_px: 32.0,
                    track_height_px: 18.4,
                    thumb_width_px: 16.0,
                    thumb_height_px: 16.0,
                    thumb_inset_px: 1.0,
                    thumb_travel_px: 14.0,
                }
            }
        }
        // `h-[16.6px] w-[28px]` + `size-3.5`.
        StyleId::Mira => {
            if small {
                SwitchSizeRecipe {
                    track_width_px: 24.0,
                    track_height_px: 14.0,
                    thumb_width_px: 12.0,
                    thumb_height_px: 12.0,
                    thumb_inset_px: 1.0,
                    thumb_travel_px: 10.0,
                }
            } else {
                SwitchSizeRecipe {
                    track_width_px: 28.0,
                    track_height_px: 16.6,
                    thumb_width_px: 14.0,
                    thumb_height_px: 14.0,
                    thumb_inset_px: 1.0,
                    thumb_travel_px: 12.0,
                }
            }
        }
        // `h-5 w-11` + oblong thumb `h-4 w-6`, `translate-x-[calc(100%-8px)]`.
        StyleId::Luma => {
            if small {
                SwitchSizeRecipe {
                    track_width_px: 28.0,
                    track_height_px: 16.0,
                    thumb_width_px: 16.0,
                    thumb_height_px: 12.0,
                    thumb_inset_px: 2.0,
                    thumb_travel_px: 8.0,
                }
            } else {
                SwitchSizeRecipe {
                    track_width_px: 44.0,
                    track_height_px: 20.0,
                    thumb_width_px: 24.0,
                    thumb_height_px: 16.0,
                    thumb_inset_px: 2.0,
                    thumb_travel_px: 16.0,
                }
            }
        }
        // `h-4.5 w-8.25` + `size-3.5`, thumb `translate-x-[calc(100%+2px)]`
        // from a `translate-x-0.25` resting position.
        StyleId::Sera => {
            if small {
                SwitchSizeRecipe {
                    track_width_px: 25.0,
                    track_height_px: 14.0,
                    thumb_width_px: 10.0,
                    thumb_height_px: 10.0,
                    thumb_inset_px: 2.0,
                    thumb_travel_px: 11.0,
                }
            } else {
                SwitchSizeRecipe {
                    track_width_px: 33.0,
                    track_height_px: 18.0,
                    thumb_width_px: 14.0,
                    thumb_height_px: 14.0,
                    thumb_inset_px: 2.0,
                    thumb_travel_px: 15.0,
                }
            }
        }
        // `h-5 w-8` + `size-4`, thumb `translate-x-[calc(100%-4px)]`.
        StyleId::Rhea => {
            if small {
                SwitchSizeRecipe {
                    track_width_px: 24.0,
                    track_height_px: 16.0,
                    thumb_width_px: 12.0,
                    thumb_height_px: 12.0,
                    thumb_inset_px: 2.0,
                    thumb_travel_px: 8.0,
                }
            } else {
                SwitchSizeRecipe {
                    track_width_px: 32.0,
                    track_height_px: 20.0,
                    thumb_width_px: 16.0,
                    thumb_height_px: 16.0,
                    thumb_inset_px: 2.0,
                    thumb_travel_px: 12.0,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_pack_keeps_the_thumb_inside_the_track() {
        for style in StyleId::ALL {
            for size in [ControlSize::Sm, ControlSize::Md] {
                let recipe = switch_size(style, size);
                let trailing_gap =
                    recipe.track_width_px - recipe.checked_thumb_inset_px() - recipe.thumb_width_px;

                assert!(
                    (trailing_gap - recipe.thumb_inset_px).abs() < 0.001,
                    "{style:?} {size:?} thumb gaps are asymmetric: {trailing_gap}",
                );
                assert!(recipe.thumb_height_px <= recipe.track_height_px);
                assert!(recipe.thumb_travel_px > 0.0);
            }
        }
    }

    #[test]
    fn small_footprint_is_never_larger_than_the_default_one() {
        for style in StyleId::ALL {
            let small = switch_size(style, ControlSize::Sm);
            let default = switch_size(style, ControlSize::Md);

            assert!(small.track_width_px <= default.track_width_px);
            assert!(small.track_height_px <= default.track_height_px);
        }
    }

    #[test]
    fn ring_and_border_widths_are_positive() {
        for style in StyleId::ALL {
            let recipe = switch_recipe(style);

            assert!(recipe.border_width_px > 0.0);
            assert!(recipe.ring_width_px > 0.0);
            assert!(recipe.ring_opacity > 0.0 && recipe.ring_opacity <= 1.0);
        }
    }
}
