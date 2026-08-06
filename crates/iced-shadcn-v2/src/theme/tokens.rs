//! Theme token resolution for `iced-shadcn-v2`.
//!
//! OKLCH tokens are converted to [`iced::Color`](iced_core::Color) directly via
//! `twill_core::tokens::ColorValue::to_rgba8` — no `twill-iced` adapter.

use crate::iced_compat::Color;
use shadcn_common::{
    AccentColor, BaseColor, FontHeading, FontId, IconSet, RadiusId, RadiusScale, ResolvedTheme,
    StyleId, StylePack, ThemeMode,
};
use twill_core::prelude::theme::SemanticColor;

use super::palette::{Palette, color_value_to_iced, preferred_text};

/// v2 theme: `shadcn-common` OKLCH tokens → iced colors.
///
/// Own this in application state and pass `&Theme` into components. There is
/// no ambient iced theme: widgets do not pick up Vega/Nova until you hand them
/// a [`Theme`].
///
/// **Defaults vs overrides:** [`StyleId`] packs supply default fonts/radius;
/// [`Self::with_font`], [`Self::with_font_heading`], [`Self::with_radius`], and
/// the other `with_*` builders take priority over that pack (same idea as the
/// shadcn-svelte create customizer).
///
/// **Several looks on one screen:** keep multiple [`Theme`] values (e.g. Vega
/// and Nova) and pass a different `&Theme` into each widget. For different
/// button treatments under one theme, use per-control APIs such as
/// [`crate::Button::variant`] / [`crate::Button::color`] instead.
///
/// ```rust
/// use iced_shadcn_v2::{AccentColor, SemanticColor, StyleId, Theme};
///
/// let theme = Theme::light()
///     .with_style(StyleId::Vega)
///     .with_accent(Some(AccentColor::Blue));
/// let primary = theme.semantic_color(SemanticColor::Primary);
/// assert!(primary.a > 0.0);
/// ```
#[derive(Debug, Clone, PartialEq)]
#[must_use = "a Theme does nothing unless passed into components"]
pub struct Theme {
    pub(super) resolved: ResolvedTheme,
    /// Cached iced palette built from the resolved theme tokens.
    pub palette: Palette,
    /// Active style pack (fonts, radius, per-component recipes).
    pub style: StylePack,
}

impl Default for Theme {
    /// Equivalent to [`Theme::light`] (Vega style, neutral base).
    fn default() -> Self {
        Self::light()
    }
}

impl Theme {
    /// Builds a theme (and its cached [`Palette`]) from a [`ResolvedTheme`].
    pub fn from_resolved(resolved: ResolvedTheme) -> Self {
        Self {
            style: resolved.style_pack(),
            palette: Palette::from_resolved(&resolved),
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
            SemanticColor::Background => self.palette.foreground,
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
