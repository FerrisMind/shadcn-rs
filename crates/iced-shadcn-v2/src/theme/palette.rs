//! Iced color palette derived from shadcn semantic tokens.

use iced::Color;
use shadcn_common::{OklchColor, ResolvedTheme};
use twill_core::tokens::ColorValue;

/// Cached iced palette built from a [`ResolvedTheme`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Palette {
    pub background: Color,
    pub foreground: Color,
    pub card: Color,
    pub card_foreground: Color,
    pub popover: Color,
    pub popover_foreground: Color,
    pub border: Color,
    pub input: Color,
    pub ring: Color,
    pub primary: Color,
    pub primary_foreground: Color,
    pub secondary: Color,
    pub secondary_foreground: Color,
    pub accent: Color,
    pub accent_foreground: Color,
    pub muted: Color,
    pub muted_foreground: Color,
    pub destructive: Color,
    pub destructive_foreground: Color,
    pub chart_1: Color,
    pub chart_2: Color,
    pub chart_3: Color,
    pub chart_4: Color,
    pub chart_5: Color,
    pub sidebar: Color,
    pub sidebar_foreground: Color,
    pub sidebar_primary: Color,
    pub sidebar_primary_foreground: Color,
    pub sidebar_accent: Color,
    pub sidebar_accent_foreground: Color,
    pub sidebar_border: Color,
    pub sidebar_ring: Color,
}

impl Palette {
    pub(super) fn from_resolved(resolved: &ResolvedTheme) -> Self {
        let table = resolved.semantic_vars();
        let destructive = oklch_to_iced(table.destructive);
        let destructive_foreground = preferred_text(destructive);

        Self {
            background: oklch_to_iced(table.background),
            foreground: oklch_to_iced(table.foreground),
            card: oklch_to_iced(table.card),
            card_foreground: oklch_to_iced(table.card_foreground),
            popover: oklch_to_iced(table.popover),
            popover_foreground: oklch_to_iced(table.popover_foreground),
            border: oklch_to_iced(table.border),
            input: oklch_to_iced(table.input),
            ring: oklch_to_iced(table.ring),
            primary: oklch_to_iced(table.primary),
            primary_foreground: oklch_to_iced(table.primary_foreground),
            secondary: oklch_to_iced(table.secondary),
            secondary_foreground: oklch_to_iced(table.secondary_foreground),
            accent: oklch_to_iced(table.accent),
            accent_foreground: oklch_to_iced(table.accent_foreground),
            muted: oklch_to_iced(table.muted),
            muted_foreground: oklch_to_iced(table.muted_foreground),
            destructive,
            destructive_foreground,
            chart_1: oklch_to_iced(table.chart_1),
            chart_2: oklch_to_iced(table.chart_2),
            chart_3: oklch_to_iced(table.chart_3),
            chart_4: oklch_to_iced(table.chart_4),
            chart_5: oklch_to_iced(table.chart_5),
            sidebar: oklch_to_iced(table.sidebar),
            sidebar_foreground: oklch_to_iced(table.sidebar_foreground),
            sidebar_primary: oklch_to_iced(table.sidebar_primary),
            sidebar_primary_foreground: oklch_to_iced(table.sidebar_primary_foreground),
            sidebar_accent: oklch_to_iced(table.sidebar_accent),
            sidebar_accent_foreground: oklch_to_iced(table.sidebar_accent_foreground),
            sidebar_border: oklch_to_iced(table.sidebar_border),
            sidebar_ring: oklch_to_iced(table.sidebar_ring),
        }
    }
}

/// Converts a twill-core color value to an iced sRGB color.
pub(super) fn color_value_to_iced(value: ColorValue) -> Color {
    let (r, g, b) = value.to_rgb8();
    Color::from_rgba(
        f32::from(r) / 255.0,
        f32::from(g) / 255.0,
        f32::from(b) / 255.0,
        value.alpha(),
    )
}

fn oklch_to_iced(color: OklchColor) -> Color {
    color_value_to_iced(color.to_color_value())
}

/// Chooses black or white text with sufficient contrast for a surface.
pub(super) fn preferred_text(background: Color) -> Color {
    let luminance = 0.2126 * background.r + 0.7152 * background.g + 0.0722 * background.b;
    if luminance > 0.55 {
        Color::BLACK
    } else {
        Color::WHITE
    }
}
