//! Button dimensions and padding conversion.

use iced::Length;
use twill_core::prelude::{Padding, PaddingValue, Spacing};

use super::error::ButtonBuildError;
use super::types::ButtonSize;

pub(super) fn resolve_padding(padding: Padding) -> Result<iced::Padding, ButtonBuildError> {
    let (top, right, bottom, left) = padding.sides();

    Ok(iced::Padding {
        top: top.map(padding_value_px).transpose()?.unwrap_or(0.0),
        right: right.map(padding_value_px).transpose()?.unwrap_or(0.0),
        bottom: bottom.map(padding_value_px).transpose()?.unwrap_or(0.0),
        left: left.map(padding_value_px).transpose()?.unwrap_or(0.0),
    })
}

fn padding_value_px(value: PaddingValue) -> Result<f32, ButtonBuildError> {
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
            Spacing::Auto => return Err(ButtonBuildError::UnsupportedPaddingAuto),
        }),
        PaddingValue::Px(px) => Ok(px.max(0.0)),
        PaddingValue::Rem(rem) => Ok((rem * 16.0).max(0.0)),
        PaddingValue::Var(name) => Err(ButtonBuildError::UnsupportedPaddingVariable {
            name: name.as_str(),
        }),
    }
}

impl ButtonSize {
    /// Control height in px — Tailwind `h-6` / `h-8` / `h-9` / `h-10` (and matching `size-*` for icons).
    pub(super) fn control_height(self) -> f32 {
        match self {
            Self::Xs | Self::IconXs => 24.0,
            Self::Sm | Self::IconSm => 32.0,
            Self::Default | Self::Icon => 36.0,
            Self::Lg | Self::IconLg => 40.0,
        }
    }

    pub(super) fn label_text_size(self) -> u16 {
        match self {
            Self::Xs | Self::IconXs => 12,
            Self::Sm | Self::IconSm | Self::Default | Self::Icon | Self::Lg | Self::IconLg => 14,
        }
    }

    pub(super) fn default_padding(self) -> iced::Padding {
        if self.is_icon() {
            return iced::Padding::ZERO;
        }

        // Horizontal padding from shadcn CSS (`px-2` / `px-2.5`); vertical centering
        // comes from the fixed control height + iced align.
        let horizontal = match self {
            Self::Xs => 8.0,
            Self::Sm | Self::Default | Self::Lg => 10.0,
            Self::IconXs | Self::IconSm | Self::Icon | Self::IconLg => 0.0,
        };

        iced::Padding {
            top: 0.0,
            right: horizontal,
            bottom: 0.0,
            left: horizontal,
        }
    }

    pub(super) fn loading_gap(self) -> f32 {
        match self {
            Self::Xs | Self::IconXs | Self::Sm | Self::IconSm => 4.0,
            Self::Default | Self::Icon | Self::Lg | Self::IconLg => 6.0,
        }
    }
}

pub(super) fn resolve_button_width(
    width: Length,
    height: Length,
    full_width: bool,
    icon: bool,
    default_height: f32,
) -> Length {
    if full_width {
        Length::Fill
    } else if icon {
        match height {
            Length::Fixed(height) => Length::Fixed(height),
            _ => Length::Fixed(default_height),
        }
    } else {
        width
    }
}
