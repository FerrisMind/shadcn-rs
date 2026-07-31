//! Canvas glyph for the sidebar trigger (Lucide `PanelLeft`).

use crate::iced_compat::widget::canvas;
use crate::iced_compat::widget::canvas::{LineCap, LineJoin, Path, Stroke};
use crate::iced_compat::{Color, Element, Length, Point, Rectangle, Renderer, Size, mouse};

const LUCIDE_VIEWBOX: f32 = 24.0;
const LUCIDE_STROKE: f32 = 2.0;

/// Canvas program that paints the Lucide `panel-left` glyph.
#[derive(Clone, Copy, Debug)]
pub(super) struct PanelLeftIcon {
    size_px: f32,
    color: Color,
}

impl PanelLeftIcon {
    pub(super) fn new(size_px: f32, color: Color) -> Self {
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

impl<Message> canvas::Program<Message> for PanelLeftIcon {
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

        let outline = Path::rounded_rectangle(
            point(5.0, 3.0),
            Size::new(14.0 * scale, 18.0 * scale),
            (2.0 * scale).into(),
        );

        let divider = Path::new(|builder| {
            builder.move_to(point(9.0, 3.0));
            builder.line_to(point(9.0, 21.0));
        });

        let stroke = Stroke::default()
            .with_width(LUCIDE_STROKE * scale)
            .with_line_cap(LineCap::Round)
            .with_line_join(LineJoin::Round)
            .with_color(self.color);

        frame.stroke(&outline, stroke);
        frame.stroke(&divider, stroke);

        vec![frame.into_geometry()]
    }
}

pub(super) fn panel_left_icon<'a, Message: 'a>(size_px: f32, color: Color) -> Element<'a, Message> {
    let icon = PanelLeftIcon::new(size_px, color);
    canvas::Canvas::<PanelLeftIcon, Message>::new(icon)
        .width(Length::Fixed(icon.size_px))
        .height(Length::Fixed(icon.size_px))
        .into()
}
