use iced::Color;

#[derive(Clone, Copy, Debug)]
pub struct Palette {
    pub background: Color,
    pub foreground: Color,
    pub primary: Color,
    pub primary_foreground: Color,
    pub secondary: Color,
    pub secondary_foreground: Color,
    pub destructive: Color,
    pub destructive_foreground: Color,
    pub border: Color,
    pub input: Color,
    pub muted: Color,
    pub muted_foreground: Color,
    pub ring: Color,
}

impl Palette {
    pub fn light() -> Self {
        Self {
            background: Color::from_rgb8(0xFF, 0xFF, 0xFF),
            foreground: Color::from_rgb8(0x0F, 0x17, 0x2A),
            primary: Color::from_rgb8(0x0F, 0x17, 0x2A),
            primary_foreground: Color::from_rgb8(0xF8, 0xFA, 0xFC),
            secondary: Color::from_rgb8(0xF1, 0xF5, 0xF9),
            secondary_foreground: Color::from_rgb8(0x0F, 0x17, 0x2A),
            destructive: Color::from_rgb8(0xEF, 0x44, 0x44),
            destructive_foreground: Color::from_rgb8(0xF8, 0xFA, 0xFC),
            border: Color::from_rgb8(0xE2, 0xE8, 0xF0),
            input: Color::from_rgb8(0xE2, 0xE8, 0xF0),
            muted: Color::from_rgb8(0xF1, 0xF5, 0xF9),
            muted_foreground: Color::from_rgb8(0x64, 0x74, 0x8B),
            ring: Color::from_rgb8(0x94, 0xA3, 0xB8),
        }
    }

    pub fn dark() -> Self {
        Self {
            background: Color::from_rgb8(0x0F, 0x17, 0x2A),
            foreground: Color::from_rgb8(0xF8, 0xFA, 0xFC),
            primary: Color::from_rgb8(0xF8, 0xFA, 0xFC),
            primary_foreground: Color::from_rgb8(0x0F, 0x17, 0x2A),
            secondary: Color::from_rgb8(0x1E, 0x29, 0x3B),
            secondary_foreground: Color::from_rgb8(0xF8, 0xFA, 0xFC),
            destructive: Color::from_rgb8(0x7F, 0x1D, 0x1D),
            destructive_foreground: Color::from_rgb8(0xF8, 0xFA, 0xFC),
            border: Color::from_rgb8(0x1E, 0x29, 0x3B),
            input: Color::from_rgb8(0x1E, 0x29, 0x3B),
            muted: Color::from_rgb8(0x1E, 0x29, 0x3B),
            muted_foreground: Color::from_rgb8(0x94, 0xA3, 0xB8),
            ring: Color::from_rgb8(0x94, 0xA3, 0xB8),
        }
    }
}

impl Default for Palette {
    fn default() -> Self {
        Self::light()
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
