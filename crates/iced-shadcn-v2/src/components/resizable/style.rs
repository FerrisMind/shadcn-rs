//! Handle and frame styling for the resizable component.

use crate::iced_compat::advanced::renderer::{Quad, Renderer as _};
use crate::iced_compat::widget::container;
use crate::iced_compat::{Background, Border, Color};

use twill_core::prelude::theme::SemanticColor;

use super::geometry;
use super::types::ResizableRadius;
use crate::theme::Theme;

/// Border width of a bordered frame (`border` → 1 px in the web packs).
const BORDER_WIDTH: f32 = 1.0;

/// Resolved colors and radii for one handle.
#[derive(Debug, Clone, Copy)]
pub(super) struct HandleStyle {
    pub(super) line: Color,
    pub(super) grip: Color,
    pub(super) grip_radius: f32,
    pub(super) focus_ring: Color,
}

/// Resolves the divider and optional grip colors from the theme.
pub(super) fn resolve_handle_style(theme: &Theme) -> HandleStyle {
    HandleStyle {
        line: theme.semantic_color(SemanticColor::Border),
        grip: theme.semantic_color(SemanticColor::Border),
        grip_radius: geometry::grip_radius_px(theme),
        focus_ring: theme.semantic_color(SemanticColor::Ring),
    }
}

/// Resolves the outer frame container style.
pub(super) fn resolve_frame_style(
    theme: &Theme,
    bordered: bool,
    radius: ResizableRadius,
) -> container::Style {
    container::Style {
        background: None,
        border: Border {
            radius: geometry::frame_radius_px(theme, radius).into(),
            width: if bordered { BORDER_WIDTH } else { 0.0 },
            color: if bordered {
                theme.semantic_color(SemanticColor::Border)
            } else {
                Color::TRANSPARENT
            },
        },
        ..container::Style::default()
    }
}

/// Paints a rounded rectangle with a solid fill.
pub(super) fn fill_rounded_rect(
    renderer: &mut crate::iced_compat::Renderer,
    bounds: crate::iced_compat::Rectangle,
    radius: f32,
    color: Color,
) {
    if bounds.width <= 0.0 || bounds.height <= 0.0 || color.a <= f32::EPSILON {
        return;
    }

    renderer.fill_quad(
        Quad {
            bounds,
            border: Border {
                radius: radius.into(),
                width: 0.0,
                color: Color::TRANSPARENT,
            },
            ..Quad::default()
        },
        Background::Color(color),
    );
}

/// Paints the optional grip icon centered in `bounds`.
pub(super) fn draw_grip(
    renderer: &mut crate::iced_compat::Renderer,
    bounds: crate::iced_compat::Rectangle,
    direction: super::types::ResizableDirection,
    style: &HandleStyle,
) {
    use super::types::ResizableDirection;
    use crate::iced_compat::Rectangle;

    let (width, height) = match direction {
        ResizableDirection::Horizontal => (geometry::GRIP_CROSS_PX, geometry::GRIP_MAIN_PX),
        ResizableDirection::Vertical => (geometry::GRIP_MAIN_PX, geometry::GRIP_CROSS_PX),
    };

    let grip = Rectangle {
        x: bounds.x + (bounds.width - width) / 2.0,
        y: bounds.y + (bounds.height - height) / 2.0,
        width,
        height,
    };

    fill_rounded_rect(renderer, grip, style.grip_radius, style.grip);
}

/// Paints a one-pixel divider line through the handle slot.
pub(super) fn draw_divider(
    renderer: &mut crate::iced_compat::Renderer,
    bounds: crate::iced_compat::Rectangle,
    style: &HandleStyle,
) {
    fill_rounded_rect(renderer, bounds, 0.0, style.line);
}

/// Paints a focus ring around the expanded hit target.
pub(super) fn draw_focus_ring(
    renderer: &mut crate::iced_compat::Renderer,
    bounds: crate::iced_compat::Rectangle,
    style: &HandleStyle,
) {
    if style.focus_ring.a <= f32::EPSILON {
        return;
    }

    renderer.fill_quad(
        Quad {
            bounds,
            border: Border {
                radius: 2.0.into(),
                width: 1.0,
                color: style.focus_ring,
            },
            ..Quad::default()
        },
        Background::Color(Color::TRANSPARENT),
    );
}
