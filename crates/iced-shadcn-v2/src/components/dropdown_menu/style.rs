//! Mapping of `.cn-dropdown-menu-*` style-pack rules to resolved iced visuals.

use crate::iced_compat::{Color, Shadow, Vector};

use shadcn_common::{
    DROPDOWN_MENU_DESTRUCTIVE_FOCUS_ALPHA, DROPDOWN_MENU_DESTRUCTIVE_FOCUS_ALPHA_DARK,
    DROPDOWN_MENU_DISABLED_OPACITY, DropdownMenuRecipe, MenuItemVariant,
};
use twill_core::prelude::theme::SemanticColor;

use crate::recipes::component_radius_px;
use crate::theme::Theme;

/// Resolved visuals of the open menu content surface.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DropdownMenuContentStyle {
    /// Surface fill (`bg-popover`).
    pub background: Color,
    /// Content text color (`text-popover-foreground`).
    pub text_color: Color,
    /// Muted label / shortcut color.
    pub muted_color: Color,
    /// Hairline ring color (`ring-foreground/N`).
    pub border_color: Color,
    /// Hairline ring width (`ring-1`).
    pub border_width: f32,
    /// Surface corner radius in px.
    pub radius: f32,
    /// Surface drop shadow.
    pub shadow: Shadow,
    /// Highlighted / focused item fill (`bg-accent`).
    pub item_highlight_background: Color,
    /// Highlighted / focused item text (`text-accent-foreground`).
    pub item_highlight_text: Color,
    /// Destructive resting text (`text-destructive`).
    pub destructive_text: Color,
    /// Destructive focus fill (`bg-destructive/10` or `/20` dark).
    pub destructive_highlight_background: Color,
    /// Separator hairline (`bg-border` or `bg-border/50`).
    pub separator_color: Color,
    /// Disabled item text alpha multiplier.
    pub item_disabled_opacity: f32,
    /// Item corner radius in px.
    pub item_radius: f32,
}

/// `.cn-dropdown-menu-*` numbers of the active pack.
pub(super) fn recipe(theme: &Theme) -> DropdownMenuRecipe {
    theme.style.dropdown_menu()
}

/// Resolves the content / item palette for the active theme.
///
/// Overlay hosts that cannot embed [`super::DropdownMenu`] (separate windows,
/// custom chrome) should call this instead of hard-coding pack radii/pads —
/// Rhea vs Vega deltas live in `theme.style.dropdown_menu()`.
pub fn dropdown_menu_content_style(theme: &Theme, submenu: bool) -> DropdownMenuContentStyle {
    resolve_content_style(theme, submenu)
}

/// Resolves the content / item palette for the active theme.
pub(super) fn resolve_content_style(theme: &Theme, submenu: bool) -> DropdownMenuContentStyle {
    let pack = recipe(theme);
    let ring_alpha = if theme.is_dark() {
        pack.content_ring_alpha_dark
    } else {
        pack.content_ring_alpha
    };
    let shadow_tokens = if submenu {
        pack.sub_content_shadow
    } else {
        pack.content_shadow
    };
    let destructive = theme.semantic_color(SemanticColor::Destructive);
    let destructive_focus_alpha = if theme.is_dark() {
        DROPDOWN_MENU_DESTRUCTIVE_FOCUS_ALPHA_DARK
    } else {
        DROPDOWN_MENU_DESTRUCTIVE_FOCUS_ALPHA
    };
    let border = theme.semantic_color(SemanticColor::Border);
    let separator_color = if pack.separator_muted {
        border.scale_alpha(0.5)
    } else {
        border
    };

    DropdownMenuContentStyle {
        background: theme.palette.popover,
        text_color: theme.palette.popover_foreground,
        muted_color: theme.semantic_color(SemanticColor::MutedForeground),
        border_color: theme.palette.foreground.scale_alpha(ring_alpha),
        border_width: 1.0,
        radius: component_radius_px(theme, pack.content_radius),
        shadow: Shadow {
            color: Color::BLACK.scale_alpha(shadow_tokens.alpha),
            offset: Vector::new(0.0, shadow_tokens.offset_y_px),
            blur_radius: shadow_tokens.blur_px,
        },
        item_highlight_background: theme.semantic_color(SemanticColor::Accent),
        item_highlight_text: theme.semantic_color(SemanticColor::AccentForeground),
        destructive_text: destructive,
        destructive_highlight_background: destructive.scale_alpha(destructive_focus_alpha),
        separator_color,
        item_disabled_opacity: DROPDOWN_MENU_DISABLED_OPACITY,
        item_radius: component_radius_px(theme, pack.item_radius),
    }
}

/// Text / icon colors for a row given highlight and variant.
pub(super) fn item_colors(
    style: DropdownMenuContentStyle,
    variant: MenuItemVariant,
    highlighted: bool,
    disabled: bool,
) -> (Color, Color, Color) {
    let (text, muted, icon) = match (variant, highlighted) {
        (MenuItemVariant::Destructive, true) => (
            style.destructive_text,
            style.destructive_text,
            style.destructive_text,
        ),
        (MenuItemVariant::Destructive, false) => (
            style.destructive_text,
            style.muted_color,
            style.destructive_text,
        ),
        (MenuItemVariant::Default, true) => (
            style.item_highlight_text,
            style.item_highlight_text,
            style.item_highlight_text,
        ),
        (MenuItemVariant::Default, false) | (_, false) => {
            (style.text_color, style.muted_color, style.text_color)
        }
        (_, true) => (
            style.item_highlight_text,
            style.item_highlight_text,
            style.item_highlight_text,
        ),
    };

    if disabled {
        (
            text.scale_alpha(style.item_disabled_opacity),
            muted.scale_alpha(style.item_disabled_opacity),
            icon.scale_alpha(style.item_disabled_opacity),
        )
    } else {
        (text, muted, icon)
    }
}

/// Highlight fill for a row.
pub(super) fn item_highlight_fill(
    style: DropdownMenuContentStyle,
    variant: MenuItemVariant,
    highlighted: bool,
) -> Option<Color> {
    if !highlighted {
        return None;
    }

    Some(match variant {
        MenuItemVariant::Destructive => style.destructive_highlight_background,
        MenuItemVariant::Default | _ => style.item_highlight_background,
    })
}
