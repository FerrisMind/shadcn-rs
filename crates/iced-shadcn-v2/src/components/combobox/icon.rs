//! Lucide canvas glyph used by the combobox trigger.

use crate::iced_compat::widget::canvas;
use crate::iced_compat::widget::canvas::{LineCap, LineJoin, Path, Stroke};
use crate::iced_compat::{Color, Element, Length, Point, Rectangle, Renderer, mouse};

const VIEWBOX: f32 = 24.0;
const STROKE: f32 = 2.0;

/// Paints Lucide's `chevrons-up-down` glyph.
#[derive(Debug, Clone, Copy)]
struct ChevronsUpDown {
    size_px: f32,
    color: Color,
}

impl ChevronsUpDown {
    fn new(size_px: f32, color: Color) -> Self {
        Self {
            size_px: if size_px.is_finite() {
                size_px.max(1.0)
            } else {
                1.0
            },
            color,
        }
    }
}

impl<Message> canvas::Program<Message> for ChevronsUpDown {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &crate::iced_compat::Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        if bounds.width <= 0.0
            || bounds.height <= 0.0
            || !bounds.width.is_finite()
            || !bounds.height.is_finite()
        {
            return Vec::new();
        }

        let size = bounds.width.min(bounds.height);
        let scale = size / VIEWBOX;
        let offset_x = (bounds.width - size) / 2.0;
        let offset_y = (bounds.height - size) / 2.0;
        let point = |x: f32, y: f32| Point::new(offset_x + x * scale, offset_y + y * scale);

        let path = Path::new(|builder| {
            // `m7 15 5 5 5-5` and `m7 9 5-5 5 5`.
            builder.move_to(point(7.0, 15.0));
            builder.line_to(point(12.0, 20.0));
            builder.line_to(point(17.0, 15.0));
            builder.move_to(point(7.0, 9.0));
            builder.line_to(point(12.0, 4.0));
            builder.line_to(point(17.0, 9.0));
        });

        let stroke = Stroke::default()
            .with_width(STROKE * scale)
            .with_line_cap(LineCap::Round)
            .with_line_join(LineJoin::Round)
            .with_color(self.color);

        let mut frame = canvas::Frame::new(renderer, bounds.size());
        frame.stroke(&path, stroke);
        vec![frame.into_geometry()]
    }
}

/// Returns a fixed-size combobox chevron element.
pub(super) fn chevrons_up_down<'a, Message: 'a>(
    size_px: f32,
    color: Color,
) -> Element<'a, Message> {
    let icon = ChevronsUpDown::new(size_px, color);

    canvas::Canvas::<ChevronsUpDown, Message>::new(icon)
        .width(Length::Fixed(icon.size_px))
        .height(Length::Fixed(icon.size_px))
        .into()
}
