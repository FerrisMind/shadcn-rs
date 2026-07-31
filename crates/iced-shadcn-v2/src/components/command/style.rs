//! Style resolution for [`super::Command`].

use shadcn_common::{
    COMMAND_DISABLED_OPACITY, CommandRecipe, ComponentRadius, TypeRecipe, command_recipe,
};

use crate::iced_compat::{Background, Border, Color, Shadow, Vector, border};
use crate::recipes::component_radius_px;
use crate::theme::Theme;

use super::types::CommandRadius;

/// Resolved chrome for the command surface and rows.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CommandStyle {
    /// `bg-popover`.
    pub background: Color,
    /// `text-popover-foreground`.
    pub foreground: Color,
    /// `text-muted-foreground`.
    pub muted_foreground: Color,
    /// `border` / ring hairline.
    pub border: Color,
    /// Selected row `bg-muted`.
    pub selected_background: Color,
    /// Selected row text (`text-foreground`).
    pub selected_foreground: Color,
    /// Input fill (`bg-input/N`).
    pub input_background: Color,
    /// Input border (`border-input/N`).
    pub input_border: Color,
    /// Separator color.
    pub separator: Color,
    /// Surface corner radius in px.
    pub radius: f32,
    /// Input corner radius in px.
    pub input_radius: f32,
    /// Item corner radius in px.
    pub item_radius: f32,
    /// Surface shadow.
    pub shadow: Shadow,
    /// Border width of the surface ring (`1` when bordered).
    pub border_width: f32,
}

/// Resolves the style-pack recipe for `theme`.
#[must_use]
pub fn recipe(theme: &Theme) -> CommandRecipe {
    command_recipe(theme.style_id())
}

/// Resolves iced colors and radii for the active theme.
#[must_use]
pub fn resolve_style(
    theme: &Theme,
    radius: Option<CommandRadius>,
    in_dialog: bool,
    show_border: bool,
    show_shadow: bool,
) -> CommandStyle {
    let recipe = recipe(theme);
    let p = &theme.palette;
    let radius_intent = radius
        .and_then(CommandRadius::to_component)
        .unwrap_or(recipe.radius);
    let item_intent = if in_dialog {
        recipe.item_radius_in_dialog
    } else {
        recipe.item_radius
    };

    let input_bg = with_alpha(p.input, recipe.input_fill_alpha);
    let input_border = with_alpha(p.input, if recipe.input_bordered { 0.3 } else { 0.0 });
    let separator = with_alpha(p.border, recipe.separator_alpha);

    let shadow = if show_shadow {
        recipe.shadow.map_or_else(Shadow::default, |s| Shadow {
            color: Color {
                a: s.alpha,
                ..Color::BLACK
            },
            offset: Vector::new(0.0, s.offset_y_px),
            blur_radius: s.blur_px,
        })
    } else {
        Shadow::default()
    };

    CommandStyle {
        // Dialog.Content already paints `bg-popover`; keep Command transparent
        // so its fill cannot cover the dialog's outside ring.
        background: if in_dialog {
            Color::TRANSPARENT
        } else {
            p.popover
        },
        foreground: p.popover_foreground,
        muted_foreground: p.muted_foreground,
        border: p.border,
        selected_background: p.muted,
        selected_foreground: p.foreground,
        input_background: input_bg,
        input_border,
        separator,
        radius: component_radius_px(theme, radius_intent),
        input_radius: component_radius_px(theme, recipe.input_radius),
        item_radius: component_radius_px(theme, item_intent),
        shadow,
        border_width: if show_border && recipe.show_border {
            1.0
        } else {
            0.0
        },
    }
}

/// Container style for the command root surface.
#[must_use]
#[allow(dead_code)]
pub fn surface_container_style(
    style: CommandStyle,
) -> crate::iced_compat::widget::container::Style {
    crate::iced_compat::widget::container::Style {
        background: Some(Background::Color(style.background)),
        text_color: Some(style.foreground),
        border: Border {
            color: style.border,
            width: style.border_width,
            radius: border::radius(style.radius),
        },
        shadow: style.shadow,
        snap: false,
    }
}

/// Item row fill for selected / idle / disabled.
#[must_use]
pub fn item_background(style: CommandStyle, selected: bool, disabled: bool) -> Color {
    let mut color = if selected {
        style.selected_background
    } else {
        Color::TRANSPARENT
    };
    if disabled {
        color.a *= COMMAND_DISABLED_OPACITY;
    }
    color
}

/// Item text color.
#[must_use]
pub fn item_foreground(style: CommandStyle, selected: bool, disabled: bool) -> Color {
    let mut color = if selected {
        style.selected_foreground
    } else {
        style.foreground
    };
    if disabled {
        color.a *= COMMAND_DISABLED_OPACITY;
    }
    color
}

/// Maps a typography recipe to iced text size.
#[must_use]
pub fn typography_size(ty: TypeRecipe) -> f32 {
    ty.size_px
}

/// Resolves a [`ComponentRadius`] against the theme (crate-local helper).
#[must_use]
#[allow(dead_code)]
pub fn radius_px(theme: &Theme, radius: ComponentRadius) -> f32 {
    component_radius_px(theme, radius)
}

fn with_alpha(color: Color, alpha: f32) -> Color {
    Color {
        a: color.a * alpha.clamp(0.0, 1.0),
        ..color
    }
}
