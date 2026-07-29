//! Mapping of radio-group states to semantic theme colors.
//!
//! Colors follow `.cn-radio-group-item` / `.cn-radio-group-indicator-icon`: the
//! indicator borders with `input` and fills with `primary` once checked, the dot
//! uses `primary-foreground` (Sera keeps the box transparent and paints border
//! and dot with `foreground`), and the ring uses `ring` for focus and
//! `destructive` for invalid values.

use shadcn_common::{RadioCheckedFill, RadioGroupRecipe, RadioSurface};
use twill_core::prelude::theme::SemanticColor;

use super::geometry::Metrics;
use super::types::{RadioGroupStatus, RadioGroupStyle};
use crate::iced_compat::Color;
use crate::theme::Theme;

/// Alpha of `destructive` borders in dark mode (`dark:border-destructive/50`).
const DARK_INVALID_BORDER_ALPHA: f32 = 0.5;
/// Alpha of the `aria-invalid` ring (`ring-destructive/20`).
const INVALID_RING_ALPHA: f32 = 0.2;
/// Alpha of the `aria-invalid` ring in dark mode (`ring-destructive/40`).
const DARK_INVALID_RING_ALPHA: f32 = 0.4;

pub(super) fn resolve_style(
    theme: &Theme,
    metrics: Metrics,
    status: RadioGroupStatus,
) -> RadioGroupStyle {
    let recipe = theme.style.radio_group();
    let (indicator, border, dot) = surface_colors(theme, recipe, status.checked);
    let border = invalid_border(theme, recipe, border, status);
    let border = focus_border(theme, border, status);
    let ring = ring_color(theme, recipe, status);
    let opacity = if status.disabled {
        recipe.disabled_opacity
    } else {
        1.0
    };

    RadioGroupStyle {
        indicator: with_alpha(indicator, opacity),
        border: with_alpha(border, opacity),
        border_width: metrics.border_width,
        radius: metrics.radius,
        indicator_size: metrics.indicator,
        dot: with_alpha(dot, opacity),
        dot_size: if status.checked { metrics.dot } else { 0.0 },
        ring: ring.map(|ring| with_alpha(ring, opacity)),
        ring_width: metrics.ring_width,
        label: with_alpha(theme.palette.foreground, opacity),
        description: with_alpha(theme.palette.muted_foreground, opacity),
    }
}

/// Returns the `(indicator, border, dot)` colors of the pack's base state.
fn surface_colors(theme: &Theme, recipe: RadioGroupRecipe, checked: bool) -> (Color, Color, Color) {
    let input = theme.semantic_color(SemanticColor::Input);
    let unchecked_alpha = if theme.is_dark() {
        recipe.dark_unchecked_opacity
    } else {
        recipe.unchecked_opacity
    };

    let (unchecked_fill, unchecked_border) = match recipe.unchecked_surface {
        // `bg-input/90 border-transparent`: the fill carries the whole shape.
        RadioSurface::Filled => (with_alpha(input, unchecked_alpha), Color::TRANSPARENT),
        // `border-input` plus `dark:bg-input/30` on top of the page background.
        RadioSurface::Outline => (with_alpha(input, unchecked_alpha), input),
        // `border-input bg-transparent` in both modes.
        RadioSurface::Transparent => (Color::TRANSPARENT, input),
        _ => (with_alpha(input, unchecked_alpha), input),
    };

    if !checked {
        return (unchecked_fill, unchecked_border, Color::TRANSPARENT);
    }

    match recipe.checked_fill {
        // `data-checked:border-foreground` with no fill and a `bg-foreground` dot.
        RadioCheckedFill::Foreground => {
            let foreground = theme.semantic_color(SemanticColor::Foreground);

            (unchecked_fill, foreground, foreground)
        }
        // `data-checked:bg-primary` with a `bg-primary-foreground` dot. Packs
        // that hide the border keep it transparent instead of `border-primary`.
        RadioCheckedFill::Primary | _ => {
            let border = if recipe.unchecked_surface == RadioSurface::Filled {
                Color::TRANSPARENT
            } else {
                theme.palette.primary
            };

            (
                theme.palette.primary,
                border,
                theme.palette.primary_foreground,
            )
        }
    }
}

/// Applies `aria-invalid:border-destructive` unless the pack keeps a checked
/// border (`aria-invalid:aria-checked:border-primary`).
///
/// Only the packs that paint a visible unchecked border ship that override, so
/// the filled packs (Luma, Rhea) do show the destructive border while checked.
fn invalid_border(
    theme: &Theme,
    recipe: RadioGroupRecipe,
    border: Color,
    status: RadioGroupStatus,
) -> Color {
    let checked_border_survives = recipe.unchecked_surface != RadioSurface::Filled;
    if !status.invalid || (status.checked && checked_border_survives) {
        return border;
    }

    let destructive = theme.semantic_color(SemanticColor::Destructive);
    if theme.is_dark() {
        with_alpha(destructive, DARK_INVALID_BORDER_ALPHA)
    } else {
        destructive
    }
}

/// `focus-visible:border-ring` — never overrides an `aria-invalid` border,
/// matching the CSS cascade order the packs generate.
fn focus_border(theme: &Theme, border: Color, status: RadioGroupStatus) -> Color {
    if status.focused && !status.invalid {
        theme.semantic_color(SemanticColor::Ring)
    } else {
        border
    }
}

/// `aria-invalid` outranks `focus-visible`, matching the CSS cascade order.
fn ring_color(theme: &Theme, recipe: RadioGroupRecipe, status: RadioGroupStatus) -> Option<Color> {
    if status.invalid {
        let alpha = if theme.is_dark() {
            DARK_INVALID_RING_ALPHA
        } else {
            INVALID_RING_ALPHA
        };

        return Some(with_alpha(
            theme.semantic_color(SemanticColor::Destructive),
            alpha,
        ));
    }

    if status.focused {
        return Some(with_alpha(
            theme.semantic_color(SemanticColor::Ring),
            recipe.ring_opacity,
        ));
    }

    None
}

fn with_alpha(color: Color, alpha: f32) -> Color {
    Color {
        a: color.a * alpha.clamp(0.0, 1.0),
        ..color
    }
}
