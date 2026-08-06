//! Public configuration types for the typography component.

use shadcn_common::{FontWeight, TypeRecipe};

/// Text treatment ported from the shadcn-svelte “Typography” page.
///
/// Each variant mirrors one documented example (`typography-h1`,
/// `typography-p`, …) with its Tailwind classes translated to backend-agnostic
/// [`TypeRecipe`] tokens. Lists and tables have dedicated builders
/// ([`super::TypographyList`] / [`super::TypographyTable`]) because they are
/// not single text blocks.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum TypographyVariant {
    /// `text-4xl font-extrabold tracking-tight` (36 px / 40 px).
    ///
    /// The web `lg:text-5xl` step has no iced media-query analog — use
    /// [`super::Typography::size`] with `48.0` on wide layouts instead.
    H1,
    /// `border-b pb-2 text-3xl font-semibold tracking-tight` (30 px / 36 px)
    /// with a 1 px underline in the theme border color.
    H2,
    /// `text-2xl font-semibold tracking-tight` (24 px / 32 px).
    H3,
    /// `text-xl font-semibold tracking-tight` (20 px / 28 px).
    H4,
    /// `leading-7` paragraph (16 px / 28 px).
    #[default]
    P,
    /// `border-s-2 ps-6 italic` quote (16 px / 24 px) with a 2 px leading bar
    /// in the theme border color.
    Blockquote,
    /// `bg-muted rounded px-[0.3rem] py-[0.2rem] font-mono text-sm
    /// font-semibold` chip (14 px / 20 px).
    InlineCode,
    /// `text-muted-foreground text-xl` intro paragraph (20 px / 28 px).
    Lead,
    /// `text-lg font-semibold` (18 px / 28 px).
    Large,
    /// `text-sm leading-none font-medium` (14 px / 14 px).
    Small,
    /// `text-muted-foreground text-sm` helper text (14 px / 20 px).
    Muted,
}

impl TypographyVariant {
    /// Every variant, in the order of the shadcn-svelte typography page.
    pub const ALL: [Self; 11] = [
        Self::H1,
        Self::H2,
        Self::H3,
        Self::H4,
        Self::P,
        Self::Blockquote,
        Self::InlineCode,
        Self::Lead,
        Self::Large,
        Self::Small,
        Self::Muted,
    ];

    /// Kebab-case name matching the shadcn-svelte example id.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::H1 => "h1",
            Self::H2 => "h2",
            Self::H3 => "h3",
            Self::H4 => "h4",
            Self::P => "p",
            Self::Blockquote => "blockquote",
            Self::InlineCode => "inline-code",
            Self::Lead => "lead",
            Self::Large => "large",
            Self::Small => "small",
            Self::Muted => "muted",
        }
    }

    /// Shared typography tokens (size, weight, tracking, line height).
    ///
    /// Exposed so custom compositions (rich text, inline links) can reuse the
    /// exact tokens of a variant without going through [`super::Typography`].
    ///
    /// `tracking_em` is carried for parity with the web classes
    /// (`tracking-tight` → −0.025 em) but is not applied yet: iced text has no
    /// letter-spacing support.
    ///
    /// ```rust
    /// use iced_shadcn_v2::TypographyVariant;
    ///
    /// let recipe = TypographyVariant::H1.type_recipe();
    /// assert_eq!(recipe.size_px, 36.0);
    /// ```
    pub const fn type_recipe(self) -> TypeRecipe {
        match self {
            Self::H1 => TypeRecipe {
                size_px: 36.0,
                weight: FontWeight::ExtraBold,
                uppercase: false,
                tracking_em: -0.025,
                line_height_px: 40.0,
            },
            Self::H2 => TypeRecipe {
                size_px: 30.0,
                weight: FontWeight::Semibold,
                uppercase: false,
                tracking_em: -0.025,
                line_height_px: 36.0,
            },
            Self::H3 => TypeRecipe {
                size_px: 24.0,
                weight: FontWeight::Semibold,
                uppercase: false,
                tracking_em: -0.025,
                line_height_px: 32.0,
            },
            Self::H4 => TypeRecipe {
                size_px: 20.0,
                weight: FontWeight::Semibold,
                uppercase: false,
                tracking_em: -0.025,
                line_height_px: 28.0,
            },
            Self::P => TypeRecipe {
                size_px: 16.0,
                weight: FontWeight::Normal,
                uppercase: false,
                tracking_em: 0.0,
                line_height_px: 28.0,
            },
            Self::Blockquote => TypeRecipe {
                size_px: 16.0,
                weight: FontWeight::Normal,
                uppercase: false,
                tracking_em: 0.0,
                line_height_px: 24.0,
            },
            Self::InlineCode => TypeRecipe {
                size_px: 14.0,
                weight: FontWeight::Semibold,
                uppercase: false,
                tracking_em: 0.0,
                line_height_px: 20.0,
            },
            Self::Lead => TypeRecipe {
                size_px: 20.0,
                weight: FontWeight::Normal,
                uppercase: false,
                tracking_em: 0.0,
                line_height_px: 28.0,
            },
            Self::Large => TypeRecipe {
                size_px: 18.0,
                weight: FontWeight::Semibold,
                uppercase: false,
                tracking_em: 0.0,
                line_height_px: 28.0,
            },
            Self::Small => TypeRecipe {
                size_px: 14.0,
                weight: FontWeight::Medium,
                uppercase: false,
                tracking_em: 0.0,
                line_height_px: 14.0,
            },
            Self::Muted => TypeRecipe {
                size_px: 14.0,
                weight: FontWeight::Normal,
                uppercase: false,
                tracking_em: 0.0,
                line_height_px: 20.0,
            },
        }
    }

    /// Whether the variant renders with the theme heading face
    /// (`--font-heading`).
    pub const fn uses_heading_font(self) -> bool {
        matches!(self, Self::H1 | Self::H2 | Self::H3 | Self::H4)
    }

    /// Whether the variant renders with the theme mono face (`font-mono`).
    pub const fn uses_mono_font(self) -> bool {
        matches!(self, Self::InlineCode)
    }

    /// Whether the variant is rendered italic (`blockquote`).
    pub const fn is_italic(self) -> bool {
        matches!(self, Self::Blockquote)
    }

    /// Whether the variant defaults to `text-muted-foreground`.
    pub const fn is_muted(self) -> bool {
        matches!(self, Self::Lead | Self::Muted)
    }

    /// Top margin used by the shadcn typography demo article flow, in px.
    ///
    /// Mirrors `mt-10` on `h2`, `mt-8` on `h3`, `[&:not(:first-child)]:mt-6`
    /// on paragraphs, and `mt-6` on blockquotes. Margins are opt-in — see
    /// [`super::Typography::default_margin`].
    pub const fn default_margin_top_px(self) -> f32 {
        match self {
            Self::H1 | Self::InlineCode | Self::Large | Self::Small | Self::Muted => 0.0,
            Self::H2 => 40.0,
            Self::H3 | Self::H4 => 32.0,
            Self::P | Self::Blockquote | Self::Lead => 24.0,
        }
    }
}
