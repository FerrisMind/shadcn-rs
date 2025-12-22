use egui::{Color32, Painter, Pos2, Stroke, vec2};

fn stroke_width(size: f32) -> f32 {
    (size * 0.12).clamp(1.5, 2.5)
}

pub fn icon_check(painter: &Painter, center: Pos2, size: f32, color: Color32) {
    let s = size;
    let w = stroke_width(s);
    let stroke = Stroke::new(w, color);

    let a = center + vec2(-s * 0.28, s * 0.02);
    let b = center + vec2(-s * 0.06, s * 0.24);
    let c = center + vec2(s * 0.34, -s * 0.20);
    painter.line_segment([a, b], stroke);
    painter.line_segment([b, c], stroke);
}

pub fn icon_chevrons_up_down(painter: &Painter, center: Pos2, size: f32, color: Color32) {
    let s = size;
    let w = stroke_width(s);
    let stroke = Stroke::new(w, color);

    let up_y = center.y - s * 0.12;
    let down_y = center.y + s * 0.12;

    let left = center.x - s * 0.20;
    let right = center.x + s * 0.20;

    painter.line_segment(
        [
            Pos2::new(left, up_y + s * 0.12),
            Pos2::new(center.x, up_y - s * 0.12),
        ],
        stroke,
    );
    painter.line_segment(
        [
            Pos2::new(center.x, up_y - s * 0.12),
            Pos2::new(right, up_y + s * 0.12),
        ],
        stroke,
    );

    painter.line_segment(
        [
            Pos2::new(left, down_y - s * 0.12),
            Pos2::new(center.x, down_y + s * 0.12),
        ],
        stroke,
    );
    painter.line_segment(
        [
            Pos2::new(center.x, down_y + s * 0.12),
            Pos2::new(right, down_y - s * 0.12),
        ],
        stroke,
    );
}

pub fn icon_calendar(painter: &Painter, center: Pos2, size: f32, color: Color32) {
    let s = size;
    let w = stroke_width(s);
    let stroke = Stroke::new(w, color);

    let half = s * 0.42;
    let top = center.y - half;
    let bottom = center.y + half;
    let left = center.x - half;
    let right = center.x + half;

    let rect = egui::Rect::from_min_max(Pos2::new(left, top), Pos2::new(right, bottom));
    painter.rect_stroke(
        rect,
        egui::CornerRadius::same((s * 0.12) as u8),
        stroke,
        egui::StrokeKind::Inside,
    );

    let header_y = top + s * 0.18;
    painter.line_segment(
        [Pos2::new(left, header_y), Pos2::new(right, header_y)],
        stroke,
    );

    let ring_y0 = top - s * 0.02;
    let ring_y1 = top + s * 0.12;
    painter.line_segment(
        [
            Pos2::new(left + s * 0.16, ring_y0),
            Pos2::new(left + s * 0.16, ring_y1),
        ],
        stroke,
    );
    painter.line_segment(
        [
            Pos2::new(right - s * 0.16, ring_y0),
            Pos2::new(right - s * 0.16, ring_y1),
        ],
        stroke,
    );
}
