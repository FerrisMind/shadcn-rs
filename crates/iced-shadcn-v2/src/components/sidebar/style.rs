//! Semantic color resolution for sidebar surfaces.

use crate::iced_compat::Color;

use shadcn_common::{
    SIDEBAR_DISABLED_OPACITY, SIDEBAR_GROUP_LABEL_FG_ALPHA, SidebarRecipe, sidebar_recipe,
};

use crate::theme::Theme;

/// Resolved sidebar palette + recipe for the active style pack.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SidebarStyle {
    /// `bg-sidebar`.
    pub background: Color,
    /// `text-sidebar-foreground`.
    pub foreground: Color,
    /// `bg-sidebar-primary`.
    pub primary: Color,
    /// `text-sidebar-primary-foreground`.
    pub primary_foreground: Color,
    /// `bg-sidebar-accent`.
    pub accent: Color,
    /// `text-sidebar-accent-foreground`.
    pub accent_foreground: Color,
    /// `border-sidebar-border`.
    pub border: Color,
    /// `ring-sidebar-ring`.
    pub ring: Color,
    /// Geometry / typography tokens.
    pub recipe: SidebarRecipe,
}

/// Resolves sidebar colors and the active style-pack recipe.
#[must_use]
pub fn resolve_style(theme: &Theme) -> SidebarStyle {
    let palette = &theme.palette;
    SidebarStyle {
        background: palette.sidebar,
        foreground: palette.sidebar_foreground,
        primary: palette.sidebar_primary,
        primary_foreground: palette.sidebar_primary_foreground,
        accent: palette.sidebar_accent,
        accent_foreground: palette.sidebar_accent_foreground,
        border: palette.sidebar_border,
        ring: palette.sidebar_ring,
        recipe: sidebar_recipe(theme.style_id()),
    }
}

/// Applies the group-label muted alpha (`/70`).
#[must_use]
pub fn group_label_color(style: &SidebarStyle) -> Color {
    with_alpha(style.foreground, SIDEBAR_GROUP_LABEL_FG_ALPHA)
}

/// Applies the disabled opacity (`opacity-50`).
#[must_use]
pub fn disabled_color(color: Color) -> Color {
    with_alpha(color, SIDEBAR_DISABLED_OPACITY)
}

fn with_alpha(color: Color, alpha: f32) -> Color {
    Color {
        a: color.a * alpha.clamp(0.0, 1.0),
        ..color
    }
}
