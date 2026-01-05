use std::f32::consts::PI;

use iced::alignment::Vertical;
use iced::widget::canvas;
use iced::widget::canvas::Text;
use iced::{Color, Font, Length, Point, Rectangle, Renderer};
use lucide_icons::Icon;

use crate::theme::Theme;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SpinnerSize {
    One,
    Two,
    Three,
    Custom(f32),
}

impl SpinnerSize {
    fn pixels(self) -> f32 {
        match self {
            SpinnerSize::One => 12.0,
            SpinnerSize::Two => 16.0,
            SpinnerSize::Three => 20.0,
            SpinnerSize::Custom(value) => value.max(1.0),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Spinner {
    progress: f32,
    color: Color,
    size: SpinnerSize,
    loading: bool,
}

impl Spinner {
    pub fn new(theme: &Theme) -> Self {
        Self {
            progress: 0.0,
            color: theme.palette.primary,
            size: SpinnerSize::Two,
            loading: true,
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

    pub fn size(mut self, size: SpinnerSize) -> Self {
        self.size = size;
        self
    }

    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }
}

pub fn spinner<Message>(spinner: Spinner) -> canvas::Canvas<Spinner, Message> {
    let size = spinner.size.pixels();
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
        if !self.loading {
            return Vec::new();
        }

        let mut frame = canvas::Frame::new(renderer, bounds.size());
        let center = frame.center();
        let icon = char::from(Icon::Loader);
        let size = self.size.pixels();
        let rotation = self.progress.rem_euclid(1.0) * 2.0 * PI;

        frame.with_save(|frame| {
            frame.translate(iced::Vector::new(center.x, center.y));
            frame.rotate(rotation);
            frame.translate(iced::Vector::new(-center.x, -center.y));

            frame.fill_text(Text {
                content: icon.to_string(),
                position: Point::new(center.x, center.y),
                color: self.color,
                size: size.into(),
                font: Font::with_name("lucide"),
                align_x: iced::widget::text::Alignment::Center,
                align_y: Vertical::Center,
                ..Text::default()
            });
        });

        vec![frame.into_geometry()]
    }
}
