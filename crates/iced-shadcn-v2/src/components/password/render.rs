//! Input end-padding helpers and trailing-action stack layout.

use shadcn_common::{StyleId, password_end_padding_px};
use twill_core::prelude::{Padding, PaddingValue};

use crate::theme::Theme;

/// Pack `px-*` for `.cn-input`, mirrored from the input style table.
pub(super) fn pack_pad_x(style: StyleId) -> f32 {
    match style {
        StyleId::Maia | StyleId::Luma => 12.0,
        StyleId::Lyra | StyleId::Mira => 8.0,
        StyleId::Sera => 0.0,
        StyleId::Vega | StyleId::Nova | StyleId::Rhea => 10.0,
    }
}

/// Pack value text size for `.cn-input`.
pub(super) fn pack_text_size(style: StyleId) -> f32 {
    match style {
        StyleId::Lyra | StyleId::Mira => 12.0,
        _ => 14.0,
    }
}

/// Absolute line box used by the input geometry module (`text_size + 6`).
pub(super) fn line_height_px(text_size: f32) -> f32 {
    text_size + 6.0
}

/// Builds twill padding that keeps pack left/vertical metrics and applies the
/// extras end-padding for trailing actions.
pub(super) fn input_padding(theme: &Theme, toggle_mounted: bool, copy_mounted: bool) -> Padding {
    let style = theme.style_id();
    let pad_x = pack_pad_x(style);
    let text_size = pack_text_size(style);
    let pad_y = ((theme.style.control_height_md_px - line_height_px(text_size)) / 2.0).max(0.0);
    let end = password_end_padding_px(toggle_mounted, copy_mounted);
    let right = if end > 0.0 { end } else { pad_x };

    Padding::individual_value(
        PaddingValue::Px(pad_y),
        PaddingValue::Px(right),
        PaddingValue::Px(pad_y),
        PaddingValue::Px(pad_x),
    )
}
