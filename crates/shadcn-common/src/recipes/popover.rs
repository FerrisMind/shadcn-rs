//! Popover recipes from `.cn-popover-content` / `.cn-popover-header` /
//! `.cn-popover-title` / `.cn-popover-description` across style packs.

use crate::style::StyleId;

use super::{ComponentRadius, FontWeight, TypeRecipe};

/// Duration of the popover open/close animation (`duration-100`).
pub const POPOVER_ANIMATION_MS: u64 = 100;

/// Distance covered by the `slide-in-from-*-2` entrance animation.
pub const POPOVER_SLIDE_PX: f32 = 8.0;

/// Initial scale of the `zoom-in-95` entrance animation.
pub const POPOVER_ZOOM_FROM: f32 = 0.95;

/// Default popover content width (`w-72`).
pub const POPOVER_WIDTH_PX: f32 = 288.0;

/// Drop shadow tokens of a popover surface (`shadow-md` / `shadow-lg` /
/// `shadow-2xl`), reduced to the primary tailwind shadow layer.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PopoverShadow {
    /// Vertical shadow offset in px.
    pub offset_y_px: f32,
    /// Blur radius in px.
    pub blur_px: f32,
    /// Black shadow alpha.
    pub alpha: f32,
}

impl PopoverShadow {
    /// Tailwind `shadow-xs`: `0 1px 2px 0 rgb(0 0 0 / 0.05)`.
    pub const XS: Self = Self {
        offset_y_px: 1.0,
        blur_px: 2.0,
        alpha: 0.05,
    };

    /// Tailwind `shadow-md`: `0 4px 6px -1px rgb(0 0 0 / 0.1)`.
    pub const MD: Self = Self {
        offset_y_px: 4.0,
        blur_px: 6.0,
        alpha: 0.10,
    };

    /// Tailwind `shadow-lg`: `0 10px 15px -3px rgb(0 0 0 / 0.1)`.
    pub const LG: Self = Self {
        offset_y_px: 10.0,
        blur_px: 15.0,
        alpha: 0.10,
    };

    /// Tailwind `shadow-xl`: `0 20px 25px -5px rgb(0 0 0 / 0.1)`.
    pub const XL: Self = Self {
        offset_y_px: 20.0,
        blur_px: 25.0,
        alpha: 0.10,
    };

    /// Tailwind `shadow-2xl`: `0 25px 50px -12px rgb(0 0 0 / 0.25)`.
    pub const XXL: Self = Self {
        offset_y_px: 25.0,
        blur_px: 50.0,
        alpha: 0.25,
    };
}

/// Geometry + typography recipe for `.cn-popover-content` and its
/// header/title/description slots.
///
/// The popover surface uses the `bg-popover` / `text-popover-foreground`
/// pair with a `ring-1 ring-foreground/N` hairline; colors stay with the
/// backend palettes, only geometry and alphas live here.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PopoverRecipe {
    /// Uniform content padding (`p-4` / `p-2.5`).
    pub pad_px: f32,
    /// Gap of the content column (`gap-4` / `gap-2.5`).
    pub gap_px: f32,
    /// Default content width (`w-72`).
    pub width_px: f32,
    /// Surface corner radius intent (`rounded-md` / `rounded-lg` / …).
    pub radius: ComponentRadius,
    /// `ring-foreground/N` alpha in light mode.
    pub ring_alpha: f32,
    /// `ring-foreground/N` alpha in dark mode (`dark:ring-foreground/N`).
    pub ring_alpha_dark: f32,
    /// Surface drop shadow.
    pub shadow: PopoverShadow,
    /// Body typography (`text-sm` / `text-xs`).
    pub typography: TypeRecipe,
    /// Gap of the header column (`gap-1` / `gap-0.5`).
    pub header_gap_px: f32,
    /// Title typography (`.cn-popover-title`).
    pub title: TypeRecipe,
    /// Description typography (`.cn-popover-description`,
    /// `text-muted-foreground`).
    pub description: TypeRecipe,
}

/// Resolves `.cn-popover-content` tokens for `style`.
pub const fn popover_recipe(style: StyleId) -> PopoverRecipe {
    let base = base_recipe();

    match style {
        StyleId::Vega => base,
        StyleId::Nova => PopoverRecipe {
            pad_px: 10.0,
            gap_px: 10.0,
            radius: ComponentRadius::Lg,
            header_gap_px: 2.0,
            ..base
        },
        StyleId::Mira => PopoverRecipe {
            pad_px: 10.0,
            radius: ComponentRadius::Lg,
            typography: text_xs(FontWeight::Normal),
            description: text_xs(FontWeight::Normal),
            ..base
        },
        StyleId::Lyra => PopoverRecipe {
            pad_px: 10.0,
            gap_px: 10.0,
            radius: ComponentRadius::None,
            typography: text_xs(FontWeight::Normal),
            description: TypeRecipe {
                line_height_px: 19.5,
                ..text_xs(FontWeight::Normal)
            },
            ..base
        },
        StyleId::Maia => PopoverRecipe {
            radius: ComponentRadius::S2xl, // rounded-2xl
            ring_alpha: 0.05,
            ring_alpha_dark: 0.05,
            shadow: PopoverShadow::XXL,
            title: text_base(FontWeight::Medium),
            ..base
        },
        StyleId::Luma | StyleId::Rhea => PopoverRecipe {
            radius: ComponentRadius::S3xl,
            ring_alpha: 0.05,
            ring_alpha_dark: 0.10,
            shadow: PopoverShadow::LG,
            title: text_base(FontWeight::Medium),
            ..base
        },
        StyleId::Sera => PopoverRecipe {
            radius: ComponentRadius::None,
            title: TypeRecipe {
                uppercase: true,
                ..text_xs(FontWeight::Semibold)
            },
            description: TypeRecipe {
                line_height_px: 22.75,
                ..text_sm(FontWeight::Normal)
            },
            ..base
        },
    }
}

const fn base_recipe() -> PopoverRecipe {
    PopoverRecipe {
        pad_px: 16.0,
        gap_px: 16.0,
        width_px: POPOVER_WIDTH_PX,
        radius: ComponentRadius::Md,
        ring_alpha: 0.10,
        ring_alpha_dark: 0.10,
        shadow: PopoverShadow::MD,
        typography: text_sm(FontWeight::Normal),
        header_gap_px: 4.0,
        title: text_sm(FontWeight::Medium),
        description: text_sm(FontWeight::Normal),
    }
}

const fn text_xs(weight: FontWeight) -> TypeRecipe {
    TypeRecipe {
        size_px: 12.0,
        weight,
        uppercase: false,
        tracking_em: 0.0,
        line_height_px: 16.0,
    }
}

const fn text_sm(weight: FontWeight) -> TypeRecipe {
    TypeRecipe {
        size_px: 14.0,
        weight,
        uppercase: false,
        tracking_em: 0.0,
        line_height_px: 20.0,
    }
}

const fn text_base(weight: FontWeight) -> TypeRecipe {
    TypeRecipe {
        size_px: 16.0,
        weight,
        uppercase: false,
        tracking_em: 0.0,
        line_height_px: 24.0,
    }
}
