//! Style resolution for the sheet surface, backdrop, and close button.

use crate::iced_compat::{Color, Shadow, Vector};
use shadcn_common::{SheetRecipe, sheet_recipe};

use crate::recipes::component_radius_px;
use crate::theme::Theme;

/// Resolved visuals of a sheet: backdrop, surface, edge border, close button.
///
/// The web component paints `bg-popover` / `text-popover-foreground` with a
/// side `border-*` hairline over a `bg-black/N` backdrop
/// (`.cn-sheet-overlay` / `.cn-sheet-content`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SheetStyle {
    /// Backdrop fill (`bg-black/10` … `bg-black/80`).
    pub overlay: Color,
    /// Surface fill (`bg-popover`).
    pub background: Color,
    /// Content text color (`text-popover-foreground`).
    pub text_color: Color,
    /// Edge border color (`border`).
    pub border_color: Color,
    /// Edge border width (`border-l` / `border-r` / …).
    pub border_width: f32,
    /// Surface drop shadow.
    pub shadow: Shadow,
    /// Resting fill of the close button (transparent ghost or `bg-secondary`).
    pub close_background: Color,
    /// Hovered fill of the close button (`bg-accent`).
    pub close_hover_background: Color,
    /// Close glyph color.
    pub close_icon_color: Color,
    /// Close button corner radius in px (`rounded-md`).
    pub close_radius: f32,
}

/// Resolves the sheet style from the active theme and style pack.
pub(super) fn resolve_style(theme: &Theme) -> SheetStyle {
    let recipe = recipe(theme);

    let shadow = Shadow {
        color: Color::BLACK.scale_alpha(recipe.shadow.alpha),
        offset: Vector::new(0.0, recipe.shadow.offset_y_px),
        blur_radius: recipe.shadow.blur_px,
    };

    let (close_background, close_icon_color) = if recipe.close_secondary_bg {
        (theme.palette.secondary, theme.palette.secondary_foreground)
    } else {
        (Color::TRANSPARENT, theme.palette.popover_foreground)
    };

    SheetStyle {
        overlay: Color::BLACK.scale_alpha(recipe.overlay_alpha),
        background: theme.palette.popover,
        text_color: theme.palette.popover_foreground,
        border_color: theme.palette.border,
        border_width: 1.0,
        shadow,
        close_background,
        close_hover_background: theme.palette.accent,
        close_icon_color,
        close_radius: component_radius_px(theme, theme.style.button_type().default_radius),
    }
}

/// The backend-agnostic geometry recipe for the active style pack.
pub(super) fn recipe(theme: &Theme) -> SheetRecipe {
    sheet_recipe(theme.style_id())
}
