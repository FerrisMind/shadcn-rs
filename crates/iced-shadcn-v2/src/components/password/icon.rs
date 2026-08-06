//! Lucide `eye` / `eye-off` canvas glyphs for the visibility toggle.

use crate::iced_compat::widget::canvas;
use crate::iced_compat::widget::canvas::{LineCap, LineJoin, Path, Stroke};
use crate::iced_compat::{Color, Element, Length, Point, Rectangle, Renderer, mouse};

const LUCIDE_VIEWBOX: f32 = 24.0;
const LUCIDE_STROKE: f32 = 2.0;

/// Which Lucide glyph to paint.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum EyeGlyph {
    /// Password is hidden — show the open eye (reveal action).
    Eye,
    /// Password is visible — show the eye-off (mask action).
    EyeOff,
}

/// Canvas program that paints a Lucide eye glyph centered in its bounds.
#[derive(Clone, Copy, Debug)]
pub(super) struct EyeIcon {
    size_px: f32,
    color: Color,
    glyph: EyeGlyph,
}

impl EyeIcon {
    pub(super) fn new(size_px: f32, color: Color, glyph: EyeGlyph) -> Self {
        Self {
            size_px: if size_px.is_finite() {
                size_px.max(1.0)
            } else {
                1.0
            },
            color,
            glyph,
        }
    }
}

impl<Message> canvas::Program<Message> for EyeIcon {
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

        let scale = size.width.min(size.height) / LUCIDE_VIEWBOX;
        let inset_x = (size.width - LUCIDE_VIEWBOX * scale) / 2.0;
        let inset_y = (size.height - LUCIDE_VIEWBOX * scale) / 2.0;
        let point = |x: f32, y: f32| Point::new(inset_x + x * scale, inset_y + y * scale);

        let mut frame = canvas::Frame::new(renderer, size);
        let stroke = Stroke::default()
            .with_width(LUCIDE_STROKE * scale)
            .with_line_cap(LineCap::Round)
            .with_line_join(LineJoin::Round)
            .with_color(self.color);

        match self.glyph {
            EyeGlyph::Eye => {
                // Lucide eye: oval + pupil.
                // path: M2.062 12.348a1 1 0 0 1 0-.696 10.75 10.75 0 0 1 19.876 0 1 1 0 0 1 0 .696 10.75 10.75 0 0 1-19.876 0
                // circle cx=12 cy=12 r=3
                let outline = Path::new(|builder| {
                    builder.move_to(point(2.0, 12.0));
                    builder.quadratic_curve_to(point(6.5, 5.5), point(12.0, 5.5));
                    builder.quadratic_curve_to(point(17.5, 5.5), point(22.0, 12.0));
                    builder.quadratic_curve_to(point(17.5, 18.5), point(12.0, 18.5));
                    builder.quadratic_curve_to(point(6.5, 18.5), point(2.0, 12.0));
                });
                frame.stroke(&outline, stroke);

                let pupil = Path::circle(point(12.0, 12.0), 3.0 * scale);
                frame.stroke(&pupil, stroke);
            }
            EyeGlyph::EyeOff => {
                // Simplified eye-off: slash + partial oval + pupil.
                let outline = Path::new(|builder| {
                    builder.move_to(point(2.0, 12.0));
                    builder.quadratic_curve_to(point(6.5, 5.5), point(12.0, 5.5));
                    builder.quadratic_curve_to(point(17.5, 5.5), point(22.0, 12.0));
                    builder.quadratic_curve_to(point(17.5, 18.5), point(12.0, 18.5));
                    builder.quadratic_curve_to(point(6.5, 18.5), point(2.0, 12.0));
                });
                frame.stroke(&outline, stroke);

                let pupil = Path::circle(point(12.0, 12.0), 3.0 * scale);
                frame.stroke(&pupil, stroke);

                let slash = Path::new(|builder| {
                    builder.move_to(point(3.0, 3.0));
                    builder.line_to(point(21.0, 21.0));
                });
                frame.stroke(&slash, stroke);
            }
        }

        vec![frame.into_geometry()]
    }
}

pub(super) fn eye_icon<'a, Message: 'a>(
    size_px: f32,
    color: Color,
    glyph: EyeGlyph,
) -> Element<'a, Message> {
    let icon = EyeIcon::new(size_px, color, glyph);
    canvas::Canvas::<EyeIcon, Message>::new(icon)
        .width(Length::Fixed(icon.size_px))
        .height(Length::Fixed(icon.size_px))
        .into()
}
