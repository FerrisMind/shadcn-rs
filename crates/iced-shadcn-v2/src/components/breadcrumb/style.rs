//! Semantic color resolution for the breadcrumb component.

use crate::iced_compat::border::Border;
use crate::iced_compat::widget::button as button_widget;
use crate::iced_compat::widget::container;
use crate::iced_compat::{Color, Shadow};

use crate::theme::Theme;

/// Resting color of the list, links, separators, and the ellipsis
/// (`.cn-breadcrumb-list` `text-muted-foreground`).
pub(super) fn muted_color(theme: &Theme) -> Color {
    theme.palette.muted_foreground
}

/// Color of the current page and of a hovered link (`text-foreground`).
pub(super) fn current_color(theme: &Theme) -> Color {
    theme.palette.foreground
}

/// Resolves the transparent link surface, applying the
/// `.cn-breadcrumb-link` `hover:text-foreground` transition.
///
/// A link without a press message resolves to
/// [`button_widget::Status::Disabled`] in iced; it keeps the resting color
/// instead of the muted-disabled treatment used by buttons, because the web
/// component has no disabled state.
pub(super) fn resolve_link_style(
    resting: Color,
    hovered: Color,
    status: button_widget::Status,
) -> button_widget::Style {
    let text_color = match status {
        button_widget::Status::Hovered | button_widget::Status::Pressed => hovered,
        button_widget::Status::Active | button_widget::Status::Disabled => resting,
    };

    button_widget::Style {
        background: None,
        text_color,
        border: Border::default(),
        shadow: Shadow::default(),
        snap: false,
    }
}

/// Container surface that only carries an inherited text color.
pub(super) fn text_color_surface(color: Color) -> container::Style {
    container::Style {
        text_color: Some(color),
        ..container::Style::default()
    }
}
