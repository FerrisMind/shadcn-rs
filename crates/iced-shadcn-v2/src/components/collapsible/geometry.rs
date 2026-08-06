//! Collapsible spacing, footprint, and padding conversion.

use shadcn_common::{ButtonSizeRecipe, ControlSize};
use twill_core::prelude::{Padding, PaddingValue, Spacing};

use super::error::CollapsibleBuildError;
use crate::components::button::ButtonSize;
use crate::theme::Theme;

/// Gap between root slots (`gap-2` in the shadcn examples).
pub(super) const DEFAULT_SPACING: f32 = 8.0;

pub(super) fn resolve_padding(
    padding: Padding,
) -> Result<crate::iced_compat::Padding, CollapsibleBuildError> {
    let (top, right, bottom, left) = padding.sides();

    Ok(crate::iced_compat::Padding {
        top: top.map(padding_value_px).transpose()?.unwrap_or(0.0),
        right: right.map(padding_value_px).transpose()?.unwrap_or(0.0),
        bottom: bottom.map(padding_value_px).transpose()?.unwrap_or(0.0),
        left: left.map(padding_value_px).transpose()?.unwrap_or(0.0),
    })
}

fn padding_value_px(value: PaddingValue) -> Result<f32, CollapsibleBuildError> {
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
            Spacing::Auto => return Err(CollapsibleBuildError::UnsupportedPaddingAuto),
        }),
        PaddingValue::Px(px) => Ok(px.max(0.0)),
        PaddingValue::Rem(rem) => Ok((rem * 16.0).max(0.0)),
        PaddingValue::Var(name) => Err(CollapsibleBuildError::UnsupportedPaddingVariable {
            name: name.as_str(),
        }),
    }
}

/// Clamps a caller-supplied pixel length to a finite, non-negative value.
pub(super) fn normalize_px(value: f32) -> f32 {
    if value.is_finite() {
        value.max(0.0)
    } else {
        0.0
    }
}

/// Button size recipe backing a trigger of the given size.
fn button_recipe(theme: &Theme, size: ButtonSize) -> ButtonSizeRecipe {
    let control = match size {
        ButtonSize::Xs | ButtonSize::IconXs => ControlSize::Xs,
        ButtonSize::Sm | ButtonSize::IconSm => ControlSize::Sm,
        ButtonSize::Lg | ButtonSize::IconLg => ControlSize::Lg,
        _ => ControlSize::Md,
    };

    theme.style.button_size(control)
}

/// Chevron footprint, matching the pack's `[&_svg]:size-*` icon slot.
pub(super) fn indicator_size_px(theme: &Theme, size: ButtonSize) -> f32 {
    button_recipe(theme, size).icon_px.max(1.0)
}

/// Gap between the chevron and the trigger label (`gap-*` of the pack).
pub(super) fn trigger_gap_px(theme: &Theme, size: ButtonSize) -> f32 {
    button_recipe(theme, size).gap_px.max(0.0)
}

/// Trigger label text size for the pack's size ladder.
pub(super) fn trigger_text_size_px(theme: &Theme, size: ButtonSize) -> f32 {
    button_recipe(theme, size).text_size_px.max(1.0)
}
