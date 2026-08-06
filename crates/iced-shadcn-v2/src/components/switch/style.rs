//! Mapping of switch states to semantic theme colors.
//!
//! Colors follow `.cn-switch` / `.cn-switch-thumb`: the track uses `primary`
//! when checked and `input` when unchecked, the thumb uses `background` (dark
//! mode swaps in `foreground` / `primary-foreground`), and the ring uses `ring`
//! for focus and `destructive` for invalid values.

use crate::iced_compat::Color;
use twill_core::prelude::theme::SemanticColor;

use super::Switch;
use super::geometry::Metrics;
use super::types::{SwitchStatus, SwitchStyle};
use crate::theme::Theme;

/// Alpha applied to `input` for unchecked dark-mode tracks (`bg-input/80`).
const DARK_UNCHECKED_TRACK_ALPHA: f32 = 0.8;
/// Alpha applied to a disabled switch (`data-disabled:opacity-50`).
const DISABLED_OPACITY: f32 = 0.5;
/// Alpha of `destructive` borders in dark mode (`border-destructive/50`).
const DARK_INVALID_BORDER_ALPHA: f32 = 0.5;

pub(super) fn resolve_style<Message>(
    switch: &Switch<'_, Message>,
    metrics: Metrics,
    status: SwitchStatus,
    track_radius: f32,
    thumb_radius: f32,
) -> SwitchStyle {
    let theme = switch.theme;
    let track = track_color(switch, status.checked);
    let thumb = thumb_color(switch, status.checked);

    // The web component paints a transparent border over the track's own
    // background (or `border-primary` when checked), so the border reads as an
    // extension of the track until a destructive state overrides it.
    let border = if status.invalid {
        let destructive = theme.semantic_color(SemanticColor::Destructive);
        if theme.is_dark() {
            with_alpha(destructive, DARK_INVALID_BORDER_ALPHA)
        } else {
            destructive
        }
    } else {
        track
    };

    let ring = ring_color(theme, status);
    let opacity = if status.disabled {
        DISABLED_OPACITY
    } else {
        1.0
    };

    SwitchStyle {
        track: with_alpha(track, opacity),
        border: with_alpha(border, opacity),
        border_width: metrics.border_width,
        track_radius,
        thumb: with_alpha(thumb, opacity),
        thumb_radius,
        ring: ring.map(|ring| with_alpha(ring, opacity)),
        ring_width: metrics.ring_width,
    }
}

fn track_color<Message>(switch: &Switch<'_, Message>, checked: bool) -> Color {
    let theme = switch.theme;

    if checked {
        return switch.checked_color.unwrap_or_else(|| match switch.color {
            Some(accent) => theme.color_with_accent(accent, SemanticColor::Primary),
            None => theme.palette.primary,
        });
    }

    switch.track_color.unwrap_or_else(|| {
        let input = theme.semantic_color(SemanticColor::Input);
        if theme.is_dark() {
            with_alpha(input, DARK_UNCHECKED_TRACK_ALPHA)
        } else {
            input
        }
    })
}

fn thumb_color<Message>(switch: &Switch<'_, Message>, checked: bool) -> Color {
    let theme = switch.theme;

    switch.thumb_color.unwrap_or_else(|| {
        if !theme.is_dark() {
            return theme.semantic_color(SemanticColor::Background);
        }

        if checked {
            match switch.color {
                Some(accent) => theme.color_with_accent(accent, SemanticColor::PrimaryForeground),
                None => theme.palette.primary_foreground,
            }
        } else {
            theme.semantic_color(SemanticColor::Foreground)
        }
    })
}

/// `aria-invalid` outranks `focus-visible`, matching the CSS cascade order.
fn ring_color(theme: &Theme, status: SwitchStatus) -> Option<Color> {
    if status.invalid {
        let alpha = if theme.is_dark() { 0.4 } else { 0.2 };
        return Some(with_alpha(
            theme.semantic_color(SemanticColor::Destructive),
            alpha,
        ));
    }

    if status.focused {
        return Some(with_alpha(
            theme.semantic_color(SemanticColor::Ring),
            theme.style.switch().ring_opacity,
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
