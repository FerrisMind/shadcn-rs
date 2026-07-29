//! Iced color palette derived from shadcn semantic tokens.

use crate::iced_compat::Color;
use shadcn_common::{OklchColor, ResolvedTheme};
use twill_core::tokens::ColorValue;

/// Cached iced palette built from a [`ResolvedTheme`].
///
/// The struct is `#[non_exhaustive]`: new semantic slots may be added in
/// minor releases, so construct it through [`crate::Theme`] instead of a
/// struct literal.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Palette {
    /// Page background color.
    pub background: Color,
    /// Default text color on [`Self::background`].
    pub foreground: Color,
    /// Card surface color.
    pub card: Color,
    /// Text color on [`Self::card`].
    pub card_foreground: Color,
    /// Popover surface color.
    pub popover: Color,
    /// Text color on [`Self::popover`].
    pub popover_foreground: Color,
    /// Default border color.
    pub border: Color,
    /// Input border/background color.
    pub input: Color,
    /// Focus ring color.
    pub ring: Color,
    /// Primary action color.
    pub primary: Color,
    /// Text color on [`Self::primary`].
    pub primary_foreground: Color,
    /// Secondary surface color.
    pub secondary: Color,
    /// Text color on [`Self::secondary`].
    pub secondary_foreground: Color,
    /// Accent surface color (hover states, highlights).
    pub accent: Color,
    /// Text color on [`Self::accent`].
    pub accent_foreground: Color,
    /// Muted surface color.
    pub muted: Color,
    /// Text color on [`Self::muted`].
    pub muted_foreground: Color,
    /// Destructive action color.
    pub destructive: Color,
    /// Text color on [`Self::destructive`].
    pub destructive_foreground: Color,
    /// First chart series color.
    pub chart_1: Color,
    /// Second chart series color.
    pub chart_2: Color,
    /// Third chart series color.
    pub chart_3: Color,
    /// Fourth chart series color.
    pub chart_4: Color,
    /// Fifth chart series color.
    pub chart_5: Color,
    /// Sidebar surface color.
    pub sidebar: Color,
    /// Text color on [`Self::sidebar`].
    pub sidebar_foreground: Color,
    /// Primary action color inside the sidebar.
    pub sidebar_primary: Color,
    /// Text color on [`Self::sidebar_primary`].
    pub sidebar_primary_foreground: Color,
    /// Accent surface color inside the sidebar.
    pub sidebar_accent: Color,
    /// Text color on [`Self::sidebar_accent`].
    pub sidebar_accent_foreground: Color,
    /// Border color inside the sidebar.
    pub sidebar_border: Color,
    /// Focus ring color inside the sidebar.
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
///
/// Relative luminance is computed on linearized sRGB channels per WCAG; the
/// threshold is the linear-light equivalent of the previous gamma-space 0.55
/// cutoff, so neutral surfaces keep their existing text color.
pub(super) fn preferred_text(background: Color) -> Color {
    fn linear(channel: f32) -> f32 {
        if channel <= 0.039_28 {
            channel / 12.92
        } else {
            ((channel + 0.055) / 1.055).powf(2.4)
        }
    }

    let luminance = 0.2126 * linear(background.r)
        + 0.7152 * linear(background.g)
        + 0.0722 * linear(background.b);
    if luminance > 0.2636 {
        Color::BLACK
    } else {
        Color::WHITE
    }
}
