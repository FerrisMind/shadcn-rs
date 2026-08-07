//! Small vector icons used by the composable tree renderer.

use crate::iced_compat::widget::canvas;
use crate::iced_compat::widget::canvas::{LineCap, LineJoin, Path, Stroke};
use crate::iced_compat::{Color, Element, Point, Radians, Rectangle, Renderer, Size};
use shadcn_common::TreeIconKey;

#[derive(Debug, Clone)]
struct TreeIcon {
    key: TreeIconKey,
    color: Color,
}

pub(super) fn element<'a, Message: 'a>(
    key: TreeIconKey,
    size: f32,
    color: Color,
) -> Element<'a, Message> {
    canvas::Canvas::new(TreeIcon { key, color })
        .width(crate::iced_compat::Length::Fixed(size))
        .height(crate::iced_compat::Length::Fixed(size))
        .into()
}

/// Text fallback used by the virtualized renderer's direct draw path.
pub(super) fn glyph(key: &TreeIconKey, open: bool) -> &'static str {
    match key {
        TreeIconKey::Folder => {
            if open {
                "▾"
            } else {
                "▸"
            }
        }
        TreeIconKey::FolderOpen => "▾",
        TreeIconKey::File => "□",
        TreeIconKey::Loader => "◌",
        TreeIconKey::Named(name) if name.eq_ignore_ascii_case("markdown") => "M",
        TreeIconKey::Named(_) => "•",
        _ => "•",
    }
}

impl<Message> canvas::Program<Message> for TreeIcon {
    type State = ();

    fn update(
        &self,
        _state: &mut Self::State,
        _event: &canvas::Event,
        _bounds: Rectangle,
        _cursor: crate::iced_compat::mouse::Cursor,
    ) -> Option<canvas::Action<Message>> {
        None
    }

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &crate::iced_compat::Theme,
        bounds: Rectangle,
        _cursor: crate::iced_compat::mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let size = bounds.width.min(bounds.height);
        if !size.is_finite() || size <= 0.0 {
            return Vec::new();
        }

        let mut frame = canvas::Frame::new(renderer, bounds.size());
        let center = frame.center();
        let stroke = Stroke::default()
            .with_width((size * 0.10).clamp(1.0, 1.8))
            .with_line_cap(LineCap::Round)
            .with_line_join(LineJoin::Round)
            .with_color(self.color);

        match &self.key {
            TreeIconKey::Folder | TreeIconKey::FolderOpen => {
                draw_folder(&mut frame, size, stroke);
            }
            TreeIconKey::File => draw_file(&mut frame, size, stroke),
            TreeIconKey::Loader => draw_loader(&mut frame, center, size, stroke),
            TreeIconKey::Named(name) if name.eq_ignore_ascii_case("markdown") => {
                draw_file(&mut frame, size, stroke);
                frame.fill_text(canvas::Text {
                    content: "M".to_owned(),
                    position: center,
                    color: self.color,
                    size: (size * 0.42).into(),
                    font: crate::iced_compat::Font::DEFAULT,
                    align_x: crate::iced_compat::widget::text::Alignment::Center,
                    align_y: crate::iced_compat::alignment::Vertical::Center,
                    ..canvas::Text::default()
                });
            }
            TreeIconKey::Named(_) => draw_named(&mut frame, center, size, stroke, self.color),
            _ => draw_named(&mut frame, center, size, stroke, self.color),
        }

        vec![frame.into_geometry()]
    }
}

fn draw_folder(frame: &mut canvas::Frame<Renderer>, size: f32, stroke: Stroke) {
    let inset = size * 0.12;
    let top = size * 0.25;
    let tab = size * 0.39;
    let path = Path::new(|builder| {
        builder.move_to(Point::new(inset, top));
        builder.line_to(Point::new(tab, top));
        builder.line_to(Point::new(tab + size * 0.10, top + size * 0.12));
        builder.line_to(Point::new(size - inset, top + size * 0.12));
        builder.line_to(Point::new(size - inset, size * 0.78));
        builder.line_to(Point::new(inset, size * 0.78));
        builder.close();
    });
    frame.stroke(&path, stroke);
}

fn draw_file(frame: &mut canvas::Frame<Renderer>, size: f32, stroke: Stroke) {
    let inset = size * 0.18;
    let fold = size * 0.28;
    let path = Path::new(|builder| {
        builder.move_to(Point::new(inset, inset));
        builder.line_to(Point::new(size - fold, inset));
        builder.line_to(Point::new(size - inset, inset + fold));
        builder.line_to(Point::new(size - inset, size - inset));
        builder.line_to(Point::new(inset, size - inset));
        builder.close();
    });
    frame.stroke(&path, stroke);
    frame.stroke(
        &Path::line(
            Point::new(size - fold, inset),
            Point::new(size - fold, inset + fold),
        ),
        stroke,
    );
}

fn draw_loader(frame: &mut canvas::Frame<Renderer>, center: Point, size: f32, stroke: Stroke) {
    let radius = size * 0.34;
    let arc = Path::new(|builder| {
        builder.arc(canvas::path::Arc {
            center,
            radius,
            start_angle: Radians(-0.8),
            end_angle: Radians(4.8),
        });
    });
    frame.stroke(&arc, stroke);
}

fn draw_named(
    frame: &mut canvas::Frame<Renderer>,
    center: Point,
    size: f32,
    stroke: Stroke,
    color: Color,
) {
    frame.stroke(&Path::circle(center, size * 0.32), stroke);
    frame.fill_rectangle(
        Point::new(center.x - size * 0.05, center.y - size * 0.05),
        Size::new(size * 0.10, size * 0.10),
        color,
    );
}
