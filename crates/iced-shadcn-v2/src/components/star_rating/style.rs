//! Style resolution for the star-rating component.

use super::StarRating;
use super::types::{StarRatingStatus, StarRatingStyle};
use crate::theme::Theme;

pub(super) fn resolve_style<Message>(
    rating: &StarRating<'_, Message>,
    status: StarRatingStatus,
) -> StarRatingStyle {
    let recipe = rating.theme.style.star_rating();
    let palette = &rating.theme.palette;

    let mut style = StarRatingStyle {
        foreground: rating.color.unwrap_or(palette.primary),
        ring: palette.ring,
        opacity: if status.disabled {
            recipe.disabled_opacity
        } else {
            1.0
        },
    };

    if let Some(override_fn) = rating.style_override.as_ref() {
        style = override_fn(style, status);
    }

    style
}

#[allow(dead_code)]
pub(super) fn theme_primary(theme: &Theme) -> crate::iced_compat::Color {
    theme.palette.primary
}
