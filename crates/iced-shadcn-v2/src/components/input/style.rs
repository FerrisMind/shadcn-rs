//! Mapping of `.cn-input` style-pack rules to iced `text_input` styles.
//!
//! Each pack ships its own `.cn-input` recipe (height, radius, fill, border,
//! focus ring). iced's `text_input::Style` has no outer-shadow slot, so the
//! translucent `focus-visible:ring-*` halo is approximated by recoloring the
//! border with `ring` (`destructive` when invalid), exactly like the solid
//! `focus-visible:border-ring` part of the web rule. Sera's underline-only
//! border cannot be expressed either — it degrades to a full hairline box.

use crate::iced_compat::border::Border;
use crate::iced_compat::widget::text_input;
use crate::iced_compat::{Background, Color};

use shadcn_common::{AccentColor, ComponentRadius, StyleId};
use twill_core::prelude::theme::SemanticColor;

use super::types::InputRadius;
use crate::recipes::component_radius_px;
use crate::theme::Theme;

/// `disabled:opacity-50` from the base `cn-input` class.
const DISABLED_OPACITY: f32 = 0.5;
/// `dark:aria-invalid:border-destructive/50`.
const DARK_INVALID_BORDER_ALPHA: f32 = 0.5;
/// Selection wash over the value text (web `::selection`).
const SELECTION_ALPHA: f32 = 0.4;

/// Status-independent `.cn-input` numbers of one style pack.
#[derive(Debug, Clone, Copy)]
struct PackRecipe {
    /// `px-*` in px (`px-2.5` → 10).
    pad_x_px: f32,
    /// Value / placeholder text size (`md:text-sm` → 14, `md:text-xs` → 12).
    text_size_px: f32,
    /// Corner treatment when the builder does not override it.
    default_radius: ComponentRadius,
    /// `bg-input/N` alpha in light mode (0 = `bg-transparent`).
    fill_alpha_light: f32,
    /// `dark:bg-input/N` alpha.
    fill_alpha_dark: f32,
    /// Whether the resting border is painted (`border-input` vs
    /// `border-transparent`).
    bordered: bool,
    /// `disabled:bg-input/50 dark:disabled:bg-input/80` (Nova, Lyra).
    disabled_fill: bool,
}

/// Vega `.cn-input` used as the fallback for unknown future packs.
const VEGA: PackRecipe = PackRecipe {
    pad_x_px: 10.0,
    text_size_px: 14.0,
    default_radius: ComponentRadius::Md,
    fill_alpha_light: 0.0,
    fill_alpha_dark: 0.3,
    bordered: true,
    disabled_fill: false,
};

fn pack_recipe(style: StyleId) -> PackRecipe {
    match style {
        StyleId::Vega => VEGA,
        // `h-8 rounded-lg px-2.5 disabled:bg-input/50`
        StyleId::Nova => PackRecipe {
            default_radius: ComponentRadius::Lg,
            disabled_fill: true,
            ..VEGA
        },
        // `bg-input/30 h-9 rounded-4xl px-3`
        StyleId::Maia => PackRecipe {
            pad_x_px: 12.0,
            default_radius: ComponentRadius::S4xl,
            fill_alpha_light: 0.3,
            ..VEGA
        },
        // `h-8 rounded-none px-2.5 text-xs disabled:bg-input/50`
        StyleId::Lyra => PackRecipe {
            text_size_px: 12.0,
            default_radius: ComponentRadius::None,
            disabled_fill: true,
            ..VEGA
        },
        // `bg-input/20 h-7 rounded-md px-2 md:text-xs`
        StyleId::Mira => PackRecipe {
            pad_x_px: 8.0,
            text_size_px: 12.0,
            default_radius: ComponentRadius::Md,
            fill_alpha_light: 0.2,
            ..VEGA
        },
        // `bg-input/50 h-9 rounded-3xl border-transparent px-3`
        StyleId::Luma => PackRecipe {
            pad_x_px: 12.0,
            default_radius: ComponentRadius::S3xl,
            fill_alpha_light: 0.5,
            fill_alpha_dark: 0.5,
            bordered: false,
            ..VEGA
        },
        // Web Sera is underline-only (`border-b-input`, `px-0`); the
        // underline is drawn by the From<Input> for Element wrapper via
        // resolve_underline_color; the text_input itself gets a transparent
        // border so it draws no box.
        StyleId::Sera => PackRecipe {
            default_radius: ComponentRadius::None,
            fill_alpha_dark: 0.0,
            ..VEGA
        },
        // `bg-input/50 h-8 rounded-2xl border-transparent px-2.5`
        StyleId::Rhea => PackRecipe {
            default_radius: ComponentRadius::S2xl,
            fill_alpha_light: 0.5,
            fill_alpha_dark: 0.5,
            bordered: false,
            ..VEGA
        },
    }
}

/// `px-*` of the active pack, consumed by the geometry module.
pub(super) fn pack_pad_x(theme: &Theme) -> f32 {
    pack_recipe(theme.style_id()).pad_x_px
}

pub(super) fn group_slot_pad_x(theme: &Theme) -> f32 {
    match theme.style_id() {
        StyleId::Sera => 8.0,
        StyleId::Vega
        | StyleId::Nova
        | StyleId::Maia
        | StyleId::Lyra
        | StyleId::Mira
        | StyleId::Luma
        | StyleId::Rhea => 6.0,
    }
}

/// `md:text-sm` / `md:text-xs` of the active pack.
pub(super) fn pack_text_size(theme: &Theme) -> f32 {
    pack_recipe(theme.style_id()).text_size_px
}

/// Sera-style underline-only: the `border-b-input` rule paints only the
/// bottom hairline. iced's `text_input::Style` always draws a full box
/// border, so the strategy is: give text_input a transparent border, then
/// draw the bottom line in the From<Input> for Element wrapper.
pub(super) fn uses_underline_only(theme: &Theme) -> bool {
    matches!(theme.style_id(), StyleId::Sera)
}

/// The semantic border color for the underline (or full border elsewhere).
///
/// Exposed so the wrapper can paint the bottom hairline with the same
/// resolved color (ring when focused, destructive when invalid, etc.)
/// without duplicating the resolution logic.
pub(super) fn resolve_underline_color(
    theme: &Theme,
    color: Option<AccentColor>,
    invalid: bool,
    disabled: bool,
    status: text_input::Status,
) -> Color {
    let input = theme.semantic_color(SemanticColor::Input);
    let mut border_color = input;

    if matches!(status, text_input::Status::Focused { .. }) {
        border_color = ring_color(theme, color);
    }

    if invalid {
        let destructive = theme.semantic_color(SemanticColor::Destructive);
        border_color = if theme.is_dark() {
            with_alpha(destructive, destructive.a * DARK_INVALID_BORDER_ALPHA)
        } else {
            destructive
        };
    }

    let disabled = disabled && status == text_input::Status::Disabled;
    if disabled {
        border_color = with_alpha(border_color, border_color.a * DISABLED_OPACITY);
    }

    border_color
}

pub(super) fn resolve_input_style(
    theme: &Theme,
    radius: Option<InputRadius>,
    color: Option<AccentColor>,
    invalid: bool,
    disabled: bool,
    status: text_input::Status,
) -> text_input::Style {
    let pack = pack_recipe(theme.style_id());
    let input = theme.semantic_color(SemanticColor::Input);
    let disabled = disabled && status == text_input::Status::Disabled;

    let fill_alpha = if theme.is_dark() {
        pack.fill_alpha_dark
    } else {
        pack.fill_alpha_light
    };
    let mut background = with_alpha(input, input.a * fill_alpha);
    let mut border_color = if pack.bordered {
        input
    } else {
        Color::TRANSPARENT
    };
    let mut value = theme.semantic_color(SemanticColor::Foreground);
    let mut placeholder = theme.semantic_color(SemanticColor::MutedForeground);
    let mut icon = placeholder;

    if matches!(status, text_input::Status::Focused { .. }) {
        // `focus-visible:border-ring` (+ the ring halo approximation).
        border_color = ring_color(theme, color);
    }

    // The CSS cascade lets `aria-invalid` outrank `focus-visible`.
    if invalid {
        let destructive = theme.semantic_color(SemanticColor::Destructive);
        border_color = if theme.is_dark() {
            with_alpha(destructive, destructive.a * DARK_INVALID_BORDER_ALPHA)
        } else {
            destructive
        };
    }

    if disabled {
        if pack.disabled_fill {
            let alpha = if theme.is_dark() { 0.8 } else { 0.5 };
            background = with_alpha(input, input.a * alpha);
        } else {
            background = with_alpha(background, background.a * DISABLED_OPACITY);
        }
        border_color = with_alpha(border_color, border_color.a * DISABLED_OPACITY);
        value = with_alpha(value, value.a * DISABLED_OPACITY);
        placeholder = with_alpha(placeholder, placeholder.a * DISABLED_OPACITY);
        icon = with_alpha(icon, icon.a * DISABLED_OPACITY);
    }

    text_input::Style {
        background: Background::Color(background),
        border: Border {
            radius: resolve_radius_px(theme, radius, pack.default_radius).into(),
            width: if uses_underline_only(theme) { 0.0 } else { 1.0 },
            color: if uses_underline_only(theme) {
                Color::TRANSPARENT
            } else {
                border_color
            },
        },
        icon,
        placeholder,
        value,
        selection: with_alpha(primary_color(theme, color), SELECTION_ALPHA),
    }
}

fn ring_color(theme: &Theme, color: Option<AccentColor>) -> Color {
    match color {
        None => theme.semantic_color(SemanticColor::Ring),
        // Accent overlays keep the neutral `ring` token, so an explicit
        // per-input accent recolors the focus border with the accent primary.
        Some(accent) => theme.color_with_accent(accent, SemanticColor::Primary),
    }
}

fn primary_color(theme: &Theme, color: Option<AccentColor>) -> Color {
    match color {
        None => theme.palette.primary,
        Some(accent) => theme.color_with_accent(accent, SemanticColor::Primary),
    }
}

fn resolve_radius_px(
    theme: &Theme,
    radius: Option<InputRadius>,
    pack_default: ComponentRadius,
) -> f32 {
    match radius {
        Some(radius) => radius_px(theme, radius),
        None => component_radius_px(theme, pack_default),
    }
}

fn radius_px(theme: &Theme, radius: InputRadius) -> f32 {
    match radius {
        InputRadius::None => 0.0,
        InputRadius::Small => theme.style.twill_radius_sm.px_value(),
        InputRadius::Medium => theme.style.twill_radius_md.px_value(),
        InputRadius::Large => theme.style.twill_radius_lg.px_value(),
        InputRadius::Full => 9999.0,
    }
}

fn with_alpha(color: Color, alpha: f32) -> Color {
    Color {
        a: alpha.clamp(0.0, 1.0),
        ..color
    }
}
