//! Context-menu recipes from `.cn-context-menu-*` across style packs.
//!
//! The shadcn-svelte context-menu shares its surface / item / label / separator
//! geometry with the dropdown-menu (`.cn-context-menu-content` resolves to the
//! same `.cn-menu-target` / `.cn-menu-translucent` rules), so the recipe type is
//! [`DropdownMenuRecipe`] and the per-pack table is reused via
//! [`dropdown_menu_recipe`]. What is context-menu specific is the trigger
//! gesture (secondary click), the cursor-anchored placement, and the explicit
//! `side` prop — covered by the `CONTEXT_MENU_*` constants below.
//!
//! Both iced and egui consume this module so their menu surfaces stay in sync.

use crate::style::StyleId;

use super::DropdownMenuRecipe;

/// Default `sideOffset` of the shadcn-svelte context-menu content (`4px`).
pub const CONTEXT_MENU_SIDE_OFFSET_PX: f32 = 4.0;

/// Extra slack added around the cursor anchor before flipping the side
/// (matches the implicit `--bits-context-menu-available-height` slack).
pub const CONTEXT_MENU_FLIP_SLACK_PX: f32 = 8.0;

/// Duration of the context-menu open/close animation (`duration-100`).
pub const CONTEXT_MENU_ANIMATION_MS: u64 = 100;

/// Distance covered by the `slide-in-from-*-2` entrance animation.
pub const CONTEXT_MENU_SLIDE_PX: f32 = 8.0;

/// Initial scale of the `zoom-in-95` entrance animation.
pub const CONTEXT_MENU_ZOOM_FROM: f32 = 0.95;

/// `data-disabled:opacity-50` on items / the disabled root.
pub const CONTEXT_MENU_DISABLED_OPACITY: f32 = 0.5;

/// Maximum content height before scrolling (`max-h-(--bits-context-menu-content-available-height)`
/// approximated as `max-h-96`).
pub const CONTEXT_MENU_CONTENT_MAX_HEIGHT_PX: f32 = 384.0;

/// Light-mode destructive item focus fill (`focus:bg-destructive/10`).
pub const CONTEXT_MENU_DESTRUCTIVE_FOCUS_ALPHA: f32 = 0.10;

/// Dark-mode destructive item focus fill (`dark:focus:bg-destructive/20`).
pub const CONTEXT_MENU_DESTRUCTIVE_FOCUS_ALPHA_DARK: f32 = 0.20;

/// Context-menu recipe — geometry + typography shared with dropdown-menu.
///
/// The shadcn-svelte context-menu uses the same `.cn-menu-target` surface and
/// item rules as the dropdown-menu, so the resolved recipe is identical. This
/// alias keeps the call sites self-documenting (`context_menu_recipe(style)`)
/// without inventing a parallel token set that would drift from the web CSS.
pub type ContextMenuRecipe = DropdownMenuRecipe;

/// Resolves `.cn-context-menu-*` tokens for `style`.
///
/// Mostly shared with [`dropdown_menu_recipe`](super::dropdown_menu_recipe).
/// Maia is the exception: dropdown content uses `dark:ring-foreground/10`,
/// while context-menu content stays at `ring-foreground/5` in both schemes.
#[must_use]
pub const fn context_menu_recipe(style: StyleId) -> ContextMenuRecipe {
    let recipe = super::dropdown_menu_recipe(style);
    match style {
        StyleId::Maia => ContextMenuRecipe {
            content_ring_alpha_dark: recipe.content_ring_alpha,
            ..recipe
        },
        _ => recipe,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::StyleId;

    #[test]
    fn every_pack_resolves_a_context_menu_recipe() {
        for style in StyleId::ALL {
            let recipe = context_menu_recipe(style);
            assert!(recipe.content_min_width_px > 0.0);
            assert!(recipe.sub_content_min_width_px > 0.0);
            assert!(recipe.item_pad_x_px > 0.0);
            assert!(recipe.item_indicator_pad_right_px >= recipe.item_indicator_size_px);
        }
    }

    #[test]
    fn context_menu_recipe_matches_dropdown_menu_recipe() {
        for style in StyleId::ALL {
            let context = context_menu_recipe(style);
            let dropdown = super::super::dropdown_menu_recipe(style);
            if matches!(style, StyleId::Maia) {
                assert_eq!(context.content_ring_alpha, dropdown.content_ring_alpha);
                assert_eq!(context.content_ring_alpha_dark, dropdown.content_ring_alpha);
                assert_ne!(
                    context.content_ring_alpha_dark,
                    dropdown.content_ring_alpha_dark
                );
            } else {
                assert_eq!(context, dropdown);
            }
        }
    }

    #[test]
    fn side_offset_matches_web_default() {
        assert_eq!(CONTEXT_MENU_SIDE_OFFSET_PX, 4.0);
    }
}
