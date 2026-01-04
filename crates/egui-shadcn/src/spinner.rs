use crate::theme::Theme;
use egui::{Color32, Painter, Rect, Response, Sense, Stroke, Ui, Vec2, vec2};
use std::f32::consts::TAU;
use std::time::Duration;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpinnerVariant {
    RadixLeaf,
    LucideLoaderCircle,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SpinnerSize {
    Size1,
    #[default]
    Size2,
    Size3,
}

impl SpinnerSize {
    pub fn side_px(self) -> f32 {
        match self {
            SpinnerSize::Size1 => 12.0,
            SpinnerSize::Size2 => 16.0,
            SpinnerSize::Size3 => 20.0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpinnerProps {
    pub size: SpinnerSize,
    pub color: Option<Color32>,
    pub loading: bool,
    pub opacity: f32,
    pub duration_ms: f32,
    pub variant: SpinnerVariant,
}

impl Default for SpinnerProps {
    fn default() -> Self {
        Self {
            size: SpinnerSize::Size2,
            color: None,
            loading: true,
            opacity: 0.65,
            duration_ms: 800.0,
            variant: SpinnerVariant::RadixLeaf,
        }
    }
}

impl SpinnerProps {
    pub fn with_size(mut self, size: SpinnerSize) -> Self {
        self.size = size;
        self
    }

    pub fn with_color(mut self, color: Color32) -> Self {
        self.color = Some(color);
        self
    }

    pub fn with_loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    pub fn with_opacity(mut self, opacity: f32) -> Self {
        self.opacity = opacity;
        self
    }

    pub fn with_duration_ms(mut self, duration_ms: f32) -> Self {
        self.duration_ms = duration_ms;
        self
    }

    pub fn with_variant(mut self, variant: SpinnerVariant) -> Self {
        self.variant = variant;
        self
    }
}

pub fn spinner(ui: &mut Ui, theme: &Theme, props: SpinnerProps) -> Response {
    if !props.loading {
        return ui.allocate_response(Vec2::ZERO, Sense::hover());
    }

    let side = props.size.side_px();
    let (rect, response) = ui.allocate_at_least(vec2(side, side), Sense::hover());
    let painter = ui.painter_at(rect);
    paint_spinner(ui, &painter, rect, &props, theme);
    response
}

pub fn spinner_with_content<R>(
    ui: &mut Ui,
    theme: &Theme,
    props: SpinnerProps,
    content: impl FnOnce(&mut Ui) -> R,
) -> (R, Response) {
    let inner = ui.scope(content);
    if !props.loading {
        return (inner.inner, inner.response);
    }

    let overlay_rect = inner.response.rect;
    let painter = ui.painter_at(overlay_rect);

    // Hide underlying content to match Radix Themes `visibility: hidden` overlay behavior.
    painter.rect_filled(overlay_rect, 0.0, ui.visuals().panel_fill);

    let spinner_rect = Rect::from_center_size(
        overlay_rect.center(),
        vec2(props.size.side_px(), props.size.side_px()),
    );
    paint_spinner(ui, &painter, spinner_rect, &props, theme);
    (inner.inner, inner.response)
}

fn paint_spinner(ui: &mut Ui, painter: &Painter, rect: Rect, props: &SpinnerProps, theme: &Theme) {
    match props.variant {
        SpinnerVariant::RadixLeaf => paint_radix_leaf(ui, painter, rect, props, theme),
        SpinnerVariant::LucideLoaderCircle => {
            paint_lucide_loader_circle(ui, painter, rect, props, theme)
        }
    }
}

fn base_color_with_opacity(props: &SpinnerProps, theme: &Theme) -> Color32 {
    let base = props.color.unwrap_or(theme.palette.foreground);
    let alpha = (base.a() as f32 * props.opacity).clamp(0.0, 255.0) as u8;
    Color32::from_rgba_unmultiplied(base.r(), base.g(), base.b(), alpha)
}

fn paint_radix_leaf(
    ui: &mut Ui,
    painter: &Painter,
    rect: Rect,
    props: &SpinnerProps,
    theme: &Theme,
) {
    let base_color = base_color_with_opacity(props, theme);
    let side = rect.width().min(rect.height());
    let radius = side * 0.5;
    let duration = (props.duration_ms / 1000.0).max(0.001);
    let time = ui.input(|i| i.time as f32);

    let leaf_len = radius * 0.6;
    let leaf_thickness = (side * 0.125).clamp(1.0, 4.0);

    for i in 0..8 {
        let phase = i as f32 / 8.0;
        let cycle = (time / duration - phase).rem_euclid(1.0);
        let opacity = 1.0 - 0.75 * cycle;
        let color = Color32::from_rgba_unmultiplied(
            base_color.r(),
            base_color.g(),
            base_color.b(),
            (base_color.a() as f32 * opacity).clamp(0.0, 255.0) as u8,
        );

        let angle = TAU * phase;
        let dir = Vec2::new(angle.cos(), angle.sin());
        let start = rect.center() + dir * (radius - leaf_len);
        let end = rect.center() + dir * (radius - leaf_len * 0.15);

        painter.line_segment([start, end], Stroke::new(leaf_thickness, color));
    }

    ui.ctx().request_repaint_after(Duration::from_millis(16));
}

fn paint_lucide_loader_circle(
    ui: &mut Ui,
    painter: &Painter,
    rect: Rect,
    props: &SpinnerProps,
    theme: &Theme,
) {
    let color = base_color_with_opacity(props, theme);
    let side = rect.width().min(rect.height());
    let radius = side * 0.5;
    let stroke_width = (side * 0.12).clamp(1.5, 3.0);
    let duration = (props.duration_ms / 1000.0).max(0.001);
    let time = ui.input(|i| i.time as f32);
    let offset = (time / duration) * TAU;

    // Lucide loader-circle approximated as a rotating 80% arc with a gap.
    let coverage = 0.8;
    let segment_count = 48;
    let start_angle = offset;
    let end_angle = offset + coverage * TAU;

    let mut points = Vec::with_capacity(segment_count + 1);
    for i in 0..=segment_count {
        let t = i as f32 / segment_count as f32;
        let angle = start_angle + (end_angle - start_angle) * t;
        let dir = Vec2::new(angle.cos(), angle.sin());
        points.push(rect.center() + dir * (radius - stroke_width));
    }

    painter.add(egui::Shape::line(points, Stroke::new(stroke_width, color)));
    ui.ctx().request_repaint_after(Duration::from_millis(16));
}