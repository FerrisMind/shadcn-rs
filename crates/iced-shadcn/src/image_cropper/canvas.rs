use std::rc::Rc;

use iced::mouse;
use iced::widget::canvas::{self, Event, Path, Stroke};
use iced::widget::image;
use iced::{Color, Point, Rectangle, Renderer, Theme as IcedTheme};

use crate::theme::Theme;

use super::geometry::{clamp_rect, move_rect};
use super::types::{ImageCropRect, ImageCropShape, ImageCropperAction};

const HANDLE_RADIUS: f32 = 8.0;
const OVERLAY_ALPHA: f32 = 0.58;

#[derive(Debug, Default)]
pub struct CropCanvasState {
    drag: Option<CanvasDrag>,
}

#[derive(Debug, Clone, Copy)]
enum CanvasDrag {
    Move {
        pointer_origin: Point,
        rect_origin: ImageCropRect,
    },
    Resize {
        corner: Corner,
        rect_origin: ImageCropRect,
    },
}

#[derive(Debug, Clone, Copy)]
enum Corner {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Clone)]
pub struct CropCanvas<'a, Message> {
    pub image_handle: image::Handle,
    pub image_size: (u32, u32),
    pub crop_rect: ImageCropRect,
    pub shape: ImageCropShape,
    pub theme: Theme,
    pub zoom: f32,
    pub on_action: Rc<dyn Fn(ImageCropperAction) -> Message + 'a>,
}

impl<Message> canvas::Program<Message, IcedTheme, Renderer> for CropCanvas<'_, Message> {
    type State = CropCanvasState;

    fn update(
        &self,
        state: &mut Self::State,
        event: &Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Option<canvas::Action<Message>> {
        let pointer = cursor.position_in(bounds)?;
        let display = image_display_rect(bounds, self.image_size, self.zoom);

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                if let Some(corner) = hit_corner(self.crop_rect, self.image_size, display, pointer)
                {
                    state.drag = Some(CanvasDrag::Resize {
                        corner,
                        rect_origin: self.crop_rect,
                    });
                    return Some(canvas::Action::capture());
                }

                if crop_rect_screen(self.crop_rect, self.image_size, display).contains(pointer) {
                    state.drag = Some(CanvasDrag::Move {
                        pointer_origin: pointer,
                        rect_origin: self.crop_rect,
                    });
                    return Some(canvas::Action::capture());
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(iced::touch::Event::FingerLifted { .. })
            | Event::Touch(iced::touch::Event::FingerLost { .. }) => {
                state.drag = None;
                return Some(canvas::Action::capture());
            }
            Event::Mouse(mouse::Event::CursorMoved { .. })
            | Event::Touch(iced::touch::Event::FingerMoved { .. }) => {
                let next = match state.drag {
                    Some(CanvasDrag::Move {
                        pointer_origin,
                        rect_origin,
                    }) => {
                        let origin = screen_to_image(pointer_origin, display, self.image_size)?;
                        let current = screen_to_image(pointer, display, self.image_size)?;
                        let dx = current.x.round() as i32 - origin.x.round() as i32;
                        let dy = current.y.round() as i32 - origin.y.round() as i32;
                        move_rect(rect_origin, dx, dy, self.image_size)
                    }
                    Some(CanvasDrag::Resize {
                        corner,
                        rect_origin,
                    }) => {
                        resize_from_corner(rect_origin, corner, pointer, display, self.image_size)?
                    }
                    None => return None,
                };

                return Some(
                    canvas::Action::publish((self.on_action)(ImageCropperAction::CropRectChanged(
                        next,
                    )))
                    .and_capture(),
                );
            }
            _ => {}
        }

        None
    }

    fn draw(
        &self,
        state: &Self::State,
        renderer: &Renderer,
        _theme: &IcedTheme,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry<Renderer>> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());
        let display = image_display_rect(bounds, self.image_size, self.zoom);
        let crop_screen = crop_rect_screen(self.crop_rect, self.image_size, display);

        frame.fill(
            &Path::rectangle(Point::ORIGIN, bounds.size()),
            self.theme.palette.muted,
        );
        frame.draw_image(
            Rectangle {
                x: display.x - bounds.x,
                y: display.y - bounds.y,
                width: display.width,
                height: display.height,
            },
            canvas::Image::new(&self.image_handle),
        );

        draw_overlay(&mut frame, bounds, crop_screen, self.shape);
        frame.stroke(
            &Path::rectangle(
                Point::new(crop_screen.x - bounds.x, crop_screen.y - bounds.y),
                crop_screen.size(),
            ),
            Stroke::default()
                .with_color(self.theme.palette.primary)
                .with_width(2.0),
        );

        for point in corner_points(crop_screen) {
            let local = Point::new(point.x - bounds.x, point.y - bounds.y);
            frame.fill(
                &Path::circle(local, HANDLE_RADIUS),
                self.theme.palette.background,
            );
            frame.stroke(
                &Path::circle(local, HANDLE_RADIUS),
                Stroke::default()
                    .with_color(self.theme.palette.primary)
                    .with_width(2.0),
            );
        }

        if state.drag.is_none() && cursor.is_over(bounds) {
            let label = if matches!(self.shape, ImageCropShape::Round) {
                "Drag or resize"
            } else {
                "Square crop"
            };
            frame.fill_text(canvas::Text {
                content: label.into(),
                position: Point::new(16.0, 20.0),
                color: self.theme.palette.foreground,
                size: iced::Pixels(14.0),
                ..Default::default()
            });
        }

        vec![frame.into_geometry()]
    }

    fn mouse_interaction(
        &self,
        state: &Self::State,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> mouse::Interaction {
        let Some(pointer) = cursor.position_in(bounds) else {
            return mouse::Interaction::default();
        };
        let display = image_display_rect(bounds, self.image_size, self.zoom);

        if state.drag.is_some() {
            return mouse::Interaction::Grabbing;
        }
        if hit_corner(self.crop_rect, self.image_size, display, pointer).is_some() {
            return mouse::Interaction::ResizingDiagonallyDown;
        }
        if crop_rect_screen(self.crop_rect, self.image_size, display).contains(pointer) {
            return mouse::Interaction::Grab;
        }
        mouse::Interaction::default()
    }
}

fn image_display_rect(bounds: Rectangle, image_size: (u32, u32), zoom: f32) -> Rectangle {
    let width_ratio = bounds.width / image_size.0.max(1) as f32;
    let height_ratio = bounds.height / image_size.1.max(1) as f32;
    let scale = width_ratio.min(height_ratio) * zoom;
    let width = image_size.0 as f32 * scale;
    let height = image_size.1 as f32 * scale;
    Rectangle {
        x: bounds.x + (bounds.width - width) / 2.0,
        y: bounds.y + (bounds.height - height) / 2.0,
        width,
        height,
    }
}

fn crop_rect_screen(rect: ImageCropRect, image_size: (u32, u32), display: Rectangle) -> Rectangle {
    let scale_x = display.width / image_size.0.max(1) as f32;
    let scale_y = display.height / image_size.1.max(1) as f32;
    Rectangle {
        x: display.x + rect.x as f32 * scale_x,
        y: display.y + rect.y as f32 * scale_y,
        width: rect.width as f32 * scale_x,
        height: rect.height as f32 * scale_y,
    }
}

fn screen_to_image(pointer: Point, display: Rectangle, image_size: (u32, u32)) -> Option<Point> {
    if display.width <= 0.0 || display.height <= 0.0 {
        return None;
    }
    Some(Point::new(
        ((pointer.x - display.x) / display.width).clamp(0.0, 1.0) * image_size.0 as f32,
        ((pointer.y - display.y) / display.height).clamp(0.0, 1.0) * image_size.1 as f32,
    ))
}

fn resize_from_corner(
    rect: ImageCropRect,
    corner: Corner,
    pointer: Point,
    display: Rectangle,
    image_size: (u32, u32),
) -> Option<ImageCropRect> {
    let point = screen_to_image(pointer, display, image_size)?;
    let anchor = match corner {
        Corner::TopLeft => Point::new((rect.x + rect.width) as f32, (rect.y + rect.height) as f32),
        Corner::TopRight => Point::new(rect.x as f32, (rect.y + rect.height) as f32),
        Corner::BottomLeft => Point::new((rect.x + rect.width) as f32, rect.y as f32),
        Corner::BottomRight => Point::new(rect.x as f32, rect.y as f32),
    };
    let size = (anchor.x - point.x)
        .abs()
        .min((anchor.y - point.y).abs())
        .round() as u32;
    let next = match corner {
        Corner::TopLeft => ImageCropRect::new(
            rect.x + rect.width - size,
            rect.y + rect.height - size,
            size,
            size,
        ),
        Corner::TopRight => ImageCropRect::new(rect.x, rect.y + rect.height - size, size, size),
        Corner::BottomLeft => ImageCropRect::new(rect.x + rect.width - size, rect.y, size, size),
        Corner::BottomRight => ImageCropRect::new(rect.x, rect.y, size, size),
    };
    Some(clamp_rect(next, image_size))
}

fn hit_corner(
    rect: ImageCropRect,
    image_size: (u32, u32),
    display: Rectangle,
    pointer: Point,
) -> Option<Corner> {
    let crop = crop_rect_screen(rect, image_size, display);
    for (corner, point) in [
        (Corner::TopLeft, Point::new(crop.x, crop.y)),
        (Corner::TopRight, Point::new(crop.x + crop.width, crop.y)),
        (Corner::BottomLeft, Point::new(crop.x, crop.y + crop.height)),
        (
            Corner::BottomRight,
            Point::new(crop.x + crop.width, crop.y + crop.height),
        ),
    ] {
        let dx = pointer.x - point.x;
        let dy = pointer.y - point.y;
        if (dx * dx + dy * dy).sqrt() <= HANDLE_RADIUS * 1.8 {
            return Some(corner);
        }
    }
    None
}

fn corner_points(rect: Rectangle) -> [Point; 4] {
    [
        Point::new(rect.x, rect.y),
        Point::new(rect.x + rect.width, rect.y),
        Point::new(rect.x, rect.y + rect.height),
        Point::new(rect.x + rect.width, rect.y + rect.height),
    ]
}

fn draw_overlay(
    frame: &mut canvas::Frame<Renderer>,
    bounds: Rectangle,
    crop: Rectangle,
    shape: ImageCropShape,
) {
    let overlay = Color {
        a: OVERLAY_ALPHA,
        ..Color::BLACK
    };
    frame.fill(
        &Path::rectangle(Point::ORIGIN, iced::Size::new(bounds.width, crop.y)),
        overlay,
    );
    frame.fill(
        &Path::rectangle(
            Point::new(0.0, crop.y),
            iced::Size::new(crop.x, crop.height),
        ),
        overlay,
    );
    frame.fill(
        &Path::rectangle(
            Point::new(crop.x + crop.width, crop.y),
            iced::Size::new((bounds.width - crop.x - crop.width).max(0.0), crop.height),
        ),
        overlay,
    );
    frame.fill(
        &Path::rectangle(
            Point::new(0.0, crop.y + crop.height),
            iced::Size::new(
                bounds.width,
                (bounds.height - crop.y - crop.height).max(0.0),
            ),
        ),
        overlay,
    );

    if matches!(shape, ImageCropShape::Round) {
        frame.stroke(
            &Path::circle(
                Point::new(crop.center_x() - bounds.x, crop.center_y() - bounds.y),
                crop.width.min(crop.height) / 2.0,
            ),
            Stroke::default().with_color(Color::WHITE).with_width(2.0),
        );
    }
}
