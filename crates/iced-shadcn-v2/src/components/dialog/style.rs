//! Style resolution for the dialog surface, backdrop, and close button.

use crate::iced_compat::{Color, Shadow, Vector};
use shadcn_common::{DialogRecipe, dialog_recipe};

use crate::recipes::component_radius_px;
use crate::theme::Theme;

/// Resolved visuals of a dialog: backdrop, surface, close button, and the
/// optional footer bar.
///
/// The web component paints `bg-popover` / `text-popover-foreground` with a
/// `ring-1 ring-foreground/N` hairline over a `bg-black/N` backdrop
/// (`.cn-dialog-overlay` / `.cn-dialog-content`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DialogStyle {
    /// Backdrop fill (`bg-black/10` … `bg-black/80`).
    pub overlay: Color,
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
    /// Surface drop shadow (`shadow-md` / `shadow-xl`, transparent when the
    /// pack casts none).
    pub shadow: Shadow,
    /// Resting fill of the close button (transparent ghost or
    /// `bg-secondary`).
    pub close_background: Color,
    /// Hovered fill of the close button (ghost hover `bg-accent`).
    pub close_hover_background: Color,
    /// Close glyph color.
    pub close_icon_color: Color,
    /// Close button corner radius in px (`rounded-md`).
    pub close_radius: f32,
    /// Footer bar fill (`bg-muted/50`), used by packs with `footer_bar`.
    pub footer_background: Color,
    /// Footer bar top border color (`border-t`).
    pub footer_border_color: Color,
}

/// Resolves the dialog style from the active theme and style pack.
pub(super) fn resolve_style(theme: &Theme) -> DialogStyle {
    let recipe = recipe(theme);
    let ring_alpha = if theme.is_dark() {
        recipe.ring_alpha_dark
    } else {
        recipe.ring_alpha
    };

    let shadow = match recipe.shadow {
        Some(shadow) => Shadow {
            color: Color::BLACK.scale_alpha(shadow.alpha),
            offset: Vector::new(0.0, shadow.offset_y_px),
            blur_radius: shadow.blur_px,
        },
        None => Shadow::default(),
    };

    let (close_background, close_icon_color) = if recipe.close_secondary_bg {
        (theme.palette.secondary, theme.palette.secondary_foreground)
    } else {
        (Color::TRANSPARENT, theme.palette.popover_foreground)
    };

    DialogStyle {
        overlay: Color::BLACK.scale_alpha(recipe.overlay_alpha),
        background: theme.palette.popover,
        text_color: theme.palette.popover_foreground,
        border_color: theme.palette.foreground.scale_alpha(ring_alpha),
        border_width: 1.0,
        radius: {
            let scaled = component_radius_px(theme, recipe.radius);
            recipe.radius_px.map_or(scaled, |cap| scaled.min(cap))
        },
        shadow,
        close_background,
        close_hover_background: theme.palette.accent,
        close_icon_color,
        close_radius: component_radius_px(theme, theme.style.button_type().default_radius),
        footer_background: theme.palette.muted.scale_alpha(0.5),
        footer_border_color: theme.palette.border,
    }
}

/// The backend-agnostic geometry recipe for the active style pack.
pub(super) fn recipe(theme: &Theme) -> DialogRecipe {
    dialog_recipe(theme.style_id())
}
