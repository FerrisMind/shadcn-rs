//! Canvas rendering and pointer handling for [`super::Slider`].

use crate::iced_compat::widget::canvas;
use crate::iced_compat::widget::canvas::{Path, Stroke};
use crate::iced_compat::{Point, Rectangle, Renderer, Size, mouse, touch};

use super::Slider;
use super::geometry::{self, Metrics};
use super::style::resolve_style;
use super::types::{SliderOrientation, SliderState, SliderStatus, SliderStyle};

impl<Message> canvas::Program<Message> for Slider<'_, Message> {
    type State = SliderState;

    fn update(
        &self,
        state: &mut Self::State,
        event: &canvas::Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Option<canvas::Action<Message>> {
        let metrics = geometry::resolve_metrics(self.theme);
        let track = geometry::track_rect(bounds.size(), metrics, self.orientation);

        match event {
            canvas::Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                if self.disabled || !self.is_interactive() {
                    return None;
                }

                let position = local_cursor(cursor, bounds)?;
                self.begin_drag(state, track, metrics, position, None)
            }
            canvas::Event::Touch(touch::Event::FingerPressed { id, position }) => {
                if self.disabled
                    || !self.is_interactive()
                    || state.dragging.is_some()
                    || !bounds.contains(*position)
                {
                    return None;
                }

                self.begin_drag(
                    state,
                    track,
                    metrics,
                    local_point(*position, bounds),
                    Some(*id),
                )
            }
            canvas::Event::Mouse(mouse::Event::CursorMoved { .. }) => {
                // A live touch drag owns the pointer; stray mouse motion
                // (e.g. a palm brushing the touchpad) must not cancel it.
                if state.active_finger.is_some() {
                    return None;
                }

                if let Some(index) = state.dragging {
                    if self.disabled {
                        state.dragging = None;
                        return None;
                    }

                    // Dragging keeps following the pointer even once it leaves
                    // the widget, so a fast gesture cannot strand a thumb.
                    let position = cursor
                        .position()
                        .map(|position| Point::new(position.x - bounds.x, position.y - bounds.y))?;
                    let value = geometry::value_at(self, track, metrics, position);

                    return Some(self.change_action(index, value).and_capture());
                }

                let hovered = local_cursor(cursor, bounds)
                    .and_then(|position| geometry::thumb_at(self, track, metrics, position));

                // Only the hovered thumb paints a ring, so a changed hover
                // target has to be repainted.
                (state.hovered != hovered).then(|| {
                    state.hovered = hovered;
                    canvas::Action::request_redraw()
                })
            }
            canvas::Event::Touch(touch::Event::FingerMoved { id, position }) => {
                if state.active_finger != Some(*id) {
                    return None;
                }

                let index = state.dragging?;
                if self.disabled {
                    state.dragging = None;
                    state.active_finger = None;
                    return None;
                }

                let value =
                    geometry::value_at(self, track, metrics, local_point(*position, bounds));
                Some(self.change_action(index, value).and_capture())
            }
            canvas::Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                if state.active_finger.is_some() {
                    return None;
                }
                self.finish_drag(state)
            }
            canvas::Event::Touch(touch::Event::FingerLifted { id, .. })
            | canvas::Event::Touch(touch::Event::FingerLost { id, .. }) => {
                if state.active_finger != Some(*id) {
                    return None;
                }
                self.finish_drag(state)
            }
            _ => None,
        }
    }

    fn draw(
        &self,
        state: &Self::State,
        renderer: &Renderer,
        _theme: &crate::iced_compat::Theme,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let size = bounds.size();
        if !size.width.is_finite()
            || !size.height.is_finite()
            || size.width <= 0.0
            || size.height <= 0.0
        {
            return Vec::new();
        }

        let metrics = geometry::resolve_metrics(self.theme);
        let track = geometry::track_rect(size, metrics, self.orientation);
        let track_radius = geometry::radius_px(
            self.theme,
            self.radius
                .unwrap_or_else(|| geometry::default_track_radius(self.theme)),
            track.size(),
        );
        let thumb_size = geometry::thumb_size(metrics, self.orientation);
        let thumb_radius = geometry::radius_px(
            self.theme,
            self.thumb_radius
                .unwrap_or_else(|| geometry::default_thumb_radius(self.theme)),
            thumb_size,
        );

        let status = SliderStatus {
            hovered: cursor.is_over(bounds),
            dragging: state.dragging.is_some(),
            disabled: self.disabled,
            focused: self.focused,
        };

        let mut style = resolve_style(self, metrics, status, track_radius, thumb_radius);
        if let Some(style_override) = self.style_override.as_ref() {
            style = style_override(style, status);
        }

        let mut frame = canvas::Frame::new(renderer, size);

        frame.fill(
            &rounded_rect(track.position(), track.size(), style.track_radius),
            style.track,
        );

        self.draw_range(&mut frame, track, metrics, &style);
        self.draw_thumbs(&mut frame, state, track, metrics, &style);

        vec![frame.into_geometry()]
    }

    fn mouse_interaction(
        &self,
        state: &Self::State,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> mouse::Interaction {
        if state.dragging.is_some() {
            return mouse::Interaction::Grabbing;
        }

        if !cursor.is_over(bounds) {
            return mouse::Interaction::default();
        }

        // `data-disabled` sliders keep the plain cursor; the web component only
        // dims them and drops pointer handling.
        if self.disabled {
            mouse::Interaction::NotAllowed
        } else if self.is_interactive() {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::default()
        }
    }
}

impl<Message> Slider<'_, Message> {
    fn begin_drag(
        &self,
        state: &mut SliderState,
        track: Rectangle,
        metrics: Metrics,
        position: Point,
        active_finger: Option<touch::Finger>,
    ) -> Option<canvas::Action<Message>> {
        if state.dragging.is_some() {
            return None;
        }

        // Pressing the track jumps the nearest thumb to the cursor and keeps
        // dragging it, exactly like the web component.
        let index = geometry::closest_thumb(self, track, metrics, position)?;
        state.dragging = Some(index);
        state.hovered = Some(index);
        state.active_finger = active_finger;

        let value = geometry::value_at(self, track, metrics, position);
        Some(self.change_action(index, value).and_capture())
    }

    fn finish_drag(&self, state: &mut SliderState) -> Option<canvas::Action<Message>> {
        let was_dragging = state.dragging.take();
        state.active_finger = None;

        was_dragging.map(|_| match self.on_release.as_ref() {
            Some(on_release) => canvas::Action::publish(on_release()),
            None => canvas::Action::request_redraw(),
        })
    }

    /// Fills the selected range: up to the thumb for a single value, between the
    /// outer thumbs when several are present.
    fn draw_range(
        &self,
        frame: &mut canvas::Frame<Renderer>,
        track: Rectangle,
        metrics: Metrics,
        style: &SliderStyle,
    ) {
        let Some((first, last)) = self.range_fractions() else {
            return;
        };

        let start = geometry::thumb_center(track, metrics, self.orientation, first);
        let end = geometry::thumb_center(track, metrics, self.orientation, last);

        let (position, size) = match self.orientation {
            SliderOrientation::Horizontal => {
                let from = if self.values.len() > 1 {
                    start.x
                } else {
                    track.x
                };
                (
                    Point::new(from, track.y),
                    Size::new((end.x - from).max(0.0), track.height),
                )
            }
            SliderOrientation::Vertical => {
                let bottom = track.y + track.height;
                let from = if self.values.len() > 1 {
                    start.y
                } else {
                    bottom
                };
                (
                    Point::new(track.x, end.y),
                    Size::new(track.width, (from - end.y).max(0.0)),
                )
            }
        };

        if size.width > 0.0 && size.height > 0.0 {
            frame.fill(
                &rounded_rect(position, size, style.track_radius),
                style.range,
            );
        }
    }

    fn draw_thumbs(
        &self,
        frame: &mut canvas::Frame<Renderer>,
        state: &SliderState,
        track: Rectangle,
        metrics: Metrics,
        style: &SliderStyle,
    ) {
        let thumb_size = geometry::thumb_size(metrics, self.orientation);

        for (index, value) in self.values.iter().copied().enumerate() {
            let center = geometry::thumb_center(
                track,
                metrics,
                self.orientation,
                geometry::snapped_fraction(value, self.min, self.max, self.step),
            );
            let position = Point::new(
                center.x - thumb_size.width / 2.0,
                center.y - thumb_size.height / 2.0,
            );

            // `hover:ring-*` / `focus-visible:ring-*`: the ring sits outside the
            // thumb box, so it is stroked on a rectangle grown by half its width.
            let active = state.dragging == Some(index)
                || (state.dragging.is_none() && state.hovered == Some(index))
                || self.focused;

            if active && style.ring_width > 0.0 && style.ring.a > f32::EPSILON {
                let inset = -style.ring_width / 2.0;
                frame.stroke(
                    &grown_rect(position, thumb_size, style.thumb_radius - inset, inset),
                    Stroke::default()
                        .with_width(style.ring_width)
                        .with_color(style.ring),
                );
            }

            frame.fill(
                &rounded_rect(position, thumb_size, style.thumb_radius),
                style.thumb,
            );

            if style.thumb_border_width > 0.0 && style.thumb_border.a > f32::EPSILON {
                let inset = style.thumb_border_width / 2.0;
                frame.stroke(
                    &grown_rect(
                        position,
                        thumb_size,
                        (style.thumb_radius - inset).max(0.0),
                        inset,
                    ),
                    Stroke::default()
                        .with_width(style.thumb_border_width)
                        .with_color(style.thumb_border),
                );
            }
        }
    }
}

fn local_cursor(cursor: mouse::Cursor, bounds: Rectangle) -> Option<Point> {
    cursor
        .position_over(bounds)
        .map(|position| Point::new(position.x - bounds.x, position.y - bounds.y))
}

fn local_point(position: Point, bounds: Rectangle) -> Point {
    Point::new(position.x - bounds.x, position.y - bounds.y)
}

fn rounded_rect(position: Point, size: Size, radius: f32) -> Path {
    Path::rounded_rectangle(position, size, radius.max(0.0).into())
}

/// Rounded rectangle grown by `inset` on every side (negative grows outwards).
fn grown_rect(position: Point, size: Size, radius: f32, inset: f32) -> Path {
    Path::rounded_rectangle(
        Point::new(position.x + inset, position.y + inset),
        Size::new(
            (size.width - inset * 2.0).max(0.0),
            (size.height - inset * 2.0).max(0.0),
        ),
        radius.max(0.0).into(),
    )
}
