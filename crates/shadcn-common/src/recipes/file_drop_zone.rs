//! File-drop-zone geometry from shadcn-svelte-extras `FileDropZone.Trigger`.
//!
//! FileDropZone itself is pack-invariant in the extras source (hard-coded
//! Tailwind: `h-48`, `p-6`, `gap-2`, `rounded-lg`, `size-14` / `size-7`, … —
//! no per-pack `.cn-*` tables). Pack-specific look comes from the **shared**
//! [`crate::style::StylePack`] on the app theme:
//!
//! - `rounded-lg` resolves through [`ComponentRadius::Lg`] → pack twill radius
//! - colors / fonts come from the same [`crate::theme::ResolvedTheme`]
//! - composed controls (Button, Progress, …) use their own pack-aware recipes
//!
//! Pass that same theme into every part — do not invent a separate FileDropZone
//! style table that branches on [`StyleId`].

use crate::style::StyleId;

use super::{ComponentRadius, FontWeight, TypeRecipe};

/// Default trigger height (`h-48` → 192 px).
pub const HEIGHT_PX: f32 = 192.0;
/// Equal inset (`p-6` → 24 px).
pub const PADDING_PX: f32 = 24.0;
/// Gap between icon ring and text stack (`gap-2` → 8 px).
pub const GAP_PX: f32 = 8.0;
/// Gap inside the text stack (`gap-0.5` → 2 px).
pub const TEXT_GAP_PX: f32 = 2.0;
/// Upload icon ring diameter (`size-14` → 56 px).
pub const ICON_CIRCLE_PX: f32 = 56.0;
/// Lucide upload glyph footprint (`size-7` → 28 px).
pub const ICON_PX: f32 = 28.0;
/// Disabled opacity (`group-aria-disabled:opacity-50`).
pub const DISABLED_OPACITY: f32 = 0.5;
/// Hover fill alpha for `hover:bg-accent/25`.
pub const HOVER_ACCENT_ALPHA: f32 = 0.25;
/// Hint text alpha for `text-muted-foreground/75`.
pub const HINT_FOREGROUND_ALPHA: f32 = 0.75;
/// Visible dashed border width.
pub const BORDER_WIDTH_PX: f32 = 1.0;
/// Lucide viewBox size used when stroking the upload path.
pub const ICON_VIEWBOX: f32 = 24.0;
/// Lucide default stroke width inside the 24×24 viewBox.
pub const ICON_STROKE_VIEWBOX: f32 = 2.0;

/// Geometry and typography tokens for one file-drop-zone trigger.
///
/// Layout numbers are identical for every [`StyleId`]. Pack identity still
/// matters when backends resolve [`Self::radius`] against the theme's radius
/// scale (Rhea `lg` ≠ Sera `lg`).
///
/// ```rust
/// use shadcn_common::{StyleId, file_drop_zone_recipe};
///
/// let recipe = file_drop_zone_recipe(StyleId::Vega);
/// assert_eq!(recipe.height_px, 192.0);
/// assert_eq!(recipe.icon_circle_px, 56.0);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FileDropZoneRecipe {
    /// Minimum / default trigger height (`h-48`).
    pub height_px: f32,
    /// Equal padding (`p-6`).
    pub padding_px: f32,
    /// Gap between the icon ring and the text column (`gap-2`).
    pub gap_px: f32,
    /// Gap between title and hint (`gap-0.5`).
    pub text_gap_px: f32,
    /// Corner radius intent of the dashed surface (`rounded-lg`).
    pub radius: ComponentRadius,
    /// Upload icon ring diameter (`size-14`).
    pub icon_circle_px: f32,
    /// Upload glyph edge length (`size-7`).
    pub icon_px: f32,
    /// Dashed border width.
    pub border_width_px: f32,
    /// Opacity when `aria-disabled`.
    pub disabled_opacity: f32,
    /// Accent fill alpha on hover.
    pub hover_accent_alpha: f32,
    /// Muted-foreground alpha for the secondary hint line.
    pub hint_foreground_alpha: f32,
    /// Primary label typography (`font-medium`, muted).
    pub title: TypeRecipe,
    /// Secondary hint typography (`text-sm`).
    pub hint: TypeRecipe,
}

impl Default for FileDropZoneRecipe {
    fn default() -> Self {
        file_drop_zone_recipe(StyleId::Vega)
    }
}

/// Returns the pack-invariant FileDropZone layout/typography tokens.
///
/// `style` is accepted for API symmetry with other recipes but is unused for
/// the numeric table: selecting Rhea (or any pack) on the theme still styles
/// the zone because backends resolve [`FileDropZoneRecipe::radius`] and
/// palette/fonts from that theme, and composed children call their own
/// pack-aware recipes with `theme.style_id()`.
#[must_use]
pub const fn file_drop_zone_recipe(style: StyleId) -> FileDropZoneRecipe {
    let _ = style;
    FileDropZoneRecipe {
        height_px: HEIGHT_PX,
        padding_px: PADDING_PX,
        gap_px: GAP_PX,
        text_gap_px: TEXT_GAP_PX,
        radius: ComponentRadius::Lg,
        icon_circle_px: ICON_CIRCLE_PX,
        icon_px: ICON_PX,
        border_width_px: BORDER_WIDTH_PX,
        disabled_opacity: DISABLED_OPACITY,
        hover_accent_alpha: HOVER_ACCENT_ALPHA,
        hint_foreground_alpha: HINT_FOREGROUND_ALPHA,
        title: TypeRecipe {
            size_px: 16.0,
            weight: FontWeight::Medium,
            uppercase: false,
            tracking_em: 0.0,
            line_height_px: 24.0,
        },
        hint: TypeRecipe {
            size_px: 14.0,
            weight: FontWeight::Normal,
            uppercase: false,
            tracking_em: 0.0,
            line_height_px: 20.0,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recipe_matches_extras_tailwind() {
        for style in StyleId::ALL {
            let recipe = file_drop_zone_recipe(style);
            assert_eq!(recipe.height_px, 192.0);
            assert_eq!(recipe.padding_px, 24.0);
            assert_eq!(recipe.gap_px, 8.0);
            assert_eq!(recipe.icon_circle_px, 56.0);
            assert_eq!(recipe.icon_px, 28.0);
            assert_eq!(recipe.disabled_opacity, 0.5);
            assert_eq!(recipe.hover_accent_alpha, 0.25);
            assert_eq!(recipe.radius, ComponentRadius::Lg);
            assert_eq!(recipe.title.size_px, 16.0);
            assert_eq!(recipe.hint.size_px, 14.0);
        }
    }

    #[test]
    fn recipe_table_is_identical_across_packs() {
        let vega = file_drop_zone_recipe(StyleId::Vega);
        for style in StyleId::ALL {
            assert_eq!(file_drop_zone_recipe(style), vega);
        }
    }
}
