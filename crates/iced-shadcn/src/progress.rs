use iced::advanced::layout;
use iced::advanced::renderer;
use iced::advanced::widget::Tree;
use iced::advanced::{Clipboard, Layout, Shell, Widget};
use iced::border::Border;
use iced::window;
use iced::{Background, Color, Element, Event, Length, Rectangle, Shadow, Size};

use crate::button::ButtonRadius;
use crate::theme::Theme;
use crate::tokens::{AccentColor, accent_color, accent_high, accent_low, is_dark};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ProgressSize {
    Size1,
    #[default]
    Size2,
    Size3,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ProgressVariant {
    Classic,
    #[default]
    Surface,
    Soft,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ProgressOrientation {
    #[default]
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy, Debug)]
pub struct ProgressProps {
    pub size: ProgressSize,
    pub variant: ProgressVariant,
    pub orientation: ProgressOrientation,
    pub color: AccentColor,
    pub radius: Option<ButtonRadius>,
    pub high_contrast: bool,
    pub duration_ms: u32,
    pub value: Option<f32>,
    pub max: f32,
}

impl Default for ProgressProps {
    fn default() -> Self {
        Self {
            size: ProgressSize::Size2,
            variant: ProgressVariant::Surface,
            orientation: ProgressOrientation::Horizontal,
            color: AccentColor::Gray,
            radius: None,
            high_contrast: false,
            duration_ms: 1200,
            value: None,
            max: 100.0,
        }
    }
}

impl ProgressProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn size(mut self, size: ProgressSize) -> Self {
        self.size = size;
        self
    }

    pub fn variant(mut self, variant: ProgressVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn orientation(mut self, orientation: ProgressOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn color(mut self, color: AccentColor) -> Self {
        self.color = color;
        self
    }

    pub fn radius(mut self, radius: ButtonRadius) -> Self {
        self.radius = Some(radius);
        self
    }

    pub fn high_contrast(mut self, high_contrast: bool) -> Self {
        self.high_contrast = high_contrast;
        self
    }

    pub fn duration_ms(mut self, duration_ms: u32) -> Self {
        self.duration_ms = duration_ms.max(1);
        self
    }

    pub fn value(mut self, value: f32) -> Self {
        self.value = Some(value);
        self
    }

    pub fn indeterminate(mut self) -> Self {
        self.value = None;
        self
    }

    pub fn max(mut self, max: f32) -> Self {
        self.max = max.max(1.0);
        self
    }
}

fn apply_opacity(color: Color, opacity: f32) -> Color {
    Color {
        a: color.a * opacity,
        ..color
    }
}

impl ProgressSize {
    fn height(self) -> f32 {
        match self {
            ProgressSize::Size1 => 4.0,
            ProgressSize::Size2 => 8.0,
            ProgressSize::Size3 => 12.0,
        }
    }
}

fn progress_radius(theme: &Theme, props: ProgressProps) -> f32 {
    let height = props.size.height();
    match props.radius {
        Some(ButtonRadius::None) => 0.0,
        Some(ButtonRadius::Small) => theme.radius.sm,
        Some(ButtonRadius::Medium) => theme.radius.md,
        Some(ButtonRadius::Large) => theme.radius.lg,
        Some(ButtonRadius::Full) => (height / 2.0).max(9999.0),
        None => (height / 3.0).max(theme.radius.sm),
    }
}

#[derive(Debug, Default)]
struct ProgressState {
    start_time: Option<iced::time::Instant>,
    phase: f32,
}

pub fn progress(props: ProgressProps, theme: &Theme) -> ProgressWidget {
    ProgressWidget::new(props, theme)
}

pub struct ProgressWidget {
    props: ProgressProps,
    theme: Theme,
}

impl ProgressWidget {
    fn new(props: ProgressProps, theme: &Theme) -> Self {
        Self {
            props,
            theme: theme.clone(),
        }
    }
}

impl<Message, AppTheme, Renderer> Widget<Message, AppTheme, Renderer> for ProgressWidget
where
    Renderer: renderer::Renderer,
{
    fn tag(&self) -> iced::advanced::widget::tree::Tag {
        iced::advanced::widget::tree::Tag::of::<ProgressState>()
    }

    fn state(&self) -> iced::advanced::widget::tree::State {
        iced::advanced::widget::tree::State::new(ProgressState::default())
    }

    fn size(&self) -> Size<Length> {
        match self.props.orientation {
            ProgressOrientation::Horizontal => {
                Size::new(Length::Fill, Length::Fixed(self.props.size.height()))
            }
            ProgressOrientation::Vertical => {
                Size::new(Length::Fixed(self.props.size.height()), Length::Fill)
            }
        }
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        match self.props.orientation {
            ProgressOrientation::Horizontal => layout::atomic(
                limits,
                Length::Fill,
                Length::Fixed(self.props.size.height()),
            ),
            ProgressOrientation::Vertical => layout::atomic(
                limits,
                Length::Fixed(self.props.size.height()),
                Length::Fill,
            ),
        }
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
        if self.props.value.is_some() {
            return;
        }

        if let Event::Window(window::Event::RedrawRequested(now)) = event {
            let state = tree.state.downcast_mut::<ProgressState>();

            if state.start_time.is_none() {
                state.start_time = Some(*now);
            }

            if let Some(start) = state.start_time {
                let elapsed = now.saturating_duration_since(start);
                let duration = iced::time::Duration::from_millis(self.props.duration_ms as u64);
                state.phase = (elapsed.as_secs_f32() / duration.as_secs_f32()) % 1.0;
            }

            shell.request_redraw();
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
        let bounds = layout.bounds();
        if !bounds.intersects(viewport) {
            return;
        }

        let palette = self.theme.palette;
        let radius = progress_radius(&self.theme, self.props);

        let base_bg = if is_dark(&palette) {
            apply_opacity(accent_low(&palette, AccentColor::Gray), 0.9)
        } else {
            apply_opacity(accent_low(&palette, AccentColor::Gray), 0.7)
        };

        let (background, border, shadow) = match self.props.variant {
            ProgressVariant::Surface => (
                Background::Color(base_bg),
                Border {
                    color: apply_opacity(palette.border, 0.7),
                    width: 1.0,
                    radius: radius.into(),
                },
                Shadow::default(),
            ),
            ProgressVariant::Classic => (
                Background::Color(base_bg),
                Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: radius.into(),
                },
                Shadow {
                    color: apply_opacity(Color::BLACK, 0.10),
                    offset: iced::Vector::new(0.0, 1.0),
                    blur_radius: 8.0,
                },
            ),
            ProgressVariant::Soft => (
                Background::Color(apply_opacity(accent_low(&palette, AccentColor::Gray), 1.0)),
                Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: radius.into(),
                },
                Shadow::default(),
            ),
        };

        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border,
                shadow,
                ..renderer::Quad::default()
            },
            background,
        );

        let indicator_color = if self.props.high_contrast {
            accent_high(&palette, self.props.color)
        } else {
            accent_color(&palette, self.props.color)
        };

        let indicator_bounds = match self.props.orientation {
            ProgressOrientation::Horizontal => {
                let (indicator_x, indicator_width) = if let Some(value) = self.props.value {
                    let ratio = if self.props.max <= 0.0 {
                        0.0
                    } else {
                        (value / self.props.max).clamp(0.0, 1.0)
                    };
                    (bounds.x, bounds.width * ratio)
                } else {
                    // Smooth back-and-forth using sin (like egui)
                    let state = tree.state.downcast_ref::<ProgressState>();
                    let bar_width = (bounds.width * 0.35).max(12.0);
                    let travel = bounds.width - bar_width;
                    let t = (state.phase * std::f32::consts::PI * 2.0).sin() * 0.5 + 0.5;
                    let x = bounds.x + travel * t;
                    (x, bar_width)
                };

                if indicator_width <= 0.0 {
                    return;
                }

                Rectangle {
                    x: indicator_x,
                    y: bounds.y,
                    width: indicator_width,
                    height: bounds.height,
                }
            }
            ProgressOrientation::Vertical => {
                let (indicator_y, indicator_height) = if let Some(value) = self.props.value {
                    let ratio = if self.props.max <= 0.0 {
                        0.0
                    } else {
                        (value / self.props.max).clamp(0.0, 1.0)
                    };
                    let h = bounds.height * ratio;
                    (bounds.y + (bounds.height - h), h)
                } else {
                    // Smooth back-and-forth using sin (like egui)
                    let state = tree.state.downcast_ref::<ProgressState>();
                    let bar_height = (bounds.height * 0.35).max(12.0);
                    let travel = bounds.height - bar_height;
                    let t = (state.phase * std::f32::consts::PI * 2.0).sin() * 0.5 + 0.5;
                    let y = bounds.y + travel * t;
                    (y, bar_height)
                };

                if indicator_height <= 0.0 {
                    return;
                }

                Rectangle {
                    x: bounds.x,
                    y: indicator_y,
                    width: bounds.width,
                    height: indicator_height,
                }
            }
        };

        renderer.fill_quad(
            renderer::Quad {
                bounds: indicator_bounds,
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: radius.into(),
                },
                ..renderer::Quad::default()
            },
            Background::Color(indicator_color),
        );
    }
}

impl<'a, Message, AppTheme, Renderer> From<ProgressWidget>
    for Element<'a, Message, AppTheme, Renderer>
where
    Renderer: renderer::Renderer + 'a,
    Message: 'a,
{
    fn from(widget: ProgressWidget) -> Element<'a, Message, AppTheme, Renderer> {
        Element::new(widget)
    }
}
