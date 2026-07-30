//! Accordion spacing, padding conversion, and trigger metrics.

use crate::components::button::{ButtonRadius, ButtonSize};
use crate::theme::Theme;
use shadcn_common::{ButtonSizeRecipe, ControlSize, FontWeight, StyleId};
use twill_core::prelude::{Padding, PaddingValue, Spacing};

use super::error::AccordionBuildError;

/// Style-pack metrics copied from the accordion selectors in shadcn-svelte.
#[derive(Debug, Clone, Copy)]
pub(super) struct AccordionMetrics {
    pub(super) trigger_padding_y: f32,
    pub(super) content_padding_x: f32,
    pub(super) content_padding_bottom: f32,
    pub(super) trigger_text_size_px: f32,
    pub(super) trigger_line_height_px: f32,
    pub(super) content_text_size_px: f32,
    pub(super) content_line_height_px: f32,
    pub(super) trigger_weight: FontWeight,
    pub(super) trigger_icon_px: f32,
    pub(super) trigger_gap_px: f32,
    pub(super) trigger_radius: ButtonRadius,
}

/// Resolves the accordion-specific CSS contract for a style pack.
pub(super) fn metrics(theme: &Theme) -> AccordionMetrics {
    match theme.style_id() {
        StyleId::Nova => AccordionMetrics {
            trigger_padding_y: 10.0,
            content_padding_x: 0.0,
            content_padding_bottom: 10.0,
            trigger_text_size_px: 14.0,
            trigger_line_height_px: 20.0,
            content_text_size_px: 14.0,
            content_line_height_px: 20.0,
            trigger_weight: FontWeight::Medium,
            trigger_icon_px: 16.0,
            trigger_gap_px: 0.0,
            trigger_radius: ButtonRadius::Large,
        },
        StyleId::Vega => AccordionMetrics {
            trigger_padding_y: 16.0,
            content_padding_x: 0.0,
            content_padding_bottom: 16.0,
            trigger_text_size_px: 14.0,
            trigger_line_height_px: 20.0,
            content_text_size_px: 14.0,
            content_line_height_px: 20.0,
            trigger_weight: FontWeight::Medium,
            trigger_icon_px: 16.0,
            trigger_gap_px: 0.0,
            trigger_radius: ButtonRadius::Medium,
        },
        StyleId::Maia => AccordionMetrics {
            trigger_padding_y: 16.0,
            content_padding_x: 16.0,
            content_padding_bottom: 16.0,
            trigger_text_size_px: 14.0,
            trigger_line_height_px: 20.0,
            content_text_size_px: 14.0,
            content_line_height_px: 20.0,
            trigger_weight: FontWeight::Medium,
            trigger_icon_px: 16.0,
            trigger_gap_px: 24.0,
            trigger_radius: ButtonRadius::None,
        },
        StyleId::Lyra => AccordionMetrics {
            trigger_padding_y: 10.0,
            content_padding_x: 0.0,
            content_padding_bottom: 10.0,
            trigger_text_size_px: 12.0,
            trigger_line_height_px: 16.0,
            content_text_size_px: 12.0,
            content_line_height_px: 16.0,
            trigger_weight: FontWeight::Medium,
            trigger_icon_px: 16.0,
            trigger_gap_px: 0.0,
            trigger_radius: ButtonRadius::None,
        },
        StyleId::Mira => AccordionMetrics {
            trigger_padding_y: 8.0,
            content_padding_x: 8.0,
            content_padding_bottom: 16.0,
            trigger_text_size_px: 12.0,
            trigger_line_height_px: 19.5,
            content_text_size_px: 12.0,
            content_line_height_px: 19.5,
            trigger_weight: FontWeight::Medium,
            trigger_icon_px: 16.0,
            trigger_gap_px: 24.0,
            trigger_radius: ButtonRadius::None,
        },
        StyleId::Luma => AccordionMetrics {
            trigger_padding_y: 16.0,
            content_padding_x: 16.0,
            content_padding_bottom: 16.0,
            trigger_text_size_px: 14.0,
            trigger_line_height_px: 20.0,
            content_text_size_px: 14.0,
            content_line_height_px: 20.0,
            trigger_weight: FontWeight::Medium,
            trigger_icon_px: 16.0,
            trigger_gap_px: 24.0,
            trigger_radius: ButtonRadius::None,
        },
        StyleId::Sera => AccordionMetrics {
            trigger_padding_y: 16.0,
            content_padding_x: 0.0,
            content_padding_bottom: 16.0,
            trigger_text_size_px: 14.0,
            trigger_line_height_px: 20.0,
            content_text_size_px: 14.0,
            content_line_height_px: 20.0,
            trigger_weight: FontWeight::Semibold,
            trigger_icon_px: 14.0,
            trigger_gap_px: 24.0,
            trigger_radius: ButtonRadius::None,
        },
        StyleId::Rhea => AccordionMetrics {
            trigger_padding_y: 16.0,
            content_padding_x: 16.0,
            content_padding_bottom: 16.0,
            trigger_text_size_px: 14.0,
            trigger_line_height_px: 20.0,
            content_text_size_px: 14.0,
            content_line_height_px: 20.0,
            trigger_weight: FontWeight::Medium,
            trigger_icon_px: 16.0,
            trigger_gap_px: 24.0,
            trigger_radius: ButtonRadius::None,
        },
    }
}

/// Returns whether the active style puts a border around the accordion root.
pub(super) fn default_root_bordered(theme: &Theme) -> bool {
    matches!(
        theme.style_id(),
        StyleId::Maia | StyleId::Mira | StyleId::Luma | StyleId::Rhea
    )
}

/// Resolves the root radius from the style-pack `.cn-accordion` selector.
pub(super) fn default_root_radius(theme: &Theme) -> f32 {
    let scale = theme.radius_scale();

    match theme.style_id() {
        StyleId::Maia | StyleId::Luma | StyleId::Rhea => scale.xxl_px,
        StyleId::Mira => scale.md_px,
        StyleId::Vega | StyleId::Nova | StyleId::Lyra | StyleId::Sera => 0.0,
    }
}

/// Returns whether an open item receives the source `bg-muted/50` surface.
pub(super) fn default_open_item_background(theme: &Theme) -> bool {
    matches!(
        theme.style_id(),
        StyleId::Maia | StyleId::Mira | StyleId::Luma | StyleId::Rhea
    )
}

/// Converts a twill padding value to an iced padding value.
pub(super) fn resolve_padding(
    padding: Padding,
) -> Result<crate::iced_compat::Padding, AccordionBuildError> {
    let (top, right, bottom, left) = padding.sides();

    Ok(crate::iced_compat::Padding {
        top: top.map(padding_value_px).transpose()?.unwrap_or(0.0),
        right: right.map(padding_value_px).transpose()?.unwrap_or(0.0),
        bottom: bottom.map(padding_value_px).transpose()?.unwrap_or(0.0),
        left: left.map(padding_value_px).transpose()?.unwrap_or(0.0),
    })
}

fn padding_value_px(value: PaddingValue) -> Result<f32, AccordionBuildError> {
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
            Spacing::Auto => return Err(AccordionBuildError::UnsupportedPaddingAuto),
        }),
        PaddingValue::Px(px) => Ok(px.max(0.0)),
        PaddingValue::Rem(rem) => Ok((rem * 16.0).max(0.0)),
        PaddingValue::Var(name) => Err(AccordionBuildError::UnsupportedPaddingVariable {
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

fn button_recipe(theme: &Theme, size: ButtonSize) -> ButtonSizeRecipe {
    let control = match size {
        ButtonSize::Xs | ButtonSize::IconXs => ControlSize::Xs,
        ButtonSize::Sm | ButtonSize::IconSm => ControlSize::Sm,
        ButtonSize::Lg | ButtonSize::IconLg => ControlSize::Lg,
        _ => ControlSize::Md,
    };

    theme.style.button_size(control)
}

/// Resolves the trigger text size from the active style pack and button size.
pub(super) fn trigger_text_size_px(theme: &Theme, size: ButtonSize) -> f32 {
    if matches!(size, ButtonSize::Default | ButtonSize::Icon) {
        metrics(theme).trigger_text_size_px
    } else {
        button_recipe(theme, size).text_size_px.max(1.0)
    }
}

/// Resolves the trigger line height from the active style pack and button size.
pub(super) fn trigger_line_height_px(theme: &Theme, size: ButtonSize) -> f32 {
    if matches!(size, ButtonSize::Default | ButtonSize::Icon) {
        metrics(theme).trigger_line_height_px
    } else {
        button_recipe(theme, size).text_size_px.max(1.0)
    }
}

/// Resolves the trigger font weight from the active style pack.
pub(super) fn trigger_weight(theme: &Theme) -> FontWeight {
    metrics(theme).trigger_weight
}

/// Resolves the trigger icon footprint from the active style pack.
pub(super) fn trigger_icon_size_px(theme: &Theme, size: ButtonSize) -> f32 {
    if matches!(size, ButtonSize::Default | ButtonSize::Icon) {
        metrics(theme).trigger_icon_px
    } else {
        button_recipe(theme, size).icon_px.max(1.0)
    }
}

/// Resolves the trigger gap from the active style pack.
pub(super) fn trigger_gap_px(theme: &Theme, size: ButtonSize) -> f32 {
    if matches!(size, ButtonSize::Default | ButtonSize::Icon) {
        metrics(theme).trigger_gap_px
    } else {
        button_recipe(theme, size).gap_px.max(0.0)
    }
}

/// Resolves the default trigger radius from the active accordion selector.
pub(super) fn default_trigger_radius(theme: &Theme) -> ButtonRadius {
    metrics(theme).trigger_radius
}

/// Builds the default trigger padding (`py-*`, no horizontal padding).
pub(super) fn default_trigger_padding(theme: &Theme) -> Padding {
    let padding = metrics(theme).trigger_padding_y;
    Padding::individual_value(
        PaddingValue::Px(padding),
        PaddingValue::Px(0.0),
        PaddingValue::Px(padding),
        PaddingValue::Px(0.0),
    )
}

/// Resolves the default content surface padding from the active style pack.
pub(super) fn default_content_padding(theme: &Theme) -> crate::iced_compat::Padding {
    let metrics = metrics(theme);
    crate::iced_compat::Padding {
        top: 0.0,
        right: metrics.content_padding_x,
        bottom: metrics.content_padding_bottom,
        left: metrics.content_padding_x,
    }
}

/// Resolves the convenience text content typography from the active style.
pub(super) fn content_text_metrics(theme: &Theme) -> (f32, f32) {
    let metrics = metrics(theme);
    (metrics.content_text_size_px, metrics.content_line_height_px)
}
