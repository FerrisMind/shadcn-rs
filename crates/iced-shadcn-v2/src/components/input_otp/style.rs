//! Mapping of `.cn-input-otp*` style-pack rules to resolved iced colors.
//!
//! Each pack ships its own `.cn-input-otp-slot` recipe (slot size, fill,
//! ring width/alpha, corner treatment). iced has no outset box-shadow, so
//! the `data-[active=true]:ring-*` halo is painted as a border-only quad
//! around the active slot. Sera's underline-only slots
//! (`border-b-input`, transparent background, `gap-1`) keep their web look
//! because the widget draws every border itself.

use crate::iced_compat::Color;

use shadcn_common::{AccentColor, ComponentRadius, StyleId};
use twill_core::prelude::theme::SemanticColor;

use super::types::{InputOtpRadius, InputOtpStatus, InputOtpStyle};
use crate::recipes::component_radius_px;
use crate::theme::Theme;

/// `has-disabled:opacity-50` from the base `cn-input-otp` class.
const DISABLED_OPACITY: f32 = 0.5;
/// `data-[active=true]:aria-invalid:ring-destructive/20`.
const INVALID_RING_ALPHA_LIGHT: f32 = 0.2;
/// `dark:data-[active=true]:aria-invalid:ring-destructive/40`.
const INVALID_RING_ALPHA_DARK: f32 = 0.4;
/// Sera `dark:aria-invalid:border-b-destructive/50`.
const SERA_DARK_INVALID_BORDER_ALPHA: f32 = 0.5;

/// Status-independent `.cn-input-otp-slot` numbers of one style pack.
#[derive(Debug, Clone, Copy)]
pub(super) struct PackRecipe {
    /// `size-*` in px (`size-9` → 36).
    pub(super) slot_size_px: f32,
    /// Character text size (`text-sm` → 14, `text-xs` → 12).
    pub(super) text_size_px: f32,
    /// `data-[active=true]:ring-*` width in px (0 = no ring, Sera).
    pub(super) ring_width: f32,
    /// `ring-ring/N` alpha of the active ring.
    pub(super) ring_alpha: f32,
    /// `bg-input/N` alpha in light mode (0 = transparent).
    pub(super) fill_alpha_light: f32,
    /// `dark:bg-input/N` alpha.
    pub(super) fill_alpha_dark: f32,
    /// Corner treatment when the builder does not override it.
    pub(super) default_radius: ComponentRadius,
    /// Sera's `border-b-input`-only slots (no box border, no ring).
    pub(super) underline_only: bool,
    /// Gap between slots inside one group (`gap-1` on Sera, 0 elsewhere).
    pub(super) slot_gap: f32,
}

/// Vega `.cn-input-otp-slot` used as the fallback for unknown future packs.
const VEGA: PackRecipe = PackRecipe {
    slot_size_px: 36.0,
    text_size_px: 14.0,
    ring_width: 3.0,
    ring_alpha: 0.5,
    fill_alpha_light: 0.0,
    fill_alpha_dark: 0.3,
    default_radius: ComponentRadius::Md,
    underline_only: false,
    slot_gap: 0.0,
};

pub(super) fn pack_recipe(style: StyleId) -> PackRecipe {
    match style {
        // `dark:bg-input/30 size-9 text-sm rounded-*-md ring-3 ring-ring/50`
        StyleId::Vega => VEGA,
        // `dark:bg-input/30 size-8 text-sm rounded-*-lg ring-3 ring-ring/50`
        StyleId::Nova => PackRecipe {
            slot_size_px: 32.0,
            default_radius: ComponentRadius::Lg,
            ..VEGA
        },
        // `bg-input/30 size-9 text-sm rounded-*-4xl`
        StyleId::Maia => PackRecipe {
            fill_alpha_light: 0.3,
            default_radius: ComponentRadius::S4xl,
            ..VEGA
        },
        // `dark:bg-input/30 size-8 text-xs rounded-none ring-1 ring-ring/50`
        StyleId::Lyra => PackRecipe {
            slot_size_px: 32.0,
            text_size_px: 12.0,
            ring_width: 1.0,
            default_radius: ComponentRadius::None,
            ..VEGA
        },
        // `bg-input/20 dark:bg-input/30 size-7 text-xs rounded-*-md ring-2 ring-ring/30`
        StyleId::Mira => PackRecipe {
            slot_size_px: 28.0,
            text_size_px: 12.0,
            ring_width: 2.0,
            ring_alpha: 0.3,
            fill_alpha_light: 0.2,
            default_radius: ComponentRadius::Md,
            ..VEGA
        },
        // `bg-input/50 size-9 text-sm rounded-*-3xl ring-3 ring-ring/30`
        StyleId::Luma => PackRecipe {
            ring_alpha: 0.3,
            fill_alpha_light: 0.5,
            fill_alpha_dark: 0.5,
            default_radius: ComponentRadius::S3xl,
            ..VEGA
        },
        // `border-b-input size-10 bg-transparent text-sm rounded-none` with
        // `gap-1` between slots and no active ring.
        StyleId::Sera => PackRecipe {
            slot_size_px: 40.0,
            ring_width: 0.0,
            fill_alpha_dark: 0.0,
            default_radius: ComponentRadius::None,
            underline_only: true,
            slot_gap: 4.0,
            ..VEGA
        },
        // `bg-input/50 size-8 text-sm rounded-*-2xl ring-3 ring-ring/30`
        StyleId::Rhea => PackRecipe {
            slot_size_px: 32.0,
            ring_alpha: 0.3,
            fill_alpha_light: 0.5,
            fill_alpha_dark: 0.5,
            default_radius: ComponentRadius::S2xl,
            ..VEGA
        },
    }
}

pub(super) fn resolve_style(
    theme: &Theme,
    radius: Option<InputOtpRadius>,
    color: Option<AccentColor>,
    status: InputOtpStatus,
) -> InputOtpStyle {
    let pack = pack_recipe(theme.style_id());
    let input = theme.semantic_color(SemanticColor::Input);
    let foreground = theme.semantic_color(SemanticColor::Foreground);

    let fill_alpha = if theme.is_dark() {
        pack.fill_alpha_dark
    } else {
        pack.fill_alpha_light
    };

    let mut slot_border = input;
    let mut active_border = ring_color(theme, color);
    let mut ring = with_alpha(active_border, pack.ring_alpha);

    // The CSS cascade lets `aria-invalid` outrank the active treatment.
    if status.invalid {
        let destructive = theme.semantic_color(SemanticColor::Destructive);
        slot_border = if pack.underline_only && theme.is_dark() {
            with_alpha(destructive, destructive.a * SERA_DARK_INVALID_BORDER_ALPHA)
        } else {
            destructive
        };
        active_border = slot_border;
        ring = with_alpha(
            destructive,
            if theme.is_dark() {
                INVALID_RING_ALPHA_DARK
            } else {
                INVALID_RING_ALPHA_LIGHT
            },
        );
    }

    let mut style = InputOtpStyle {
        slot_background: with_alpha(input, input.a * fill_alpha),
        slot_border,
        slot_text: foreground,
        active_border,
        ring,
        ring_width: pack.ring_width,
        caret: foreground,
        separator: foreground,
        radius: resolve_radius_px(theme, radius, pack.default_radius),
        underline_only: pack.underline_only,
    };

    if status.disabled {
        style.slot_background = fade(style.slot_background);
        style.slot_border = fade(style.slot_border);
        style.slot_text = fade(style.slot_text);
        style.active_border = fade(style.active_border);
        style.ring = fade(style.ring);
        style.caret = fade(style.caret);
        style.separator = fade(style.separator);
    }

    style
}

fn ring_color(theme: &Theme, color: Option<AccentColor>) -> Color {
    match color {
        None => theme.semantic_color(SemanticColor::Ring),
        // Accent overlays keep the neutral `ring` token, so an explicit
        // per-control accent recolors the active treatment with the accent
        // primary, mirroring `Input::color`.
        Some(accent) => theme.color_with_accent(accent, SemanticColor::Primary),
    }
}

fn resolve_radius_px(
    theme: &Theme,
    radius: Option<InputOtpRadius>,
    pack_default: ComponentRadius,
) -> f32 {
    match radius {
        Some(radius) => radius_px(theme, radius),
        None => component_radius_px(theme, pack_default),
    }
}

fn radius_px(theme: &Theme, radius: InputOtpRadius) -> f32 {
    match radius {
        InputOtpRadius::None => 0.0,
        InputOtpRadius::Small => theme.style.twill_radius_sm.px_value(),
        InputOtpRadius::Medium => theme.style.twill_radius_md.px_value(),
        InputOtpRadius::Large => theme.style.twill_radius_lg.px_value(),
        InputOtpRadius::Full => 9999.0,
    }
}

fn fade(color: Color) -> Color {
    with_alpha(color, color.a * DISABLED_OPACITY)
}

fn with_alpha(color: Color, alpha: f32) -> Color {
    Color {
        a: alpha.clamp(0.0, 1.0),
        ..color
    }
}
