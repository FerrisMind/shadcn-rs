//! Backend-agnostic component recipes derived from shadcn-svelte style CSS.
//!
//! These tokens intentionally avoid iced/egui types so both GUI backends can
//! share the same StyleId tables. Backends map [`FontWeight`] / [`ComponentRadius`]
//! onto their native font and radius APIs.

mod badge;
mod button;
mod kbd;
mod label;
mod progress;
mod skeleton;

pub use badge::{BadgeRecipe, badge_recipe};
pub use button::{ButtonSizeRecipe, ButtonTypeRecipe, ControlSize, button_size, button_type};
pub use kbd::{KbdRecipe, kbd_recipe};
pub use label::{LabelContext, LabelRecipe, label_recipe};
pub use progress::{ProgressRecipe, progress_recipe};
pub use skeleton::{SkeletonRecipe, skeleton_default_radius, skeleton_recipe};

/// Backend-agnostic font weight matching CSS `font-normal` / `font-medium` / ….
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum FontWeight {
    Thin,
    ExtraLight,
    Light,
    #[default]
    Normal,
    Medium,
    Semibold,
    Bold,
    ExtraBold,
    Black,
}

/// Corner-radius intent from style CSS (`rounded-none` / `rounded-md` / …).
///
/// Backends resolve this against [`crate::StylePack`]'s twill radius slots
/// (or a pill / zero) — the enum itself stays unitless.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum ComponentRadius {
    /// `rounded-none` / locked styles.
    None,
    /// `rounded-sm`.
    Sm,
    /// `rounded-md` (typical control default).
    #[default]
    Md,
    /// `rounded-lg`.
    Lg,
    /// `rounded-xl`.
    Xl,
    /// Pill / `rounded-full` / `rounded-4xl` treated as fully rounded.
    Full,
}

/// Shared typography recipe (size, weight, casing, tracking, line-height).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TypeRecipe {
    /// Font size in CSS px (`text-sm` → 14, `text-xs` → 12, `0.625rem` → 10).
    pub size_px: f32,
    pub weight: FontWeight,
    pub uppercase: bool,
    /// Letter-spacing in `em` (`tracking-wide` → 0.025, `tracking-widest` → 0.1).
    pub tracking_em: f32,
    /// Absolute line height in px.
    pub line_height_px: f32,
}

impl TypeRecipe {
    /// Letter-spacing converted to absolute px for the current size.
    pub const fn tracking_px(self) -> f32 {
        self.size_px * self.tracking_em
    }
}
