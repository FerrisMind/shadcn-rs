//! Slider recipes from `.cn-slider*` (track / range / thumb) across style packs.
//!
//! The tables below are transcribed from the Tailwind utilities of every pack
//! (`h-1.5` = `6px`, `size-4` = `16px`, `min-h-40` = `160px`, …). Colors stay
//! backend-agnostic: the recipe only names the semantic slot and its opacity,
//! and each GUI backend resolves it against its own palette.

use crate::style::StyleId;

use super::ComponentRadius;

/// Semantic surface a slider track is filled with.
///
/// ```rust
/// use shadcn_common::{SliderTrackSurface, StyleId, slider_recipe};
///
/// assert_eq!(slider_recipe(StyleId::Vega).track_surface, SliderTrackSurface::Muted);
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum SliderTrackSurface {
    /// `bg-muted`.
    #[default]
    Muted,
    /// `bg-input` (packs dim it through [`SliderRecipe::track_opacity`]).
    Input,
}

/// Semantic fill of a slider thumb.
///
/// ```rust
/// use shadcn_common::{SliderThumbFill, StyleId, slider_recipe};
///
/// assert_eq!(slider_recipe(StyleId::Sera).thumb_fill, SliderThumbFill::Primary);
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum SliderThumbFill {
    /// `bg-white` — the packs keep the thumb light in both modes.
    #[default]
    Surface,
    /// `bg-primary`.
    Primary,
}

/// Semantic border tone of a slider thumb.
///
/// ```rust
/// use shadcn_common::{SliderThumbBorder, StyleId, slider_recipe};
///
/// assert_eq!(slider_recipe(StyleId::Sera).thumb_border, SliderThumbBorder::None);
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum SliderThumbBorder {
    /// `border-none`.
    None,
    /// `border-primary`.
    Primary,
    /// `border-ring`.
    #[default]
    Ring,
    /// `ring-1 ring-black/10` — a hairline that only separates thumb from track.
    Subtle,
}

/// Geometry and semantic tokens of `.cn-slider*` for one style pack.
///
/// Sizes are expressed along and across the slider axis, so one table serves
/// both orientations: `thumb_length_px` runs along the track (horizontal width /
/// vertical height) and `thumb_thickness_px` across it. Luma is the only pack
/// with an oblong thumb (`h-4 w-6`, mirrored when vertical).
///
/// ```rust
/// use shadcn_common::{StyleId, slider_recipe};
///
/// let vega = slider_recipe(StyleId::Vega);
/// assert_eq!(vega.track_thickness_px, 6.0);
/// assert_eq!(vega.thumb_length_px, 16.0);
/// // Every pack keeps the thumb thicker than the track it rides on.
/// assert!(vega.thumb_thickness_px > vega.track_thickness_px);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SliderRecipe {
    /// Track thickness across the axis (`h-*` horizontal / `w-*` vertical).
    pub track_thickness_px: f32,
    /// Track and range corner radius.
    pub track_radius: ComponentRadius,
    /// Semantic surface of the track.
    pub track_surface: SliderTrackSurface,
    /// Opacity applied to the track surface (`bg-input/90` → `0.9`).
    pub track_opacity: f32,
    /// Thumb size along the axis.
    pub thumb_length_px: f32,
    /// Thumb size across the axis.
    pub thumb_thickness_px: f32,
    /// Thumb corner radius.
    pub thumb_radius: ComponentRadius,
    /// Thumb fill.
    pub thumb_fill: SliderThumbFill,
    /// Thumb border tone.
    pub thumb_border: SliderThumbBorder,
    /// Thumb border width in logical pixels (`0` when the pack has no border).
    pub thumb_border_px: f32,
    /// `hover:ring-*` / `focus-visible:ring-*` width in logical pixels.
    pub ring_width_px: f32,
    /// Alpha of the ring color (`ring-ring/50` → `0.5`).
    pub ring_opacity: f32,
    /// Minimum length of a vertical slider (`data-vertical:min-h-40`).
    pub min_length_px: f32,
}

/// `data-vertical:min-h-40` is shared by every pack.
const MIN_LENGTH_PX: f32 = 160.0;

/// Resolves `.cn-slider*` tokens for `style`.
///
/// ```rust
/// use shadcn_common::{ComponentRadius, StyleId, slider_recipe};
///
/// assert_eq!(slider_recipe(StyleId::Lyra).track_radius, ComponentRadius::None);
/// assert_eq!(slider_recipe(StyleId::Rhea).thumb_radius, ComponentRadius::S2xl);
/// ```
pub const fn slider_recipe(style: StyleId) -> SliderRecipe {
    match style {
        // track `h-1.5 rounded-full bg-muted`; thumb `size-4 border-primary
        // bg-white`, `hover:ring-4 ring-ring/50`.
        StyleId::Vega => SliderRecipe {
            track_thickness_px: 6.0,
            track_radius: ComponentRadius::Full,
            track_surface: SliderTrackSurface::Muted,
            track_opacity: 1.0,
            thumb_length_px: 16.0,
            thumb_thickness_px: 16.0,
            thumb_radius: ComponentRadius::Full,
            thumb_fill: SliderThumbFill::Surface,
            thumb_border: SliderThumbBorder::Primary,
            thumb_border_px: 1.0,
            ring_width_px: 4.0,
            ring_opacity: 0.5,
            min_length_px: MIN_LENGTH_PX,
        },
        // track `h-1 rounded-full`; thumb `size-3 border-ring`, `hover:ring-3`.
        StyleId::Nova => SliderRecipe {
            track_thickness_px: 4.0,
            track_radius: ComponentRadius::Full,
            track_surface: SliderTrackSurface::Muted,
            track_opacity: 1.0,
            thumb_length_px: 12.0,
            thumb_thickness_px: 12.0,
            thumb_radius: ComponentRadius::Full,
            thumb_fill: SliderThumbFill::Surface,
            thumb_border: SliderThumbBorder::Ring,
            thumb_border_px: 1.0,
            ring_width_px: 3.0,
            ring_opacity: 0.5,
            min_length_px: MIN_LENGTH_PX,
        },
        // track `h-3 rounded-4xl`; thumb `size-4 rounded-4xl border-primary`, `hover:ring-4`.
        StyleId::Maia => SliderRecipe {
            track_thickness_px: 12.0,
            track_radius: ComponentRadius::S4xl,
            track_surface: SliderTrackSurface::Muted,
            track_opacity: 1.0,
            thumb_length_px: 16.0,
            thumb_thickness_px: 16.0,
            thumb_radius: ComponentRadius::S4xl,
            thumb_fill: SliderThumbFill::Surface,
            thumb_border: SliderThumbBorder::Primary,
            thumb_border_px: 1.0,
            ring_width_px: 4.0,
            ring_opacity: 0.5,
            min_length_px: MIN_LENGTH_PX,
        },
        // track `h-1 rounded-none`; thumb `size-3 rounded-none`, `hover:ring-1`.
        StyleId::Lyra => SliderRecipe {
            track_thickness_px: 4.0,
            track_radius: ComponentRadius::None,
            track_surface: SliderTrackSurface::Muted,
            track_opacity: 1.0,
            thumb_length_px: 12.0,
            thumb_thickness_px: 12.0,
            thumb_radius: ComponentRadius::None,
            thumb_fill: SliderThumbFill::Surface,
            thumb_border: SliderThumbBorder::Ring,
            thumb_border_px: 1.0,
            ring_width_px: 1.0,
            ring_opacity: 0.5,
            min_length_px: MIN_LENGTH_PX,
        },
        // track `h-1 rounded-md`; thumb `size-3 rounded-md`, `hover:ring-2 ring/30`.
        StyleId::Mira => SliderRecipe {
            track_thickness_px: 4.0,
            track_radius: ComponentRadius::Md,
            track_surface: SliderTrackSurface::Muted,
            track_opacity: 1.0,
            thumb_length_px: 12.0,
            thumb_thickness_px: 12.0,
            thumb_radius: ComponentRadius::Md,
            thumb_fill: SliderThumbFill::Surface,
            thumb_border: SliderThumbBorder::Ring,
            thumb_border_px: 1.0,
            ring_width_px: 2.0,
            ring_opacity: 0.3,
            min_length_px: MIN_LENGTH_PX,
        },
        // track `h-2 rounded-full bg-input/90`; oblong thumb `h-4 w-6` with a
        // `ring-1 ring-black/10` hairline, `hover:ring-4 ring/30`.
        StyleId::Luma => SliderRecipe {
            track_thickness_px: 8.0,
            track_radius: ComponentRadius::Full,
            track_surface: SliderTrackSurface::Input,
            track_opacity: 0.9,
            thumb_length_px: 24.0,
            thumb_thickness_px: 16.0,
            thumb_radius: ComponentRadius::Full,
            thumb_fill: SliderThumbFill::Surface,
            thumb_border: SliderThumbBorder::Subtle,
            thumb_border_px: 1.0,
            ring_width_px: 4.0,
            ring_opacity: 0.3,
            min_length_px: MIN_LENGTH_PX,
        },
        // track `h-0.5 bg-input/50`, square corners; thumb `size-3 bg-primary
        // border-none`, `hover:ring-2 ring/30`.
        StyleId::Sera => SliderRecipe {
            track_thickness_px: 2.0,
            track_radius: ComponentRadius::None,
            track_surface: SliderTrackSurface::Input,
            track_opacity: 0.5,
            thumb_length_px: 12.0,
            thumb_thickness_px: 12.0,
            thumb_radius: ComponentRadius::None,
            thumb_fill: SliderThumbFill::Primary,
            thumb_border: SliderThumbBorder::None,
            thumb_border_px: 0.0,
            ring_width_px: 2.0,
            ring_opacity: 0.3,
            min_length_px: MIN_LENGTH_PX,
        },
        // track `h-1 rounded-2xl bg-input/90`; thumb `size-4 rounded-2xl` with a
        // `ring-1 ring-black/10` hairline, `hover:ring-4 ring/30`.
        StyleId::Rhea => SliderRecipe {
            track_thickness_px: 4.0,
            track_radius: ComponentRadius::S2xl,
            track_surface: SliderTrackSurface::Input,
            track_opacity: 0.9,
            thumb_length_px: 16.0,
            thumb_thickness_px: 16.0,
            thumb_radius: ComponentRadius::S2xl,
            thumb_fill: SliderThumbFill::Surface,
            thumb_border: SliderThumbBorder::Subtle,
            thumb_border_px: 1.0,
            ring_width_px: 4.0,
            ring_opacity: 0.3,
            min_length_px: MIN_LENGTH_PX,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thumbs_always_cover_the_track_they_ride_on() {
        for style in StyleId::ALL {
            let recipe = slider_recipe(style);

            assert!(
                recipe.thumb_thickness_px > recipe.track_thickness_px,
                "{style:?} thumb is thinner than its track",
            );
            assert!(recipe.thumb_length_px >= recipe.thumb_thickness_px);
            assert!(recipe.track_thickness_px > 0.0);
        }
    }

    #[test]
    fn ring_and_opacity_tokens_stay_in_range() {
        for style in StyleId::ALL {
            let recipe = slider_recipe(style);

            assert!(recipe.ring_width_px > 0.0);
            assert!(recipe.ring_opacity > 0.0 && recipe.ring_opacity <= 1.0);
            assert!(recipe.track_opacity > 0.0 && recipe.track_opacity <= 1.0);
            assert!(recipe.min_length_px >= 160.0);
        }
    }

    #[test]
    fn borderless_packs_report_zero_border_width() {
        for style in StyleId::ALL {
            let recipe = slider_recipe(style);

            assert_eq!(
                recipe.thumb_border == SliderThumbBorder::None,
                recipe.thumb_border_px == 0.0,
                "{style:?} border tone and width disagree",
            );
        }
    }
}
