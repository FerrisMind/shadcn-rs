//! Mapping of semantic button variants to iced styles.

use crate::iced_compat::border::Border;
use crate::iced_compat::widget::button as button_widget;
use crate::iced_compat::{Color, Shadow, Vector};

use shadcn_common::AccentColor;
use twill_core::prelude::theme::SemanticColor;

use super::{ButtonRadius, ButtonVariant};
use crate::recipes::component_radius_px;
use crate::theme::Theme;

/// Resolved visual state of a button: everything iced needs for one status.
#[derive(Debug, Clone, Copy)]
struct Visual {
    background: Option<Color>,
    text: Color,
    border_color: Color,
    border_width: f32,
    shadow: Option<Shadow>,
}

pub(super) fn resolve_button_style(
    theme: &Theme,
    variant: ButtonVariant,
    radius: Option<ButtonRadius>,
    color: Option<AccentColor>,
    disabled: bool,
    status: button_widget::Status,
) -> button_widget::Style {
    let base = base_visual(theme, variant, color);

    let visual = match status {
        button_widget::Status::Hovered => hovered_visual(theme, variant, color, base),
        button_widget::Status::Pressed => pressed_visual(theme, variant, color, base),
        button_widget::Status::Disabled => {
            if disabled {
                disabled_visual(theme)
            } else {
                base
            }
        }
        button_widget::Status::Active => base,
    };

    button_widget::Style {
        background: visual
            .background
            .filter(|color| color.a > f32::EPSILON)
            .map(crate::iced_compat::Background::Color),
        text_color: visual.text,
        border: Border {
            radius: resolve_radius_px(theme, radius).into(),
            width: visual.border_width,
            color: visual.border_color,
        },
        shadow: visual.shadow.unwrap_or_default(),
        snap: true,
    }
}

fn base_visual(theme: &Theme, variant: ButtonVariant, color: Option<AccentColor>) -> Visual {
    let accent = accent_fill(theme, color);
    let accent_fg = accent_on_fill(theme, color);
    let accent_txt = accent_text(theme, color);
    let soft_bg = accent_soft_fill(theme, color);

    match variant {
        ButtonVariant::Default => Visual {
            background: Some(accent),
            text: accent_fg,
            border_color: accent,
            border_width: 0.0,
            shadow: None,
        },
        ButtonVariant::Secondary => Visual {
            background: Some(theme.semantic_color(SemanticColor::Secondary)),
            text: theme.semantic_color(SemanticColor::SecondaryForeground),
            border_color: theme.semantic_color(SemanticColor::Secondary),
            border_width: 0.0,
            shadow: None,
        },
        ButtonVariant::Destructive => {
            // shadcn: `bg-destructive/10 text-destructive` (dark: `/20`)
            let destructive = theme.semantic_color(SemanticColor::Destructive);
            Visual {
                background: Some(destructive_soft_fill(
                    theme,
                    destructive_soft_alpha(theme, SoftState::Base),
                )),
                text: destructive,
                border_color: Color::TRANSPARENT,
                border_width: 0.0,
                shadow: None,
            }
        }
        ButtonVariant::Outline => Visual {
            background: None,
            text: theme.semantic_color(SemanticColor::Foreground),
            border_color: theme.semantic_color(SemanticColor::Input),
            border_width: 1.0,
            shadow: None,
        },
        ButtonVariant::Ghost => Visual {
            background: None,
            text: theme.semantic_color(SemanticColor::Foreground),
            border_color: Color::TRANSPARENT,
            border_width: 0.0,
            shadow: None,
        },
        ButtonVariant::Link => Visual {
            background: None,
            text: accent,
            border_color: Color::TRANSPARENT,
            border_width: 0.0,
            shadow: None,
        },
        ButtonVariant::Soft => Visual {
            background: Some(soft_bg),
            text: accent_txt,
            border_color: soft_bg,
            border_width: 0.0,
            shadow: None,
        },
        ButtonVariant::Surface => Visual {
            background: Some(theme.semantic_color(SemanticColor::Background)),
            text: accent_txt,
            border_color: theme.semantic_color(SemanticColor::Border),
            border_width: 1.0,
            shadow: Some(Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.10),
                offset: Vector::new(0.0, 1.0),
                blur_radius: 3.0,
            }),
        },
    }
}

fn hovered_visual(
    theme: &Theme,
    variant: ButtonVariant,
    color: Option<AccentColor>,
    base: Visual,
) -> Visual {
    match variant {
        ButtonVariant::Default => Visual {
            background: Some(shift_toward(
                accent_fill(theme, color),
                theme.is_dark(),
                0.12,
            )),
            ..base
        },
        ButtonVariant::Secondary | ButtonVariant::Outline | ButtonVariant::Ghost => Visual {
            background: Some(theme.semantic_color(SemanticColor::Accent)),
            text: theme.semantic_color(SemanticColor::AccentForeground),
            ..base
        },
        ButtonVariant::Destructive => Visual {
            background: Some(destructive_soft_fill(
                theme,
                destructive_soft_alpha(theme, SoftState::Hover),
            )),
            text: theme.semantic_color(SemanticColor::Destructive),
            ..base
        },
        ButtonVariant::Soft | ButtonVariant::Surface => Visual {
            background: Some(shift_toward(
                accent_soft_fill(theme, color),
                theme.is_dark(),
                0.1,
            )),
            ..base
        },
        ButtonVariant::Link => Visual {
            text: current_text_for_state(
                base.text,
                theme.semantic_color(SemanticColor::Foreground),
            ),
            ..base
        },
    }
}

fn pressed_visual(
    theme: &Theme,
    variant: ButtonVariant,
    color: Option<AccentColor>,
    base: Visual,
) -> Visual {
    match variant {
        ButtonVariant::Default => Visual {
            background: Some(shift_toward(
                accent_fill(theme, color),
                theme.is_dark(),
                0.22,
            )),
            ..base
        },
        ButtonVariant::Destructive => Visual {
            background: Some(destructive_soft_fill(
                theme,
                destructive_soft_alpha(theme, SoftState::Pressed),
            )),
            text: theme.semantic_color(SemanticColor::Destructive),
            ..base
        },
        ButtonVariant::Secondary
        | ButtonVariant::Soft
        | ButtonVariant::Surface
        | ButtonVariant::Ghost
        | ButtonVariant::Outline => Visual {
            background: Some(theme.semantic_color(SemanticColor::Muted)),
            ..base
        },
        ButtonVariant::Link => base,
    }
}

fn disabled_visual(theme: &Theme) -> Visual {
    Visual {
        background: Some(theme.semantic_color(SemanticColor::Muted)),
        text: theme.semantic_color(SemanticColor::MutedForeground),
        border_color: theme.semantic_color(SemanticColor::Border),
        border_width: 1.0,
        shadow: None,
    }
}

fn accent_fill(theme: &Theme, color: Option<AccentColor>) -> Color {
    match color {
        None => theme.palette.primary,
        Some(accent) => theme.color_with_accent(accent, SemanticColor::Primary),
    }
}

fn accent_on_fill(theme: &Theme, color: Option<AccentColor>) -> Color {
    match color {
        None => theme.palette.primary_foreground,
        Some(accent) => theme.color_with_accent(accent, SemanticColor::PrimaryForeground),
    }
}

pub(super) fn accent_text(theme: &Theme, color: Option<AccentColor>) -> Color {
    match color {
        None => theme.palette.primary,
        Some(accent) => theme.color_with_accent(accent, SemanticColor::Primary),
    }
}

fn accent_soft_fill(theme: &Theme, color: Option<AccentColor>) -> Color {
    match color {
        None => theme.palette.secondary,
        Some(accent) => theme.color_with_accent(accent, SemanticColor::Secondary),
    }
}

#[derive(Clone, Copy)]
enum SoftState {
    Base,
    Hover,
    Pressed,
}

/// shadcn destructive button: `bg-destructive/10` (dark `/20`), hover `/20` (dark `/30`).
fn destructive_soft_alpha(theme: &Theme, state: SoftState) -> f32 {
    match (theme.is_dark(), state) {
        (false, SoftState::Base) => 0.10,
        (true, SoftState::Base) => 0.20,
        (false, SoftState::Hover) => 0.20,
        (true, SoftState::Hover) => 0.30,
        (false, SoftState::Pressed) => 0.25,
        (true, SoftState::Pressed) => 0.35,
    }
}

fn destructive_soft_fill(theme: &Theme, alpha: f32) -> Color {
    mix_color(
        theme.semantic_color(SemanticColor::Background),
        theme.semantic_color(SemanticColor::Destructive),
        alpha,
    )
}

fn mix_color(a: Color, b: Color, t: f32) -> Color {
    let t = t.clamp(0.0, 1.0);
    Color {
        r: a.r + (b.r - a.r) * t,
        g: a.g + (b.g - a.g) * t,
        b: a.b + (b.b - a.b) * t,
        a: a.a + (b.a - a.a) * t,
    }
}

fn shift_toward(color: Color, dark: bool, amount: f32) -> Color {
    if dark {
        mix_color(color, Color::WHITE, amount)
    } else {
        mix_color(color, Color::BLACK, amount)
    }
}

fn current_text_for_state(current: Color, fallback: Color) -> Color {
    let alpha = 0.85;
    Color {
        r: current.r * alpha + fallback.r * (1.0 - alpha),
        g: current.g * alpha + fallback.g * (1.0 - alpha),
        b: current.b * alpha + fallback.b * (1.0 - alpha),
        a: 1.0,
    }
}

/// Default button corner radius in px for the active style pack.
pub(super) fn default_radius_px(theme: &Theme) -> f32 {
    component_radius_px(theme, theme.style.button_type().default_radius)
}

fn resolve_radius_px(theme: &Theme, radius: Option<ButtonRadius>) -> f32 {
    match radius {
        Some(radius) => radius_px(theme, radius),
        None => default_radius_px(theme),
    }
}

fn radius_px(theme: &Theme, radius: ButtonRadius) -> f32 {
    match radius {
        ButtonRadius::None => 0.0,
        ButtonRadius::Small => theme.style.twill_radius_sm.px_value(),
        ButtonRadius::Medium => theme.style.twill_radius_md.px_value(),
        ButtonRadius::Large => theme.style.twill_radius_lg.px_value(),
        ButtonRadius::Full => 9999.0,
    }
}
