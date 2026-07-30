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
pub mod selection;
pub mod style;
pub mod theme;
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
    BadgeRecipe, ButtonSizeRecipe, ButtonTypeRecipe, ComponentRadius, ControlSize, FontWeight,
    KbdRecipe, LabelContext, LabelRecipe, NATIVE_SELECT_DISABLED_OPACITY,
    NATIVE_SELECT_MENU_GROUP_INDENT_PX, NATIVE_SELECT_MENU_ITEM_PAD_X_PX,
    NATIVE_SELECT_MENU_ITEM_PAD_Y_PX, NATIVE_SELECT_MENU_MAX_HEIGHT_PX, NativeSelectRecipe,
    POPOVER_ANIMATION_MS, POPOVER_SLIDE_PX, POPOVER_WIDTH_PX, POPOVER_ZOOM_FROM, PopoverRecipe,
    PopoverShadow, ProgressRecipe, RadioCheckedFill, RadioGroupRecipe, RadioSurface,
    SkeletonRecipe, SliderRecipe, SliderThumbBorder, SliderThumbFill, SliderTrackSurface,
    SwitchRecipe, SwitchSizeRecipe, TOOLTIP_ANIMATION_MS, TOOLTIP_SLIDE_PX, TOOLTIP_ZOOM_FROM,
    ToggleRecipe, ToggleSizeRecipe, TooltipRecipe, TypeRecipe, badge_recipe, button_size,
    button_type, kbd_recipe, label_recipe, native_select_recipe, popover_recipe, progress_recipe,
    radio_group_recipe, skeleton_default_radius, skeleton_recipe, slider_recipe, switch_recipe,
    switch_size, toggle_recipe, toggle_size, tooltip_recipe,
};
pub use selection::{Selection, SelectionMode};
pub use style::{StyleId, StylePack};
pub use theme::{ResolvedTheme, SemanticThemeTable};
pub use transition::{Easing, TransitionValue};
pub use typography::{FontHeading, FontId, FontPack};
pub use value_mapping::{
    ValueRange, closest_index, decrement, finite_or_zero, fraction, increment, max_value_at_index,
    min_value_at_index, modulo, round_to_step_precision, set_value_at_index, snap, snap_value_to_step,
    snapped_fraction, transform_value, value_at_fraction, value_ranges, wrap,
};
