//! Input dimensions and padding conversion.
//!
//! `.cn-input` fixes the control height per style pack (`h-9` on Vega, `h-7`
//! on Mira, …), which matches the pack's `md` control-height slot. iced sizes
//! a `text_input` from `line_height + vertical padding`, so the default
//! padding is derived backwards from the target height.

use twill_core::prelude::{Padding, PaddingValue, Spacing};

use super::error::InputBuildError;
use super::types::InputSize;
use crate::theme::Theme;

pub(super) fn resolve_padding(
    padding: Padding,
) -> Result<crate::iced_compat::Padding, InputBuildError> {
    let (top, right, bottom, left) = padding.sides();

    Ok(crate::iced_compat::Padding {
        top: top.map(padding_value_px).transpose()?.unwrap_or(0.0),
        right: right.map(padding_value_px).transpose()?.unwrap_or(0.0),
        bottom: bottom.map(padding_value_px).transpose()?.unwrap_or(0.0),
        left: left.map(padding_value_px).transpose()?.unwrap_or(0.0),
    })
}

fn padding_value_px(value: PaddingValue) -> Result<f32, InputBuildError> {
    match value {
        PaddingValue::Scale(scale) => Ok(match scale {
            Spacing::S0 => 0.0,
            Spacing::Px => 1.0,
            Spacing::S0_5 => 2.0,
            Spacing::S1 => 4.0,
            Spacing::S1_5 => 6.0,
            Spacing::S2 => 8.0,
            Spacing::S2_5 => 10.0,
            Spacing::S3 => 12.0,
            Spacing::S3_5 => 14.0,
            Spacing::S4 => 16.0,
            Spacing::S5 => 20.0,
            Spacing::S6 => 24.0,
            Spacing::S7 => 28.0,
            Spacing::S8 => 32.0,
            Spacing::S9 => 36.0,
            Spacing::S10 => 40.0,
            Spacing::S11 => 44.0,
            Spacing::S12 => 48.0,
            Spacing::S14 => 56.0,
            Spacing::S16 => 64.0,
            Spacing::S20 => 80.0,
            Spacing::S24 => 96.0,
            Spacing::S28 => 112.0,
            Spacing::S32 => 128.0,
            Spacing::S36 => 144.0,
            Spacing::S40 => 160.0,
            Spacing::S44 => 176.0,
            Spacing::S48 => 192.0,
            Spacing::S52 => 208.0,
            Spacing::S56 => 224.0,
            Spacing::S60 => 240.0,
            Spacing::S64 => 256.0,
            Spacing::S72 => 288.0,
            Spacing::S80 => 320.0,
            Spacing::S96 => 384.0,
            Spacing::Auto => return Err(InputBuildError::UnsupportedPaddingAuto),
        }),
        PaddingValue::Px(px) => Ok(px.max(0.0)),
        PaddingValue::Rem(rem) => Ok((rem * 16.0).max(0.0)),
        PaddingValue::Var(name) => Err(InputBuildError::UnsupportedPaddingVariable {
            name: name.as_str(),
        }),
    }
}

impl InputSize {
    /// Control height in px from the style pack size ladder.
    ///
    /// [`InputSize::Default`] equals the pack's `.cn-input` height.
    pub(super) fn control_height(self, theme: &Theme) -> f32 {
        match self {
            Self::Sm => theme.style.control_height_sm_px,
            Self::Default => theme.style.control_height_md_px,
            Self::Lg => theme.style.control_height_lg_px,
        }
    }
}

/// Line box reserved for the value glyphs.
///
/// An absolute line height keeps the control height exact regardless of the
/// font's own metrics (same trick as the button label).
pub(super) fn line_height_px(text_size: f32) -> f32 {
    text_size + 6.0
}

/// Optical nudge: iced centers `paragraph.min_bounds()` in the Absolute line
/// box, and Geist (and similar UI fonts) sit slightly low in that box. Shift
/// 1px of vertical padding upward without changing the control height.
const VALUE_OPTICAL_NUDGE_PX: f32 = 1.0;

/// Default padding recreating `.cn-input` (`px-*` from the pack, `py`
/// derived from the fixed control height).
pub(super) fn default_padding(
    theme: &Theme,
    size: InputSize,
    text_size: f32,
) -> crate::iced_compat::Padding {
    let pad_x = super::style::pack_pad_x(theme);
    let pad_y = ((size.control_height(theme) - line_height_px(text_size)) / 2.0).max(0.0);
    let nudge = VALUE_OPTICAL_NUDGE_PX.min(pad_y);

    crate::iced_compat::Padding {
        top: pad_y - nudge,
        right: pad_x,
        bottom: pad_y + nudge,
        left: pad_x,
    }
}
