//! Command recipes from `.cn-command*` across style packs.
//!
//! Geometry and typography only — colors stay on backend palettes
//! (`bg-popover`, `text-muted-foreground`, `bg-muted` for selected rows).

use crate::style::StyleId;

use super::{ComponentRadius, FontWeight, PopoverShadow, TypeRecipe};

/// Maximum list height before scroll (`max-h-72` → 288).
pub const COMMAND_LIST_MAX_HEIGHT_PX: f32 = 288.0;

/// Default dialog vertical anchor (`top-1/3`).
pub const COMMAND_DIALOG_VERTICAL_ANCHOR: f32 = 1.0 / 3.0;

/// `disabled:opacity-50` / `data-[disabled=true]:opacity-50`.
pub const COMMAND_DISABLED_OPACITY: f32 = 0.5;

/// Search-icon opacity (`opacity-50`).
pub const COMMAND_INPUT_ICON_OPACITY: f32 = 0.5;

/// Geometry + typography recipe for `.cn-command-*` slots.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CommandRecipe {
    /// Root surface padding (`p-1` → 4, Lyra `0`).
    pub pad_px: f32,
    /// Root corner radius intent.
    pub radius: ComponentRadius,
    /// Whether the inline root paints a hairline ring / border.
    pub show_border: bool,
    /// Drop shadow of the inline surface (`shadow-md` packs).
    pub shadow: Option<PopoverShadow>,

    /// Input wrapper padding (`p-1 pb-0` → top/sides 4, bottom 0).
    pub input_wrapper_pad_px: f32,
    /// Whether the input wrapper paints a bottom hairline (`border-b`, Lyra).
    pub input_wrapper_border_bottom: bool,
    /// Input-group height (`h-8` → 32, `h-9` → 36).
    pub input_height_px: f32,
    /// Input-group corner radius.
    pub input_radius: ComponentRadius,
    /// `bg-input/N` alpha for the input group fill.
    pub input_fill_alpha: f32,
    /// Whether the input group paints a resting border.
    pub input_bordered: bool,
    /// Sera underline-only input chrome.
    pub input_underline_only: bool,
    /// Input text typography.
    pub input_typography: TypeRecipe,
    /// Search icon edge (`size-4` → 16, Mira/Sera `size-3.5` → 14).
    pub input_icon_size_px: f32,

    /// List max height (`max-h-72`).
    pub list_max_height_px: f32,
    /// List vertical scroll padding (`scroll-py-1` → 4).
    pub list_scroll_pad_y_px: f32,

    /// Empty-state vertical padding (`py-6` → 24).
    pub empty_pad_y_px: f32,
    /// Empty-state typography.
    pub empty_typography: TypeRecipe,

    /// Group padding (`p-1` → 4, Luma/Sera `p-1.5` → 6, Lyra `0`).
    pub group_pad_px: f32,
    /// Group heading typography.
    pub heading_typography: TypeRecipe,
    /// Heading horizontal padding.
    pub heading_pad_x_px: f32,
    /// Heading vertical padding.
    pub heading_pad_y_px: f32,

    /// Separator vertical margin (`my-*`).
    pub separator_margin_y_px: f32,
    /// Separator horizontal bleed (`-mx-*`).
    pub separator_margin_x_px: f32,
    /// Separator alpha multiplier on `border` (`bg-border/50` → 0.5).
    pub separator_alpha: f32,

    /// Item horizontal padding.
    pub item_pad_x_px: f32,
    /// Item vertical padding.
    pub item_pad_y_px: f32,
    /// Item gap between icon and label (`gap-2` → 8).
    pub item_gap_px: f32,
    /// Item corner radius (inline / non-dialog).
    pub item_radius: ComponentRadius,
    /// Item corner radius inside a dialog (`in-data-[slot=dialog-content]:rounded-*`).
    pub item_radius_in_dialog: ComponentRadius,
    /// Item body typography.
    pub item_typography: TypeRecipe,
    /// Leading icon edge.
    pub item_icon_size_px: f32,
    /// Minimum item height when set (`min-h-7` → 28).
    pub item_min_height_px: Option<f32>,

    /// Shortcut typography (`.cn-command-shortcut`).
    pub shortcut_typography: TypeRecipe,
}

/// Resolves `.cn-command-*` tokens for `style`.
pub const fn command_recipe(style: StyleId) -> CommandRecipe {
    match style {
        StyleId::Vega => VEGA,
        // `rounded-xl!`; input `h-8 rounded-lg`; items dialog `rounded-lg!`.
        StyleId::Nova => CommandRecipe {
            radius: ComponentRadius::Xl,
            input_radius: ComponentRadius::Lg,
            item_radius_in_dialog: ComponentRadius::Lg,
            ..VEGA
        },
        // `rounded-4xl`; input `h-9` inherits input-group `rounded-4xl`.
        StyleId::Maia => CommandRecipe {
            radius: ComponentRadius::S4xl,
            input_height_px: 36.0,
            input_radius: ComponentRadius::S4xl,
            heading_pad_x_px: 12.0,
            heading_pad_y_px: 8.0,
            separator_margin_y_px: 4.0,
            separator_margin_x_px: 0.0,
            separator_alpha: 0.5,
            item_pad_x_px: 12.0,
            item_pad_y_px: 8.0,
            item_radius: ComponentRadius::Lg,
            item_radius_in_dialog: ComponentRadius::S2xl,
            ..VEGA
        },
        // Square, no root pad; input wrapper `border-b`; `text-xs` everywhere.
        StyleId::Lyra => CommandRecipe {
            pad_px: 0.0,
            radius: ComponentRadius::None,
            input_wrapper_pad_px: 0.0,
            input_wrapper_border_bottom: true,
            input_radius: ComponentRadius::None,
            input_bordered: false,
            input_typography: text_xs(FontWeight::Normal),
            list_scroll_pad_y_px: 0.0,
            empty_typography: text_xs(FontWeight::Normal),
            group_pad_px: 0.0,
            heading_typography: text_xs(FontWeight::Normal),
            separator_margin_y_px: 0.0,
            item_pad_y_px: 8.0,
            item_radius: ComponentRadius::None,
            item_radius_in_dialog: ComponentRadius::None,
            item_typography: text_xs(FontWeight::Normal),
            ..VEGA
        },
        // Compact: input inherits input-group `rounded-md`; `bg-input/20`.
        StyleId::Mira => CommandRecipe {
            radius: ComponentRadius::Xl,
            input_radius: ComponentRadius::Md,
            input_fill_alpha: 0.2,
            input_typography: text_xs_relaxed(FontWeight::Normal),
            input_icon_size_px: 14.0,
            empty_typography: text_xs_relaxed(FontWeight::Normal),
            heading_pad_x_px: 10.0,
            separator_margin_y_px: 4.0,
            separator_alpha: 0.5,
            item_pad_x_px: 10.0,
            item_radius: ComponentRadius::Md,
            item_radius_in_dialog: ComponentRadius::Md,
            item_typography: text_xs_relaxed(FontWeight::Normal),
            item_icon_size_px: 14.0,
            item_min_height_px: Some(28.0),
            shortcut_typography: TypeRecipe {
                size_px: 10.0,
                weight: FontWeight::Normal,
                uppercase: false,
                tracking_em: 0.1,
                line_height_px: 14.0,
            },
            ..VEGA
        },
        // Soft: input inherits input-group `rounded-4xl`; `bg-input/50 h-9`.
        StyleId::Luma => CommandRecipe {
            radius: ComponentRadius::S4xl,
            input_height_px: 36.0,
            input_radius: ComponentRadius::S4xl,
            input_fill_alpha: 0.5,
            input_bordered: false,
            group_pad_px: 6.0,
            heading_pad_x_px: 12.0,
            heading_pad_y_px: 8.0,
            separator_margin_y_px: 6.0,
            separator_margin_x_px: 0.0,
            separator_alpha: 0.5,
            item_pad_x_px: 12.0,
            item_pad_y_px: 8.0,
            item_radius: ComponentRadius::S2xl,
            item_radius_in_dialog: ComponentRadius::S3xl,
            item_typography: text_sm(FontWeight::Medium),
            ..VEGA
        },
        // Underline input; square items; uppercase headings.
        // `.cn-command` is only `bg-popover text-popover-foreground` (no pad/radius/ring).
        StyleId::Sera => CommandRecipe {
            pad_px: 0.0,
            radius: ComponentRadius::None,
            input_wrapper_pad_px: 4.0,
            input_height_px: 36.0,
            input_radius: ComponentRadius::None,
            input_fill_alpha: 0.0,
            input_bordered: false,
            input_underline_only: true,
            input_icon_size_px: 14.0,
            group_pad_px: 6.0,
            heading_typography: TypeRecipe {
                size_px: 12.0,
                weight: FontWeight::Semibold,
                uppercase: true,
                tracking_em: 0.05,
                line_height_px: 16.0,
            },
            heading_pad_x_px: 12.0,
            heading_pad_y_px: 8.0,
            separator_margin_y_px: 6.0,
            separator_margin_x_px: 6.0,
            separator_alpha: 0.5,
            item_pad_x_px: 12.0,
            item_pad_y_px: 8.0,
            item_radius: ComponentRadius::None,
            item_radius_in_dialog: ComponentRadius::None,
            item_icon_size_px: 14.0,
            ..VEGA
        },
        // Soft rounded: input inherits input-group `rounded-2xl`; `bg-input/50`.
        StyleId::Rhea => CommandRecipe {
            radius: ComponentRadius::S3xl,
            input_radius: ComponentRadius::S2xl,
            input_fill_alpha: 0.5,
            input_bordered: false,
            separator_margin_y_px: 4.0,
            separator_margin_x_px: 0.0,
            separator_alpha: 0.5,
            item_radius: ComponentRadius::Xl,
            item_radius_in_dialog: ComponentRadius::S2xl,
            item_min_height_px: Some(28.0),
            ..VEGA
        },
    }
}

/// Vega `.cn-command-*` used as the fallback for unknown future packs.
///
/// `.cn-command` itself has no ring/border/shadow across packs — the docs
/// inline demo wraps it in `Card.Root` (`ring-1`) for the outer chrome.
const VEGA: CommandRecipe = CommandRecipe {
    pad_px: 4.0,
    radius: ComponentRadius::Xl,
    show_border: false,
    shadow: None,
    input_wrapper_pad_px: 4.0,
    input_wrapper_border_bottom: false,
    input_height_px: 32.0,
    input_radius: ComponentRadius::Lg,
    input_fill_alpha: 0.3,
    input_bordered: true,
    input_underline_only: false,
    input_typography: text_sm(FontWeight::Normal),
    input_icon_size_px: 16.0,
    list_max_height_px: COMMAND_LIST_MAX_HEIGHT_PX,
    list_scroll_pad_y_px: 4.0,
    empty_pad_y_px: 24.0,
    empty_typography: text_sm(FontWeight::Normal),
    group_pad_px: 4.0,
    heading_typography: text_xs(FontWeight::Medium),
    heading_pad_x_px: 8.0,
    heading_pad_y_px: 6.0,
    separator_margin_y_px: 0.0,
    separator_margin_x_px: 4.0,
    separator_alpha: 1.0,
    item_pad_x_px: 8.0,
    item_pad_y_px: 6.0,
    item_gap_px: 8.0,
    item_radius: ComponentRadius::Sm,
    item_radius_in_dialog: ComponentRadius::Lg,
    item_typography: text_sm(FontWeight::Normal),
    item_icon_size_px: 16.0,
    item_min_height_px: None,
    shortcut_typography: TypeRecipe {
        size_px: 12.0,
        weight: FontWeight::Normal,
        uppercase: false,
        tracking_em: 0.1,
        line_height_px: 16.0,
    },
};

const fn text_xs(weight: FontWeight) -> TypeRecipe {
    TypeRecipe {
        size_px: 12.0,
        weight,
        uppercase: false,
        tracking_em: 0.0,
        line_height_px: 16.0,
    }
}

const fn text_xs_relaxed(weight: FontWeight) -> TypeRecipe {
    TypeRecipe {
        size_px: 12.0,
        weight,
        uppercase: false,
        tracking_em: 0.0,
        line_height_px: 18.0,
    }
}

const fn text_sm(weight: FontWeight) -> TypeRecipe {
    TypeRecipe {
        size_px: 14.0,
        weight,
        uppercase: false,
        tracking_em: 0.0,
        line_height_px: 20.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_pack_resolves_a_recipe() {
        for style in [
            StyleId::Vega,
            StyleId::Nova,
            StyleId::Maia,
            StyleId::Lyra,
            StyleId::Mira,
            StyleId::Luma,
            StyleId::Sera,
            StyleId::Rhea,
        ] {
            let recipe = command_recipe(style);
            assert!(recipe.list_max_height_px > 0.0);
            assert!(recipe.input_height_px > 0.0);
        }
    }

    #[test]
    fn lyra_is_square_and_sera_is_underline() {
        let lyra = command_recipe(StyleId::Lyra);
        assert_eq!(lyra.radius, ComponentRadius::None);
        assert!(lyra.input_wrapper_border_bottom);

        let sera = command_recipe(StyleId::Sera);
        assert!(sera.input_underline_only);
        assert!(sera.heading_typography.uppercase);
    }

    #[test]
    fn mira_is_compact() {
        let mira = command_recipe(StyleId::Mira);
        assert_eq!(mira.input_icon_size_px, 14.0);
        assert_eq!(mira.item_min_height_px, Some(28.0));
        assert_eq!(mira.shortcut_typography.size_px, 10.0);
    }

    #[test]
    fn surface_radii_match_shadcn_svelte_css() {
        // `.cn-command` / `.cn-command-dialog` radius (same token per pack).
        assert_eq!(command_recipe(StyleId::Vega).radius, ComponentRadius::Xl);
        assert_eq!(command_recipe(StyleId::Nova).radius, ComponentRadius::Xl);
        assert_eq!(command_recipe(StyleId::Maia).radius, ComponentRadius::S4xl);
        assert_eq!(command_recipe(StyleId::Luma).radius, ComponentRadius::S4xl);
        assert_eq!(command_recipe(StyleId::Rhea).radius, ComponentRadius::S3xl);
        assert_eq!(command_recipe(StyleId::Mira).radius, ComponentRadius::Xl);
        assert_eq!(command_recipe(StyleId::Lyra).radius, ComponentRadius::None);
        assert_eq!(command_recipe(StyleId::Sera).radius, ComponentRadius::None);

        // Item radii: inline vs `in-data-[slot=dialog-content]:…`.
        let luma = command_recipe(StyleId::Luma);
        assert_eq!(luma.item_radius, ComponentRadius::S2xl);
        assert_eq!(luma.item_radius_in_dialog, ComponentRadius::S3xl);

        let maia = command_recipe(StyleId::Maia);
        assert_eq!(maia.item_radius, ComponentRadius::Lg);
        assert_eq!(maia.item_radius_in_dialog, ComponentRadius::S2xl);

        let rhea = command_recipe(StyleId::Rhea);
        assert_eq!(rhea.item_radius, ComponentRadius::Xl);
        assert_eq!(rhea.item_radius_in_dialog, ComponentRadius::S2xl);
    }
}
