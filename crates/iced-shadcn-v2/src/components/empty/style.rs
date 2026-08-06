//! Semantic style resolution for the empty-state component.

use crate::iced_compat::widget::container;
use crate::iced_compat::{Background, Border, Color};
use crate::theme::Theme;

use super::geometry;
use super::types::{EmptyBorderStyle, EmptyMediaVariant, EmptyRadius};

/// Default visible border width for [`EmptyBorderStyle::Solid`] and
/// [`EmptyBorderStyle::Dashed`].
pub(super) const DEFAULT_BORDER_WIDTH_PX: f32 = 1.0;

/// Resolves the root container style before a caller's override is applied.
pub(super) fn resolve_root_style(
    theme: &Theme,
    radius: EmptyRadius,
    border_style: EmptyBorderStyle,
    border_width: f32,
    border_color: Color,
    background: Option<Background>,
) -> container::Style {
    let radius = geometry::radius_px(theme, radius);
    let visible_border = match border_style {
        EmptyBorderStyle::None => 0.0,
        EmptyBorderStyle::Solid | EmptyBorderStyle::Dashed => border_width,
    };

    container::Style {
        text_color: Some(theme.palette.foreground),
        background,
        border: Border {
            color: border_color,
            width: visible_border,
            radius: crate::iced_compat::border::radius(radius),
        },
        ..container::Style::default()
    }
}

/// Resolves the media wrapper style.
pub(super) fn resolve_media_style(
    theme: &Theme,
    variant: EmptyMediaVariant,
    radius: Option<f32>,
) -> container::Style {
    let mut style = container::Style {
        text_color: Some(theme.palette.foreground),
        ..container::Style::default()
    };

    if variant == EmptyMediaVariant::Icon {
        style.background = Some(Background::Color(theme.palette.muted));
        style.border.radius = crate::iced_compat::border::radius(
            radius.unwrap_or_else(|| geometry::metrics(theme).media_radius_px),
        );
    }

    style
}

/// Resolves a typed text wrapper style.
pub(super) fn resolve_text_style(color: Color) -> container::Style {
    container::Style {
        text_color: Some(color),
        ..container::Style::default()
    }
}
