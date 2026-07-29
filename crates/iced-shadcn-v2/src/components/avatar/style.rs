//! Semantic style resolution for the avatar component.

use crate::iced_compat::border::Border;
use crate::iced_compat::widget::container;
use crate::iced_compat::{Background, Color};

use super::geometry;
use super::types::{AvatarRadius, AvatarSize};
use crate::theme::Theme;

pub(super) fn resolve_root_style(theme: &Theme, radius: AvatarRadius) -> container::Style {
    container::Style {
        text_color: Some(theme.palette.foreground),
        border: Border {
            color: theme.palette.border,
            width: 1.0,
            radius: geometry::radius_px(theme, radius).into(),
        },
        snap: true,
        ..container::Style::default()
    }
}

pub(super) fn resolve_fallback_style(theme: &Theme, radius: f32) -> container::Style {
    container::Style {
        background: Some(Background::Color(theme.palette.muted)),
        text_color: Some(theme.palette.muted_foreground),
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: radius.into(),
        },
        snap: true,
        ..container::Style::default()
    }
}

pub(super) fn resolve_badge_style(theme: &Theme, size: AvatarSize) -> container::Style {
    container::Style {
        background: Some(Background::Color(theme.palette.primary)),
        text_color: Some(theme.palette.primary_foreground),
        border: Border {
            color: theme.palette.background,
            width: 2.0,
            radius: geometry::badge_size(size).into(),
        },
        snap: true,
        ..container::Style::default()
    }
}

pub(super) fn resolve_badge_ring_style(color: Color, radius: f32, width: f32) -> container::Style {
    container::Style {
        border: Border {
            color,
            width: geometry::normalize_px(width),
            radius: geometry::normalize_px(radius).into(),
        },
        snap: true,
        ..container::Style::default()
    }
}

pub(super) fn resolve_group_ring_style(theme: &Theme, radius: f32) -> container::Style {
    container::Style {
        border: Border {
            color: theme.palette.background,
            width: 2.0,
            radius: geometry::normalize_px(radius).min(9999.0).into(),
        },
        snap: true,
        ..container::Style::default()
    }
}

pub(super) fn resolve_group_count_style(theme: &Theme, size: f32) -> container::Style {
    container::Style {
        background: Some(Background::Color(theme.palette.muted)),
        text_color: Some(theme.palette.muted_foreground),
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: size.min(9999.0).into(),
        },
        snap: true,
        ..container::Style::default()
    }
}

pub(super) fn resolve_group_style(_theme: &Theme) -> container::Style {
    container::Style {
        snap: true,
        ..container::Style::default()
    }
}
