//! Canvas rendering and pointer handling for [`super::StarRating`].

use std::f32::consts::PI;

use crate::iced_compat::widget::canvas;
use crate::iced_compat::widget::canvas::{Fill, LineCap, LineJoin, Path, Stroke};
use crate::iced_compat::{Color, Point, Rectangle, Renderer, Size, mouse};

use shadcn_common::{
    Direction, STAR_STROKE_VIEWBOX, STAR_VIEWBOX, StarRatingItemState, apply_click, display_value,
    hover_preview_value, item_state, rating_from_pointer,
};

use super::StarRating;
use super::geometry::{self, Metrics};
use super::style::resolve_style;
use super::types::{StarRatingState, StarRatingStatus};
use crate::recipes::component_radius_px;

impl<Message> canvas::Program<Message> for StarRating<'_, Message> {
    type State = StarRatingState;

    fn update(
        &self,
        state: &mut Self::State,
        event: &canvas::Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Option<canvas::Action<Message>> {
        let metrics = geometry::resolve_metrics(self);

        match event {
            canvas::Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                if !self.is_interactive() {
                    return None;
                }
                let position = local_cursor(cursor, bounds)?;
                let index = geometry::hit_star(metrics, self.orientation, position)?;
                let rect = geometry::star_rect(metrics, self.orientation, index);
                let fraction = geometry::fraction_in_star(rect, self.orientation, position);
                let pointer = rating_from_pointer(index, fraction, self.config());
                let next = apply_click(index, self.value, pointer, self.config());
                // Keep the committed value as the hover preview so the next
                // frame does not flash empty stars before the app re-renders.
                state.hover_value = Some(next);
                state.hovered_index = Some(index);
                Some(
                    self.emit_change(next)
                        .map(canvas::Action::publish)?
                        .and_capture(),
                )
            }
            canvas::Event::Mouse(mouse::Event::CursorMoved { .. })
            | canvas::Event::Mouse(mouse::Event::CursorEntered) => {
                // Outside the widget → clear. Inside a gap between stars → keep
                // the previous preview (hit targets absorb gaps; this is a
                // safety net for ring padding / rounding).
                let Some(position) = local_cursor(cursor, bounds) else {
                    return clear_hover(state);
                };
                let index = geometry::hit_star(metrics, self.orientation, position)?;
                let rect = geometry::star_rect(metrics, self.orientation, index);
                let fraction = geometry::fraction_in_star(rect, self.orientation, position);
                let proposed = rating_from_pointer(index, fraction, self.config());
                let hover = hover_preview_value(
                    Some(proposed),
                    self.config(),
                    self.readonly,
                    self.disabled,
                    self.hover_preview && self.is_interactive(),
                );

                if state.hover_value == hover && state.hovered_index == Some(index) {
                    return None;
                }
                state.hover_value = hover;
                state.hovered_index = Some(index);
                Some(canvas::Action::request_redraw())
            }
            canvas::Event::Mouse(mouse::Event::CursorLeft) => clear_hover(state),
            _ => None,
        }
    }

    fn draw(
        &self,
        state: &Self::State,
        renderer: &Renderer,
        _theme: &crate::iced_compat::Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let size = bounds.size();
        if !size.width.is_finite()
            || !size.height.is_finite()
            || size.width <= 0.0
            || size.height <= 0.0
        {
            return Vec::new();
        }

        let metrics = geometry::resolve_metrics(self);
        let status = StarRatingStatus {
            hovered: state.hovered_index.is_some(),
            disabled: self.disabled,
            readonly: self.readonly,
            focused: self.focused,
        };
        let style = resolve_style(self, status);
        let display = display_value(self.value, state.hover_value, self.config());
        let radius = component_radius_px(self.theme, self.theme.style.star_rating().item_radius);

        let mut frame = canvas::Frame::new(renderer, size);
        let paint = apply_opacity(style.foreground, style.opacity);
        let ring = apply_opacity(style.ring, style.opacity);

        for index in 0..metrics.count {
            let rect = geometry::star_rect(metrics, self.orientation, index);
            let item = item_state(index, display, self.allow_half);
            draw_star(&mut frame, rect, item, paint, self.direction);

            if self.focused {
                draw_focus_ring(&mut frame, rect, metrics, radius, ring);
            }
        }

        vec![frame.into_geometry()]
    }

    fn mouse_interaction(
        &self,
        _state: &Self::State,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> mouse::Interaction {
        if self.is_interactive() && cursor.is_over(bounds) {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::None
        }
    }
}

fn clear_hover<Message>(state: &mut StarRatingState) -> Option<canvas::Action<Message>> {
    if state.hover_value.is_none() && state.hovered_index.is_none() {
        return None;
    }
    state.hover_value = None;
    state.hovered_index = None;
    Some(canvas::Action::request_redraw())
}

fn local_cursor(cursor: mouse::Cursor, bounds: Rectangle) -> Option<Point> {
    cursor.position_in(bounds)
}

fn draw_focus_ring(
    frame: &mut canvas::Frame<Renderer>,
    star: Rectangle,
    metrics: Metrics,
    radius: f32,
    color: Color,
) {
    let inset = metrics.ring_offset;
    let ring = Rectangle {
        x: star.x - inset,
        y: star.y - inset,
        width: star.width + inset * 2.0,
        height: star.height + inset * 2.0,
    };
    let path = Path::rounded_rectangle(
        Point::new(ring.x, ring.y),
        Size::new(ring.width, ring.height),
        radius.max(0.0).into(),
    );
    frame.stroke(
        &path,
        Stroke::default()
            .with_width(metrics.ring_width)
            .with_color(color),
    );
}

fn draw_star(
    frame: &mut canvas::Frame<Renderer>,
    rect: Rectangle,
    state: StarRatingItemState,
    color: Color,
    direction: Direction,
) {
    let scale = (rect.width.min(rect.height) / STAR_VIEWBOX).max(0.001);
    let stroke_width = (STAR_STROKE_VIEWBOX * scale).max(1.0);
    let stroke = Stroke::default()
        .with_width(stroke_width)
        .with_line_cap(LineCap::Round)
        .with_line_join(LineJoin::Round)
        .with_color(color);

    // Build paths in absolute canvas coordinates. Avoid `with_clip` after
    // translate/scale — iced pastes the clipped sub-frame without the parent
    // transform, which ghosted the half-fill onto the first star.
    let vertices = lucide_star_vertices(rect);
    let full = path_from_vertices(&vertices);

    match state {
        StarRatingItemState::Active => {
            frame.fill(&full, Fill::from(color));
            frame.stroke(&full, stroke);
        }
        StarRatingItemState::Partial => {
            let mid_x = rect.x + rect.width * 0.5;
            // LTR fills the leading (left) half; RTL fills the trailing (right) half.
            let half_vertices = match direction {
                Direction::Rtl => clip_polygon_x(&vertices, mid_x, false),
                Direction::Ltr | _ => clip_polygon_x(&vertices, mid_x, true),
            };
            if half_vertices.len() >= 3 {
                frame.fill(&path_from_vertices(&half_vertices), Fill::from(color));
            }
            frame.stroke(&full, stroke);
        }
        StarRatingItemState::Inactive | _ => {
            frame.stroke(&full, stroke);
        }
    }
}

/// Lucide-proportioned star vertices mapped into `rect`.
fn lucide_star_vertices(rect: Rectangle) -> Vec<Point> {
    const CX: f32 = 12.0;
    const CY: f32 = 12.0;
    const OUTER: f32 = 10.05;
    const INNER: f32 = 3.85;

    let scale = (rect.width.min(rect.height) / STAR_VIEWBOX).max(0.001);
    let origin_x = rect.x + (rect.width - STAR_VIEWBOX * scale) * 0.5;
    let origin_y = rect.y + (rect.height - STAR_VIEWBOX * scale) * 0.5;

    (0..10)
        .map(|i| {
            let angle = -PI / 2.0 + i as f32 * PI / 5.0;
            let radius = if i % 2 == 0 { OUTER } else { INNER };
            Point::new(
                origin_x + (CX + radius * angle.cos()) * scale,
                origin_y + (CY + radius * angle.sin()) * scale,
            )
        })
        .collect()
}

fn path_from_vertices(vertices: &[Point]) -> Path {
    Path::new(|builder| {
        if let Some(first) = vertices.first() {
            builder.move_to(*first);
            for point in vertices.iter().skip(1) {
                builder.line_to(*point);
            }
            builder.close();
        }
    })
}

/// Sutherland–Hodgman clip of a polygon against `x = edge` (`keep_left` → `x <= edge`).
fn clip_polygon_x(vertices: &[Point], edge: f32, keep_left: bool) -> Vec<Point> {
    if vertices.is_empty() {
        return Vec::new();
    }

    let inside = |point: Point| {
        if keep_left {
            point.x <= edge + f32::EPSILON
        } else {
            point.x >= edge - f32::EPSILON
        }
    };
    let intersect = |a: Point, b: Point| {
        let dx = b.x - a.x;
        if dx.abs() <= f32::EPSILON {
            return Point::new(edge, a.y);
        }
        let t = (edge - a.x) / dx;
        Point::new(edge, a.y + (b.y - a.y) * t)
    };

    let mut output = vertices.to_vec();
    let mut input = Vec::with_capacity(vertices.len() + 2);
    std::mem::swap(&mut input, &mut output);

    let mut previous = *input.last().expect("non-empty");
    for &current in &input {
        let prev_in = inside(previous);
        let curr_in = inside(current);
        match (prev_in, curr_in) {
            (true, true) => output.push(current),
            (true, false) => output.push(intersect(previous, current)),
            (false, true) => {
                output.push(intersect(previous, current));
                output.push(current);
            }
            (false, false) => {}
        }
        previous = current;
    }

    output
}

fn apply_opacity(color: Color, opacity: f32) -> Color {
    Color {
        a: color.a * opacity.clamp(0.0, 1.0),
        ..color
    }
}

#[cfg(test)]
mod half_star_tests {
    use super::*;

    #[test]
    fn clip_polygon_keeps_only_the_left_half() {
        let square = vec![
            Point::new(0.0, 0.0),
            Point::new(10.0, 0.0),
            Point::new(10.0, 10.0),
            Point::new(0.0, 10.0),
        ];
        let clipped = clip_polygon_x(&square, 5.0, true);
        assert!(clipped.len() >= 3);
        assert!(clipped.iter().all(|point| point.x <= 5.0 + f32::EPSILON));
    }

    #[test]
    fn star_vertices_sit_inside_their_rect() {
        let rect = Rectangle {
            x: 10.0,
            y: 20.0,
            width: 20.0,
            height: 20.0,
        };
        let vertices = lucide_star_vertices(rect);
        assert_eq!(vertices.len(), 10);
        for point in vertices {
            assert!(point.x >= rect.x - 0.5 && point.x <= rect.x + rect.width + 0.5);
            assert!(point.y >= rect.y - 0.5 && point.y <= rect.y + rect.height + 0.5);
        }
    }
}
