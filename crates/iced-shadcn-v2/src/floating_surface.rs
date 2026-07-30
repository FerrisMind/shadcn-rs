//! Floating surface paint helpers matching shadcn-svelte CSS rings.
//!
//! Tailwind `ring-1 ring-foreground/N` is a **box-shadow**
//! (`0 0 0 1px color`), drawn **outside** the surface. Painting the same
//! token as an iced inset [`Border`] makes soft packs (Maia / Luma / Rhea)
//! look outlined when the reference site barely shows a hairline.

use crate::iced_compat::{
    Background, Border, Color, Rectangle, Renderer, Shadow, advanced::renderer,
};

/// Paints a popover-like surface: drop shadow + fill, then an optional CSS
/// `ring-1` hairline outside the bounds.
pub fn fill_floating_surface(
    renderer: &mut Renderer,
    bounds: Rectangle,
    background: Color,
    ring_color: Color,
    ring_width: f32,
    radius: f32,
    shadow: Shadow,
) {
    use renderer::Renderer as _;

    renderer.fill_quad(
        renderer::Quad {
            bounds,
            border: Border {
                radius: radius.into(),
                width: 0.0,
                color: Color::TRANSPARENT,
            },
            shadow,
            ..renderer::Quad::default()
        },
        Background::Color(background),
    );

    paint_outside_ring(renderer, bounds, ring_color, ring_width, radius);
}

/// CSS `ring-1` as an outside stroke (transparent fill + border on expanded
/// bounds). No-op when `ring_width` or alpha is zero.
pub fn paint_outside_ring(
    renderer: &mut Renderer,
    bounds: Rectangle,
    ring_color: Color,
    ring_width: f32,
    radius: f32,
) {
    use renderer::Renderer as _;

    if ring_width <= f32::EPSILON || ring_color.a <= f32::EPSILON {
        return;
    }

    let outer = bounds.expand(ring_width);
    renderer.fill_quad(
        renderer::Quad {
            bounds: outer,
            border: Border {
                radius: (radius + ring_width).into(),
                width: ring_width,
                color: ring_color,
            },
            shadow: Shadow::default(),
            ..renderer::Quad::default()
        },
        Background::Color(Color::TRANSPARENT),
    );
}
