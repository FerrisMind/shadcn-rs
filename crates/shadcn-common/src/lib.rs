//! Shared shadcn design tokens for iced-shadcn and egui-shadcn.
//!
//! Built on [`twill_core`] — no iced/egui types. Backends adapt via twill-iced / twill-egui.
//!
//! Interaction helpers (selection, pagination, presence, color-space math) are
//! ports of Zag pure utilities so egui and iced share one behaviour layer.

#![forbid(unsafe_code)]

pub mod collection_navigation;
pub mod color;
pub mod color_space;
pub mod date_time;
pub mod floating;
#[cfg(feature = "fonts")]
pub mod fonts;
pub mod icons;
pub mod interaction_keys;
pub mod pagination;
pub mod presence;
pub mod radius;
pub mod recipes;
pub mod select_value;
pub mod selection;
pub mod style;
pub mod theme;
pub mod toast;
pub mod transition;
pub mod typography;
pub mod value_mapping;

mod generated;

pub use collection_navigation::{first_enabled_index, last_enabled_index, step_index};
pub use color::{AccentColor, BaseColor, OklchColor, ThemeMode};
pub use color_space::{Hsba, Hsla, Rgba};
pub use date_time::{
    DateDefaultConfig, DateGranularity, DateParts, DateTimeError, DateTimeParts, DateValue,
    TimeDefaultConfig, TimeGranularity, TimeParts, add_days, add_months, clamp_date_parts,
    clamp_date_value, days_in_month_of, days_in_week, default_date_value, default_time_value,
    month_days, parse_date, parse_date_time, parse_like_reference, parse_time, start_of_month,
    start_of_week, truncate_date_value, weekday_sunday,
};
pub use floating::{
    FloatingAlign, FloatingConfig, FloatingPadding, FloatingPlacement, FloatingRect, FloatingSide,
    FloatingSticky, FloatingStrategy, FloatingUpdateStrategy, compute_floating,
};
pub use icons::{IconName, IconSet};
pub use interaction_keys::{Direction, NavAction, NavKey, Orientation, resolve_nav_action};
pub use pagination::{
    DEFAULT_BOUNDARY_COUNT, DEFAULT_SIBLING_COUNT, PageContext, PaginationItem, page_items,
    total_pages,
};
pub use presence::{Presence, PresenceEvent, PresenceState};
pub use radius::{RadiusId, RadiusScale};
pub use recipes::{
    AlertDialogRecipe, BadgeRecipe, ButtonSizeRecipe, ButtonTypeRecipe, CONTEXT_MENU_ANIMATION_MS,
    CONTEXT_MENU_CONTENT_MAX_HEIGHT_PX, CONTEXT_MENU_DESTRUCTIVE_FOCUS_ALPHA,
    CONTEXT_MENU_DESTRUCTIVE_FOCUS_ALPHA_DARK, CONTEXT_MENU_DISABLED_OPACITY,
    CONTEXT_MENU_FLIP_SLACK_PX, CONTEXT_MENU_SIDE_OFFSET_PX, CONTEXT_MENU_SLIDE_PX,
    CONTEXT_MENU_ZOOM_FROM, ComponentRadius, ContextMenuRecipe, ControlSize, DIALOG_ANIMATION_MS,
    DIALOG_CLOSE_ICON_PX, DIALOG_CLOSE_SIZE_PX, DIALOG_MARGIN_PX, DIALOG_ZOOM_FROM,
    DRAWER_ANIMATION_MS, DRAWER_EDGE_INSET_PX, DRAWER_HANDLE_HEIGHT_COMPACT_PX,
    DRAWER_HANDLE_HEIGHT_PX, DRAWER_HANDLE_MARGIN_TOP_PX, DRAWER_HANDLE_WIDTH_PX,
    DRAWER_MAX_HEIGHT_FRACTION, DRAWER_MAX_WIDTH_PX, DRAWER_SIDE_WIDTH_FRACTION,
    DROPDOWN_MENU_ANIMATION_MS, DROPDOWN_MENU_CONTENT_MAX_HEIGHT_PX,
    DROPDOWN_MENU_DESTRUCTIVE_FOCUS_ALPHA, DROPDOWN_MENU_DESTRUCTIVE_FOCUS_ALPHA_DARK,
    DROPDOWN_MENU_DISABLED_OPACITY, DROPDOWN_MENU_SIDE_OFFSET_PX, DROPDOWN_MENU_SLIDE_PX,
    DROPDOWN_MENU_ZOOM_FROM, DialogRecipe, DrawerCornerMask, DrawerDirection, DrawerPanelMetrics,
    DrawerRecipe, DropdownMenuRecipe, FontWeight, HOVER_CARD_ANIMATION_MS,
    HOVER_CARD_CLOSE_DELAY_MS, HOVER_CARD_OPEN_DELAY_MS, HOVER_CARD_SLIDE_PX, HOVER_CARD_ZOOM_FROM,
    HoverCardRecipe, KbdRecipe, LabelContext, LabelRecipe, MenuActivateKind, MenuItemVariant,
    NATIVE_SELECT_DISABLED_OPACITY, NATIVE_SELECT_MENU_GROUP_INDENT_PX,
    NATIVE_SELECT_MENU_ITEM_PAD_X_PX, NATIVE_SELECT_MENU_ITEM_PAD_Y_PX,
    NATIVE_SELECT_MENU_MAX_HEIGHT_PX, NativeSelectRecipe, POPOVER_ANIMATION_MS, POPOVER_SLIDE_PX,
    POPOVER_WIDTH_PX, POPOVER_ZOOM_FROM, PopoverRecipe, PopoverShadow, ProgressRecipe,
    RadioCheckedFill, RadioGroupRecipe, RadioSurface, SELECT_ANIMATION_MS,
    SELECT_CONTENT_MAX_HEIGHT_PX, SELECT_DISABLED_OPACITY, SELECT_SIDE_OFFSET_PX, SELECT_SLIDE_PX,
    SELECT_ZOOM_FROM, SHEET_ANIMATION_MS, SHEET_CLOSE_ICON_PX, SHEET_CLOSE_SIZE_PX,
    SHEET_MAX_WIDTH_PX, SHEET_SIDE_WIDTH_FRACTION, SHEET_SLIDE_PX, SelectRecipe, SheetPanelMetrics,
    SheetRecipe, SheetSide, SkeletonRecipe, SliderRecipe, SliderThumbBorder, SliderThumbFill,
    SliderTrackSurface, SwitchRecipe, SwitchSizeRecipe, TOOLTIP_ANIMATION_MS, TOOLTIP_SLIDE_PX,
    TOOLTIP_ZOOM_FROM, ToggleRecipe, ToggleSizeRecipe, TooltipRecipe, TypeRecipe,
    alert_dialog_recipe, badge_recipe, button_size, button_type, context_menu_recipe,
    dialog_recipe, drawer_corner_mask, drawer_panel_metrics, drawer_recipe, dropdown_menu_recipe,
    hover_card_recipe, kbd_recipe, label_recipe, native_select_recipe, popover_recipe,
    progress_recipe, radio_group_recipe, select_recipe, sheet_panel_metrics, sheet_recipe,
    skeleton_default_radius, skeleton_recipe, slider_recipe, switch_recipe, switch_size,
    toggle_recipe, toggle_size, tooltip_recipe,
};
pub use select_value::{
    SelectMode, multiple_selection_label, next_multiple_values, next_single_value,
};
pub use selection::{Selection, SelectionMode};
pub use style::{StyleId, StylePack};
pub use theme::{ResolvedTheme, SemanticThemeTable};
pub use transition::{Easing, TransitionValue};
pub use typography::{FontHeading, FontId, FontPack};
pub use value_mapping::{
    ValueRange, closest_index, decrement, finite_or_zero, fraction, increment, max_value_at_index,
    min_value_at_index, modulo, round_to_step_precision, set_value_at_index, snap,
    snap_value_to_step, snapped_fraction, transform_value, value_at_fraction, value_ranges, wrap,
};
