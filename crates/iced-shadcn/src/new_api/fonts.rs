//! Font faces matching shadcn-svelte `@fontsource(-variable)` families.
//!
//! Sources (same families as shadcn-svelte `font-definitions.ts`):
//! - Geist / Geist Mono — vercel/geist-font (TTF)
//! - Inter — `@fontsource-variable/inter` (TTF copies)
//! - Instrument Serif — google/fonts OFL
//! - JetBrains Mono — JetBrains/JetBrainsMono

use iced::Font;
use shadcn_common::FontId;

/// Geist Regular.
pub const GEIST_REGULAR: &[u8] = include_bytes!("../../assets/fonts/Geist-Regular.ttf");
/// Geist Medium.
pub const GEIST_MEDIUM: &[u8] = include_bytes!("../../assets/fonts/Geist-Medium.ttf");
/// Geist SemiBold.
pub const GEIST_SEMIBOLD: &[u8] = include_bytes!("../../assets/fonts/Geist-SemiBold.ttf");
/// Geist Bold.
pub const GEIST_BOLD: &[u8] = include_bytes!("../../assets/fonts/Geist-Bold.ttf");

/// Geist Mono Regular.
pub const GEIST_MONO_REGULAR: &[u8] = include_bytes!("../../assets/fonts/GeistMono-Regular.ttf");
/// Geist Mono Medium.
pub const GEIST_MONO_MEDIUM: &[u8] = include_bytes!("../../assets/fonts/GeistMono-Medium.ttf");

/// Inter Regular.
pub const INTER_REGULAR: &[u8] = include_bytes!("../../assets/fonts/Inter-Regular.ttf");
/// Inter SemiBold.
pub const INTER_SEMIBOLD: &[u8] = include_bytes!("../../assets/fonts/Inter-SemiBold.ttf");
/// Inter Bold.
pub const INTER_BOLD: &[u8] = include_bytes!("../../assets/fonts/Inter-Bold.ttf");

/// Instrument Serif Regular.
pub const INSTRUMENT_SERIF_REGULAR: &[u8] =
    include_bytes!("../../assets/fonts/InstrumentSerif-Regular.ttf");

/// JetBrains Mono Regular.
pub const JETBRAINS_MONO_REGULAR: &[u8] =
    include_bytes!("../../assets/fonts/JetBrainsMono-Regular.ttf");
/// JetBrains Mono Medium.
pub const JETBRAINS_MONO_MEDIUM: &[u8] =
    include_bytes!("../../assets/fonts/JetBrainsMono-Medium.ttf");

/// All faces to register with repeated `iced::application(...).font(...)`.
pub const ALL_FACES: &[&[u8]] = &[
    GEIST_REGULAR,
    GEIST_MEDIUM,
    GEIST_SEMIBOLD,
    GEIST_BOLD,
    GEIST_MONO_REGULAR,
    GEIST_MONO_MEDIUM,
    INTER_REGULAR,
    INTER_SEMIBOLD,
    INTER_BOLD,
    INSTRUMENT_SERIF_REGULAR,
    JETBRAINS_MONO_REGULAR,
    JETBRAINS_MONO_MEDIUM,
];

/// iced [`Font`] for a [`FontId`] (family name must match the loaded TTF).
pub fn iced_font(id: FontId) -> Font {
    Font::with_name(id.family_name())
}
