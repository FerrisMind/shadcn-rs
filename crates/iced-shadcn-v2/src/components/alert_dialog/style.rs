//! Style resolution for the alert-dialog surface, backdrop, and media box.

use crate::iced_compat::{Color, Shadow, Vector};
use shadcn_common::{AlertDialogRecipe, alert_dialog_recipe};

use crate::recipes::component_radius_px;
use crate::theme::Theme;

/// Resolved visuals of an alert dialog: backdrop, surface, and media box.
///
/// The web component paints `bg-popover` / `text-popover-foreground` with a
/// `ring-1 ring-foreground/N` hairline over a `bg-black/N` backdrop
/// (`.cn-alert-dialog-overlay` / `.cn-alert-dialog-content`); the media
/// slot rests on `bg-muted`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AlertDialogStyle {
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
    /// Media box fill (`.cn-alert-dialog-media`: `bg-muted`).
    pub media_background: Color,
    /// Media box corner radius in px (`rounded-md` / `rounded-full` /
    /// `rounded-none`).
    pub media_radius: f32,
}

/// Resolves the alert-dialog style from the active theme and style pack.
pub(super) fn resolve_style(theme: &Theme) -> AlertDialogStyle {
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

    AlertDialogStyle {
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
        media_background: theme.palette.muted,
        media_radius: component_radius_px(theme, recipe.media_radius)
            .min(recipe.media_size_px / 2.0),
    }
}

/// The backend-agnostic geometry recipe for the active style pack.
pub(super) fn recipe(theme: &Theme) -> AlertDialogRecipe {
    alert_dialog_recipe(theme.style_id())
}
