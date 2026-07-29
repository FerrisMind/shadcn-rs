//! Canvas rendering and animation for [`super::Progress`].

use std::f32::consts::TAU;
use std::time::Duration;

use iced::border::Radius;
use iced::widget::canvas;
use iced::widget::canvas::Path;
use iced::window;
use iced::{Point, Rectangle, Renderer, Size};

use super::geometry::{display_ratio, normalized_ratio};
use super::style::resolve_visual;
use super::types::{Progress, ProgressOrientation, ProgressState};

const FRAME_INTERVAL: Duration = Duration::from_millis(33);

impl<Message> canvas::Program<Message> for Progress<'_> {
    type State = ProgressState;

    fn update(
        &self,
        state: &mut Self::State,
        event: &canvas::Event,
        _bounds: Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Option<canvas::Action<Message>> {
        let canvas::Event::Window(window::Event::RedrawRequested(now)) = event else {
            return None;
        };

        let determinate = self.value.is_some();
        let target_ratio = normalized_ratio(self.value, self.max);

        if !state.initialized {
            state.initialized = true;
            state.determinate = determinate;
            state.target_ratio = target_ratio;
            state.displayed_ratio = target_ratio;
            state.transition_from = target_ratio;
            state.transition_to = target_ratio;
            state.transition_start = None;
            state.start_time = if !determinate && self.animated {
                Some(*now)
            } else {
                None
            };
        } else if state.determinate != determinate
            || (determinate && (state.target_ratio - target_ratio).abs() > f32::EPSILON)
        {
            let was_determinate = state.determinate;
            let previous_ratio = state.displayed_ratio;

            state.determinate = determinate;
            state.target_ratio = target_ratio;
            state.start_time = if !determinate && self.animated {
                Some(*now)
            } else {
                None
            };

            if determinate {
                state.transition_from = if was_determinate { previous_ratio } else { 0.0 };
                state.transition_to = target_ratio;
                state.transition_start = if self.animated { Some(*now) } else { None };
                if !self.animated {
                    state.displayed_ratio = target_ratio;
                }
            } else {
                state.displayed_ratio = 0.0;
                state.transition_start = None;
            }
        }

        if !self.animated {
            state.transition_start = None;
            if determinate {
                state.displayed_ratio = target_ratio;
            }
            return None;
        }

        if determinate {
            // Advance an in-flight value transition, if one is running.
            if let Some(start) = state.transition_start {
                let elapsed = now.saturating_duration_since(start);
                let progress = (elapsed.as_secs_f32() / self.transition_duration.as_secs_f32())
                    .clamp(0.0, 1.0);
                let eased = progress * progress * (3.0 - 2.0 * progress);
                state.displayed_ratio =
                    state.transition_from + (state.transition_to - state.transition_from) * eased;

                if progress >= 1.0 {
                    state.displayed_ratio = target_ratio;
                    state.transition_start = None;
                }
            }
        } else {
            let start = state.start_time.get_or_insert(*now);
            let elapsed = now.saturating_duration_since(*start);
            state.phase =
                (elapsed.as_secs_f32() / self.indeterminate_duration.as_secs_f32()).rem_euclid(1.0);
        }

        // While animated, keep the redraw loop alive so a value that changes
        // while the bar is otherwise idle is picked up on the next frame. A
        // canvas program only observes `value` changes from inside `update`,
        // which runs on `RedrawRequested`; without this self-driven loop a
        // determinate bar stops requesting frames once settled and never sees
        // the new value (this mirrors the spinner's animation loop). Non-
        // animated bars returned early above and stay purely event-driven.
        Some(canvas::Action::request_redraw_at(*now + FRAME_INTERVAL))
    }

    fn draw(
        &self,
        state: &Self::State,
        renderer: &Renderer,
        _theme: &iced::Theme,
        bounds: Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let size = bounds.size();
        if !size.width.is_finite()
            || !size.height.is_finite()
            || size.width <= 0.0
            || size.height <= 0.0
        {
            return Vec::new();
        }

        let mut frame = canvas::Frame::new(renderer, size);
        let radius = super::geometry::radius_px(self.theme, self, size.width, size.height);
        let visual = resolve_visual(self, self.theme);

        let track = Path::rounded_rectangle(Point::ORIGIN, size, radius.into());
        frame.fill(&track, visual.track);

        let (offset, length, rounded_ends, full) = if self.value.is_some() {
            // Read the live `value`/`max` at rest and only fall back to the
            // animated `displayed_ratio` while a transition is actually
            // running. A determinate bar stops requesting redraws once it
            // settles, so `update` is no longer pumped when the value later
            // changes while idle; without this, `draw` would keep painting the
            // stale `displayed_ratio` and the bar would appear frozen.
            let ratio = display_ratio(state, self.value, self.max);

            match self.orientation {
                ProgressOrientation::Horizontal => {
                    (0.0, size.width * ratio, false, ratio >= 1.0 - f32::EPSILON)
                }
                ProgressOrientation::Vertical => {
                    let length = size.height * ratio;
                    (
                        size.height - length,
                        length,
                        false,
                        ratio >= 1.0 - f32::EPSILON,
                    )
                }
            }
        } else {
            let phase = if state.initialized { state.phase } else { 0.0 };
            let position = phase.mul_add(TAU, -std::f32::consts::FRAC_PI_2).sin() * 0.5 + 0.5;

            match self.orientation {
                ProgressOrientation::Horizontal => {
                    let length = (size.width * 0.35).clamp(1.0, size.width);
                    ((size.width - length) * position, length, true, false)
                }
                ProgressOrientation::Vertical => {
                    let length = (size.height * 0.35).clamp(1.0, size.height);
                    (
                        (size.height - length) * (1.0 - position),
                        length,
                        true,
                        false,
                    )
                }
            }
        };

        if length > 0.0 {
            let (top_left, indicator_size) = match self.orientation {
                ProgressOrientation::Horizontal => {
                    (Point::new(offset, 0.0), Size::new(length, size.height))
                }
                ProgressOrientation::Vertical => {
                    (Point::new(0.0, offset), Size::new(size.width, length))
                }
            };

            let indicator_radius = if rounded_ends || full {
                radius.into()
            } else {
                match self.orientation {
                    ProgressOrientation::Horizontal => Radius::default().left(radius),
                    ProgressOrientation::Vertical => Radius::default().bottom(radius),
                }
            };
            let indicator = Path::rounded_rectangle(top_left, indicator_size, indicator_radius);

            // Fill the indicator directly, exactly like the track above. An
            // earlier `frame.with_clip(..)` wrapper produced no visible fill in
            // the wgpu/tiny-skia backends (the clipped sub-frame was not
            // composited), so every bar showed only its track. The indicator
            // path already stays within `bounds`, so no clip is needed.
            frame.fill(&indicator, visual.indicator);
        }

        vec![frame.into_geometry()]
    }
}
