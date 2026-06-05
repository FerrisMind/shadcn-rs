use iced::advanced::layout;
use iced::advanced::renderer;
use iced::advanced::text as advanced_text;
use iced::advanced::widget::Tree;
use iced::advanced::{Clipboard, Layout, Shell, Widget};
use iced::border::Border;
use iced::gradient;
use iced::window;
use iced::{Background, Color, Element, Event, Font, Length, Point, Rectangle, Shadow, Size};
use std::sync::OnceLock;
use std::time::Duration;

use crate::profiling::profile_span;
use crate::theme::Theme;
use crate::tokens::{AccentColor, accent_low, mix};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SkeletonAnimation {
    #[default]
    Pulse,
    Shimmer,
}

#[derive(Clone, Copy, Debug)]
pub struct SkeletonProps {
    pub loading: bool,
    pub width: Length,
    pub height: Length,
    pub radius: Option<f32>,
    pub circle: bool,
    pub animation: SkeletonAnimation,
    pub spread: f32,
    pub content_length: f32,
    pub duration_ms: u32,
}

impl Default for SkeletonProps {
    fn default() -> Self {
        Self {
            loading: true,
            width: Length::Fill,
            height: Length::Fixed(12.0),
            radius: None,
            circle: false,
            animation: SkeletonAnimation::Pulse,
            spread: 2.0,
            content_length: 30.0,
            duration_ms: 2000,
        }
    }
}

impl SkeletonProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    pub fn radius(mut self, radius: f32) -> Self {
        self.radius = Some(radius);
        self
    }

    pub fn circle(mut self, circle: bool) -> Self {
        self.circle = circle;
        self
    }

    pub fn animation(mut self, animation: SkeletonAnimation) -> Self {
        self.animation = animation;
        self
    }

    pub fn shimmer(mut self, shimmer: bool) -> Self {
        self.animation = if shimmer {
            SkeletonAnimation::Shimmer
        } else {
            SkeletonAnimation::Pulse
        };
        self
    }

    pub fn spread(mut self, spread: f32) -> Self {
        self.spread = spread.max(0.1);
        self
    }

    pub fn content_length(mut self, content_length: f32) -> Self {
        self.content_length = content_length.max(1.0);
        self
    }

    pub fn duration_ms(mut self, duration_ms: u32) -> Self {
        self.duration_ms = duration_ms.max(1);
        self
    }
}

fn apply_opacity(color: Color, opacity: f32) -> Color {
    Color {
        a: color.a * opacity,
        ..color
    }
}

#[derive(Debug, Default)]
struct SkeletonState {
    phase: f32,
}

#[derive(Debug, Default)]
struct SkeletonShimmerLabelState {
    phase: f32,
}

fn shared_animation_start(now: iced::time::Instant) -> iced::time::Instant {
    static START: OnceLock<iced::time::Instant> = OnceLock::new();
    *START.get_or_init(|| now)
}

fn compute_phase(now: iced::time::Instant, duration_ms: u32) -> f32 {
    let start = shared_animation_start(now);
    let elapsed = now.saturating_duration_since(start);
    let duration = iced::time::Duration::from_millis(duration_ms.max(1) as u64);
    (elapsed.as_secs_f32() / duration.as_secs_f32()) % 1.0
}

const SKELETON_FRAME_INTERVAL: Duration = Duration::from_millis(33);

pub fn skeleton(props: SkeletonProps, theme: &Theme) -> SkeletonWidget {
    SkeletonWidget::new(props, theme)
}

pub struct SkeletonWidget {
    props: SkeletonProps,
    theme: Theme,
}

impl SkeletonWidget {
    fn new(props: SkeletonProps, theme: &Theme) -> Self {
        Self {
            props,
            theme: theme.clone(),
        }
    }
}

impl<Message, AppTheme, Renderer> Widget<Message, AppTheme, Renderer> for SkeletonWidget
where
    Renderer: renderer::Renderer,
{
    fn tag(&self) -> iced::advanced::widget::tree::Tag {
        iced::advanced::widget::tree::Tag::of::<SkeletonState>()
    }

    fn state(&self) -> iced::advanced::widget::tree::State {
        iced::advanced::widget::tree::State::new(SkeletonState::default())
    }

    fn size(&self) -> Size<Length> {
        Size::new(self.props.width, self.props.height)
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout::atomic(limits, self.props.width, self.props.height)
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        _cursor: iced::mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        let _profile = profile_span("skeleton.update");

        if !self.props.loading {
            return;
        }

        if let Event::Window(window::Event::RedrawRequested(now)) = event {
            let state = tree.state.downcast_mut::<SkeletonState>();
            state.phase = compute_phase(*now, self.props.duration_ms);

            if layout.bounds().intersects(viewport) {
                shell.request_redraw_at(*now + SKELETON_FRAME_INTERVAL);
            }
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        _theme: &AppTheme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: iced::mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let _profile = profile_span("skeleton.draw");

        if !self.props.loading {
            return;
        }

        let bounds = layout.bounds();
        if !bounds.intersects(viewport) {
            return;
        }

        let palette = self.theme.palette;
        let base = accent_low(&palette, AccentColor::Gray);

        let state = tree.state.downcast_ref::<SkeletonState>();

        let radius = if self.props.circle {
            (bounds.width.min(bounds.height) / 2.0).into()
        } else {
            self.props.radius.unwrap_or(self.theme.radius.sm).into()
        };

        let fill_layer = |renderer: &mut Renderer, color: Color, layer_bounds: Rectangle| {
            renderer.fill_quad(
                renderer::Quad {
                    bounds: layer_bounds,
                    border: Border {
                        color: Color::TRANSPARENT,
                        width: 0.0,
                        radius,
                    },
                    shadow: Shadow::default(),
                    ..renderer::Quad::default()
                },
                Background::Color(color),
            );
        };

        match self.props.animation {
            SkeletonAnimation::Pulse => {
                // Pulse curve: 1.0 -> 0.5 -> 1.0
                let pulse_mix = (state.phase * std::f32::consts::PI * 2.0).cos() * 0.25 + 0.75;
                fill_layer(renderer, apply_opacity(base, pulse_mix), bounds);
            }
            SkeletonAnimation::Shimmer => {
                // Text-like shimmer for skeleton blocks: darker base + subtle moving highlight.
                let base_color = mix(accent_low(&palette, AccentColor::Gray), palette.muted, 0.35);
                fill_layer(renderer, base_color, bounds);

                if bounds.width <= 0.0 || bounds.height <= 0.0 {
                    return;
                }

                let dynamic_spread_px = (self.props.content_length * self.props.spread).max(1.0);
                let half_width =
                    (dynamic_spread_px * 0.35).clamp(6.0, (bounds.width * 0.22).max(6.0));
                let travel_start = bounds.x - half_width;
                let travel_end = bounds.x + bounds.width + half_width;
                let center_x = travel_start + (travel_end - travel_start) * state.phase;

                let center = (center_x - bounds.x) / bounds.width;
                let spread = half_width / bounds.width;
                let left_edge = center - spread;
                let right_edge = center + spread;

                if right_edge <= 0.0 || left_edge >= 1.0 {
                    return;
                }

                let left = left_edge.clamp(0.0, 1.0);
                let center = center.clamp(0.0, 1.0);
                let right = right_edge.clamp(0.0, 1.0);
                let left_soft = (left + (center - left) * 0.55).clamp(0.0, 1.0);
                let right_soft = (right - (right - center) * 0.55).clamp(0.0, 1.0);
                let highlight_color = mix(base_color, palette.muted_foreground, 0.75);
                let highlight_strong = apply_opacity(highlight_color, 0.85);
                let highlight_soft = apply_opacity(highlight_color, 0.35);

                let shimmer_gradient = gradient::Linear::new(std::f32::consts::FRAC_PI_2)
                    .add_stop(0.0, Color::TRANSPARENT)
                    .add_stop(left, Color::TRANSPARENT)
                    .add_stop(left_soft, highlight_soft)
                    .add_stop(center, highlight_strong)
                    .add_stop(right_soft, highlight_soft)
                    .add_stop(right, Color::TRANSPARENT)
                    .add_stop(1.0, Color::TRANSPARENT);

                renderer.fill_quad(
                    renderer::Quad {
                        bounds,
                        border: Border {
                            color: Color::TRANSPARENT,
                            width: 0.0,
                            radius,
                        },
                        shadow: Shadow::default(),
                        ..renderer::Quad::default()
                    },
                    Background::Gradient(shimmer_gradient.into()),
                );
            }
        }
    }
}

impl<'a, Message, AppTheme, Renderer> From<SkeletonWidget>
    for Element<'a, Message, AppTheme, Renderer>
where
    Renderer: renderer::Renderer + 'a,
    Message: 'a,
{
    fn from(widget: SkeletonWidget) -> Element<'a, Message, AppTheme, Renderer> {
        Element::new(widget)
    }
}

pub struct SkeletonShimmerLabelWidget {
    content: String,
    font_size: f32,
    duration_ms: u32,
    spread: f32,
    content_length: f32,
    theme: Theme,
}

pub fn skeleton_shimmer_label<'a, Message: 'a>(
    content: impl Into<String>,
    font_size: f32,
    duration_ms: u32,
    spread: f32,
    content_length: f32,
    theme: &Theme,
) -> Element<'a, Message> {
    Element::new(SkeletonShimmerLabelWidget {
        content: content.into(),
        font_size,
        duration_ms: duration_ms.max(1),
        spread: spread.max(0.1),
        content_length: content_length.max(1.0),
        theme: theme.clone(),
    })
}

fn intersect_rect(a: Rectangle, b: Rectangle) -> Option<Rectangle> {
    let x1 = a.x.max(b.x);
    let y1 = a.y.max(b.y);
    let x2 = (a.x + a.width).min(b.x + b.width);
    let y2 = (a.y + a.height).min(b.y + b.height);
    let width = x2 - x1;
    let height = y2 - y1;
    if width <= 0.0 || height <= 0.0 {
        None
    } else {
        Some(Rectangle {
            x: x1,
            y: y1,
            width,
            height,
        })
    }
}

impl<Message, AppTheme, Renderer> Widget<Message, AppTheme, Renderer> for SkeletonShimmerLabelWidget
where
    Renderer: renderer::Renderer + advanced_text::Renderer<Font = Font>,
{
    fn tag(&self) -> iced::advanced::widget::tree::Tag {
        iced::advanced::widget::tree::Tag::of::<SkeletonShimmerLabelState>()
    }

    fn state(&self) -> iced::advanced::widget::tree::State {
        iced::advanced::widget::tree::State::new(SkeletonShimmerLabelState::default())
    }

    fn size(&self) -> Size<Length> {
        Size::new(Length::Shrink, Length::Shrink)
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let chars = self.content.chars().count() as f32;
        let width_chars = chars.max(self.content_length);
        let width = (width_chars * self.font_size * 0.58).clamp(24.0, 1400.0);
        let height = (self.font_size * 1.35).clamp(12.0, 120.0);
        layout::atomic(limits, Length::Fixed(width), Length::Fixed(height))
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        _cursor: iced::mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        let _profile = profile_span("skeleton.shimmer_label.update");

        if let Event::Window(window::Event::RedrawRequested(now)) = event {
            let state = tree.state.downcast_mut::<SkeletonShimmerLabelState>();
            state.phase = compute_phase(*now, self.duration_ms);

            if layout.bounds().intersects(viewport) {
                shell.request_redraw_at(*now + SKELETON_FRAME_INTERVAL);
            }
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        _theme: &AppTheme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: iced::mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let _profile = profile_span("skeleton.shimmer_label.draw");

        let bounds = layout.bounds();
        if !bounds.intersects(viewport) {
            return;
        }

        let state = tree.state.downcast_ref::<SkeletonShimmerLabelState>();
        let base_color = self.theme.palette.muted_foreground;
        let highlight_color = mix(base_color, self.theme.palette.foreground, 0.85);
        let text_origin = Point::new(bounds.x, bounds.y + bounds.height * 0.5);

        renderer.fill_text(
            advanced_text::Text {
                content: self.content.clone(),
                bounds: bounds.size(),
                size: self.font_size.into(),
                line_height: advanced_text::LineHeight::Absolute(self.font_size.into()),
                font: Font::DEFAULT,
                align_x: advanced_text::Alignment::Left,
                align_y: iced::alignment::Vertical::Center,
                shaping: advanced_text::Shaping::Advanced,
                wrapping: advanced_text::Wrapping::None,
            },
            text_origin,
            base_color,
            bounds,
        );

        let half_width = (self.content_length * self.spread).max(1.0);
        let center_start = bounds.x - half_width;
        let center_end = bounds.x + bounds.width + half_width;
        let center = center_start + (center_end - center_start) * state.phase;

        let highlight_band = Rectangle {
            x: center - half_width,
            y: bounds.y,
            width: half_width * 2.0,
            height: bounds.height,
        };

        if let Some(clip_band) = intersect_rect(highlight_band, bounds)
            && let Some(clip) = intersect_rect(clip_band, *viewport)
        {
            renderer.fill_text(
                advanced_text::Text {
                    content: self.content.clone(),
                    bounds: bounds.size(),
                    size: self.font_size.into(),
                    line_height: advanced_text::LineHeight::Absolute(self.font_size.into()),
                    font: Font::DEFAULT,
                    align_x: advanced_text::Alignment::Left,
                    align_y: iced::alignment::Vertical::Center,
                    shaping: advanced_text::Shaping::Advanced,
                    wrapping: advanced_text::Wrapping::None,
                },
                text_origin,
                highlight_color,
                clip,
            );
        }
    }
}

/// Helper for text skeleton with multiple lines.
pub fn skeleton_text<'a, Message: 'a>(
    lines: usize,
    line_height: f32,
    theme: &Theme,
) -> Element<'a, Message> {
    let mut col = iced::widget::column![].spacing(8);
    for i in 0..lines {
        let width = if i == lines - 1 {
            Length::FillPortion(6)
        } else {
            Length::Fill
        };
        col = col.push(skeleton(
            SkeletonProps::new().width(width).height(line_height),
            theme,
        ));
    }
    col.into()
}

/// Helper for text-like shimmer skeleton with ai-elements defaults.
pub fn skeleton_shimmer_text<'a, Message: 'a>(
    lines: usize,
    line_height: f32,
    duration_ms: u32,
    spread: f32,
    content_length: f32,
    theme: &Theme,
) -> Element<'a, Message> {
    let mut col = iced::widget::column![].spacing(8);
    let chars_to_width =
        |chars: f32| -> Length { Length::Fixed((chars * 7.2).clamp(48.0, 1200.0)) };
    for i in 0..lines {
        let line_content_length = if i == lines - 1 {
            (content_length * 0.8).max(1.0)
        } else {
            content_length
        };
        col = col.push(skeleton(
            SkeletonProps::new()
                .animation(SkeletonAnimation::Shimmer)
                .width(chars_to_width(line_content_length))
                .height(line_height)
                .duration_ms(duration_ms)
                .spread(spread)
                .content_length(line_content_length),
            theme,
        ));
    }
    col.into()
}
