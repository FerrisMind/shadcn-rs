//! Shared shadcn design tokens for iced-shadcn and egui-shadcn.
//!
//! Built on [`twill_core`] — no iced/egui types. Backends adapt via twill-iced / twill-egui.

#![forbid(unsafe_code)]

pub mod color;
#[cfg(feature = "fonts")]
pub mod fonts;
pub mod icons;
pub mod radius;
pub mod style;
pub mod theme;
pub mod typography;

mod generated;

pub use color::{AccentColor, BaseColor, OklchColor, ThemeMode};
pub use icons::{IconName, IconSet};
pub use radius::{RadiusId, RadiusScale};
pub use style::{StyleId, StylePack};
pub use theme::{ResolvedTheme, SemanticThemeTable};
pub use typography::{FontHeading, FontId, FontPack};
