//! Style resolution for the tooltip surface and arrow.

use crate::iced_compat::Color;
use shadcn_common::{TooltipRecipe, tooltip_recipe};

use crate::recipes::component_radius_px;
use crate::theme::Theme;

/// Resolved visuals of a tooltip bubble.
///
/// The web component paints the swapped theme pair — `bg-foreground` /
/// `text-background` — so tooltips stay readable in both modes.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TooltipStyle {
    /// Bubble and arrow fill (`bg-foreground`).
    pub background: Color,
    /// Content text color (`text-background`).
    pub text_color: Color,
    /// Bubble corner radius in px.
    pub radius: f32,
    /// Edge length of the square diamond arrow in px.
    pub arrow_size: f32,
    /// Corner radius of the arrow in px.
    pub arrow_radius: f32,
}

/// Resolves the tooltip style from the active theme and style pack.
pub(super) fn resolve_style(theme: &Theme) -> TooltipStyle {
    let recipe = recipe(theme);

    TooltipStyle {
        background: theme.palette.foreground,
        text_color: theme.palette.background,
        radius: component_radius_px(theme, recipe.radius),
        arrow_size: recipe.arrow_size_px,
        arrow_radius: recipe.arrow_radius_px,
    }
}

/// The backend-agnostic geometry recipe for the active style pack.
pub(super) fn recipe(theme: &Theme) -> TooltipRecipe {
    tooltip_recipe(theme.style_id())
}
