//! Semantic colors and surfaces for the tabs list and its triggers.

use crate::iced_compat::widget::{button, container};
use crate::iced_compat::{Background, Border, Color, Shadow, Vector};
use crate::theme::Theme;

use super::geometry::TabsMetrics;
use super::{TabsHover, TabsListVariant, TabsOrientation};

/// Resolves the list surface, preserving the transparent `line` treatment.
pub(super) fn resolve_list_style(
    theme: &Theme,
    metrics: TabsMetrics,
    variant: TabsListVariant,
    orientation: TabsOrientation,
) -> container::Style {
    let list_radius = if variant == TabsListVariant::Line {
        0.0
    } else if orientation.is_vertical() {
        metrics.list_radius.min(16.0)
    } else {
        metrics.list_radius
    };

    container::Style {
        background: (variant == TabsListVariant::Default)
            .then_some(Background::Color(theme.palette.muted)),
        border: Border {
            radius: list_radius.into(),
            ..Border::default()
        },
        ..container::Style::default()
    }
}

/// Resolves one trigger's active, hover, disabled, and focus-independent
/// colors. Focus is painted by the list widget so it can follow arrow-key
/// navigation even though the application rebuilds its view on selection.
pub(super) fn resolve_trigger_style(
    theme: &Theme,
    metrics: TabsMetrics,
    variant: TabsListVariant,
    hover: TabsHover,
    active: bool,
    disabled: bool,
    status: button::Status,
) -> button::Style {
    let hovered = matches!(status, button::Status::Hovered | button::Status::Pressed);
    let mut text_color = if active || (hovered && !matches!(hover, TabsHover::None)) {
        theme.palette.foreground
    } else if theme.is_dark() {
        theme.palette.muted_foreground
    } else {
        with_alpha(theme.palette.foreground, 0.60)
    };

    if disabled {
        text_color = with_alpha(text_color, 0.50);
    }

    let background = if active && variant == TabsListVariant::Default {
        Some(Background::Color(if theme.is_dark() {
            with_alpha(theme.palette.input, 0.30)
        } else {
            theme.palette.background
        }))
    } else {
        None
    };

    let border = if active && variant == TabsListVariant::Default && theme.is_dark() {
        Border {
            color: theme.palette.input,
            width: 1.0,
            radius: metrics.trigger_radius.into(),
        }
    } else {
        Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: metrics.trigger_radius.into(),
        }
    };

    let shadow = if active && metrics.active_shadow {
        Shadow {
            color: with_alpha(Color::BLACK, 0.10),
            offset: Vector::new(0.0, 1.0),
            blur_radius: 3.0,
        }
    } else {
        Shadow::default()
    };

    button::Style {
        background,
        text_color,
        border,
        shadow,
        snap: true,
    }
}

/// Resolves the panel's inherited foreground color.
pub(super) fn resolve_content_style(theme: &Theme) -> container::Style {
    container::Style {
        text_color: Some(theme.palette.foreground),
        ..container::Style::default()
    }
}

fn with_alpha(color: Color, alpha: f32) -> Color {
    Color {
        a: (color.a * alpha).clamp(0.0, 1.0),
        ..color
    }
}
