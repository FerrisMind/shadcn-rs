//! Mapping of semantic badge variants to iced styles.

use crate::iced_compat::border::Border;
use crate::iced_compat::widget::button as button_widget;
use crate::iced_compat::widget::container;
use crate::iced_compat::{Color, Shadow};

use shadcn_common::AccentColor;
use twill_core::prelude::theme::SemanticColor;

use super::{BadgeRadius, BadgeVariant};
use crate::recipes::component_radius_px;
use crate::theme::Theme;

/// Resolved visual state of a badge.
#[derive(Debug, Clone, Copy)]
struct Visual {
    background: Option<Color>,
    text: Color,
    border_color: Color,
    border_width: f32,
}

/// Resolves a static (non-interactive) container style.
pub(super) fn resolve_container_style(
    theme: &Theme,
    variant: BadgeVariant,
    radius: Option<BadgeRadius>,
    color: Option<AccentColor>,
) -> container::Style {
    let visual = base_visual(theme, variant, color);
    to_container_style(theme, visual, radius)
}

/// Resolves an interactive button style for a given status.
pub(super) fn resolve_button_style(
    theme: &Theme,
    variant: BadgeVariant,
    radius: Option<BadgeRadius>,
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
        shadow: Shadow::default(),
        snap: true,
    }
}

fn to_container_style(
    theme: &Theme,
    visual: Visual,
    radius: Option<BadgeRadius>,
) -> container::Style {
    container::Style {
        background: visual
            .background
            .filter(|color| color.a > f32::EPSILON)
            .map(crate::iced_compat::Background::Color),
        text_color: Some(visual.text),
        border: Border {
            radius: resolve_radius_px(theme, radius).into(),
            width: visual.border_width,
            color: visual.border_color,
        },
        shadow: Shadow::default(),
        snap: true,
    }
}

fn base_visual(theme: &Theme, variant: BadgeVariant, color: Option<AccentColor>) -> Visual {
    let accent = accent_fill(theme, color);
    let accent_fg = accent_on_fill(theme, color);

    match variant {
        BadgeVariant::Default => Visual {
            background: Some(accent),
            text: accent_fg,
            border_color: Color::TRANSPARENT,
            border_width: 0.0,
        },
        BadgeVariant::Secondary => Visual {
            background: Some(theme.semantic_color(SemanticColor::Secondary)),
            text: theme.semantic_color(SemanticColor::SecondaryForeground),
            border_color: Color::TRANSPARENT,
            border_width: 0.0,
        },
        BadgeVariant::Destructive => {
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
            }
        }
        BadgeVariant::Outline => Visual {
            background: None,
            text: theme.semantic_color(SemanticColor::Foreground),
            border_color: theme.semantic_color(SemanticColor::Border),
            border_width: 1.0,
        },
        BadgeVariant::Ghost => Visual {
            background: None,
            text: theme.semantic_color(SemanticColor::Foreground),
            border_color: Color::TRANSPARENT,
            border_width: 0.0,
        },
        BadgeVariant::Link => Visual {
            background: None,
            text: accent,
            border_color: Color::TRANSPARENT,
            border_width: 0.0,
        },
    }
}

fn hovered_visual(
    theme: &Theme,
    variant: BadgeVariant,
    color: Option<AccentColor>,
    base: Visual,
) -> Visual {
    match variant {
        // shadcn: `[a]:hover:bg-primary/80`
        BadgeVariant::Default => Visual {
            background: Some(with_alpha(accent_fill(theme, color), 0.80)),
            ..base
        },
        // shadcn: `[a]:hover:bg-secondary/80`
        BadgeVariant::Secondary => Visual {
            background: Some(with_alpha(
                theme.semantic_color(SemanticColor::Secondary),
                0.80,
            )),
            ..base
        },
        // shadcn: `[a]:hover:bg-destructive/20` (dark `/30` via SoftState::Hover)
        BadgeVariant::Destructive => Visual {
            background: Some(destructive_soft_fill(
                theme,
                destructive_soft_alpha(theme, SoftState::Hover),
            )),
            text: theme.semantic_color(SemanticColor::Destructive),
            ..base
        },
        // shadcn: `[a]:hover:bg-muted [a]:hover:text-muted-foreground`
        BadgeVariant::Outline => Visual {
            background: Some(theme.semantic_color(SemanticColor::Muted)),
            text: theme.semantic_color(SemanticColor::MutedForeground),
            ..base
        },
        // shadcn: `hover:bg-muted hover:text-muted-foreground`
        BadgeVariant::Ghost => Visual {
            background: Some(if theme.is_dark() {
                with_alpha(theme.semantic_color(SemanticColor::Muted), 0.50)
            } else {
                theme.semantic_color(SemanticColor::Muted)
            }),
            text: theme.semantic_color(SemanticColor::MutedForeground),
            ..base
        },
        BadgeVariant::Link => base,
    }
}

fn pressed_visual(
    theme: &Theme,
    variant: BadgeVariant,
    color: Option<AccentColor>,
    base: Visual,
) -> Visual {
    match variant {
        BadgeVariant::Default => Visual {
            background: Some(with_alpha(accent_fill(theme, color), 0.70)),
            ..base
        },
        BadgeVariant::Secondary => Visual {
            background: Some(with_alpha(
                theme.semantic_color(SemanticColor::Secondary),
                0.70,
            )),
            ..base
        },
        BadgeVariant::Destructive => Visual {
            background: Some(destructive_soft_fill(
                theme,
                destructive_soft_alpha(theme, SoftState::Pressed),
            )),
            text: theme.semantic_color(SemanticColor::Destructive),
            ..base
        },
        BadgeVariant::Outline | BadgeVariant::Ghost => Visual {
            background: Some(theme.semantic_color(SemanticColor::Muted)),
            text: theme.semantic_color(SemanticColor::MutedForeground),
            ..base
        },
        BadgeVariant::Link => base,
    }
}

fn disabled_visual(theme: &Theme) -> Visual {
    Visual {
        background: Some(theme.semantic_color(SemanticColor::Muted)),
        text: theme.semantic_color(SemanticColor::MutedForeground),
        border_color: theme.semantic_color(SemanticColor::Border),
        border_width: 1.0,
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

/// Text color for the active (non-hovered) badge surface.
pub(super) fn label_color(
    theme: &Theme,
    variant: BadgeVariant,
    color: Option<AccentColor>,
) -> Color {
    base_visual(theme, variant, color).text
}

#[derive(Clone, Copy)]
enum SoftState {
    Base,
    Hover,
    Pressed,
}

/// shadcn destructive badge: `bg-destructive/10` (dark `/20`), hover `/20` (dark `/30`).
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

fn with_alpha(mut color: Color, alpha: f32) -> Color {
    color.a = alpha.clamp(0.0, 1.0);
    color
}

/// Unset radius → style-pack badge default from shadcn-common.
#[cfg(test)]
pub(super) fn effective_radius(theme: &Theme, radius: Option<BadgeRadius>) -> BadgeRadius {
    match radius {
        Some(radius) => radius,
        None => match theme.style.badge().default_radius {
            shadcn_common::ComponentRadius::None => BadgeRadius::None,
            shadcn_common::ComponentRadius::Sm => BadgeRadius::Small,
            shadcn_common::ComponentRadius::Md => BadgeRadius::Medium,
            shadcn_common::ComponentRadius::Lg
            | shadcn_common::ComponentRadius::Xl
            | shadcn_common::ComponentRadius::S2xl
            | shadcn_common::ComponentRadius::S3xl
            | shadcn_common::ComponentRadius::S4xl => BadgeRadius::Large,
            shadcn_common::ComponentRadius::Full => BadgeRadius::Full,
            _ => BadgeRadius::Full,
        },
    }
}

fn resolve_radius_px(theme: &Theme, radius: Option<BadgeRadius>) -> f32 {
    match radius {
        Some(radius) => radius_px(theme, radius),
        None => component_radius_px(theme, theme.style.badge().default_radius),
    }
}

fn radius_px(theme: &Theme, radius: BadgeRadius) -> f32 {
    match radius {
        BadgeRadius::None => 0.0,
        BadgeRadius::Small => theme.style.twill_radius_sm.px_value(),
        BadgeRadius::Medium => theme.style.twill_radius_md.px_value(),
        BadgeRadius::Large => theme.style.twill_radius_lg.px_value(),
        BadgeRadius::Full => 9999.0,
    }
}
