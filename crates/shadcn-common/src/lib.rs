//! Shared shadcn design tokens for iced-shadcn and egui-shadcn.
//!
//! Built on [`twill_core`] — no iced/egui types. Backends adapt via twill-iced / twill-egui.
//!
//! Interaction helpers (selection, pagination, presence, color-space math) are
//! ports of Zag pure utilities so egui and iced share one behaviour layer.

#![forbid(unsafe_code)]

pub mod calendar;
pub mod carousel;
pub mod chart;
pub mod collection_navigation;
pub mod color;
pub mod color_space;
pub mod command;
pub mod data_table;
pub mod date_picker;
pub mod date_time;
pub mod file_drop_zone;
pub mod floating;
#[cfg(feature = "fonts")]
pub mod fonts;
pub mod form;
#[cfg(feature = "syntax")]
pub mod highlight;
pub mod icons;
pub mod interaction_keys;
pub mod meter;
pub mod navigation_menu;
pub mod pagination;
pub mod password;
pub mod phone_input;
pub mod presence;
pub mod radius;
pub mod recipes;
pub mod select_value;
pub mod selection;
pub mod sidebar;
pub mod star_rating;
pub mod style;
pub mod syntax;
pub mod theme;
pub mod toast;
pub mod transition;
pub mod tree;
pub mod typography;
pub mod value_mapping;

mod generated;

pub use calendar::{
    CALENDAR_CELL_SIZE_PX, CALENDAR_DISABLED_OPACITY, CALENDAR_DROPDOWN_CHEVRON_PX,
    CALENDAR_HEADER_GAP_PX, CALENDAR_HOVER_ACCENT_ALPHA, CALENDAR_MONTHS_GAP_PX,
    CALENDAR_NAV_ICON_PX, CALENDAR_PADDING_PX, CALENDAR_TEXT_PX, CALENDAR_WEEK_ROW_GAP_PX,
    CALENDAR_WEEKDAY_TEXT_PX, CalendarCaptionLayout, CalendarDayState, CalendarMonthFormat,
    CalendarPick, CalendarSelection, CalendarWeekdayFormat, CalendarYearFormat, DateRange,
    RangeCalendarPick, RangeDayPosition, calendar_date_in_bounds, calendar_day_pick,
    calendar_default_years, calendar_month_grid, calendar_month_name, calendar_nav_target,
    calendar_next_disabled, calendar_prev_disabled, calendar_today_utc, calendar_visible_months,
    calendar_weekday_name, calendar_weekdays, calendar_year_name, range_calendar_day_pick,
    range_day_position, range_days_valid, range_highlight,
};
pub use carousel::{
    CAROUSEL_ANIMATION_MS, CAROUSEL_AUTOPLAY_DELAY_MS, CAROUSEL_CONTROL_OFFSET_PX,
    CAROUSEL_DRAG_THRESHOLD_FRACTION, CAROUSEL_GAP_PX, CarouselAlign, CarouselLayout,
    carousel_can_scroll_next, carousel_can_scroll_prev, carousel_drag_steps, carousel_loop_target,
    carousel_nearest_snap, carousel_next_snap, carousel_previous_snap, carousel_slot_positions,
    carousel_snap_offsets, carousel_snap_offsets_weighted, carousel_step_snap,
    carousel_wrap_position,
};
pub use chart::{
    CHART_AREA_FILL_OPACITY, CHART_ASPECT_RATIO, CHART_BAND_PADDING_FRACTION,
    CHART_GROUP_PADDING_FRACTION, CHART_HIGHLIGHT_POINT_RADIUS_PX, CHART_MOTION_MS,
    CHART_TICK_COUNT, CHART_TOOLTIP_MIN_WIDTH_PX, ChartCubicSegment, ChartPieSlice,
    chart_band_slots, chart_format_value, chart_group_slots, chart_linear_fraction,
    chart_natural_curve, chart_nearest_center, chart_nice_domain, chart_nice_ticks, chart_pie_hit,
    chart_pie_slices, chart_stack_spans, chart_value_extent,
};
pub use collection_navigation::{first_enabled_index, last_enabled_index, step_index};
pub use color::{AccentColor, BaseColor, OklchColor, ThemeMode};
pub use color_space::{Hsba, Hsla, Rgba};
pub use command::{
    CommandFilter, command_matches, default_command_filter, first_selectable_index, fuzzy_score,
    last_selectable_index, step_selectable_index,
};
pub use date_picker::{
    DATE_PICKER_RANGE_TRIGGER_WIDTH_PX, DATE_PICKER_TRIGGER_WIDTH_PX, DatePickerIconPosition,
    DatePickerMode, format_date_long, format_date_medium, format_date_range,
};
pub use date_time::{
    DateDefaultConfig, DateGranularity, DateParts, DateTimeError, DateTimeParts, DateValue,
    TimeDefaultConfig, TimeGranularity, TimeParts, add_days, add_months, clamp_date_parts,
    clamp_date_value, days_in_month_of, days_in_week, default_date_value, default_time_value,
    month_days, parse_date, parse_date_time, parse_like_reference, parse_time, start_of_month,
    start_of_week, truncate_date_value, weekday_sunday,
};
pub use file_drop_zone::{
    ACCEPT_AUDIO, ACCEPT_IMAGE, ACCEPT_VIDEO, BYTE, DEFAULT_TRIGGER_LABEL, FileCandidate,
    FileDropZoneConfig, FileRejectedReason, GIGABYTE, KILOBYTE, MEGABYTE, accept_matches,
    accepts_multiple, can_upload as file_drop_zone_can_upload,
    default_hint as file_drop_zone_default_hint, display_size, guess_mime,
    partition_candidates as file_drop_zone_partition_candidates, should_accept_file,
};
pub use floating::{
    FloatingAlign, FloatingConfig, FloatingPadding, FloatingPlacement, FloatingRect, FloatingSide,
    FloatingSticky, FloatingStrategy, FloatingUpdateStrategy, compute_floating,
};
pub use form::{
    FieldConstraints, FieldValue, FormFieldIds, FormFieldState, FormState, ValidationMode,
    Validator, compose, email, max_length, min_length, none, pattern, required,
};
#[cfg(feature = "syntax")]
pub use highlight::highlight_code;
pub use icons::{IconName, IconSet};
pub use interaction_keys::{Direction, NavAction, NavKey, Orientation, resolve_nav_action};
pub use meter::{
    MeterConfig, MeterFillTone, clamp_meter_value, meter_fill_tone, meter_ratio, meter_value_label,
    sanitize_bounds as sanitize_meter_bounds, sanitize_scalar as sanitize_meter_scalar,
};
pub use navigation_menu::{
    NavRect, NavigationMenuAlign, NavigationMenuSide, NavigationMenuTiming, indicator_diamond,
    motion_duration_ms, motion_offset_x, place_content as place_navigation_menu_content,
    place_viewport as place_navigation_menu_viewport,
};
pub use pagination::{
    DEFAULT_BOUNDARY_COUNT, DEFAULT_SIBLING_COUNT, PageContext, PaginationItem, page_items,
    total_pages,
};
pub use password::{
    PasswordAction, PasswordScore, PasswordState, PasswordStrength, estimate_password_strength,
    password_reduce,
};
pub use phone_input::{
    CountryCode, DetailedPhoneValue, PhoneCountry, PhoneInputError, PhoneInputOptions,
    apply_country_change, apply_input_change, auto_placeholder, default_country_order, flag_emoji,
    format_phone_value, is_phone_valid, normalize_to_e164, parse_phone_input, phone_countries,
    phone_country, sort_countries,
};
pub use presence::{Presence, PresenceEvent, PresenceState};
pub use radius::{RadiusId, RadiusScale};
pub use recipes::{
    AlertDialogRecipe, BadgeRecipe, ButtonSizeRecipe, ButtonTypeRecipe,
    COMMAND_DIALOG_VERTICAL_ANCHOR, COMMAND_DISABLED_OPACITY, COMMAND_INPUT_ICON_OPACITY,
    COMMAND_LIST_MAX_HEIGHT_PX, CONTEXT_MENU_ANIMATION_MS, CONTEXT_MENU_CONTENT_MAX_HEIGHT_PX,
    CONTEXT_MENU_DESTRUCTIVE_FOCUS_ALPHA, CONTEXT_MENU_DESTRUCTIVE_FOCUS_ALPHA_DARK,
    CONTEXT_MENU_DISABLED_OPACITY, CONTEXT_MENU_FLIP_SLACK_PX, CONTEXT_MENU_SIDE_OFFSET_PX,
    CONTEXT_MENU_SLIDE_PX, CONTEXT_MENU_ZOOM_FROM, CalendarRecipe, CarouselRecipe, ChartRecipe,
    CheckboxRecipe, CodeRecipe, CommandRecipe, ComponentRadius, ContextMenuRecipe, ControlSize,
    DARK_INVALID_BORDER_ALPHA, DIALOG_ANIMATION_MS, DIALOG_CLOSE_ICON_PX, DIALOG_CLOSE_SIZE_PX,
    DIALOG_MARGIN_PX, DIALOG_ZOOM_FROM, DISABLED_OPACITY, DRAWER_ANIMATION_MS,
    DRAWER_EDGE_INSET_PX, DRAWER_HANDLE_HEIGHT_COMPACT_PX, DRAWER_HANDLE_HEIGHT_PX,
    DRAWER_HANDLE_MARGIN_TOP_PX, DRAWER_HANDLE_WIDTH_PX, DRAWER_MAX_HEIGHT_FRACTION,
    DRAWER_MAX_WIDTH_PX, DRAWER_SIDE_WIDTH_FRACTION, DROPDOWN_MENU_ANIMATION_MS,
    DROPDOWN_MENU_CONTENT_MAX_HEIGHT_PX, DROPDOWN_MENU_DESTRUCTIVE_FOCUS_ALPHA,
    DROPDOWN_MENU_DESTRUCTIVE_FOCUS_ALPHA_DARK, DROPDOWN_MENU_DISABLED_OPACITY,
    DROPDOWN_MENU_SIDE_OFFSET_PX, DROPDOWN_MENU_SLIDE_PX, DROPDOWN_MENU_ZOOM_FROM, DialogRecipe,
    DrawerCornerMask, DrawerDirection, DrawerPanelMetrics, DrawerRecipe, DropdownMenuRecipe,
    FILE_DROP_ZONE_BORDER_WIDTH_PX, FILE_DROP_ZONE_DISABLED_OPACITY, FILE_DROP_ZONE_GAP_PX,
    FILE_DROP_ZONE_HEIGHT_PX, FILE_DROP_ZONE_HINT_FOREGROUND_ALPHA,
    FILE_DROP_ZONE_HOVER_ACCENT_ALPHA, FILE_DROP_ZONE_ICON_CIRCLE_PX, FILE_DROP_ZONE_ICON_PX,
    FILE_DROP_ZONE_ICON_STROKE_VIEWBOX, FILE_DROP_ZONE_ICON_VIEWBOX, FILE_DROP_ZONE_PADDING_PX,
    FILE_DROP_ZONE_TEXT_GAP_PX, FileDropZoneRecipe, FontWeight, FormRecipe,
    HOVER_CARD_ANIMATION_MS, HOVER_CARD_CLOSE_DELAY_MS, HOVER_CARD_OPEN_DELAY_MS,
    HOVER_CARD_SLIDE_PX, HOVER_CARD_ZOOM_FROM, HoverCardRecipe, INVALID_RING_ALPHA_DARK,
    INVALID_RING_ALPHA_LIGHT, KbdRecipe, LabelContext, LabelRecipe, MENU_SUB_SIDE_OFFSET_PX,
    MENUBAR_ALIGN_OFFSET_PX, MENUBAR_ANIMATION_MS, MENUBAR_CONTENT_MAX_HEIGHT_PX,
    MENUBAR_DESTRUCTIVE_FOCUS_ALPHA, MENUBAR_DESTRUCTIVE_FOCUS_ALPHA_DARK,
    MENUBAR_DISABLED_OPACITY, MENUBAR_SIDE_OFFSET_PX, MENUBAR_SLIDE_PX, MENUBAR_ZOOM_FROM,
    METER_HEIGHT_PX, METER_TRACK_ALPHA, METER_TRANSITION_MS, METER_WARNING_RATIO, MIN_HEIGHT_PX,
    MenuActivateKind, MenuItemVariant, MenubarRecipe, MeterRecipe, NATIVE_SELECT_DISABLED_OPACITY,
    NATIVE_SELECT_MENU_GROUP_INDENT_PX, NATIVE_SELECT_MENU_ITEM_PAD_X_PX,
    NATIVE_SELECT_MENU_ITEM_PAD_Y_PX, NATIVE_SELECT_MENU_MAX_HEIGHT_PX,
    NAVIGATION_MENU_CHEVRON_ROTATE_MS, NAVIGATION_MENU_CHEVRON_SIZE_PX,
    NAVIGATION_MENU_CLOSE_DELAY_MS, NAVIGATION_MENU_CONTENT_ANIM_MS,
    NAVIGATION_MENU_CONTENT_ZOOM_FROM, NAVIGATION_MENU_DELAY_DURATION_MS,
    NAVIGATION_MENU_DISABLED_OPACITY, NAVIGATION_MENU_FAST_DELAY_MS,
    NAVIGATION_MENU_INDICATOR_ANIM_MS, NAVIGATION_MENU_MOTION_ANIM_MS,
    NAVIGATION_MENU_MOTION_DISTANCE_CONTENT_PX, NAVIGATION_MENU_MOTION_DISTANCE_VIEWPORT_PX,
    NAVIGATION_MENU_OPEN_MUTED_ALPHA, NAVIGATION_MENU_SIDE_OFFSET_PX,
    NAVIGATION_MENU_SKIP_DELAY_DURATION_MS, NAVIGATION_MENU_VIEWPORT_ANIM_MS,
    NAVIGATION_MENU_VIEWPORT_PAD_PX, NAVIGATION_MENU_VIEWPORT_ZOOM_FROM, NativeSelectRecipe,
    NavigationMenuRecipe, PASSWORD_ACTION_ICON_PX, PASSWORD_ACTION_SIZE_PX,
    PASSWORD_DEFAULT_MIN_SCORE, PASSWORD_END_PAD_BOTH_PX, PASSWORD_END_PAD_ONE_PX,
    PASSWORD_ROOT_GAP_PX, PASSWORD_SCORE_GREEN_RGB, PASSWORD_SCORE_RED_RGB,
    PASSWORD_SCORE_YELLOW_RGB, PASSWORD_STRENGTH_GAP_PX, PASSWORD_STRENGTH_HEIGHT_PX,
    PASSWORD_STRENGTH_RING_PX, PASSWORD_STRENGTH_SEGMENTS, PASSWORD_STRENGTH_TRANSITION_MS,
    PASSWORD_TOGGLE_COMPACT_WIDTH_PX, PHONE_INPUT_CHEVRON_SIZE_PX, PHONE_INPUT_DISABLED_OPACITY,
    PHONE_INPUT_FLAG_HEIGHT_PX, PHONE_INPUT_FLAG_WIDTH_PX, PHONE_INPUT_JOINT_OVERLAP_PX,
    PHONE_INPUT_LIST_HEIGHT_PX, PHONE_INPUT_POPOVER_WIDTH_PX, PHONE_INPUT_TRIGGER_GAP_PX,
    PHONE_INPUT_TRIGGER_PAD_X_PX, POPOVER_ANIMATION_MS, POPOVER_SLIDE_PX, POPOVER_WIDTH_PX,
    POPOVER_ZOOM_FROM, PasswordRecipe, PhoneInputRecipe, PopoverRecipe, PopoverShadow,
    ProgressRecipe, RadioCheckedFill, RadioGroupRecipe, RadioSurface, SELECT_ANIMATION_MS,
    SELECT_CONTENT_MAX_HEIGHT_PX, SELECT_DISABLED_OPACITY, SELECT_SIDE_OFFSET_PX, SELECT_SLIDE_PX,
    SELECT_ZOOM_FROM, SELECTION_ALPHA, SHEET_ANIMATION_MS, SHEET_CLOSE_ICON_PX,
    SHEET_CLOSE_SIZE_PX, SHEET_MAX_WIDTH_PX, SHEET_SIDE_WIDTH_FRACTION, SHEET_SLIDE_PX,
    SIDEBAR_DISABLED_OPACITY, SIDEBAR_GROUP_LABEL_FG_ALPHA, SIDEBAR_ICON_SIZE_PX,
    SIDEBAR_RAIL_INDICATOR_PX, SIDEBAR_RAIL_WIDTH_PX, SIDEBAR_TRANSITION_MS, STAR_GAP_PX,
    STAR_RATING_DISABLED_OPACITY, STAR_RATING_RING_OFFSET_PX, STAR_RATING_RING_WIDTH_PX,
    STAR_SIZE_PX, STAR_STROKE_VIEWBOX, STAR_VIEWBOX, SelectRecipe, SheetPanelMetrics, SheetRecipe,
    SheetSide, SidebarRecipe, SkeletonRecipe, SliderRecipe, SliderThumbBorder, SliderThumbFill,
    SliderTrackSurface, SnippetRecipe, StarRatingRecipe, SwitchRecipe, SwitchSizeRecipe,
    TOOLTIP_ANIMATION_MS, TOOLTIP_SLIDE_PX, TOOLTIP_ZOOM_FROM, TextareaRecipe, ToggleRecipe,
    ToggleSizeRecipe, TooltipRecipe, TypeRecipe, alert_dialog_recipe, badge_recipe, button_size,
    button_type, calendar_recipe, carousel_recipe, chart_recipe, checkbox_recipe, code_recipe,
    command_recipe, context_menu_recipe, dialog_recipe, drawer_corner_mask, drawer_panel_metrics,
    drawer_recipe, dropdown_menu_recipe, file_drop_zone_recipe, form_recipe, hover_card_recipe,
    kbd_recipe, label_recipe, menubar_recipe, meter_recipe, native_select_recipe,
    navigation_menu_recipe, password_end_padding_px, password_recipe, password_score_rgb,
    phone_input_recipe, popover_recipe, progress_recipe, radio_group_recipe, select_recipe,
    sheet_panel_metrics, sheet_recipe, sidebar_recipe, skeleton_default_radius, skeleton_recipe,
    slider_recipe, snippet_recipe, star_rating_recipe, switch_recipe, switch_size, textarea_recipe,
    toggle_recipe, toggle_size, tooltip_recipe,
};
pub use select_value::{
    SelectMode, multiple_selection_label, next_multiple_values, next_single_value,
};
pub use selection::{Selection, SelectionMode};
pub use sidebar::{
    SIDEBAR_ANIMATION_MS, SIDEBAR_COOKIE_MAX_AGE_SECS, SIDEBAR_COOKIE_NAME,
    SIDEBAR_FLOATING_ICON_EXTRA_PX, SIDEBAR_FLOATING_PAD_PX, SIDEBAR_KEYBOARD_SHORTCUT,
    SIDEBAR_MOBILE_BREAKPOINT_PX, SIDEBAR_WIDTH_ICON_PX, SIDEBAR_WIDTH_MOBILE_PX, SIDEBAR_WIDTH_PX,
    SidebarCollapsible, SidebarController, SidebarDisplayState, SidebarSide, SidebarVariant,
    lerp_sidebar_gap, matches_sidebar_shortcut, sidebar_gap_width, sidebar_panel_width,
};
pub use star_rating::{
    StarRatingConfig, StarRatingItem, StarRatingItemState, StarRatingKey, StarRatingKeyEffect,
    adjust_rating, apply_click, apply_key_effect, aria_valuetext, clamp_rating, display_value,
    hover_preview_value, item_state, items as star_rating_items,
    items_into as star_rating_items_into, key_delta as star_rating_key_delta, rating_from_pointer,
    should_clear_on_first_star,
};
pub use style::{StyleId, StylePack};
pub use syntax::{
    CodeLineHighlight, CodePalette, CodeToken, LanguageId, SyntaxKind, code_palette,
    line_is_highlighted,
};
pub use theme::{ResolvedTheme, SemanticThemeTable};
pub use transition::{Easing, TransitionValue};
pub use tree::{
    FolderState, TreeFile, TreeFolder, TreeIconKey, TreeNode, TreeNodeId, TreeNodeIdError,
    TreeOrdering, TreeValidationError, TreeViewAction, TreeViewState, VisibleTreeNode,
    flatten_visible, flatten_visible_ordered, truncate_tree_label, validate_tree,
};
pub use typography::{FontHeading, FontId, FontPack};
pub use value_mapping::{
    ValueRange, closest_index, decrement, finite_or_zero, fraction, increment, max_value_at_index,
    min_value_at_index, modulo, round_to_step_precision, set_value_at_index, snap,
    snap_value_to_step, snapped_fraction, transform_value, value_at_fraction, value_ranges, wrap,
};
