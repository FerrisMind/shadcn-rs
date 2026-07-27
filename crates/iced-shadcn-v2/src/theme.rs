//! Theme for `iced-shadcn-v2`, sourced from `shadcn-common`.
//!
//! OKLCH tokens are converted to [`iced::Color`] directly via
//! `twill_core::tokens::ColorValue::to_rgba8` — no `twill-iced` adapter.

use iced::Color;
use shadcn_common::{
    AccentColor, BaseColor, FontHeading, FontId, FontPack, IconSet, OklchColor, RadiusId,
    RadiusScale, ResolvedTheme, StyleId, StylePack, ThemeMode,
};
use twill_core::prelude::theme::SemanticColor;
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

/// v2 theme: `shadcn-common` OKLCH tokens → iced colors.
///
/// ```rust
/// use iced_shadcn_v2::{AccentColor, SemanticColor, Theme};
///
/// let theme = Theme::light().with_accent(Some(AccentColor::Blue));
/// let primary = theme.semantic_color(SemanticColor::Primary);
/// assert!(primary.a > 0.0);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Theme {
    resolved: ResolvedTheme,
    pub palette: Palette,
    pub style: StylePack,
}

impl Theme {
    /// Builds a theme (and its cached [`Palette`]) from a [`ResolvedTheme`].
    pub fn from_resolved(resolved: ResolvedTheme) -> Self {
        let table = resolved.semantic_vars();
        let destructive = oklch_to_iced(table.destructive);
        let destructive_foreground = preferred_text(destructive);

        Self {
            style: resolved.style_pack(),
            palette: Palette {
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
            },
            resolved,
        }
    }

    /// Default light theme (Vega style, neutral base).
    pub fn light() -> Self {
        Self::from_resolved(ResolvedTheme::default())
    }

    /// Default dark theme (Vega style, neutral base).
    pub fn dark() -> Self {
        Self::from_resolved(ResolvedTheme::default().with_mode(ThemeMode::Dark))
    }

    /// The backend-agnostic theme this palette was resolved from.
    pub fn resolved(&self) -> ResolvedTheme {
        self.resolved
    }

    /// Current light/dark mode.
    pub fn mode(&self) -> ThemeMode {
        self.resolved.mode
    }

    /// Whether the theme is in dark mode.
    pub fn is_dark(&self) -> bool {
        self.resolved.mode.is_dark()
    }

    /// Selected shadcn style system.
    pub fn style_id(&self) -> StyleId {
        self.resolved.style
    }

    /// Selected base color.
    pub fn base(&self) -> BaseColor {
        self.resolved.base
    }

    /// Selected accent overlay, if any.
    pub fn accent(&self) -> Option<AccentColor> {
        self.resolved.accent
    }

    /// Resolved font pack (sans/mono/heading).
    pub fn font_pack(&self) -> FontPack {
        self.resolved.font_pack()
    }

    /// Resolved body font.
    pub fn font_id(&self) -> FontId {
        self.resolved.font_id()
    }

    /// Resolved heading font.
    pub fn font_heading(&self) -> FontHeading {
        self.resolved.font_heading()
    }

    /// Icon set associated with the theme.
    pub fn icon_set(&self) -> IconSet {
        self.resolved.icon_set()
    }

    /// Radius scale of the active style pack.
    pub fn radius_scale(&self) -> RadiusScale {
        self.style.radius
    }

    /// Resolved radius picker value.
    pub fn radius_id(&self) -> RadiusId {
        self.resolved.radius_id()
    }

    /// Returns the theme with a different style system.
    pub fn with_style(self, style: StyleId) -> Self {
        Self::from_resolved(self.resolved.with_style(style))
    }

    /// Returns the theme with a different base color.
    pub fn with_base(self, base: BaseColor) -> Self {
        Self::from_resolved(self.resolved.with_base(base))
    }

    /// Returns the theme with a different accent overlay.
    pub fn with_accent(self, accent: Option<AccentColor>) -> Self {
        Self::from_resolved(self.resolved.with_accent(accent))
    }

    /// Returns the theme with a different light/dark mode.
    pub fn with_mode(self, mode: ThemeMode) -> Self {
        Self::from_resolved(self.resolved.with_mode(mode))
    }

    /// Returns the theme with a different body font.
    pub fn with_font(self, font: FontId) -> Self {
        Self::from_resolved(self.resolved.with_font(font))
    }

    /// Returns the theme with a different heading font.
    pub fn with_font_heading(self, font_heading: FontHeading) -> Self {
        Self::from_resolved(self.resolved.with_font_heading(font_heading))
    }

    /// Returns the theme with a different radius picker value.
    pub fn with_radius(self, radius: RadiusId) -> Self {
        Self::from_resolved(self.resolved.with_radius(radius))
    }

    /// iced color for a semantic slot from the cached [`Palette`].
    pub fn semantic_color(&self, token: SemanticColor) -> Color {
        match token {
            SemanticColor::Background => self.palette.background,
            SemanticColor::Foreground => self.palette.foreground,
            SemanticColor::Card => self.palette.card,
            SemanticColor::CardForeground => self.palette.card_foreground,
            SemanticColor::Popover => self.palette.popover,
            SemanticColor::PopoverForeground => self.palette.popover_foreground,
            SemanticColor::Primary => self.palette.primary,
            SemanticColor::PrimaryForeground => self.palette.primary_foreground,
            SemanticColor::Secondary => self.palette.secondary,
            SemanticColor::SecondaryForeground => self.palette.secondary_foreground,
            SemanticColor::Muted => self.palette.muted,
            SemanticColor::MutedForeground => self.palette.muted_foreground,
            SemanticColor::Accent => self.palette.accent,
            SemanticColor::AccentForeground => self.palette.accent_foreground,
            SemanticColor::Destructive => self.palette.destructive,
            SemanticColor::Border => self.palette.border,
            SemanticColor::Input => self.palette.input,
            SemanticColor::Ring => self.palette.ring,
            SemanticColor::Chart1 => self.palette.chart_1,
            SemanticColor::Chart2 => self.palette.chart_2,
            SemanticColor::Chart3 => self.palette.chart_3,
            SemanticColor::Chart4 => self.palette.chart_4,
            SemanticColor::Chart5 => self.palette.chart_5,
            SemanticColor::Sidebar => self.palette.sidebar,
            SemanticColor::SidebarForeground => self.palette.sidebar_foreground,
            SemanticColor::SidebarPrimary => self.palette.sidebar_primary,
            SemanticColor::SidebarPrimaryForeground => self.palette.sidebar_primary_foreground,
            SemanticColor::SidebarAccent => self.palette.sidebar_accent,
            SemanticColor::SidebarAccentForeground => self.palette.sidebar_accent_foreground,
            SemanticColor::SidebarBorder => self.palette.sidebar_border,
            SemanticColor::SidebarRing => self.palette.sidebar_ring,
        }
    }

    /// Foreground color paired with a semantic surface slot.
    pub fn semantic_foreground(&self, token: SemanticColor) -> Color {
        match token {
            SemanticColor::Destructive => self.palette.destructive_foreground,
            SemanticColor::Primary => self.palette.primary_foreground,
            SemanticColor::Secondary => self.palette.secondary_foreground,
            SemanticColor::Accent => self.palette.accent_foreground,
            SemanticColor::Card => self.palette.card_foreground,
            SemanticColor::Popover => self.palette.popover_foreground,
            SemanticColor::Muted => self.palette.muted_foreground,
            SemanticColor::Sidebar => self.palette.sidebar_foreground,
            SemanticColor::SidebarPrimary => self.palette.sidebar_primary_foreground,
            SemanticColor::SidebarAccent => self.palette.sidebar_accent_foreground,
            other => preferred_text(self.semantic_color(other)),
        }
    }

    /// Resolve a semantic slot as if `accent` were applied on top of this theme.
    pub fn color_with_accent(&self, accent: AccentColor, token: SemanticColor) -> Color {
        let resolved = self.resolved.with_accent(Some(accent));
        resolved
            .semantic_color_value(token)
            .map(color_value_to_iced)
            .unwrap_or(Color::BLACK)
    }
}

/// twill-core [`ColorValue`] (OKLCH) → iced sRGB color.
pub(crate) fn color_value_to_iced(value: ColorValue) -> Color {
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

fn preferred_text(background: Color) -> Color {
    let luminance = 0.2126 * background.r + 0.7152 * background.g + 0.0722 * background.b;
    if luminance > 0.55 {
        Color::BLACK
    } else {
        Color::WHITE
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn light_neutral_has_bright_background() {
        let theme = Theme::light();
        assert!(theme.palette.background.r > 0.9);
        assert!(!theme.is_dark());
    }

    #[test]
    fn accent_overlay_changes_primary() {
        let base = Theme::light();
        let amber = base.clone().with_accent(Some(AccentColor::Amber));
        assert_ne!(base.palette.primary, amber.palette.primary);
    }

    #[test]
    fn color_value_conversion_preserves_alpha() {
        let value = ColorValue::from_oklch(0.6, 0.1, 200.0).with_alpha(0.5);
        let color = color_value_to_iced(value);
        assert!((color.a - 0.5).abs() < f32::EPSILON);
    }
}
