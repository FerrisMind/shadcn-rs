//! Style resolution for the popover surface.

use crate::iced_compat::{Color, Shadow, Vector};
use shadcn_common::{PopoverRecipe, popover_recipe};

use crate::recipes::component_radius_px;
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
        radius: component_radius_px(theme, recipe.radius),
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
