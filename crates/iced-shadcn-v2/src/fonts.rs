//! iced adapter over backend-agnostic font faces from [`shadcn_common`].
//!
//! [`iced_font`] is always available. Font bytes and [`ALL_FACES`] live in
//! `shadcn-common` and are re-exported only when this crate's `fonts` feature
//! is enabled (forwards `shadcn-common/fonts`).

use crate::iced_compat::Font;
use shadcn_common::FontId;

#[cfg(feature = "fonts")]
pub use shadcn_common::fonts::{
    ALL_FACES, GEIST_BOLD, GEIST_MEDIUM, GEIST_MONO_MEDIUM, GEIST_MONO_REGULAR, GEIST_REGULAR,
    GEIST_SEMIBOLD, INSTRUMENT_SERIF_REGULAR, INTER_BOLD, INTER_REGULAR, INTER_SEMIBOLD,
    JETBRAINS_MONO_MEDIUM, JETBRAINS_MONO_REGULAR,
};

/// iced [`Font`] for a [`FontId`] (family name must match the loaded TTF).
pub fn iced_font(id: FontId) -> Font {
    Font::with_name(id.family_name())
}
