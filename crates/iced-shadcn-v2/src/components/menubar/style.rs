//! Mapping of `.cn-menubar-*` style-pack rules to resolved iced visuals.

use crate::iced_compat::{Color, Shadow, Vector};

use shadcn_common::{
    MENUBAR_DESTRUCTIVE_FOCUS_ALPHA, MENUBAR_DESTRUCTIVE_FOCUS_ALPHA_DARK,
    MENUBAR_DISABLED_OPACITY, MenuItemVariant, MenubarRecipe,
};
use twill_core::prelude::theme::SemanticColor;

use crate::recipes::component_radius_px;
use crate::theme::Theme;

/// Resolved visuals of the persistent menubar bar.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MenubarBarStyle {
    /// Bar fill (`bg-background`).
    pub background: Color,
    /// Bar border (`border`).
    pub border_color: Color,
    /// Border width.
    pub border_width: f32,
    /// Bar corner radius in px.
    pub radius: f32,
    /// Optional bar drop shadow.
    pub shadow: Option<Shadow>,
    /// Trigger hover / expanded fill (`bg-muted`).
    pub trigger_muted: Color,
    /// Trigger text color.
    pub trigger_text: Color,
    /// Disabled opacity multiplier.
    pub disabled_opacity: f32,
}

/// Resolved visuals of the open menu content surface.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MenubarContentStyle {
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
    /// Plain item corner radius in px.
    pub item_radius: f32,
    /// Checkbox / radio item corner radius in px.
    pub checkable_item_radius: f32,
}

/// `.cn-menubar-*` numbers of the active pack.
pub(super) fn recipe(theme: &Theme) -> MenubarRecipe {
    theme.style.menubar()
}

/// Resolves the bar palette for the active theme.
pub(super) fn resolve_bar_style(theme: &Theme) -> MenubarBarStyle {
    let pack = recipe(theme);
    let shadow = pack.bar_shadow.map(|tokens| Shadow {
        color: Color::BLACK.scale_alpha(tokens.alpha),
        offset: Vector::new(0.0, tokens.offset_y_px),
        blur_radius: tokens.blur_px,
    });

    MenubarBarStyle {
        background: theme.palette.background,
        border_color: theme.palette.border,
        border_width: 1.0,
        radius: component_radius_px(theme, pack.bar_radius),
        shadow,
        trigger_muted: theme.semantic_color(SemanticColor::Muted),
        trigger_text: theme.palette.foreground,
        disabled_opacity: MENUBAR_DISABLED_OPACITY,
    }
}

/// Resolves the content / item palette for the active theme.
pub(super) fn resolve_content_style(theme: &Theme, submenu: bool) -> MenubarContentStyle {
    let pack = recipe(theme);
    let menu = pack.menu;
    // Root content hardcodes `ring-foreground/10` in menubar-content.svelte.
    // Sub-content ring comes from pack `.cn-menubar-sub-content` via the same
    // alphas stored on the menu recipe for packs that differ in dark mode.
    let ring_alpha = if theme.is_dark() {
        menu.content_ring_alpha_dark
    } else {
        menu.content_ring_alpha
    };
    let shadow_tokens = if submenu {
        menu.sub_content_shadow
    } else {
        menu.content_shadow
    };
    let radius_intent = if submenu {
        pack.sub_content_radius
    } else {
        menu.content_radius
    };
    let destructive = theme.semantic_color(SemanticColor::Destructive);
    let destructive_focus_alpha = if theme.is_dark() {
        MENUBAR_DESTRUCTIVE_FOCUS_ALPHA_DARK
    } else {
        MENUBAR_DESTRUCTIVE_FOCUS_ALPHA
    };
    let border = theme.semantic_color(SemanticColor::Border);
    let separator_color = if menu.separator_muted {
        border.scale_alpha(0.5)
    } else {
        border
    };

    MenubarContentStyle {
        background: theme.palette.popover,
        text_color: theme.palette.popover_foreground,
        muted_color: theme.semantic_color(SemanticColor::MutedForeground),
        border_color: theme.palette.foreground.scale_alpha(ring_alpha),
        border_width: 1.0,
        radius: component_radius_px(theme, radius_intent),
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
        item_disabled_opacity: MENUBAR_DISABLED_OPACITY,
        item_radius: component_radius_px(theme, menu.item_radius),
        checkable_item_radius: component_radius_px(theme, pack.checkable_item_radius),
    }
}

/// Text / icon colors for a row given highlight and variant.
pub(super) fn item_colors(
    style: MenubarContentStyle,
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
    style: MenubarContentStyle,
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
