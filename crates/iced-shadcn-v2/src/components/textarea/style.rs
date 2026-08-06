//! Mapping of `.cn-textarea` style-pack rules to iced `text_editor` styles.
//!
//! Each pack ships its own `.cn-textarea` recipe (radius, fill, border, focus
//! ring, shadow). The tokens come from the shared [`TextareaRecipe`] in
//! `shadcn-common`, so iced and egui share one source of truth. iced's
//! `text_editor::Style` has no outer-shadow slot, so the translucent
//! `focus-visible:ring-*` halo is approximated by recoloring the border with
//! `ring` (`destructive` when invalid), exactly like the solid
//! `focus-visible:border-ring` part of the web rule. Sera's underline-only
//! border degrades to a transparent box plus a bottom hairline drawn by the
//! `From<Textarea> for Element` wrapper.

use crate::iced_compat::border::Border;
use crate::iced_compat::widget::text_editor;
use crate::iced_compat::{Background, Color};

use shadcn_common::{
    AccentColor, ComponentRadius, DARK_INVALID_BORDER_ALPHA, DISABLED_OPACITY, SELECTION_ALPHA,
    StyleId, TextareaRecipe,
};
use twill_core::prelude::theme::SemanticColor;

use super::types::{TextareaRadius, TextareaResize};
use crate::recipes::component_radius_px;
use crate::theme::Theme;

/// Resolves the active pack's [`TextareaRecipe`].
pub(super) fn pack_recipe(theme: &Theme) -> TextareaRecipe {
    theme.style.textarea()
}

/// Sera-style underline-only: the `border-b-input` rule paints only the bottom
/// hairline. iced's `text_editor::Style` always draws a full box border, so the
/// strategy is: give the editor a transparent border, then draw the bottom
/// line in the `From<Textarea> for Element` wrapper.
pub(super) fn uses_underline_only(theme: &Theme) -> bool {
    matches!(theme.style_id(), StyleId::Sera) && pack_recipe(theme).underline_only
}

/// The semantic border color for the underline (or full border elsewhere).
/// Exposed so the wrapper can paint the bottom hairline with the same resolved
/// color (ring when focused, destructive when invalid, etc.) without
/// duplicating the resolution logic.
pub(super) fn resolve_underline_color(
    theme: &Theme,
    color: Option<AccentColor>,
    invalid: bool,
    disabled: bool,
    status: text_editor::Status,
) -> Color {
    let input = theme.semantic_color(SemanticColor::Input);
    let mut border_color = input;

    if matches!(status, text_editor::Status::Focused { .. }) {
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

    let disabled = disabled && matches!(status, text_editor::Status::Disabled);
    if disabled {
        border_color = with_alpha(border_color, border_color.a * DISABLED_OPACITY);
    }

    border_color
}

/// Resolves the full `text_editor::Style` for the textarea surface.
///
/// This owns the border, fill, focus ring, invalid and disabled treatments —
/// the opposite of [`crate::components::input_group`] which renders the editor
/// transparent and lets the group own the surface.
#[allow(clippy::too_many_arguments)]
pub(super) fn resolve_textarea_style(
    theme: &Theme,
    recipe: TextareaRecipe,
    radius: Option<TextareaRadius>,
    color: Option<AccentColor>,
    invalid: bool,
    disabled: bool,
    read_only: bool,
    status: text_editor::Status,
) -> text_editor::Style {
    let input = theme.semantic_color(SemanticColor::Input);
    let disabled = disabled && matches!(status, text_editor::Status::Disabled);

    let fill_alpha = if theme.is_dark() {
        recipe.fill_alpha_dark
    } else {
        recipe.fill_alpha_light
    };
    let mut background = with_alpha(input, input.a * fill_alpha);
    let mut border_color = if recipe.bordered {
        input
    } else {
        Color::TRANSPARENT
    };
    let mut value = theme.semantic_color(SemanticColor::Foreground);
    let mut placeholder = theme.semantic_color(SemanticColor::MutedForeground);
    let mut selection = with_alpha(primary_color(theme, color), SELECTION_ALPHA);

    if matches!(status, text_editor::Status::Focused { .. }) {
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

    if disabled || read_only {
        value = theme.semantic_color(SemanticColor::MutedForeground);
        placeholder = theme.semantic_color(SemanticColor::MutedForeground);
        selection = theme.semantic_color(SemanticColor::Muted);
    }

    if disabled {
        if recipe.disabled_fill {
            let alpha = if theme.is_dark() { 0.8 } else { 0.5 };
            background = with_alpha(input, input.a * alpha);
        } else {
            background = with_alpha(background, background.a * DISABLED_OPACITY);
        }
        border_color = with_alpha(border_color, border_color.a * DISABLED_OPACITY);
        value = with_alpha(value, value.a * DISABLED_OPACITY);
        placeholder = with_alpha(placeholder, placeholder.a * DISABLED_OPACITY);
        selection = with_alpha(selection, selection.a * DISABLED_OPACITY);
    }

    let underline_only = uses_underline_only(theme);

    text_editor::Style {
        background: Background::Color(background),
        border: Border {
            radius: resolve_radius_px(theme, radius, recipe.default_radius).into(),
            width: if underline_only { 0.0 } else { 1.0 },
            color: if underline_only {
                Color::TRANSPARENT
            } else {
                border_color
            },
        },
        placeholder,
        value,
        selection,
    }
}

/// Extra horizontal padding reserved for an inline addon slot when the
/// textarea is later embedded in an [`crate::InputGroup`]. Mirrors the input
/// group's slot padding so a future migration stays pixel-accurate.
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

/// Whether the resize policy fixes the textarea height.
pub(super) fn fixes_height(resize: TextareaResize) -> bool {
    matches!(resize, TextareaResize::None)
}

fn ring_color(theme: &Theme, color: Option<AccentColor>) -> Color {
    match color {
        None => theme.semantic_color(SemanticColor::Ring),
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
    radius: Option<TextareaRadius>,
    pack_default: ComponentRadius,
) -> f32 {
    match radius {
        Some(radius) => radius_px(theme, radius),
        None => component_radius_px(theme, pack_default),
    }
}

fn radius_px(theme: &Theme, radius: TextareaRadius) -> f32 {
    match radius {
        TextareaRadius::None => 0.0,
        TextareaRadius::Small => theme.style.twill_radius_sm.px_value(),
        TextareaRadius::Medium => theme.style.twill_radius_md.px_value(),
        TextareaRadius::Large => theme.style.twill_radius_lg.px_value(),
        TextareaRadius::Full => 9999.0,
    }
}

fn with_alpha(color: Color, alpha: f32) -> Color {
    Color {
        a: alpha.clamp(0.0, 1.0),
        ..color
    }
}
