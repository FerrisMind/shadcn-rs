//! Base and accent color registries ported from shadcn-svelte `registry/colors`.

use twill_core::prelude::theme::SemanticColor;
use twill_core::tokens::ColorValue;

use crate::generated;

/// OKLCH color sample with optional alpha.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OklchColor {
    pub l: f32,
    pub c: f32,
    pub h: f32,
    pub a: f32,
}

impl OklchColor {
    pub const fn solid(l: f32, c: f32, h: f32) -> Self {
        Self { l, c, h, a: 1.0 }
    }

    pub fn to_color_value(self) -> ColorValue {
        ColorValue::from_oklch(self.l, self.c, self.h).with_alpha(self.a)
    }
}

/// Full-theme base colors (include background/foreground/...).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BaseColor {
    Neutral,
    Zinc,
    Stone,
    Mauve,
    Mist,
    Olive,
    Taupe,
}

impl BaseColor {
    pub const ALL: [Self; 7] = [
        Self::Neutral,
        Self::Zinc,
        Self::Stone,
        Self::Mauve,
        Self::Mist,
        Self::Olive,
        Self::Taupe,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Neutral => "neutral",
            Self::Zinc => "zinc",
            Self::Stone => "stone",
            Self::Mauve => "mauve",
            Self::Mist => "mist",
            Self::Olive => "olive",
            Self::Taupe => "taupe",
        }
    }

    pub fn from_str_name(name: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|base| base.as_str().eq_ignore_ascii_case(name))
    }
}

/// Accent overlays (primary/chart/sidebar-primary family).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AccentColor {
    Amber,
    Blue,
    Cyan,
    Emerald,
    Fuchsia,
    Green,
    Indigo,
    Lime,
    Orange,
    Pink,
    Purple,
    Red,
    Rose,
    Sky,
    Teal,
    Violet,
    Yellow,
}

impl AccentColor {
    pub const ALL: [Self; 17] = [
        Self::Amber,
        Self::Blue,
        Self::Cyan,
        Self::Emerald,
        Self::Fuchsia,
        Self::Green,
        Self::Indigo,
        Self::Lime,
        Self::Orange,
        Self::Pink,
        Self::Purple,
        Self::Red,
        Self::Rose,
        Self::Sky,
        Self::Teal,
        Self::Violet,
        Self::Yellow,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Amber => "amber",
            Self::Blue => "blue",
            Self::Cyan => "cyan",
            Self::Emerald => "emerald",
            Self::Fuchsia => "fuchsia",
            Self::Green => "green",
            Self::Indigo => "indigo",
            Self::Lime => "lime",
            Self::Orange => "orange",
            Self::Pink => "pink",
            Self::Purple => "purple",
            Self::Red => "red",
            Self::Rose => "rose",
            Self::Sky => "sky",
            Self::Teal => "teal",
            Self::Violet => "violet",
            Self::Yellow => "yellow",
        }
    }

    pub fn from_str_name(name: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|accent| accent.as_str().eq_ignore_ascii_case(name))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ThemeMode {
    Light,
    Dark,
}

impl ThemeMode {
    pub const fn is_dark(self) -> bool {
        matches!(self, Self::Dark)
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Light => "light",
            Self::Dark => "dark",
        }
    }
}

pub(crate) fn base_token(
    base: BaseColor,
    mode: ThemeMode,
    token: SemanticColor,
) -> Option<OklchColor> {
    match mode {
        ThemeMode::Light => base_light(base, token),
        ThemeMode::Dark => base_dark(base, token),
    }
}

pub(crate) fn accent_token(
    accent: AccentColor,
    mode: ThemeMode,
    token: SemanticColor,
) -> Option<OklchColor> {
    match mode {
        ThemeMode::Light => accent_light(accent, token),
        ThemeMode::Dark => accent_dark(accent, token),
    }
}

macro_rules! map_base_token {
    ($mod:path, $token:ident) => {{
        use $mod as palette;
        match $token {
            SemanticColor::Background => Some(palette::BACKGROUND),
            SemanticColor::Foreground => Some(palette::FOREGROUND),
            SemanticColor::Card => Some(palette::CARD),
            SemanticColor::CardForeground => Some(palette::CARD_FOREGROUND),
            SemanticColor::Popover => Some(palette::POPOVER),
            SemanticColor::PopoverForeground => Some(palette::POPOVER_FOREGROUND),
            SemanticColor::Primary => Some(palette::PRIMARY),
            SemanticColor::PrimaryForeground => Some(palette::PRIMARY_FOREGROUND),
            SemanticColor::Secondary => Some(palette::SECONDARY),
            SemanticColor::SecondaryForeground => Some(palette::SECONDARY_FOREGROUND),
            SemanticColor::Muted => Some(palette::MUTED),
            SemanticColor::MutedForeground => Some(palette::MUTED_FOREGROUND),
            SemanticColor::Accent => Some(palette::ACCENT),
            SemanticColor::AccentForeground => Some(palette::ACCENT_FOREGROUND),
            SemanticColor::Destructive => Some(palette::DESTRUCTIVE),
            SemanticColor::Border => Some(palette::BORDER),
            SemanticColor::Input => Some(palette::INPUT),
            SemanticColor::Ring => Some(palette::RING),
            SemanticColor::Chart1 => Some(palette::CHART_1),
            SemanticColor::Chart2 => Some(palette::CHART_2),
            SemanticColor::Chart3 => Some(palette::CHART_3),
            SemanticColor::Chart4 => Some(palette::CHART_4),
            SemanticColor::Chart5 => Some(palette::CHART_5),
            SemanticColor::Sidebar => Some(palette::SIDEBAR),
            SemanticColor::SidebarForeground => Some(palette::SIDEBAR_FOREGROUND),
            SemanticColor::SidebarPrimary => Some(palette::SIDEBAR_PRIMARY),
            SemanticColor::SidebarPrimaryForeground => Some(palette::SIDEBAR_PRIMARY_FOREGROUND),
            SemanticColor::SidebarAccent => Some(palette::SIDEBAR_ACCENT),
            SemanticColor::SidebarAccentForeground => Some(palette::SIDEBAR_ACCENT_FOREGROUND),
            SemanticColor::SidebarBorder => Some(palette::SIDEBAR_BORDER),
            SemanticColor::SidebarRing => Some(palette::SIDEBAR_RING),
        }
    }};
}

macro_rules! map_accent_token {
    ($mod:path, $token:ident) => {{
        use $mod as palette;
        match $token {
            SemanticColor::Primary => Some(palette::PRIMARY),
            SemanticColor::PrimaryForeground => Some(palette::PRIMARY_FOREGROUND),
            SemanticColor::Secondary => Some(palette::SECONDARY),
            SemanticColor::SecondaryForeground => Some(palette::SECONDARY_FOREGROUND),
            SemanticColor::Chart1 => Some(palette::CHART_1),
            SemanticColor::Chart2 => Some(palette::CHART_2),
            SemanticColor::Chart3 => Some(palette::CHART_3),
            SemanticColor::Chart4 => Some(palette::CHART_4),
            SemanticColor::Chart5 => Some(palette::CHART_5),
            SemanticColor::SidebarPrimary => Some(palette::SIDEBAR_PRIMARY),
            SemanticColor::SidebarPrimaryForeground => Some(palette::SIDEBAR_PRIMARY_FOREGROUND),
            _ => None,
        }
    }};
}

fn base_light(base: BaseColor, token: SemanticColor) -> Option<OklchColor> {
    match base {
        BaseColor::Neutral => map_base_token!(generated::neutral::light, token),
        BaseColor::Zinc => map_base_token!(generated::zinc::light, token),
        BaseColor::Stone => map_base_token!(generated::stone::light, token),
        BaseColor::Mauve => map_base_token!(generated::mauve::light, token),
        BaseColor::Mist => map_base_token!(generated::mist::light, token),
        BaseColor::Olive => map_base_token!(generated::olive::light, token),
        BaseColor::Taupe => map_base_token!(generated::taupe::light, token),
    }
}

fn base_dark(base: BaseColor, token: SemanticColor) -> Option<OklchColor> {
    match base {
        BaseColor::Neutral => map_base_token!(generated::neutral::dark, token),
        BaseColor::Zinc => map_base_token!(generated::zinc::dark, token),
        BaseColor::Stone => map_base_token!(generated::stone::dark, token),
        BaseColor::Mauve => map_base_token!(generated::mauve::dark, token),
        BaseColor::Mist => map_base_token!(generated::mist::dark, token),
        BaseColor::Olive => map_base_token!(generated::olive::dark, token),
        BaseColor::Taupe => map_base_token!(generated::taupe::dark, token),
    }
}

fn accent_light(accent: AccentColor, token: SemanticColor) -> Option<OklchColor> {
    match accent {
        AccentColor::Amber => map_accent_token!(generated::amber::light, token),
        AccentColor::Blue => map_accent_token!(generated::blue::light, token),
        AccentColor::Cyan => map_accent_token!(generated::cyan::light, token),
        AccentColor::Emerald => map_accent_token!(generated::emerald::light, token),
        AccentColor::Fuchsia => map_accent_token!(generated::fuchsia::light, token),
        AccentColor::Green => map_accent_token!(generated::green::light, token),
        AccentColor::Indigo => map_accent_token!(generated::indigo::light, token),
        AccentColor::Lime => map_accent_token!(generated::lime::light, token),
        AccentColor::Orange => map_accent_token!(generated::orange::light, token),
        AccentColor::Pink => map_accent_token!(generated::pink::light, token),
        AccentColor::Purple => map_accent_token!(generated::purple::light, token),
        AccentColor::Red => map_accent_token!(generated::red::light, token),
        AccentColor::Rose => map_accent_token!(generated::rose::light, token),
        AccentColor::Sky => map_accent_token!(generated::sky::light, token),
        AccentColor::Teal => map_accent_token!(generated::teal::light, token),
        AccentColor::Violet => map_accent_token!(generated::violet::light, token),
        AccentColor::Yellow => map_accent_token!(generated::yellow::light, token),
    }
}

fn accent_dark(accent: AccentColor, token: SemanticColor) -> Option<OklchColor> {
    match accent {
        AccentColor::Amber => map_accent_token!(generated::amber::dark, token),
        AccentColor::Blue => map_accent_token!(generated::blue::dark, token),
        AccentColor::Cyan => map_accent_token!(generated::cyan::dark, token),
        AccentColor::Emerald => map_accent_token!(generated::emerald::dark, token),
        AccentColor::Fuchsia => map_accent_token!(generated::fuchsia::dark, token),
        AccentColor::Green => map_accent_token!(generated::green::dark, token),
        AccentColor::Indigo => map_accent_token!(generated::indigo::dark, token),
        AccentColor::Lime => map_accent_token!(generated::lime::dark, token),
        AccentColor::Orange => map_accent_token!(generated::orange::dark, token),
        AccentColor::Pink => map_accent_token!(generated::pink::dark, token),
        AccentColor::Purple => map_accent_token!(generated::purple::dark, token),
        AccentColor::Red => map_accent_token!(generated::red::dark, token),
        AccentColor::Rose => map_accent_token!(generated::rose::dark, token),
        AccentColor::Sky => map_accent_token!(generated::sky::dark, token),
        AccentColor::Teal => map_accent_token!(generated::teal::dark, token),
        AccentColor::Violet => map_accent_token!(generated::violet::dark, token),
        AccentColor::Yellow => map_accent_token!(generated::yellow::dark, token),
    }
}
