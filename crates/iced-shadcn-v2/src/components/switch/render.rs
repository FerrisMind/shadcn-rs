//! Canvas rendering, animation, and pointer handling for [`super::Switch`].

use std::time::Duration;

use crate::iced_compat::widget::canvas;
use crate::iced_compat::widget::canvas::{Path, Stroke};
use crate::iced_compat::{Point, Rectangle, Renderer, Size, mouse, touch, window};
use shadcn_common::Easing;

use super::Switch;
use super::geometry;
use super::style::resolve_style;
use super::types::{SwitchState, SwitchStatus};

/// Frame pacing while the thumb slides, matching the other animated components.
const FRAME_INTERVAL: Duration = Duration::from_millis(16);

impl<Message> canvas::Program<Message> for Switch<'_, Message> {
    type State = SwitchState;

    fn update(
        &self,
        state: &mut Self::State,
        event: &canvas::Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Option<canvas::Action<Message>> {
        match event {
            canvas::Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                let on_toggle = self.on_toggle.as_ref()?;

                if self.disabled || !cursor.is_over(bounds) {
                    return None;
                }

                Some(canvas::Action::publish(on_toggle(!self.checked)).and_capture())
            }
            canvas::Event::Touch(touch::Event::FingerPressed { position, .. }) => {
                let on_toggle = self.on_toggle.as_ref()?;

                if self.disabled || !bounds.contains(*position) {
                    return None;
                }

                Some(canvas::Action::publish(on_toggle(!self.checked)).and_capture())
            }
            canvas::Event::Window(window::Event::RedrawRequested(now)) => {
                self.advance(state, *now);

                state
                    .transition
                    .is_running()
                    .then(|| canvas::Action::request_redraw_at(*now + FRAME_INTERVAL))
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

        let metrics = geometry::resolve_metrics(self.theme, self.size);
        let track_origin = centered_origin(size, metrics.track);
        let thumb_size = metrics.thumb;
        let track_radius = geometry::radius_px(self, metrics.track);
        let thumb_radius = geometry::radius_px(self, thumb_size);
        let status = SwitchStatus {
            checked: self.checked,
            hovered: cursor.is_over(bounds),
            disabled: self.disabled,
            focused: self.focused,
            invalid: self.invalid,
        };

        let mut style = resolve_style(self, metrics, status, track_radius, thumb_radius);
        if let Some(style_override) = self.style_override.as_ref() {
            style = style_override(style, status);
        }

        let mut frame = canvas::Frame::new(renderer, size);

        if let Some(ring) = style.ring.filter(|ring| ring.a > f32::EPSILON)
            && style.ring_width > 0.0
        {
            // A CSS ring sits outside the border box, so the stroke is centred
            // on a rectangle grown by half the ring width.
            let inset = -style.ring_width / 2.0;
            frame.stroke(
                &rounded_rect(
                    track_origin,
                    metrics.track,
                    style.track_radius - inset,
                    inset,
                ),
                Stroke::default()
                    .with_width(style.ring_width)
                    .with_color(ring),
            );
        }

        frame.fill(
            &rounded_rect(track_origin, metrics.track, style.track_radius, 0.0),
            style.track,
        );

        if style.border_width > 0.0 {
            let inset = style.border_width / 2.0;
            frame.stroke(
                &rounded_rect(
                    track_origin,
                    metrics.track,
                    (style.track_radius - inset).max(0.0),
                    inset,
                ),
                Stroke::default()
                    .with_width(style.border_width)
                    .with_color(style.border),
            );
        }

        let thumb_origin = Point::new(
            track_origin.x + geometry::thumb_offset(metrics, self.position(state)),
            track_origin.y + (metrics.track.height - thumb_size.height) / 2.0,
        );
        frame.fill(
            &rounded_rect(thumb_origin, thumb_size, style.thumb_radius, 0.0),
            style.thumb,
        );

        vec![frame.into_geometry()]
    }

    fn mouse_interaction(
        &self,
        _state: &Self::State,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> mouse::Interaction {
        if !cursor.is_over(bounds) {
            return mouse::Interaction::default();
        }

        // `data-disabled:cursor-not-allowed` in the web component.
        if self.disabled {
            mouse::Interaction::NotAllowed
        } else if self.on_toggle.is_some() {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::default()
        }
    }
}

impl<Message> Switch<'_, Message> {
    /// Advances the thumb transition for the frame drawn at `now`.
    pub(super) fn advance(&self, state: &mut SwitchState, now: crate::iced_compat::time::Instant) {
        let target = f32::from(u8::from(self.checked));
        state
            .transition
            .advance(target, self.animated, self.duration, Easing::EaseInOut, now);
    }

    /// Thumb position the canvas paints, in `0.0..=1.0`.
    ///
    /// While a transition runs the eased value drives the animation. At rest the
    /// position is derived from `checked` directly, so a switch that changes
    /// while no frames are requested still paints the correct state.
    pub(super) fn position(&self, state: &SwitchState) -> f32 {
        state
            .transition
            .displayed(f32::from(u8::from(self.checked)))
            .clamp(0.0, 1.0)
    }
}

fn centered_origin(bounds: Size, track: Size) -> Point {
    Point::new(
        ((bounds.width - track.width) / 2.0).max(0.0),
        ((bounds.height - track.height) / 2.0).max(0.0),
    )
}

/// Rounded rectangle grown by `inset` on every side (negative grows outwards).
fn rounded_rect(origin: Point, size: Size, radius: f32, inset: f32) -> Path {
    Path::rounded_rectangle(
        Point::new(origin.x + inset, origin.y + inset),
        Size::new(
            (size.width - inset * 2.0).max(0.0),
            (size.height - inset * 2.0).max(0.0),
        ),
        radius.max(0.0).into(),
    )
}
