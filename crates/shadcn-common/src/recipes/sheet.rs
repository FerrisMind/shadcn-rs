//! Sheet recipes from `.cn-sheet-overlay` / `.cn-sheet-content` /
//! `.cn-sheet-header` / `.cn-sheet-footer` / `.cn-sheet-title` /
//! `.cn-sheet-description` / `.cn-sheet-close` across style packs.
//!
//! Sheet is a dialog-like modal docked to an edge (`side`) with a
//! `slide-in-from-*-10` entrance (`duration-200`).

use crate::style::StyleId;

use super::{FontWeight, PopoverShadow, TypeRecipe};

/// Duration of the sheet open/close animation (`duration-200`).
pub const SHEET_ANIMATION_MS: u64 = 200;

/// Distance covered by `slide-in-from-*-10` (`2.5rem`).
pub const SHEET_SLIDE_PX: f32 = 40.0;

/// Left/right panel width fraction (`w-3/4`).
pub const SHEET_SIDE_WIDTH_FRACTION: f32 = 0.75;

/// Left/right `sm:max-w-sm` cap.
pub const SHEET_MAX_WIDTH_PX: f32 = 384.0;

/// Footprint of the close button (`size = icon-sm` → `size-8`).
pub const SHEET_CLOSE_SIZE_PX: f32 = 32.0;

/// Close button glyph size (Lucide `XIcon` at `size-4`).
pub const SHEET_CLOSE_ICON_PX: f32 = 16.0;

/// Edge the sheet docks to (`data-side`).
///
/// Matches the `side` prop of the shadcn-svelte `Sheet.Content` component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum SheetSide {
    /// Docks to the top edge (`inset-x-0 top-0`).
    Top,
    /// Docks to the right edge (`inset-y-0 right-0`) — the web default.
    #[default]
    Right,
    /// Docks to the bottom edge (`inset-x-0 bottom-0`).
    Bottom,
    /// Docks to the left edge (`inset-y-0 left-0`).
    Left,
}

impl SheetSide {
    /// Every supported edge, in documentation order.
    pub const ALL: [Self; 4] = [Self::Top, Self::Right, Self::Bottom, Self::Left];

    /// Whether the sheet spans the full viewport height (`left` / `right`).
    #[must_use]
    pub const fn is_vertical_edge(self) -> bool {
        matches!(self, Self::Left | Self::Right)
    }

    /// Whether the sheet spans the full viewport width (`top` / `bottom`).
    #[must_use]
    pub const fn is_horizontal_edge(self) -> bool {
        matches!(self, Self::Top | Self::Bottom)
    }
}

/// Resolved panel geometry for a viewport and [`SheetSide`].
///
/// Shared by iced and egui so both backends size and slide identically.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SheetPanelMetrics {
    /// Panel origin X in viewport coordinates.
    pub x: f32,
    /// Panel origin Y in viewport coordinates.
    pub y: f32,
    /// Panel width in px.
    pub width: f32,
    /// Panel height in px.
    pub height: f32,
    /// Translation applied at progress `0` (fully off-screen offset for the
    /// `slide-in-from-*-10` entrance — a short nudge, not a full dismiss).
    pub slide_from_x: f32,
    /// See [`Self::slide_from_x`].
    pub slide_from_y: f32,
}

/// Computes docked panel bounds and the entrance slide vector.
///
/// Left/right panels use `min(viewport * 3/4, max_width_px)`. Top/bottom
/// panels span the full width and use `content_height` capped by
/// `max_height_px` when set.
#[must_use]
pub fn sheet_panel_metrics(
    viewport_width: f32,
    viewport_height: f32,
    side: SheetSide,
    max_width_px: f32,
    content_height: f32,
    max_height_px: Option<f32>,
) -> SheetPanelMetrics {
    let vw = viewport_width.max(0.0);
    let vh = viewport_height.max(0.0);
    let max_w = max_width_px.max(0.0);
    let slide = SHEET_SLIDE_PX;

    match side {
        SheetSide::Right => {
            let width = (vw * SHEET_SIDE_WIDTH_FRACTION).min(max_w).min(vw);
            SheetPanelMetrics {
                x: (vw - width).max(0.0),
                y: 0.0,
                width,
                height: vh,
                slide_from_x: slide,
                slide_from_y: 0.0,
            }
        }
        SheetSide::Left => {
            let width = (vw * SHEET_SIDE_WIDTH_FRACTION).min(max_w).min(vw);
            SheetPanelMetrics {
                x: 0.0,
                y: 0.0,
                width,
                height: vh,
                slide_from_x: -slide,
                slide_from_y: 0.0,
            }
        }
        SheetSide::Top => {
            let mut height = content_height.max(0.0).min(vh);
            if let Some(max_h) = max_height_px {
                height = height.min(max_h.max(0.0));
            }
            SheetPanelMetrics {
                x: 0.0,
                y: 0.0,
                width: vw,
                height,
                slide_from_x: 0.0,
                slide_from_y: -slide,
            }
        }
        SheetSide::Bottom => {
            let mut height = content_height.max(0.0).min(vh);
            if let Some(max_h) = max_height_px {
                height = height.min(max_h.max(0.0));
            }
            SheetPanelMetrics {
                x: 0.0,
                y: (vh - height).max(0.0),
                width: vw,
                height,
                slide_from_x: 0.0,
                slide_from_y: slide,
            }
        }
    }
}

/// Geometry + typography recipe for `.cn-sheet-*` slots.
///
/// Colors stay with the backend palettes (`bg-popover` /
/// `text-popover-foreground` / `border`); only geometry, alphas, and type
/// tokens live here.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SheetRecipe {
    /// Backdrop black alpha (`bg-black/10` … `bg-black/80`).
    pub overlay_alpha: f32,
    /// Gap of the content column (`gap-4` / none).
    pub gap_px: f32,
    /// Left/right `sm:max-w-sm` width cap.
    pub max_width_px: f32,
    /// Surface drop shadow (`shadow-lg` / `shadow-xl` / `shadow-md`).
    pub shadow: PopoverShadow,
    /// Body typography (`text-sm` / `text-xs/relaxed`).
    pub typography: TypeRecipe,
    /// Header padding (`p-4` / `p-6` / `p-8`).
    pub header_pad_px: f32,
    /// Gap of the header column (`gap-1.5` / `gap-0.5`).
    pub header_gap_px: f32,
    /// Footer padding (`p-4` / `p-6` / `p-8`).
    pub footer_pad_px: f32,
    /// Gap of the footer column (`gap-2`).
    pub footer_gap_px: f32,
    /// Title typography (`.cn-sheet-title`).
    pub title: TypeRecipe,
    /// Description typography (`.cn-sheet-description`).
    pub description: TypeRecipe,
    /// Inset of the close button (`top-4 right-4` / `top-3 right-3`).
    pub close_offset_px: f32,
    /// Whether the close button rests on `bg-secondary`.
    pub close_secondary_bg: bool,
}

/// Resolves `.cn-sheet-*` tokens for `style`.
pub const fn sheet_recipe(style: StyleId) -> SheetRecipe {
    let base = base_recipe();

    match style {
        StyleId::Vega => base,
        StyleId::Nova => SheetRecipe {
            header_gap_px: 2.0,
            close_offset_px: 12.0,
            title: text_base(FontWeight::Medium),
            ..base
        },
        StyleId::Maia => SheetRecipe {
            overlay_alpha: 0.80,
            gap_px: 0.0,
            header_pad_px: 24.0,
            footer_pad_px: 24.0,
            title: text_base(FontWeight::Medium),
            ..base
        },
        StyleId::Luma => SheetRecipe {
            overlay_alpha: 0.30,
            gap_px: 0.0,
            shadow: PopoverShadow::XL,
            header_pad_px: 24.0,
            footer_pad_px: 24.0,
            title: text_base(FontWeight::Medium),
            close_secondary_bg: true,
            ..base
        },
        StyleId::Rhea => SheetRecipe {
            overlay_alpha: 0.30,
            gap_px: 0.0,
            shadow: PopoverShadow::XL,
            header_pad_px: 24.0,
            footer_pad_px: 24.0,
            title: text_base(FontWeight::Medium),
            close_secondary_bg: true,
            ..base
        },
        StyleId::Sera => SheetRecipe {
            overlay_alpha: 0.20,
            gap_px: 0.0,
            shadow: PopoverShadow::MD,
            header_pad_px: 32.0,
            footer_pad_px: 32.0,
            title: TypeRecipe {
                uppercase: true,
                tracking_em: 0.05,
                ..text_lg(FontWeight::Semibold)
            },
            description: TypeRecipe {
                line_height_px: 22.75,
                ..text_sm(FontWeight::Normal)
            },
            close_secondary_bg: true,
            ..base
        },
        StyleId::Mira => SheetRecipe {
            overlay_alpha: 0.80,
            gap_px: 0.0,
            typography: text_xs_relaxed(FontWeight::Normal),
            header_pad_px: 24.0,
            footer_pad_px: 24.0,
            title: text_sm(FontWeight::Medium),
            description: text_xs_relaxed(FontWeight::Normal),
            ..base
        },
        StyleId::Lyra => SheetRecipe {
            gap_px: 0.0,
            typography: text_xs_relaxed(FontWeight::Normal),
            header_gap_px: 2.0,
            close_offset_px: 12.0,
            title: text_sm(FontWeight::Medium),
            description: text_xs_relaxed(FontWeight::Normal),
            ..base
        },
    }
}

const fn base_recipe() -> SheetRecipe {
    SheetRecipe {
        overlay_alpha: 0.10,
        gap_px: 16.0,
        max_width_px: SHEET_MAX_WIDTH_PX,
        shadow: PopoverShadow::LG,
        typography: text_sm(FontWeight::Normal),
        header_pad_px: 16.0,
        header_gap_px: 6.0,
        footer_pad_px: 16.0,
        footer_gap_px: 8.0,
        title: text_sm(FontWeight::Medium),
        description: text_sm(FontWeight::Normal),
        close_offset_px: 16.0,
        close_secondary_bg: false,
    }
}

const fn text_xs_relaxed(weight: FontWeight) -> TypeRecipe {
    TypeRecipe {
        size_px: 12.0,
        weight,
        uppercase: false,
        tracking_em: 0.0,
        line_height_px: 19.5,
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

const fn text_lg(weight: FontWeight) -> TypeRecipe {
    TypeRecipe {
        size_px: 18.0,
        weight,
        uppercase: false,
        tracking_em: 0.0,
        line_height_px: 28.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vega_matches_style_css() {
        let vega = sheet_recipe(StyleId::Vega);
        assert_eq!(vega.overlay_alpha, 0.10);
        assert_eq!(vega.gap_px, 16.0);
        assert_eq!(vega.max_width_px, 384.0);
        assert_eq!(vega.shadow, PopoverShadow::LG);
        assert_eq!(vega.header_pad_px, 16.0);
        assert_eq!(vega.header_gap_px, 6.0);
        assert_eq!(vega.close_offset_px, 16.0);
        assert!(!vega.close_secondary_bg);
        assert_eq!(vega.title.size_px, 14.0);
    }

    #[test]
    fn packs_track_their_overrides() {
        let nova = sheet_recipe(StyleId::Nova);
        assert_eq!(nova.close_offset_px, 12.0);
        assert_eq!(nova.header_gap_px, 2.0);
        assert_eq!(nova.title.size_px, 16.0);

        let maia = sheet_recipe(StyleId::Maia);
        assert_eq!(maia.overlay_alpha, 0.80);
        assert_eq!(maia.gap_px, 0.0);
        assert_eq!(maia.header_pad_px, 24.0);

        let rhea = sheet_recipe(StyleId::Rhea);
        assert_eq!(rhea.shadow, PopoverShadow::XL);
        assert!(rhea.close_secondary_bg);

        let sera = sheet_recipe(StyleId::Sera);
        assert_eq!(sera.shadow, PopoverShadow::MD);
        assert!(sera.title.uppercase);
        assert_eq!(sera.title.tracking_em, 0.05);
        assert_eq!(sera.header_pad_px, 32.0);

        let mira = sheet_recipe(StyleId::Mira);
        assert_eq!(mira.typography.size_px, 12.0);
        assert_eq!(mira.description.line_height_px, 19.5);

        let lyra = sheet_recipe(StyleId::Lyra);
        assert_eq!(lyra.close_offset_px, 12.0);
        assert_eq!(lyra.overlay_alpha, 0.10);
    }

    #[test]
    fn panel_metrics_right_caps_at_sm_max() {
        let m = sheet_panel_metrics(800.0, 600.0, SheetSide::Right, 384.0, 0.0, None);
        assert_eq!(m.width, 384.0);
        assert_eq!(m.height, 600.0);
        assert_eq!(m.x, 416.0);
        assert_eq!(m.slide_from_x, SHEET_SLIDE_PX);
    }

    #[test]
    fn panel_metrics_left_uses_three_quarter_on_narrow() {
        let m = sheet_panel_metrics(400.0, 500.0, SheetSide::Left, 384.0, 0.0, None);
        assert_eq!(m.width, 300.0);
        assert_eq!(m.x, 0.0);
        assert_eq!(m.slide_from_x, -SHEET_SLIDE_PX);
    }

    #[test]
    fn panel_metrics_bottom_respects_max_height() {
        let m = sheet_panel_metrics(640.0, 800.0, SheetSide::Bottom, 384.0, 500.0, Some(400.0));
        assert_eq!(m.width, 640.0);
        assert_eq!(m.height, 400.0);
        assert_eq!(m.y, 400.0);
        assert_eq!(m.slide_from_y, SHEET_SLIDE_PX);
    }

    #[test]
    fn side_edge_helpers() {
        assert!(SheetSide::Right.is_vertical_edge());
        assert!(SheetSide::Top.is_horizontal_edge());
        assert_eq!(SheetSide::default(), SheetSide::Right);
    }
}
