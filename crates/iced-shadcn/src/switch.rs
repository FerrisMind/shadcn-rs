use iced::Background;
use iced::Element;
use iced::Event;
use iced::Length;
use iced::Rectangle;
use iced::Shadow;
use iced::Size;
use iced::Vector;
use iced::advanced::layout;
use iced::advanced::renderer;
use iced::advanced::widget::Tree;
use iced::advanced::{Clipboard, Layout, Shell, Widget};
use iced::border;
use iced::mouse;
use iced::time::{Duration, Instant};
use iced::widget::{button as button_widget, button};
use iced::window;

use crate::theme::Theme;
use crate::tokens::{AccentColor, accent_color, accent_foreground, accent_soft, is_dark};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwitchSize {
    One,
    Two,
    Three,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwitchVariant {
    Classic,
    Surface,
    Soft,
}

#[derive(Clone, Copy, Debug)]
pub struct SwitchProps {
    pub size: SwitchSize,
    pub variant: SwitchVariant,
    pub color: AccentColor,
    pub radius: Option<crate::button::ButtonRadius>,
    pub high_contrast: bool,
    pub disabled: bool,
}

impl Default for SwitchProps {
    fn default() -> Self {
        Self {
            size: SwitchSize::Two,
            variant: SwitchVariant::Surface,
            color: AccentColor::Gray,
            radius: None,
            high_contrast: false,
            disabled: false,
        }
    }
}

impl SwitchProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn size(mut self, size: SwitchSize) -> Self {
        self.size = size;
        self
    }

    pub fn variant(mut self, variant: SwitchVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn color(mut self, color: AccentColor) -> Self {
        self.color = color;
        self
    }

    pub fn radius(mut self, radius: crate::button::ButtonRadius) -> Self {
        self.radius = Some(radius);
        self
    }

    pub fn high_contrast(mut self, high_contrast: bool) -> Self {
        self.high_contrast = high_contrast;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl SwitchSize {
    fn metrics(self) -> SwitchMetrics {
        let scale = match self {
            SwitchSize::One => 0.8,
            SwitchSize::Two => 1.0,
            SwitchSize::Three => 1.2,
        };
        let height = 18.4 * scale;
        let width = 32.0 * scale;
        let thumb = 14.0 * scale;
        let thumb_inset_y = (height - thumb) / 2.0;
        let thumb_inset_x = thumb_inset_y;
        SwitchMetrics {
            height,
            width,
            thumb,
            thumb_inset_y,
            thumb_inset_x,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct SwitchMetrics {
    height: f32,
    width: f32,
    thumb: f32,
    thumb_inset_y: f32,
    thumb_inset_x: f32,
}

#[derive(Clone, Copy, Debug)]
struct SwitchColors {
    track: iced::Color,
    thumb: iced::Color,
}

fn switch_radius(theme: &Theme, props: SwitchProps) -> f32 {
    let height = props.size.metrics().height;
    let radius = match props.radius {
        Some(crate::button::ButtonRadius::None) => 0.0,
        Some(crate::button::ButtonRadius::Small) => theme.radius.sm,
        Some(crate::button::ButtonRadius::Medium) => theme.radius.md,
        Some(crate::button::ButtonRadius::Large) => theme.radius.lg,
        Some(crate::button::ButtonRadius::Full) => height / 2.0,
        None => height / 2.0,
    };
    radius.min(height / 2.0)
}

pub fn switch<'a, Message: Clone + 'a, F>(
    is_checked: bool,
    on_toggle: Option<F>,
    props: SwitchProps,
    theme: &Theme,
) -> button_widget::Button<'a, Message>
where
    F: Fn(bool) -> Message + 'a,
{
    let metrics = props.size.metrics();
    let radius = switch_radius(theme, props);
    let thumb_radius = (radius - metrics.thumb_inset_y)
        .max(0.0)
        .min(metrics.thumb / 2.0);
    let dark_mode = is_dark(&theme.palette);
    let disabled = props.disabled || on_toggle.is_none();
    let colors_off = switch_colors(theme, props, false, disabled, dark_mode);
    let colors_on = switch_colors(theme, props, true, disabled, dark_mode);
    let track_shadow = shadow_xs(if disabled { 0.5 } else { 1.0 });

    let content = SwitchVisual {
        is_checked,
        metrics,
        radius,
        thumb_radius,
        colors_off,
        colors_on,
        track_shadow,
    };

    let mut widget = button_widget(content)
        .padding(0)
        .width(Length::Fixed(metrics.width))
        .height(Length::Fixed(metrics.height))
        .style(|_theme, _status| button::Style {
            background: None,
            text_color: iced::Color::TRANSPARENT,
            border: border::Border::default(),
            shadow: Shadow::default(),
            snap: false,
        });

    if !disabled && let Some(on_toggle) = on_toggle {
        widget = widget.on_press(on_toggle(!is_checked));
    }

    widget
}

fn switch_colors(
    theme: &Theme,
    props: SwitchProps,
    is_checked: bool,
    disabled: bool,
    dark_mode: bool,
) -> SwitchColors {
    let palette = theme.palette;
    let accent = accent_color(&palette, props.color);
    let accent_fg = accent_foreground(&palette, props.color);
    let soft_bg = accent_soft(&palette, props.color);

    let mut track_unchecked = palette.input;
    if dark_mode {
        track_unchecked = apply_opacity(track_unchecked, 0.8);
    }
    let checked_track = match props.variant {
        SwitchVariant::Soft => soft_bg,
        SwitchVariant::Classic | SwitchVariant::Surface => accent,
    };
    let mut track = if is_checked {
        checked_track
    } else {
        track_unchecked
    };

    let mut thumb = if dark_mode {
        if is_checked {
            accent_fg
        } else {
            palette.foreground
        }
    } else {
        palette.background
    };

    if is_checked && props.high_contrast {
        track = palette.foreground;
        thumb = palette.background;
    }

    if disabled {
        track = apply_opacity(track, 0.5);
        thumb = apply_opacity(thumb, 0.5);
    }

    SwitchColors { track, thumb }
}

fn apply_opacity(color: iced::Color, opacity: f32) -> iced::Color {
    iced::Color {
        a: (color.a * opacity).clamp(0.0, 1.0),
        ..color
    }
}

fn shadow_xs(opacity: f32) -> Shadow {
    Shadow {
        color: apply_opacity(iced::Color::BLACK, 0.05 * opacity),
        offset: Vector::new(0.0, 1.0),
        blur_radius: 2.0,
    }
}

const SWITCH_ANIMATION_DURATION: Duration = Duration::from_millis(150);

#[derive(Debug)]
struct SwitchAnimation {
    last_checked: bool,
    progress: f32,
    from: f32,
    to: f32,
    start: Option<Instant>,
    animating: bool,
}

impl SwitchAnimation {
    fn new(is_checked: bool) -> Self {
        let progress = if is_checked { 1.0 } else { 0.0 };
        Self {
            last_checked: is_checked,
            progress,
            from: progress,
            to: progress,
            start: None,
            animating: false,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct SwitchVisual {
    is_checked: bool,
    metrics: SwitchMetrics,
    radius: f32,
    thumb_radius: f32,
    colors_off: SwitchColors,
    colors_on: SwitchColors,
    track_shadow: Shadow,
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for SwitchVisual
where
    Renderer: renderer::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size::new(
            Length::Fixed(self.metrics.width),
            Length::Fixed(self.metrics.height),
        )
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout::atomic(
            limits,
            Length::Fixed(self.metrics.width),
            Length::Fixed(self.metrics.height),
        )
    }

    fn tag(&self) -> iced::advanced::widget::tree::Tag {
        iced::advanced::widget::tree::Tag::of::<SwitchAnimation>()
    }

    fn state(&self) -> iced::advanced::widget::tree::State {
        iced::advanced::widget::tree::State::new(SwitchAnimation::new(self.is_checked))
    }

    fn diff(&self, tree: &mut Tree) {
        let state = tree.state.downcast_mut::<SwitchAnimation>();
        if state.last_checked != self.is_checked {
            state.last_checked = self.is_checked;
            state.from = state.progress;
            state.to = if self.is_checked { 1.0 } else { 0.0 };
            state.start = None;
            state.animating = true;
        } else if !state.animating {
            state.progress = if self.is_checked { 1.0 } else { 0.0 };
            state.from = state.progress;
            state.to = state.progress;
        }
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        _layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        if let Event::Window(window::Event::RedrawRequested(now)) = event {
            let state = tree.state.downcast_mut::<SwitchAnimation>();
            if state.animating {
                let start = state.start.get_or_insert(*now);
                let elapsed = (*now - *start).as_secs_f32();
                let duration = SWITCH_ANIMATION_DURATION.as_secs_f32().max(0.0001);
                let t = (elapsed / duration).min(1.0);
                let eased = cubic_bezier(t, 0.4, 0.0, 0.2, 1.0);
                state.progress = state.from + (state.to - state.from) * eased;

                if t < 1.0 {
                    shell.request_redraw();
                } else {
                    state.progress = state.to;
                    state.animating = false;
                    state.start = None;
                }
            }
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        _theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        if !bounds.intersects(viewport) {
            return;
        }

        let state = tree.state.downcast_ref::<SwitchAnimation>();
        let progress = state.progress.clamp(0.0, 1.0);
        let track_x = bounds.x;
        let track_y = bounds.y + (bounds.height - self.metrics.height) * 0.5;
        let track_bounds = Rectangle {
            x: track_x,
            y: track_y,
            width: self.metrics.width,
            height: self.metrics.height,
        };
        let track_color = mix_color(self.colors_off.track, self.colors_on.track, progress);
        let thumb_color = mix_color(self.colors_off.thumb, self.colors_on.thumb, progress);
        let available_width =
            (track_bounds.width - (self.metrics.thumb + self.metrics.thumb_inset_x * 2.0)).max(0.0);
        let thumb_x = track_bounds.x + self.metrics.thumb_inset_x + (available_width * progress);
        let thumb_bounds = Rectangle {
            x: thumb_x,
            y: track_bounds.y + self.metrics.thumb_inset_y,
            width: self.metrics.thumb,
            height: self.metrics.thumb,
        };

        renderer.fill_quad(
            renderer::Quad {
                bounds: track_bounds,
                border: border::Border {
                    radius: self.radius.into(),
                    width: 1.0,
                    color: iced::Color::TRANSPARENT,
                },
                shadow: self.track_shadow,
                snap: false,
            },
            Background::Color(track_color),
        );

        renderer.fill_quad(
            renderer::Quad {
                bounds: thumb_bounds,
                border: border::Border {
                    radius: self.thumb_radius.into(),
                    width: 0.0,
                    color: iced::Color::TRANSPARENT,
                },
                shadow: Shadow::default(),
                snap: false,
            },
            Background::Color(thumb_color),
        );
    }
}

impl<'a, Message, Theme, Renderer> From<SwitchVisual> for Element<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer + 'a,
    Theme: 'a,
{
    fn from(value: SwitchVisual) -> Self {
        Self::new(value)
    }
}

fn mix_color(from: iced::Color, to: iced::Color, t: f32) -> iced::Color {
    let t = t.clamp(0.0, 1.0);
    let lerp = |a: f32, b: f32| (a * (1.0 - t)) + (b * t);
    iced::Color {
        r: lerp(from.r, to.r),
        g: lerp(from.g, to.g),
        b: lerp(from.b, to.b),
        a: lerp(from.a, to.a),
    }
}

fn cubic_bezier(t: f32, x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    if t <= 0.0 {
        return 0.0;
    }
    if t >= 1.0 {
        return 1.0;
    }

    let sample = |t: f32, p1: f32, p2: f32| {
        let c = 3.0 * p1;
        let b = 3.0 * (p2 - p1) - c;
        let a = 1.0 - c - b;
        ((a * t + b) * t + c) * t
    };
    let derivative = |t: f32, p1: f32, p2: f32| {
        let c = 3.0 * p1;
        let b = 3.0 * (p2 - p1) - c;
        let a = 1.0 - c - b;
        (3.0 * a * t * t) + (2.0 * b * t) + c
    };

    let mut u = t;
    for _ in 0..6 {
        let x = sample(u, x1, x2);
        let dx = derivative(u, x1, x2);
        if dx.abs() < 1e-6 {
            break;
        }
        u = (u - (x - t) / dx).clamp(0.0, 1.0);
    }

    sample(u, y1, y2)
}
