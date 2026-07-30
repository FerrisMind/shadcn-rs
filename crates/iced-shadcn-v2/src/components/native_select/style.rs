//! Native-select style-pack recipes and iced style resolution.

use crate::iced_compat::widget::pick_list;
use crate::iced_compat::{Background, Border, Color};

use shadcn_common::StyleId;
use twill_core::prelude::theme::SemanticColor;

use super::types::{NativeSelectRadius, NativeSelectSize};
use crate::theme::Theme;

const DISABLED_OPACITY: f32 = 0.5;
const DARK_INVALID_BORDER_ALPHA: f32 = 0.5;

#[derive(Debug, Clone, Copy)]
struct PackRecipe {
    pad_left_px: f32,
    pad_right_px: f32,
    pad_vertical_default_px: f32,
    pad_vertical_sm_px: f32,
    icon_size_px: f32,
    icon_right_px: f32,
    text_size_px: f32,
    focus_ring_px: f32,
    default_radius: NativeSelectRadius,
    fill_alpha_light: f32,
    fill_alpha_dark: f32,
    bordered: bool,
}

const VEGA: PackRecipe = PackRecipe {
    pad_left_px: 10.0,
    pad_right_px: 32.0,
    pad_vertical_default_px: 4.0,
    pad_vertical_sm_px: 4.0,
    icon_size_px: 16.0,
    icon_right_px: 10.0,
    text_size_px: 14.0,
    focus_ring_px: 3.0,
    default_radius: NativeSelectRadius::Small,
    fill_alpha_light: 0.0,
    fill_alpha_dark: 0.3,
    bordered: true,
};

fn pack_recipe(style: StyleId) -> PackRecipe {
    match style {
        StyleId::Vega => VEGA,
        StyleId::Nova => PackRecipe {
            pad_vertical_sm_px: 2.0,
            default_radius: NativeSelectRadius::Medium,
            ..VEGA
        },
        StyleId::Maia => PackRecipe {
            pad_left_px: 12.0,
            icon_right_px: 14.0,
            default_radius: NativeSelectRadius::Full,
            fill_alpha_light: 0.3,
            ..VEGA
        },
        StyleId::Lyra => PackRecipe {
            pad_vertical_sm_px: 2.0,
            text_size_px: 12.0,
            focus_ring_px: 1.0,
            default_radius: NativeSelectRadius::None,
            ..VEGA
        },
        StyleId::Mira => PackRecipe {
            pad_left_px: 8.0,
            pad_right_px: 24.0,
            pad_vertical_default_px: 2.0,
            pad_vertical_sm_px: 2.0,
            icon_size_px: 14.0,
            icon_right_px: 6.0,
            text_size_px: 12.0,
            focus_ring_px: 2.0,
            default_radius: NativeSelectRadius::Medium,
            fill_alpha_light: 0.2,
            ..VEGA
        },
        StyleId::Luma => PackRecipe {
            pad_left_px: 12.0,
            default_radius: NativeSelectRadius::Full,
            fill_alpha_light: 0.5,
            fill_alpha_dark: 0.5,
            bordered: false,
            ..VEGA
        },
        StyleId::Sera => PackRecipe {
            pad_left_px: 0.0,
            pad_vertical_default_px: 8.0,
            pad_vertical_sm_px: 8.0,
            icon_size_px: 14.0,
            icon_right_px: 0.0,
            default_radius: NativeSelectRadius::None,
            fill_alpha_dark: 0.0,
            focus_ring_px: 0.0,
            bordered: false,
            ..VEGA
        },
        StyleId::Rhea => PackRecipe {
            pad_left_px: 10.0,
            default_radius: NativeSelectRadius::Large,
            fill_alpha_light: 0.5,
            fill_alpha_dark: 0.5,
            bordered: false,
            ..VEGA
        },
    }
}

pub(super) fn text_size(theme: &Theme) -> f32 {
    pack_recipe(theme.style_id()).text_size_px
}

pub(super) fn text_size_for(theme: &Theme, size: NativeSelectSize) -> f32 {
    if matches!(
        (theme.style_id(), size),
        (StyleId::Mira, NativeSelectSize::Sm)
    ) {
        10.0
    } else {
        text_size(theme)
    }
}

pub(super) fn icon_size(theme: &Theme) -> f32 {
    pack_recipe(theme.style_id()).icon_size_px
}

pub(super) fn icon_size_for(theme: &Theme, size: NativeSelectSize) -> f32 {
    if matches!(
        (theme.style_id(), size),
        (StyleId::Mira, NativeSelectSize::Sm)
    ) {
        12.0
    } else {
        icon_size(theme)
    }
}

pub(super) fn icon_right(theme: &Theme) -> f32 {
    pack_recipe(theme.style_id()).icon_right_px
}

pub(super) fn focus_ring_px(theme: &Theme) -> f32 {
    pack_recipe(theme.style_id()).focus_ring_px
}

pub(super) fn padding(theme: &Theme, size: NativeSelectSize) -> crate::iced_compat::Padding {
    let recipe = pack_recipe(theme.style_id());
    let vertical = match size {
        NativeSelectSize::Sm => recipe.pad_vertical_sm_px,
        NativeSelectSize::Default => recipe.pad_vertical_default_px,
    };

    crate::iced_compat::Padding {
        top: vertical,
        right: recipe.pad_right_px,
        bottom: vertical,
        left: recipe.pad_left_px,
    }
}

pub(super) fn line_height_px(theme: &Theme, size: NativeSelectSize, text_size: f32) -> f32 {
    let default_text_size = text_size_for(theme, size);
    let default_line_height = match theme.style_id() {
        StyleId::Lyra => 16.0,
        StyleId::Mira => 12.0 * 1.625,
        _ => 20.0,
    };

    if (text_size - default_text_size).abs() < f32::EPSILON {
        default_line_height
    } else {
        text_size + 6.0
    }
}

pub(super) fn radius_px(theme: &Theme, radius: Option<NativeSelectRadius>) -> f32 {
    match radius.unwrap_or_else(|| pack_recipe(theme.style_id()).default_radius) {
        NativeSelectRadius::None => 0.0,
        NativeSelectRadius::Small => theme.style.twill_radius_sm.px_value(),
        NativeSelectRadius::Medium => theme.style.twill_radius_md.px_value(),
        NativeSelectRadius::Large => theme.style.twill_radius_lg.px_value(),
        NativeSelectRadius::Full => 9999.0,
    }
}

pub(super) fn resolve(
    theme: &Theme,
    radius: Option<NativeSelectRadius>,
    invalid: bool,
    disabled: bool,
    status: pick_list::Status,
) -> pick_list::Style {
    let recipe = pack_recipe(theme.style_id());
    let input = theme.semantic_color(SemanticColor::Input);
    let mut background = with_alpha(
        input,
        input.a
            * if theme.is_dark() {
                recipe.fill_alpha_dark
            } else {
                recipe.fill_alpha_light
            },
    );
    let mut border = if recipe.bordered || uses_bottom_border(theme) {
        input
    } else {
        Color::TRANSPARENT
    };
    let mut text = theme.semantic_color(SemanticColor::Foreground);
    let mut placeholder = theme.semantic_color(SemanticColor::MutedForeground);
    let mut handle = placeholder;

    let hovered = matches!(
        status,
        pick_list::Status::Hovered | pick_list::Status::Opened { is_hovered: true }
    );
    if theme.is_dark()
        && hovered
        && matches!(
            theme.style_id(),
            StyleId::Vega | StyleId::Nova | StyleId::Maia | StyleId::Lyra
        )
    {
        background = with_alpha(input, input.a * 0.5);
    }

    if matches!(status, pick_list::Status::Opened { .. }) {
        border = theme.semantic_color(SemanticColor::Ring);
    }

    if invalid {
        let destructive = theme.semantic_color(SemanticColor::Destructive);
        border = if theme.is_dark() {
            with_alpha(destructive, destructive.a * DARK_INVALID_BORDER_ALPHA)
        } else {
            destructive
        };
    }

    if disabled {
        background = with_alpha(background, background.a * DISABLED_OPACITY);
        border = with_alpha(border, border.a * DISABLED_OPACITY);
        text = with_alpha(text, text.a * DISABLED_OPACITY);
        placeholder = with_alpha(placeholder, placeholder.a * DISABLED_OPACITY);
        handle = with_alpha(handle, handle.a * DISABLED_OPACITY);
    }

    pick_list::Style {
        text_color: text,
        placeholder_color: placeholder,
        handle_color: handle,
        background: Background::Color(background),
        border: Border {
            color: border,
            width: 1.0,
            radius: radius_px(theme, radius).into(),
        },
    }
}

pub(super) fn menu_style(
    theme: &Theme,
    radius: Option<NativeSelectRadius>,
) -> iced_widget::overlay::menu::Style {
    let palette = theme.palette;
    iced_widget::overlay::menu::Style {
        background: Background::Color(palette.card),
        border: Border {
            color: palette.border,
            width: 1.0,
            radius: radius_px(theme, radius).into(),
        },
        text_color: palette.foreground,
        selected_text_color: palette.foreground,
        selected_background: Background::Color(with_alpha(palette.accent, 0.5)),
        shadow: Default::default(),
    }
}

pub(super) fn uses_bottom_border(theme: &Theme) -> bool {
    matches!(theme.style_id(), StyleId::Sera)
}

fn with_alpha(color: Color, alpha: f32) -> Color {
    Color {
        a: alpha.clamp(0.0, 1.0),
        ..color
    }
}
