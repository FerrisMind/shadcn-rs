//! Style resolution for the hover-card surface.

use crate::iced_compat::{Color, Shadow, Vector};
use shadcn_common::{HoverCardRecipe, hover_card_recipe};

use crate::recipes::component_radius_px;
use crate::theme::Theme;

/// Resolved visuals of a hover-card surface.
///
/// The web component paints `bg-popover` / `text-popover-foreground` with a
/// `ring-1 ring-foreground/N` hairline and a drop shadow, exactly like the
/// popover surface.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HoverCardStyle {
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

/// Resolves the hover-card style from the active theme and style pack.
pub(super) fn resolve_style(theme: &Theme) -> HoverCardStyle {
    let recipe = recipe(theme);
    let ring_alpha = if theme.is_dark() {
        recipe.ring_alpha_dark
    } else {
        recipe.ring_alpha
    };

    HoverCardStyle {
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
pub(super) fn recipe(theme: &Theme) -> HoverCardRecipe {
    hover_card_recipe(theme.style_id())
}
