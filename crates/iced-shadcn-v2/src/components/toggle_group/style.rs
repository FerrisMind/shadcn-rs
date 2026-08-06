//! Root-container styling for toggle groups.

use crate::components::toggle::ToggleVariant;
use crate::components::toggle_group::geometry;
use crate::iced_compat::border::Border;
use crate::iced_compat::widget::container;
use crate::theme::Theme;
use twill_core::prelude::theme::SemanticColor;

pub(super) fn resolve_group_style(
    theme: &Theme,
    variant: ToggleVariant,
    spacing: f32,
    disabled: bool,
) -> container::Style {
    // The web component suppresses only each following item's leading border.
    // Iced's public `Border` is uniform, so `ToggleGroup` paints the matching
    // outer frame here and inserts one-pixel separators between item widgets.
    let merged = geometry::merged_borders(variant, spacing);
    let radius = if merged {
        geometry::default_radius_px(theme)
    } else {
        0.0
    };

    container::Style {
        background: None,
        text_color: None,
        border: if merged {
            Border {
                radius: radius.into(),
                width: 1.0,
                color: with_alpha(theme.semantic_color(SemanticColor::Input), disabled),
            }
        } else {
            Border::default()
        },
        shadow: Default::default(),
        snap: true,
    }
}

fn with_alpha(mut color: crate::iced_compat::Color, disabled: bool) -> crate::iced_compat::Color {
    if disabled {
        color.a *= 0.5;
    }
    color
}
