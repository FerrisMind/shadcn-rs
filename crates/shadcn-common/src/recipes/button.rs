//! Button size / type recipes from `.cn-button` + `.cn-button-size-*`.

use crate::style::StyleId;

use super::{ComponentRadius, FontWeight, TypeRecipe};

/// Logical control size ladder (`xs` / `sm` / `default` / `lg`).
///
/// Icon-only footprints share the same height tokens as the matching text size.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum ControlSize {
    Xs,
    Sm,
    #[default]
    Md,
    Lg,
}

/// Geometry tokens for one button size under a style pack.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ButtonSizeRecipe {
    pub height_px: f32,
    /// Default horizontal padding (`px-*`) when no inline icons.
    pub pad_x_px: f32,
    /// Leading padding when an inline-start icon is present.
    pub pad_x_icon_start_px: f32,
    /// Trailing padding when an inline-end icon is present.
    pub pad_x_icon_end_px: f32,
    /// Gap between spinner/icon and label.
    pub gap_px: f32,
    /// Label text size for this size slot (may override base `.cn-button`).
    pub text_size_px: f32,
    /// Default SVG / icon footprint (`[&_svg]:size-*`).
    pub icon_px: f32,
}

/// Base `.cn-button` type recipe (weight / casing / tracking) for a style pack.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ButtonTypeRecipe {
    pub typography: TypeRecipe,
    pub default_radius: ComponentRadius,
}

/// Resolves `.cn-button-size-*` geometry for `style` + `size`.
pub const fn button_size(style: StyleId, size: ControlSize) -> ButtonSizeRecipe {
    match style {
        StyleId::Vega => vega_size(size),
        StyleId::Nova => nova_size(size),
        StyleId::Maia => maia_size(size),
        StyleId::Lyra => lyra_size(size),
        StyleId::Mira => mira_size(size),
        StyleId::Luma => luma_size(size),
        StyleId::Sera => sera_size(size),
        StyleId::Rhea => rhea_size(size),
    }
}

/// Resolves base `.cn-button` type + default radius for `style`.
pub const fn button_type(style: StyleId) -> ButtonTypeRecipe {
    match style {
        StyleId::Sera => ButtonTypeRecipe {
            typography: TypeRecipe {
                size_px: 12.0,
                weight: FontWeight::Semibold,
                uppercase: true,
                tracking_em: 0.1, // tracking-widest
                line_height_px: 16.0,
            },
            default_radius: ComponentRadius::None,
        },
        StyleId::Lyra => ButtonTypeRecipe {
            typography: TypeRecipe {
                size_px: 12.0,
                weight: FontWeight::Medium,
                uppercase: false,
                tracking_em: 0.0,
                line_height_px: 16.0,
            },
            default_radius: ComponentRadius::None,
        },
        StyleId::Mira => ButtonTypeRecipe {
            typography: TypeRecipe {
                size_px: 12.0,
                weight: FontWeight::Medium,
                uppercase: false,
                tracking_em: 0.0,
                line_height_px: 12.0 * 1.625,
            },
            default_radius: ComponentRadius::Md,
        },
        StyleId::Nova => ButtonTypeRecipe {
            typography: TypeRecipe {
                size_px: 14.0,
                weight: FontWeight::Medium,
                uppercase: false,
                tracking_em: 0.0,
                line_height_px: 20.0,
            },
            default_radius: ComponentRadius::Lg,
        },
        StyleId::Maia => ButtonTypeRecipe {
            typography: TypeRecipe {
                size_px: 14.0,
                weight: FontWeight::Medium,
                uppercase: false,
                tracking_em: 0.0,
                line_height_px: 20.0,
            },
            default_radius: ComponentRadius::S4xl, // rounded-4xl
        },
        StyleId::Luma => ButtonTypeRecipe {
            typography: TypeRecipe {
                size_px: 14.0,
                weight: FontWeight::Medium,
                uppercase: false,
                tracking_em: 0.0,
                line_height_px: 20.0,
            },
            default_radius: ComponentRadius::S4xl, // rounded-4xl
        },
        StyleId::Rhea => ButtonTypeRecipe {
            typography: TypeRecipe {
                size_px: 14.0,
                weight: FontWeight::Medium,
                uppercase: false,
                tracking_em: 0.0,
                line_height_px: 20.0,
            },
            default_radius: ComponentRadius::S2xl, // rounded-2xl
        },
        StyleId::Vega => ButtonTypeRecipe {
            typography: TypeRecipe {
                size_px: 14.0,
                weight: FontWeight::Medium,
                uppercase: false,
                tracking_em: 0.0,
                line_height_px: 20.0,
            },
            default_radius: ComponentRadius::Md,
        },
    }
}

const fn vega_size(size: ControlSize) -> ButtonSizeRecipe {
    match size {
        ControlSize::Xs => ButtonSizeRecipe {
            height_px: 24.0,
            pad_x_px: 8.0,
            pad_x_icon_start_px: 6.0,
            pad_x_icon_end_px: 6.0,
            gap_px: 4.0,
            text_size_px: 12.0,
            icon_px: 12.0,
        },
        ControlSize::Sm => ButtonSizeRecipe {
            height_px: 32.0,
            pad_x_px: 10.0,
            pad_x_icon_start_px: 6.0,
            pad_x_icon_end_px: 6.0,
            gap_px: 4.0,
            text_size_px: 14.0,
            icon_px: 16.0,
        },
        ControlSize::Md => ButtonSizeRecipe {
            height_px: 36.0,
            pad_x_px: 10.0,
            pad_x_icon_start_px: 8.0,
            pad_x_icon_end_px: 8.0,
            gap_px: 6.0,
            text_size_px: 14.0,
            icon_px: 16.0,
        },
        ControlSize::Lg => ButtonSizeRecipe {
            height_px: 40.0,
            pad_x_px: 10.0,
            pad_x_icon_start_px: 8.0,
            pad_x_icon_end_px: 8.0,
            gap_px: 6.0,
            text_size_px: 14.0,
            icon_px: 16.0,
        },
    }
}

const fn nova_size(size: ControlSize) -> ButtonSizeRecipe {
    match size {
        ControlSize::Xs => ButtonSizeRecipe {
            height_px: 24.0,
            pad_x_px: 8.0,
            pad_x_icon_start_px: 6.0,
            pad_x_icon_end_px: 6.0,
            gap_px: 4.0,
            text_size_px: 12.0,
            icon_px: 12.0,
        },
        ControlSize::Sm => ButtonSizeRecipe {
            height_px: 28.0,
            pad_x_px: 10.0,
            pad_x_icon_start_px: 6.0,
            pad_x_icon_end_px: 6.0,
            gap_px: 4.0,
            text_size_px: 12.8, // text-[0.8rem]
            icon_px: 14.0,
        },
        ControlSize::Md => ButtonSizeRecipe {
            height_px: 32.0,
            pad_x_px: 10.0,
            pad_x_icon_start_px: 8.0,
            pad_x_icon_end_px: 8.0,
            gap_px: 6.0,
            text_size_px: 14.0,
            icon_px: 16.0,
        },
        ControlSize::Lg => ButtonSizeRecipe {
            height_px: 36.0,
            pad_x_px: 10.0,
            pad_x_icon_start_px: 8.0,
            pad_x_icon_end_px: 8.0,
            gap_px: 6.0,
            text_size_px: 14.0,
            icon_px: 16.0,
        },
    }
}

const fn maia_size(size: ControlSize) -> ButtonSizeRecipe {
    match size {
        ControlSize::Xs => ButtonSizeRecipe {
            height_px: 24.0,
            pad_x_px: 10.0,
            pad_x_icon_start_px: 8.0,
            pad_x_icon_end_px: 8.0,
            gap_px: 4.0,
            text_size_px: 12.0,
            icon_px: 12.0,
        },
        ControlSize::Sm => ButtonSizeRecipe {
            height_px: 32.0,
            pad_x_px: 12.0,
            pad_x_icon_start_px: 8.0,
            pad_x_icon_end_px: 8.0,
            gap_px: 4.0,
            text_size_px: 14.0,
            icon_px: 16.0,
        },
        ControlSize::Md => ButtonSizeRecipe {
            height_px: 36.0,
            pad_x_px: 12.0,
            pad_x_icon_start_px: 10.0,
            pad_x_icon_end_px: 10.0,
            gap_px: 6.0,
            text_size_px: 14.0,
            icon_px: 16.0,
        },
        ControlSize::Lg => ButtonSizeRecipe {
            height_px: 40.0,
            pad_x_px: 16.0,
            pad_x_icon_start_px: 12.0,
            pad_x_icon_end_px: 12.0,
            gap_px: 6.0,
            text_size_px: 14.0,
            icon_px: 16.0,
        },
    }
}

const fn lyra_size(size: ControlSize) -> ButtonSizeRecipe {
    match size {
        ControlSize::Xs => ButtonSizeRecipe {
            height_px: 24.0,
            pad_x_px: 8.0,
            pad_x_icon_start_px: 6.0,
            pad_x_icon_end_px: 6.0,
            gap_px: 4.0,
            text_size_px: 12.0,
            icon_px: 12.0,
        },
        ControlSize::Sm => ButtonSizeRecipe {
            height_px: 28.0,
            pad_x_px: 10.0,
            pad_x_icon_start_px: 6.0,
            pad_x_icon_end_px: 6.0,
            gap_px: 4.0,
            text_size_px: 12.0,
            icon_px: 14.0,
        },
        ControlSize::Md => ButtonSizeRecipe {
            height_px: 32.0,
            pad_x_px: 10.0,
            pad_x_icon_start_px: 8.0,
            pad_x_icon_end_px: 8.0,
            gap_px: 6.0,
            text_size_px: 12.0,
            icon_px: 16.0,
        },
        ControlSize::Lg => ButtonSizeRecipe {
            height_px: 36.0,
            pad_x_px: 10.0,
            pad_x_icon_start_px: 8.0,
            pad_x_icon_end_px: 8.0,
            gap_px: 6.0,
            text_size_px: 12.0,
            icon_px: 16.0,
        },
    }
}

const fn mira_size(size: ControlSize) -> ButtonSizeRecipe {
    match size {
        ControlSize::Xs => ButtonSizeRecipe {
            height_px: 20.0,
            pad_x_px: 8.0,
            pad_x_icon_start_px: 6.0,
            pad_x_icon_end_px: 6.0,
            gap_px: 4.0,
            text_size_px: 10.0, // 0.625rem
            icon_px: 10.0,
        },
        ControlSize::Sm => ButtonSizeRecipe {
            height_px: 24.0,
            pad_x_px: 8.0,
            pad_x_icon_start_px: 6.0,
            pad_x_icon_end_px: 6.0,
            gap_px: 4.0,
            text_size_px: 12.0,
            icon_px: 12.0,
        },
        ControlSize::Md => ButtonSizeRecipe {
            height_px: 28.0,
            pad_x_px: 8.0,
            pad_x_icon_start_px: 6.0,
            pad_x_icon_end_px: 6.0,
            gap_px: 4.0,
            text_size_px: 12.0,
            icon_px: 14.0,
        },
        ControlSize::Lg => ButtonSizeRecipe {
            height_px: 32.0,
            pad_x_px: 10.0,
            pad_x_icon_start_px: 8.0,
            pad_x_icon_end_px: 8.0,
            gap_px: 4.0,
            text_size_px: 12.0,
            icon_px: 16.0,
        },
    }
}

const fn luma_size(size: ControlSize) -> ButtonSizeRecipe {
    // Same size ladder as Maia in style-luma.css.
    maia_size(size)
}

const fn sera_size(size: ControlSize) -> ButtonSizeRecipe {
    match size {
        ControlSize::Xs => ButtonSizeRecipe {
            height_px: 28.0,
            pad_x_px: 12.0,
            pad_x_icon_start_px: 8.0,
            pad_x_icon_end_px: 8.0,
            gap_px: 4.0,
            text_size_px: 12.0,
            icon_px: 12.0,
        },
        ControlSize::Sm => ButtonSizeRecipe {
            height_px: 36.0,
            pad_x_px: 16.0,
            pad_x_icon_start_px: 12.0,
            pad_x_icon_end_px: 12.0,
            gap_px: 4.0,
            text_size_px: 12.0,
            icon_px: 14.0,
        },
        ControlSize::Md => ButtonSizeRecipe {
            height_px: 40.0,
            pad_x_px: 24.0,
            pad_x_icon_start_px: 16.0,
            pad_x_icon_end_px: 16.0,
            gap_px: 6.0,
            text_size_px: 12.0,
            icon_px: 14.0,
        },
        ControlSize::Lg => ButtonSizeRecipe {
            height_px: 44.0,
            pad_x_px: 32.0,
            pad_x_icon_start_px: 20.0,
            pad_x_icon_end_px: 20.0,
            gap_px: 6.0,
            text_size_px: 12.0,
            icon_px: 14.0,
        },
    }
}

const fn rhea_size(size: ControlSize) -> ButtonSizeRecipe {
    match size {
        ControlSize::Xs => ButtonSizeRecipe {
            height_px: 24.0,
            pad_x_px: 10.0,
            pad_x_icon_start_px: 8.0,
            pad_x_icon_end_px: 8.0,
            gap_px: 4.0,
            text_size_px: 12.0,
            icon_px: 12.0,
        },
        ControlSize::Sm => ButtonSizeRecipe {
            height_px: 28.0,
            pad_x_px: 12.0,
            pad_x_icon_start_px: 8.0,
            pad_x_icon_end_px: 8.0,
            gap_px: 4.0,
            text_size_px: 14.0,
            icon_px: 16.0,
        },
        ControlSize::Md => ButtonSizeRecipe {
            height_px: 32.0,
            pad_x_px: 12.0,
            pad_x_icon_start_px: 10.0,
            pad_x_icon_end_px: 10.0,
            gap_px: 6.0,
            text_size_px: 14.0,
            icon_px: 16.0,
        },
        ControlSize::Lg => ButtonSizeRecipe {
            height_px: 36.0,
            pad_x_px: 16.0,
            pad_x_icon_start_px: 12.0,
            pad_x_icon_end_px: 12.0,
            gap_px: 6.0,
            text_size_px: 14.0,
            icon_px: 16.0,
        },
    }
}
