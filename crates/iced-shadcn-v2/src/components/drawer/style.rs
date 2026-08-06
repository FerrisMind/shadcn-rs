//! Style resolution for the drawer surface, backdrop, and drag handle.

use crate::iced_compat::border::Radius;
use crate::iced_compat::{Border, Color};
use shadcn_common::{
    DrawerCornerMask, DrawerDirection, DrawerRecipe, drawer_corner_mask, drawer_recipe,
};

use crate::recipes::component_radius_px;
use crate::theme::Theme;

/// Resolved visuals of a drawer: backdrop, surface, edge border, handle.
///
/// The web component paints `bg-popover` / `text-popover-foreground` with a
/// side `border-*` hairline over a `bg-black/N` backdrop
/// (`.cn-drawer-overlay` / `.cn-drawer-content`), plus a `bg-muted` handle
/// on bottom drawers.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DrawerStyle {
    /// Backdrop fill (`bg-black/10` … `bg-black/80`).
    pub overlay: Color,
    /// Surface fill (`bg-popover`).
    pub background: Color,
    /// Content text color (`text-popover-foreground`).
    pub text_color: Color,
    /// Edge border color (`border`).
    pub border_color: Color,
    /// Edge border width (`border-t` / `border-l` / …).
    pub border_width: f32,
    /// Absolute corner radius in px for enabled corners.
    pub radius_px: f32,
    /// Which corners receive [`Self::radius_px`].
    pub corner_mask: DrawerCornerMask,
    /// Floating-pack outer inset (`p-2` / `p-4`); `0` for solid packs.
    pub floating_pad_px: f32,
    /// Drag-handle fill (`bg-muted`).
    pub handle_color: Color,
    /// Drag-handle height in px.
    pub handle_height_px: f32,
    /// Drag-handle corner radius in px.
    pub handle_radius_px: f32,
}

impl DrawerStyle {
    /// Builds an iced [`Radius`] from the corner mask.
    #[must_use]
    pub fn surface_radius(self) -> Radius {
        let r = self.radius_px;
        let m = self.corner_mask;
        Radius {
            top_left: if m.top_left { r } else { 0.0 },
            top_right: if m.top_right { r } else { 0.0 },
            bottom_right: if m.bottom_right { r } else { 0.0 },
            bottom_left: if m.bottom_left { r } else { 0.0 },
        }
    }

    /// Surface border with the resolved per-corner radius and edge hairline.
    #[must_use]
    pub fn surface_border(self, progress: f32) -> Border {
        Border {
            color: self.border_color.scale_alpha(progress),
            width: self.border_width,
            radius: self.surface_radius(),
        }
    }
}

/// Resolves the drawer style from the active theme, style pack, and direction.
pub(super) fn resolve_style(theme: &Theme, direction: DrawerDirection) -> DrawerStyle {
    let recipe = recipe(theme);
    let mask = drawer_corner_mask(direction, &recipe);

    DrawerStyle {
        overlay: Color::BLACK.scale_alpha(recipe.overlay_alpha),
        background: theme.palette.popover,
        text_color: theme.palette.popover_foreground,
        border_color: theme.palette.border,
        border_width: 1.0,
        radius_px: component_radius_px(theme, recipe.radius),
        corner_mask: mask,
        floating_pad_px: recipe.floating_pad_px,
        handle_color: theme.palette.muted,
        handle_height_px: recipe.handle_height_px,
        handle_radius_px: component_radius_px(theme, recipe.handle_radius),
    }
}

/// The backend-agnostic geometry recipe for the active style pack.
pub(super) fn recipe(theme: &Theme) -> DrawerRecipe {
    drawer_recipe(theme.style_id())
}
