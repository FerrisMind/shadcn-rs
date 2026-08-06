//! Dropdown-menu recipes from `.cn-dropdown-menu-*` across style packs.
//!
//! Geometry and typography shared by iced and egui; colors stay with backend
//! palettes (`bg-popover`, `text-destructive`, `bg-accent`, …).

use crate::style::StyleId;

use super::{ComponentRadius, FontWeight, PopoverShadow, TypeRecipe};

/// Duration of the menu open/close animation (`duration-100`).
pub const DROPDOWN_MENU_ANIMATION_MS: u64 = 100;

/// Distance covered by the `slide-in-from-*-2` entrance animation.
pub const DROPDOWN_MENU_SLIDE_PX: f32 = 8.0;

/// Initial scale of the `zoom-in-95` entrance animation.
pub const DROPDOWN_MENU_ZOOM_FROM: f32 = 0.95;

/// Default `sideOffset` of the shadcn-svelte dropdown-menu content.
pub const DROPDOWN_MENU_SIDE_OFFSET_PX: f32 = 4.0;

/// Gap between a parent menu panel and its `SubContent` (`sideOffset`).
///
/// bits-ui `menu-sub-content.svelte` does not set `sideOffset`; it falls through
/// to `floating-layer-content.svelte` which defaults to `0`. shadcn-svelte
/// `*-sub-content.svelte` wrappers also leave it unset. Do **not** reuse the
/// root content `sideOffset` (4 / 8) for this gap.
pub const MENU_SUB_SIDE_OFFSET_PX: f32 = 0.0;

/// `data-disabled:opacity-50` on items / the disabled root.
pub const DROPDOWN_MENU_DISABLED_OPACITY: f32 = 0.5;

/// Maximum content height before scrolling (`max-h-(--bits-dropdown-menu-content-available-height)`
/// approximated as `max-h-96`).
pub const DROPDOWN_MENU_CONTENT_MAX_HEIGHT_PX: f32 = 384.0;

/// Light-mode destructive item focus fill (`focus:bg-destructive/10`).
pub const DROPDOWN_MENU_DESTRUCTIVE_FOCUS_ALPHA: f32 = 0.10;

/// Dark-mode destructive item focus fill (`dark:focus:bg-destructive/20`).
pub const DROPDOWN_MENU_DESTRUCTIVE_FOCUS_ALPHA_DARK: f32 = 0.20;

/// Visual variant of a menu item (`data-variant`).
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum MenuItemVariant {
    /// Default accent focus treatment.
    #[default]
    Default,
    /// Destructive / danger action (`text-destructive`).
    Destructive,
}

/// Kind of activateable menu row, used for close-on-select policy.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MenuActivateKind {
    /// Plain `DropdownMenu.Item` — closes the menu by default.
    Item,
    /// `DropdownMenu.CheckboxItem` — stays open (toggle).
    Checkbox,
    /// `DropdownMenu.RadioItem` — closes after pick (bits-ui default).
    Radio,
    /// `DropdownMenu.SubTrigger` — opens a nested menu, does not close root.
    SubTrigger,
}

impl MenuActivateKind {
    /// Whether activating this kind should close the root menu by default.
    #[must_use]
    pub const fn closes_menu_by_default(self) -> bool {
        match self {
            Self::Item | Self::Radio => true,
            Self::Checkbox | Self::SubTrigger => false,
        }
    }
}

/// Geometry + typography recipe for `.cn-dropdown-menu-*`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DropdownMenuRecipe {
    /// Content `min-w-*` in px.
    pub content_min_width_px: f32,
    /// Content corner radius intent.
    pub content_radius: ComponentRadius,
    /// Viewport padding (`p-1` → 4).
    pub content_pad_px: f32,
    /// `ring-foreground/N` alpha in light mode.
    pub content_ring_alpha: f32,
    /// `ring-foreground/N` alpha in dark mode.
    pub content_ring_alpha_dark: f32,
    /// Content drop shadow.
    pub content_shadow: PopoverShadow,
    /// Separator hairline uses `bg-border/50` instead of full `bg-border`.
    pub separator_muted: bool,

    /// Sub-content `min-w-*` in px (`min-w-[96px]` → 96).
    pub sub_content_min_width_px: f32,
    /// Sub-content drop shadow (usually `shadow-lg`).
    pub sub_content_shadow: PopoverShadow,

    /// Item horizontal padding (`px-2` → 8).
    pub item_pad_x_px: f32,
    /// Item vertical padding (`py-1.5` → 6).
    pub item_pad_y_px: f32,
    /// Extra left padding when `data-inset` (`pl-8` → 32).
    pub item_inset_pad_left_px: f32,
    /// Right padding reserved for checkbox/radio indicators (`pr-8` → 32).
    pub item_indicator_pad_right_px: f32,
    /// Gap between leading icon and label (`gap-2` → 8).
    pub item_gap_px: f32,
    /// Minimum item height (`min-h-7` → 28), or `0` when unset.
    pub item_min_height_px: f32,
    /// Item corner radius intent.
    pub item_radius: ComponentRadius,
    /// Item body typography.
    pub item_typography: TypeRecipe,
    /// Leading / trailing icon edge (`size-4` → 16).
    pub item_icon_size_px: f32,
    /// Indicator edge (`size-4` / `size-3.5`).
    pub item_indicator_size_px: f32,
    /// Distance from the item's end edge to the indicator (`right-2` → 8).
    pub item_indicator_right_px: f32,

    /// Group / section label typography (`.cn-dropdown-menu-label`).
    pub label_typography: TypeRecipe,
    /// Label horizontal padding.
    pub label_pad_x_px: f32,
    /// Label vertical padding.
    pub label_pad_y_px: f32,
    /// Label inset left padding when `data-inset`.
    pub label_inset_pad_left_px: f32,

    /// Separator vertical margin (`my-1` → 4). Equals [`Self::content_pad_px`]
    /// so the gap above/below the hairline matches the panel edge inset.
    pub separator_margin_y_px: f32,
    /// Separator horizontal bleed (`-mx-1` → 4). Equals [`Self::content_pad_px`]
    /// so the line spans edge-to-edge of the panel.
    pub separator_margin_x_px: f32,

    /// Shortcut typography (`.cn-dropdown-menu-shortcut`).
    pub shortcut_typography: TypeRecipe,
}

/// Resolves `.cn-dropdown-menu-*` tokens for `style`.
pub const fn dropdown_menu_recipe(style: StyleId) -> DropdownMenuRecipe {
    match style {
        StyleId::Vega => VEGA,
        // `rounded-lg`; items `gap-1.5 rounded-md px-1.5 py-1`; inset `pl-7`.
        StyleId::Nova => DropdownMenuRecipe {
            content_radius: ComponentRadius::Lg,
            item_pad_x_px: 6.0,
            item_pad_y_px: 4.0,
            item_inset_pad_left_px: 28.0,
            item_gap_px: 6.0,
            item_radius: ComponentRadius::Md,
            label_pad_x_px: 6.0,
            label_pad_y_px: 4.0,
            label_inset_pad_left_px: 28.0,
            sub_content_min_width_px: 96.0,
            ..VEGA
        },
        // `min-w-48 rounded-2xl shadow-2xl ring-foreground/5`; items
        // `gap-2.5 rounded-xl px-3 py-2`; inset `pl-9.5`.
        StyleId::Maia => DropdownMenuRecipe {
            content_min_width_px: 192.0,
            content_radius: ComponentRadius::S2xl,
            content_ring_alpha: 0.05,
            content_ring_alpha_dark: 0.10,
            content_shadow: PopoverShadow::XXL,
            item_pad_x_px: 12.0,
            item_pad_y_px: 8.0,
            item_inset_pad_left_px: 38.0,
            item_gap_px: 10.0,
            item_radius: ComponentRadius::Xl,
            label_pad_x_px: 12.0,
            label_pad_y_px: 8.0,
            label_inset_pad_left_px: 38.0,
            sub_content_min_width_px: 96.0,
            ..VEGA
        },
        // Square, compact type; content has no extra padding in CSS.
        // Separator is `h-px` with no `my-*` (gap matches the 0 content pad).
        StyleId::Lyra => DropdownMenuRecipe {
            content_radius: ComponentRadius::None,
            content_pad_px: 0.0,
            item_pad_y_px: 8.0,
            item_inset_pad_left_px: 28.0,
            item_radius: ComponentRadius::None,
            item_typography: text_xs(FontWeight::Normal),
            label_inset_pad_left_px: 28.0,
            separator_margin_y_px: 0.0,
            separator_margin_x_px: 0.0,
            sub_content_min_width_px: 96.0,
            ..VEGA
        },
        // Compact: `rounded-lg`, items `min-h-7 text-xs`, icons `size-3.5`.
        StyleId::Mira => DropdownMenuRecipe {
            content_radius: ComponentRadius::Lg,
            item_pad_y_px: 4.0,
            item_inset_pad_left_px: 30.0,
            item_min_height_px: 28.0,
            item_radius: ComponentRadius::Md,
            item_typography: text_xs(FontWeight::Normal),
            item_icon_size_px: 14.0,
            item_indicator_size_px: 14.0,
            label_typography: text_xs(FontWeight::Normal),
            label_inset_pad_left_px: 30.0,
            separator_muted: true,
            sub_content_min_width_px: 96.0,
            ..VEGA
        },
        // Soft rounded panel: `min-w-48 rounded-3xl p-1.5 shadow-lg`; items
        // `rounded-2xl font-medium`. (`Full` would paint a stadium pill —
        // wrong for a tall menu surface.)
        StyleId::Luma => DropdownMenuRecipe {
            content_min_width_px: 192.0,
            content_radius: ComponentRadius::S3xl,
            content_pad_px: 6.0,
            content_ring_alpha: 0.05,
            content_ring_alpha_dark: 0.10,
            content_shadow: PopoverShadow::LG,
            item_pad_x_px: 12.0,
            item_pad_y_px: 8.0,
            item_inset_pad_left_px: 38.0,
            item_gap_px: 10.0,
            item_radius: ComponentRadius::S2xl,
            item_typography: text_sm(FontWeight::Medium),
            label_pad_x_px: 12.0,
            label_pad_y_px: 8.0,
            label_inset_pad_left_px: 38.0,
            separator_muted: true,
            separator_margin_y_px: 6.0,
            separator_margin_x_px: 6.0,
            sub_content_min_width_px: 96.0,
            sub_content_shadow: PopoverShadow::LG,
            ..VEGA
        },
        // Underline / editorial: `min-w-48 rounded-none p-1.5`, uppercase
        // tracking, `size-3.5` icons.
        StyleId::Sera => DropdownMenuRecipe {
            content_min_width_px: 192.0,
            content_radius: ComponentRadius::None,
            content_pad_px: 6.0,
            item_pad_x_px: 12.0,
            item_pad_y_px: 8.0,
            item_inset_pad_left_px: 38.0,
            item_gap_px: 10.0,
            item_radius: ComponentRadius::None,
            item_typography: TypeRecipe {
                size_px: 12.0,
                weight: FontWeight::Medium,
                uppercase: true,
                tracking_em: 0.05,
                line_height_px: 16.0,
            },
            item_icon_size_px: 14.0,
            item_indicator_size_px: 14.0,
            label_typography: TypeRecipe {
                size_px: 12.0,
                weight: FontWeight::Semibold,
                uppercase: true,
                tracking_em: 0.05,
                line_height_px: 16.0,
            },
            label_pad_x_px: 12.0,
            label_pad_y_px: 8.0,
            label_inset_pad_left_px: 38.0,
            separator_muted: true,
            separator_margin_y_px: 6.0,
            separator_margin_x_px: 6.0,
            sub_content_min_width_px: 144.0,
            sub_content_shadow: PopoverShadow::MD,
            ..VEGA
        },
        // Soft rounded: `rounded-2xl shadow-lg ring-foreground/5`; items
        // `min-h-7 rounded-xl`; inset `pl-7`.
        StyleId::Rhea => DropdownMenuRecipe {
            content_radius: ComponentRadius::S2xl,
            content_ring_alpha: 0.05,
            content_ring_alpha_dark: 0.10,
            content_shadow: PopoverShadow::LG,
            item_inset_pad_left_px: 28.0,
            item_min_height_px: 28.0,
            item_radius: ComponentRadius::Xl,
            label_pad_y_px: 4.0,
            label_typography: text_xs(FontWeight::Normal),
            label_inset_pad_left_px: 28.0,
            separator_muted: true,
            sub_content_min_width_px: 96.0,
            sub_content_shadow: PopoverShadow::LG,
            ..VEGA
        },
    }
}

/// Vega `.cn-dropdown-menu-*` used as the fallback for unknown future packs.
const VEGA: DropdownMenuRecipe = DropdownMenuRecipe {
    content_min_width_px: 128.0,
    content_radius: ComponentRadius::Md,
    content_pad_px: 4.0,
    content_ring_alpha: 0.10,
    content_ring_alpha_dark: 0.10,
    content_shadow: PopoverShadow::MD,
    separator_muted: false,
    sub_content_min_width_px: 96.0,
    sub_content_shadow: PopoverShadow::LG,
    item_pad_x_px: 8.0,
    item_pad_y_px: 6.0,
    item_inset_pad_left_px: 32.0,
    item_indicator_pad_right_px: 32.0,
    item_gap_px: 8.0,
    item_min_height_px: 0.0,
    item_radius: ComponentRadius::Sm,
    item_typography: text_sm(FontWeight::Normal),
    item_icon_size_px: 16.0,
    item_indicator_size_px: 16.0,
    item_indicator_right_px: 8.0,
    label_typography: text_xs(FontWeight::Medium),
    label_pad_x_px: 8.0,
    label_pad_y_px: 6.0,
    label_inset_pad_left_px: 32.0,
    separator_margin_y_px: 4.0,
    separator_margin_x_px: 4.0,
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
    fn sub_content_side_offset_matches_bits_ui_floating_default() {
        // menu-sub-content.svelte leaves sideOffset unset → floating-layer = 0.
        assert_eq!(MENU_SUB_SIDE_OFFSET_PX, 0.0);
        assert_ne!(MENU_SUB_SIDE_OFFSET_PX, DROPDOWN_MENU_SIDE_OFFSET_PX);
    }

    #[test]
    fn every_pack_resolves_a_recipe() {
        for style in StyleId::ALL {
            let recipe = dropdown_menu_recipe(style);
            assert!(recipe.content_min_width_px > 0.0);
            assert!(recipe.sub_content_min_width_px > 0.0);
            assert!(recipe.item_indicator_pad_right_px >= recipe.item_indicator_size_px);
        }
    }

    #[test]
    fn vega_matches_web_baseline() {
        let recipe = dropdown_menu_recipe(StyleId::Vega);
        assert_eq!(recipe.content_min_width_px, 128.0);
        assert_eq!(recipe.content_pad_px, 4.0);
        assert_eq!(recipe.item_pad_x_px, 8.0);
        assert_eq!(recipe.item_pad_y_px, 6.0);
        assert_eq!(recipe.item_inset_pad_left_px, 32.0);
        assert_eq!(recipe.sub_content_min_width_px, 96.0);
        assert_eq!(recipe.content_shadow, PopoverShadow::MD);
        assert_eq!(recipe.sub_content_shadow, PopoverShadow::LG);
    }

    #[test]
    fn activate_kind_close_policy_matches_bits_ui() {
        assert!(MenuActivateKind::Item.closes_menu_by_default());
        assert!(MenuActivateKind::Radio.closes_menu_by_default());
        assert!(!MenuActivateKind::Checkbox.closes_menu_by_default());
        assert!(!MenuActivateKind::SubTrigger.closes_menu_by_default());
    }

    #[test]
    fn separator_margins_match_content_pad() {
        // shadcn: separator `my-*` / `-mx-*` equals content `p-*`, so the gap
        // from an item to the hairline matches the gap from an item to the
        // panel edge.
        for style in StyleId::ALL {
            let recipe = dropdown_menu_recipe(style);
            assert_eq!(
                recipe.separator_margin_y_px, recipe.content_pad_px,
                "{style:?} separator my must match content pad"
            );
            assert_eq!(
                recipe.separator_margin_x_px, recipe.content_pad_px,
                "{style:?} separator -mx must match content pad"
            );
        }
    }

    #[test]
    fn mira_is_compact_and_sera_is_uppercase() {
        let mira = dropdown_menu_recipe(StyleId::Mira);
        assert_eq!(mira.item_typography.size_px, 12.0);
        assert_eq!(mira.item_icon_size_px, 14.0);
        assert_eq!(mira.item_min_height_px, 28.0);

        let sera = dropdown_menu_recipe(StyleId::Sera);
        assert!(sera.item_typography.uppercase);
        assert_eq!(sera.content_radius, ComponentRadius::None);
        assert_eq!(sera.content_min_width_px, 192.0);
    }

    #[test]
    fn luma_uses_rounded_3xl_not_pill() {
        let luma = dropdown_menu_recipe(StyleId::Luma);
        assert_eq!(luma.content_radius, ComponentRadius::S3xl);
        assert_eq!(luma.item_radius, ComponentRadius::S2xl);
        assert_ne!(luma.content_radius, ComponentRadius::Full);
    }
}
