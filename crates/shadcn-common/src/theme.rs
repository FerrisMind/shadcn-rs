//! Resolved shadcn theme: style + base + accent + mode + optional token overrides.

use twill_core::prelude::theme::SemanticColor;
use twill_core::tokens::ColorValue;

use crate::color::{AccentColor, BaseColor, OklchColor, ThemeMode, accent_token, base_token};
use crate::icons::IconSet;
use crate::radius::RadiusId;
use crate::style::{StyleId, StylePack};
use crate::typography::{FontHeading, FontId, FontPack};

/// Fully resolved design state for backends to adapt.
///
/// Optional overrides (`font` / `font_heading` / `radius`) match the shadcn-svelte
/// create customizer knobs and win over the selected [`StyleId`] pack.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ResolvedTheme {
    pub style: StyleId,
    pub base: BaseColor,
    pub accent: Option<AccentColor>,
    pub mode: ThemeMode,
    /// Body font (`--font-sans` / customizer `font`).
    pub font: Option<FontId>,
    /// Heading font (`--font-heading` / customizer `fontHeading`).
    pub font_heading: Option<FontHeading>,
    pub radius: Option<RadiusId>,
}

impl Default for ResolvedTheme {
    fn default() -> Self {
        Self::new(StyleId::Vega, BaseColor::Neutral, None, ThemeMode::Light)
    }
}

impl ResolvedTheme {
    pub const fn new(
        style: StyleId,
        base: BaseColor,
        accent: Option<AccentColor>,
        mode: ThemeMode,
    ) -> Self {
        Self {
            style,
            base,
            accent,
            mode,
            font: None,
            font_heading: None,
            radius: None,
        }
    }

    pub const fn with_style(mut self, style: StyleId) -> Self {
        self.style = style;
        // Locked styles (Lyra/Sera) and Rhea+large keep the picker on `default`
        // so rem comes from the style (0 for Lyra/Sera, 0.625 otherwise).
        if style.locks_radius()
            || (matches!(self.radius, Some(RadiusId::Large)) && style.disallows_large_radius())
        {
            self.radius = Some(RadiusId::Default);
        }
        self
    }

    pub const fn with_base(mut self, base: BaseColor) -> Self {
        self.base = base;
        self
    }

    pub const fn with_accent(mut self, accent: Option<AccentColor>) -> Self {
        self.accent = accent;
        self
    }

    pub const fn with_mode(mut self, mode: ThemeMode) -> Self {
        self.mode = mode;
        self
    }

    /// Override body font (`--font-sans`).
    pub const fn with_font(mut self, font: FontId) -> Self {
        self.font = Some(font);
        self
    }

    /// Override heading font (`--font-heading`), including [`FontHeading::Inherit`].
    pub const fn with_font_heading(mut self, font_heading: FontHeading) -> Self {
        self.font_heading = Some(font_heading);
        self
    }

    pub const fn with_radius(mut self, radius: RadiusId) -> Self {
        self.radius = Some(self.style.resolve_radius(radius));
        self
    }

    pub const fn style_pack(self) -> StylePack {
        let mut pack = self.style.pack();
        let radius = match self.radius {
            Some(radius) => radius,
            None => self.style.default_radius_id(),
        };
        pack = pack.with_radius(radius);

        let style_fonts = self.style.pack().font_pack;
        let heading_inherits = match self.font_heading {
            Some(FontHeading::Inherit) => true,
            Some(FontHeading::Font(_)) => false,
            None => font_id_eq(style_fonts.heading, style_fonts.sans),
        };

        if let Some(font) = self.font {
            pack = pack.with_font(font);
            if heading_inherits {
                pack = pack.with_font_heading(font);
            }
        }

        if let Some(FontHeading::Font(heading)) = self.font_heading {
            pack = pack.with_font_heading(heading);
        } else if matches!(self.font_heading, Some(FontHeading::Inherit)) {
            pack = pack.with_font_heading(pack.font_pack.sans);
        }

        pack
    }

    pub const fn font_pack(self) -> FontPack {
        self.style_pack().font_pack
    }

    pub fn font_id(self) -> FontId {
        self.font
            .unwrap_or_else(|| self.style.pack().font_pack.sans)
    }

    pub fn font_heading(self) -> FontHeading {
        self.font_heading
            .unwrap_or_else(|| FontHeading::from_pack(self.style.pack().font_pack))
    }

    pub fn radius_id(self) -> RadiusId {
        let radius = self
            .radius
            .unwrap_or_else(|| self.style.default_radius_id());
        self.style.resolve_radius(radius)
    }

    pub const fn icon_set(self) -> IconSet {
        IconSet
    }

    /// OKLCH token for a semantic color slot after base + optional accent overlay.
    pub fn semantic_oklch(self, token: SemanticColor) -> Option<OklchColor> {
        if let Some(accent) = self.accent
            && let Some(overlaid) = accent_token(accent, self.mode, token)
        {
            return Some(overlaid);
        }
        base_token(self.base, self.mode, token)
    }

    /// Twill [`ColorValue`] for a semantic slot.
    pub fn semantic_color_value(self, token: SemanticColor) -> Option<ColorValue> {
        self.semantic_oklch(token).map(OklchColor::to_color_value)
    }

    /// All semantic slots that resolve for this theme.
    pub fn semantic_vars(self) -> SemanticThemeTable {
        SemanticThemeTable {
            background: self.require(SemanticColor::Background),
            foreground: self.require(SemanticColor::Foreground),
            card: self.require(SemanticColor::Card),
            card_foreground: self.require(SemanticColor::CardForeground),
            popover: self.require(SemanticColor::Popover),
            popover_foreground: self.require(SemanticColor::PopoverForeground),
            primary: self.require(SemanticColor::Primary),
            primary_foreground: self.require(SemanticColor::PrimaryForeground),
            secondary: self.require(SemanticColor::Secondary),
            secondary_foreground: self.require(SemanticColor::SecondaryForeground),
            muted: self.require(SemanticColor::Muted),
            muted_foreground: self.require(SemanticColor::MutedForeground),
            accent: self.require(SemanticColor::Accent),
            accent_foreground: self.require(SemanticColor::AccentForeground),
            destructive: self.require(SemanticColor::Destructive),
            border: self.require(SemanticColor::Border),
            input: self.require(SemanticColor::Input),
            ring: self.require(SemanticColor::Ring),
            chart_1: self.require(SemanticColor::Chart1),
            chart_2: self.require(SemanticColor::Chart2),
            chart_3: self.require(SemanticColor::Chart3),
            chart_4: self.require(SemanticColor::Chart4),
            chart_5: self.require(SemanticColor::Chart5),
            sidebar: self.require(SemanticColor::Sidebar),
            sidebar_foreground: self.require(SemanticColor::SidebarForeground),
            sidebar_primary: self.require(SemanticColor::SidebarPrimary),
            sidebar_primary_foreground: self.require(SemanticColor::SidebarPrimaryForeground),
            sidebar_accent: self.require(SemanticColor::SidebarAccent),
            sidebar_accent_foreground: self.require(SemanticColor::SidebarAccentForeground),
            sidebar_border: self.require(SemanticColor::SidebarBorder),
            sidebar_ring: self.require(SemanticColor::SidebarRing),
        }
    }

    fn require(self, token: SemanticColor) -> OklchColor {
        self.semantic_oklch(token).unwrap_or_else(|| {
            panic!(
                "missing semantic token {:?} for {:?} / {:?} / {:?}",
                token, self.base, self.accent, self.mode
            )
        })
    }
}

const fn font_id_eq(a: FontId, b: FontId) -> bool {
    matches!(
        (a, b),
        (FontId::Geist, FontId::Geist)
            | (FontId::GeistMono, FontId::GeistMono)
            | (FontId::Inter, FontId::Inter)
            | (FontId::InstrumentSerif, FontId::InstrumentSerif)
            | (FontId::JetBrainsMono, FontId::JetBrainsMono)
    )
}

/// Resolved OKLCH semantic palette (backend-agnostic).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SemanticThemeTable {
    pub background: OklchColor,
    pub foreground: OklchColor,
    pub card: OklchColor,
    pub card_foreground: OklchColor,
    pub popover: OklchColor,
    pub popover_foreground: OklchColor,
    pub primary: OklchColor,
    pub primary_foreground: OklchColor,
    pub secondary: OklchColor,
    pub secondary_foreground: OklchColor,
    pub muted: OklchColor,
    pub muted_foreground: OklchColor,
    pub accent: OklchColor,
    pub accent_foreground: OklchColor,
    pub destructive: OklchColor,
    pub border: OklchColor,
    pub input: OklchColor,
    pub ring: OklchColor,
    pub chart_1: OklchColor,
    pub chart_2: OklchColor,
    pub chart_3: OklchColor,
    pub chart_4: OklchColor,
    pub chart_5: OklchColor,
    pub sidebar: OklchColor,
    pub sidebar_foreground: OklchColor,
    pub sidebar_primary: OklchColor,
    pub sidebar_primary_foreground: OklchColor,
    pub sidebar_accent: OklchColor,
    pub sidebar_accent_foreground: OklchColor,
    pub sidebar_border: OklchColor,
    pub sidebar_ring: OklchColor,
}

impl SemanticThemeTable {
    pub fn get(self, token: SemanticColor) -> Option<OklchColor> {
        Some(match token {
            SemanticColor::Background => self.background,
            SemanticColor::Foreground => self.foreground,
            SemanticColor::Card => self.card,
            SemanticColor::CardForeground => self.card_foreground,
            SemanticColor::Popover => self.popover,
            SemanticColor::PopoverForeground => self.popover_foreground,
            SemanticColor::Primary => self.primary,
            SemanticColor::PrimaryForeground => self.primary_foreground,
            SemanticColor::Secondary => self.secondary,
            SemanticColor::SecondaryForeground => self.secondary_foreground,
            SemanticColor::Muted => self.muted,
            SemanticColor::MutedForeground => self.muted_foreground,
            SemanticColor::Accent => self.accent,
            SemanticColor::AccentForeground => self.accent_foreground,
            SemanticColor::Destructive => self.destructive,
            SemanticColor::Border => self.border,
            SemanticColor::Input => self.input,
            SemanticColor::Ring => self.ring,
            SemanticColor::Chart1 => self.chart_1,
            SemanticColor::Chart2 => self.chart_2,
            SemanticColor::Chart3 => self.chart_3,
            SemanticColor::Chart4 => self.chart_4,
            SemanticColor::Chart5 => self.chart_5,
            SemanticColor::Sidebar => self.sidebar,
            SemanticColor::SidebarForeground => self.sidebar_foreground,
            SemanticColor::SidebarPrimary => self.sidebar_primary,
            SemanticColor::SidebarPrimaryForeground => self.sidebar_primary_foreground,
            SemanticColor::SidebarAccent => self.sidebar_accent,
            SemanticColor::SidebarAccentForeground => self.sidebar_accent_foreground,
            SemanticColor::SidebarBorder => self.sidebar_border,
            SemanticColor::SidebarRing => self.sidebar_ring,
        })
    }
}
