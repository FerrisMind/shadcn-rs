//! Menubar recipes matching the **actual** shadcn-svelte component wiring
//! under `docs/src/lib/registry/ui/menubar/`, not the unused `.cn-menubar-content`
//! rules alone.
//!
//! ## How styles compose in shadcn-svelte (source of truth)
//!
//! | Part | Component class wiring | Effective geometry |
//! | --- | --- | --- |
//! | Bar | `cn-menubar` | pack `.cn-menubar` |
//! | Trigger | `cn-menubar-trigger` | pack `.cn-menubar-trigger` |
//! | **Content** | **no** `cn-menubar-content` — hardcodes `min-w-36 rounded-lg p-1 shadow-md ring-foreground/10` in `menubar-content.svelte` (identical in every pack’s `registry/styles/*/menubar.json`) | **always** `p-1` / `rounded-lg` / `min-w-36` |
//! | Items / labels / checkables | `cn-menubar-item` / … | pack `.cn-menubar-*` |
//! | Separator | `cn-menubar-separator` **plus** hardcode `-mx-1 my-1` in `menubar-separator.svelte` | margins **4**; color from pack |
//! | Sub-content | `cn-menubar-sub-content` | pack `.cn-menubar-sub-content` |
//!
//! Item / label tokens start from [`dropdown_menu_recipe`] and only override
//! menubar-specific deltas. Content-panel tokens are **not** taken from
//! dropdown / `.cn-menubar-content` — they follow the Svelte hardcode.

use crate::style::StyleId;

use super::{
    ComponentRadius, DropdownMenuRecipe, FontWeight, PopoverShadow, TypeRecipe,
    dropdown_menu_recipe,
};

/// Default `sideOffset` of the shadcn-svelte menubar content (`8px`).
pub const MENUBAR_SIDE_OFFSET_PX: f32 = 8.0;

/// Default `alignOffset` of the shadcn-svelte menubar content (`-4px`).
pub const MENUBAR_ALIGN_OFFSET_PX: f32 = -4.0;

/// Duration of the menubar open/close animation (`duration-100`).
pub const MENUBAR_ANIMATION_MS: u64 = 100;

/// Distance covered by the `slide-in-from-*-2` entrance animation.
pub const MENUBAR_SLIDE_PX: f32 = 8.0;

/// Initial scale of the `zoom-in-95` entrance animation.
pub const MENUBAR_ZOOM_FROM: f32 = 0.95;

/// `data-disabled:opacity-50` on items / the disabled root.
pub const MENUBAR_DISABLED_OPACITY: f32 = 0.5;

/// Maximum content height before scrolling (approximated as `max-h-96`).
pub const MENUBAR_CONTENT_MAX_HEIGHT_PX: f32 = 384.0;

/// Light-mode destructive item focus fill (`focus:bg-destructive/10`).
pub const MENUBAR_DESTRUCTIVE_FOCUS_ALPHA: f32 = 0.10;

/// Dark-mode destructive item focus fill (`dark:focus:bg-destructive/20`).
pub const MENUBAR_DESTRUCTIVE_FOCUS_ALPHA_DARK: f32 = 0.20;

/// Hardcoded panel tokens from `menubar-content.svelte` (all packs).
const CONTENT_MIN_WIDTH_PX: f32 = 144.0; // `min-w-36`
const CONTENT_PAD_PX: f32 = 4.0; // `p-1`
const CONTENT_RING_ALPHA: f32 = 0.10; // `ring-foreground/10`
/// Hardcoded separator margins from `menubar-separator.svelte` (`-mx-1 my-1`).
const SEPARATOR_MARGIN_PX: f32 = 4.0;

/// Geometry + typography recipe for the menubar as wired in shadcn-svelte.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MenubarRecipe {
    /// Bar height (`h-9` → 36).
    pub bar_height_px: f32,
    /// Gap between triggers (`gap-1` → 4). `0` when the pack omits `gap-*`.
    pub bar_gap_px: f32,
    /// Uniform bar padding (`p-1` → 4).
    pub bar_pad_px: f32,
    /// Bar corner radius intent.
    pub bar_radius: ComponentRadius,
    /// Optional bar drop shadow (`shadow-xs`); `None` when the pack omits it.
    pub bar_shadow: Option<PopoverShadow>,
    /// Trigger horizontal padding (`px-2` → 8).
    pub trigger_pad_x_px: f32,
    /// Trigger vertical padding (`py-1` → 4).
    pub trigger_pad_y_px: f32,
    /// Trigger corner radius intent.
    pub trigger_radius: ComponentRadius,
    /// Trigger label typography.
    pub trigger_typography: TypeRecipe,
    /// Root content + item / label / separator tokens.
    ///
    /// `content_*` here always mirror `menubar-content.svelte` hardcode.
    /// Item metrics come from pack `.cn-menubar-item` (via dropdown deltas).
    pub menu: DropdownMenuRecipe,
    /// Sub-content viewport padding from `.cn-menubar-sub-content`.
    pub sub_content_pad_px: f32,
    /// Sub-content corner radius from `.cn-menubar-sub-content`.
    pub sub_content_radius: ComponentRadius,
    /// Checkbox / radio item corner radius (`rounded-md` in Vega).
    pub checkable_item_radius: ComponentRadius,
    /// Whether checkbox / radio indicators sit on the leading edge.
    pub indicator_leading: bool,
    /// Distance from the item's leading edge to the indicator (`left-2` → 8).
    pub item_indicator_left_px: f32,
}

/// Resolves menubar tokens for `style` as the Svelte components actually wire them.
#[must_use]
pub const fn menubar_recipe(style: StyleId) -> MenubarRecipe {
    match style {
        StyleId::Vega => VEGA,
        StyleId::Nova => {
            let dropdown = dropdown_menu_recipe(StyleId::Nova);
            MenubarRecipe {
                bar_height_px: 32.0,
                bar_gap_px: 2.0,
                bar_pad_px: 3.0,
                bar_radius: ComponentRadius::Lg,
                bar_shadow: None,
                trigger_pad_x_px: 6.0,
                trigger_pad_y_px: 2.0,
                trigger_radius: ComponentRadius::Sm,
                trigger_typography: text_sm(FontWeight::Medium),
                menu: root_content(DropdownMenuRecipe {
                    item_indicator_pad_right_px: dropdown.item_inset_pad_left_px,
                    label_typography: text_sm(FontWeight::Medium),
                    sub_content_min_width_px: 128.0,
                    sub_content_shadow: PopoverShadow::LG,
                    ..dropdown
                }),
                // `.cn-menubar-sub-content`: `min-w-32 rounded-lg p-1 shadow-lg`
                sub_content_pad_px: 4.0,
                sub_content_radius: ComponentRadius::Lg,
                checkable_item_radius: ComponentRadius::Md,
                item_indicator_left_px: 6.0,
                indicator_leading: true,
            }
        }
        StyleId::Maia => {
            let dropdown = dropdown_menu_recipe(StyleId::Maia);
            MenubarRecipe {
                bar_height_px: 36.0,
                bar_gap_px: 0.0,
                bar_pad_px: 4.0,
                bar_radius: ComponentRadius::S4xl,
                bar_shadow: None,
                trigger_pad_x_px: 8.0,
                trigger_pad_y_px: 3.0,
                trigger_radius: ComponentRadius::Xl,
                trigger_typography: text_sm(FontWeight::Medium),
                menu: root_content(DropdownMenuRecipe {
                    item_indicator_pad_right_px: dropdown.item_inset_pad_left_px,
                    label_pad_x_px: 14.0,
                    label_pad_y_px: 10.0,
                    sub_content_min_width_px: 128.0,
                    sub_content_shadow: PopoverShadow::XXL,
                    ..dropdown
                }),
                // `.cn-menubar-sub-content`: `rounded-2xl p-1 shadow-2xl`
                sub_content_pad_px: 4.0,
                sub_content_radius: ComponentRadius::S2xl,
                checkable_item_radius: ComponentRadius::Xl,
                item_indicator_left_px: 12.0,
                indicator_leading: true,
            }
        }
        StyleId::Lyra => {
            let dropdown = dropdown_menu_recipe(StyleId::Lyra);
            MenubarRecipe {
                bar_height_px: 32.0,
                bar_gap_px: 2.0,
                bar_pad_px: 4.0,
                bar_radius: ComponentRadius::None,
                bar_shadow: None,
                trigger_pad_x_px: 6.0,
                trigger_pad_y_px: 3.2,
                trigger_radius: ComponentRadius::None,
                trigger_typography: text_xs(FontWeight::Medium),
                menu: root_content(DropdownMenuRecipe {
                    // Root content still hardcodes `p-1` (cn-menubar-content is unused).
                    // Items: menubar `data-inset:pl-8` (dropdown Lyra uses `pl-7`).
                    item_inset_pad_left_px: 32.0,
                    item_indicator_pad_right_px: 32.0,
                    label_inset_pad_left_px: 32.0,
                    item_indicator_right_px: 8.0,
                    sub_content_min_width_px: 128.0,
                    sub_content_shadow: PopoverShadow::LG,
                    ..dropdown
                }),
                // `.cn-menubar-sub-content`: no pack padding → 0, `rounded-none`
                sub_content_pad_px: 0.0,
                sub_content_radius: ComponentRadius::None,
                checkable_item_radius: ComponentRadius::None,
                item_indicator_left_px: 6.0,
                indicator_leading: true,
            }
        }
        StyleId::Mira => {
            let dropdown = dropdown_menu_recipe(StyleId::Mira);
            MenubarRecipe {
                bar_height_px: 36.0,
                bar_gap_px: 0.0,
                bar_pad_px: 4.0,
                bar_radius: ComponentRadius::Lg,
                bar_shadow: None,
                trigger_pad_x_px: 8.0,
                trigger_pad_y_px: 3.4,
                trigger_radius: ComponentRadius::Sm,
                trigger_typography: TypeRecipe {
                    size_px: 12.0,
                    weight: FontWeight::Medium,
                    uppercase: false,
                    tracking_em: 0.0,
                    line_height_px: 18.0,
                },
                menu: root_content(DropdownMenuRecipe {
                    item_indicator_pad_right_px: dropdown.item_inset_pad_left_px,
                    item_indicator_size_px: 16.0,
                    sub_content_min_width_px: 128.0,
                    sub_content_shadow: PopoverShadow::MD,
                    ..dropdown
                }),
                // `.cn-menubar-sub-content`: `rounded-lg p-1 shadow-md`
                sub_content_pad_px: 4.0,
                sub_content_radius: ComponentRadius::Lg,
                checkable_item_radius: ComponentRadius::Md,
                item_indicator_left_px: 8.0,
                indicator_leading: true,
            }
        }
        StyleId::Luma => {
            let dropdown = dropdown_menu_recipe(StyleId::Luma);
            MenubarRecipe {
                bar_height_px: 36.0,
                bar_gap_px: 0.0,
                bar_pad_px: 4.0,
                bar_radius: ComponentRadius::S3xl,
                bar_shadow: None,
                trigger_pad_x_px: 8.0,
                trigger_pad_y_px: 3.0,
                trigger_radius: ComponentRadius::S2xl,
                trigger_typography: text_sm(FontWeight::Medium),
                menu: root_content(DropdownMenuRecipe {
                    item_indicator_pad_right_px: dropdown.item_inset_pad_left_px,
                    label_pad_x_px: 14.0,
                    label_pad_y_px: 10.0,
                    sub_content_min_width_px: 128.0,
                    sub_content_shadow: PopoverShadow::LG,
                    ..dropdown
                }),
                // `.cn-menubar-sub-content`: `rounded-3xl p-1.5 shadow-lg`
                // (root content stays hardcoded `p-1` / `rounded-lg`).
                sub_content_pad_px: 6.0,
                sub_content_radius: ComponentRadius::S3xl,
                checkable_item_radius: ComponentRadius::S2xl,
                item_indicator_left_px: 12.0,
                indicator_leading: true,
            }
        }
        StyleId::Sera => {
            let dropdown = dropdown_menu_recipe(StyleId::Sera);
            MenubarRecipe {
                bar_height_px: 40.0,
                bar_gap_px: 0.0,
                bar_pad_px: 4.0,
                bar_radius: ComponentRadius::None,
                bar_shadow: None,
                trigger_pad_x_px: 8.0,
                trigger_pad_y_px: 3.0,
                trigger_radius: ComponentRadius::None,
                trigger_typography: text_sm(FontWeight::Medium),
                menu: root_content(DropdownMenuRecipe {
                    // Plain `.cn-menubar-item` is `text-sm` (not dropdown uppercase).
                    item_typography: text_sm(FontWeight::Normal),
                    item_indicator_pad_right_px: dropdown.item_inset_pad_left_px,
                    label_pad_x_px: 14.0,
                    label_pad_y_px: 8.0,
                    sub_content_min_width_px: 128.0,
                    sub_content_shadow: PopoverShadow::MD,
                    ..dropdown
                }),
                // `.cn-menubar-sub-content`: `rounded-none p-1.5`
                sub_content_pad_px: 6.0,
                sub_content_radius: ComponentRadius::None,
                checkable_item_radius: ComponentRadius::None,
                item_indicator_left_px: 12.0,
                indicator_leading: true,
            }
        }
        StyleId::Rhea => {
            let dropdown = dropdown_menu_recipe(StyleId::Rhea);
            MenubarRecipe {
                bar_height_px: 32.0,
                bar_gap_px: 0.0,
                bar_pad_px: 3.0,
                bar_radius: ComponentRadius::S2xl,
                bar_shadow: None,
                trigger_pad_x_px: 6.0,
                trigger_pad_y_px: 2.0,
                trigger_radius: ComponentRadius::S2xl,
                trigger_typography: text_sm(FontWeight::Medium),
                menu: root_content(DropdownMenuRecipe {
                    item_indicator_pad_right_px: dropdown.item_inset_pad_left_px,
                    item_indicator_right_px: 6.0,
                    label_typography: text_sm(FontWeight::Normal),
                    sub_content_min_width_px: 128.0,
                    sub_content_shadow: PopoverShadow::LG,
                    ..dropdown
                }),
                // `.cn-menubar-sub-content`: `rounded-2xl p-1 shadow-lg`
                sub_content_pad_px: 4.0,
                sub_content_radius: ComponentRadius::S2xl,
                checkable_item_radius: ComponentRadius::Xl,
                item_indicator_left_px: 6.0,
                indicator_leading: true,
            }
        }
    }
}

/// Forces root content tokens to `menubar-content.svelte` hardcode and
/// separator margins to `menubar-separator.svelte` (`-mx-1 my-1`).
const fn root_content(menu: DropdownMenuRecipe) -> DropdownMenuRecipe {
    DropdownMenuRecipe {
        content_min_width_px: CONTENT_MIN_WIDTH_PX,
        content_radius: ComponentRadius::Lg,
        content_pad_px: CONTENT_PAD_PX,
        content_ring_alpha: CONTENT_RING_ALPHA,
        content_ring_alpha_dark: CONTENT_RING_ALPHA,
        content_shadow: PopoverShadow::MD,
        separator_margin_y_px: SEPARATOR_MARGIN_PX,
        separator_margin_x_px: SEPARATOR_MARGIN_PX,
        ..menu
    }
}

/// Vega bar / trigger from `.cn-menubar*`; content from Svelte hardcode.
const VEGA: MenubarRecipe = MenubarRecipe {
    bar_height_px: 36.0,
    bar_gap_px: 4.0,
    bar_pad_px: 4.0,
    bar_radius: ComponentRadius::Md,
    bar_shadow: Some(PopoverShadow::XS),
    trigger_pad_x_px: 8.0,
    trigger_pad_y_px: 4.0,
    trigger_radius: ComponentRadius::Sm,
    trigger_typography: text_sm(FontWeight::Medium),
    menu: root_content(DropdownMenuRecipe {
        label_typography: text_sm(FontWeight::Medium),
        sub_content_min_width_px: 128.0,
        sub_content_shadow: PopoverShadow::LG,
        ..dropdown_menu_recipe(StyleId::Vega)
    }),
    // `.cn-menubar-sub-content`: `rounded-md p-1 shadow-lg`
    sub_content_pad_px: 4.0,
    sub_content_radius: ComponentRadius::Md,
    checkable_item_radius: ComponentRadius::Md,
    indicator_leading: true,
    item_indicator_left_px: 8.0,
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
    use crate::StyleId;

    #[test]
    fn every_pack_resolves_a_menubar_recipe() {
        for style in StyleId::ALL {
            let recipe = menubar_recipe(style);
            assert!(recipe.bar_height_px > 0.0);
            assert!(recipe.bar_pad_px > 0.0, "{style:?} bar must have padding");
            assert!(recipe.trigger_pad_x_px > 0.0);
            assert!(recipe.menu.content_min_width_px > 0.0);
            assert!(recipe.indicator_leading);
            assert!(recipe.item_indicator_left_px > 0.0);
            assert!(
                recipe.bar_height_px > recipe.bar_pad_px * 2.0,
                "{style:?} bar height must exceed padding"
            );
        }
    }

    #[test]
    fn root_content_matches_menubar_content_svelte_hardcode_for_every_pack() {
        // menubar-content.svelte does not apply `cn-menubar-content`; every
        // pack registry ships the same hardcoded class string.
        for style in StyleId::ALL {
            let recipe = menubar_recipe(style);
            assert_eq!(recipe.menu.content_min_width_px, 144.0, "{style:?}");
            assert_eq!(recipe.menu.content_pad_px, 4.0, "{style:?}");
            assert_eq!(recipe.menu.content_radius, ComponentRadius::Lg, "{style:?}");
            assert_eq!(recipe.menu.content_shadow, PopoverShadow::MD, "{style:?}");
            assert_eq!(recipe.menu.content_ring_alpha, 0.10, "{style:?}");
            assert_eq!(recipe.menu.content_ring_alpha_dark, 0.10, "{style:?}");
            assert_eq!(recipe.menu.separator_margin_y_px, 4.0, "{style:?}");
            assert_eq!(recipe.menu.separator_margin_x_px, 4.0, "{style:?}");
        }
    }

    #[test]
    fn root_content_pad_is_not_forced_to_match_dropdown() {
        // On Luma/Sera, dropdown content is `p-1.5` but menubar content hardcode
        // is `p-1` — this is how shadcn-svelte actually ships.
        let luma = menubar_recipe(StyleId::Luma);
        let dropdown = dropdown_menu_recipe(StyleId::Luma);
        assert_eq!(luma.menu.content_pad_px, 4.0);
        assert_eq!(dropdown.content_pad_px, 6.0);
        assert_eq!(luma.sub_content_pad_px, 6.0);
        assert_eq!(luma.sub_content_radius, ComponentRadius::S3xl);
    }

    #[test]
    fn vega_bar_and_items_match_pack_css() {
        let recipe = menubar_recipe(StyleId::Vega);
        assert_eq!(recipe.bar_height_px, 36.0);
        assert_eq!(recipe.bar_gap_px, 4.0);
        assert_eq!(recipe.bar_pad_px, 4.0);
        assert_eq!(recipe.bar_radius, ComponentRadius::Md);
        assert_eq!(recipe.trigger_pad_x_px, 8.0);
        assert_eq!(recipe.trigger_pad_y_px, 4.0);
        assert_eq!(recipe.menu.item_pad_x_px, 8.0);
        assert_eq!(recipe.menu.item_pad_y_px, 6.0);
        assert_eq!(recipe.menu.item_radius, ComponentRadius::Sm);
        assert_eq!(recipe.sub_content_radius, ComponentRadius::Md);
        assert_eq!(recipe.checkable_item_radius, ComponentRadius::Md);
        assert_eq!(MENUBAR_SIDE_OFFSET_PX, 8.0);
        assert_eq!(MENUBAR_ALIGN_OFFSET_PX, -4.0);
    }

    #[test]
    fn lyra_root_content_still_has_p1_hardcode() {
        let recipe = menubar_recipe(StyleId::Lyra);
        assert_eq!(recipe.menu.content_pad_px, 4.0);
        assert_eq!(recipe.sub_content_pad_px, 0.0);
        assert_eq!(recipe.bar_gap_px, 2.0);
        assert_eq!(recipe.menu.item_inset_pad_left_px, 32.0);
    }

    #[test]
    fn sera_plain_items_are_text_sm_not_uppercase() {
        let recipe = menubar_recipe(StyleId::Sera);
        assert_eq!(recipe.menu.item_typography.size_px, 14.0);
        assert!(!recipe.menu.item_typography.uppercase);
        assert_eq!(recipe.menu.content_pad_px, 4.0);
        assert_eq!(recipe.sub_content_pad_px, 6.0);
    }
}
