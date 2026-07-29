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

/// Resolves a [`ComponentRadius`] intent against the theme’s twill radius slots.
pub fn component_radius_px(theme: &Theme, radius: ComponentRadius) -> f32 {
    match radius {
        ComponentRadius::None => 0.0,
        ComponentRadius::Sm => theme.style.twill_radius_sm.px_value(),
        ComponentRadius::Md => theme.style.twill_radius_md.px_value(),
        ComponentRadius::Lg | ComponentRadius::Xl => theme.style.twill_radius_lg.px_value(),
        ComponentRadius::Full => 9999.0,
        _ => theme.style.twill_radius_md.px_value(),
    }
}
