//! Button dimensions and padding conversion.

use iced::Length;
use twill_core::prelude::{Padding, PaddingValue, Spacing};

use super::error::ButtonBuildError;
use super::types::ButtonSize;
use crate::theme::Theme;

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
    pub(super) fn control_height(self, theme: &Theme) -> f32 {
        match self {
            Self::Size0 => theme.style.control_height_sm_px - 8.0,
            Self::Size1 => theme.style.control_height_sm_px,
            Self::Size2 => theme.style.control_height_md_px,
            Self::Size3 => theme.style.control_height_lg_px,
            Self::Size4 => theme.style.control_height_lg_px + 8.0,
        }
        .max(0.0)
    }

    pub(super) fn label_text_size(self) -> u16 {
        match self {
            Self::Size0 => 12,
            Self::Size1 | Self::Size2 | Self::Size3 => 14,
            Self::Size4 => 16,
        }
    }

    pub(super) fn default_padding(self) -> iced::Padding {
        let (vertical, horizontal) = match self {
            Self::Size0 => (4.0, 8.0),
            Self::Size1 => (6.0, 12.0),
            Self::Size2 => (8.0, 16.0),
            Self::Size3 => (10.0, 24.0),
            Self::Size4 => (12.0, 28.0),
        };

        iced::Padding {
            top: vertical,
            right: horizontal,
            bottom: vertical,
            left: horizontal,
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
