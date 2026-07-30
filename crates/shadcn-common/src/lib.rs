//! Shared shadcn design tokens for iced-shadcn and egui-shadcn.
//!
//! Built on [`twill_core`] — no iced/egui types. Backends adapt via twill-iced / twill-egui.

#![forbid(unsafe_code)]

pub mod collection_navigation;
pub mod color;
pub mod date_time;
pub mod floating;
#[cfg(feature = "fonts")]
pub mod fonts;
pub mod icons;
pub mod interaction_keys;
pub mod radius;
pub mod recipes;
pub mod style;
pub mod theme;
pub mod transition;
pub mod typography;
pub mod value_mapping;

mod generated;

pub use collection_navigation::{first_enabled_index, last_enabled_index, step_index};
pub use color::{AccentColor, BaseColor, OklchColor, ThemeMode};
pub use date_time::{
    DateDefaultConfig, DateGranularity, DateParts, DateTimeError, DateTimeParts, DateValue,
    TimeDefaultConfig, TimeGranularity, TimeParts, clamp_date_value, default_date_value,
    default_time_value, parse_date, parse_date_time, parse_like_reference, parse_time,
    truncate_date_value,
};
pub use floating::{
    FloatingAlign, FloatingConfig, FloatingPadding, FloatingPlacement, FloatingRect, FloatingSide,
    FloatingSticky, FloatingStrategy, FloatingUpdateStrategy, compute_floating,
};
pub use icons::{IconName, IconSet};
pub use interaction_keys::{Direction, NavAction, NavKey, Orientation, resolve_nav_action};
pub use radius::{RadiusId, RadiusScale};
pub use recipes::{
    BadgeRecipe, ButtonSizeRecipe, ButtonTypeRecipe, ComponentRadius, ControlSize, FontWeight,
    KbdRecipe, LabelContext, LabelRecipe, NATIVE_SELECT_DISABLED_OPACITY,
    NATIVE_SELECT_MENU_GROUP_INDENT_PX, NATIVE_SELECT_MENU_ITEM_PAD_X_PX,
    NATIVE_SELECT_MENU_ITEM_PAD_Y_PX, NATIVE_SELECT_MENU_MAX_HEIGHT_PX, NativeSelectRecipe,
    ProgressRecipe, RadioCheckedFill, RadioGroupRecipe, RadioSurface, SkeletonRecipe, SliderRecipe,
    SliderThumbBorder, SliderThumbFill, SliderTrackSurface, SwitchRecipe, SwitchSizeRecipe,
    TOOLTIP_ANIMATION_MS, TOOLTIP_SLIDE_PX, TOOLTIP_ZOOM_FROM, ToggleRecipe, ToggleSizeRecipe,
    TooltipRecipe, TypeRecipe, badge_recipe, button_size, button_type, kbd_recipe, label_recipe,
    native_select_recipe, progress_recipe, radio_group_recipe, skeleton_default_radius,
    skeleton_recipe, slider_recipe, switch_recipe, switch_size, toggle_recipe, toggle_size,
    tooltip_recipe,
};
pub use style::{StyleId, StylePack};
pub use theme::{ResolvedTheme, SemanticThemeTable};
pub use transition::{Easing, TransitionValue};
pub use typography::{FontHeading, FontId, FontPack};
pub use value_mapping::{closest_index, fraction, snap, snapped_fraction, value_at_fraction};
