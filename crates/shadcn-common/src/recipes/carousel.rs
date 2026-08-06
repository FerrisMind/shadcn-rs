//! Carousel control recipes from `.cn-carousel-previous` / `.cn-carousel-next`
//! across style packs.

use crate::style::StyleId;

use super::ComponentRadius;

/// Style-pack tokens for the carousel prev/next controls.
///
/// The packs restyle only the control corner radius; geometry (offsets, gaps)
/// is shared and lives in [`crate::carousel`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CarouselRecipe {
    /// Corner radius forced onto the controls, or `None` to keep the pack's
    /// default button radius.
    pub control_radius: Option<ComponentRadius>,
}

/// Resolves `.cn-carousel-previous` / `.cn-carousel-next` tokens for `style`.
///
/// ```rust
/// use shadcn_common::{ComponentRadius, StyleId, carousel_recipe};
///
/// assert_eq!(
///     carousel_recipe(StyleId::Vega).control_radius,
///     Some(ComponentRadius::Full)
/// );
/// assert_eq!(carousel_recipe(StyleId::Lyra).control_radius, None);
/// ```
pub const fn carousel_recipe(style: StyleId) -> CarouselRecipe {
    match style {
        // `rounded-full` controls.
        StyleId::Vega | StyleId::Nova | StyleId::Maia | StyleId::Mira | StyleId::Luma => {
            CarouselRecipe {
                control_radius: Some(ComponentRadius::Full),
            }
        }
        // `rounded-2xl` controls.
        StyleId::Rhea => CarouselRecipe {
            control_radius: Some(ComponentRadius::S2xl),
        },
        // No carousel overrides — the pack's button radius applies.
        StyleId::Lyra | StyleId::Sera => CarouselRecipe {
            control_radius: None,
        },
    }
}
