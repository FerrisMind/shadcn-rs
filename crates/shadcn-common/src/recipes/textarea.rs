//! Textarea recipes from `.cn-textarea` across style packs.
//!
//! These tokens capture the per-pack geometry of the shadcn-svelte textarea
//! component (padding, text size, minimum height, corner radius, fill alpha,
//! border treatment, focus ring, shadow, and the Sera underline-only variant).
//! They are backend-agnostic: iced and egui both resolve them against their
//! own theme and widget APIs, so a single source of truth describes every
//! style pack.

use crate::style::StyleId;

use super::ComponentRadius;

/// Geometry + surface tokens for `.cn-textarea`.
///
/// Every shadcn-svelte style pack ships its own `.cn-textarea` rule. This
/// struct captures the parts that differ between packs so both iced and egui
/// can share one table. The `min-h-16` (`min-height: 64px`) token is shared by
/// every pack and lives here as a constant.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TextareaRecipe {
    /// Horizontal padding (`px-*`).
    pub pad_x_px: f32,
    /// Vertical padding (`py-*`).
    pub pad_y_px: f32,
    /// Value / placeholder text size (`md:text-sm` → 14, `md:text-xs` → 12).
    pub text_size_px: f32,
    /// Default corner treatment when the builder does not override it.
    pub default_radius: ComponentRadius,
    /// `bg-input/N` alpha in light mode (0 = `bg-transparent`).
    pub fill_alpha_light: f32,
    /// `dark:bg-input/N` alpha.
    pub fill_alpha_dark: f32,
    /// Whether the resting border is painted (`border-input` vs
    /// `border-transparent`).
    pub bordered: bool,
    /// `disabled:bg-input/50 dark:disabled:bg-input/80` (Nova, Lyra).
    pub disabled_fill: bool,
    /// `focus-visible:ring-*` width in px (0 = no ring, e.g. Sera).
    pub focus_ring_px: f32,
    /// Whether `shadow-xs` is applied (Vega, Nova).
    pub shadow: bool,
    /// Sera-style underline-only: `border-b-input` paints only the bottom
    /// hairline. Backends give the editor a transparent box border and draw
    /// the bottom line separately.
    pub underline_only: bool,
}

/// `min-h-16` from the base `.cn-textarea` class — identical for every pack.
pub const MIN_HEIGHT_PX: f32 = 64.0;

/// `disabled:opacity-50` from the base `.cn-textarea` class.
pub const DISABLED_OPACITY: f32 = 0.5;

/// `dark:aria-invalid:border-destructive/50`.
pub const DARK_INVALID_BORDER_ALPHA: f32 = 0.5;

/// `aria-invalid:ring-destructive/20` in light mode.
pub const INVALID_RING_ALPHA_LIGHT: f32 = 0.2;

/// `dark:aria-invalid:ring-destructive/40`.
pub const INVALID_RING_ALPHA_DARK: f32 = 0.4;

/// Selection wash over the value text (web `::selection`).
pub const SELECTION_ALPHA: f32 = 0.4;

/// Resolves `.cn-textarea` tokens for `style`.
pub const fn textarea_recipe(style: StyleId) -> TextareaRecipe {
    match style {
        StyleId::Vega => TextareaRecipe {
            pad_x_px: 10.0,
            pad_y_px: 8.0,
            text_size_px: 14.0,
            default_radius: ComponentRadius::Md,
            fill_alpha_light: 0.0,
            fill_alpha_dark: 0.3,
            bordered: true,
            disabled_fill: false,
            focus_ring_px: 3.0,
            shadow: true,
            underline_only: false,
        },
        // `rounded-lg ... disabled:bg-input/50 dark:disabled:bg-input/80`
        StyleId::Nova => TextareaRecipe {
            default_radius: ComponentRadius::Lg,
            disabled_fill: true,
            shadow: false,
            ..textarea_recipe(StyleId::Vega)
        },
        // `bg-input/30 ... rounded-xl ... px-3 py-3`
        StyleId::Maia => TextareaRecipe {
            pad_x_px: 12.0,
            pad_y_px: 12.0,
            fill_alpha_light: 0.3,
            default_radius: ComponentRadius::Xl,
            shadow: false,
            ..textarea_recipe(StyleId::Vega)
        },
        // `rounded-none ... text-xs ... focus-visible:ring-1 ... disabled:bg-input/50`
        StyleId::Lyra => TextareaRecipe {
            text_size_px: 12.0,
            default_radius: ComponentRadius::None,
            disabled_fill: true,
            focus_ring_px: 1.0,
            shadow: false,
            ..textarea_recipe(StyleId::Vega)
        },
        // `bg-input/20 ... rounded-md ... px-2 py-2 text-sm ... focus-visible:ring-2`
        StyleId::Mira => TextareaRecipe {
            pad_x_px: 8.0,
            pad_y_px: 8.0,
            text_size_px: 12.0,
            fill_alpha_light: 0.2,
            focus_ring_px: 2.0,
            shadow: false,
            ..textarea_recipe(StyleId::Vega)
        },
        // `bg-input/50 ... rounded-2xl border-transparent px-3 py-3`
        StyleId::Luma => TextareaRecipe {
            pad_x_px: 12.0,
            pad_y_px: 12.0,
            fill_alpha_light: 0.5,
            fill_alpha_dark: 0.5,
            bordered: false,
            default_radius: ComponentRadius::S2xl,
            shadow: false,
            ..textarea_recipe(StyleId::Vega)
        },
        // Web Sera is underline-only (`border-b-input`, `px-0 py-3`); the
        // editor gets a transparent box border and the bottom hairline is
        // drawn separately.
        StyleId::Sera => TextareaRecipe {
            pad_x_px: 0.0,
            pad_y_px: 12.0,
            fill_alpha_dark: 0.0,
            default_radius: ComponentRadius::None,
            focus_ring_px: 0.0,
            shadow: false,
            underline_only: true,
            ..textarea_recipe(StyleId::Vega)
        },
        // `bg-input/50 ... rounded-2xl border-transparent px-2.5 py-2`
        StyleId::Rhea => TextareaRecipe {
            fill_alpha_light: 0.5,
            fill_alpha_dark: 0.5,
            bordered: false,
            default_radius: ComponentRadius::S2xl,
            shadow: false,
            ..textarea_recipe(StyleId::Vega)
        },
    }
}
