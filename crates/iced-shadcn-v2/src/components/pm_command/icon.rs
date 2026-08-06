//! Small Lucide-compatible canvas icons used by PMCommand.

use crate::iced_compat::widget::canvas;
use crate::iced_compat::widget::canvas::{LineCap, LineJoin, Path, Stroke};
use crate::iced_compat::{Color, Element, Point, Renderer, Size};

const VIEWBOX: f32 = 24.0;

/// The terminal glyph shown beside the package-manager tabs.
#[derive(Debug, Clone, Copy)]
pub(super) struct TerminalIcon {
    color: Color,
    size: f32,
}

impl TerminalIcon {
    /// Builds an iced element containing the terminal glyph.
    pub(super) fn element<'a, Message: 'a>(color: Color, size: f32) -> Element<'a, Message> {
        canvas::Canvas::<Self, Message>::new(Self { color, size })
            .width(crate::iced_compat::Length::Fixed(size))
            .height(crate::iced_compat::Length::Fixed(size))
            .into()
    }
}

/// The clipboard glyph used as PMCommand's idle copy icon.
#[derive(Debug, Clone, Copy)]
pub(super) struct ClipboardIcon {
    color: Color,
    size: f32,
}

impl ClipboardIcon {
    /// Builds an iced element containing the clipboard glyph.
    pub(super) fn element<'a, Message: 'a>(color: Color, size: f32) -> Element<'a, Message> {
        canvas::Canvas::<Self, Message>::new(Self { color, size })
            .width(crate::iced_compat::Length::Fixed(size))
            .height(crate::iced_compat::Length::Fixed(size))
            .into()
    }
}

impl<Message> canvas::Program<Message> for TerminalIcon {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &crate::iced_compat::Theme,
        bounds: crate::iced_compat::Rectangle,
        _cursor: crate::iced_compat::mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());
        let scale = (self.size / VIEWBOX).max(0.001);
        frame.with_save(|frame| {
            frame.translate(crate::iced_compat::Vector::new(
                (bounds.width - VIEWBOX * scale) / 2.0,
                (bounds.height - VIEWBOX * scale) / 2.0,
            ));
            frame.scale(scale);

            let stroke = Stroke::default()
                .with_width(2.0)
                .with_line_cap(LineCap::Round)
                .with_line_join(LineJoin::Round)
                .with_color(self.color);
            let prompt = Path::new(|builder| {
                builder.move_to(Point::new(4.0, 4.0));
                builder.line_to(Point::new(10.0, 10.0));
                builder.line_to(Point::new(4.0, 16.0));
            });
            let cursor = Path::line(Point::new(12.0, 19.0), Point::new(20.0, 19.0));
            frame.stroke(&prompt, stroke);
            frame.stroke(&cursor, stroke);
        });

        vec![frame.into_geometry()]
    }
}

impl<Message> canvas::Program<Message> for ClipboardIcon {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &crate::iced_compat::Theme,
        bounds: crate::iced_compat::Rectangle,
        _cursor: crate::iced_compat::mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());
        let scale = (self.size / VIEWBOX).max(0.001);
        frame.with_save(|frame| {
            frame.translate(crate::iced_compat::Vector::new(
                (bounds.width - VIEWBOX * scale) / 2.0,
                (bounds.height - VIEWBOX * scale) / 2.0,
            ));
            frame.scale(scale);

            let stroke = Stroke::default()
                .with_width(2.0)
                .with_line_cap(LineCap::Round)
                .with_line_join(LineJoin::Round)
                .with_color(self.color);
            let body = Path::new(|builder| {
                builder.move_to(Point::new(6.0, 4.0));
                builder.line_to(Point::new(4.0, 4.0));
                builder.arc_to(Point::new(2.0, 4.0), Point::new(2.0, 6.0), 2.0);
                builder.line_to(Point::new(2.0, 20.0));
                builder.arc_to(Point::new(2.0, 22.0), Point::new(4.0, 22.0), 2.0);
                builder.line_to(Point::new(18.0, 22.0));
                builder.arc_to(Point::new(20.0, 22.0), Point::new(20.0, 20.0), 2.0);
                builder.line_to(Point::new(20.0, 6.0));
                builder.arc_to(Point::new(20.0, 4.0), Point::new(18.0, 4.0), 2.0);
                builder.line_to(Point::new(16.0, 4.0));
            });
            let tab =
                Path::rounded_rectangle(Point::new(8.0, 2.0), Size::new(8.0, 4.0), 1.0.into());
            frame.stroke(&body, stroke);
            frame.stroke(&tab, stroke);
        });

        vec![frame.into_geometry()]
    }
}
