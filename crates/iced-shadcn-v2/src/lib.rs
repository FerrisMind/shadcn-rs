//! Builder-first shadcn-inspired component kit for iced — v2 API.
//!
//! Successor of `iced-shadcn::new_api`. Theme tokens come from
//! [`shadcn_common`]; iced styles are built directly from `twill-core`
//! tokens, without the `twill` style-composition facade. The crate
//! intentionally does not depend on `iced-shadcn` v1.
//!
//! # Theming model
//!
//! Store a [`Theme`] in app state and pass `&Theme` into components. Style
//! packs (`StyleId::Vega`, …) live on [`Theme`]; overrides via `Theme::with_*`
//! beat pack defaults; per-widget knobs (e.g. [`Button::color`]) beat that
//! theme for one control. Two looks on one screen ⇒ two [`Theme`] values (or
//! one theme + different button variants). See the crate README “Theming”
//! section for the three common patterns.
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
pub mod recipes;
pub mod theme;

/// Backwards-compatible access to the aspect-ratio component.
pub use components::aspect_ratio;
/// Backwards-compatible access to the badge component.
pub use components::badge;
/// Backwards-compatible access to the button component.
pub use components::button;
/// Backwards-compatible access to the checkbox component.
pub use components::checkbox;
/// Backwards-compatible access to the kbd component.
pub use components::kbd;
/// Backwards-compatible access to the label component.
pub use components::label;
/// Backwards-compatible access to the progress component.
pub use components::progress;
/// Backwards-compatible access to the separator component.
pub use components::separator;
/// Backwards-compatible access to the skeleton component.
pub use components::skeleton;
/// Backwards-compatible access to the spinner component.
pub use components::spinner;
/// Backwards-compatible access to the typography component.
pub use components::typography;

pub use components::aspect_ratio::{AspectRatio, MIN_ASPECT_RATIO, aspect_ratio};
pub use components::badge::{Badge, BadgeBuildError, BadgeRadius, BadgeVariant};
pub use components::button::{Button, ButtonBuildError, ButtonRadius, ButtonSize, ButtonVariant};
pub use components::checkbox::{
    Checkbox, CheckboxConfig, CheckboxSize, CheckboxState, CheckboxVariant,
};
pub use components::kbd::{Kbd, KbdBuildError, KbdGroup, KbdRadius, KbdSurface};
pub use components::label::{Label, LabelContext};
pub use components::progress::{
    Progress, ProgressOrientation, ProgressRadius, ProgressSize, ProgressState, ProgressVariant,
    progress,
};
pub use components::separator::{Separator, SeparatorOrientation, separator};
pub use components::skeleton::{
    Skeleton, SkeletonAnimation, SkeletonFill, SkeletonRadius, SkeletonShape,
};
pub use components::spinner::{Spinner, SpinnerSize, SpinnerVariant, spinner};
pub use components::typography::{Typography, TypographyList, TypographyTable, TypographyVariant};
pub use fonts::{ALL_FACES, iced_font};
pub use theme::{Palette, Theme};

pub use shadcn_common::{
    AccentColor, BaseColor, ComponentRadius, ControlSize, FontHeading, FontId, FontPack,
    FontWeight, RadiusId, RadiusScale, ResolvedTheme, StyleId, StylePack, ThemeMode, TypeRecipe,
};

/// Semantic color slots resolved by [`Theme::semantic_color`].
pub use twill_core::prelude::theme::SemanticColor;

/// Spacing tokens accepted by [`Button::padding`].
pub use twill_core::prelude::{Padding, PaddingValue, PaddingVar, Spacing};
