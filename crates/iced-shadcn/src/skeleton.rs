use iced::advanced::layout;
use iced::advanced::renderer;
use iced::advanced::widget::Tree;
use iced::advanced::{Clipboard, Layout, Shell, Widget};
use iced::border::Border;
use iced::window;
use iced::{Background, Color, Element, Event, Length, Rectangle, Shadow, Size};

use crate::theme::Theme;
use crate::tokens::{AccentColor, accent_low};

#[derive(Clone, Copy, Debug)]
pub struct SkeletonProps {
    pub loading: bool,
    pub width: Length,
    pub height: Length,
    pub radius: f32,
    pub duration_ms: u32,
}

impl Default for SkeletonProps {
    fn default() -> Self {
        Self {
            loading: true,
            width: Length::Shrink,
            height: Length::Fixed(12.0),
            radius: 6.0,
            duration_ms: 1000,
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
        self.radius = radius.max(0.0);
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
    last_redraw: Option<std::time::Instant>,
}

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
        _layout: Layout<'_>,
        _cursor: iced::mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        if !self.props.loading {
            return;
        }

        if let Event::Window(window::Event::RedrawRequested(now)) = event {
            let state = tree.state.downcast_mut::<SkeletonState>();
            let last = state.last_redraw.replace(*now);
            let delta = last
                .map(|t| now.saturating_duration_since(t))
                .unwrap_or_else(|| std::time::Duration::from_millis(16));

            let duration = std::time::Duration::from_millis(self.props.duration_ms as u64);
            let dt = (delta.as_secs_f32() / duration.as_secs_f32()).clamp(0.0, 1.0);
            state.phase = (state.phase + dt) % 1.0;
        }

        shell.request_redraw();
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
        let t = (state.phase * 2.0).min(2.0);
        let mix = if t <= 1.0 { t } else { 2.0 - t };
        let background = apply_opacity(base, 0.65 + 0.15 * mix);

        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: self.props.radius.into(),
                },
                shadow: Shadow::default(),
                ..renderer::Quad::default()
            },
            Background::Color(background),
        );
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
