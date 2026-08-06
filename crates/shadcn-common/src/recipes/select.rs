//! Select recipes from `.cn-select-trigger` / `.cn-select-content` /
//! `.cn-select-item` / `.cn-select-label` / `.cn-select-separator` across
//! style packs.
//!
//! Unlike [`super::native_select`], the custom select owns both the trigger
//! field and a design-system dropdown (popover surface + checkable items).
//! Geometry here is shared by iced and egui; colors stay with backend
//! palettes.

use crate::style::StyleId;

use super::{ComponentRadius, FontWeight, PopoverShadow, TypeRecipe};

/// Duration of the select content open/close animation (`duration-100`).
pub const SELECT_ANIMATION_MS: u64 = 100;

/// Distance covered by the `slide-in-from-*-2` entrance animation.
pub const SELECT_SLIDE_PX: f32 = 8.0;

/// Initial scale of the `zoom-in-95` entrance animation.
pub const SELECT_ZOOM_FROM: f32 = 0.95;

/// Default `sideOffset` of the shadcn-svelte select content.
pub const SELECT_SIDE_OFFSET_PX: f32 = 4.0;

/// `disabled:opacity-50` on the trigger.
pub const SELECT_DISABLED_OPACITY: f32 = 0.5;

/// Maximum dropdown height before it scrolls (`max-h-96`).
pub const SELECT_CONTENT_MAX_HEIGHT_PX: f32 = 384.0;

/// Geometry + typography recipe for the custom select.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SelectRecipe {
    /// Trigger `pl-*` in px.
    pub trigger_pad_left_px: f32,
    /// Trigger `pr-*` in px (room for the inline chevron).
    pub trigger_pad_right_px: f32,
    /// Gap between value and chevron (`gap-1.5` → 6).
    pub trigger_gap_px: f32,
    /// Value text size of the default size.
    pub trigger_text_size_px: f32,
    /// Value text size of the `sm` size (Mira may differ).
    pub trigger_text_size_sm_px: f32,
    /// Corner treatment of the default trigger size.
    pub trigger_radius: ComponentRadius,
    /// Corner treatment of the `sm` trigger size.
    pub trigger_radius_sm: ComponentRadius,
    /// `bg-input/N` alpha in light mode (0 = `bg-transparent`).
    pub fill_alpha_light: f32,
    /// `dark:bg-input/N` alpha.
    pub fill_alpha_dark: f32,
    /// `dark:hover:bg-input/N` alpha.
    pub hover_fill_alpha_dark: f32,
    /// Whether the resting border is painted (`border-input` vs transparent).
    pub bordered: bool,
    /// Sera's `border-b-input`: only the bottom hairline is painted.
    pub underline_only: bool,
    /// Chevron icon edge (`size-4` → 16).
    pub icon_size_px: f32,
    /// Chevron icon edge of the `sm` size.
    pub icon_size_sm_px: f32,

    /// Content `min-w-*` in px (`min-w-36` → 144, Mira `min-w-32` → 128).
    pub content_min_width_px: f32,
    /// Content corner radius intent.
    pub content_radius: ComponentRadius,
    /// Viewport / group padding (`p-1` → 4).
    pub content_pad_px: f32,
    /// `ring-foreground/N` alpha in light mode.
    pub content_ring_alpha: f32,
    /// `ring-foreground/N` alpha in dark mode.
    pub content_ring_alpha_dark: f32,
    /// Content drop shadow.
    pub content_shadow: PopoverShadow,

    /// Item vertical padding (`py-1.5` → 6).
    pub item_pad_y_px: f32,
    /// Item left padding (`pl-2` → 8).
    pub item_pad_left_px: f32,
    /// Item right padding reserved for the check (`pr-8` → 32).
    pub item_pad_right_px: f32,
    /// Item corner radius intent.
    pub item_radius: ComponentRadius,
    /// Item body typography.
    pub item_typography: TypeRecipe,
    /// Check indicator edge (`size-3.5` / `size-4`).
    pub item_indicator_size_px: f32,
    /// Distance from the item's end edge to the indicator (`right-2` → 8).
    pub item_indicator_right_px: f32,

    /// Group / section label typography (`.cn-select-label`).
    pub label_typography: TypeRecipe,
    /// Label horizontal padding (`px-2` → 8).
    pub label_pad_x_px: f32,
    /// Label vertical padding (`py-1.5` → 6).
    pub label_pad_y_px: f32,

    /// Separator vertical margin (`my-1` → 4). Equals [`Self::content_pad_px`].
    pub separator_margin_y_px: f32,
    /// Separator horizontal bleed (`-mx-1` → 4). Equals [`Self::content_pad_px`].
    pub separator_margin_x_px: f32,

    /// Scroll-button vertical padding (`py-1` → 4).
    pub scroll_button_pad_y_px: f32,
}

/// Resolves `.cn-select-*` tokens for `style`.
pub const fn select_recipe(style: StyleId) -> SelectRecipe {
    match style {
        StyleId::Vega => VEGA,
        // `rounded-lg` trigger / content; sm trigger
        // `rounded-[min(--radius-md,10px)]` → md slot; items `rounded-md
        // py-1 pr-8 pl-1.5`.
        StyleId::Nova => SelectRecipe {
            trigger_radius: ComponentRadius::Lg,
            trigger_radius_sm: ComponentRadius::Md,
            content_radius: ComponentRadius::Lg,
            item_pad_y_px: 4.0,
            item_pad_left_px: 6.0,
            item_radius: ComponentRadius::Md,
            ..VEGA
        },
        // `bg-input/30 rounded-4xl px-3`; content `rounded-2xl shadow-2xl
        // ring-foreground/5`; items `rounded-xl py-2 pr-8 pl-3`.
        StyleId::Maia => SelectRecipe {
            trigger_pad_left_px: 12.0,
            trigger_pad_right_px: 12.0,
            trigger_radius: ComponentRadius::S4xl,
            trigger_radius_sm: ComponentRadius::S4xl,
            fill_alpha_light: 0.3,
            fill_alpha_dark: 0.3,
            hover_fill_alpha_dark: 0.5,
            content_radius: ComponentRadius::S2xl,
            content_ring_alpha: 0.05,
            content_ring_alpha_dark: 0.05,
            content_shadow: PopoverShadow::XXL,
            item_pad_y_px: 8.0,
            item_pad_left_px: 12.0,
            item_radius: ComponentRadius::Xl,
            ..VEGA
        },
        // `rounded-none text-xs`; content square, no group pad; items `py-2`.
        // Separator is bare `h-px` (no `my-*`) so gaps match the 0 content pad.
        StyleId::Lyra => SelectRecipe {
            trigger_text_size_px: 12.0,
            trigger_text_size_sm_px: 12.0,
            trigger_radius: ComponentRadius::None,
            trigger_radius_sm: ComponentRadius::None,
            content_radius: ComponentRadius::None,
            content_pad_px: 0.0,
            item_pad_y_px: 8.0,
            item_radius: ComponentRadius::None,
            item_typography: text_xs(FontWeight::Normal),
            separator_margin_y_px: 0.0,
            separator_margin_x_px: 0.0,
            ..VEGA
        },
        // Compact: `bg-input/20 h-7 text-xs`, content `min-w-32 rounded-lg`,
        // items `min-h-7 rounded-md px-2 py-1`, icons `size-3.5`.
        StyleId::Mira => SelectRecipe {
            trigger_pad_left_px: 8.0,
            trigger_pad_right_px: 8.0,
            trigger_text_size_px: 12.0,
            trigger_text_size_sm_px: 12.0,
            fill_alpha_light: 0.2,
            icon_size_px: 14.0,
            icon_size_sm_px: 14.0,
            content_min_width_px: 128.0,
            content_radius: ComponentRadius::Lg,
            item_pad_y_px: 4.0,
            item_radius: ComponentRadius::Md,
            item_typography: text_xs(FontWeight::Normal),
            item_indicator_size_px: 14.0,
            label_typography: text_xs(FontWeight::Normal),
            ..VEGA
        },
        // Soft panel: trigger `rounded-3xl` on h-9; content `rounded-3xl`;
        // group `p-1.5`; items `rounded-2xl font-medium`.
        StyleId::Luma => SelectRecipe {
            trigger_pad_left_px: 12.0,
            trigger_pad_right_px: 12.0,
            trigger_radius: ComponentRadius::S3xl,
            trigger_radius_sm: ComponentRadius::S3xl,
            fill_alpha_light: 0.5,
            fill_alpha_dark: 0.5,
            hover_fill_alpha_dark: 0.5,
            bordered: false,
            content_radius: ComponentRadius::S3xl,
            content_pad_px: 6.0,
            content_ring_alpha: 0.05,
            content_ring_alpha_dark: 0.10,
            content_shadow: PopoverShadow::LG,
            item_pad_y_px: 8.0,
            item_pad_left_px: 12.0,
            item_radius: ComponentRadius::S2xl,
            item_typography: text_sm(FontWeight::Medium),
            separator_margin_y_px: 6.0,
            separator_margin_x_px: 6.0,
            ..VEGA
        },
        // Underline-only trigger; square content; group `p-1.5`; items `py-2 pl-3`.
        StyleId::Sera => SelectRecipe {
            trigger_pad_left_px: 0.0,
            trigger_pad_right_px: 0.0,
            trigger_radius: ComponentRadius::None,
            trigger_radius_sm: ComponentRadius::None,
            fill_alpha_dark: 0.0,
            hover_fill_alpha_dark: 0.0,
            underline_only: true,
            icon_size_px: 14.0,
            icon_size_sm_px: 14.0,
            content_radius: ComponentRadius::None,
            content_pad_px: 6.0,
            item_pad_y_px: 8.0,
            item_pad_left_px: 12.0,
            item_radius: ComponentRadius::None,
            item_indicator_size_px: 14.0,
            separator_margin_y_px: 6.0,
            separator_margin_x_px: 6.0,
            ..VEGA
        },
        // Soft rounded: `bg-input/50 rounded-2xl border-transparent`; content
        // `rounded-2xl shadow-lg`; items `min-h-7 rounded-xl`.
        StyleId::Rhea => SelectRecipe {
            trigger_pad_left_px: 12.0,
            trigger_pad_right_px: 12.0,
            trigger_radius: ComponentRadius::S2xl,
            trigger_radius_sm: ComponentRadius::S2xl,
            fill_alpha_light: 0.5,
            fill_alpha_dark: 0.5,
            hover_fill_alpha_dark: 0.5,
            bordered: false,
            content_radius: ComponentRadius::S2xl,
            content_ring_alpha: 0.05,
            content_ring_alpha_dark: 0.10,
            content_shadow: PopoverShadow::LG,
            item_radius: ComponentRadius::Xl,
            ..VEGA
        },
    }
}

/// Vega `.cn-select-*` used as the fallback for unknown future packs.
const VEGA: SelectRecipe = SelectRecipe {
    trigger_pad_left_px: 10.0,
    trigger_pad_right_px: 8.0,
    trigger_gap_px: 6.0,
    trigger_text_size_px: 14.0,
    trigger_text_size_sm_px: 14.0,
    trigger_radius: ComponentRadius::Md,
    trigger_radius_sm: ComponentRadius::Md,
    fill_alpha_light: 0.0,
    fill_alpha_dark: 0.3,
    hover_fill_alpha_dark: 0.5,
    bordered: true,
    underline_only: false,
    icon_size_px: 16.0,
    icon_size_sm_px: 16.0,
    content_min_width_px: 144.0,
    content_radius: ComponentRadius::Md,
    content_pad_px: 4.0,
    content_ring_alpha: 0.10,
    content_ring_alpha_dark: 0.10,
    content_shadow: PopoverShadow::MD,
    item_pad_y_px: 6.0,
    item_pad_left_px: 8.0,
    item_pad_right_px: 32.0,
    item_radius: ComponentRadius::Sm,
    item_typography: text_sm(FontWeight::Normal),
    item_indicator_size_px: 14.0,
    item_indicator_right_px: 8.0,
    label_typography: text_xs(FontWeight::Normal),
    label_pad_x_px: 8.0,
    label_pad_y_px: 6.0,
    separator_margin_y_px: 4.0,
    separator_margin_x_px: 4.0,
    scroll_button_pad_y_px: 4.0,
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
            let recipe = select_recipe(style);
            assert!(recipe.content_min_width_px > 0.0);
            assert!(recipe.item_pad_right_px >= recipe.item_indicator_size_px);
        }
    }

    #[test]
    fn separator_margins_match_content_pad() {
        // shadcn: separator `my-*` / `-mx-*` equals group `p-*`.
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
            let recipe = select_recipe(style);
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
    fn mira_is_compact_and_sera_is_underline_only() {
        let mira = select_recipe(StyleId::Mira);
        assert_eq!(mira.content_min_width_px, 128.0);
        assert_eq!(mira.trigger_text_size_px, 12.0);

        let sera = select_recipe(StyleId::Sera);
        assert!(sera.underline_only);
        assert_eq!(sera.trigger_radius, ComponentRadius::None);
    }
}
