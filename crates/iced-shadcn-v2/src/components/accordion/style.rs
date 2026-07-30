//! Semantic surface and trigger-state styling for the accordion.

use crate::iced_compat::widget::{button, container};
use crate::iced_compat::{Background, Border, Color};
use crate::theme::Theme;
use twill_core::prelude::theme::SemanticColor;

use super::types::AccordionValue;

/// Surface state shared by the accordion root, items, and content.
#[derive(Debug, Clone, Copy)]
pub(super) struct Surface {
    pub(super) background: Option<SemanticColor>,
    pub(super) bordered: bool,
    pub(super) radius: Option<f32>,
}

/// Resolves a semantic surface onto an iced container style.
pub(super) fn resolve_surface(theme: &Theme, surface: Surface) -> container::Style {
    container::Style {
        text_color: surface
            .background
            .map(|slot| theme.semantic_foreground(slot)),
        background: surface
            .background
            .map(|slot| Background::Color(theme.semantic_color(slot))),
        border: Border {
            radius: surface.radius.unwrap_or(0.0).into(),
            width: if surface.bordered { 1.0 } else { 0.0 },
            color: if surface.bordered {
                theme.semantic_color(SemanticColor::Border)
            } else {
                Color::TRANSPARENT
            },
        },
        ..container::Style::default()
    }
}

/// Resolves an item surface. The root inserts default dividers between items,
/// because iced container borders are uniform on all four sides.
pub(super) fn resolve_item_surface(
    theme: &Theme,
    background: Option<SemanticColor>,
    bordered: bool,
    radius: Option<f32>,
    open: bool,
) -> container::Style {
    let mut resolved = resolve_surface(
        theme,
        Surface {
            background,
            bordered,
            radius,
        },
    );

    if background.is_none() && open && super::geometry::default_open_item_background(theme) {
        let muted = with_alpha(theme.semantic_color(SemanticColor::Muted), 0.5);
        resolved.background = Some(Background::Color(muted));
    }

    resolved
}

/// Removes the ghost button's filled hover surface so the trigger matches the
/// source component's `hover:underline` treatment.
pub(super) fn normalize_ghost_trigger_style(
    theme: &Theme,
    style: &mut button::Style,
    status: button::Status,
    disabled: bool,
) {
    let foreground = theme.semantic_color(SemanticColor::Foreground);

    match status {
        button::Status::Hovered | button::Status::Pressed => {
            style.background = None;
            style.border.width = 0.0;
            style.text_color = foreground;
        }
        button::Status::Disabled if disabled => {
            style.background = None;
            style.border.width = 0.0;
            style.text_color = with_alpha(foreground, 0.5);
        }
        button::Status::Disabled => {
            // A missing callback creates a read-only preview rather than a
            // semantically disabled item.
            style.background = None;
            style.border.width = 0.0;
            style.text_color = foreground;
        }
        button::Status::Active => {}
    }
}

/// Resolves the indicator color used by the down/up chevron.
pub(super) fn indicator_color(theme: &Theme, disabled: bool) -> Color {
    let color = theme.semantic_color(SemanticColor::MutedForeground);
    if disabled {
        with_alpha(color, 0.5)
    } else {
        color
    }
}

/// Computes the next value when one item is toggled.
pub(super) fn next_value(
    value: &AccordionValue,
    accordion_type: super::types::AccordionType,
    item: &str,
) -> AccordionValue {
    value.toggled(accordion_type, item)
}

fn with_alpha(color: Color, alpha: f32) -> Color {
    Color {
        a: color.a * alpha,
        ..color
    }
}
