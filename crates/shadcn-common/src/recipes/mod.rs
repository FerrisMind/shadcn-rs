//! Backend-agnostic component recipes derived from shadcn-svelte style CSS.
//!
//! These tokens intentionally avoid iced/egui types so both GUI backends can
//! share the same StyleId tables. Backends map [`FontWeight`] / [`ComponentRadius`]
//! onto their native font and radius APIs.

mod alert_dialog;
mod badge;
mod button;
mod context_menu;
mod dialog;
mod drawer;
mod dropdown_menu;
mod hover_card;
mod kbd;
mod label;
mod menubar;
mod native_select;
mod navigation_menu;
mod popover;
mod progress;
mod radio_group;
mod select;
mod sheet;
mod skeleton;
mod slider;
mod switch;
mod toggle;
mod tooltip;

pub use alert_dialog::{AlertDialogRecipe, alert_dialog_recipe};
pub use badge::{BadgeRecipe, badge_recipe};
pub use button::{ButtonSizeRecipe, ButtonTypeRecipe, ControlSize, button_size, button_type};
pub use context_menu::{
    CONTEXT_MENU_ANIMATION_MS, CONTEXT_MENU_CONTENT_MAX_HEIGHT_PX,
    CONTEXT_MENU_DESTRUCTIVE_FOCUS_ALPHA, CONTEXT_MENU_DESTRUCTIVE_FOCUS_ALPHA_DARK,
    CONTEXT_MENU_DISABLED_OPACITY, CONTEXT_MENU_FLIP_SLACK_PX, CONTEXT_MENU_SIDE_OFFSET_PX,
    CONTEXT_MENU_SLIDE_PX, CONTEXT_MENU_ZOOM_FROM, ContextMenuRecipe, context_menu_recipe,
};
pub use dialog::{
    DIALOG_ANIMATION_MS, DIALOG_CLOSE_ICON_PX, DIALOG_CLOSE_SIZE_PX, DIALOG_MARGIN_PX,
    DIALOG_ZOOM_FROM, DialogRecipe, dialog_recipe,
};
pub use hover_card::{
    HOVER_CARD_ANIMATION_MS, HOVER_CARD_CLOSE_DELAY_MS, HOVER_CARD_OPEN_DELAY_MS,
    HOVER_CARD_SLIDE_PX, HOVER_CARD_ZOOM_FROM, HoverCardRecipe, hover_card_recipe,
};
pub use kbd::{KbdRecipe, kbd_recipe};
pub use label::{LabelContext, LabelRecipe, label_recipe};
pub use menubar::{
    MENUBAR_ALIGN_OFFSET_PX, MENUBAR_ANIMATION_MS, MENUBAR_CONTENT_MAX_HEIGHT_PX,
    MENUBAR_DESTRUCTIVE_FOCUS_ALPHA, MENUBAR_DESTRUCTIVE_FOCUS_ALPHA_DARK,
    MENUBAR_DISABLED_OPACITY, MENUBAR_SIDE_OFFSET_PX, MENUBAR_SLIDE_PX, MENUBAR_ZOOM_FROM,
    MenubarRecipe, menubar_recipe,
};
pub use native_select::{
    NATIVE_SELECT_DISABLED_OPACITY, NATIVE_SELECT_MENU_GROUP_INDENT_PX,
    NATIVE_SELECT_MENU_ITEM_PAD_X_PX, NATIVE_SELECT_MENU_ITEM_PAD_Y_PX,
    NATIVE_SELECT_MENU_MAX_HEIGHT_PX, NativeSelectRecipe, native_select_recipe,
};
pub use navigation_menu::{
    NAVIGATION_MENU_CHEVRON_ROTATE_MS, NAVIGATION_MENU_CHEVRON_SIZE_PX,
    NAVIGATION_MENU_CLOSE_DELAY_MS, NAVIGATION_MENU_CONTENT_ANIM_MS,
    NAVIGATION_MENU_CONTENT_ZOOM_FROM, NAVIGATION_MENU_DELAY_DURATION_MS,
    NAVIGATION_MENU_DISABLED_OPACITY, NAVIGATION_MENU_FAST_DELAY_MS,
    NAVIGATION_MENU_INDICATOR_ANIM_MS, NAVIGATION_MENU_MOTION_ANIM_MS,
    NAVIGATION_MENU_MOTION_DISTANCE_CONTENT_PX, NAVIGATION_MENU_MOTION_DISTANCE_VIEWPORT_PX,
    NAVIGATION_MENU_OPEN_MUTED_ALPHA, NAVIGATION_MENU_SIDE_OFFSET_PX,
    NAVIGATION_MENU_SKIP_DELAY_DURATION_MS, NAVIGATION_MENU_VIEWPORT_ANIM_MS,
    NAVIGATION_MENU_VIEWPORT_PAD_PX, NAVIGATION_MENU_VIEWPORT_ZOOM_FROM, NavigationMenuRecipe,
    navigation_menu_recipe,
};
pub use popover::{
    POPOVER_ANIMATION_MS, POPOVER_SLIDE_PX, POPOVER_WIDTH_PX, POPOVER_ZOOM_FROM, PopoverRecipe,
    PopoverShadow, popover_recipe,
};
pub use progress::{ProgressRecipe, progress_recipe};
pub use radio_group::{RadioCheckedFill, RadioGroupRecipe, RadioSurface, radio_group_recipe};
pub use select::{
    SELECT_ANIMATION_MS, SELECT_CONTENT_MAX_HEIGHT_PX, SELECT_DISABLED_OPACITY,
    SELECT_SIDE_OFFSET_PX, SELECT_SLIDE_PX, SELECT_ZOOM_FROM, SelectRecipe, select_recipe,
};
pub use skeleton::{SkeletonRecipe, skeleton_default_radius, skeleton_recipe};
pub use slider::{
    SliderRecipe, SliderThumbBorder, SliderThumbFill, SliderTrackSurface, slider_recipe,
};
pub use switch::{SwitchRecipe, SwitchSizeRecipe, switch_recipe, switch_size};
pub use toggle::{ToggleRecipe, ToggleSizeRecipe, toggle_recipe, toggle_size};
pub use tooltip::{
    TOOLTIP_ANIMATION_MS, TOOLTIP_SLIDE_PX, TOOLTIP_ZOOM_FROM, TooltipRecipe, tooltip_recipe,
};

/// Backend-agnostic font weight matching CSS `font-normal` / `font-medium` / ….
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum FontWeight {
    Thin,
    ExtraLight,
    Light,
    #[default]
    Normal,
    Medium,
    Semibold,
    Bold,
    ExtraBold,
    Black,
}

/// Corner-radius intent from style CSS (`rounded-none` / `rounded-md` / …).
///
/// Backends resolve this against [`crate::StylePack`]'s twill radius slots
/// (or a pill / zero) — the enum itself stays unitless.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum ComponentRadius {
    /// `rounded-none` / locked styles.
    None,
    /// `rounded-sm`.
    Sm,
    /// `rounded-md` (typical control default).
    #[default]
    Md,
    /// `rounded-lg`.
    Lg,
    /// `rounded-xl` / `--radius-xl` = `calc(var(--radius) + 4px)`.
    Xl,
    /// `rounded-2xl` / `--radius-2xl` = `calc(var(--radius) + 8px)`.
    ///
    /// Named `S2xl` (not `2xl`) because Rust identifiers cannot start with a digit.
    S2xl,
    /// `rounded-3xl` / `--radius-3xl` = `calc(var(--radius) + 12px)` — soft panels.
    S3xl,
    /// `rounded-4xl` / `--radius-4xl` = `calc(var(--radius) + 16px)` — large soft surfaces.
    S4xl,
    /// Pill / `rounded-full` treated as fully rounded.
    Full,
}

/// Shared typography recipe (size, weight, casing, tracking, line-height).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TypeRecipe {
    /// Font size in CSS px (`text-sm` → 14, `text-xs` → 12, `0.625rem` → 10).
    pub size_px: f32,
    pub weight: FontWeight,
    pub uppercase: bool,
    /// Letter-spacing in `em` (`tracking-wide` → 0.025, `tracking-widest` → 0.1).
    pub tracking_em: f32,
    /// Absolute line height in px.
    pub line_height_px: f32,
}

impl TypeRecipe {
    /// Letter-spacing converted to absolute px for the current size.
    pub const fn tracking_px(self) -> f32 {
        self.size_px * self.tracking_em
    }
}
pub use drawer::{
    DRAWER_ANIMATION_MS, DRAWER_EDGE_INSET_PX, DRAWER_HANDLE_HEIGHT_COMPACT_PX,
    DRAWER_HANDLE_HEIGHT_PX, DRAWER_HANDLE_MARGIN_TOP_PX, DRAWER_HANDLE_WIDTH_PX,
    DRAWER_MAX_HEIGHT_FRACTION, DRAWER_MAX_WIDTH_PX, DRAWER_SIDE_WIDTH_FRACTION, DrawerCornerMask,
    DrawerDirection, DrawerPanelMetrics, DrawerRecipe, drawer_corner_mask, drawer_panel_metrics,
    drawer_recipe,
};
pub use dropdown_menu::{
    DROPDOWN_MENU_ANIMATION_MS, DROPDOWN_MENU_CONTENT_MAX_HEIGHT_PX,
    DROPDOWN_MENU_DESTRUCTIVE_FOCUS_ALPHA, DROPDOWN_MENU_DESTRUCTIVE_FOCUS_ALPHA_DARK,
    DROPDOWN_MENU_DISABLED_OPACITY, DROPDOWN_MENU_SIDE_OFFSET_PX, DROPDOWN_MENU_SLIDE_PX,
    DROPDOWN_MENU_ZOOM_FROM, DropdownMenuRecipe, MENU_SUB_SIDE_OFFSET_PX, MenuActivateKind,
    MenuItemVariant, dropdown_menu_recipe,
};
pub use sheet::{
    SHEET_ANIMATION_MS, SHEET_CLOSE_ICON_PX, SHEET_CLOSE_SIZE_PX, SHEET_MAX_WIDTH_PX,
    SHEET_SIDE_WIDTH_FRACTION, SHEET_SLIDE_PX, SheetPanelMetrics, SheetRecipe, SheetSide,
    sheet_panel_metrics, sheet_recipe,
};
