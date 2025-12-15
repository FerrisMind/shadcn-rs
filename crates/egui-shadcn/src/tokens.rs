use egui::{Color32, CornerRadius, FontId, Stroke, Vec2};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MotionTokens {
    pub fast_ms: f32,
    pub base_ms: f32,
    pub slow_ms: f32,
    pub easing: &'static str,
}

impl Default for MotionTokens {
    fn default() -> Self {
        Self {
            fast_ms: 150.0,
            base_ms: 200.0,
            slow_ms: 250.0,
            easing: "cubic-bezier(0.16, 1, 0.3, 1)",
        }
    }
}

pub fn ease_out_cubic(t: f32) -> f32 {
    let clamped = t.clamp(0.0, 1.0);
    let inv = 1.0 - clamped;
    1.0 - inv * inv * inv
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RadiusScale {
    pub r1: f32,
    pub r2: f32,
    pub r3: f32,
    pub r4: f32,
    pub r5: f32,
    pub r6: f32,
}

impl RadiusScale {
    pub fn step(&self, index: u8) -> f32 {
        match index {
            1 => self.r1,
            2 => self.r2,
            3 => self.r3,
            4 => self.r4,
            5 => self.r5,
            _ => self.r6,
        }
    }
}

impl Default for RadiusScale {
    fn default() -> Self {
        Self {
            r1: 4.0,
            r2: 6.0,
            r3: 8.0,
            r4: 10.0,
            r5: 12.0,
            r6: 16.0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FocusTokens {
    pub ring_width: f32,
}

impl FocusTokens {
    pub fn stroke(&self, color: Color32) -> Stroke {
        Stroke::new(self.ring_width, color)
    }
}

impl Default for FocusTokens {
    fn default() -> Self {
        Self { ring_width: 3.0 }
    }
}

pub const DEFAULT_MOTION: MotionTokens = MotionTokens {
    fast_ms: 150.0,
    base_ms: 200.0,
    slow_ms: 250.0,
    easing: "cubic-bezier(0.16, 1, 0.3, 1)",
};

pub const DEFAULT_RADIUS: RadiusScale = RadiusScale {
    r1: 4.0,
    r2: 6.0,
    r3: 8.0,
    r4: 10.0,
    r5: 12.0,
    r6: 16.0,
};

pub const DEFAULT_FOCUS: FocusTokens = FocusTokens { ring_width: 3.0 };

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ControlVariant {
    Primary,
    Secondary,
    Ghost,
    Outline,
    Destructive,
    Link,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ControlSize {
    Sm,
    Md,
    Lg,
    IconSm,
    Icon,
    IconLg,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwitchSize {
    One,
    Two,
    Three,
}

impl From<ControlSize> for SwitchSize {
    fn from(size: ControlSize) -> Self {
        match size {
            ControlSize::Sm => SwitchSize::One,
            ControlSize::Md | ControlSize::IconSm => SwitchSize::Two,
            ControlSize::Lg | ControlSize::Icon | ControlSize::IconLg => SwitchSize::Three,
        }
    }
}

impl From<SwitchSize> for ControlSize {
    fn from(size: SwitchSize) -> Self {
        match size {
            SwitchSize::One => ControlSize::Sm,
            SwitchSize::Two => ControlSize::Md,
            SwitchSize::Three => ControlSize::Lg,
        }
    }
}

impl ControlSize {
    pub fn padding(self) -> Vec2 {
        match self {
            ControlSize::Sm => Vec2::new(12.0, 8.0),
            ControlSize::Md => Vec2::new(14.0, 9.0),
            ControlSize::Lg => Vec2::new(16.0, 10.0),
            ControlSize::IconSm => Vec2::new(8.0, 8.0),
            ControlSize::Icon => Vec2::new(9.0, 9.0),
            ControlSize::IconLg => Vec2::new(10.0, 10.0),
        }
    }

    pub fn rounding(self) -> CornerRadius {
        self.rounding_with_scale(&DEFAULT_RADIUS)
    }

    pub fn rounding_with_scale(self, scale: &RadiusScale) -> CornerRadius {
        let radius = match self {
            ControlSize::Sm => scale.r2,
            ControlSize::Md => scale.r3,
            ControlSize::Lg => scale.r4,
            ControlSize::IconSm => scale.r3,
            ControlSize::Icon => (scale.r3 + scale.r4) * 0.5,
            ControlSize::IconLg => scale.r4,
        };
        let radius_clamped = radius.round().clamp(0.0, u8::MAX as f32) as u8;
        CornerRadius::same(radius_clamped)
    }

    pub fn expansion(self) -> f32 {
        match self {
            ControlSize::Sm => 1.0,
            ControlSize::Md => 1.25,
            ControlSize::Lg => 1.5,
            ControlSize::IconSm => 1.0,
            ControlSize::Icon => 1.0,
            ControlSize::IconLg => 1.0,
        }
    }

    pub fn font(self) -> FontId {
        FontId::proportional(match self {
            ControlSize::Sm => 13.0,
            ControlSize::Md => 14.0,
            ControlSize::Lg => 15.0,
            ControlSize::IconSm => 13.0,
            ControlSize::Icon => 14.0,
            ControlSize::IconLg => 15.0,
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ColorPalette {
    pub background: Color32,
    pub foreground: Color32,
    pub card: Color32,
    pub card_foreground: Color32,
    pub popover: Color32,
    pub popover_foreground: Color32,
    pub border: Color32,
    pub input: Color32,
    pub ring: Color32,
    pub primary: Color32,
    pub primary_foreground: Color32,
    pub secondary: Color32,
    pub secondary_foreground: Color32,
    pub accent: Color32,
    pub accent_foreground: Color32,
    pub muted: Color32,
    pub muted_foreground: Color32,
    pub destructive: Color32,
    pub destructive_foreground: Color32,

    pub chart_1: Color32,
    pub chart_2: Color32,
    pub chart_3: Color32,
    pub chart_4: Color32,
    pub chart_5: Color32,

    pub sidebar: Color32,
    pub sidebar_foreground: Color32,
    pub sidebar_primary: Color32,
    pub sidebar_primary_foreground: Color32,
    pub sidebar_accent: Color32,
    pub sidebar_accent_foreground: Color32,
    pub sidebar_border: Color32,
    pub sidebar_ring: Color32,
}

impl Default for ColorPalette {
    fn default() -> Self {
        Self::dark()
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
    fn to_color32(self) -> Color32 {

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

        fn to_srgb_u8(linear: f32) -> u8 {
            let clamped = linear.clamp(0.0, 1.0);
            let srgb = if clamped <= 0.003_130_8 {
                12.92 * clamped
            } else {
                1.055 * clamped.powf(1.0 / 2.4) - 0.055
            };
            (srgb.clamp(0.0, 1.0) * 255.0).round() as u8
        }

        let r = to_srgb_u8(r_lin);
        let g = to_srgb_u8(g_lin);
        let b = to_srgb_u8(b_lin);
        let a = (self.alpha.clamp(0.0, 1.0) * 255.0).round() as u8;

        Color32::from_rgba_unmultiplied(r, g, b, a)
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
    fn to_color_palette(self) -> ColorPalette {
        ColorPalette {
            background: self.background.to_color32(),
            foreground: self.foreground.to_color32(),
            card: self.card.to_color32(),
            card_foreground: self.card_foreground.to_color32(),
            popover: self.popover.to_color32(),
            popover_foreground: self.popover_foreground.to_color32(),
            border: self.border.to_color32(),
            input: self.input.to_color32(),
            ring: self.ring.to_color32(),
            primary: self.primary.to_color32(),
            primary_foreground: self.primary_foreground.to_color32(),
            secondary: self.secondary.to_color32(),
            secondary_foreground: self.secondary_foreground.to_color32(),
            accent: self.accent.to_color32(),
            accent_foreground: self.accent_foreground.to_color32(),
            muted: self.muted.to_color32(),
            muted_foreground: self.muted_foreground.to_color32(),
            destructive: self.destructive.to_color32(),

            destructive_foreground: Oklch::new(0.985, 0.0, 0.0).to_color32(),
            chart_1: self.chart_1.to_color32(),
            chart_2: self.chart_2.to_color32(),
            chart_3: self.chart_3.to_color32(),
            chart_4: self.chart_4.to_color32(),
            chart_5: self.chart_5.to_color32(),
            sidebar: self.sidebar.to_color32(),
            sidebar_foreground: self.sidebar_foreground.to_color32(),
            sidebar_primary: self.sidebar_primary.to_color32(),
            sidebar_primary_foreground: self.sidebar_primary_foreground.to_color32(),
            sidebar_accent: self.sidebar_accent.to_color32(),
            sidebar_accent_foreground: self.sidebar_accent_foreground.to_color32(),
            sidebar_border: self.sidebar_border.to_color32(),
            sidebar_ring: self.sidebar_ring.to_color32(),
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

impl ColorPalette {
    pub fn dark() -> Self {
        Self::shadcn_dark(ShadcnBaseColor::Neutral)
    }

    pub fn light() -> Self {
        Self::shadcn_light(ShadcnBaseColor::Neutral)
    }

    pub fn shadcn_light(base: ShadcnBaseColor) -> Self {
        let (light, _dark) = shadcn_oklch_palettes(base);
        light.to_color_palette()
    }

    pub fn shadcn_dark(base: ShadcnBaseColor) -> Self {
        let (_light, dark) = shadcn_oklch_palettes(base);
        dark.to_color_palette()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwitchVariant {
    Surface,
    Classic,
    Soft,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InputVariant {
    Surface,
    Classic,
    Soft,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StateColors {
    pub bg_fill: Color32,
    pub fg_stroke: Stroke,
    pub border: Stroke,
}

impl StateColors {
    pub fn new(bg_fill: Color32, fg: Color32, border: Color32) -> Self {
        Self {
            bg_fill,
            fg_stroke: Stroke::new(1.0, fg),
            border: Stroke::new(1.0, border),
        }
    }

    pub fn with_border(bg_fill: Color32, fg: Color32, border: Stroke) -> Self {
        Self {
            bg_fill,
            fg_stroke: Stroke::new(1.0, fg),
            border,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VariantTokens {
    pub idle: StateColors,
    pub hovered: StateColors,
    pub active: StateColors,
    pub disabled: StateColors,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InputTokens {
    pub idle: StateColors,
    pub hovered: StateColors,
    pub focused: StateColors,
    pub disabled: StateColors,
    pub invalid: StateColors,

    pub selection_bg: Color32,

    pub selection_fg: Color32,

    pub placeholder: Color32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ToggleState {
    pub idle: StateColors,
    pub hovered: StateColors,
    pub active: StateColors,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ToggleTokens {
    pub off: ToggleState,
    pub on: ToggleState,
    pub disabled: StateColors,
    pub thumb_on: Color32,
    pub thumb_off: Color32,
    pub thumb_disabled: Color32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SwitchTokens {
    pub toggle: ToggleTokens,
    pub focus_ring: Stroke,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SwitchTokenOptions {
    pub variant: SwitchVariant,
    pub high_contrast: bool,
    pub accent: Color32,
    pub thumb_color: Option<Color32>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ToggleMetrics {
    pub track_size: Vec2,
    pub thumb_size: Vec2,
}

pub fn mix(color: Color32, other: Color32, t: f32) -> Color32 {
    let t_clamped = t.clamp(0.0, 1.0);
    let [r1, g1, b1, a1] = color.to_array();
    let [r2, g2, b2, a2] = other.to_array();
    let lerp = |c1: u8, c2: u8| -> u8 {
        ((c1 as f32 * (1.0 - t_clamped)) + (c2 as f32 * t_clamped)).round() as u8
    };
    Color32::from_rgba_unmultiplied(lerp(r1, r2), lerp(g1, g2), lerp(b1, b2), lerp(a1, a2))
}

fn disabled_state(palette: &ColorPalette) -> StateColors {
    StateColors::new(
        mix(palette.muted, palette.background, 0.6),
        mix(palette.muted_foreground, palette.foreground, 0.5),
        mix(palette.border, palette.muted_foreground, 0.5),
    )
}

pub fn variant_tokens(palette: &ColorPalette, variant: ControlVariant) -> VariantTokens {
    let disabled = disabled_state(palette);
    match variant {
        ControlVariant::Primary => VariantTokens {
            idle: StateColors::new(palette.primary, palette.primary_foreground, palette.border),
            hovered: StateColors::new(
                mix(palette.primary, Color32::WHITE, 0.06),
                palette.primary_foreground,
                mix(palette.border, Color32::WHITE, 0.08),
            ),
            active: StateColors::new(
                mix(palette.primary, Color32::WHITE, 0.1),
                palette.primary_foreground,
                mix(palette.border, Color32::WHITE, 0.12),
            ),
            disabled,
        },
        ControlVariant::Destructive => VariantTokens {
            idle: StateColors::new(
                palette.destructive,
                palette.destructive_foreground,
                palette.border,
            ),
            hovered: StateColors::new(
                mix(palette.destructive, Color32::WHITE, 0.08),
                palette.destructive_foreground,
                mix(palette.border, Color32::WHITE, 0.12),
            ),
            active: StateColors::new(
                mix(palette.destructive, Color32::from_rgb(127, 29, 29), 0.12),
                palette.destructive_foreground,
                mix(palette.border, Color32::from_rgb(127, 29, 29), 0.2),
            ),
            disabled,
        },
        ControlVariant::Secondary => VariantTokens {
            idle: StateColors::new(
                palette.secondary,
                palette.secondary_foreground,
                palette.border,
            ),
            hovered: StateColors::new(
                mix(palette.secondary, Color32::WHITE, 0.08),
                palette.secondary_foreground,
                mix(palette.border, Color32::WHITE, 0.1),
            ),
            active: StateColors::new(
                mix(palette.secondary, palette.border, 0.3),
                palette.secondary_foreground,
                mix(palette.border, palette.foreground, 0.15),
            ),
            disabled,
        },
        ControlVariant::Link => VariantTokens {
            idle: StateColors::new(Color32::TRANSPARENT, palette.primary, palette.border),
            hovered: StateColors::new(
                mix(palette.muted, palette.background, 0.5),
                palette.primary,
                palette.border,
            ),
            active: StateColors::new(
                mix(palette.muted, palette.primary, 0.4),
                palette.primary,
                palette.border,
            ),
            disabled,
        },
        ControlVariant::Ghost => VariantTokens {
            idle: StateColors::new(Color32::TRANSPARENT, palette.foreground, palette.border),
            hovered: StateColors::new(
                mix(palette.muted, palette.background, 0.5),
                palette.foreground,
                mix(palette.border, palette.foreground, 0.05),
            ),
            active: StateColors::new(
                mix(palette.muted, palette.border, 0.4),
                palette.foreground,
                palette.border,
            ),
            disabled,
        },
        ControlVariant::Outline => VariantTokens {
            idle: StateColors::new(Color32::TRANSPARENT, palette.foreground, palette.border),
            hovered: StateColors::new(
                mix(palette.background, palette.border, 0.25),
                palette.foreground,
                mix(palette.border, palette.foreground, 0.1),
            ),
            active: StateColors::new(
                mix(palette.background, palette.primary, 0.1),
                palette.foreground,
                mix(palette.primary, palette.border, 0.2),
            ),
            disabled,
        },
    }
}

pub fn input_tokens(palette: &ColorPalette, variant: InputVariant) -> InputTokens {
    let disabled = disabled_state(palette);
    let focus = FocusTokens::default();

    match variant {
        InputVariant::Surface => {
            let base_bg = Color32::TRANSPARENT;
            let border = palette.input;
            let focus_color = Color32::from_rgba_unmultiplied(
                palette.ring.r(),
                palette.ring.g(),
                palette.ring.b(),
                128,
            );
            InputTokens {
                idle: StateColors::with_border(
                    base_bg,
                    palette.foreground,
                    Stroke::new(1.0, border),
                ),
                hovered: StateColors::with_border(
                    base_bg,
                    palette.foreground,
                    Stroke::new(1.0, border),
                ),
                focused: StateColors::with_border(
                    base_bg,
                    palette.foreground,
                    focus.stroke(focus_color),
                ),
                disabled,
                invalid: StateColors::with_border(
                    base_bg,
                    palette.foreground,
                    Stroke::new(1.0, palette.destructive),
                ),
                selection_bg: palette.primary,
                selection_fg: palette.primary_foreground,
                placeholder: palette.muted_foreground,
            }
        }
        InputVariant::Classic => {
            let base_bg = palette.background;
            let border = palette.input;
            let focus_color = Color32::from_rgba_unmultiplied(
                palette.ring.r(),
                palette.ring.g(),
                palette.ring.b(),
                128,
            );
            InputTokens {
                idle: StateColors::with_border(
                    base_bg,
                    palette.foreground,
                    Stroke::new(1.0, border),
                ),
                hovered: StateColors::with_border(
                    base_bg,
                    palette.foreground,
                    Stroke::new(1.0, border),
                ),
                focused: StateColors::with_border(
                    base_bg,
                    palette.foreground,
                    focus.stroke(focus_color),
                ),
                disabled,
                invalid: StateColors::with_border(
                    base_bg,
                    palette.foreground,
                    Stroke::new(1.0, palette.destructive),
                ),
                selection_bg: palette.primary,
                selection_fg: palette.primary_foreground,
                placeholder: palette.muted_foreground,
            }
        }
        InputVariant::Soft => {
            let base_bg = mix(palette.accent, palette.background, 0.4);
            let focus_color = Color32::from_rgba_unmultiplied(
                palette.ring.r(),
                palette.ring.g(),
                palette.ring.b(),
                128,
            );
            InputTokens {
                idle: StateColors::with_border(
                    base_bg,
                    palette.accent_foreground,
                    Stroke::new(0.0, Color32::TRANSPARENT),
                ),
                hovered: StateColors::with_border(
                    mix(base_bg, Color32::WHITE, 0.12),
                    palette.accent_foreground,
                    Stroke::new(0.0, Color32::TRANSPARENT),
                ),
                focused: StateColors::with_border(
                    mix(base_bg, focus_color, 0.2),
                    palette.accent_foreground,
                    focus.stroke(focus_color),
                ),
                disabled,
                invalid: StateColors::with_border(
                    mix(base_bg, palette.destructive, 0.2),
                    palette.accent_foreground,
                    Stroke::new(1.0, palette.destructive),
                ),
                selection_bg: mix(palette.accent, Color32::WHITE, 0.12),
                selection_fg: palette.accent_foreground,
                placeholder: mix(palette.accent_foreground, palette.background, 0.45),
            }
        }
    }
}

pub fn checkbox_tokens(palette: &ColorPalette, variant: ControlVariant) -> ToggleTokens {
    checkbox_tokens_with_high_contrast(palette, variant, false)
}

pub fn checkbox_tokens_with_high_contrast(
    palette: &ColorPalette,
    variant: ControlVariant,
    high_contrast: bool,
) -> ToggleTokens {
    let fg_off = mix(palette.foreground, palette.muted_foreground, 0.4);
    let off_idle =
        StateColors::with_border(palette.input, fg_off, Stroke::new(1.0, palette.border));
    let off_hovered = StateColors::with_border(
        mix(palette.input, Color32::WHITE, 0.04),
        fg_off,
        Stroke::new(1.0, mix(palette.border, palette.foreground, 0.1)),
    );
    let off_active = StateColors::with_border(
        mix(palette.input, Color32::WHITE, 0.08),
        fg_off,
        Stroke::new(1.5, mix(palette.border, palette.primary, 0.25)),
    );

    let on = variant_tokens(palette, variant);
    let disabled = disabled_state(palette);
    let thumb_disabled = mix(palette.muted_foreground, palette.background, 0.5);
    let mut tokens = ToggleTokens {
        off: ToggleState {
            idle: off_idle,
            hovered: off_hovered,
            active: off_active,
        },
        on: ToggleState {
            idle: on.idle,
            hovered: on.hovered,
            active: on.active,
        },
        disabled,
        thumb_on: on.idle.fg_stroke.color,
        thumb_off: fg_off,
        thumb_disabled,
    };

    if high_contrast {
        tokens.on.idle.bg_fill = mix(tokens.on.idle.bg_fill, Color32::WHITE, 0.2);
        tokens.on.hovered.bg_fill = mix(tokens.on.hovered.bg_fill, Color32::WHITE, 0.25);
        tokens.on.active.bg_fill = mix(tokens.on.active.bg_fill, Color32::WHITE, 0.3);

        tokens.off.idle.bg_fill = mix(tokens.off.idle.bg_fill, Color32::WHITE, 0.25);
        tokens.off.hovered.bg_fill = mix(tokens.off.hovered.bg_fill, Color32::WHITE, 0.3);
        tokens.off.active.bg_fill = mix(tokens.off.active.bg_fill, Color32::WHITE, 0.35);

        tokens.disabled.bg_fill = mix(tokens.disabled.bg_fill, palette.background, 0.35);
        tokens.thumb_on = mix(tokens.thumb_on, palette.background, 0.12);
        tokens.thumb_off = mix(tokens.thumb_off, palette.background, 0.12);
    }

    tokens
}

pub fn switch_tokens(palette: &ColorPalette, variant: ControlVariant) -> SwitchTokens {
    let accent = match variant {
        ControlVariant::Primary | ControlVariant::Link => palette.primary,
        ControlVariant::Destructive => palette.destructive,
        ControlVariant::Secondary => palette.secondary,
        ControlVariant::Ghost | ControlVariant::Outline => palette.accent,
    };
    switch_tokens_with_options(
        palette,
        SwitchTokenOptions {
            variant: SwitchVariant::Surface,
            high_contrast: false,
            accent,
            thumb_color: None,
        },
    )
}

pub fn switch_tokens_with_options(
    palette: &ColorPalette,
    options: SwitchTokenOptions,
) -> SwitchTokens {
    let disabled = disabled_state(palette);

    let accent_luminance = (options.accent.r() as f32 * 0.299
        + options.accent.g() as f32 * 0.587
        + options.accent.b() as f32 * 0.114)
        / 255.0;
    let accent_is_light = accent_luminance > 0.5;

    let thumb_base = options.thumb_color.unwrap_or_else(|| {
        mix(
            palette.background,
            mix(palette.foreground, Color32::WHITE, 0.15),
            0.9,
        )
    });

    let thumb_on = if let Some(custom) = options.thumb_color {
        custom
    } else if options.high_contrast {
        mix(options.accent, palette.foreground, 0.65)
    } else if accent_is_light {
        mix(palette.primary_foreground, palette.background, 0.15)
    } else {
        thumb_base
    };

    let thumb_off = if options.high_contrast {
        mix(thumb_base, palette.background, 0.25)
    } else {
        thumb_base
    };

    let off_fg = mix(palette.foreground, palette.muted_foreground, 0.35);
    let (off_idle, off_hovered, off_active) = match options.variant {
        SwitchVariant::Soft => {
            let base = mix(
                palette.background,
                palette.accent,
                if options.high_contrast { 0.35 } else { 0.25 },
            );
            let border = Stroke::new(1.0, mix(palette.accent, palette.foreground, 0.18));
            let idle = StateColors::with_border(base, off_fg, border);
            let hovered = StateColors::with_border(
                mix(base, Color32::WHITE, 0.08),
                off_fg,
                Stroke::new(1.0, mix(border.color, palette.foreground, 0.15)),
            );
            let active = StateColors::with_border(
                mix(base, options.accent, 0.2),
                off_fg,
                Stroke::new(1.2, mix(border.color, options.accent, 0.25)),
            );
            (idle, hovered, active)
        }
        _ => {
            let off_bg = palette.input;
            let off_border = Stroke::new(1.0, mix(palette.border, palette.input, 0.4));
            let idle = StateColors::with_border(off_bg, off_fg, off_border);
            let hovered = StateColors::with_border(
                mix(off_bg, Color32::WHITE, 0.06),
                off_fg,
                Stroke::new(1.0, mix(off_border.color, palette.foreground, 0.1)),
            );
            let active = StateColors::with_border(
                mix(off_bg, Color32::WHITE, 0.12),
                off_fg,
                Stroke::new(1.2, mix(off_border.color, options.accent, 0.2)),
            );
            (idle, hovered, active)
        }
    };

    let (on_idle_bg, on_hover_bg, on_active_bg, on_border) = match options.variant {
        SwitchVariant::Surface => (
            options.accent,
            mix(options.accent, Color32::WHITE, 0.08),
            mix(options.accent, Color32::WHITE, 0.14),
            Stroke::new(1.0, mix(options.accent, palette.foreground, 0.08)),
        ),
        SwitchVariant::Classic => (
            mix(options.accent, palette.background, 0.06),
            mix(options.accent, Color32::WHITE, 0.12),
            mix(options.accent, Color32::WHITE, 0.2),
            Stroke::new(1.0, mix(options.accent, palette.foreground, 0.18)),
        ),
        SwitchVariant::Soft => (
            mix(options.accent, palette.background, 0.45),
            mix(options.accent, Color32::WHITE, 0.16),
            mix(options.accent, Color32::WHITE, 0.22),
            Stroke::new(1.0, mix(options.accent, palette.foreground, 0.12)),
        ),
    };

    let on_idle = StateColors::with_border(on_idle_bg, palette.foreground, on_border);
    let on_hovered = StateColors::with_border(
        on_hover_bg,
        palette.foreground,
        Stroke::new(on_border.width, on_border.color),
    );
    let on_active = StateColors::with_border(
        on_active_bg,
        palette.foreground,
        Stroke::new(on_border.width * 1.05, on_border.color),
    );

    let thumb_disabled = mix(palette.muted_foreground, palette.background, 0.5);

    let toggle = ToggleTokens {
        off: ToggleState {
            idle: off_idle,
            hovered: off_hovered,
            active: off_active,
        },
        on: ToggleState {
            idle: on_idle,
            hovered: on_hovered,
            active: on_active,
        },
        disabled,
        thumb_on,
        thumb_off,
        thumb_disabled,
    };

    let focus_color = if options.high_contrast {
        mix(options.accent, palette.foreground, 0.35)
    } else {
        mix(options.accent, palette.foreground, 0.15)
    };
    let focus_ring = DEFAULT_FOCUS.stroke(focus_color);

    SwitchTokens { toggle, focus_ring }
}

pub fn checkbox_metrics(size: ControlSize) -> ToggleMetrics {
    let track = match size {
        ControlSize::Sm | ControlSize::IconSm => 16.0,
        ControlSize::Md => 16.0,
        ControlSize::Lg | ControlSize::Icon | ControlSize::IconLg => 18.0,
    };
    ToggleMetrics {
        track_size: Vec2::splat(track),
        thumb_size: Vec2::splat(track * 0.6),
    }
}

pub fn toggle_metrics(size: ControlSize) -> ToggleMetrics {
    let (track_w, track_h): (f32, f32) = match size {
        ControlSize::Sm => (38.0, 20.0),
        ControlSize::Md | ControlSize::IconSm => (44.0, 24.0),
        ControlSize::Lg | ControlSize::Icon => (50.0, 28.0),
        ControlSize::IconLg => (54.0, 30.0),
    };
    let thumb = (track_h - 6.0_f32).max(12.0_f32);
    ToggleMetrics {
        track_size: Vec2::new(track_w, track_h),
        thumb_size: Vec2::splat(thumb),
    }
}

pub fn switch_metrics(size: SwitchSize) -> ToggleMetrics {
    let (height, thumb_inset): (f32, f32) = match size {
        SwitchSize::One => (16.0, 1.0),
        SwitchSize::Two => (20.0, 1.0),
        SwitchSize::Three => (24.0, 1.0),
    };
    let track_w = height * 1.75;
    let thumb = (height - thumb_inset * 2.0).max(12.0);
    ToggleMetrics {
        track_size: Vec2::new(track_w, height),
        thumb_size: Vec2::splat(thumb),
    }
}

pub fn switch_metrics_for_control_size(size: ControlSize) -> ToggleMetrics {
    switch_metrics(SwitchSize::from(size))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToggleVariant {
    Default,
    Outline,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ToggleButtonTokens {
    pub off: VariantTokens,
    pub on: VariantTokens,
}

fn accent_tokens(palette: &ColorPalette) -> VariantTokens {
    let disabled = disabled_state(palette);
    let border = Stroke::new(1.0, palette.accent);
    VariantTokens {
        idle: StateColors::with_border(palette.accent, palette.accent_foreground, border),
        hovered: StateColors::with_border(
            mix(palette.accent, Color32::WHITE, 0.08),
            palette.accent_foreground,
            Stroke::new(border.width, mix(palette.accent, Color32::WHITE, 0.1)),
        ),
        active: StateColors::with_border(
            mix(palette.accent, palette.foreground, 0.14),
            palette.accent_foreground,
            Stroke::new(border.width, mix(palette.accent, palette.foreground, 0.14)),
        ),
        disabled,
    }
}

pub fn toggle_button_tokens(palette: &ColorPalette, variant: ToggleVariant) -> ToggleButtonTokens {
    let disabled = disabled_state(palette);
    let off = match variant {
        ToggleVariant::Default => VariantTokens {
            idle: StateColors::with_border(
                Color32::TRANSPARENT,
                palette.foreground,
                Stroke::new(0.0, Color32::TRANSPARENT),
            ),
            hovered: StateColors::with_border(
                palette.muted,
                palette.muted_foreground,
                Stroke::new(0.0, Color32::TRANSPARENT),
            ),
            active: StateColors::with_border(
                mix(palette.muted, palette.foreground, 0.12),
                palette.muted_foreground,
                Stroke::new(0.0, Color32::TRANSPARENT),
            ),
            disabled,
        },
        ToggleVariant::Outline => {
            let border = Stroke::new(1.0, palette.border);
            VariantTokens {
                idle: StateColors::with_border(Color32::TRANSPARENT, palette.foreground, border),
                hovered: StateColors::with_border(
                    palette.accent,
                    palette.accent_foreground,
                    Stroke::new(border.width, palette.accent),
                ),
                active: StateColors::with_border(
                    mix(palette.accent, palette.foreground, 0.12),
                    palette.accent_foreground,
                    Stroke::new(border.width, mix(palette.accent, palette.foreground, 0.12)),
                ),
                disabled,
            }
        }
    };
    let on = accent_tokens(palette);
    ToggleButtonTokens { off, on }
}
