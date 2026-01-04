use std::f32::consts::PI;

use iced::widget::canvas;
use iced::widget::canvas::{Path, Stroke, stroke};
use iced::{Color, Length, Point, Rectangle, Renderer};

use crate::theme::Theme;

#[derive(Clone, Copy, Debug)]
pub struct Spinner {
    progress: f32,
    color: Color,
    stroke_width: f32,
    size: f32,
}

impl Spinner {
    pub fn new(theme: &Theme) -> Self {
        Self {
            progress: 0.0,
            color: theme.palette.primary,
            stroke_width: 2.0,
            size: 24.0,
        }
    }

    pub fn progress(mut self, progress: f32) -> Self {
        self.progress = progress;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn stroke_width(mut self, width: f32) -> Self {
        self.stroke_width = width;
        self
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }
}

pub fn spinner<Message>(spinner: Spinner) -> canvas::Canvas<Spinner, Message> {
    let size = spinner.size;
    canvas::Canvas::new(spinner)
        .width(Length::Fixed(size))
        .height(Length::Fixed(size))
}

impl<Message> canvas::Program<Message> for Spinner {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &iced::Theme,
        bounds: Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());
        let center = frame.center();
        let radius = (bounds.width.min(bounds.height) - self.stroke_width) * 0.5;

        let start_angle = self.progress.rem_euclid(1.0) * 2.0 * PI;
        let sweep = PI * 1.5;
        let segments = 32;

        let path = Path::new(|builder| {
            for i in 0..=segments {
                let t = start_angle + sweep * (i as f32 / segments as f32);
                let point = Point::new(center.x + radius * t.cos(), center.y + radius * t.sin());
                if i == 0 {
                    builder.move_to(point);
                } else {
                    builder.line_to(point);
                }
            }
        });

        frame.stroke(
            &path,
            Stroke {
                style: stroke::Style::Solid(self.color),
                width: self.stroke_width,
                line_cap: canvas::LineCap::Round,
                ..Stroke::default()
            },
        );

        vec![frame.into_geometry()]
    }
}
