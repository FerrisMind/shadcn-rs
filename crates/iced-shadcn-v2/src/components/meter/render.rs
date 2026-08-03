//! Canvas rendering and animation for [`super::Meter`].

use std::time::Duration;

use crate::iced_compat::border::Radius;
use crate::iced_compat::widget::canvas;
use crate::iced_compat::widget::canvas::Path;
use crate::iced_compat::window;
use crate::iced_compat::{Point, Rectangle, Renderer, Size};

use super::geometry::{display_ratio, sync_transition};
use super::style::resolve_visual;
use super::types::{Meter, MeterOrientation, MeterState};

const FRAME_INTERVAL: Duration = Duration::from_millis(33);

impl<Message> canvas::Program<Message> for Meter<'_> {
    type State = MeterState;

    fn update(
        &self,
        state: &mut Self::State,
        event: &canvas::Event,
        _bounds: Rectangle,
        _cursor: crate::iced_compat::mouse::Cursor,
    ) -> Option<canvas::Action<Message>> {
        let canvas::Event::Window(window::Event::RedrawRequested(now)) = event else {
            return None;
        };

        sync_transition(state, self, self.animated, *now);

        state
            .transition
            .is_running()
            .then(|| canvas::Action::request_redraw_at(*now + FRAME_INTERVAL))
    }

    fn draw(
        &self,
        state: &Self::State,
        renderer: &Renderer,
        _theme: &crate::iced_compat::Theme,
        bounds: Rectangle,
        _cursor: crate::iced_compat::mouse::Cursor,
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

        let ratio = display_ratio(state, self);
        let (offset, length, full) = match self.orientation {
            MeterOrientation::Horizontal => {
                let length = size.width * ratio;
                (0.0, length, ratio >= 1.0 - f32::EPSILON)
            }
            MeterOrientation::Vertical => {
                let length = size.height * ratio;
                (size.height - length, length, ratio >= 1.0 - f32::EPSILON)
            }
        };

        if length > 0.0 {
            let (top_left, indicator_size) = match self.orientation {
                MeterOrientation::Horizontal => {
                    (Point::new(offset, 0.0), Size::new(length, size.height))
                }
                MeterOrientation::Vertical => {
                    (Point::new(0.0, offset), Size::new(size.width, length))
                }
            };

            let indicator_radius = if full {
                radius.into()
            } else {
                match self.orientation {
                    MeterOrientation::Horizontal => Radius::default().left(radius),
                    MeterOrientation::Vertical => Radius::default().bottom(radius),
                }
            };
            let indicator = Path::rounded_rectangle(top_left, indicator_size, indicator_radius);
            frame.fill(&indicator, visual.indicator);
        }

        vec![frame.into_geometry()]
    }
}
