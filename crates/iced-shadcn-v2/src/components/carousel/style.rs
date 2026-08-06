//! Colors and radii for the carousel prev/next controls.

use crate::iced_compat::Color;

use shadcn_common::AccentColor;
use twill_core::prelude::theme::SemanticColor;

use crate::components::button::ButtonVariant;
use crate::recipes::component_radius_px;
use crate::theme::Theme;

/// Resting glyph color of a control button, matching the text color the
/// button variant resolves for its label content.
pub(super) fn control_icon_color(
    theme: &Theme,
    variant: ButtonVariant,
    color: Option<AccentColor>,
    disabled: bool,
) -> Color {
    if disabled {
        return theme.semantic_color(SemanticColor::MutedForeground);
    }

    let accent_primary = match color {
        None => theme.palette.primary,
        Some(accent) => theme.color_with_accent(accent, SemanticColor::Primary),
    };

    match variant {
        ButtonVariant::Default => match color {
            None => theme.palette.primary_foreground,
            Some(accent) => theme.color_with_accent(accent, SemanticColor::PrimaryForeground),
        },
        ButtonVariant::Secondary => theme.semantic_color(SemanticColor::SecondaryForeground),
        ButtonVariant::Destructive => theme.semantic_color(SemanticColor::Destructive),
        ButtonVariant::Outline | ButtonVariant::Ghost => {
            theme.semantic_color(SemanticColor::Foreground)
        }
        ButtonVariant::Link | ButtonVariant::Soft | ButtonVariant::Surface => accent_primary,
    }
}

/// Corner radius forced onto the controls by the pack's `.cn-carousel-*`
/// CSS, or `None` to keep the default button radius.
pub(super) fn control_radius_px(theme: &Theme) -> Option<f32> {
    theme
        .style
        .carousel()
        .control_radius
        .map(|radius| component_radius_px(theme, radius))
}
