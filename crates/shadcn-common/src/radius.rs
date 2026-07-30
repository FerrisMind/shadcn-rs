//! Radius tokens matching shadcn-svelte `RADII` / `PRESET_RADII`.

use twill_core::tokens::BorderRadius;

/// Named radius picker values from shadcn-svelte create customizer.
///
/// `Default` means “use radius from style” — Lyra/Sera resolve to [`Self::None`];
/// other styles use the base-color CSS `--radius` (`0.625rem`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum RadiusId {
    /// Use the selected style’s intrinsic `--radius`.
    #[default]
    Default,
    /// `0rem`
    None,
    /// `0.45rem`
    Small,
    /// `0.625rem`
    Medium,
    /// `0.875rem`
    Large,
}

impl RadiusId {
    pub const ALL: [Self; 5] = [
        Self::Default,
        Self::None,
        Self::Small,
        Self::Medium,
        Self::Large,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::None => "none",
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Large => "large",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Default => "Default",
            Self::None => "None",
            Self::Small => "Small",
            Self::Medium => "Medium",
            Self::Large => "Large",
        }
    }

    /// Explicit rem for non-[`Self::Default`] ids (shadcn `RADII[].value`).
    pub const fn rem(self) -> Option<f32> {
        match self {
            Self::Default => None,
            Self::None => Some(0.0),
            Self::Small => Some(0.45),
            Self::Medium => Some(0.625),
            Self::Large => Some(0.875),
        }
    }

    /// Resolved rem given the style’s intrinsic default rem.
    pub const fn resolved_rem(self, style_default_rem: f32) -> f32 {
        match self.rem() {
            Some(rem) => rem,
            None => style_default_rem,
        }
    }

    /// Twill corner tokens for a resolved rem.
    pub const fn twill_radii_for_rem(rem: f32) -> (BorderRadius, BorderRadius, BorderRadius) {
        if rem <= 0.0 {
            (BorderRadius::None, BorderRadius::None, BorderRadius::None)
        } else if rem <= 0.45 {
            (BorderRadius::Sm, BorderRadius::Md, BorderRadius::Md)
        } else if rem <= 0.5 {
            (BorderRadius::Sm, BorderRadius::Md, BorderRadius::Lg)
        } else if rem <= 0.625 {
            (BorderRadius::Md, BorderRadius::Lg, BorderRadius::Xl)
        } else {
            (BorderRadius::Lg, BorderRadius::Xl, BorderRadius::S2xl)
        }
    }

    /// Nearest explicit preset for an arbitrary rem (not `default`).
    pub fn from_rem(rem: f32) -> Self {
        if rem <= 0.0 {
            return Self::None;
        }
        let explicit: [(Self, f32); 3] = [
            (Self::Small, 0.45),
            (Self::Medium, 0.625),
            (Self::Large, 0.875),
        ];
        explicit
            .into_iter()
            .min_by(|(_, a), (_, b)| {
                (a - rem)
                    .abs()
                    .partial_cmp(&(b - rem).abs())
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(id, _)| id)
            .unwrap_or(Self::Medium)
    }
}

/// Pixel radius scale derived from a rem base (1rem = 16px).
///
/// Matches shadcn-svelte `app.css` `@theme` tokens:
/// `--radius-sm/md/lg/xl/2xl/3xl/4xl` as `calc(var(--radius) ± Npx)`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RadiusScale {
    pub base_rem: f32,
    pub sm_px: f32,
    pub md_px: f32,
    pub lg_px: f32,
    pub xl_px: f32,
    /// `--radius-2xl` = base + 8.
    pub xxl_px: f32,
    /// `--radius-3xl` = base + 12.
    pub xxxl_px: f32,
    /// `--radius-4xl` = base + 16.
    pub xxxxl_px: f32,
}

impl RadiusScale {
    pub const fn from_rem(base_rem: f32) -> Self {
        let base_px = base_rem * 16.0;
        Self {
            base_rem,
            sm_px: (base_px - 4.0).max(0.0),
            md_px: (base_px - 2.0).max(0.0),
            lg_px: base_px,
            xl_px: base_px + 4.0,
            xxl_px: base_px + 8.0,
            xxxl_px: base_px + 12.0,
            xxxxl_px: base_px + 16.0,
        }
    }

    pub const fn none() -> Self {
        Self {
            base_rem: 0.0,
            sm_px: 0.0,
            md_px: 0.0,
            lg_px: 0.0,
            xl_px: 0.0,
            xxl_px: 0.0,
            xxxl_px: 0.0,
            xxxxl_px: 0.0,
        }
    }

    pub fn id(self) -> RadiusId {
        if self.base_rem <= 0.0 {
            RadiusId::None
        } else {
            RadiusId::from_rem(self.base_rem)
        }
    }
}

impl Default for RadiusScale {
    fn default() -> Self {
        // Base-color CSS `--radius` when picker is `default`.
        Self::from_rem(0.625)
    }
}
