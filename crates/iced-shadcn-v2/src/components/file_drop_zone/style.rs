//! Resolved colors for the file-drop-zone surface.

use crate::iced_compat::Color;
use crate::theme::Theme;

use shadcn_common::FileDropZoneConfig;
use shadcn_common::file_drop_zone_can_upload;

use super::types::{FileDropZoneState, FileDropZoneVariant};

/// Painted surface for one interaction frame.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct FileDropZoneStyle {
    pub(super) background: Color,
    pub(super) border: Color,
    pub(super) foreground: Color,
    pub(super) hint: Color,
    pub(super) icon_ring: Color,
    pub(super) opacity: f32,
}

/// Resolves colors from the theme, variant, hover, and uploadability.
pub(super) fn resolve(
    theme: &Theme,
    variant: FileDropZoneVariant,
    state: &FileDropZoneState,
    config: &FileDropZoneConfig,
) -> FileDropZoneStyle {
    let recipe = theme.style.file_drop_zone();
    let palette = &theme.palette;
    let can_upload = file_drop_zone_can_upload(config);
    let hovered = state.hovered && can_upload;

    let mut background = match variant {
        FileDropZoneVariant::Default => Color::TRANSPARENT,
        FileDropZoneVariant::Surface => palette.card,
        FileDropZoneVariant::Soft => palette.muted,
    };

    let mut border = palette.border;
    if hovered {
        border = palette.ring;
        let accent = palette.accent.scale_alpha(recipe.hover_accent_alpha);
        background = overlay(background, accent);
    }

    let opacity = if can_upload {
        1.0
    } else {
        recipe.disabled_opacity
    };

    FileDropZoneStyle {
        background,
        border,
        foreground: palette.muted_foreground,
        hint: palette
            .muted_foreground
            .scale_alpha(recipe.hint_foreground_alpha),
        icon_ring: palette.border,
        opacity,
    }
}

fn overlay(base: Color, top: Color) -> Color {
    let a = top.a.clamp(0.0, 1.0);
    let inv = 1.0 - a;
    Color {
        r: base.r * inv + top.r * a,
        g: base.g * inv + top.g * a,
        b: base.b * inv + top.b * a,
        a: (base.a + top.a).min(1.0),
    }
}
