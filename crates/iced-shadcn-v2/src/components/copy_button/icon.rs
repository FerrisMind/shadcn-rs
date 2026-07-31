//! Lucide-shaped icons and their short entrance animation.

use std::time::Duration;

use crate::iced_compat::widget::canvas;
use crate::iced_compat::widget::canvas::{LineCap, LineJoin, Path, Stroke};
use crate::iced_compat::window;
use crate::iced_compat::{Color, Element, Point, Rectangle, Renderer, Size, Vector};

use super::CopyButtonStatus;

const VIEWBOX: f32 = 24.0;
const FRAME_INTERVAL: Duration = Duration::from_millis(16);
const INITIAL_SCALE: f32 = 0.85;

/// Built-in icon program used when a caller does not provide a custom idle
/// icon. The program is deliberately independent of the application message
/// type so it can be embedded in any `Element`.
///
/// `pub(crate)` so the snippet component can draw the same Copy/Check/X set
/// with its own hover colors (the snippet reference keeps the ghost button
/// transparent on hover and fades the icon instead).
#[derive(Debug, Clone, Copy)]
pub(crate) struct CopyButtonIcon {
    pub(crate) status: CopyButtonStatus,
    pub(crate) color: Color,
    pub(crate) hover_color: Color,
    pub(crate) size: f32,
    pub(crate) animation_duration: Duration,
}

impl CopyButtonIcon {
    pub(crate) fn element<'a, Message: 'a>(self) -> Element<'a, Message> {
        canvas::Canvas::<Self, Message>::new(self)
            .width(crate::iced_compat::Length::Fixed(self.size))
            .height(crate::iced_compat::Length::Fixed(self.size))
            .into()
    }
}

/// Runtime-owned state for one icon canvas.
#[derive(Debug, Default)]
pub(crate) struct CopyButtonIconState {
    status: Option<CopyButtonStatus>,
    started_at: Option<crate::iced_compat::time::Instant>,
    progress: f32,
}

impl<Message> canvas::Program<Message> for CopyButtonIcon {
    type State = CopyButtonIconState;

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

        if state.status != Some(self.status) {
            state.status = Some(self.status);
            state.started_at = Some(*now);
            state.progress = 0.0;
        }

        if self.animation_duration.is_zero() {
            state.started_at = None;
            state.progress = 1.0;
            return None;
        }

        let started_at = state.started_at?;

        state.progress = (now.saturating_duration_since(started_at).as_secs_f32()
            / self.animation_duration.as_secs_f32())
        .clamp(0.0, 1.0);

        if state.progress >= 1.0 {
            state.started_at = None;
            None
        } else {
            Some(canvas::Action::request_redraw_at(*now + FRAME_INTERVAL))
        }
    }

    fn draw(
        &self,
        state: &Self::State,
        renderer: &Renderer,
        _theme: &crate::iced_compat::Theme,
        bounds: Rectangle,
        cursor: crate::iced_compat::mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        if bounds.width <= 0.0
            || bounds.height <= 0.0
            || !bounds.width.is_finite()
            || !bounds.height.is_finite()
        {
            return Vec::new();
        }

        let mut frame = canvas::Frame::new(renderer, bounds.size());
        let center = frame.center();
        let progress = state.progress.clamp(0.0, 1.0);
        let scale = INITIAL_SCALE + (1.0 - INITIAL_SCALE) * ease_out(progress);
        let viewbox_scale = (self.size / VIEWBOX).max(0.001) * scale;

        frame.with_save(|frame| {
            frame.translate(Vector::new(center.x, center.y));
            frame.scale(viewbox_scale);
            frame.translate(Vector::new(-VIEWBOX / 2.0, -VIEWBOX / 2.0));

            let color = if cursor.is_over(bounds) {
                self.hover_color
            } else {
                self.color
            };
            let stroke = Stroke::default()
                .with_width(2.0)
                .with_line_cap(LineCap::Round)
                .with_line_join(LineJoin::Round)
                .with_color(color);

            match self.status {
                CopyButtonStatus::Idle => draw_copy(frame, &stroke),
                CopyButtonStatus::Success => draw_check(frame, &stroke),
                CopyButtonStatus::Failure => draw_failure(frame, &stroke),
            }
        });

        vec![frame.into_geometry()]
    }
}

fn ease_out(progress: f32) -> f32 {
    1.0 - (1.0 - progress).powi(2)
}

fn draw_copy(frame: &mut canvas::Frame<Renderer>, stroke: &Stroke) {
    let front = Path::rounded_rectangle(Point::new(8.0, 8.0), Size::new(14.0, 14.0), 2.0.into());
    frame.stroke(&front, *stroke);

    let back = Path::new(|builder| {
        builder.move_to(Point::new(16.0, 2.0));
        builder.line_to(Point::new(4.0, 2.0));
        builder.arc_to(Point::new(2.0, 2.0), Point::new(2.0, 4.0), 2.0);
        builder.line_to(Point::new(2.0, 14.0));
        builder.arc_to(Point::new(2.0, 16.0), Point::new(4.0, 16.0), 2.0);
    });
    frame.stroke(&back, *stroke);
}

fn draw_check(frame: &mut canvas::Frame<Renderer>, stroke: &Stroke) {
    let path = Path::new(|builder| {
        builder.move_to(Point::new(20.0, 6.0));
        builder.line_to(Point::new(9.0, 17.0));
        builder.line_to(Point::new(4.0, 12.0));
    });
    frame.stroke(&path, *stroke);
}

fn draw_failure(frame: &mut canvas::Frame<Renderer>, stroke: &Stroke) {
    let first = Path::line(Point::new(6.0, 6.0), Point::new(18.0, 18.0));
    let second = Path::line(Point::new(18.0, 6.0), Point::new(6.0, 18.0));
    frame.stroke(&first, *stroke);
    frame.stroke(&second, *stroke);
}
