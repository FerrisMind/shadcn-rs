//! Colors the collapsible has to resolve itself, because iced cannot inherit them.

use crate::iced_compat::widget::container;
use crate::iced_compat::{Background, Border, Color};

use shadcn_common::AccentColor;
use twill_core::prelude::theme::SemanticColor;

use crate::components::button::ButtonVariant;
use crate::theme::Theme;

/// Border width of a bordered surface (`border` → 1 px in the web packs).
const BORDER_WIDTH: f32 = 1.0;

/// Surface style shared by the collapsible root and its content panel.
#[derive(Debug, Clone, Copy)]
pub(super) struct Surface {
    pub(super) background: Option<SemanticColor>,
    pub(super) bordered: bool,
    pub(super) radius: Option<f32>,
}

impl Surface {
    pub(super) const NONE: Self = Self {
        background: None,
        bordered: false,
        radius: None,
    };
}

/// Resolves a surface onto an iced container style.
pub(super) fn resolve_surface(theme: &Theme, surface: Surface) -> container::Style {
    container::Style {
        text_color: surface
            .background
            .map(|slot| theme.semantic_foreground(slot)),
        background: surface
            .background
            .map(|slot| Background::Color(theme.semantic_color(slot))),
        border: Border {
            radius: surface.radius.unwrap_or(0.0).into(),
            width: if surface.bordered { BORDER_WIDTH } else { 0.0 },
            color: if surface.bordered {
                theme.semantic_color(SemanticColor::Border)
            } else {
                Color::TRANSPARENT
            },
        },
        ..container::Style::default()
    }
}

/// Resting label color of a trigger button.
///
/// A canvas cannot inherit `button::Style::text_color`, so the chevron
/// indicator has to resolve the color the label is painted with. The mapping
/// mirrors the resting visuals of [`crate::Button`].
pub(super) fn trigger_text_color(
    theme: &Theme,
    variant: ButtonVariant,
    color: Option<AccentColor>,
    disabled: bool,
) -> Color {
    if disabled {
        return theme.semantic_color(SemanticColor::MutedForeground);
    }

    let accent = match color {
        None => theme.palette.primary,
        Some(accent) => theme.color_with_accent(accent, SemanticColor::Primary),
    };

    match variant {
        ButtonVariant::Default => match color {
            None => theme.palette.primary_foreground,
            Some(accent) => theme.color_with_accent(accent, SemanticColor::PrimaryForeground),
        },
        ButtonVariant::Destructive => theme.semantic_color(SemanticColor::Destructive),
        ButtonVariant::Secondary => theme.semantic_color(SemanticColor::SecondaryForeground),
        ButtonVariant::Outline | ButtonVariant::Ghost => {
            theme.semantic_color(SemanticColor::Foreground)
        }
        ButtonVariant::Link | ButtonVariant::Soft | ButtonVariant::Surface => accent,
    }
}
