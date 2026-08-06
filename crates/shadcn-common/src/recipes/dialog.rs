//! Dialog recipes from `.cn-dialog-overlay` / `.cn-dialog-content` /
//! `.cn-dialog-header` / `.cn-dialog-footer` / `.cn-dialog-title` /
//! `.cn-dialog-description` / `.cn-dialog-close` across style packs.

use crate::style::StyleId;

use super::{ComponentRadius, FontWeight, PopoverShadow, TypeRecipe};

/// Duration of the dialog open/close animation (`duration-100`).
pub const DIALOG_ANIMATION_MS: u64 = 100;

/// Initial scale of the `zoom-in-95` entrance animation.
pub const DIALOG_ZOOM_FROM: f32 = 0.95;

/// Margin kept between the surface and the window edges
/// (`max-w-[calc(100%-2rem)]` — one rem per side).
pub const DIALOG_MARGIN_PX: f32 = 16.0;

/// Footprint of the close button (`size = icon-sm` → `size-8`).
pub const DIALOG_CLOSE_SIZE_PX: f32 = 32.0;

/// Close button glyph size (Lucide `XIcon` at `size-4`).
pub const DIALOG_CLOSE_ICON_PX: f32 = 16.0;

/// Geometry + typography recipe for `.cn-dialog-*` slots.
///
/// The dialog surface uses the `bg-popover` / `text-popover-foreground`
/// pair with a `ring-1 ring-foreground/N` hairline over a `bg-black/N`
/// backdrop; colors stay with the backend palettes, only geometry and
/// alphas live here.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DialogRecipe {
    /// Backdrop black alpha (`bg-black/10` … `bg-black/80`).
    pub overlay_alpha: f32,
    /// Maximum surface width (`sm:max-w-md` / `sm:max-w-sm`).
    pub max_width_px: f32,
    /// Uniform content padding (`p-6` / `p-4`).
    pub pad_px: f32,
    /// Gap of the content grid (`gap-6` / `gap-4`).
    pub gap_px: f32,
    /// Surface corner radius intent (`rounded-xl` / `rounded-4xl` / `rounded-none`).
    pub radius: ComponentRadius,
    /// Optional **cap** in px for `min(scale(radius), cap)` — Rhea's
    /// `rounded-[min(var(--radius-4xl),24px)]`. Not a Tailwind-literal
    /// substitute for `rounded-4xl` (that is [`ComponentRadius::S4xl`] →
    /// `--radius-4xl` = base + 16).
    pub radius_px: Option<f32>,
    /// `ring-foreground/N` alpha in light mode.
    pub ring_alpha: f32,
    /// `ring-foreground/N` alpha in dark mode (`dark:ring-foreground/N`).
    pub ring_alpha_dark: f32,
    /// Surface drop shadow (`shadow-md` / `shadow-xl`), if any.
    pub shadow: Option<PopoverShadow>,
    /// Body typography (`text-sm` / `text-xs/relaxed`).
    pub typography: TypeRecipe,
    /// Gap of the header column (`gap-2` / `gap-1.5` / `gap-1`).
    pub header_gap_px: f32,
    /// Gap of the footer row (`gap-2`).
    pub footer_gap_px: f32,
    /// Title typography (`.cn-dialog-title`).
    pub title: TypeRecipe,
    /// Description typography (`.cn-dialog-description`,
    /// `text-muted-foreground`).
    pub description: TypeRecipe,
    /// Inset of the close button from the top-right corner
    /// (`top-4 right-4` / `top-2 right-2` / `top-5 right-5`).
    pub close_offset_px: f32,
    /// Whether the close button rests on `bg-secondary` instead of the
    /// transparent ghost fill.
    pub close_secondary_bg: bool,
    /// Whether the footer renders as a full-width muted bar
    /// (`bg-muted/50 -mx-4 -mb-4 rounded-b-xl border-t p-4`).
    pub footer_bar: bool,
}

/// Resolves `.cn-dialog-*` tokens for `style`.
pub const fn dialog_recipe(style: StyleId) -> DialogRecipe {
    let base = base_recipe();

    match style {
        StyleId::Vega => base,
        StyleId::Nova => DialogRecipe {
            max_width_px: 384.0,
            pad_px: 16.0,
            gap_px: 16.0,
            title: leading_none(text_base(FontWeight::Medium)),
            close_offset_px: 8.0,
            footer_bar: true,
            ..base
        },
        StyleId::Maia => DialogRecipe {
            overlay_alpha: 0.80,
            radius: ComponentRadius::S4xl,
            ring_alpha: 0.05,
            ring_alpha_dark: 0.05,
            title: leading_none(text_base(FontWeight::Medium)),
            ..base
        },
        StyleId::Luma => DialogRecipe {
            overlay_alpha: 0.30,
            radius: ComponentRadius::S4xl,
            ring_alpha: 0.05,
            ring_alpha_dark: 0.10,
            shadow: Some(SHADOW_XL),
            header_gap_px: 6.0,
            title: leading_none(text_base(FontWeight::Medium)),
            close_secondary_bg: true,
            ..base
        },
        StyleId::Rhea => DialogRecipe {
            overlay_alpha: 0.30,
            radius: ComponentRadius::S4xl,
            radius_px: Some(24.0),
            ring_alpha: 0.05,
            ring_alpha_dark: 0.10,
            shadow: Some(SHADOW_XL),
            header_gap_px: 6.0,
            title: leading_none(text_base(FontWeight::Medium)),
            close_secondary_bg: true,
            ..base
        },
        StyleId::Sera => DialogRecipe {
            overlay_alpha: 0.20,
            radius: ComponentRadius::None,
            shadow: Some(PopoverShadow::MD),
            title: TypeRecipe {
                uppercase: true,
                tracking_em: 0.05,
                ..leading_none(text_lg(FontWeight::Semibold))
            },
            description: TypeRecipe {
                line_height_px: 22.75,
                ..text_sm(FontWeight::Normal)
            },
            close_offset_px: 20.0,
            close_secondary_bg: true,
            ..base
        },
        StyleId::Mira => DialogRecipe {
            overlay_alpha: 0.80,
            max_width_px: 384.0,
            pad_px: 16.0,
            gap_px: 16.0,
            typography: text_xs_relaxed(FontWeight::Normal),
            header_gap_px: 4.0,
            title: text_sm(FontWeight::Medium),
            description: text_xs_relaxed(FontWeight::Normal),
            close_offset_px: 8.0,
            ..base
        },
        StyleId::Lyra => DialogRecipe {
            max_width_px: 384.0,
            pad_px: 16.0,
            gap_px: 16.0,
            radius: ComponentRadius::None,
            typography: text_xs_relaxed(FontWeight::Normal),
            header_gap_px: 4.0,
            title: text_sm(FontWeight::Medium),
            description: text_xs_relaxed(FontWeight::Normal),
            close_offset_px: 8.0,
            ..base
        },
    }
}

/// Tailwind `shadow-xl`: `0 20px 25px -5px rgb(0 0 0 / 0.1)`.
const SHADOW_XL: PopoverShadow = PopoverShadow {
    offset_y_px: 20.0,
    blur_px: 25.0,
    alpha: 0.10,
};

const fn base_recipe() -> DialogRecipe {
    DialogRecipe {
        overlay_alpha: 0.10,
        max_width_px: 448.0,
        pad_px: 24.0,
        gap_px: 24.0,
        radius: ComponentRadius::Xl,
        radius_px: None,
        ring_alpha: 0.10,
        ring_alpha_dark: 0.10,
        shadow: None,
        typography: text_sm(FontWeight::Normal),
        header_gap_px: 8.0,
        footer_gap_px: 8.0,
        title: leading_none(text_sm(FontWeight::Medium)),
        description: text_sm(FontWeight::Normal),
        close_offset_px: 16.0,
        close_secondary_bg: false,
        footer_bar: false,
    }
}

/// `leading-none`: line height collapses to the font size.
const fn leading_none(recipe: TypeRecipe) -> TypeRecipe {
    TypeRecipe {
        line_height_px: recipe.size_px,
        ..recipe
    }
}

const fn text_xs_relaxed(weight: FontWeight) -> TypeRecipe {
    TypeRecipe {
        size_px: 12.0,
        weight,
        uppercase: false,
        tracking_em: 0.0,
        line_height_px: 19.5,
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

const fn text_base(weight: FontWeight) -> TypeRecipe {
    TypeRecipe {
        size_px: 16.0,
        weight,
        uppercase: false,
        tracking_em: 0.0,
        line_height_px: 24.0,
    }
}

const fn text_lg(weight: FontWeight) -> TypeRecipe {
    TypeRecipe {
        size_px: 18.0,
        weight,
        uppercase: false,
        tracking_em: 0.0,
        line_height_px: 28.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vega_matches_style_css() {
        // `bg-black/10`, `gap-6 rounded-xl p-6 text-sm ring-1 sm:max-w-md`.
        let vega = dialog_recipe(StyleId::Vega);
        assert_eq!(vega.overlay_alpha, 0.10);
        assert_eq!(vega.max_width_px, 448.0);
        assert_eq!(vega.pad_px, 24.0);
        assert_eq!(vega.gap_px, 24.0);
        assert_eq!(vega.radius, ComponentRadius::Xl);
        assert_eq!(vega.radius_px, None);
        assert_eq!(vega.shadow, None);
        assert_eq!(vega.close_offset_px, 16.0);
        // Title inherits the content `text-sm` with `leading-none`.
        assert_eq!(vega.title.size_px, 14.0);
        assert_eq!(vega.title.line_height_px, 14.0);
    }

    #[test]
    fn packs_track_their_overrides() {
        // Nova: compact `sm:max-w-sm p-4 gap-4` with the muted footer bar.
        let nova = dialog_recipe(StyleId::Nova);
        assert_eq!(nova.max_width_px, 384.0);
        assert!(nova.footer_bar);
        assert_eq!(nova.close_offset_px, 8.0);

        // Maia: `bg-black/80` backdrop, `rounded-4xl` → `--radius-4xl`.
        let maia = dialog_recipe(StyleId::Maia);
        assert_eq!(maia.overlay_alpha, 0.80);
        assert_eq!(maia.radius, ComponentRadius::S4xl);
        assert_eq!(maia.radius_px, None);
        assert_eq!(maia.ring_alpha, 0.05);

        // Rhea caps the radius at 24 px and casts `shadow-xl`.
        let rhea = dialog_recipe(StyleId::Rhea);
        assert_eq!(rhea.radius, ComponentRadius::S4xl);
        assert_eq!(rhea.radius_px, Some(24.0));
        assert_eq!(rhea.shadow, Some(SHADOW_XL));
        assert!(rhea.close_secondary_bg);

        // Sera: square, uppercase wide-tracked title, `top-5 right-5`.
        let sera = dialog_recipe(StyleId::Sera);
        assert_eq!(sera.radius, ComponentRadius::None);
        assert!(sera.title.uppercase);
        assert_eq!(sera.title.tracking_em, 0.05);
        assert_eq!(sera.close_offset_px, 20.0);

        // Mira / Lyra: `text-xs/relaxed` body copy.
        let mira = dialog_recipe(StyleId::Mira);
        assert_eq!(mira.typography.size_px, 12.0);
        assert_eq!(mira.typography.line_height_px, 19.5);
        let lyra = dialog_recipe(StyleId::Lyra);
        assert_eq!(lyra.radius, ComponentRadius::None);
        assert_eq!(lyra.overlay_alpha, 0.10);
    }
}
