//! Drawer recipes from `.cn-drawer-overlay` / `.cn-drawer-content` /
//! `.cn-drawer-handle` / `.cn-drawer-header` / `.cn-drawer-footer` /
//! `.cn-drawer-title` / `.cn-drawer-description` across style packs.
//!
//! Drawer is the vaul-backed sheet variant: default direction is bottom,
//! content carries an optional drag handle, docks with `mt-24` /
//! `max-h-[80vh]` on the horizontal edges, and rounds the inner corners
//! (`rounded-*-xl`). Geometry is shared by iced and egui.

use crate::style::StyleId;

use super::{ComponentRadius, FontWeight, TypeRecipe};

/// Duration of the drawer open/close animation (vaul default ≈ 500 ms).
pub const DRAWER_ANIMATION_MS: u64 = 500;

/// Opposite-edge inset for top/bottom drawers (`mt-24` / `mb-24` → 6 rem).
pub const DRAWER_EDGE_INSET_PX: f32 = 96.0;

/// Top/bottom height cap (`max-h-[80vh]`).
pub const DRAWER_MAX_HEIGHT_FRACTION: f32 = 0.80;

/// Left/right panel width fraction (`w-3/4`).
pub const DRAWER_SIDE_WIDTH_FRACTION: f32 = 0.75;

/// Left/right `sm:max-w-sm` cap.
pub const DRAWER_MAX_WIDTH_PX: f32 = 384.0;

/// Drag-handle width (`w-[100px]`).
pub const DRAWER_HANDLE_WIDTH_PX: f32 = 100.0;

/// Default drag-handle height (`h-1.5` → 0.375 rem).
pub const DRAWER_HANDLE_HEIGHT_PX: f32 = 6.0;

/// Compact drag-handle height (`h-1` → 0.25 rem) used by Nova / Lyra.
pub const DRAWER_HANDLE_HEIGHT_COMPACT_PX: f32 = 4.0;

/// Top margin above the handle (`mt-4`).
pub const DRAWER_HANDLE_MARGIN_TOP_PX: f32 = 16.0;

/// Edge the drawer docks to (`data-vaul-drawer-direction`).
///
/// Matches the `direction` prop of the shadcn-svelte / vaul `Drawer.Root`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum DrawerDirection {
    /// Docks to the top edge (`inset-x-0 top-0`).
    Top,
    /// Docks to the right edge (`inset-y-0 right-0`).
    Right,
    /// Docks to the bottom edge (`inset-x-0 bottom-0`) — the web default.
    #[default]
    Bottom,
    /// Docks to the left edge (`inset-y-0 left-0`).
    Left,
}

impl DrawerDirection {
    /// Every supported direction, in documentation order.
    pub const ALL: [Self; 4] = [Self::Top, Self::Right, Self::Bottom, Self::Left];

    /// Whether the drawer spans the full viewport height (`left` / `right`).
    #[must_use]
    pub const fn is_vertical_edge(self) -> bool {
        matches!(self, Self::Left | Self::Right)
    }

    /// Whether the drawer spans the full viewport width (`top` / `bottom`).
    #[must_use]
    pub const fn is_horizontal_edge(self) -> bool {
        matches!(self, Self::Top | Self::Bottom)
    }

    /// Whether the bottom-only drag handle is shown.
    #[must_use]
    pub const fn shows_handle(self) -> bool {
        matches!(self, Self::Bottom)
    }

    /// Whether header text is centered on narrow layouts (top / bottom).
    #[must_use]
    pub const fn centers_header(self) -> bool {
        matches!(self, Self::Top | Self::Bottom)
    }
}

/// Which corners receive the content radius for a given direction.
///
/// Matches `rounded-t-xl` / `rounded-b-xl` / `rounded-l-xl` / `rounded-r-xl`.
/// Floating packs (`Maia` / `Luma` / `Mira` / `Rhea`) round every corner.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct DrawerCornerMask {
    /// Top-left corner.
    pub top_left: bool,
    /// Top-right corner.
    pub top_right: bool,
    /// Bottom-right corner.
    pub bottom_right: bool,
    /// Bottom-left corner.
    pub bottom_left: bool,
}

impl DrawerCornerMask {
    /// Mask for docked drawers (inner-edge rounding only).
    #[must_use]
    pub const fn for_direction(direction: DrawerDirection) -> Self {
        match direction {
            DrawerDirection::Bottom => Self {
                top_left: true,
                top_right: true,
                bottom_right: false,
                bottom_left: false,
            },
            DrawerDirection::Top => Self {
                top_left: false,
                top_right: false,
                bottom_right: true,
                bottom_left: true,
            },
            DrawerDirection::Left => Self {
                top_left: false,
                top_right: true,
                bottom_right: true,
                bottom_left: false,
            },
            DrawerDirection::Right => Self {
                top_left: true,
                top_right: false,
                bottom_right: false,
                bottom_left: true,
            },
        }
    }

    /// Mask that rounds every corner (floating inset packs).
    #[must_use]
    pub const fn all() -> Self {
        Self {
            top_left: true,
            top_right: true,
            bottom_right: true,
            bottom_left: true,
        }
    }
}

/// Resolved panel geometry for a viewport and [`DrawerDirection`].
///
/// Shared by iced and egui so both backends size and slide identically.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DrawerPanelMetrics {
    /// Panel origin X in viewport coordinates.
    pub x: f32,
    /// Panel origin Y in viewport coordinates.
    pub y: f32,
    /// Panel width in px.
    pub width: f32,
    /// Panel height in px.
    pub height: f32,
    /// Translation applied at progress `0` (fully off-screen for vaul).
    pub slide_from_x: f32,
    /// See [`Self::slide_from_x`].
    pub slide_from_y: f32,
}

/// Computes docked panel bounds and the entrance slide vector.
///
/// Left/right panels use `min(viewport * 3/4, max_width_px)`. Top/bottom
/// panels span the full width, leave [`DRAWER_EDGE_INSET_PX`] on the opposite
/// edge, and cap height at `max(content, snap) ∩ 80vh ∩ (vh − inset)`.
///
/// `snap_fraction`, when set, forces the panel height to that fraction of the
/// viewport (vaul `activeSnapPoint`) before applying the same caps.
#[must_use]
pub fn drawer_panel_metrics(
    viewport_width: f32,
    viewport_height: f32,
    direction: DrawerDirection,
    max_width_px: f32,
    content_height: f32,
    max_height_px: Option<f32>,
    snap_fraction: Option<f32>,
) -> DrawerPanelMetrics {
    let vw = viewport_width.max(0.0);
    let vh = viewport_height.max(0.0);
    let max_w = max_width_px.max(0.0);
    let inset = DRAWER_EDGE_INSET_PX;
    let vh_cap = vh * DRAWER_MAX_HEIGHT_FRACTION;

    match direction {
        DrawerDirection::Right => {
            let width = (vw * DRAWER_SIDE_WIDTH_FRACTION).min(max_w).min(vw);
            DrawerPanelMetrics {
                x: (vw - width).max(0.0),
                y: 0.0,
                width,
                height: vh,
                slide_from_x: width,
                slide_from_y: 0.0,
            }
        }
        DrawerDirection::Left => {
            let width = (vw * DRAWER_SIDE_WIDTH_FRACTION).min(max_w).min(vw);
            DrawerPanelMetrics {
                x: 0.0,
                y: 0.0,
                width,
                height: vh,
                slide_from_x: -width,
                slide_from_y: 0.0,
            }
        }
        DrawerDirection::Top => {
            let mut height = content_height.max(0.0);
            if let Some(snap) = snap_fraction {
                height = (vh * snap.clamp(0.0, 1.0)).max(height);
            }
            height = height.min(vh_cap).min((vh - inset).max(0.0));
            if let Some(max_h) = max_height_px {
                height = height.min(max_h.max(0.0));
            }
            DrawerPanelMetrics {
                x: 0.0,
                y: 0.0,
                width: vw,
                height,
                slide_from_x: 0.0,
                slide_from_y: -height,
            }
        }
        DrawerDirection::Bottom => {
            let mut height = content_height.max(0.0);
            if let Some(snap) = snap_fraction {
                height = (vh * snap.clamp(0.0, 1.0)).max(0.0);
            }
            height = height.min(vh_cap).min((vh - inset).max(0.0));
            if let Some(max_h) = max_height_px {
                height = height.min(max_h.max(0.0));
            }
            DrawerPanelMetrics {
                x: 0.0,
                y: (vh - height).max(0.0),
                width: vw,
                height,
                slide_from_x: 0.0,
                slide_from_y: height,
            }
        }
    }
}

/// Geometry + typography recipe for `.cn-drawer-*` slots.
///
/// Colors stay with the backend palettes (`bg-popover` /
/// `text-popover-foreground` / `border` / `bg-muted`); only geometry,
/// alphas, and type tokens live here.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DrawerRecipe {
    /// Backdrop black alpha (`bg-black/10` … `bg-black/80`).
    pub overlay_alpha: f32,
    /// Left/right `sm:max-w-sm` width cap.
    pub max_width_px: f32,
    /// Surface corner radius intent (`rounded-xl` / `rounded-none` / …).
    pub radius: ComponentRadius,
    /// Whether every corner is rounded (floating inset packs).
    pub round_all_corners: bool,
    /// Inset padding of floating packs (`p-2` / `p-4`); `0` for solid packs.
    pub floating_pad_px: f32,
    /// Body typography (`text-sm` / `text-xs/relaxed`).
    pub typography: TypeRecipe,
    /// Header padding (`p-4`).
    pub header_pad_px: f32,
    /// Gap of the header column (`gap-0.5` / `gap-1` / `gap-1.5`).
    pub header_gap_px: f32,
    /// Footer padding (`p-4`).
    pub footer_pad_px: f32,
    /// Gap of the footer column (`gap-2`).
    pub footer_gap_px: f32,
    /// Title typography (`.cn-drawer-title`).
    pub title: TypeRecipe,
    /// Description typography (`.cn-drawer-description`).
    pub description: TypeRecipe,
    /// Drag-handle height in px.
    pub handle_height_px: f32,
    /// Drag-handle corner radius intent.
    pub handle_radius: ComponentRadius,
}

/// Resolves `.cn-drawer-*` tokens for `style`.
pub const fn drawer_recipe(style: StyleId) -> DrawerRecipe {
    let base = base_recipe();

    match style {
        StyleId::Vega => base,
        StyleId::Nova => DrawerRecipe {
            header_gap_px: 2.0,
            title: text_base(FontWeight::Medium),
            handle_height_px: DRAWER_HANDLE_HEIGHT_COMPACT_PX,
            ..base
        },
        // Soft shell: `rounded-4xl` / `before:rounded-4xl` floating panel.
        StyleId::Maia => DrawerRecipe {
            overlay_alpha: 0.80,
            radius: ComponentRadius::S4xl,
            round_all_corners: true,
            floating_pad_px: 16.0,
            title: text_base(FontWeight::Medium),
            ..base
        },
        // Soft shell: `before:rounded-4xl` floating panel.
        StyleId::Luma => DrawerRecipe {
            overlay_alpha: 0.30,
            radius: ComponentRadius::S4xl,
            round_all_corners: true,
            floating_pad_px: 16.0,
            title: text_base(FontWeight::Medium),
            ..base
        },
        // Soft shell: `rounded-[min(var(--radius-4xl),24px)]` → 24px (`S3xl`).
        StyleId::Rhea => DrawerRecipe {
            overlay_alpha: 0.30,
            radius: ComponentRadius::S3xl,
            round_all_corners: true,
            floating_pad_px: 16.0,
            title: text_base(FontWeight::Medium),
            ..base
        },
        StyleId::Sera => DrawerRecipe {
            overlay_alpha: 0.20,
            radius: ComponentRadius::None,
            handle_radius: ComponentRadius::None,
            header_gap_px: 4.0,
            title: TypeRecipe {
                uppercase: true,
                tracking_em: 0.05,
                ..text_lg(FontWeight::Semibold)
            },
            description: TypeRecipe {
                line_height_px: 22.75,
                ..text_sm(FontWeight::Normal)
            },
            ..base
        },
        StyleId::Mira => DrawerRecipe {
            overlay_alpha: 0.80,
            radius: ComponentRadius::Xl,
            round_all_corners: true,
            floating_pad_px: 8.0,
            typography: text_xs_relaxed(FontWeight::Normal),
            header_gap_px: 4.0,
            title: text_sm(FontWeight::Medium),
            description: text_xs_relaxed(FontWeight::Normal),
            ..base
        },
        StyleId::Lyra => DrawerRecipe {
            radius: ComponentRadius::None,
            handle_height_px: DRAWER_HANDLE_HEIGHT_COMPACT_PX,
            handle_radius: ComponentRadius::None,
            typography: text_xs_relaxed(FontWeight::Normal),
            header_gap_px: 2.0,
            title: text_sm(FontWeight::Medium),
            description: text_xs_relaxed(FontWeight::Normal),
            ..base
        },
    }
}

/// Corner mask for `direction` under the given recipe.
#[must_use]
pub const fn drawer_corner_mask(
    direction: DrawerDirection,
    recipe: &DrawerRecipe,
) -> DrawerCornerMask {
    if recipe.round_all_corners {
        DrawerCornerMask::all()
    } else {
        DrawerCornerMask::for_direction(direction)
    }
}

const fn base_recipe() -> DrawerRecipe {
    DrawerRecipe {
        overlay_alpha: 0.10,
        max_width_px: DRAWER_MAX_WIDTH_PX,
        radius: ComponentRadius::Xl,
        round_all_corners: false,
        floating_pad_px: 0.0,
        typography: text_sm(FontWeight::Normal),
        header_pad_px: 16.0,
        header_gap_px: 2.0,
        footer_pad_px: 16.0,
        footer_gap_px: 8.0,
        title: text_sm(FontWeight::Medium),
        description: text_sm(FontWeight::Normal),
        handle_height_px: DRAWER_HANDLE_HEIGHT_PX,
        handle_radius: ComponentRadius::Full,
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
        let vega = drawer_recipe(StyleId::Vega);
        assert_eq!(vega.overlay_alpha, 0.10);
        assert_eq!(vega.max_width_px, 384.0);
        assert_eq!(vega.radius, ComponentRadius::Xl);
        assert!(!vega.round_all_corners);
        assert_eq!(vega.header_pad_px, 16.0);
        assert_eq!(vega.header_gap_px, 2.0);
        assert_eq!(vega.handle_height_px, DRAWER_HANDLE_HEIGHT_PX);
        assert_eq!(vega.title.size_px, 14.0);
        assert_eq!(DrawerDirection::default(), DrawerDirection::Bottom);
    }

    #[test]
    fn packs_track_their_overrides() {
        let nova = drawer_recipe(StyleId::Nova);
        assert_eq!(nova.handle_height_px, DRAWER_HANDLE_HEIGHT_COMPACT_PX);
        assert_eq!(nova.title.size_px, 16.0);

        let maia = drawer_recipe(StyleId::Maia);
        assert_eq!(maia.overlay_alpha, 0.80);
        assert_eq!(maia.radius, ComponentRadius::S4xl);
        assert!(maia.round_all_corners);
        assert_eq!(maia.floating_pad_px, 16.0);

        let rhea = drawer_recipe(StyleId::Rhea);
        assert_eq!(rhea.radius, ComponentRadius::S3xl);

        let sera = drawer_recipe(StyleId::Sera);
        assert_eq!(sera.radius, ComponentRadius::None);
        assert!(sera.title.uppercase);
        assert_eq!(sera.title.tracking_em, 0.05);

        let mira = drawer_recipe(StyleId::Mira);
        assert_eq!(mira.typography.size_px, 12.0);
        assert_eq!(mira.floating_pad_px, 8.0);

        let lyra = drawer_recipe(StyleId::Lyra);
        assert_eq!(lyra.radius, ComponentRadius::None);
        assert_eq!(lyra.handle_radius, ComponentRadius::None);
    }

    #[test]
    fn panel_metrics_bottom_respects_80vh_and_inset() {
        let m = drawer_panel_metrics(
            640.0,
            800.0,
            DrawerDirection::Bottom,
            384.0,
            900.0,
            None,
            None,
        );
        assert_eq!(m.width, 640.0);
        // min(900, 0.8*800=640, 800-96=704) = 640
        assert_eq!(m.height, 640.0);
        assert_eq!(m.y, 160.0);
        assert_eq!(m.slide_from_y, 640.0);
    }

    #[test]
    fn panel_metrics_bottom_snap_fraction() {
        let m = drawer_panel_metrics(
            640.0,
            800.0,
            DrawerDirection::Bottom,
            384.0,
            0.0,
            None,
            Some(0.5),
        );
        assert_eq!(m.height, 400.0);
        assert_eq!(m.y, 400.0);
    }

    #[test]
    fn panel_metrics_right_caps_at_sm_max() {
        let m = drawer_panel_metrics(800.0, 600.0, DrawerDirection::Right, 384.0, 0.0, None, None);
        assert_eq!(m.width, 384.0);
        assert_eq!(m.height, 600.0);
        assert_eq!(m.x, 416.0);
        assert_eq!(m.slide_from_x, 384.0);
    }

    #[test]
    fn corner_mask_follows_direction() {
        let bottom = DrawerCornerMask::for_direction(DrawerDirection::Bottom);
        assert!(bottom.top_left && bottom.top_right);
        assert!(!bottom.bottom_left && !bottom.bottom_right);

        let floating = drawer_corner_mask(DrawerDirection::Bottom, &drawer_recipe(StyleId::Maia));
        assert!(floating.top_left && floating.bottom_right);
    }

    #[test]
    fn direction_helpers() {
        assert!(DrawerDirection::Bottom.shows_handle());
        assert!(DrawerDirection::Bottom.centers_header());
        assert!(DrawerDirection::Right.is_vertical_edge());
        assert!(!DrawerDirection::Right.shows_handle());
    }
}
