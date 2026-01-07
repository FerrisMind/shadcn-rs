use iced::Color;

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

impl AccentColor {
    pub fn is_destructive(self) -> bool {
        matches!(
            self,
            AccentColor::Red | AccentColor::Tomato | AccentColor::Ruby | AccentColor::Crimson
        )
    }
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

#[derive(Clone, Copy, Debug)]
struct AccentPalette {
    light: AccentSwatch,
    dark: AccentSwatch,
}

const ACCENT_PALETTES: [AccentPalette; 26] = include!("accent_palette.rs");

fn accent_palette(color: AccentColor) -> AccentPalette {
    let index = match color {
        AccentColor::Gray => 0,
        AccentColor::Gold => 1,
        AccentColor::Bronze => 2,
        AccentColor::Brown => 3,
        AccentColor::Yellow => 4,
        AccentColor::Amber => 5,
        AccentColor::Orange => 6,
        AccentColor::Tomato => 7,
        AccentColor::Red => 8,
        AccentColor::Ruby => 9,
        AccentColor::Crimson => 10,
        AccentColor::Pink => 11,
        AccentColor::Plum => 12,
        AccentColor::Purple => 13,
        AccentColor::Violet => 14,
        AccentColor::Iris => 15,
        AccentColor::Indigo => 16,
        AccentColor::Blue => 17,
        AccentColor::Cyan => 18,
        AccentColor::Teal => 19,
        AccentColor::Jade => 20,
        AccentColor::Green => 21,
        AccentColor::Grass => 22,
        AccentColor::Lime => 23,
        AccentColor::Mint => 24,
        AccentColor::Sky => 25,
    };
    ACCENT_PALETTES[index]
}

fn accent_swatch(palette: &Palette, color: AccentColor) -> AccentSwatch {
    let accents = accent_palette(color);
    if is_dark(palette) {
        accents.dark
    } else {
        accents.light
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
    fn to_linear(channel: f32) -> f32 {
        if channel <= 0.04045 {
            channel / 12.92
        } else {
            ((channel + 0.055) / 1.055).powf(2.4)
        }
    }

    let r = to_linear(palette.background.r);
    let g = to_linear(palette.background.g);
    let b = to_linear(palette.background.b);
    let luminance = 0.2126 * r + 0.7152 * g + 0.0722 * b;

    luminance < 0.5
}

impl Palette {
    pub fn dark() -> Self {
        Self::shadcn_dark(ShadcnBaseColor::Neutral)
    }

    pub fn light() -> Self {
        Self::shadcn_light(ShadcnBaseColor::Neutral)
    }

    pub fn shadcn_light(base: ShadcnBaseColor) -> Self {
        let (light, _dark) = shadcn_oklch_palettes(base);
        light.to_palette()
    }

    pub fn shadcn_dark(base: ShadcnBaseColor) -> Self {
        let (_light, dark) = shadcn_oklch_palettes(base);
        dark.to_palette()
    }
}

impl Default for Palette {
    fn default() -> Self {
        Self::light()
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
struct Oklch {
    l: f32,
    c: f32,
    h_deg: f32,
    alpha: f32,
}

impl Oklch {
    const fn new(l: f32, c: f32, h_deg: f32) -> Self {
        Self {
            l,
            c,
            h_deg,
            alpha: 1.0,
        }
    }

    const fn with_alpha(l: f32, c: f32, h_deg: f32, alpha: f32) -> Self {
        Self { l, c, h_deg, alpha }
    }

    #[allow(clippy::excessive_precision)]
    fn to_color(self) -> Color {
        let h_rad = self.h_deg.to_radians();
        let a = self.c * h_rad.cos();
        let b = self.c * h_rad.sin();

        let l_ = self.l + 0.396_337_777_4 * a + 0.215_803_757_3 * b;
        let m_ = self.l - 0.105_561_345_8 * a - 0.063_854_172_8 * b;
        let s_ = self.l - 0.089_484_177_5 * a - 1.291_485_548_0 * b;

        let l = l_ * l_ * l_;
        let m = m_ * m_ * m_;
        let s = s_ * s_ * s_;

        let r_lin = 4.076_741_662_1 * l - 3.307_711_591_3 * m + 0.230_969_929_2 * s;
        let g_lin = -1.268_438_004_6 * l + 2.609_757_401_1 * m - 0.341_319_396_5 * s;
        let b_lin = -0.004_196_086_3 * l - 0.703_418_614_7 * m + 1.707_614_701_0 * s;

        fn to_srgb(linear: f32) -> f32 {
            let clamped = linear.clamp(0.0, 1.0);
            if clamped <= 0.003_130_8 {
                12.92 * clamped
            } else {
                1.055 * clamped.powf(1.0 / 2.4) - 0.055
            }
        }

        let r = to_srgb(r_lin).clamp(0.0, 1.0);
        let g = to_srgb(g_lin).clamp(0.0, 1.0);
        let b = to_srgb(b_lin).clamp(0.0, 1.0);
        let a = self.alpha.clamp(0.0, 1.0);

        Color::from_rgba(r, g, b, a)
    }
}

#[derive(Clone, Copy, Debug)]
struct ShadcnOklchPalette {
    background: Oklch,
    foreground: Oklch,
    card: Oklch,
    card_foreground: Oklch,
    popover: Oklch,
    popover_foreground: Oklch,
    primary: Oklch,
    primary_foreground: Oklch,
    secondary: Oklch,
    secondary_foreground: Oklch,
    muted: Oklch,
    muted_foreground: Oklch,
    accent: Oklch,
    accent_foreground: Oklch,
    destructive: Oklch,
    border: Oklch,
    input: Oklch,
    ring: Oklch,
    chart_1: Oklch,
    chart_2: Oklch,
    chart_3: Oklch,
    chart_4: Oklch,
    chart_5: Oklch,
    sidebar: Oklch,
    sidebar_foreground: Oklch,
    sidebar_primary: Oklch,
    sidebar_primary_foreground: Oklch,
    sidebar_accent: Oklch,
    sidebar_accent_foreground: Oklch,
    sidebar_border: Oklch,
    sidebar_ring: Oklch,
}

impl ShadcnOklchPalette {
    fn to_palette(self) -> Palette {
        Palette {
            background: self.background.to_color(),
            foreground: self.foreground.to_color(),
            card: self.card.to_color(),
            card_foreground: self.card_foreground.to_color(),
            popover: self.popover.to_color(),
            popover_foreground: self.popover_foreground.to_color(),
            border: self.border.to_color(),
            input: self.input.to_color(),
            ring: self.ring.to_color(),
            primary: self.primary.to_color(),
            primary_foreground: self.primary_foreground.to_color(),
            secondary: self.secondary.to_color(),
            secondary_foreground: self.secondary_foreground.to_color(),
            accent: self.accent.to_color(),
            accent_foreground: self.accent_foreground.to_color(),
            muted: self.muted.to_color(),
            muted_foreground: self.muted_foreground.to_color(),
            destructive: self.destructive.to_color(),
            destructive_foreground: Oklch::new(0.985, 0.0, 0.0).to_color(),
            chart_1: self.chart_1.to_color(),
            chart_2: self.chart_2.to_color(),
            chart_3: self.chart_3.to_color(),
            chart_4: self.chart_4.to_color(),
            chart_5: self.chart_5.to_color(),
            sidebar: self.sidebar.to_color(),
            sidebar_foreground: self.sidebar_foreground.to_color(),
            sidebar_primary: self.sidebar_primary.to_color(),
            sidebar_primary_foreground: self.sidebar_primary_foreground.to_color(),
            sidebar_accent: self.sidebar_accent.to_color(),
            sidebar_accent_foreground: self.sidebar_accent_foreground.to_color(),
            sidebar_border: self.sidebar_border.to_color(),
            sidebar_ring: self.sidebar_ring.to_color(),
        }
    }
}

fn shadcn_oklch_palettes(base: ShadcnBaseColor) -> (ShadcnOklchPalette, ShadcnOklchPalette) {
    let chart_light = (
        Oklch::new(0.646, 0.222, 41.116),
        Oklch::new(0.6, 0.118, 184.704),
        Oklch::new(0.398, 0.07, 227.392),
        Oklch::new(0.828, 0.189, 84.429),
        Oklch::new(0.769, 0.188, 70.08),
    );
    let chart_dark = (
        Oklch::new(0.488, 0.243, 264.376),
        Oklch::new(0.696, 0.17, 162.48),
        Oklch::new(0.769, 0.188, 70.08),
        Oklch::new(0.627, 0.265, 303.9),
        Oklch::new(0.645, 0.246, 16.439),
    );

    let destructive_light = Oklch::new(0.577, 0.245, 27.325);
    let destructive_dark = Oklch::new(0.704, 0.191, 22.216);
    let dark_border = Oklch::with_alpha(1.0, 0.0, 0.0, 0.10);
    let dark_input = Oklch::with_alpha(1.0, 0.0, 0.0, 0.15);
    let white = Oklch::new(1.0, 0.0, 0.0);

    match base {
        ShadcnBaseColor::Neutral => {
            let light_fg = Oklch::new(0.145, 0.0, 0.0);
            let primary = Oklch::new(0.205, 0.0, 0.0);
            let primary_fg = Oklch::new(0.985, 0.0, 0.0);
            let secondary = Oklch::new(0.97, 0.0, 0.0);
            let muted_fg = Oklch::new(0.556, 0.0, 0.0);
            let border = Oklch::new(0.922, 0.0, 0.0);
            let ring = Oklch::new(0.708, 0.0, 0.0);
            let sidebar = Oklch::new(0.985, 0.0, 0.0);

            let light = ShadcnOklchPalette {
                background: white,
                foreground: light_fg,
                card: white,
                card_foreground: light_fg,
                popover: white,
                popover_foreground: light_fg,
                primary,
                primary_foreground: primary_fg,
                secondary,
                secondary_foreground: primary,
                muted: secondary,
                muted_foreground: muted_fg,
                accent: secondary,
                accent_foreground: primary,
                destructive: destructive_light,
                border,
                input: border,
                ring,
                chart_1: chart_light.0,
                chart_2: chart_light.1,
                chart_3: chart_light.2,
                chart_4: chart_light.3,
                chart_5: chart_light.4,
                sidebar,
                sidebar_foreground: light_fg,
                sidebar_primary: primary,
                sidebar_primary_foreground: sidebar,
                sidebar_accent: secondary,
                sidebar_accent_foreground: primary,
                sidebar_border: border,
                sidebar_ring: ring,
            };

            let dark_bg = light_fg;
            let dark_fg = primary_fg;
            let dark_card = primary;
            let dark_popover = Oklch::new(0.269, 0.0, 0.0);
            let dark_secondary = dark_popover;
            let dark_muted = dark_popover;
            let dark_muted_fg = ring;
            let dark_accent = Oklch::new(0.371, 0.0, 0.0);
            let dark_ring = Oklch::new(0.556, 0.0, 0.0);
            let dark_sidebar_ring = Oklch::new(0.439, 0.0, 0.0);

            let dark = ShadcnOklchPalette {
                background: dark_bg,
                foreground: dark_fg,
                card: dark_card,
                card_foreground: dark_fg,
                popover: dark_popover,
                popover_foreground: dark_fg,
                primary: border,
                primary_foreground: dark_card,
                secondary: dark_secondary,
                secondary_foreground: dark_fg,
                muted: dark_muted,
                muted_foreground: dark_muted_fg,
                accent: dark_accent,
                accent_foreground: dark_fg,
                destructive: destructive_dark,
                border: dark_border,
                input: dark_input,
                ring: dark_ring,
                chart_1: chart_dark.0,
                chart_2: chart_dark.1,
                chart_3: chart_dark.2,
                chart_4: chart_dark.3,
                chart_5: chart_dark.4,
                sidebar: dark_card,
                sidebar_foreground: dark_fg,
                sidebar_primary: chart_dark.0,
                sidebar_primary_foreground: dark_fg,
                sidebar_accent: dark_secondary,
                sidebar_accent_foreground: dark_fg,
                sidebar_border: dark_border,
                sidebar_ring: dark_sidebar_ring,
            };

            (light, dark)
        }

        ShadcnBaseColor::Stone => {
            let light_fg = Oklch::new(0.147, 0.004, 49.25);
            let primary = Oklch::new(0.216, 0.006, 56.043);
            let primary_fg = Oklch::new(0.985, 0.001, 106.423);
            let secondary = Oklch::new(0.97, 0.001, 106.424);
            let muted_fg = Oklch::new(0.553, 0.013, 58.071);
            let border = Oklch::new(0.923, 0.003, 48.717);
            let ring = Oklch::new(0.709, 0.01, 56.259);

            let light = ShadcnOklchPalette {
                background: white,
                foreground: light_fg,
                card: white,
                card_foreground: light_fg,
                popover: white,
                popover_foreground: light_fg,
                primary,
                primary_foreground: primary_fg,
                secondary,
                secondary_foreground: primary,
                muted: secondary,
                muted_foreground: muted_fg,
                accent: secondary,
                accent_foreground: primary,
                destructive: destructive_light,
                border,
                input: border,
                ring,
                chart_1: chart_light.0,
                chart_2: chart_light.1,
                chart_3: chart_light.2,
                chart_4: chart_light.3,
                chart_5: chart_light.4,
                sidebar: primary_fg,
                sidebar_foreground: light_fg,
                sidebar_primary: primary,
                sidebar_primary_foreground: primary_fg,
                sidebar_accent: secondary,
                sidebar_accent_foreground: primary,
                sidebar_border: border,
                sidebar_ring: ring,
            };

            let dark_secondary = Oklch::new(0.268, 0.007, 34.298);
            let dark = ShadcnOklchPalette {
                background: light_fg,
                foreground: primary_fg,
                card: primary,
                card_foreground: primary_fg,
                popover: primary,
                popover_foreground: primary_fg,
                primary: border,
                primary_foreground: primary,
                secondary: dark_secondary,
                secondary_foreground: primary_fg,
                muted: dark_secondary,
                muted_foreground: ring,
                accent: dark_secondary,
                accent_foreground: primary_fg,
                destructive: destructive_dark,
                border: dark_border,
                input: dark_input,
                ring: muted_fg,
                chart_1: chart_dark.0,
                chart_2: chart_dark.1,
                chart_3: chart_dark.2,
                chart_4: chart_dark.3,
                chart_5: chart_dark.4,
                sidebar: primary,
                sidebar_foreground: primary_fg,
                sidebar_primary: chart_dark.0,
                sidebar_primary_foreground: primary_fg,
                sidebar_accent: dark_secondary,
                sidebar_accent_foreground: primary_fg,
                sidebar_border: dark_border,
                sidebar_ring: muted_fg,
            };

            (light, dark)
        }

        ShadcnBaseColor::Zinc => {
            let light_fg = Oklch::new(0.141, 0.005, 285.823);
            let primary = Oklch::new(0.21, 0.006, 285.885);
            let primary_fg = Oklch::new(0.985, 0.0, 0.0);
            let secondary = Oklch::new(0.967, 0.001, 286.375);
            let muted_fg = Oklch::new(0.552, 0.016, 285.938);
            let border = Oklch::new(0.92, 0.004, 286.32);
            let ring = Oklch::new(0.705, 0.015, 286.067);

            let light = ShadcnOklchPalette {
                background: white,
                foreground: light_fg,
                card: white,
                card_foreground: light_fg,
                popover: white,
                popover_foreground: light_fg,
                primary,
                primary_foreground: primary_fg,
                secondary,
                secondary_foreground: primary,
                muted: secondary,
                muted_foreground: muted_fg,
                accent: secondary,
                accent_foreground: primary,
                destructive: destructive_light,
                border,
                input: border,
                ring,
                chart_1: chart_light.0,
                chart_2: chart_light.1,
                chart_3: chart_light.2,
                chart_4: chart_light.3,
                chart_5: chart_light.4,
                sidebar: primary_fg,
                sidebar_foreground: light_fg,
                sidebar_primary: primary,
                sidebar_primary_foreground: primary_fg,
                sidebar_accent: secondary,
                sidebar_accent_foreground: primary,
                sidebar_border: border,
                sidebar_ring: ring,
            };

            let dark_secondary = Oklch::new(0.274, 0.006, 286.033);
            let dark = ShadcnOklchPalette {
                background: light_fg,
                foreground: primary_fg,
                card: primary,
                card_foreground: primary_fg,
                popover: primary,
                popover_foreground: primary_fg,
                primary: border,
                primary_foreground: primary,
                secondary: dark_secondary,
                secondary_foreground: primary_fg,
                muted: dark_secondary,
                muted_foreground: ring,
                accent: dark_secondary,
                accent_foreground: primary_fg,
                destructive: destructive_dark,
                border: dark_border,
                input: dark_input,
                ring: muted_fg,
                chart_1: chart_dark.0,
                chart_2: chart_dark.1,
                chart_3: chart_dark.2,
                chart_4: chart_dark.3,
                chart_5: chart_dark.4,
                sidebar: primary,
                sidebar_foreground: primary_fg,
                sidebar_primary: chart_dark.0,
                sidebar_primary_foreground: primary_fg,
                sidebar_accent: dark_secondary,
                sidebar_accent_foreground: primary_fg,
                sidebar_border: dark_border,
                sidebar_ring: muted_fg,
            };

            (light, dark)
        }

        ShadcnBaseColor::Gray => {
            let light_fg = Oklch::new(0.13, 0.028, 261.692);
            let primary = Oklch::new(0.21, 0.034, 264.665);
            let primary_fg = Oklch::new(0.985, 0.002, 247.839);
            let secondary = Oklch::new(0.967, 0.003, 264.542);
            let muted_fg = Oklch::new(0.551, 0.027, 264.364);
            let border = Oklch::new(0.928, 0.006, 264.531);
            let ring = Oklch::new(0.707, 0.022, 261.325);

            let light = ShadcnOklchPalette {
                background: white,
                foreground: light_fg,
                card: white,
                card_foreground: light_fg,
                popover: white,
                popover_foreground: light_fg,
                primary,
                primary_foreground: primary_fg,
                secondary,
                secondary_foreground: primary,
                muted: secondary,
                muted_foreground: muted_fg,
                accent: secondary,
                accent_foreground: primary,
                destructive: destructive_light,
                border,
                input: border,
                ring,
                chart_1: chart_light.0,
                chart_2: chart_light.1,
                chart_3: chart_light.2,
                chart_4: chart_light.3,
                chart_5: chart_light.4,
                sidebar: primary_fg,
                sidebar_foreground: light_fg,
                sidebar_primary: primary,
                sidebar_primary_foreground: primary_fg,
                sidebar_accent: secondary,
                sidebar_accent_foreground: primary,
                sidebar_border: border,
                sidebar_ring: ring,
            };

            let dark_secondary = Oklch::new(0.278, 0.033, 256.848);
            let dark = ShadcnOklchPalette {
                background: light_fg,
                foreground: primary_fg,
                card: primary,
                card_foreground: primary_fg,
                popover: primary,
                popover_foreground: primary_fg,
                primary: border,
                primary_foreground: primary,
                secondary: dark_secondary,
                secondary_foreground: primary_fg,
                muted: dark_secondary,
                muted_foreground: ring,
                accent: dark_secondary,
                accent_foreground: primary_fg,
                destructive: destructive_dark,
                border: dark_border,
                input: dark_input,
                ring: muted_fg,
                chart_1: chart_dark.0,
                chart_2: chart_dark.1,
                chart_3: chart_dark.2,
                chart_4: chart_dark.3,
                chart_5: chart_dark.4,
                sidebar: primary,
                sidebar_foreground: primary_fg,
                sidebar_primary: chart_dark.0,
                sidebar_primary_foreground: primary_fg,
                sidebar_accent: dark_secondary,
                sidebar_accent_foreground: primary_fg,
                sidebar_border: dark_border,
                sidebar_ring: muted_fg,
            };

            (light, dark)
        }

        ShadcnBaseColor::Slate => {
            let light_fg = Oklch::new(0.129, 0.042, 264.695);
            let primary = Oklch::new(0.208, 0.042, 265.755);
            let primary_fg = Oklch::new(0.984, 0.003, 247.858);
            let secondary = Oklch::new(0.968, 0.007, 247.896);
            let muted_fg = Oklch::new(0.554, 0.046, 257.417);
            let border = Oklch::new(0.929, 0.013, 255.508);
            let ring = Oklch::new(0.704, 0.04, 256.788);

            let light = ShadcnOklchPalette {
                background: white,
                foreground: light_fg,
                card: white,
                card_foreground: light_fg,
                popover: white,
                popover_foreground: light_fg,
                primary,
                primary_foreground: primary_fg,
                secondary,
                secondary_foreground: primary,
                muted: secondary,
                muted_foreground: muted_fg,
                accent: secondary,
                accent_foreground: primary,
                destructive: destructive_light,
                border,
                input: border,
                ring,
                chart_1: chart_light.0,
                chart_2: chart_light.1,
                chart_3: chart_light.2,
                chart_4: chart_light.3,
                chart_5: chart_light.4,
                sidebar: primary_fg,
                sidebar_foreground: light_fg,
                sidebar_primary: primary,
                sidebar_primary_foreground: primary_fg,
                sidebar_accent: secondary,
                sidebar_accent_foreground: primary,
                sidebar_border: border,
                sidebar_ring: ring,
            };

            let dark_secondary = Oklch::new(0.279, 0.041, 260.031);
            let dark_ring = Oklch::new(0.551, 0.027, 264.364);
            let dark = ShadcnOklchPalette {
                background: light_fg,
                foreground: primary_fg,
                card: primary,
                card_foreground: primary_fg,
                popover: primary,
                popover_foreground: primary_fg,
                primary: border,
                primary_foreground: primary,
                secondary: dark_secondary,
                secondary_foreground: primary_fg,
                muted: dark_secondary,
                muted_foreground: ring,
                accent: dark_secondary,
                accent_foreground: primary_fg,
                destructive: destructive_dark,
                border: dark_border,
                input: dark_input,
                ring: dark_ring,
                chart_1: chart_dark.0,
                chart_2: chart_dark.1,
                chart_3: chart_dark.2,
                chart_4: chart_dark.3,
                chart_5: chart_dark.4,
                sidebar: primary,
                sidebar_foreground: primary_fg,
                sidebar_primary: chart_dark.0,
                sidebar_primary_foreground: primary_fg,
                sidebar_accent: dark_secondary,
                sidebar_accent_foreground: primary_fg,
                sidebar_border: dark_border,
                sidebar_ring: dark_ring,
            };

            (light, dark)
        }
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
            sm: 6.0,
            md: 8.0,
            lg: 12.0,
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
