//! Canvas rendering and animation for [`super::Skeleton`].

use std::f32::consts::TAU;
use std::time::Duration;

use crate::iced_compat::widget::canvas;
use crate::iced_compat::widget::canvas::Path;
use crate::iced_compat::window;
use crate::iced_compat::{Color, Point, Rectangle, Renderer, Size};

use super::geometry::radius_px;
use super::types::{Skeleton, SkeletonAnimation, SkeletonFill, SkeletonState};

const FRAME_INTERVAL: Duration = Duration::from_millis(33);

impl<Message> canvas::Program<Message> for Skeleton<'_> {
    type State = SkeletonState;

    fn update(
        &self,
        state: &mut Self::State,
        event: &canvas::Event,
        _bounds: Rectangle,
        _cursor: crate::iced_compat::mouse::Cursor,
    ) -> Option<canvas::Action<Message>> {
        if matches!(self.animation, SkeletonAnimation::Static) {
            return None;
        }

        if let canvas::Event::Window(window::Event::RedrawRequested(now)) = event {
            if state.start_time.is_none() {
                state.start_time = Some(*now);
            }

            if let Some(start) = state.start_time {
                let elapsed = now.saturating_duration_since(start);
                state.phase = (elapsed.as_secs_f32() / self.duration.as_secs_f32()) % 1.0;
            }

            return Some(canvas::Action::request_redraw_at(*now + FRAME_INTERVAL));
        }

        None
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
        let mut frame = canvas::Frame::new(renderer, size);
        let radius = radius_px(self.theme, self.shape, size);
        let base = resolve_fill(self);

        match self.animation {
            SkeletonAnimation::Static => fill_rounded(&mut frame, size, radius, base),
            SkeletonAnimation::Pulse => {
                let opacity = 0.75 + (state.phase * TAU).cos() * 0.25;
                fill_rounded(&mut frame, size, radius, apply_opacity(base, opacity));
            }
        }

        vec![frame.into_geometry()]
    }
}

fn resolve_fill(skeleton: &Skeleton<'_>) -> Color {
    match skeleton.fill {
        SkeletonFill::Semantic(color) => skeleton.theme.semantic_color(color),
        SkeletonFill::Custom(color) => color,
    }
}

fn fill_rounded(frame: &mut canvas::Frame<Renderer>, size: Size, radius: f32, color: Color) {
    let path = Path::rounded_rectangle(Point::ORIGIN, size, radius.into());
    frame.fill(&path, color);
}

fn apply_opacity(color: Color, opacity: f32) -> Color {
    Color {
        a: color.a * opacity.clamp(0.0, 1.0),
        ..color
    }
}
