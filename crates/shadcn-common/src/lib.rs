//! Shared shadcn design tokens for iced-shadcn and egui-shadcn.
//!
//! Built on [`twill_core`] — no iced/egui types. Backends adapt via twill-iced / twill-egui.

#![forbid(unsafe_code)]

pub mod color;
pub mod floating;
#[cfg(feature = "fonts")]
pub mod fonts;
pub mod icons;
pub mod radius;
pub mod recipes;
pub mod style;
pub mod theme;
pub mod typography;

mod generated;

pub use color::{AccentColor, BaseColor, OklchColor, ThemeMode};
pub use floating::{
    FloatingAlign, FloatingConfig, FloatingPlacement, FloatingRect, FloatingSide, compute_floating,
};
pub use icons::{IconName, IconSet};
pub use radius::{RadiusId, RadiusScale};
pub use recipes::{
    BadgeRecipe, ButtonSizeRecipe, ButtonTypeRecipe, ComponentRadius, ControlSize, FontWeight,
    KbdRecipe, LabelContext, LabelRecipe, ProgressRecipe, RadioCheckedFill, RadioGroupRecipe,
    RadioSurface, SkeletonRecipe, SliderRecipe, SliderThumbBorder, SliderThumbFill,
    SliderTrackSurface, SwitchRecipe, SwitchSizeRecipe, TOOLTIP_ANIMATION_MS, TOOLTIP_SLIDE_PX,
    TOOLTIP_ZOOM_FROM, ToggleRecipe, ToggleSizeRecipe, TooltipRecipe, TypeRecipe, badge_recipe,
    button_size, button_type, kbd_recipe, label_recipe, progress_recipe, radio_group_recipe,
    skeleton_default_radius, skeleton_recipe, slider_recipe, switch_recipe, switch_size,
    toggle_recipe, toggle_size, tooltip_recipe,
};
pub use style::{StyleId, StylePack};
pub use theme::{ResolvedTheme, SemanticThemeTable};
pub use typography::{FontHeading, FontId, FontPack};
