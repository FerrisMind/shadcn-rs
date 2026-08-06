use std::f32::consts::TAU;
use std::time::Duration;

use iced::alignment::Vertical;
use iced::widget::canvas;
use iced::widget::canvas::{LineCap, LineJoin, Path, Stroke, Text};
use iced::window;
use iced::{Color, Font, Length, Point, Rectangle, Renderer, Size, Vector};

use crate::profiling::profile_span;
use crate::theme::Theme;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SpinnerVariant {
    #[default]
    LegacyLucide,
    AiLoaderIcon,
    PromptCircular,
    PromptClassic,
    PromptPulse,
    PromptPulseDot,
    PromptDots,
    PromptTyping,
    PromptWave,
    PromptBars,
    PromptTerminal,
    PromptTextBlink,
    PromptTextShimmer,
    PromptLoadingDots,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SpinnerSize {
    Size1,
    Size2,
    Size3,
    Custom(f32),
}

impl SpinnerSize {
    fn pixels(self) -> f32 {
        match self {
            SpinnerSize::Size1 => 12.0,
            SpinnerSize::Size2 => 16.0,
            SpinnerSize::Size3 => 20.0,
            SpinnerSize::Custom(value) => value.max(1.0),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Spinner {
    progress: f32,
    color: Color,
    size: SpinnerSize,
    loading: bool,
    animated: bool,
    duration_ms: u32,
    variant: SpinnerVariant,
    /// Per-bar amplitude values for Wave/Bars variants (0.0–1.0 each).
    /// When `Some`, overrides the sine-wave animation with real audio levels.
    /// When `None`, falls back to phase-driven sine animation.
    amplitudes: Option<[f32; 5]>,
}

impl Spinner {
    pub fn new(theme: &Theme) -> Self {
        Self::from_color(theme.palette.primary)
    }

    /// Spinner with an explicit color (for `new_api` themes that are not crate [`Theme`]).
    pub fn from_color(color: Color) -> Self {
        Self {
            progress: 0.0,
            color,
            size: SpinnerSize::Size2,
            loading: true,
            animated: false,
            duration_ms: 1000,
            variant: SpinnerVariant::AiLoaderIcon,
            amplitudes: None,
        }
    }

    pub fn progress(mut self, progress: f32) -> Self {
        self.progress = progress;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn size(mut self, size: SpinnerSize) -> Self {
        self.size = size;
        self
    }

    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    pub fn animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }

    pub fn duration_ms(mut self, duration_ms: u32) -> Self {
        self.duration_ms = duration_ms.max(1);
        self
    }

    pub fn variant(mut self, variant: SpinnerVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set per-bar amplitude values for Wave/Bars variants.
    ///
    /// Each value in `[f32; 5]` is clamped to `[0.0, 1.0]` and maps to one bar.
    /// When set, real audio amplitudes are used instead of the time-driven sine wave.
    /// For the `PromptBars` variant (3 bars) only the first 3 values are used.
    pub fn amplitudes(mut self, amps: [f32; 5]) -> Self {
        self.amplitudes = Some(amps.map(|a| a.clamp(0.0, 1.0)));
        self
    }

    fn resolved_progress(self, state: &SpinnerState) -> f32 {
        if self.animated {
            state.phase
        } else {
            self.progress
        }
    }

    fn dimensions(self) -> Size {
        let size = self.size.pixels();
        match self.variant {
            SpinnerVariant::PromptTerminal => Size::new(size * 2.4, size),
            SpinnerVariant::PromptTextBlink
            | SpinnerVariant::PromptTextShimmer
            | SpinnerVariant::PromptLoadingDots => Size::new(size * 4.8, size * 1.2),
            _ => Size::new(size, size),
        }
    }
}

pub fn spinner<Message>(spinner: Spinner) -> canvas::Canvas<Spinner, Message> {
    let size = spinner.dimensions();
    canvas::Canvas::new(spinner)
        .width(Length::Fixed(size.width))
        .height(Length::Fixed(size.height))
}

#[derive(Debug, Default)]
pub struct SpinnerState {
    start_time: Option<iced::time::Instant>,
    phase: f32,
}

const AI_LOADER_VIEWBOX: f32 = 16.0;
const AI_LOADER_STROKE: f32 = 1.5;
type AiLoaderSegment = ((f32, f32), (f32, f32), f32);
const AI_LOADER_SEGMENTS: [AiLoaderSegment; 10] = [
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
        _cursor: iced::mouse::Cursor,
    ) -> Option<canvas::Action<Message>> {
        let _profile = profile_span("spinner.update");

        if !self.loading || !self.animated {
            return None;
        }

        if let canvas::Event::Window(window::Event::RedrawRequested(now)) = event {
            if state.start_time.is_none() {
                state.start_time = Some(*now);
            }

            if let Some(start) = state.start_time {
                let elapsed = now.saturating_duration_since(start);
                let duration = Duration::from_millis(self.duration_ms as u64);
                state.phase = (elapsed.as_secs_f32() / duration.as_secs_f32()) % 1.0;
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
        _theme: &iced::Theme,
        bounds: Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let _profile = profile_span("spinner.draw");

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
            SpinnerVariant::PromptCircular => {
                draw_prompt_circular(&mut frame, center, size, self.color, phase)
            }
            SpinnerVariant::PromptClassic => {
                draw_prompt_classic(&mut frame, center, size, self.color, phase)
            }
            SpinnerVariant::PromptPulse => {
                draw_prompt_pulse(&mut frame, center, size, self.color, phase)
            }
            SpinnerVariant::PromptPulseDot => {
                draw_prompt_pulse_dot(&mut frame, center, size, self.color, phase)
            }
            SpinnerVariant::PromptDots => {
                draw_prompt_dots(&mut frame, center, size, self.color, phase, false)
            }
            SpinnerVariant::PromptTyping => {
                draw_prompt_dots(&mut frame, center, size, self.color, phase, true)
            }
            SpinnerVariant::PromptWave => draw_prompt_wave(
                &mut frame,
                center,
                size,
                self.color,
                phase,
                5,
                self.amplitudes,
            ),
            SpinnerVariant::PromptBars => draw_prompt_wave(
                &mut frame,
                center,
                size,
                self.color,
                phase,
                3,
                self.amplitudes,
            ),
            SpinnerVariant::PromptTerminal => {
                draw_prompt_terminal(&mut frame, bounds, size, self.color, phase)
            }
            SpinnerVariant::PromptTextBlink => draw_prompt_text(
                &mut frame,
                bounds,
                size,
                self.color,
                "Thinking",
                phase,
                TextAnimation::Blink,
            ),
            SpinnerVariant::PromptTextShimmer => draw_prompt_text(
                &mut frame,
                bounds,
                size,
                self.color,
                "Thinking",
                phase,
                TextAnimation::Shimmer,
            ),
            SpinnerVariant::PromptLoadingDots => draw_prompt_text(
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

fn draw_prompt_circular(
    frame: &mut canvas::Frame<Renderer>,
    center: Point,
    size: f32,
    color: Color,
    phase: f32,
) {
    let radius = (size * 0.1).clamp(1.5, 2.4);
    let distance_from_center = size * 0.5 - radius;
    let (y, x) = (phase * TAU).sin_cos();
    let position = Point::new(
        center.x + x * distance_from_center,
        center.y + y * distance_from_center,
    );

    frame.fill(&Path::circle(position, radius), color);
}

fn draw_prompt_classic(
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

fn draw_prompt_pulse(
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

fn draw_prompt_pulse_dot(
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

fn draw_prompt_dots(
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

fn draw_prompt_wave(
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

fn draw_prompt_terminal(
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
        align_x: iced::widget::text::Alignment::Center,
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

fn draw_prompt_text(
    frame: &mut canvas::Frame<Renderer>,
    bounds: Rectangle,
    size: f32,
    color: Color,
    text: &str,
    phase: f32,
    animation: TextAnimation,
) {
    let mut content = text.to_string();
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
            let dots = ((phase * 3.0).floor() as i32).clamp(0, 3) as usize;
            content.push_str(&".".repeat(dots));
        }
    }

    frame.fill_text(Text {
        content,
        position: Point::new(bounds.width / 2.0, bounds.height / 2.0),
        color: paint,
        size: (size * 0.54).max(8.0).into(),
        font: Font::DEFAULT,
        align_x: iced::widget::text::Alignment::Center,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spinner_defaults_to_ai_loader_without_internal_animation() {
        let theme = Theme::default();
        let spinner = Spinner::new(&theme);
        assert_eq!(spinner.variant, SpinnerVariant::AiLoaderIcon);
        assert!(!spinner.animated);
        assert_eq!(spinner.duration_ms, 1000);
    }

    #[test]
    fn spinner_progress_compatibility_uses_external_progress_when_not_animated() {
        let theme = Theme::default();
        let spinner = Spinner::new(&theme).progress(0.37);
        let state = SpinnerState::default();
        assert!((spinner.resolved_progress(&state) - 0.37).abs() < f32::EPSILON);
    }

    #[test]
    fn ai_loader_segments_match_reference_contract() {
        assert_eq!(AI_LOADER_SEGMENTS.len(), 10);
        let expected_alpha = [1.0, 0.5, 0.9, 0.1, 0.4, 0.6, 0.2, 0.7, 0.3, 0.8];
        for (index, (_, _, alpha)) in AI_LOADER_SEGMENTS.iter().copied().enumerate() {
            assert!((alpha - expected_alpha[index]).abs() < 1e-6);
        }
    }

    #[test]
    fn ai_loader_segment_scaling_stays_within_bounds() {
        let size = 16.0;
        let scale = size / AI_LOADER_VIEWBOX;
        for (start, end, _) in AI_LOADER_SEGMENTS {
            for (x, y) in [start, end] {
                let sx = x * scale;
                let sy = y * scale;
                assert!((0.0..=size).contains(&sx));
                assert!((0.0..=size).contains(&sy));
            }
        }
    }
}
