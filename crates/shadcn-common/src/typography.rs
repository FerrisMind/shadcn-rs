//! Font tokens for shadcn style packs (`--font-sans` / `--font-heading` / `--font-mono`).

/// Logical font family ids used by demos and backends when loading faces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FontId {
    Geist,
    GeistMono,
    Inter,
    InstrumentSerif,
    JetBrainsMono,
}

impl FontId {
    pub const ALL: [Self; 5] = [
        Self::Geist,
        Self::Inter,
        Self::InstrumentSerif,
        Self::GeistMono,
        Self::JetBrainsMono,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Geist => "geist",
            Self::GeistMono => "geist-mono",
            Self::Inter => "inter",
            Self::InstrumentSerif => "instrument-serif",
            Self::JetBrainsMono => "jetbrains-mono",
        }
    }

    /// Human label (shadcn font picker title).
    pub const fn title(self) -> &'static str {
        match self {
            Self::Geist => "Geist",
            Self::GeistMono => "Geist Mono",
            Self::Inter => "Inter",
            Self::InstrumentSerif => "Instrument Serif",
            Self::JetBrainsMono => "JetBrains Mono",
        }
    }

    /// Family name embedded in the bundled TTF (for iced `Font::with_name`).
    pub const fn family_name(self) -> &'static str {
        self.title()
    }

    pub const fn css_family(self) -> &'static str {
        match self {
            Self::Geist => "'Geist Variable', sans-serif",
            Self::GeistMono => "'Geist Mono Variable', monospace",
            Self::Inter => "'Inter Variable', sans-serif",
            Self::InstrumentSerif => "'Instrument Serif', serif",
            Self::JetBrainsMono => "'JetBrains Mono Variable', monospace",
        }
    }
}

/// Body / heading / mono families — maps to `--font-sans`, `--font-heading`, `--font-mono`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FontPack {
    pub sans: FontId,
    pub heading: FontId,
    pub mono: FontId,
}

impl FontPack {
    pub const GEIST: Self = Self {
        sans: FontId::Geist,
        heading: FontId::Geist,
        mono: FontId::GeistMono,
    };

    pub const GEIST_MONO: Self = Self {
        sans: FontId::GeistMono,
        heading: FontId::GeistMono,
        mono: FontId::GeistMono,
    };

    pub const INTER: Self = Self {
        sans: FontId::Inter,
        heading: FontId::Inter,
        mono: FontId::JetBrainsMono,
    };

    pub const INSTRUMENT_SERIF: Self = Self {
        sans: FontId::Geist,
        heading: FontId::InstrumentSerif,
        mono: FontId::GeistMono,
    };

    pub const fn with_sans(mut self, sans: FontId) -> Self {
        self.sans = sans;
        self
    }

    pub const fn with_heading_font(mut self, heading: FontId) -> Self {
        self.heading = heading;
        self
    }

    pub const fn with_mono(mut self, mono: FontId) -> Self {
        self.mono = mono;
        self
    }
}

/// `--font-heading` picker value (shadcn create customizer: `fontHeading`).
///
/// `Inherit` means headings use the current body (`--font-sans`) family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum FontHeading {
    #[default]
    Inherit,
    Font(FontId),
}

impl FontHeading {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Inherit => "inherit",
            Self::Font(font) => font.as_str(),
        }
    }

    pub const fn title(self) -> &'static str {
        match self {
            Self::Inherit => "Inherit",
            Self::Font(font) => font.title(),
        }
    }

    /// Resolve to a concrete face given the current body font.
    pub const fn resolve(self, sans: FontId) -> FontId {
        match self {
            Self::Inherit => sans,
            Self::Font(font) => font,
        }
    }

    /// Infer picker state from a resolved pack (heading == sans → inherit).
    pub const fn from_pack(pack: FontPack) -> Self {
        if font_id_eq(pack.heading, pack.sans) {
            Self::Inherit
        } else {
            Self::Font(pack.heading)
        }
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
