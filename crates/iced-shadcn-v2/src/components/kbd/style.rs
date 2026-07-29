//! Mapping of kbd surfaces to iced container styles.

use crate::iced_compat::border::Border;
use crate::iced_compat::widget::container;
use crate::iced_compat::{Color, Shadow};

use twill_core::prelude::theme::SemanticColor;

use super::{KbdRadius, KbdSurface};
use crate::theme::Theme;

/// Resolves the container style for a kbd on the given surface.
pub(super) fn resolve_container_style(
    theme: &Theme,
    surface: KbdSurface,
    radius: Option<KbdRadius>,
) -> container::Style {
    let (background, text) = surface_visual(theme, surface);

    container::Style {
        background: background
            .filter(|color| color.a > f32::EPSILON)
            .map(crate::iced_compat::Background::Color),
        text_color: Some(text),
        border: Border {
            radius: radius_px(theme, effective_radius(theme, radius)).into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        shadow: Shadow::default(),
        snap: true,
    }
}

/// Background / text pair for a surface.
fn surface_visual(theme: &Theme, surface: KbdSurface) -> (Option<Color>, Color) {
    match surface {
        // shadcn: `bg-muted text-muted-foreground`
        KbdSurface::Default => (
            Some(theme.semantic_color(SemanticColor::Muted)),
            theme.semantic_color(SemanticColor::MutedForeground),
        ),
        // shadcn: `in-data-[slot=tooltip-content]:bg-background/20` (dark `/10`)
        // and `in-data-[slot=tooltip-content]:text-background` — the tooltip
        // bubble is painted with the theme foreground, so `background` reads
        // as a translucent inverse chip.
        KbdSurface::Tooltip => {
            let alpha = if theme.is_dark() { 0.10 } else { 0.20 };
            let background = theme.semantic_color(SemanticColor::Background);
            (Some(with_alpha(background, alpha)), background)
        }
        // shadcn: `in-data-[slot=input-group]:bg-input`
        KbdSurface::InputGroup => (
            Some(theme.semantic_color(SemanticColor::Input)),
            theme.semantic_color(SemanticColor::MutedForeground),
        ),
    }
}

fn with_alpha(mut color: Color, alpha: f32) -> Color {
    color.a = alpha.clamp(0.0, 1.0);
    color
}

/// Unset radius → small pack radius, unless the active style pack locks
/// radius to none (Lyra / Sera).
pub(super) fn effective_radius(theme: &Theme, radius: Option<KbdRadius>) -> KbdRadius {
    match radius {
        Some(radius) => radius,
        None if theme.style_id().locks_radius() => KbdRadius::None,
        None => KbdRadius::Small,
    }
}

fn radius_px(theme: &Theme, radius: KbdRadius) -> f32 {
    match radius {
        KbdRadius::None => 0.0,
        KbdRadius::Small => theme.style.twill_radius_sm.px_value(),
        KbdRadius::Medium => theme.style.twill_radius_md.px_value(),
        KbdRadius::Large => theme.style.twill_radius_lg.px_value(),
        KbdRadius::Full => 9999.0,
    }
}
