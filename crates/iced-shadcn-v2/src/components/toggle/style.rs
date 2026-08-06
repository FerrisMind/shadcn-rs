//! Mapping of toggle states to iced button styles.
//!
//! Follows `.cn-toggle`: both variants are transparent at rest, fill with
//! `muted` on hover and while pressed on (`aria-pressed:bg-muted`), and the
//! `outline` variant adds an `input` border (plus a `shadow-xs` on Vega).

use crate::iced_compat::border::Border;
use crate::iced_compat::widget::button as button_widget;
use crate::iced_compat::{Color, Shadow, Vector};

use twill_core::prelude::theme::SemanticColor;

use super::{ToggleRadius, ToggleVariant};
use crate::recipes::component_radius_px;
use crate::theme::Theme;

/// Alpha applied to a disabled toggle (`disabled:opacity-50`).
const DISABLED_OPACITY: f32 = 0.5;
/// Alpha of the destructive ring color (`aria-invalid:ring-destructive/20|40`).
const INVALID_RING_LIGHT_ALPHA: f32 = 0.2;
const INVALID_RING_DARK_ALPHA: f32 = 0.4;

pub(super) fn resolve_toggle_style(
    theme: &Theme,
    variant: ToggleVariant,
    pressed: bool,
    radius: Option<ToggleRadius>,
    invalid: bool,
    disabled: bool,
    status: button_widget::Status,
) -> button_widget::Style {
    let muted = theme.semantic_color(SemanticColor::Muted);
    let foreground = theme.semantic_color(SemanticColor::Foreground);

    // `bg-transparent` at rest, `hover:bg-muted`, `aria-pressed:bg-muted`.
    // A held mouse button nudges the fill so the press reads on desktop,
    // matching how the button port treats its flat variants.
    let background = match status {
        button_widget::Status::Active => pressed.then_some(muted),
        button_widget::Status::Hovered => Some(muted),
        button_widget::Status::Pressed => Some(shift_toward(muted, theme.is_dark(), 0.08)),
        button_widget::Status::Disabled => pressed.then_some(muted),
    };

    // `aria-invalid:border-destructive` beats the variant border; the web ring
    // (`ring-destructive/20`) has no iced counterpart, so the border stands in.
    let (border_color, border_width) = if invalid {
        let alpha = if theme.is_dark() {
            INVALID_RING_DARK_ALPHA
        } else {
            INVALID_RING_LIGHT_ALPHA
        };
        let destructive = theme.semantic_color(SemanticColor::Destructive);
        match variant {
            ToggleVariant::Default => (with_alpha(destructive, alpha), 1.0),
            ToggleVariant::Outline => (destructive, 1.0),
        }
    } else {
        match variant {
            ToggleVariant::Default => (Color::TRANSPARENT, 0.0),
            ToggleVariant::Outline => (theme.semantic_color(SemanticColor::Input), 1.0),
        }
    };

    // Vega's outline toggle carries a `shadow-xs`.
    let shadow = if variant == ToggleVariant::Outline && theme.style.toggle().outline_shadow {
        Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.05),
            offset: Vector::new(0.0, 1.0),
            blur_radius: 2.0,
        }
    } else {
        Shadow::default()
    };

    // `disabled:opacity-50` keeps the resolved colors and only dims them.
    let opacity = if disabled { DISABLED_OPACITY } else { 1.0 };

    button_widget::Style {
        background: background
            .map(|color| with_alpha(color, opacity))
            .filter(|color| color.a > f32::EPSILON)
            .map(crate::iced_compat::Background::Color),
        text_color: with_alpha(foreground, opacity),
        border: Border {
            radius: resolve_radius_px(theme, radius).into(),
            width: border_width,
            color: with_alpha(border_color, opacity),
        },
        shadow,
        snap: true,
    }
}

fn resolve_radius_px(theme: &Theme, radius: Option<ToggleRadius>) -> f32 {
    match radius {
        Some(radius) => radius_px(theme, radius),
        None => component_radius_px(theme, theme.style.toggle().default_radius),
    }
}

fn radius_px(theme: &Theme, radius: ToggleRadius) -> f32 {
    match radius {
        ToggleRadius::None => 0.0,
        ToggleRadius::Small => theme.style.twill_radius_sm.px_value(),
        ToggleRadius::Medium => theme.style.twill_radius_md.px_value(),
        ToggleRadius::Large => theme.style.twill_radius_lg.px_value(),
        ToggleRadius::Full => 9999.0,
    }
}

fn shift_toward(color: Color, dark: bool, amount: f32) -> Color {
    let target = if dark { Color::WHITE } else { Color::BLACK };
    let amount = amount.clamp(0.0, 1.0);

    Color {
        r: color.r + (target.r - color.r) * amount,
        g: color.g + (target.g - color.g) * amount,
        b: color.b + (target.b - color.b) * amount,
        a: color.a + (target.a - color.a) * amount,
    }
}

fn with_alpha(color: Color, alpha: f32) -> Color {
    Color {
        a: color.a * alpha.clamp(0.0, 1.0),
        ..color
    }
}
