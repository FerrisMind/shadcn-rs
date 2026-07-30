//! Canvas glyph for the dialog close button.
//!
//! The crate has no icon font dependency, so the Lucide `XIcon` the source
//! component renders inside `DialogPrimitive.Close` is drawn directly on a
//! canvas from its path data.

use crate::iced_compat::widget::canvas;
use crate::iced_compat::widget::canvas::{LineCap, LineJoin, Path, Stroke};
use crate::iced_compat::{Color, Element, Length, Point, Rectangle, Renderer, mouse};

/// Lucide icons are authored on a 24×24 grid with a 2 px stroke.
const LUCIDE_VIEWBOX: f32 = 24.0;
const LUCIDE_STROKE: f32 = 2.0;

/// Canvas program that paints the Lucide `x` glyph centered in its bounds.
#[derive(Clone, Copy, Debug)]
pub(super) struct CloseIcon {
    size_px: f32,
    color: Color,
}

impl CloseIcon {
    /// Creates the glyph program; `size_px` is clamped to at least 1 px.
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

impl<Message> canvas::Program<Message> for CloseIcon {
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

        // `<path d="M18 6 6 18" /><path d="m6 6 12 12" />`
        let path = Path::new(|builder| {
            builder.move_to(point(18.0, 6.0));
            builder.line_to(point(6.0, 18.0));
            builder.move_to(point(6.0, 6.0));
            builder.line_to(point(18.0, 18.0));
        });

        frame.stroke(
            &path,
            Stroke::default()
                .with_width(LUCIDE_STROKE * scale)
                .with_line_cap(LineCap::Round)
                .with_line_join(LineJoin::Round)
                .with_color(self.color),
        );

        vec![frame.into_geometry()]
    }
}

/// Wraps a [`CloseIcon`] program into a fixed-size canvas element.
pub(super) fn close_icon<'a, Message: 'a>(size_px: f32, color: Color) -> Element<'a, Message> {
    let icon = CloseIcon::new(size_px, color);

    canvas::Canvas::<CloseIcon, Message>::new(icon)
        .width(Length::Fixed(icon.size_px))
        .height(Length::Fixed(icon.size_px))
        .into()
}
