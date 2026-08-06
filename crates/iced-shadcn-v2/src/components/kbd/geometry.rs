//! Kbd dimensions from shadcn-common recipes, plus padding conversion.

use twill_core::prelude::{Padding, PaddingValue, Spacing};

use super::error::KbdBuildError;
use crate::theme::Theme;

/// Default group gap (`gap-1` → 4 px) shared by every style pack.
pub(super) const DEFAULT_KBD_GAP: f32 = 4.0;

/// Gap between adjacent content slots from the style pack recipe.
pub(super) fn gap(theme: &Theme) -> f32 {
    theme.style.kbd().gap_px
}

/// Icon/sidecar footprint from the style pack recipe.
pub(super) fn icon_px(theme: &Theme) -> f32 {
    theme.style.kbd().icon_px
}

/// Control height in px from `.cn-kbd`.
pub(super) fn control_height(theme: &Theme) -> f32 {
    theme.style.kbd().height_px
}

/// Minimum control width in px from `.cn-kbd`.
pub(super) fn min_width(theme: &Theme) -> f32 {
    theme.style.kbd().min_width_px
}

/// Horizontal padding in px from `.cn-kbd`.
pub(super) fn horizontal_padding(theme: &Theme) -> f32 {
    theme.style.kbd().pad_x_px
}

/// Label text size in px from `.cn-kbd`.
pub(super) fn text_size(theme: &Theme) -> f32 {
    theme.style.kbd().typography.size_px
}

/// Default padding: horizontal from the style pack, vertical `0`.
pub(super) fn default_padding(theme: &Theme) -> crate::iced_compat::Padding {
    let horizontal = horizontal_padding(theme);
    crate::iced_compat::Padding {
        top: 0.0,
        right: horizontal,
        bottom: 0.0,
        left: horizontal,
    }
}

pub(super) fn resolve_padding(
    padding: Padding,
) -> Result<crate::iced_compat::Padding, KbdBuildError> {
    let (top, right, bottom, left) = padding.sides();

    Ok(crate::iced_compat::Padding {
        top: top.map(padding_value_px).transpose()?.unwrap_or(0.0),
        right: right.map(padding_value_px).transpose()?.unwrap_or(0.0),
        bottom: bottom.map(padding_value_px).transpose()?.unwrap_or(0.0),
        left: left.map(padding_value_px).transpose()?.unwrap_or(0.0),
    })
}

fn padding_value_px(value: PaddingValue) -> Result<f32, KbdBuildError> {
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
            Spacing::Auto => return Err(KbdBuildError::UnsupportedPaddingAuto),
        }),
        PaddingValue::Px(px) => Ok(px.max(0.0)),
        PaddingValue::Rem(rem) => Ok((rem * 16.0).max(0.0)),
        PaddingValue::Var(name) => Err(KbdBuildError::UnsupportedPaddingVariable {
            name: name.as_str(),
        }),
    }
}
