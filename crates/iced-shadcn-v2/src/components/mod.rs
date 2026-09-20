//! Reusable iced UI components.
//!
//! Each component owns its public builder API and keeps implementation details
//!—geometry, rendering, styles, errors, and tests—inside its own submodule.
//!
//! Modules are gated by category / heavy cargo features (see crate root).

// --- core ---
#[cfg(feature = "core")]
pub mod accordion;
#[cfg(feature = "core")]
pub mod alert;
#[cfg(feature = "core")]
pub mod aspect_ratio;
#[cfg(feature = "core")]
pub mod badge;
#[cfg(feature = "core")]
pub mod button;
#[cfg(feature = "core")]
pub mod button_group;
#[cfg(feature = "core")]
pub mod card;
#[cfg(feature = "core")]
pub mod collapsible;
#[cfg(feature = "core")]
pub mod copy_button;
#[cfg(feature = "core")]
pub mod empty;
#[cfg(feature = "core")]
pub mod item;
#[cfg(feature = "core")]
pub mod kbd;
#[cfg(feature = "core")]
pub mod label;
#[cfg(feature = "core")]
pub mod meter;
#[cfg(feature = "core")]
pub mod progress;
#[cfg(feature = "core")]
pub mod scroll_area;
#[cfg(feature = "core")]
pub mod separator;
#[cfg(feature = "core")]
pub mod skeleton;
#[cfg(feature = "core")]
pub mod snippet;
#[cfg(feature = "core")]
pub mod spinner;
#[cfg(feature = "core")]
pub mod typography;

// --- forms ---
#[cfg(feature = "forms")]
pub mod checkbox;
#[cfg(feature = "forms")]
pub mod field;
#[cfg(feature = "forms")]
pub mod form;
#[cfg(feature = "forms")]
pub mod input;
#[cfg(feature = "forms")]
pub mod input_group;
#[cfg(feature = "forms")]
pub mod input_otp;
#[cfg(feature = "forms")]
pub mod native_select;
#[cfg(feature = "forms")]
pub mod radio_group;
#[cfg(feature = "forms")]
pub mod rename;
#[cfg(feature = "forms")]
pub mod select;
#[cfg(feature = "forms")]
pub mod slider;
#[cfg(feature = "forms")]
pub mod star_rating;
#[cfg(feature = "forms")]
pub mod switch;
#[cfg(feature = "forms")]
pub mod textarea;
#[cfg(feature = "forms")]
pub mod toggle;
#[cfg(feature = "forms")]
pub mod toggle_group;

// --- overlays ---
#[cfg(feature = "overlays")]
pub mod alert_dialog;
#[cfg(feature = "overlays")]
pub mod combobox;
#[cfg(feature = "overlays")]
pub mod command;
#[cfg(feature = "overlays")]
pub mod context_menu;
#[cfg(feature = "overlays")]
pub mod dialog;
#[cfg(feature = "overlays")]
pub mod drawer;
#[cfg(feature = "overlays")]
pub mod dropdown_menu;
#[cfg(feature = "overlays")]
pub mod hover_card;
#[cfg(feature = "overlays")]
pub mod menubar;
#[cfg(feature = "overlays")]
pub mod navigation_menu;
#[cfg(feature = "overlays")]
pub mod pm_command;
#[cfg(feature = "overlays")]
pub mod popover;
#[cfg(feature = "overlays")]
pub mod sheet;
#[cfg(feature = "overlays")]
pub mod tooltip;

// --- navigation ---
#[cfg(feature = "navigation")]
pub mod breadcrumb;
#[cfg(feature = "navigation")]
pub mod carousel;
#[cfg(feature = "navigation")]
pub mod pagination;
#[cfg(feature = "navigation")]
pub mod sidebar;
#[cfg(feature = "navigation")]
pub mod stepper;
#[cfg(feature = "navigation")]
pub mod tabs;

// --- data ---
#[cfg(feature = "data")]
pub mod avatar;
#[cfg(feature = "data")]
pub mod calendar;
#[cfg(feature = "data")]
pub mod chart;
#[cfg(feature = "data")]
pub mod date_picker;
#[cfg(feature = "data")]
pub mod range_calendar;
#[cfg(feature = "data")]
pub mod resizable;
#[cfg(feature = "data")]
pub mod table;
#[cfg(feature = "data")]
pub mod tree_view;

// --- feedback ---
#[cfg(feature = "feedback")]
pub mod sonner;

// --- file-drop ---
#[cfg(feature = "file-drop")]
pub mod file_drop_zone;

// --- heavy ---
#[cfg(feature = "syntax")]
pub mod code;
#[cfg(feature = "password")]
pub mod password;
#[cfg(feature = "phone")]
pub mod phone_input;
#[cfg(feature = "emoji-picker")]
pub mod emoji_picker;
#[cfg(feature = "data-table")]
pub mod data_table;
