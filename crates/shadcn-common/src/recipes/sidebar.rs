//! Sidebar recipes from `.cn-sidebar-*` across style packs.
//!
//! Geometry and typography only — colors stay on backend palettes
//! (`bg-sidebar`, `text-sidebar-foreground`, `sidebar-border`, …).

use crate::style::StyleId;

use super::{ComponentRadius, FontWeight, TypeRecipe};

/// Open/close width transition (`duration-200`).
pub const SIDEBAR_TRANSITION_MS: u64 = 200;

/// Rail hit-target width (`w-4` → 16).
pub const SIDEBAR_RAIL_WIDTH_PX: f32 = 16.0;

/// Rail hover indicator (`after:w-[2px]`).
pub const SIDEBAR_RAIL_INDICATOR_PX: f32 = 2.0;

/// Menu-button icon size (`size-4` → 16).
pub const SIDEBAR_ICON_SIZE_PX: f32 = 16.0;

/// Disabled / aria-disabled opacity (`opacity-50`).
pub const SIDEBAR_DISABLED_OPACITY: f32 = 0.5;

/// Group-label muted foreground alpha (`text-sidebar-foreground/70`).
pub const SIDEBAR_GROUP_LABEL_FG_ALPHA: f32 = 0.70;

/// Geometry + typography recipe for `.cn-sidebar-*` slots.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SidebarRecipe {
    /// Header / footer padding (`p-2` → 8).
    pub section_pad_px: f32,
    /// Header / footer column gap (`gap-2` → 8).
    pub section_gap_px: f32,
    /// Content column gap (`gap-2` / `gap-0`).
    pub content_gap_px: f32,
    /// Group padding (`p-2` → 8, Mira `px-2 py-1`).
    pub group_pad_x_px: f32,
    /// See [`Self::group_pad_x_px`].
    pub group_pad_y_px: f32,
    /// Menu column gap (`gap-1` / `gap-0` / `gap-px` / `gap-0.5`).
    pub menu_gap_px: f32,
    /// Group-label height (`h-8` → 32).
    pub group_label_height_px: f32,
    /// Group-label horizontal padding (`px-2` → 8).
    pub group_label_pad_x_px: f32,
    /// Group-label typography (`text-xs font-medium`).
    pub group_label: TypeRecipe,
    /// Group-action footprint (`w-5` → 20).
    pub group_action_size_px: f32,
    /// Group-action top/right inset (`top-3.5 right-3`).
    pub group_action_top_px: f32,
    /// See [`Self::group_action_top_px`].
    pub group_action_right_px: f32,
    /// Default menu-button height (`h-8` → 32).
    pub menu_button_height_px: f32,
    /// Small menu-button height (`h-7` → 28).
    pub menu_button_sm_height_px: f32,
    /// Large menu-button height (`h-12` → 48).
    pub menu_button_lg_height_px: f32,
    /// Menu-button padding (`p-2` → 8).
    pub menu_button_pad_px: f32,
    /// Menu-button gap (`gap-2` → 8).
    pub menu_button_gap_px: f32,
    /// Menu-button corner radius.
    pub menu_button_radius: ComponentRadius,
    /// Default menu-button typography (`text-sm`).
    pub menu_button: TypeRecipe,
    /// Small menu-button typography (`text-xs`).
    pub menu_button_sm: TypeRecipe,
    /// Menu-action / badge size (`w-5` / `h-5` → 20).
    pub menu_action_size_px: f32,
    /// Menu-action / badge top inset for default size (`top-1.5` → 6).
    pub menu_action_top_default_px: f32,
    /// Menu-action / badge top inset for `sm` (`top-1` → 4).
    pub menu_action_top_sm_px: f32,
    /// Menu-action / badge top inset for `lg` (`top-2.5` → 10).
    pub menu_action_top_lg_px: f32,
    /// Menu-action / badge right inset (`right-1` → 4).
    pub menu_action_right_px: f32,
    /// Menu-badge typography (`text-xs font-medium`).
    pub menu_badge: TypeRecipe,
    /// Menu-skeleton height (`h-8` → 32).
    pub menu_skeleton_height_px: f32,
    /// Menu-skeleton horizontal padding (`px-2` → 8).
    pub menu_skeleton_pad_x_px: f32,
    /// Menu-skeleton gap (`gap-2` → 8).
    pub menu_skeleton_gap_px: f32,
    /// Menu-sub horizontal margin (`mx-3.5` → 14).
    pub menu_sub_margin_x_px: f32,
    /// Menu-sub horizontal padding (`px-2.5` → 10).
    pub menu_sub_pad_x_px: f32,
    /// Menu-sub vertical padding (`py-0.5` → 2).
    pub menu_sub_pad_y_px: f32,
    /// Menu-sub column gap (`gap-1` → 4).
    pub menu_sub_gap_px: f32,
    /// Menu-sub-button height (`h-7` → 28).
    pub menu_sub_button_height_px: f32,
    /// Menu-sub-button horizontal padding (`px-2` → 8).
    pub menu_sub_button_pad_x_px: f32,
    /// Menu-sub-button gap (`gap-2` → 8).
    pub menu_sub_button_gap_px: f32,
    /// Menu-sub-button `md` typography (`text-sm`).
    pub menu_sub_button_md: TypeRecipe,
    /// Menu-sub-button `sm` typography (`text-xs`).
    pub menu_sub_button_sm: TypeRecipe,
    /// Separator horizontal margin (`mx-2` → 8).
    pub separator_margin_x_px: f32,
    /// Sidebar input height (`h-8` → 32).
    pub input_height_px: f32,
    /// Floating / inset inner radius (`rounded-lg` / `rounded-2xl` / `rounded-none`).
    pub floating_radius: ComponentRadius,
    /// Inset main-area margin (`m-2` → 8).
    pub inset_margin_px: f32,
    /// Inset main-area radius (`rounded-xl`).
    pub inset_radius: ComponentRadius,
}

/// Resolves `.cn-sidebar-*` tokens for `style`.
pub const fn sidebar_recipe(style: StyleId) -> SidebarRecipe {
    let base = base_recipe();

    match style {
        StyleId::Vega => base,
        StyleId::Nova => SidebarRecipe {
            content_gap_px: 0.0,
            menu_gap_px: 0.0,
            ..base
        },
        StyleId::Maia => SidebarRecipe {
            floating_radius: ComponentRadius::Xl,
            menu_button_radius: ComponentRadius::Xl,
            ..base
        },
        StyleId::Lyra => SidebarRecipe {
            content_gap_px: 0.0,
            menu_gap_px: 0.0,
            floating_radius: ComponentRadius::None,
            ..base
        },
        StyleId::Mira => SidebarRecipe {
            content_gap_px: 0.0,
            menu_gap_px: 1.0,
            group_pad_y_px: 4.0,
            ..base
        },
        StyleId::Luma => SidebarRecipe {
            menu_gap_px: 2.0,
            floating_radius: ComponentRadius::S2xl,
            menu_button_radius: ComponentRadius::Xl,
            ..base
        },
        StyleId::Sera => SidebarRecipe {
            menu_gap_px: 2.0,
            floating_radius: ComponentRadius::None,
            menu_button_radius: ComponentRadius::None,
            ..base
        },
        StyleId::Rhea => SidebarRecipe {
            menu_gap_px: 2.0,
            floating_radius: ComponentRadius::S2xl,
            menu_button_radius: ComponentRadius::Xl,
            ..base
        },
    }
}

const fn base_recipe() -> SidebarRecipe {
    SidebarRecipe {
        section_pad_px: 8.0,
        section_gap_px: 8.0,
        content_gap_px: 8.0,
        group_pad_x_px: 8.0,
        group_pad_y_px: 8.0,
        menu_gap_px: 4.0,
        group_label_height_px: 32.0,
        group_label_pad_x_px: 8.0,
        group_label: text_xs(FontWeight::Medium),
        group_action_size_px: 20.0,
        group_action_top_px: 14.0,
        group_action_right_px: 12.0,
        menu_button_height_px: 32.0,
        menu_button_sm_height_px: 28.0,
        menu_button_lg_height_px: 48.0,
        menu_button_pad_px: 8.0,
        menu_button_gap_px: 8.0,
        menu_button_radius: ComponentRadius::Md,
        menu_button: text_sm(FontWeight::Normal),
        menu_button_sm: text_xs(FontWeight::Normal),
        menu_action_size_px: 20.0,
        menu_action_top_default_px: 6.0,
        menu_action_top_sm_px: 4.0,
        menu_action_top_lg_px: 10.0,
        menu_action_right_px: 4.0,
        menu_badge: text_xs(FontWeight::Medium),
        menu_skeleton_height_px: 32.0,
        menu_skeleton_pad_x_px: 8.0,
        menu_skeleton_gap_px: 8.0,
        menu_sub_margin_x_px: 14.0,
        menu_sub_pad_x_px: 10.0,
        menu_sub_pad_y_px: 2.0,
        menu_sub_gap_px: 4.0,
        menu_sub_button_height_px: 28.0,
        menu_sub_button_pad_x_px: 8.0,
        menu_sub_button_gap_px: 8.0,
        menu_sub_button_md: text_sm(FontWeight::Normal),
        menu_sub_button_sm: text_xs(FontWeight::Normal),
        separator_margin_x_px: 8.0,
        input_height_px: 32.0,
        floating_radius: ComponentRadius::Lg,
        inset_margin_px: 8.0,
        inset_radius: ComponentRadius::Xl,
    }
}

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
    fn vega_matches_style_css() {
        let vega = sidebar_recipe(StyleId::Vega);
        assert_eq!(vega.section_pad_px, 8.0);
        assert_eq!(vega.content_gap_px, 8.0);
        assert_eq!(vega.menu_gap_px, 4.0);
        assert_eq!(vega.menu_button_height_px, 32.0);
        assert_eq!(vega.menu_button_sm_height_px, 28.0);
        assert_eq!(vega.menu_button_lg_height_px, 48.0);
        assert_eq!(vega.floating_radius, ComponentRadius::Lg);
        assert_eq!(vega.inset_radius, ComponentRadius::Xl);
    }

    #[test]
    fn packs_track_their_overrides() {
        assert_eq!(sidebar_recipe(StyleId::Nova).menu_gap_px, 0.0);
        assert_eq!(sidebar_recipe(StyleId::Nova).content_gap_px, 0.0);
        assert_eq!(
            sidebar_recipe(StyleId::Lyra).floating_radius,
            ComponentRadius::None
        );
        assert_eq!(sidebar_recipe(StyleId::Mira).menu_gap_px, 1.0);
        assert_eq!(sidebar_recipe(StyleId::Mira).group_pad_y_px, 4.0);
        assert_eq!(
            sidebar_recipe(StyleId::Luma).floating_radius,
            ComponentRadius::S2xl
        );
        assert_eq!(
            sidebar_recipe(StyleId::Sera).menu_button_radius,
            ComponentRadius::None
        );
        assert_eq!(
            sidebar_recipe(StyleId::Rhea).floating_radius,
            ComponentRadius::S2xl
        );
    }
}
