use iced::{Background, Color};
use twill::backends::iced::to_color_value;
use twill::prelude::{
    BorderRadius, Color as TwillColor, ColorFamily, ColorValue, ComputeValue, DynamicSemanticTheme,
    Scale, SemanticColor, SemanticThemeVars, ThemeVariant,
};

#[derive(Clone, Copy, Debug)]
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AccentColor {
    Gray,
    Gold,
    Bronze,
    Brown,
    Yellow,
    Amber,
    Orange,
    Tomato,
    Red,
    Ruby,
    Crimson,
    Pink,
    Plum,
    Purple,
    Violet,
    Iris,
    Indigo,
    Blue,
    Cyan,
    Teal,
    Jade,
    Green,
    Grass,
    Lime,
    Mint,
    Sky,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ControlSize {
    Sm,
    #[default]
    Md,
    Lg,
    Icon,
    IconSm,
    IconLg,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ControlVariant {
    #[default]
    Primary,
    Secondary,
    Destructive,
}

impl ControlSize {
    pub fn button_size(self) -> crate::button::ButtonSize {
        use crate::button::ButtonSize;
        match self {
            ControlSize::Sm | ControlSize::IconSm => ButtonSize::Size1,
            ControlSize::Md | ControlSize::Icon => ButtonSize::Size2,
            ControlSize::Lg | ControlSize::IconLg => ButtonSize::Size3,
        }
    }

    pub fn radius(self) -> crate::button::ButtonRadius {
        use crate::button::ButtonRadius;
        match self {
            ControlSize::Sm => ButtonRadius::Small,
            ControlSize::Md | ControlSize::IconSm => ButtonRadius::Medium,
            ControlSize::Lg | ControlSize::Icon | ControlSize::IconLg => ButtonRadius::Large,
        }
    }
}

impl AccentColor {
    pub fn is_destructive(self) -> bool {
        matches!(
            self,
            AccentColor::Red | AccentColor::Tomato | AccentColor::Ruby | AccentColor::Crimson
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShadcnBaseColor {
    Neutral,
    Stone,
    Zinc,
    Gray,
    Slate,
}

#[derive(Clone, Copy, Debug)]
struct AccentSwatch {
    low: Color,
    accent: Color,
    text: Color,
    soft: Color,
    contrast: Color,
    strong: Color,
}

impl Palette {
    pub fn dark() -> Self {
        palette_from_base(ShadcnBaseColor::Neutral, ThemeVariant::Dark)
    }

    pub fn light() -> Self {
        palette_from_base(ShadcnBaseColor::Neutral, ThemeVariant::Light)
    }

    pub fn shadcn_light(base: ShadcnBaseColor) -> Self {
        palette_from_base(base, ThemeVariant::Light)
    }

    pub fn shadcn_dark(base: ShadcnBaseColor) -> Self {
        palette_from_base(base, ThemeVariant::Dark)
    }
}

impl Default for Palette {
    fn default() -> Self {
        Self::light()
    }
}

pub fn accent_color(palette: &Palette, color: AccentColor) -> Color {
    accent_swatch(palette, color).accent
}

pub fn accent_foreground(palette: &Palette, color: AccentColor) -> Color {
    accent_swatch(palette, color).contrast
}

pub fn accent_text(palette: &Palette, color: AccentColor) -> Color {
    accent_swatch(palette, color).text
}

pub fn accent_soft(palette: &Palette, color: AccentColor) -> Color {
    accent_swatch(palette, color).soft
}

pub fn accent_soft_foreground(palette: &Palette, color: AccentColor) -> Color {
    accent_swatch(palette, color).text
}

pub fn accent_low(palette: &Palette, color: AccentColor) -> Color {
    accent_swatch(palette, color).low
}

pub fn accent_high(palette: &Palette, color: AccentColor) -> Color {
    accent_swatch(palette, color).strong
}

pub(crate) fn is_dark(palette: &Palette) -> bool {
    relative_luminance(palette.background) < 0.5
}

pub(crate) fn ensure_contrast(
    background: Background,
    fallback_bg: Color,
    foreground: Color,
) -> Color {
    let bg = effective_background(background, fallback_bg);
    let contrast = contrast_ratio(bg, foreground);
    if contrast >= 2.0 {
        return foreground;
    }

    if contrast_ratio(bg, Color::WHITE) >= contrast_ratio(bg, Color::BLACK) {
        Color::WHITE
    } else {
        Color::BLACK
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Radius {
    pub sm: f32,
    pub md: f32,
    pub lg: f32,
}

impl Default for Radius {
    fn default() -> Self {
        Self {
            sm: BorderRadius::Md.px_value(),
            md: BorderRadius::Lg.px_value(),
            lg: BorderRadius::Xl.px_value(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Spacing {
    pub xs: f32,
    pub sm: f32,
    pub md: f32,
    pub lg: f32,
}

impl Default for Spacing {
    fn default() -> Self {
        Self {
            xs: 4.0,
            sm: 8.0,
            md: 12.0,
            lg: 16.0,
        }
    }
}

/// Linearly interpolate between two colors. `t=0.0` returns `a`, `t=1.0` returns `b`.
pub fn mix(a: Color, b: Color, t: f32) -> Color {
    let t = t.clamp(0.0, 1.0);
    Color {
        r: a.r + (b.r - a.r) * t,
        g: a.g + (b.g - a.g) * t,
        b: a.b + (b.b - a.b) * t,
        a: a.a + (b.a - a.a) * t,
    }
}

fn palette_from_base(base: ShadcnBaseColor, variant: ThemeVariant) -> Palette {
    if matches!(base, ShadcnBaseColor::Neutral) {
        return palette_from_semantic_theme(SemanticThemeVars::shadcn_neutral(), variant);
    }

    let family = base_family(base);
    let white = to_color_value(ColorValue::from_color(TwillColor::white()));
    let light_bg = white;
    let light_fg = family_color(family, Scale::S950);
    let light_primary = family_color(family, Scale::S900);
    let light_primary_fg = family_color(family, Scale::S50);
    let light_secondary = family_color(family, Scale::S100);
    let light_secondary_fg = family_color(family, Scale::S900);
    let light_muted_fg = family_color(family, Scale::S500);
    let light_border = family_color(family, Scale::S200);
    let light_ring = family_color(family, Scale::S400);

    let dark_bg = family_color(family, Scale::S950);
    let dark_fg = family_color(family, Scale::S50);
    let dark_surface = family_color(family, Scale::S900);
    let dark_secondary = family_color(family, Scale::S800);
    let dark_border = apply_opacity(family_color(family, Scale::S200), 0.10);
    let dark_input = apply_opacity(family_color(family, Scale::S200), 0.15);
    let dark_ring = family_color(family, Scale::S500);

    let chart_light = [
        family_color(ColorFamily::Orange, Scale::S600),
        family_color(ColorFamily::Teal, Scale::S600),
        family_color(ColorFamily::Cyan, Scale::S900),
        family_color(ColorFamily::Amber, Scale::S400),
        family_color(ColorFamily::Amber, Scale::S500),
    ];
    let chart_dark = [
        family_color(ColorFamily::Violet, Scale::S500),
        family_color(ColorFamily::Emerald, Scale::S400),
        family_color(ColorFamily::Amber, Scale::S500),
        family_color(ColorFamily::Purple, Scale::S400),
        family_color(ColorFamily::Rose, Scale::S500),
    ];

    match variant {
        ThemeVariant::Light => Palette {
            background: light_bg,
            foreground: light_fg,
            card: light_bg,
            card_foreground: light_fg,
            popover: light_bg,
            popover_foreground: light_fg,
            border: light_border,
            input: light_border,
            ring: light_ring,
            primary: light_primary,
            primary_foreground: light_primary_fg,
            secondary: light_secondary,
            secondary_foreground: light_secondary_fg,
            accent: light_secondary,
            accent_foreground: light_secondary_fg,
            muted: light_secondary,
            muted_foreground: light_muted_fg,
            destructive: family_color(ColorFamily::Red, Scale::S600),
            destructive_foreground: Color::WHITE,
            chart_1: chart_light[0],
            chart_2: chart_light[1],
            chart_3: chart_light[2],
            chart_4: chart_light[3],
            chart_5: chart_light[4],
            sidebar: light_primary_fg,
            sidebar_foreground: light_fg,
            sidebar_primary: light_primary,
            sidebar_primary_foreground: light_primary_fg,
            sidebar_accent: light_secondary,
            sidebar_accent_foreground: light_secondary_fg,
            sidebar_border: light_border,
            sidebar_ring: light_ring,
        },
        ThemeVariant::Dark => Palette {
            background: dark_bg,
            foreground: dark_fg,
            card: dark_surface,
            card_foreground: dark_fg,
            popover: dark_surface,
            popover_foreground: dark_fg,
            border: dark_border,
            input: dark_input,
            ring: dark_ring,
            primary: family_color(family, Scale::S200),
            primary_foreground: dark_surface,
            secondary: dark_secondary,
            secondary_foreground: dark_fg,
            accent: dark_secondary,
            accent_foreground: dark_fg,
            muted: dark_secondary,
            muted_foreground: family_color(family, Scale::S400),
            destructive: family_color(ColorFamily::Red, Scale::S500),
            destructive_foreground: Color::WHITE,
            chart_1: chart_dark[0],
            chart_2: chart_dark[1],
            chart_3: chart_dark[2],
            chart_4: chart_dark[3],
            chart_5: chart_dark[4],
            sidebar: dark_surface,
            sidebar_foreground: dark_fg,
            sidebar_primary: chart_dark[0],
            sidebar_primary_foreground: dark_fg,
            sidebar_accent: dark_secondary,
            sidebar_accent_foreground: dark_fg,
            sidebar_border: dark_border,
            sidebar_ring: dark_ring,
        },
    }
}

pub(crate) fn palette_from_semantic_theme(
    theme: &SemanticThemeVars,
    variant: ThemeVariant,
) -> Palette {
    let destructive = semantic_theme_color(theme, SemanticColor::Destructive, variant);

    palette_from_resolver(
        |token| semantic_theme_color(theme, token, variant),
        preferred_text_for(destructive),
    )
}

pub(crate) fn palette_from_dynamic_semantic_theme(
    theme: &DynamicSemanticTheme,
    variant: ThemeVariant,
) -> Palette {
    let destructive = dynamic_semantic_color(theme, SemanticColor::Destructive, variant);

    palette_from_resolver(
        |token| dynamic_semantic_color(theme, token, variant),
        preferred_text_for(destructive),
    )
}

fn palette_from_resolver(
    mut resolve: impl FnMut(SemanticColor) -> Color,
    destructive_foreground: Color,
) -> Palette {
    Palette {
        background: resolve(SemanticColor::Background),
        foreground: resolve(SemanticColor::Foreground),
        card: resolve(SemanticColor::Card),
        card_foreground: resolve(SemanticColor::CardForeground),
        popover: resolve(SemanticColor::Popover),
        popover_foreground: resolve(SemanticColor::PopoverForeground),
        border: resolve(SemanticColor::Border),
        input: resolve(SemanticColor::Input),
        ring: resolve(SemanticColor::Ring),
        primary: resolve(SemanticColor::Primary),
        primary_foreground: resolve(SemanticColor::PrimaryForeground),
        secondary: resolve(SemanticColor::Secondary),
        secondary_foreground: resolve(SemanticColor::SecondaryForeground),
        accent: resolve(SemanticColor::Accent),
        accent_foreground: resolve(SemanticColor::AccentForeground),
        muted: resolve(SemanticColor::Muted),
        muted_foreground: resolve(SemanticColor::MutedForeground),
        destructive: resolve(SemanticColor::Destructive),
        destructive_foreground,
        chart_1: resolve(SemanticColor::Chart1),
        chart_2: resolve(SemanticColor::Chart2),
        chart_3: resolve(SemanticColor::Chart3),
        chart_4: resolve(SemanticColor::Chart4),
        chart_5: resolve(SemanticColor::Chart5),
        sidebar: resolve(SemanticColor::Sidebar),
        sidebar_foreground: resolve(SemanticColor::SidebarForeground),
        sidebar_primary: resolve(SemanticColor::SidebarPrimary),
        sidebar_primary_foreground: resolve(SemanticColor::SidebarPrimaryForeground),
        sidebar_accent: resolve(SemanticColor::SidebarAccent),
        sidebar_accent_foreground: resolve(SemanticColor::SidebarAccentForeground),
        sidebar_border: resolve(SemanticColor::SidebarBorder),
        sidebar_ring: resolve(SemanticColor::SidebarRing),
    }
}

fn semantic_theme_color(
    theme: &SemanticThemeVars,
    token: SemanticColor,
    variant: ThemeVariant,
) -> Color {
    theme
        .resolve_value(token, variant)
        .map(to_color_value)
        .unwrap_or(Color::BLACK)
}

fn dynamic_semantic_color(
    theme: &DynamicSemanticTheme,
    token: SemanticColor,
    variant: ThemeVariant,
) -> Color {
    theme
        .resolve(token, variant)
        .map(to_color_value)
        .unwrap_or(Color::BLACK)
}

fn accent_swatch(palette: &Palette, color: AccentColor) -> AccentSwatch {
    let variant = if is_dark(palette) {
        ThemeVariant::Dark
    } else {
        ThemeVariant::Light
    };
    let scale = accent_scale(color);

    let (low, accent, text, soft, strong) = match variant {
        ThemeVariant::Light => (
            scale_color(&scale, Scale::S100),
            scale_color(&scale, Scale::S600),
            scale_color(&scale, Scale::S700),
            scale_color(&scale, Scale::S100),
            scale_color(&scale, Scale::S800),
        ),
        ThemeVariant::Dark => (
            scale_color(&scale, Scale::S800),
            scale_color(&scale, Scale::S400),
            scale_color(&scale, Scale::S300),
            scale_color(&scale, Scale::S900),
            scale_color(&scale, Scale::S200),
        ),
    };

    let contrast = preferred_text_for(accent);

    AccentSwatch {
        low,
        accent,
        text,
        soft,
        contrast,
        strong,
    }
}

fn accent_scale(color: AccentColor) -> [(Scale, ColorValue); 11] {
    accent_seed(color).generate_scale_map_oklch()
}

fn accent_seed(color: AccentColor) -> ColorValue {
    match color {
        AccentColor::Gray => TwillColor::gray(Scale::S600).compute(),
        AccentColor::Gold => ColorValue::from_rgb(210, 160, 70),
        AccentColor::Bronze => ColorValue::from_rgb(161, 104, 63),
        AccentColor::Brown => ColorValue::from_rgb(128, 92, 74),
        AccentColor::Yellow => TwillColor::yellow(Scale::S500).compute(),
        AccentColor::Amber => TwillColor::amber(Scale::S500).compute(),
        AccentColor::Orange => TwillColor::orange(Scale::S500).compute(),
        AccentColor::Tomato => ColorValue::from_rgb(229, 77, 46),
        AccentColor::Red => TwillColor::red(Scale::S500).compute(),
        AccentColor::Ruby => ColorValue::from_rgb(196, 58, 112),
        AccentColor::Crimson => ColorValue::from_rgb(220, 38, 94),
        AccentColor::Pink => TwillColor::pink(Scale::S500).compute(),
        AccentColor::Plum => ColorValue::from_rgb(143, 82, 179),
        AccentColor::Purple => TwillColor::purple(Scale::S500).compute(),
        AccentColor::Violet => TwillColor::violet(Scale::S500).compute(),
        AccentColor::Iris => ColorValue::from_rgb(92, 104, 216),
        AccentColor::Indigo => TwillColor::indigo(Scale::S500).compute(),
        AccentColor::Blue => TwillColor::blue(Scale::S500).compute(),
        AccentColor::Cyan => TwillColor::cyan(Scale::S500).compute(),
        AccentColor::Teal => TwillColor::teal(Scale::S500).compute(),
        AccentColor::Jade => ColorValue::from_rgb(0, 168, 107),
        AccentColor::Green => TwillColor::green(Scale::S500).compute(),
        AccentColor::Grass => ColorValue::from_rgb(95, 159, 53),
        AccentColor::Lime => TwillColor::lime(Scale::S500).compute(),
        AccentColor::Mint => ColorValue::from_rgb(35, 183, 131),
        AccentColor::Sky => TwillColor::sky(Scale::S500).compute(),
    }
}

fn scale_color(scale: &[(Scale, ColorValue); 11], wanted: Scale) -> Color {
    scale
        .iter()
        .find_map(|(key, value)| (*key == wanted).then_some(to_color_value(*value)))
        .unwrap_or(Color::BLACK)
}

fn preferred_text_for(color: Color) -> Color {
    let rgb = color_to_rgb8(color);
    let preferred = ColorValue::from_rgb(rgb.0, rgb.1, rgb.2).preferred_text_color();
    match preferred {
        twill::prelude::SpecialColor::Black => Color::BLACK,
        twill::prelude::SpecialColor::White => Color::WHITE,
        twill::prelude::SpecialColor::Transparent | twill::prelude::SpecialColor::Current => {
            Color::WHITE
        }
    }
}

fn base_family(base: ShadcnBaseColor) -> ColorFamily {
    match base {
        ShadcnBaseColor::Neutral => ColorFamily::Neutral,
        ShadcnBaseColor::Stone => ColorFamily::Stone,
        ShadcnBaseColor::Zinc => ColorFamily::Zinc,
        ShadcnBaseColor::Gray => ColorFamily::Gray,
        ShadcnBaseColor::Slate => ColorFamily::Slate,
    }
}

fn family_color(family: ColorFamily, scale: Scale) -> Color {
    to_color_value(TwillColor::new(family, scale).compute())
}

fn apply_opacity(mut color: Color, alpha: f32) -> Color {
    color.a *= alpha.clamp(0.0, 1.0);
    color
}

fn effective_background(background: Background, fallback_bg: Color) -> Color {
    match background {
        Background::Color(color) => {
            if color.a >= 1.0 {
                color
            } else {
                blend_over(color, fallback_bg)
            }
        }
        _ => fallback_bg,
    }
}

fn blend_over(fg: Color, bg: Color) -> Color {
    let a = fg.a.clamp(0.0, 1.0);
    Color {
        r: fg.r * a + bg.r * (1.0 - a),
        g: fg.g * a + bg.g * (1.0 - a),
        b: fg.b * a + bg.b * (1.0 - a),
        a: 1.0,
    }
}

fn contrast_ratio(a: Color, b: Color) -> f32 {
    let l1 = relative_luminance(a);
    let l2 = relative_luminance(b);
    let (lighter, darker) = if l1 >= l2 { (l1, l2) } else { (l2, l1) };
    (lighter + 0.05) / (darker + 0.05)
}

fn relative_luminance(color: Color) -> f32 {
    fn to_linear(channel: f32) -> f32 {
        if channel <= 0.04045 {
            channel / 12.92
        } else {
            ((channel + 0.055) / 1.055).powf(2.4)
        }
    }

    let r = to_linear(color.r);
    let g = to_linear(color.g);
    let b = to_linear(color.b);

    0.2126 * r + 0.7152 * g + 0.0722 * b
}

fn color_to_rgb8(color: Color) -> (u8, u8, u8) {
    let clamp = |value: f32| (value.clamp(0.0, 1.0) * 255.0).round() as u8;
    (clamp(color.r), clamp(color.g), clamp(color.b))
}
