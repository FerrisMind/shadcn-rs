//! Item spacing, radius, and typography geometry.
//!
//! All values are transcribed from the `cn-item*` rules of the eight
//! shadcn-svelte style packs (`style-vega.css` … `style-rhea.css`).

use shadcn_common::{FontWeight, StyleId};

use super::types::{ItemRadius, ItemSize};
use crate::theme::Theme;

/// Row gap and padding for one style pack and density.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct SizeMetrics {
    /// Gap between row slots (`gap-*`).
    pub(super) gap: f32,
    /// Horizontal row inset (`px-*`).
    pub(super) padding_x: f32,
    /// Vertical row inset (`py-*`).
    pub(super) padding_y: f32,
}

/// Resolved `cn-item-size-*` metrics.
pub(super) fn size_metrics(theme: &Theme, size: ItemSize) -> SizeMetrics {
    let (gap, padding_x, padding_y) = match theme.style_id() {
        StyleId::Vega => match size {
            ItemSize::Default => (14.0, 16.0, 14.0),
            ItemSize::Sm => (10.0, 12.0, 10.0),
            ItemSize::Xs => (8.0, 10.0, 8.0),
        },
        StyleId::Nova | StyleId::Lyra => match size {
            ItemSize::Default | ItemSize::Sm => (10.0, 12.0, 10.0),
            ItemSize::Xs => (8.0, 10.0, 8.0),
        },
        StyleId::Mira => match size {
            ItemSize::Default | ItemSize::Sm => (10.0, 12.0, 10.0),
            ItemSize::Xs => (10.0, 10.0, 8.0),
        },
        StyleId::Maia | StyleId::Luma | StyleId::Sera => match size {
            ItemSize::Default => (14.0, 16.0, 14.0),
            ItemSize::Sm => (14.0, 14.0, 12.0),
            ItemSize::Xs => (10.0, 12.0, 10.0),
        },
        StyleId::Rhea => match size {
            ItemSize::Default => (14.0, 16.0, 14.0),
            ItemSize::Sm => (14.0, 14.0, 12.0),
            ItemSize::Xs => (8.0, 10.0, 8.0),
        },
    };

    SizeMetrics {
        gap,
        padding_x,
        padding_y,
    }
}

/// Resolved item corner radius in pixels.
pub(super) fn radius_px(theme: &Theme, radius: ItemRadius) -> f32 {
    let scale = theme.style.radius;

    let resolved = match radius {
        ItemRadius::Theme => match theme.style_id() {
            // rounded-md
            StyleId::Vega | StyleId::Mira => scale.md_px,
            // rounded-lg
            StyleId::Nova => scale.lg_px,
            // rounded-none
            StyleId::Lyra | StyleId::Sera => 0.0,
            // rounded-2xl
            StyleId::Maia | StyleId::Luma | StyleId::Rhea => scale.xxl_px,
        },
        ItemRadius::None => 0.0,
        ItemRadius::Small => scale.sm_px,
        ItemRadius::Medium => scale.md_px,
        ItemRadius::Large => scale.lg_px,
        ItemRadius::Xl => scale.xl_px,
        ItemRadius::Full => 9999.0,
        ItemRadius::Custom(value) if value.is_finite() => value.max(0.0),
        ItemRadius::Custom(_) => 0.0,
    };

    resolved.max(0.0)
}

/// Gap between multiple media children (`cn-item-media` `gap-2`).
pub(super) fn media_gap() -> f32 {
    8.0
}

/// Square edge of the `image` media variant for one density.
pub(super) fn media_image_size_px(theme: &Theme, size: ItemSize) -> f32 {
    match size {
        ItemSize::Default => {
            if theme.style_id() == StyleId::Mira {
                32.0
            } else {
                40.0
            }
        }
        ItemSize::Sm => 32.0,
        ItemSize::Xs => 24.0,
    }
}

/// Corner radius of the `image` media variant for one density.
pub(super) fn media_image_radius_px(theme: &Theme, size: ItemSize) -> f32 {
    let scale = theme.style.radius;
    let xs = size == ItemSize::Xs;

    match theme.style_id() {
        // rounded-sm
        StyleId::Vega | StyleId::Nova | StyleId::Mira => scale.sm_px,
        // rounded-none
        StyleId::Lyra | StyleId::Sera => 0.0,
        // rounded-lg, xs: rounded-md
        StyleId::Maia => {
            if xs {
                scale.md_px
            } else {
                scale.lg_px
            }
        }
        // rounded-xl, xs: rounded-lg
        StyleId::Luma | StyleId::Rhea => {
            if xs {
                scale.lg_px
            } else {
                scale.xl_px
            }
        }
    }
}

/// Top offset applied to media next to a description (`translate-y-0.5`).
pub(super) fn media_description_offset() -> f32 {
    2.0
}

/// Gap between title and description rows (`cn-item-content`).
pub(super) fn content_gap(theme: &Theme, size: ItemSize) -> f32 {
    if size == ItemSize::Xs {
        match theme.style_id() {
            StyleId::Vega | StyleId::Nova | StyleId::Lyra => 0.0,
            StyleId::Mira | StyleId::Maia | StyleId::Luma | StyleId::Sera | StyleId::Rhea => 2.0,
        }
    } else {
        4.0
    }
}

/// Gap inside actions, header, and footer rows (`gap-2`).
pub(super) fn section_gap() -> f32 {
    8.0
}

/// Source item text metrics for one style pack.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct TextMetrics {
    pub(super) size_px: f32,
    pub(super) line_height_px: f32,
    pub(super) weight: FontWeight,
    pub(super) uppercase: bool,
}

/// Resolved `cn-item-title` typography.
pub(super) fn title_metrics(theme: &Theme) -> TextMetrics {
    match theme.style_id() {
        StyleId::Vega | StyleId::Nova | StyleId::Maia | StyleId::Luma | StyleId::Rhea => {
            // text-sm leading-snug font-medium
            TextMetrics {
                size_px: 14.0,
                line_height_px: 19.25,
                weight: FontWeight::Medium,
                uppercase: false,
            }
        }
        StyleId::Lyra => TextMetrics {
            // text-xs font-medium
            size_px: 12.0,
            line_height_px: 16.0,
            weight: FontWeight::Medium,
            uppercase: false,
        },
        StyleId::Mira => TextMetrics {
            // text-xs leading-snug font-medium
            size_px: 12.0,
            line_height_px: 16.5,
            weight: FontWeight::Medium,
            uppercase: false,
        },
        StyleId::Sera => TextMetrics {
            // text-xs leading-snug font-semibold uppercase
            size_px: 12.0,
            line_height_px: 16.5,
            weight: FontWeight::Semibold,
            uppercase: true,
        },
    }
}

/// Resolved `cn-item-description` typography for one density.
pub(super) fn description_metrics(theme: &Theme, size: ItemSize) -> TextMetrics {
    let (size_px, line_height_px) = match theme.style_id() {
        // text-sm leading-normal, xs: text-xs
        StyleId::Vega | StyleId::Nova => {
            if size == ItemSize::Xs {
                (12.0, 16.0)
            } else {
                (14.0, 21.0)
            }
        }
        // text-xs/relaxed
        StyleId::Lyra | StyleId::Mira => (12.0, 19.5),
        // text-sm
        StyleId::Maia | StyleId::Luma | StyleId::Rhea => (14.0, 20.0),
        // text-sm leading-relaxed
        StyleId::Sera => (14.0, 22.75),
    };

    TextMetrics {
        size_px,
        line_height_px,
        weight: FontWeight::Normal,
        uppercase: false,
    }
}

/// Vertical gap between grouped items (`cn-item-group`).
///
/// The source `gap-4 has-data-[size=sm]:gap-2.5 has-data-[size=xs]:gap-2`
/// cascade resolves against the densest item present in the group.
pub(super) fn group_gap(has_sm: bool, has_xs: bool) -> f32 {
    if has_xs {
        8.0
    } else if has_sm {
        10.0
    } else {
        16.0
    }
}

/// Vertical margin around an item separator (`my-2`).
pub(super) fn separator_margin_y() -> f32 {
    8.0
}
