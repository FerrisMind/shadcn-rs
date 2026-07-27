//! Experimental builder-first APIs.
//!
//! Theme tokens come from [`shadcn_common`] via [`theme::Theme`]. Crate-level
//! [`crate::theme::Theme`] / twill `SemanticThemeVars::shadcn_neutral()` are
//! intentionally not used here.

pub mod button;
pub mod fonts;
pub mod theme;

pub use button::{Button, ButtonBuildError, ButtonRadius, ButtonSize, ButtonVariant};
pub use fonts::{ALL_FACES, iced_font};
pub use theme::{Palette, Theme};

pub use shadcn_common::{
    AccentColor, BaseColor, FontHeading, FontId, FontPack, RadiusId, RadiusScale, ResolvedTheme,
    StyleId, StylePack, ThemeMode,
};
