//! Alert-dialog recipes from `.cn-alert-dialog-overlay` /
//! `.cn-alert-dialog-content` / `.cn-alert-dialog-header` /
//! `.cn-alert-dialog-media` / `.cn-alert-dialog-title` /
//! `.cn-alert-dialog-description` across style packs.
//!
//! The alert dialog shares the modal machinery with the dialog but is a
//! separate component in shadcn-svelte: it has no close button, ignores
//! outside interactions by default (`interactOutsideBehavior: "ignore"`),
//! offers a `size` prop (`default` / `sm`), and adds the media slot plus
//! the action/cancel button pair.

use crate::style::StyleId;

use super::{ComponentRadius, FontWeight, PopoverShadow, TypeRecipe};

/// Geometry + typography recipe for `.cn-alert-dialog-*` slots.
///
/// The surface uses the `bg-popover` / `text-popover-foreground` pair with
/// a `ring-1 ring-foreground/N` hairline over a `bg-black/N` backdrop;
/// colors stay with the backend palettes, only geometry and alphas live
/// here. Open/close animation constants are shared with the dialog
/// ([`super::DIALOG_ANIMATION_MS`], [`super::DIALOG_ZOOM_FROM`],
/// [`super::DIALOG_MARGIN_PX`]).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AlertDialogRecipe {
    /// Backdrop black alpha (`bg-black/10` … `bg-black/80`).
    pub overlay_alpha: f32,
    /// Maximum surface width for `size="default"` at the `sm:` breakpoint
    /// (`data-[size=default]:sm:max-w-lg` / `sm:max-w-md` / `sm:max-w-sm`).
    pub max_width_px: f32,
    /// Maximum surface width for `size="sm"`
    /// (`data-[size=sm]:max-w-xs` / `max-w-64`).
    pub max_width_sm_px: f32,
    /// Uniform content padding (`p-6` / `p-4`).
    pub pad_px: f32,
    /// Gap of the content grid (`gap-6` / `gap-4` / `gap-3`).
    pub gap_px: f32,
    /// Surface corner radius intent (`rounded-xl` / `rounded-4xl` / …).
    pub radius: ComponentRadius,
    /// Optional **cap** in px for `min(scale(radius), cap)` — Rhea's
    /// `rounded-[min(var(--radius-4xl),24px)]`.
    pub radius_px: Option<f32>,
    /// `ring-foreground/N` alpha in light mode.
    pub ring_alpha: f32,
    /// `ring-foreground/N` alpha in dark mode (`dark:ring-foreground/N`).
    pub ring_alpha_dark: f32,
    /// Surface drop shadow (`shadow-md` / `shadow-xl`), if any.
    pub shadow: Option<PopoverShadow>,
    /// Gap of the header grid (`gap-2` / `gap-1.5` / `gap-1`).
    pub header_gap_px: f32,
    /// Gap of the footer row (`gap-2`).
    pub footer_gap_px: f32,
    /// Title typography (`.cn-alert-dialog-title`).
    pub title: TypeRecipe,
    /// Description typography (`.cn-alert-dialog-description`,
    /// `text-muted-foreground`).
    pub description: TypeRecipe,
    /// Extra top margin of the description (`mt-0.5`, Sera only).
    pub description_margin_top_px: f32,
    /// Side of the square media box (`size-16` / `size-10` / `size-8`).
    pub media_size_px: f32,
    /// Media box corner radius (`rounded-md` / `rounded-full` /
    /// `rounded-none`), painted on `bg-muted`.
    pub media_radius: ComponentRadius,
    /// Recommended glyph size inside the media box
    /// (`*:[svg:not([class*='size-'])]:size-8` and friends).
    pub media_icon_px: f32,
    /// Column gap between the media box and the header text at
    /// `size="default"` (`has-data-[slot=alert-dialog-media]:gap-x-6`).
    pub media_gap_x_px: f32,
    /// Bottom margin under the media box in the stacked layout (`mb-2`).
    pub media_margin_bottom_px: f32,
}

/// Resolves `.cn-alert-dialog-*` tokens for `style`.
pub const fn alert_dialog_recipe(style: StyleId) -> AlertDialogRecipe {
    let base = base_recipe();

    match style {
        StyleId::Vega => base,
        StyleId::Nova => AlertDialogRecipe {
            max_width_px: 384.0,
            pad_px: 16.0,
            gap_px: 16.0,
            title: text_base(FontWeight::Medium),
            media_size_px: 40.0,
            media_icon_px: 24.0,
            media_gap_x_px: 16.0,
            ..base
        },
        StyleId::Maia => AlertDialogRecipe {
            overlay_alpha: 0.80,
            max_width_px: 448.0,
            radius: ComponentRadius::S4xl,
            ring_alpha: 0.05,
            ring_alpha_dark: 0.05,
            media_radius: ComponentRadius::Full,
            ..base
        },
        StyleId::Luma => AlertDialogRecipe {
            overlay_alpha: 0.30,
            max_width_px: 448.0,
            radius: ComponentRadius::S4xl,
            ring_alpha: 0.05,
            ring_alpha_dark: 0.10,
            shadow: Some(SHADOW_XL),
            media_radius: ComponentRadius::Full,
            ..base
        },
        StyleId::Rhea => AlertDialogRecipe {
            overlay_alpha: 0.30,
            max_width_px: 448.0,
            radius: ComponentRadius::S4xl,
            radius_px: Some(24.0),
            ring_alpha: 0.05,
            ring_alpha_dark: 0.10,
            shadow: Some(SHADOW_XL),
            media_radius: ComponentRadius::Full,
            ..base
        },
        StyleId::Sera => AlertDialogRecipe {
            overlay_alpha: 0.20,
            max_width_px: 448.0,
            radius: ComponentRadius::None,
            shadow: Some(PopoverShadow::MD),
            header_gap_px: 8.0,
            title: TypeRecipe {
                uppercase: true,
                tracking_em: 0.05,
                ..text_lg(FontWeight::Semibold)
            },
            description: TypeRecipe {
                line_height_px: 22.75,
                ..text_sm(FontWeight::Normal)
            },
            description_margin_top_px: 2.0,
            media_radius: ComponentRadius::None,
            ..base
        },
        StyleId::Mira => AlertDialogRecipe {
            overlay_alpha: 0.80,
            max_width_px: 384.0,
            max_width_sm_px: 256.0,
            pad_px: 16.0,
            gap_px: 12.0,
            header_gap_px: 4.0,
            title: text_sm(FontWeight::Medium),
            description: text_xs_relaxed(FontWeight::Normal),
            media_size_px: 32.0,
            media_icon_px: 16.0,
            media_gap_x_px: 16.0,
            ..base
        },
        StyleId::Lyra => AlertDialogRecipe {
            max_width_px: 384.0,
            pad_px: 16.0,
            gap_px: 16.0,
            radius: ComponentRadius::None,
            title: text_sm(FontWeight::Medium),
            description: text_xs_relaxed(FontWeight::Normal),
            media_size_px: 40.0,
            media_radius: ComponentRadius::None,
            media_icon_px: 24.0,
            media_gap_x_px: 16.0,
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

const fn base_recipe() -> AlertDialogRecipe {
    AlertDialogRecipe {
        overlay_alpha: 0.10,
        max_width_px: 512.0,
        max_width_sm_px: 320.0,
        pad_px: 24.0,
        gap_px: 24.0,
        radius: ComponentRadius::Xl,
        radius_px: None,
        ring_alpha: 0.10,
        ring_alpha_dark: 0.10,
        shadow: None,
        header_gap_px: 6.0,
        footer_gap_px: 8.0,
        title: text_lg(FontWeight::Medium),
        description: text_sm(FontWeight::Normal),
        description_margin_top_px: 0.0,
        media_size_px: 64.0,
        media_radius: ComponentRadius::Md,
        media_icon_px: 32.0,
        media_gap_x_px: 24.0,
        media_margin_bottom_px: 8.0,
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
        // `bg-black/10`, `gap-6 rounded-xl p-6 ring-1 ring-foreground/10`,
        // `data-[size=default]:sm:max-w-lg data-[size=sm]:max-w-xs`.
        let vega = alert_dialog_recipe(StyleId::Vega);
        assert_eq!(vega.overlay_alpha, 0.10);
        assert_eq!(vega.max_width_px, 512.0);
        assert_eq!(vega.max_width_sm_px, 320.0);
        assert_eq!(vega.pad_px, 24.0);
        assert_eq!(vega.gap_px, 24.0);
        assert_eq!(vega.radius, ComponentRadius::Xl);
        assert_eq!(vega.radius_px, None);
        assert_eq!(vega.ring_alpha, 0.10);
        assert_eq!(vega.shadow, None);
        // `text-lg font-medium` title over a `text-sm` description.
        assert_eq!(vega.title.size_px, 18.0);
        assert_eq!(vega.title.line_height_px, 28.0);
        assert_eq!(vega.title.weight, FontWeight::Medium);
        assert_eq!(vega.description.size_px, 14.0);
        // `size-16 rounded-md` media with `size-8` glyphs and `gap-x-6`.
        assert_eq!(vega.media_size_px, 64.0);
        assert_eq!(vega.media_radius, ComponentRadius::Md);
        assert_eq!(vega.media_icon_px, 32.0);
        assert_eq!(vega.media_gap_x_px, 24.0);
        assert_eq!(vega.media_margin_bottom_px, 8.0);
    }

    #[test]
    fn packs_track_their_overrides() {
        // Nova: compact `sm:max-w-sm p-4 gap-4`, `text-base` title,
        // `size-10` media with `size-6` glyphs.
        let nova = alert_dialog_recipe(StyleId::Nova);
        assert_eq!(nova.max_width_px, 384.0);
        assert_eq!(nova.pad_px, 16.0);
        assert_eq!(nova.gap_px, 16.0);
        assert_eq!(nova.title.size_px, 16.0);
        assert_eq!(nova.media_size_px, 40.0);
        assert_eq!(nova.media_icon_px, 24.0);
        assert_eq!(nova.media_gap_x_px, 16.0);

        // Maia: `bg-black/80`, `rounded-4xl` (`--radius-4xl`), `ring-foreground/5`,
        // `rounded-full` media.
        let maia = alert_dialog_recipe(StyleId::Maia);
        assert_eq!(maia.overlay_alpha, 0.80);
        assert_eq!(maia.max_width_px, 448.0);
        assert_eq!(maia.radius, ComponentRadius::S4xl);
        assert_eq!(maia.radius_px, None);
        assert_eq!(maia.ring_alpha, 0.05);
        assert_eq!(maia.ring_alpha_dark, 0.05);
        assert_eq!(maia.media_radius, ComponentRadius::Full);

        // Luma: `bg-black/30`, `shadow-xl`, `dark:ring-foreground/10`.
        let luma = alert_dialog_recipe(StyleId::Luma);
        assert_eq!(luma.overlay_alpha, 0.30);
        assert_eq!(luma.shadow, Some(SHADOW_XL));
        assert_eq!(luma.ring_alpha_dark, 0.10);

        // Rhea caps the radius at 24 px.
        let rhea = alert_dialog_recipe(StyleId::Rhea);
        assert_eq!(rhea.radius_px, Some(24.0));
        assert_eq!(rhea.shadow, Some(SHADOW_XL));

        // Sera: square, `shadow-md`, uppercase wide-tracked semibold title,
        // relaxed description with `mt-0.5`, `gap-2` header.
        let sera = alert_dialog_recipe(StyleId::Sera);
        assert_eq!(sera.radius, ComponentRadius::None);
        assert_eq!(sera.shadow, Some(PopoverShadow::MD));
        assert_eq!(sera.header_gap_px, 8.0);
        assert!(sera.title.uppercase);
        assert_eq!(sera.title.tracking_em, 0.05);
        assert_eq!(sera.title.weight, FontWeight::Semibold);
        assert_eq!(sera.description.line_height_px, 22.75);
        assert_eq!(sera.description_margin_top_px, 2.0);
        assert_eq!(sera.media_radius, ComponentRadius::None);

        // Mira: `bg-black/80 gap-3 p-4`, `max-w-64` at `size="sm"`,
        // `size-8` media with `size-4` glyphs.
        let mira = alert_dialog_recipe(StyleId::Mira);
        assert_eq!(mira.overlay_alpha, 0.80);
        assert_eq!(mira.gap_px, 12.0);
        assert_eq!(mira.max_width_sm_px, 256.0);
        assert_eq!(mira.header_gap_px, 4.0);
        assert_eq!(mira.title.size_px, 14.0);
        assert_eq!(mira.description.line_height_px, 19.5);
        assert_eq!(mira.media_size_px, 32.0);
        assert_eq!(mira.media_icon_px, 16.0);

        // Lyra: square compact pack with `size-10 rounded-none` media.
        let lyra = alert_dialog_recipe(StyleId::Lyra);
        assert_eq!(lyra.radius, ComponentRadius::None);
        assert_eq!(lyra.max_width_px, 384.0);
        assert_eq!(lyra.gap_px, 16.0);
        assert_eq!(lyra.media_size_px, 40.0);
        assert_eq!(lyra.media_radius, ComponentRadius::None);
    }

    #[test]
    fn footer_gap_is_uniform() {
        // `.cn-alert-dialog-footer` ships `gap-2` from the component class
        // list for every pack.
        for style in [
            StyleId::Vega,
            StyleId::Nova,
            StyleId::Maia,
            StyleId::Luma,
            StyleId::Rhea,
            StyleId::Sera,
            StyleId::Mira,
            StyleId::Lyra,
        ] {
            assert_eq!(alert_dialog_recipe(style).footer_gap_px, 8.0);
        }
    }
}
