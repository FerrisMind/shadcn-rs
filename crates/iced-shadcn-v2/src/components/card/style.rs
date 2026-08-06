//! Semantic card style resolution.

use crate::iced_compat::border::{Border, Radius};
use crate::iced_compat::widget::container;
use crate::iced_compat::{Background, Color, Shadow, Vector};
use shadcn_common::StyleId;

use super::geometry;
use super::types::{CardBorder, CardRadius};
use crate::theme::Theme;

/// Resolves the outer card container style.
///
/// Fill + radius + shadow only. The CSS `ring-1` hairline is painted **outside**
/// the bounds by [`super::render::with_outside_ring`] — an iced inset
/// [`Border`] sits under edge-to-edge children (e.g. Command in `p-0`) and
/// vanishes on large radii (Maia / Luma).
pub(super) fn resolve_root_style(theme: &Theme, radius: CardRadius) -> container::Style {
    container::Style {
        background: Some(Background::Color(theme.palette.card)),
        text_color: Some(theme.palette.card_foreground),
        border: Border {
            radius: geometry::radius_px(theme, radius).into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        shadow: card_shadow(theme),
        snap: false,
    }
}

/// `ring-1 ring-foreground/N` tokens for the outside hairline.
pub(super) fn root_ring(theme: &Theme) -> (Color, f32) {
    (ring_color(theme), 1.0)
}

/// Resolves a section wrapper. Section borders are painted as explicit rules
/// in [`super::render`] because iced's `Border` is uniform on all sides.
pub(super) fn resolve_header_style(theme: &Theme, radius: CardRadius) -> container::Style {
    container::Style {
        text_color: Some(theme.palette.card_foreground),
        border: Border {
            color: theme.palette.border,
            radius: Radius::default().top(geometry::radius_px(theme, radius)),
            ..Border::default()
        },
        snap: true,
        ..container::Style::default()
    }
}

/// Resolves the unadorned content section style.
pub(super) fn resolve_content_style(theme: &Theme) -> container::Style {
    container::Style {
        text_color: Some(theme.palette.card_foreground),
        snap: true,
        ..container::Style::default()
    }
}

/// Resolves a footer wrapper, including the style-specific muted footer.
pub(super) fn resolve_footer_style(
    theme: &Theme,
    radius: CardRadius,
    background_override: Option<Color>,
) -> container::Style {
    let background = background_override.or_else(|| default_footer_background(theme));

    container::Style {
        background: background.map(Background::Color),
        text_color: Some(theme.palette.card_foreground),
        border: Border {
            color: theme.palette.border,
            radius: Radius::default().bottom(geometry::radius_px(theme, radius)),
            ..Border::default()
        },
        snap: true,
        ..container::Style::default()
    }
}

pub(super) fn header_has_border(_theme: &Theme, border: CardBorder) -> bool {
    match border {
        CardBorder::Theme | CardBorder::None => false,
        CardBorder::Present => true,
    }
}

pub(super) fn footer_has_border(theme: &Theme, border: CardBorder) -> bool {
    match border {
        CardBorder::Theme => matches!(theme.style_id(), StyleId::Nova | StyleId::Lyra),
        CardBorder::None => false,
        CardBorder::Present => true,
    }
}

pub(super) fn default_footer_background(theme: &Theme) -> Option<Color> {
    match theme.style_id() {
        // Nova's source footer is `bg-muted/50`; Lyra only adds a border.
        StyleId::Nova => Some(with_alpha(theme.palette.muted, 0.5)),
        StyleId::Vega
        | StyleId::Maia
        | StyleId::Lyra
        | StyleId::Mira
        | StyleId::Luma
        | StyleId::Sera
        | StyleId::Rhea => None,
    }
}

fn ring_color(theme: &Theme) -> Color {
    let alpha = match theme.style_id() {
        StyleId::Luma | StyleId::Rhea => {
            if theme.is_dark() {
                0.10
            } else {
                0.05
            }
        }
        StyleId::Sera => 0.05,
        StyleId::Vega | StyleId::Nova | StyleId::Maia | StyleId::Lyra | StyleId::Mira => 0.10,
    };

    with_alpha(theme.palette.foreground, alpha)
}

fn card_shadow(theme: &Theme) -> Shadow {
    let (alpha, offset_y, blur) = match theme.style_id() {
        StyleId::Vega => (0.05, 1.0, 2.0),
        StyleId::Luma => (0.10, 2.0, 6.0),
        StyleId::Sera | StyleId::Rhea => (0.10, 1.0, 3.0),
        StyleId::Nova | StyleId::Maia | StyleId::Lyra | StyleId::Mira => return Shadow::default(),
    };

    Shadow {
        color: Color::from_rgba(0.0, 0.0, 0.0, alpha),
        offset: Vector::new(0.0, offset_y),
        blur_radius: blur,
    }
}

fn with_alpha(mut color: Color, alpha: f32) -> Color {
    color.a *= alpha.clamp(0.0, 1.0);
    color
}
