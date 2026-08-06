//! Textarea dimensions derived from the shared [`TextareaRecipe`].
//!
//! `.cn-textarea` fixes a `min-h-16` minimum height per style pack; the size
//! ladder scales it (`Sm` → 64, `Default` → 64, `Lg` → 96) and the text size.
//! Padding comes straight from the recipe's `px-*` / `py-*` tokens.

use iced_core::text::Wrapping;

use shadcn_common::{MIN_HEIGHT_PX, TextareaRecipe};

use super::types::TextareaSize;

/// `px-*` / `py-*` of the active pack as `[vertical, horizontal]`.
pub(super) fn pack_padding(recipe: TextareaRecipe, size: TextareaSize) -> [f32; 2] {
    let vertical = recipe.pad_y_px;
    let horizontal = recipe.pad_x_px;

    match size {
        TextareaSize::Sm => [vertical.max(6.0), horizontal],
        TextareaSize::Default => [vertical, horizontal],
        TextareaSize::Lg => [vertical + 2.0, horizontal + 2.0],
    }
}

/// `md:text-sm` / `md:text-xs` of the active pack, scaled by the size ladder.
pub(super) fn pack_text_size(recipe: TextareaRecipe, size: TextareaSize) -> f32 {
    let pack_size = recipe.text_size_px;

    match size {
        TextareaSize::Sm => (pack_size - 1.0).max(1.0),
        TextareaSize::Default => pack_size,
        TextareaSize::Lg => pack_size + 2.0,
    }
}

/// Absolute line height reserved for one row of value glyphs.
pub(super) fn line_height_px(text_size: f32) -> f32 {
    text_size * 1.4
}

/// Minimum textarea height, honoring explicit `rows` when supplied.
///
/// `min-h-16` (64px) is shared by every pack; `Lg` jumps to `min-h-24` (96px).
pub(super) fn min_height(
    size: TextareaSize,
    text_size: f32,
    padding: [f32; 2],
    rows: Option<usize>,
) -> f32 {
    if let Some(rows) = rows {
        return line_height_px(text_size) * rows.max(1) as f32 + padding[0] * 2.0;
    }

    match size {
        TextareaSize::Sm => MIN_HEIGHT_PX,
        TextareaSize::Default => MIN_HEIGHT_PX,
        TextareaSize::Lg => 96.0,
    }
}

/// Maximum textarea height, using explicit row limits when supplied.
pub(super) fn max_height(
    text_size: f32,
    padding: [f32; 2],
    max_rows: Option<usize>,
) -> Option<f32> {
    let rows = max_rows?;
    Some(line_height_px(text_size) * rows.max(1) as f32 + padding[0] * 2.0)
}

/// Default wrapping strategy matching the web component (`overflow-wrap: anywhere`
/// approximated by `WordOrGlyph`).
pub(super) const fn default_wrapping() -> Wrapping {
    Wrapping::WordOrGlyph
}
