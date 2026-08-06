//! Style resolution for typography — thin iced adapter over shared recipes.

use crate::iced_compat::font::Style as FontStyle;
use crate::iced_compat::{Color, Font};

use super::types::TypographyVariant;
use crate::fonts::iced_font;
use crate::recipes::iced_font_weight;
use crate::theme::Theme;

/// `border-b` thickness under `h2` and the table grid stroke, in px.
pub(super) const RULE_PX: f32 = 1.0;

/// `pb-2` gap between `h2` text and its underline, in px.
pub(super) const H2_UNDERLINE_GAP_PX: f32 = 8.0;

/// `border-s-2` leading bar width of a blockquote, in px.
pub(super) const BLOCKQUOTE_BAR_PX: f32 = 2.0;

/// `ps-6` inset between the blockquote bar and its text, in px.
pub(super) const BLOCKQUOTE_INSET_PX: f32 = 24.0;

/// Inline-code chip padding: `px-[0.3rem]` / `py-[0.2rem]`, in px.
pub(super) const INLINE_CODE_PADDING_X_PX: f32 = 4.8;
pub(super) const INLINE_CODE_PADDING_Y_PX: f32 = 3.2;

/// `ms-6` list indent, in px.
pub(super) const LIST_INDENT_PX: f32 = 24.0;

/// `[&>li]:mt-2` gap between list items, in px.
pub(super) const LIST_ITEM_GAP_PX: f32 = 8.0;

/// Gap between a list bullet and its item text, in px.
pub(super) const LIST_MARKER_GAP_PX: f32 = 8.0;

/// `list-disc` marker glyph.
pub(super) const LIST_MARKER: &str = "•";

/// Table cell padding: `px-4` / `py-2`, in px.
pub(super) const TABLE_CELL_PADDING_X_PX: f32 = 16.0;
pub(super) const TABLE_CELL_PADDING_Y_PX: f32 = 8.0;

/// Resolves the iced font (family, weight, italics) for a variant.
pub(super) fn resolve_font(theme: &Theme, variant: TypographyVariant) -> Font {
    let pack = theme.font_pack();
    let family = if variant.uses_heading_font() {
        pack.heading
    } else if variant.uses_mono_font() {
        pack.mono
    } else {
        pack.sans
    };

    let mut font = iced_font(family);
    font.weight = iced_font_weight(variant.type_recipe().weight);
    if variant.is_italic() {
        font.style = FontStyle::Italic;
    }
    font
}

/// Resolves the foreground color (explicit override beats variant defaults).
pub(super) fn resolve_color(
    theme: &Theme,
    variant: TypographyVariant,
    color: Option<Color>,
) -> Color {
    color.unwrap_or(if variant.is_muted() {
        theme.palette.muted_foreground
    } else {
        theme.palette.foreground
    })
}
