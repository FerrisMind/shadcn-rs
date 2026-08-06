//! Canvas rendering and animation for [`super::Spinner`].

use std::f32::consts::TAU;
use std::time::Duration;

use crate::iced_compat::alignment::Vertical;
use crate::iced_compat::widget::canvas;
use crate::iced_compat::widget::canvas::{LineCap, LineJoin, Path, Stroke, Text};
use crate::iced_compat::window;
use crate::iced_compat::{Color, Font, Point, Radians, Rectangle, Renderer, Size, Vector};

use super::types::{Spinner, SpinnerState, SpinnerVariant};

pub(super) const AI_LOADER_VIEWBOX: f32 = 16.0;
const AI_LOADER_STROKE: f32 = 1.5;
type AiLoaderSegment = ((f32, f32), (f32, f32), f32);
pub(super) const AI_LOADER_SEGMENTS: [AiLoaderSegment; 10] = [
    ((8.0, 0.0), (8.0, 4.0), 1.0),
    ((8.0, 16.0), (8.0, 12.0), 0.5),
    ((3.29773, 1.52783), (5.64887, 4.7639), 0.9),
    ((12.7023, 1.52783), (10.3511, 4.7639), 0.1),
    ((12.7023, 14.472), (10.3511, 11.236), 0.4),
    ((3.29773, 14.472), (5.64887, 11.236), 0.6),
    ((15.6085, 5.52783), (11.8043, 6.7639), 0.2),
    ((0.391602, 10.472), (4.19583, 9.23598), 0.7),
    ((15.6085, 10.4722), (11.8043, 9.2361), 0.3),
    ((0.391602, 5.52783), (4.19583, 6.7639), 0.8),
];
const SPINNER_FRAME_INTERVAL: Duration = Duration::from_millis(33);

impl<Message> canvas::Program<Message> for Spinner {
    type State = SpinnerState;

    fn update(
        &self,
        state: &mut Self::State,
        event: &canvas::Event,
        _bounds: Rectangle,
        _cursor: crate::iced_compat::mouse::Cursor,
    ) -> Option<canvas::Action<Message>> {
        if !self.loading || !self.animated {
            return None;
        }

        if let canvas::Event::Window(window::Event::RedrawRequested(now)) = event {
            if state.start_time.is_none() {
                state.start_time = Some(*now);
            }

            if let Some(start) = state.start_time {
                let elapsed = now.saturating_duration_since(start);
                state.phase = (elapsed.as_secs_f32() / self.duration.as_secs_f32()) % 1.0;
            }

            return Some(canvas::Action::request_redraw_at(
                *now + SPINNER_FRAME_INTERVAL,
            ));
        }

        None
    }

    fn draw(
        &self,
        state: &Self::State,
        renderer: &Renderer,
        _theme: &crate::iced_compat::Theme,
        bounds: Rectangle,
        _cursor: crate::iced_compat::mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        if !self.loading {
            return Vec::new();
        }

        let mut frame = canvas::Frame::new(renderer, bounds.size());
        let center = frame.center();
        let size = self.size.pixels();
        let phase = self.resolved_progress(state).rem_euclid(1.0);

        match self.variant {
            SpinnerVariant::LegacyLucide | SpinnerVariant::AiLoaderIcon => {
                let rotation = phase * TAU;
                frame.with_save(|frame| {
                    frame.translate(Vector::new(center.x, center.y));
                    frame.rotate(rotation);
                    frame.translate(Vector::new(-center.x, -center.y));

                    match self.variant {
                        SpinnerVariant::LegacyLucide => {
                            draw_legacy_spinner_icon(frame, center, size, self.color);
                        }
                        SpinnerVariant::AiLoaderIcon => {
                            draw_ai_loader_icon(frame, center, size, self.color);
                        }
                        _ => {}
                    }
                });
            }
            SpinnerVariant::Circular => draw_circular(&mut frame, center, size, self.color, phase),
            SpinnerVariant::Classic => draw_classic(&mut frame, center, size, self.color, phase),
            SpinnerVariant::Pulse => draw_pulse(&mut frame, center, size, self.color, phase),
            SpinnerVariant::PulseDot => draw_pulse_dot(&mut frame, center, size, self.color, phase),
            SpinnerVariant::Dots => draw_dots(&mut frame, center, size, self.color, phase, false),
            SpinnerVariant::Typing => draw_dots(&mut frame, center, size, self.color, phase, true),
            SpinnerVariant::Wave => draw_wave(
                &mut frame,
                center,
                size,
                self.color,
                phase,
                5,
                self.amplitudes,
            ),
            SpinnerVariant::Bars => draw_wave(
                &mut frame,
                center,
                size,
                self.color,
                phase,
                3,
                self.amplitudes,
            ),
            SpinnerVariant::Terminal => draw_terminal(&mut frame, bounds, size, self.color, phase),
            SpinnerVariant::TextBlink => draw_text(
                &mut frame,
                bounds,
                size,
                self.color,
                "Thinking",
                phase,
                TextAnimation::Blink,
            ),
            SpinnerVariant::TextShimmer => draw_text(
                &mut frame,
                bounds,
                size,
                self.color,
                "Thinking",
                phase,
                TextAnimation::Shimmer,
            ),
            SpinnerVariant::LoadingDots => draw_text(
                &mut frame,
                bounds,
                size,
                self.color,
                "Loading",
                phase,
                TextAnimation::LoadingDots,
            ),
        }

        vec![frame.into_geometry()]
    }
}

fn draw_legacy_spinner_icon(
    frame: &mut canvas::Frame<Renderer>,
    center: Point,
    size: f32,
    color: Color,
) {
    let inner = size * 0.18;
    let outer = size * 0.44;
    let stroke = (size * 0.10).clamp(1.0, 2.4);

    for i in 0..8 {
        let angle = (i as f32 / 8.0) * TAU - TAU / 4.0;
        let dir = Vector::new(angle.cos(), angle.sin());
        let start = Point::new(center.x + dir.x * inner, center.y + dir.y * inner);
        let end = Point::new(center.x + dir.x * outer, center.y + dir.y * outer);

        frame.stroke(
            &Path::line(start, end),
            Stroke::default()
                .with_width(stroke)
                .with_line_cap(LineCap::Round)
                .with_line_join(LineJoin::Round)
                .with_color(color),
        );
    }
}

fn draw_circular(
    frame: &mut canvas::Frame<Renderer>,
    center: Point,
    size: f32,
    color: Color,
    phase: f32,
) {
    // ~270° stroke with a ~90° gap — classic “ring with cutout” loader.
    let radius = (size * 0.38).max(2.0);
    let stroke = (size * 0.12).clamp(1.5, 3.0);
    let start = phase * TAU - TAU / 4.0;
    let sweep = TAU * 0.75;

    let arc = Path::new(|builder| {
        builder.arc(canvas::path::Arc {
            center,
            radius,
            start_angle: Radians(start),
            end_angle: Radians(start + sweep),
        });
    });

    frame.stroke(
        &arc,
        Stroke::default()
            .with_width(stroke)
            .with_line_cap(LineCap::Round)
            .with_line_join(LineJoin::Round)
            .with_color(color),
    );
}

fn draw_classic(
    frame: &mut canvas::Frame<Renderer>,
    center: Point,
    size: f32,
    color: Color,
    phase: f32,
) {
    let inner = size * 0.18;
    let outer = size * 0.42;
    let stroke = (size * 0.09).clamp(1.0, 2.6);

    for i in 0..12 {
        let angle = (i as f32 / 12.0) * TAU - TAU / 4.0;
        let dir = Vector::new(angle.cos(), angle.sin());
        let start = Point::new(center.x + dir.x * inner, center.y + dir.y * inner);
        let end = Point::new(center.x + dir.x * outer, center.y + dir.y * outer);
        let local = ((i as f32 / 12.0 - phase).rem_euclid(1.0) * 12.0).floor();
        let alpha = 0.15 + (11.0 - local) / 11.0 * 0.85;

        frame.stroke(
            &Path::line(start, end),
            Stroke::default()
                .with_width(stroke)
                .with_line_join(LineJoin::Round)
                .with_color(apply_opacity(color, alpha)),
        );
    }
}

fn draw_pulse(
    frame: &mut canvas::Frame<Renderer>,
    center: Point,
    size: f32,
    color: Color,
    phase: f32,
) {
    let pulse = ((phase * TAU).sin() * 0.5 + 0.5).clamp(0.0, 1.0);
    let radius = size * (0.28 + pulse * 0.08);
    let stroke = (size * 0.11).clamp(1.0, 3.0);
    let circle = Path::circle(center, radius);
    frame.stroke(
        &circle,
        Stroke::default()
            .with_width(stroke)
            .with_color(apply_opacity(color, 0.45 + pulse * 0.55)),
    );
}

fn draw_pulse_dot(
    frame: &mut canvas::Frame<Renderer>,
    center: Point,
    size: f32,
    color: Color,
    phase: f32,
) {
    let pulse = ((phase * TAU).sin() * 0.5 + 0.5).clamp(0.0, 1.0);
    let radius = (size * (0.10 + pulse * 0.08)).max(1.0);
    frame.fill(
        &Path::circle(center, radius),
        apply_opacity(color, 0.5 + pulse * 0.5),
    );
}

fn draw_dots(
    frame: &mut canvas::Frame<Renderer>,
    center: Point,
    size: f32,
    color: Color,
    phase: f32,
    typing: bool,
) {
    let gap = size * 0.22;
    let radius = size * if typing { 0.08 } else { 0.12 };
    let amp = size * if typing { 0.08 } else { 0.14 };
    let speed = if typing { 1.0 } else { 1.4 };

    for i in 0..3 {
        let t = (phase * speed + i as f32 * 0.18).rem_euclid(1.0);
        let y = center.y - ((t * TAU).sin() * 0.5 + 0.5) * amp;
        let alpha = 0.35 + ((t * TAU).sin() * 0.5 + 0.5) * 0.65;
        let x = center.x + (i as f32 - 1.0) * gap;
        frame.fill(
            &Path::circle(Point::new(x, y), radius.max(1.0)),
            apply_opacity(color, alpha),
        );
    }
}

fn draw_wave(
    frame: &mut canvas::Frame<Renderer>,
    center: Point,
    size: f32,
    color: Color,
    phase: f32,
    bars: usize,
    amplitudes: Option<[f32; 5]>,
) {
    let width = if bars == 3 { size * 0.16 } else { size * 0.09 };
    let spacing = if bars == 3 { size * 0.22 } else { size * 0.14 };

    for i in 0..bars {
        let x = center.x + (i as f32 - (bars as f32 - 1.0) / 2.0) * spacing;

        let (wave, base, amp) = if let Some(amps) = amplitudes {
            // ── Real amplitude mode ──────────────────────────────────────────
            // Each bar = its own audio RMS window (0 = silence, 1 = loud).
            // Tiny base so bars nearly collapse in silence — maximum visual range.
            (amps[i.min(4)], size * 0.08, size * 0.44)
        } else {
            // ── Sine animation fallback (no mic data) ────────────────────────
            let s = ((phase * TAU) + i as f32 * 0.7).sin() * 0.5 + 0.5;
            (s, size * 0.22, size * 0.34)
        };

        let h = (base + wave * amp).max(1.0);
        let top = center.y - h / 2.0;
        frame.fill_rectangle(
            Point::new(x - width / 2.0, top),
            Size::new(width.max(1.0), h),
            apply_opacity(color, 0.4 + wave * 0.6),
        );
    }
}

fn draw_terminal(
    frame: &mut canvas::Frame<Renderer>,
    bounds: Rectangle,
    size: f32,
    color: Color,
    phase: f32,
) {
    let y = bounds.height / 2.0;
    let font_size = (size * 0.68).max(9.0);
    let cursor_h = (size * 0.62).max(8.0);
    let cursor_w = (size * 0.24).max(2.0);
    let blink = if (phase * 2.0).fract() > 0.5 {
        1.0
    } else {
        0.2
    };

    frame.fill_text(Text {
        content: ">".into(),
        position: Point::new(bounds.width * 0.28, y),
        color,
        size: font_size.into(),
        font: Font::MONOSPACE,
        align_x: crate::iced_compat::widget::text::Alignment::Center,
        align_y: Vertical::Center,
        ..Text::default()
    });

    frame.fill_rectangle(
        Point::new(bounds.width * 0.45, y - cursor_h / 2.0),
        Size::new(cursor_w, cursor_h),
        apply_opacity(color, blink),
    );
}

#[derive(Clone, Copy)]
enum TextAnimation {
    Blink,
    Shimmer,
    LoadingDots,
}

fn draw_text(
    frame: &mut canvas::Frame<Renderer>,
    bounds: Rectangle,
    size: f32,
    color: Color,
    text: &str,
    phase: f32,
    animation: TextAnimation,
) {
    // `canvas::Text` owns a `String`, so one allocation per frame is
    // unavoidable; reserving for the dot suffix avoids a second one.
    let mut content = String::with_capacity(text.len() + 3);
    content.push_str(text);
    let mut paint = color;

    match animation {
        TextAnimation::Blink => {
            let blink = ((phase * TAU / 2.0).sin() * 0.5 + 0.5).clamp(0.0, 1.0);
            paint = apply_opacity(color, 0.35 + blink * 0.65);
        }
        TextAnimation::Shimmer => {
            let mix = ((phase * TAU).sin() * 0.5 + 0.5).clamp(0.0, 1.0);
            paint = mix_colors(apply_opacity(color, 0.55), color, mix);
        }
        TextAnimation::LoadingDots => {
            // Four equal time slots so the cycle reaches "Loading...":
            // phase ∈ [0, 1) ⇒ floor(phase * 4) ∈ {0, 1, 2, 3}.
            const DOT_FRAMES: [&str; 4] = ["", ".", "..", "..."];
            let dots = ((phase * 4.0).floor() as usize).min(3);
            content.push_str(DOT_FRAMES[dots]);
        }
    }

    frame.fill_text(Text {
        content,
        position: Point::new(bounds.width / 2.0, bounds.height / 2.0),
        color: paint,
        size: (size * 0.54).max(8.0).into(),
        font: Font::DEFAULT,
        align_x: crate::iced_compat::widget::text::Alignment::Center,
        align_y: Vertical::Center,
        ..Text::default()
    });
}

fn mix_colors(start: Color, end: Color, t: f32) -> Color {
    let clamped = t.clamp(0.0, 1.0);
    Color {
        r: start.r + (end.r - start.r) * clamped,
        g: start.g + (end.g - start.g) * clamped,
        b: start.b + (end.b - start.b) * clamped,
        a: start.a + (end.a - start.a) * clamped,
    }
}

fn draw_ai_loader_icon(
    frame: &mut canvas::Frame<Renderer>,
    center: Point,
    size: f32,
    color: Color,
) {
    let scale = size / AI_LOADER_VIEWBOX;
    let top_left = Point::new(center.x - size / 2.0, center.y - size / 2.0);
    let stroke = (AI_LOADER_STROKE * scale).max(0.1);

    for (start, end, alpha) in AI_LOADER_SEGMENTS {
        let from = Point::new(top_left.x + start.0 * scale, top_left.y + start.1 * scale);
        let to = Point::new(top_left.x + end.0 * scale, top_left.y + end.1 * scale);
        let segment = Path::line(from, to);

        frame.stroke(
            &segment,
            Stroke::default()
                .with_width(stroke)
                .with_line_join(LineJoin::Round)
                .with_color(apply_opacity(color, alpha)),
        );
    }
}

fn apply_opacity(color: Color, opacity: f32) -> Color {
    Color {
        a: color.a * opacity,
        ..color
    }
}
