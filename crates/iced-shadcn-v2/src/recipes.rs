//! Adapters from [`shadcn_common::recipes`] onto iced types.

use crate::iced_compat::font::Weight;
use shadcn_common::{ComponentRadius, FontWeight};

use crate::theme::Theme;

/// Maps a backend-agnostic [`FontWeight`] to iced’s font weight.
pub fn iced_font_weight(weight: FontWeight) -> Weight {
    match weight {
        FontWeight::Thin => Weight::Thin,
        FontWeight::ExtraLight => Weight::ExtraLight,
        FontWeight::Light => Weight::Light,
        FontWeight::Normal => Weight::Normal,
        FontWeight::Medium => Weight::Medium,
        FontWeight::Semibold => Weight::Semibold,
        FontWeight::Bold => Weight::Bold,
        FontWeight::ExtraBold => Weight::ExtraBold,
        FontWeight::Black => Weight::Black,
        _ => Weight::Normal,
    }
}

/// Resolves a [`ComponentRadius`] intent against the theme.
///
/// - `Sm` / `Md` / `Lg` → style-pack twill slots (pack-specific remapping).
/// - `Xl` / `S2xl` / `S3xl` / `S4xl` → shadcn `RadiusScale`
///   (`calc(var(--radius) + 4/8/12/16px)`), **not** raw Tailwind 12/16/24/32.
pub fn component_radius_px(theme: &Theme, radius: ComponentRadius) -> f32 {
    let scale = theme.style.radius;

    match radius {
        ComponentRadius::None => 0.0,
        ComponentRadius::Sm => theme.style.twill_radius_sm.px_value(),
        ComponentRadius::Md => theme.style.twill_radius_md.px_value(),
        ComponentRadius::Lg => theme.style.twill_radius_lg.px_value(),
        ComponentRadius::Xl => scale.xl_px,
        ComponentRadius::S2xl => scale.xxl_px,
        ComponentRadius::S3xl => scale.xxxl_px,
        ComponentRadius::S4xl => scale.xxxxl_px,
        ComponentRadius::Full => 9999.0,
        _ => theme.style.twill_radius_md.px_value(),
    }
}
