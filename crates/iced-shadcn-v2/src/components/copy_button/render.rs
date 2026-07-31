//! Button content helpers for [`super::CopyButton`].

use crate::components::button::{ButtonSize, ButtonVariant};
use crate::fonts::iced_font;
use crate::iced_compat::widget::text as iced_text;
use crate::iced_compat::widget::text::{Fragment, LineHeight};
use crate::iced_compat::{Color, Element};
use crate::theme::Theme;
use shadcn_common::{AccentColor, ControlSize};
use twill_core::prelude::theme::SemanticColor;

/// Resolves the style-pack control ladder for a button size.
pub(super) fn control_size(size: ButtonSize) -> ControlSize {
    match size {
        ButtonSize::Xs | ButtonSize::IconXs => ControlSize::Xs,
        ButtonSize::Sm | ButtonSize::IconSm => ControlSize::Sm,
        ButtonSize::Default | ButtonSize::Icon => ControlSize::Md,
        ButtonSize::Lg | ButtonSize::IconLg => ControlSize::Lg,
    }
}

/// Resolves the icon footprint from the active style pack.
pub(super) fn icon_size(size: ButtonSize, theme: &Theme) -> f32 {
    theme.style.button_size(control_size(size)).icon_px.max(1.0)
}

/// Resolves the fixed `gap-2` used by the source component.
pub(super) const fn content_gap() -> f32 {
    8.0
}

/// Builds a button label with the same typography recipe as [`super::super::Button`].
pub(super) fn label_element<'a, Message>(
    label: Fragment<'a>,
    size: ButtonSize,
    theme: &Theme,
) -> Element<'a, Message>
where
    Message: 'a,
{
    let recipe = theme.style.button_size(control_size(size));
    let type_recipe = theme.style.button_type();
    let mut font = iced_font(theme.font_pack().sans);
    font.weight = crate::recipes::iced_font_weight(type_recipe.typography.weight);
    let text = if type_recipe.typography.uppercase {
        label.as_ref().to_uppercase()
    } else {
        label.into_owned()
    };

    iced_text(text)
        .size(recipe.text_size_px)
        .font(font)
        .line_height(LineHeight::Absolute(recipe.text_size_px.into()))
        .into()
}

/// Resolves the resting icon color so built-in icons follow the same semantic
/// color mapping as the button text.
pub(super) fn icon_color(
    theme: &Theme,
    variant: ButtonVariant,
    color: Option<AccentColor>,
) -> Color {
    let palette = &theme.palette;
    let accent = |token| {
        color
            .map(|accent| theme.color_with_accent(accent, token))
            .unwrap_or_else(|| theme.semantic_color(token))
    };

    match variant {
        ButtonVariant::Default => accent(SemanticColor::PrimaryForeground),
        ButtonVariant::Destructive => palette.destructive,
        ButtonVariant::Outline | ButtonVariant::Ghost => palette.foreground,
        ButtonVariant::Secondary => palette.secondary_foreground,
        ButtonVariant::Link => accent(SemanticColor::Primary),
        ButtonVariant::Soft | ButtonVariant::Surface => accent(SemanticColor::Primary),
    }
}

/// Resolves the icon color used while the button is hovered.
pub(super) fn icon_hover_color(
    theme: &Theme,
    variant: ButtonVariant,
    color: Option<AccentColor>,
) -> Color {
    match variant {
        ButtonVariant::Outline | ButtonVariant::Ghost | ButtonVariant::Secondary => {
            theme.palette.accent_foreground
        }
        ButtonVariant::Link => theme.palette.foreground,
        ButtonVariant::Default
        | ButtonVariant::Destructive
        | ButtonVariant::Soft
        | ButtonVariant::Surface => icon_color(theme, variant, color),
    }
}
