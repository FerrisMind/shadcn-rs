//! Style-pack recipes for the input-group root and its slots.

use crate::iced_compat::border::Border;
use crate::iced_compat::widget::{container, text_editor};
use crate::iced_compat::{Background, Color, Padding, Shadow, Vector};

use shadcn_common::{ComponentRadius, StyleId};
use twill_core::prelude::theme::SemanticColor;

use super::types::{InputGroupAddonAlign, InputGroupRadius, InputGroupTextareaProps};
use crate::recipes::component_radius_px;
use crate::theme::Theme;

/// Metrics that differ between shadcn-svelte style packs.
#[derive(Debug, Clone, Copy)]
struct PackRecipe {
    addon_inline_px: f32,
    addon_block_px: f32,
    addon_vertical_px: f32,
    addon_block_start_top_px: f32,
    addon_block_end_bottom_px: f32,
    addon_spacing_px: f32,
    text_size_px: f32,
    fill_alpha_light: f32,
    fill_alpha_dark: f32,
    default_radius: ComponentRadius,
    bordered: bool,
    disabled_fill: bool,
    focus_ring_px: f32,
    shadow: bool,
}

const VEGA: PackRecipe = PackRecipe {
    addon_inline_px: 8.0,
    addon_block_px: 10.0,
    addon_vertical_px: 6.0,
    addon_block_start_top_px: 8.0,
    addon_block_end_bottom_px: 8.0,
    addon_spacing_px: 8.0,
    text_size_px: 14.0,
    fill_alpha_light: 0.0,
    fill_alpha_dark: 0.3,
    default_radius: ComponentRadius::Md,
    bordered: true,
    disabled_fill: false,
    focus_ring_px: 3.0,
    shadow: true,
};

fn pack_recipe(style: StyleId) -> PackRecipe {
    match style {
        StyleId::Vega => VEGA,
        StyleId::Nova => PackRecipe {
            disabled_fill: true,
            default_radius: ComponentRadius::Lg,
            ..VEGA
        },
        StyleId::Maia => PackRecipe {
            addon_inline_px: 12.0,
            addon_block_px: 12.0,
            addon_vertical_px: 8.0,
            addon_block_start_top_px: 12.0,
            addon_block_end_bottom_px: 12.0,
            fill_alpha_light: 0.3,
            fill_alpha_dark: 0.3,
            default_radius: ComponentRadius::S4xl,
            shadow: false,
            ..VEGA
        },
        StyleId::Lyra => PackRecipe {
            text_size_px: 12.0,
            default_radius: ComponentRadius::None,
            disabled_fill: true,
            focus_ring_px: 1.0,
            shadow: false,
            ..VEGA
        },
        StyleId::Mira => PackRecipe {
            addon_inline_px: 8.0,
            addon_block_px: 8.0,
            addon_vertical_px: 8.0,
            addon_block_start_top_px: 8.0,
            addon_block_end_bottom_px: 8.0,
            addon_spacing_px: 4.0,
            text_size_px: 12.0,
            fill_alpha_light: 0.2,
            fill_alpha_dark: 0.3,
            default_radius: ComponentRadius::Md,
            focus_ring_px: 2.0,
            shadow: false,
            ..VEGA
        },
        StyleId::Luma => PackRecipe {
            addon_inline_px: 12.0,
            addon_block_px: 12.0,
            addon_vertical_px: 8.0,
            addon_block_start_top_px: 14.0,
            addon_block_end_bottom_px: 14.0,
            fill_alpha_light: 0.5,
            fill_alpha_dark: 0.5,
            default_radius: ComponentRadius::S4xl,
            bordered: false,
            shadow: false,
            ..VEGA
        },
        StyleId::Sera => PackRecipe {
            addon_inline_px: 0.0,
            addon_block_px: 0.0,
            addon_vertical_px: 8.0,
            addon_block_start_top_px: 14.0,
            addon_block_end_bottom_px: 14.0,
            fill_alpha_light: 0.0,
            fill_alpha_dark: 0.0,
            default_radius: ComponentRadius::None,
            focus_ring_px: 0.0,
            shadow: false,
            ..VEGA
        },
        StyleId::Rhea => PackRecipe {
            addon_inline_px: 8.0,
            addon_block_px: 10.0,
            fill_alpha_light: 0.5,
            fill_alpha_dark: 0.5,
            default_radius: ComponentRadius::S2xl,
            bordered: false,
            shadow: false,
            ..VEGA
        },
    }
}

fn recipe(theme: &Theme) -> PackRecipe {
    pack_recipe(theme.style_id())
}

/// Resolves the outer group surface, border, focus ring, and disabled state.
pub(super) fn resolve_group_style(
    theme: &Theme,
    radius: Option<InputGroupRadius>,
    invalid: bool,
    disabled: bool,
    focused: bool,
) -> container::Style {
    let pack = recipe(theme);
    let input = theme.semantic_color(SemanticColor::Input);
    let foreground = theme.semantic_color(SemanticColor::Foreground);
    let muted_foreground = theme.semantic_color(SemanticColor::MutedForeground);

    let fill_alpha = if theme.is_dark() {
        pack.fill_alpha_dark
    } else {
        pack.fill_alpha_light
    };
    let mut background = with_alpha(input, input.a * fill_alpha);
    let mut border_color = if pack.bordered || uses_bottom_border(theme) {
        input
    } else {
        Color::TRANSPARENT
    };
    let mut text_color = foreground;
    let mut shadow = if pack.shadow {
        Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.05),
            offset: Vector::new(0.0, 1.0),
            blur_radius: 2.0,
        }
    } else {
        Shadow::default()
    };

    if focused && (pack.focus_ring_px > 0.0 || uses_bottom_border(theme)) {
        border_color = theme.semantic_color(SemanticColor::Ring);
        if pack.focus_ring_px > 0.0 {
            shadow = ring_shadow(
                theme.semantic_color(SemanticColor::Ring),
                if theme.is_dark() { 0.4 } else { 0.5 },
                pack.focus_ring_px,
            );
        }
    }

    // The invalid treatment intentionally wins over the focus treatment, as
    // it does in the CSS `has([aria-invalid=true])` cascade.
    if invalid {
        let destructive = theme.semantic_color(SemanticColor::Destructive);
        border_color = if theme.is_dark() {
            with_alpha(destructive, destructive.a * 0.5)
        } else {
            destructive
        };
        shadow = ring_shadow(
            destructive,
            if theme.is_dark() { 0.4 } else { 0.2 },
            pack.focus_ring_px,
        );
    }

    if disabled {
        if pack.disabled_fill {
            background = with_alpha(input, input.a * if theme.is_dark() { 0.8 } else { 0.5 });
        } else {
            background = with_alpha(background, background.a * 0.5);
        }
        border_color = with_alpha(border_color, border_color.a * 0.5);
        text_color = with_alpha(muted_foreground, muted_foreground.a * 0.5);
        shadow = Shadow::default();
    }

    container::Style {
        background: Some(Background::Color(background)),
        text_color: Some(text_color),
        border: Border {
            radius: resolve_radius_px(theme, radius, pack.default_radius).into(),
            width: 1.0,
            color: border_color,
        },
        shadow,
        snap: true,
    }
}

/// Padding for a slot, following the active style pack's addon recipe.
pub(super) fn addon_padding(theme: &Theme, align: InputGroupAddonAlign) -> Padding {
    let pack = recipe(theme);
    let vertical = pack.addon_vertical_px;

    match align {
        InputGroupAddonAlign::InlineStart => Padding {
            top: vertical,
            right: 0.0,
            bottom: vertical,
            left: pack.addon_inline_px,
        },
        InputGroupAddonAlign::InlineEnd => Padding {
            top: vertical,
            right: pack.addon_inline_px,
            bottom: vertical,
            left: 0.0,
        },
        InputGroupAddonAlign::BlockStart => Padding {
            top: pack.addon_block_start_top_px,
            right: pack.addon_block_px,
            bottom: vertical,
            left: pack.addon_block_px,
        },
        InputGroupAddonAlign::BlockEnd => Padding {
            top: vertical,
            right: pack.addon_block_px,
            bottom: pack.addon_block_end_bottom_px,
            left: pack.addon_block_px,
        },
    }
}

pub(super) fn addon_spacing(theme: &Theme) -> f32 {
    recipe(theme).addon_spacing_px
}

/// Whether the active pack paints only the lower edge of the group border.
pub(super) fn uses_bottom_border(theme: &Theme) -> bool {
    matches!(theme.style_id(), StyleId::Sera)
}

/// Text size used by addons and `InputGroupText`.
pub(super) fn addon_text_size(theme: &Theme) -> f32 {
    recipe(theme).text_size_px
}

/// Maps the input-group size ladder to a textarea's default padding.
pub(super) fn textarea_padding(theme: &Theme, props: InputGroupTextareaProps) -> [f32; 2] {
    props.padding.unwrap_or_else(|| {
        let horizontal: f32 = match theme.style_id() {
            StyleId::Maia | StyleId::Luma => 12.0,
            StyleId::Mira => 8.0,
            StyleId::Sera => 0.0,
            StyleId::Vega | StyleId::Nova | StyleId::Lyra | StyleId::Rhea => 10.0,
        };
        let vertical: f32 = match theme.style_id() {
            StyleId::Luma | StyleId::Sera => 10.0,
            StyleId::Vega
            | StyleId::Nova
            | StyleId::Maia
            | StyleId::Lyra
            | StyleId::Mira
            | StyleId::Rhea => 8.0,
        };

        match props.size {
            crate::components::input::InputSize::Sm => [vertical.max(6.0), horizontal],
            crate::components::input::InputSize::Default => [vertical, horizontal],
            crate::components::input::InputSize::Lg => [vertical + 2.0, horizontal + 2.0],
        }
    })
}

/// Text size for the textarea control.
pub(super) fn textarea_text_size(theme: &Theme, props: InputGroupTextareaProps) -> f32 {
    let pack_size: f32 = match theme.style_id() {
        StyleId::Lyra | StyleId::Mira => 12.0,
        StyleId::Vega
        | StyleId::Nova
        | StyleId::Maia
        | StyleId::Luma
        | StyleId::Sera
        | StyleId::Rhea => 14.0,
    };

    match props.size {
        crate::components::input::InputSize::Sm => (pack_size - 1.0).max(1.0),
        crate::components::input::InputSize::Default => pack_size,
        crate::components::input::InputSize::Lg => pack_size + 2.0,
    }
}

/// Minimum textarea height, using explicit rows when supplied.
pub(super) fn textarea_min_height(theme: &Theme, props: InputGroupTextareaProps) -> f32 {
    let padding = textarea_padding(theme, props);
    if let Some(rows) = props.rows {
        return textarea_text_size(theme, props) * 1.4 * rows.max(1) as f32 + padding[0] * 2.0;
    }

    match props.size {
        crate::components::input::InputSize::Sm => 64.0,
        crate::components::input::InputSize::Default => 64.0,
        crate::components::input::InputSize::Lg => 96.0,
    }
}

/// Maximum textarea height, using explicit row limits when supplied.
pub(super) fn textarea_max_height(theme: &Theme, props: InputGroupTextareaProps) -> Option<f32> {
    let rows = props.max_rows?;
    let padding = textarea_padding(theme, props);
    Some(textarea_text_size(theme, props) * 1.4 * rows.max(1) as f32 + padding[0] * 2.0)
}

/// Resolves a transparent text-editor style so the group owns the surface.
pub(super) fn resolve_textarea_style(
    theme: &Theme,
    props: InputGroupTextareaProps,
    status: text_editor::Status,
) -> text_editor::Style {
    let disabled = props.disabled || matches!(status, text_editor::Status::Disabled);
    let mut value = theme.semantic_color(SemanticColor::Foreground);
    let mut placeholder = theme.semantic_color(SemanticColor::MutedForeground);
    let mut selection = theme.semantic_color(SemanticColor::Primary);

    if disabled || props.read_only {
        value = theme.semantic_color(SemanticColor::MutedForeground);
        placeholder = theme.semantic_color(SemanticColor::MutedForeground);
        selection = theme.semantic_color(SemanticColor::Muted);
    }

    if props.invalid && !disabled {
        value = theme.semantic_color(SemanticColor::Destructive);
    }

    text_editor::Style {
        background: Background::Color(Color::TRANSPARENT),
        border: Border::default(),
        placeholder,
        value,
        selection,
    }
}

/// Converts a radius preset to the active style pack's pixel value.
pub(super) fn radius_px(theme: &Theme, radius: InputGroupRadius) -> f32 {
    match radius {
        InputGroupRadius::None => 0.0,
        InputGroupRadius::Small => theme.style.twill_radius_sm.px_value(),
        InputGroupRadius::Medium => theme.style.twill_radius_md.px_value(),
        InputGroupRadius::Large => theme.style.twill_radius_lg.px_value(),
        InputGroupRadius::Full => 9999.0,
    }
}

fn resolve_radius_px(
    theme: &Theme,
    radius: Option<InputGroupRadius>,
    pack_default: ComponentRadius,
) -> f32 {
    match radius {
        Some(radius) => radius_px(theme, radius),
        None => component_radius_px(theme, pack_default),
    }
}

fn ring_shadow(color: Color, alpha: f32, width: f32) -> Shadow {
    Shadow {
        color: with_alpha(color, color.a * alpha),
        offset: Vector::new(0.0, 0.0),
        blur_radius: width,
    }
}

fn with_alpha(color: Color, alpha: f32) -> Color {
    Color {
        a: alpha.clamp(0.0, 1.0),
        ..color
    }
}
