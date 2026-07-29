//! Shared shadcn design tokens for iced-shadcn and egui-shadcn.
//!
//! Built on [`twill_core`] — no iced/egui types. Backends adapt via twill-iced / twill-egui.

#![forbid(unsafe_code)]

pub mod color;
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
pub use icons::{IconName, IconSet};
pub use radius::{RadiusId, RadiusScale};
pub use recipes::{
    BadgeRecipe, ButtonSizeRecipe, ButtonTypeRecipe, ComponentRadius, ControlSize, FontWeight,
    KbdRecipe, LabelContext, LabelRecipe, ProgressRecipe, SkeletonRecipe, SliderRecipe,
    SliderThumbBorder, SliderThumbFill, SliderTrackSurface, SwitchRecipe, SwitchSizeRecipe,
    ToggleRecipe, ToggleSizeRecipe, TypeRecipe, badge_recipe, button_size, button_type, kbd_recipe,
    label_recipe, progress_recipe, skeleton_default_radius, skeleton_recipe, slider_recipe,
    switch_recipe, switch_size, toggle_recipe, toggle_size,
};
pub use style::{StyleId, StylePack};
pub use theme::{ResolvedTheme, SemanticThemeTable};
pub use typography::{FontHeading, FontId, FontPack};
