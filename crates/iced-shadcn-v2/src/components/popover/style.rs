//! Style resolution for the popover surface.

use crate::iced_compat::{Color, Shadow, Vector};
use shadcn_common::{ComponentRadius, PopoverRecipe, popover_recipe};

use crate::theme::Theme;

/// Resolved visuals of a popover surface.
///
/// The web component paints `bg-popover` / `text-popover-foreground` with a
/// `ring-1 ring-foreground/N` hairline and a drop shadow.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PopoverStyle {
    /// Surface fill (`bg-popover`).
    pub background: Color,
    /// Content text color (`text-popover-foreground`).
    pub text_color: Color,
    /// Hairline ring color (`ring-foreground/N`).
    pub border_color: Color,
    /// Hairline ring width (`ring-1`).
    pub border_width: f32,
    /// Surface corner radius in px.
    pub radius: f32,
    /// Surface drop shadow (`shadow-md` / `shadow-lg` / `shadow-2xl`).
    pub shadow: Shadow,
}

/// Resolves the radius token used by the active style's `.cn-popover-content`.
///
/// Popover recipes use CSS radius tokens (`rounded-md`, `rounded-lg`,
/// `rounded-2xl`, ...), so they must resolve against the active radius scale.
/// The generic component-radius adapter intentionally maps control radii
/// through style-pack twill slots and is therefore not interchangeable here.
pub(crate) fn surface_radius(theme: &Theme) -> f32 {
    let scale = theme.radius_scale();
    let radius = recipe(theme).radius;

    match radius {
        ComponentRadius::None => 0.0,
        ComponentRadius::Sm => scale.sm_px,
        ComponentRadius::Md => scale.md_px,
        ComponentRadius::Lg => scale.lg_px,
        ComponentRadius::Xl => scale.xl_px,
        ComponentRadius::S2xl => scale.xxl_px,
        ComponentRadius::S3xl => scale.xxxl_px,
        ComponentRadius::S4xl => scale.xxxxl_px,
        ComponentRadius::Full => 9999.0,
        _ => scale.md_px,
    }
}

/// Resolves the popover style from the active theme and style pack.
pub(super) fn resolve_style(theme: &Theme) -> PopoverStyle {
    let recipe = recipe(theme);
    let ring_alpha = if theme.is_dark() {
        recipe.ring_alpha_dark
    } else {
        recipe.ring_alpha
    };

    PopoverStyle {
        background: theme.palette.popover,
        text_color: theme.palette.popover_foreground,
        border_color: theme.palette.foreground.scale_alpha(ring_alpha),
        border_width: 1.0,
        radius: surface_radius(theme),
        shadow: Shadow {
            color: Color::BLACK.scale_alpha(recipe.shadow.alpha),
            offset: Vector::new(0.0, recipe.shadow.offset_y_px),
            blur_radius: recipe.shadow.blur_px,
        },
    }
}

/// The backend-agnostic geometry recipe for the active style pack.
pub(super) fn recipe(theme: &Theme) -> PopoverRecipe {
    popover_recipe(theme.style_id())
}
