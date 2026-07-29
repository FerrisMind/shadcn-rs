//! Breadcrumb geometry and typography metrics.
//!
//! Values are transcribed from the `.cn-breadcrumb-*` rules of the source
//! style packs. The `sm:` gap step of the web CSS has no iced media-query
//! analog, so the mobile-first base value is used; override it with
//! [`super::BreadcrumbList::spacing`] on wide layouts.

use shadcn_common::StyleId;

use crate::theme::Theme;

/// Resolved breadcrumb dimensions for one shadcn style pack.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct Metrics {
    /// Gap between list entries (`.cn-breadcrumb-list` `gap-1.5`).
    pub(super) list_gap_px: f32,
    /// Gap between the children of one item (`.cn-breadcrumb-item` `gap-1.5` / `gap-1`).
    pub(super) item_gap_px: f32,
    /// List font size (`text-sm` → 14 px, `text-xs` → 12 px).
    pub(super) text_size_px: f32,
    /// List line height in px.
    pub(super) line_height_px: f32,
    /// Whether the list is uppercased (`uppercase`, Sera only).
    pub(super) uppercase: bool,
    /// Letter spacing in `em`, carried for parity — iced has no letter spacing.
    pub(super) tracking_em: f32,
    /// Default separator glyph footprint (`.cn-breadcrumb-separator` `[&>svg]:size-3.5`).
    pub(super) separator_icon_px: f32,
    /// Ellipsis box footprint (`.cn-breadcrumb-ellipsis` `size-5` / Mira `size-4`).
    pub(super) ellipsis_box_px: f32,
    /// Ellipsis glyph footprint (`[&>svg]:size-4` / Mira `size-3.5`).
    pub(super) ellipsis_icon_px: f32,
}

/// Resolves the dimensions encoded by the source style CSS.
pub(super) fn metrics(theme: &Theme) -> Metrics {
    match theme.style_id() {
        StyleId::Vega | StyleId::Maia | StyleId::Luma | StyleId::Rhea => Metrics {
            list_gap_px: 6.0,
            item_gap_px: 6.0,
            text_size_px: 14.0,
            line_height_px: 20.0,
            uppercase: false,
            tracking_em: 0.0,
            separator_icon_px: 14.0,
            ellipsis_box_px: 20.0,
            ellipsis_icon_px: 16.0,
        },
        StyleId::Nova => Metrics {
            list_gap_px: 6.0,
            item_gap_px: 4.0,
            text_size_px: 14.0,
            line_height_px: 20.0,
            uppercase: false,
            tracking_em: 0.0,
            separator_icon_px: 14.0,
            ellipsis_box_px: 20.0,
            ellipsis_icon_px: 16.0,
        },
        StyleId::Lyra => Metrics {
            list_gap_px: 6.0,
            item_gap_px: 4.0,
            text_size_px: 12.0,
            line_height_px: 16.0,
            uppercase: false,
            tracking_em: 0.0,
            separator_icon_px: 14.0,
            ellipsis_box_px: 20.0,
            ellipsis_icon_px: 16.0,
        },
        // Mira: `text-xs/relaxed` → 12 px with a 1.625 line-height ratio.
        StyleId::Mira => Metrics {
            list_gap_px: 6.0,
            item_gap_px: 4.0,
            text_size_px: 12.0,
            line_height_px: 12.0 * 1.625,
            uppercase: false,
            tracking_em: 0.0,
            separator_icon_px: 14.0,
            ellipsis_box_px: 16.0,
            ellipsis_icon_px: 14.0,
        },
        // Sera: `text-xs tracking-wide uppercase`.
        StyleId::Sera => Metrics {
            list_gap_px: 6.0,
            item_gap_px: 6.0,
            text_size_px: 12.0,
            line_height_px: 16.0,
            uppercase: true,
            tracking_em: 0.025,
            separator_icon_px: 14.0,
            ellipsis_box_px: 20.0,
            ellipsis_icon_px: 16.0,
        },
    }
}

/// Clamps a layout value to a non-negative, finite pixel amount.
pub(super) fn normalize_px(value: f32) -> f32 {
    if value.is_finite() {
        value.max(0.0)
    } else {
        0.0
    }
}

/// Clamps a typography value to a finite pixel amount of at least 1 px.
pub(super) fn normalize_min_px(value: f32) -> f32 {
    if value.is_finite() {
        value.max(1.0)
    } else {
        1.0
    }
}
