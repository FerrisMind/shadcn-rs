//! Builder-first shadcn-inspired component kit for iced — v2 API.
//!
//! Successor of `iced-shadcn::new_api`. Theme tokens come from
//! [`shadcn_common`]; iced styles are built directly from `twill-core`
//! tokens, without the `twill` style-composition facade. The crate
//! intentionally does not depend on `iced-shadcn` v1.
//!
//! ```rust,no_run
//! use iced::Element;
//! use iced_shadcn_v2::{Button, ButtonVariant, Theme};
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     Save,
//! }
//!
//! fn view(theme: &Theme) -> Element<'_, Message> {
//!     Button::text("Save", theme)
//!         .variant(ButtonVariant::Default)
//!         .on_press(Message::Save)
//!         .into()
//! }
//! ```

pub mod components;
pub mod fonts;
pub mod theme;

/// Backwards-compatible access to the button component.
pub use components::button;
/// Backwards-compatible access to the spinner component.
pub use components::spinner;

pub use components::button::{Button, ButtonBuildError, ButtonRadius, ButtonSize, ButtonVariant};
pub use components::spinner::{Spinner, SpinnerSize, SpinnerVariant, spinner};
pub use fonts::{ALL_FACES, iced_font};
pub use theme::{Palette, Theme};

pub use shadcn_common::{
    AccentColor, BaseColor, FontHeading, FontId, FontPack, RadiusId, RadiusScale, ResolvedTheme,
    StyleId, StylePack, ThemeMode,
};

/// Semantic color slots resolved by [`Theme::semantic_color`].
pub use twill_core::prelude::theme::SemanticColor;

/// Spacing tokens accepted by [`Button::padding`].
pub use twill_core::prelude::{Padding, PaddingValue, PaddingVar, Spacing};
