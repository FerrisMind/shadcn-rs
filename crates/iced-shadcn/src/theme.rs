use crate::tokens::{Palette, Radius, Spacing};
use iced::Color;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorToken {
    Background,
    Foreground,
    Card,
    CardForeground,
    Popover,
    PopoverForeground,
    Border,
    Input,
    Ring,
    Primary,
    PrimaryForeground,
    Secondary,
    SecondaryForeground,
    Accent,
    AccentForeground,
    Muted,
    MutedForeground,
    Destructive,
    DestructiveForeground,
    Chart1,
    Chart2,
    Chart3,
    Chart4,
    Chart5,
    Sidebar,
    SidebarForeground,
    SidebarPrimary,
    SidebarPrimaryForeground,
    SidebarAccent,
    SidebarAccentForeground,
    SidebarBorder,
    SidebarRing,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RadiusToken {
    Sm,
    Md,
    Lg,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpacingToken {
    Xs,
    Sm,
    Md,
    Lg,
}

pub trait ThemeTokensSource {
    fn color(&self, token: ColorToken) -> Color;
    fn radius(&self, token: RadiusToken) -> f32;
    fn spacing(&self, token: SpacingToken) -> f32;
}

#[derive(Clone, Debug)]
pub struct Theme {
    pub palette: Palette,
    pub radius: Radius,
    pub spacing: Spacing,
}

impl Theme {
    pub fn from_parts(palette: Palette, radius: Radius, spacing: Spacing) -> Self {
        Self {
            palette,
            radius,
            spacing,
        }
    }

    pub fn light() -> Self {
        Self::from_parts(Palette::light(), Radius::default(), Spacing::default())
    }

    pub fn dark() -> Self {
        Self::from_parts(Palette::dark(), Radius::default(), Spacing::default())
    }

    pub fn with_palette(palette: Palette) -> Self {
        Self::from_parts(palette, Radius::default(), Spacing::default())
    }

    pub fn with_radius(mut self, radius: Radius) -> Self {
        self.radius = radius;
        self
    }

    pub fn with_spacing(mut self, spacing: Spacing) -> Self {
        self.spacing = spacing;
        self
    }

    pub fn from_tokens(source: &impl ThemeTokensSource) -> Self {
        let palette = Palette {
            background: source.color(ColorToken::Background),
            foreground: source.color(ColorToken::Foreground),
            card: source.color(ColorToken::Card),
            card_foreground: source.color(ColorToken::CardForeground),
            popover: source.color(ColorToken::Popover),
            popover_foreground: source.color(ColorToken::PopoverForeground),
            border: source.color(ColorToken::Border),
            input: source.color(ColorToken::Input),
            ring: source.color(ColorToken::Ring),
            primary: source.color(ColorToken::Primary),
            primary_foreground: source.color(ColorToken::PrimaryForeground),
            secondary: source.color(ColorToken::Secondary),
            secondary_foreground: source.color(ColorToken::SecondaryForeground),
            accent: source.color(ColorToken::Accent),
            accent_foreground: source.color(ColorToken::AccentForeground),
            muted: source.color(ColorToken::Muted),
            muted_foreground: source.color(ColorToken::MutedForeground),
            destructive: source.color(ColorToken::Destructive),
            destructive_foreground: source.color(ColorToken::DestructiveForeground),
            chart_1: source.color(ColorToken::Chart1),
            chart_2: source.color(ColorToken::Chart2),
            chart_3: source.color(ColorToken::Chart3),
            chart_4: source.color(ColorToken::Chart4),
            chart_5: source.color(ColorToken::Chart5),
            sidebar: source.color(ColorToken::Sidebar),
            sidebar_foreground: source.color(ColorToken::SidebarForeground),
            sidebar_primary: source.color(ColorToken::SidebarPrimary),
            sidebar_primary_foreground: source.color(ColorToken::SidebarPrimaryForeground),
            sidebar_accent: source.color(ColorToken::SidebarAccent),
            sidebar_accent_foreground: source.color(ColorToken::SidebarAccentForeground),
            sidebar_border: source.color(ColorToken::SidebarBorder),
            sidebar_ring: source.color(ColorToken::SidebarRing),
        };

        let radius = Radius {
            sm: source.radius(RadiusToken::Sm),
            md: source.radius(RadiusToken::Md),
            lg: source.radius(RadiusToken::Lg),
        };

        let spacing = Spacing {
            xs: source.spacing(SpacingToken::Xs),
            sm: source.spacing(SpacingToken::Sm),
            md: source.spacing(SpacingToken::Md),
            lg: source.spacing(SpacingToken::Lg),
        };

        Self::from_parts(palette, radius, spacing)
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::light()
    }
}
