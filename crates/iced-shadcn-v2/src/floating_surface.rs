//! Floating surface paint helpers matching shadcn-svelte CSS rings.
//!
//! Tailwind `ring-1 ring-foreground/N` is a **box-shadow**
//! (`0 0 0 1px color`), drawn **outside** the surface. Painting the same
//! token as an iced inset [`Border`] makes soft packs (Maia / Luma / Rhea)
//! look outlined when the reference site barely shows a hairline.
//!
//! Call order for overlays with opaque children (Command, forms, …):
//! 1. [`fill_floating_surface`] — shadow + fill
//! 2. draw children
//! 3. [`paint_outside_ring`] — hairline **after** content so edge-to-edge
//!    fills cannot cover the ring at large radii

use crate::iced_compat::{
    Background, Border, Color, Rectangle, Renderer, Shadow, advanced::renderer,
};

/// Paints a popover-like surface: drop shadow + fill only.
///
/// Does **not** paint the CSS ring — callers must invoke
/// [`paint_outside_ring`] **after** drawing surface children.
pub fn fill_floating_surface(
    renderer: &mut Renderer,
    bounds: Rectangle,
    background: Color,
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
            snap: false,
        },
        Background::Color(background),
    );
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
            snap: false,
        },
        Background::Color(Color::TRANSPARENT),
    );
}
