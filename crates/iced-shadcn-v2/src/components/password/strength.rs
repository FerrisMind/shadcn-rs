//! Segmented password strength meter.

use std::fmt;

use crate::iced_compat::widget::canvas;
use crate::iced_compat::widget::canvas::{Fill, Path, Stroke};
use crate::iced_compat::{Color, Element, Length, Point, Rectangle, Renderer, Size, mouse};
use crate::recipes::component_radius_px;
use crate::theme::Theme;
use shadcn_common::{PASSWORD_STRENGTH_SEGMENTS, PasswordScore, password_score_rgb};
use twill_core::prelude::theme::SemanticColor;

/// Strength meter (`Password.Strength`).
///
/// Paints a 6 px rounded track with a score-colored fill and four segment
/// rings that create the gap look from the extras `Meter` markup.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct PasswordStrength<'a> {
    theme: &'a Theme,
    score: PasswordScore,
    width: Length,
}

impl fmt::Debug for PasswordStrength<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PasswordStrength")
            .field("theme", &self.theme)
            .field("score", &self.score)
            .field("width", &self.width)
            .finish()
    }
}

impl<'a> PasswordStrength<'a> {
    /// Creates a strength meter at score `0`.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            score: PasswordScore::Zero,
            width: Length::Fill,
        }
    }

    /// Sets the zxcvbn score (`0`–`4`).
    #[must_use = "builder methods return the modified strength meter"]
    pub fn score(mut self, score: PasswordScore) -> Self {
        self.score = score;
        self
    }

    /// Sets the meter width.
    #[must_use = "builder methods return the modified strength meter"]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Builds the meter as an iced element.
    pub fn into_element<Message: 'a>(self) -> Element<'a, Message> {
        let recipe = self.theme.style.password();
        let track = self.theme.semantic_color(SemanticColor::Accent);
        let background = self.theme.semantic_color(SemanticColor::Background);
        let (r, g, b) = password_score_rgb(self.score.as_u8());
        let fill_color = Color::from_rgb(r, g, b);
        let radius = component_radius_px(self.theme, recipe.strength_radius);

        let program = StrengthMeter {
            score: self.score.as_u8(),
            track,
            fill: fill_color,
            ring: background,
            height: recipe.strength_height_px,
            gap: recipe.strength_gap_px,
            ring_width: recipe.strength_ring_px,
            radius,
            segments: PASSWORD_STRENGTH_SEGMENTS,
        };

        canvas::Canvas::new(program)
            .width(self.width)
            .height(Length::Fixed(recipe.strength_height_px))
            .into()
    }
}

impl<'a, Message: 'a> From<PasswordStrength<'a>> for Element<'a, Message> {
    fn from(strength: PasswordStrength<'a>) -> Self {
        strength.into_element()
    }
}

#[derive(Clone, Copy, Debug)]
struct StrengthMeter {
    score: u8,
    track: Color,
    fill: Color,
    ring: Color,
    height: f32,
    gap: f32,
    ring_width: f32,
    radius: f32,
    segments: u8,
}

impl<Message> canvas::Program<Message> for StrengthMeter {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
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

        let mut frame = canvas::Frame::new(renderer, size);
        let height = self.height.min(size.height);
        let track_rect = Rectangle {
            x: 0.0,
            y: (size.height - height) / 2.0,
            width: size.width,
            height,
        };

        let track = Path::rounded_rectangle(
            Point::new(track_rect.x, track_rect.y),
            Size::new(track_rect.width, track_rect.height),
            self.radius.max(0.0).into(),
        );
        frame.fill(&track, Fill::from(self.track));

        let fraction = f32::from(self.score) / 4.0;
        if fraction > 0.0 {
            let fill_width = (track_rect.width * fraction).max(0.0);
            let fill = Path::rounded_rectangle(
                Point::new(track_rect.x, track_rect.y),
                Size::new(fill_width, track_rect.height),
                self.radius.max(0.0).into(),
            );
            frame.fill(&fill, Fill::from(self.fill));
        }

        let segments = u32::from(self.segments.max(1));
        let slot = track_rect.width / segments as f32;
        for index in 0..segments {
            let x = track_rect.x + slot * index as f32;
            let ring = Path::rounded_rectangle(
                Point::new(x, track_rect.y),
                Size::new(slot, track_rect.height),
                self.radius.max(0.0).into(),
            );
            frame.stroke(
                &ring,
                Stroke::default()
                    .with_width(self.ring_width.max(1.0))
                    .with_color(self.ring),
            );

            if index + 1 < segments {
                let gap_x = x + slot - self.gap / 2.0;
                let gap = Path::rectangle(
                    Point::new(gap_x, track_rect.y),
                    Size::new(self.gap, track_rect.height),
                );
                frame.fill(&gap, Fill::from(self.ring));
            }
        }

        vec![frame.into_geometry()]
    }
}
