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
use iced::border::Border;
use iced::keyboard;
use iced::keyboard::key::{self, Key};
use iced::mouse;
use iced::touch;
use iced::window;
use num_traits::FromPrimitive;

use crate::button::ButtonRadius;
use crate::theme::Theme;
use crate::tokens::{AccentColor, accent_color, accent_soft, is_dark};

use std::cmp::Ordering;
use std::ops::RangeInclusive;

#[derive(Clone, Copy, Debug)]
pub enum SliderSize {
    One,
    Two,
    Three,
}

#[derive(Clone, Copy, Debug)]
pub enum SliderVariant {
    Classic,
    Surface,
    Soft,
}

#[derive(Clone, Copy, Debug)]
pub struct SliderProps {
    pub size: SliderSize,
    pub variant: SliderVariant,
    pub color: AccentColor,
    pub radius: Option<ButtonRadius>,
    pub high_contrast: bool,
    pub disabled: bool,
}

impl Default for SliderProps {
    fn default() -> Self {
        Self {
            size: SliderSize::Two,
            variant: SliderVariant::Surface,
            color: AccentColor::Gray,
            radius: None,
            high_contrast: false,
            disabled: false,
        }
    }
}

impl SliderProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn size(mut self, size: SliderSize) -> Self {
        self.size = size;
        self
    }

    pub fn variant(mut self, variant: SliderVariant) -> Self {
        self.variant = variant;
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

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl SliderSize {
    fn rail_height(self) -> f32 {
        match self {
            SliderSize::One => 4.0,
            SliderSize::Two => 6.0,
            SliderSize::Three => 8.0,
        }
    }

    fn handle_radius(self) -> f32 {
        match self {
            SliderSize::One => 6.0,
            SliderSize::Two => 8.0,
            SliderSize::Three => 10.0,
        }
    }

    fn thumb_size(self) -> f32 {
        self.handle_radius() * 2.0
    }
}

fn slider_radius(theme: &Theme, props: SliderProps) -> f32 {
    let _ = theme;
    let track_size = props.size.rail_height();
    let (factor, thumb_radius) = radius_tokens(props.radius);
    let base = (factor * track_size / 3.0).max(factor * thumb_radius);
    base.min(track_size / 2.0)
}

fn slider_thumb_radius(props: SliderProps) -> f32 {
    let thumb_size = props.size.thumb_size();
    let (factor, thumb_radius) = radius_tokens(props.radius);
    let base = (3.0 * factor).max(thumb_radius);
    base.min(thumb_size / 2.0)
}

fn radius_tokens(radius: Option<ButtonRadius>) -> (f32, f32) {
    match radius.unwrap_or(ButtonRadius::Medium) {
        ButtonRadius::None => (0.0, 0.5),
        ButtonRadius::Small => (0.75, 0.5),
        ButtonRadius::Medium => (1.0, 9999.0),
        ButtonRadius::Large => (1.5, 9999.0),
        ButtonRadius::Full => (1.5, 9999.0),
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SliderOrientation {
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SliderStatus {
    Active,
    Hovered,
    Dragged,
}

#[derive(Debug, Clone)]
struct SliderState {
    is_dragging: bool,
    active_thumb: Option<usize>,
    hovered_thumb: Option<usize>,
    keyboard_modifiers: keyboard::Modifiers,
}

impl Default for SliderState {
    fn default() -> Self {
        Self {
            is_dragging: false,
            active_thumb: None,
            hovered_thumb: None,
            keyboard_modifiers: keyboard::Modifiers::default(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct SliderStyle {
    track_background: iced::Color,
    range_background: iced::Color,
    track_radius: f32,
    track_border_color: iced::Color,
    track_border_width: f32,
    track_shadow: Shadow,
    range_border_color: iced::Color,
    range_border_width: f32,
    thumb_background: iced::Color,
    thumb_border_color: iced::Color,
    thumb_border_width: f32,
    thumb_radius: f32,
    thumb_shadow: Shadow,
    ring_color: iced::Color,
    ring_width: f32,
    show_ring: bool,
}

pub struct Slider<'a, T, Message> {
    range: RangeInclusive<T>,
    values: Vec<T>,
    on_change: Option<Box<dyn Fn(Vec<T>) -> Message + 'a>>,
    on_release: Option<Message>,
    width: Length,
    height: Length,
    step: T,
    shift_step: Option<T>,
    min_steps_between_thumbs: u16,
    inverted: bool,
    orientation: SliderOrientation,
    props: SliderProps,
    theme: Theme,
    status: Option<SliderStatus>,
}

pub fn slider<'a, Message: Clone + 'a, T, F>(
    range: RangeInclusive<T>,
    values: Vec<T>,
    on_change: Option<F>,
    props: SliderProps,
    theme: &Theme,
) -> Slider<'a, T, Message>
where
    T: Copy + From<u8> + PartialOrd + Into<f64> + FromPrimitive,
    F: Fn(Vec<T>) -> Message + 'a,
{
    Slider::new(
        range,
        values,
        on_change,
        props,
        theme,
        SliderOrientation::Horizontal,
    )
}

pub fn vertical_slider<'a, Message: Clone + 'a, T, F>(
    range: RangeInclusive<T>,
    values: Vec<T>,
    on_change: Option<F>,
    props: SliderProps,
    theme: &Theme,
) -> Slider<'a, T, Message>
where
    T: Copy + From<u8> + PartialOrd + Into<f64> + FromPrimitive,
    F: Fn(Vec<T>) -> Message + 'a,
{
    Slider::new(
        range,
        values,
        on_change,
        props,
        theme,
        SliderOrientation::Vertical,
    )
}

impl<'a, T, Message> Slider<'a, T, Message>
where
    T: Copy + From<u8> + PartialOrd + Into<f64> + FromPrimitive,
    Message: Clone,
{
    fn new<F>(
        range: RangeInclusive<T>,
        values: Vec<T>,
        on_change: Option<F>,
        props: SliderProps,
        theme: &Theme,
        orientation: SliderOrientation,
    ) -> Self
    where
        F: Fn(Vec<T>) -> Message + 'a,
    {
        let values = normalize_values(&range, values);
        let default_extent = props.size.thumb_size();
        let (width, height) = match orientation {
            SliderOrientation::Horizontal => (Length::Fill, Length::Fixed(default_extent)),
            SliderOrientation::Vertical => (Length::Fixed(default_extent), Length::Fill),
        };

        Slider {
            range,
            values,
            on_change: on_change
                .map(|handler| Box::new(handler) as Box<dyn Fn(Vec<T>) -> Message + 'a>),
            on_release: None,
            width,
            height,
            step: T::from(1),
            shift_step: None,
            min_steps_between_thumbs: 0,
            inverted: false,
            orientation,
            props,
            theme: theme.clone(),
            status: None,
        }
    }

    pub fn step(mut self, step: impl Into<T>) -> Self {
        self.step = step.into();
        self
    }

    pub fn shift_step(mut self, shift_step: impl Into<T>) -> Self {
        self.shift_step = Some(shift_step.into());
        self
    }

    pub fn min_steps_between_thumbs(mut self, steps: u16) -> Self {
        self.min_steps_between_thumbs = steps;
        self
    }

    pub fn inverted(mut self, inverted: bool) -> Self {
        self.inverted = inverted;
        self
    }

    pub fn orientation(mut self, orientation: SliderOrientation) -> Self {
        let default_extent = self.props.size.thumb_size();
        match orientation {
            SliderOrientation::Horizontal => {
                if matches!(self.width, Length::Fixed(_)) && matches!(self.height, Length::Fill) {
                    self.width = Length::Fill;
                    self.height = Length::Fixed(default_extent);
                }
            }
            SliderOrientation::Vertical => {
                if matches!(self.height, Length::Fixed(_)) && matches!(self.width, Length::Fill) {
                    self.width = Length::Fixed(default_extent);
                    self.height = Length::Fill;
                }
            }
        }
        self.orientation = orientation;
        self
    }

    pub fn on_release(mut self, on_release: Message) -> Self {
        self.on_release = Some(on_release);
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

    fn step_value(&self, modifiers: keyboard::Modifiers) -> f64 {
        let step_value = if modifiers.shift() {
            self.shift_step.unwrap_or(self.step)
        } else {
            self.step
        };
        step_value.into()
    }

    fn min_distance(&self, step: f64) -> f64 {
        if step <= 0.0 {
            0.0
        } else {
            step * f64::from(self.min_steps_between_thumbs)
        }
    }

    fn apply_value(
        &mut self,
        shell: &mut Shell<'_, Message>,
        index: usize,
        candidate: f64,
        start: f64,
        end: f64,
        min_distance: f64,
    ) {
        let next = clamp_value_for_thumb(&self.values, index, start, end, min_distance, candidate);
        if let Some(next) = T::from_f64(next) {
            if !approx_eq(self.values[index].into(), next.into()) {
                let mut updated = self.values.clone();
                updated[index] = next;
                if let Some(handler) = &self.on_change {
                    shell.publish(handler(updated.clone()));
                }
                self.values = updated;
            }
        }
    }
}

impl<Message, Theme, Renderer, T> Widget<Message, Theme, Renderer> for Slider<'_, T, Message>
where
    T: Copy + Into<f64> + FromPrimitive + PartialOrd + From<u8>,
    Message: Clone,
    Renderer: renderer::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size::new(self.width, self.height)
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout::atomic(limits, self.width, self.height)
    }

    fn tag(&self) -> iced::advanced::widget::tree::Tag {
        iced::advanced::widget::tree::Tag::of::<SliderState>()
    }

    fn state(&self) -> iced::advanced::widget::tree::State {
        iced::advanced::widget::tree::State::new(SliderState::default())
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_mut::<SliderState>();
        let bounds = layout.bounds();
        let disabled = self.props.disabled || self.on_change.is_none();

        if disabled {
            state.is_dragging = false;
            state.active_thumb = None;
            state.hovered_thumb = None;
            let current_status = SliderStatus::Active;
            if let Event::Window(window::Event::RedrawRequested(_now)) = event {
                self.status = Some(current_status);
            } else if self.status.is_some_and(|status| status != current_status) {
                shell.request_redraw();
            }
            return;
        }

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if let Some(cursor_position) = cursor.position_over(bounds) {
                    if let Some(index) = nearest_thumb(
                        &self.values,
                        *self.range.start(),
                        *self.range.end(),
                        bounds,
                        self.orientation,
                        self.inverted,
                        self.props.size.thumb_size(),
                        cursor_position,
                    ) {
                        state.active_thumb = Some(index);
                        state.is_dragging = true;
                        let step = self.step_value(state.keyboard_modifiers);
                        let min_distance = self.min_distance(step);
                        let start = (*self.range.start()).into();
                        let end = (*self.range.end()).into();
                        let candidate = position_to_value(
                            cursor_position,
                            bounds,
                            self.orientation,
                            self.inverted,
                            start,
                            end,
                            step,
                        );
                        self.apply_value(shell, index, candidate, start, end, min_distance);
                        shell.capture_event();
                    }
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. })
            | Event::Touch(touch::Event::FingerLost { .. }) => {
                if state.is_dragging {
                    if let Some(on_release) = self.on_release.clone() {
                        shell.publish(on_release);
                    }
                    state.is_dragging = false;
                    state.active_thumb = None;
                }
            }
            Event::Mouse(mouse::Event::CursorMoved { .. })
            | Event::Touch(touch::Event::FingerMoved { .. }) => {
                if state.is_dragging {
                    if let Some(cursor_position) = cursor.position() {
                        if let Some(index) = state.active_thumb {
                            let step = self.step_value(state.keyboard_modifiers);
                            let min_distance = self.min_distance(step);
                            let start = (*self.range.start()).into();
                            let end = (*self.range.end()).into();
                            let candidate = position_to_value(
                                cursor_position,
                                bounds,
                                self.orientation,
                                self.inverted,
                                start,
                                end,
                                step,
                            );
                            self.apply_value(shell, index, candidate, start, end, min_distance);
                        }
                        shell.capture_event();
                    }
                } else {
                    if let Some(cursor_position) = cursor.position_over(bounds) {
                        state.hovered_thumb = nearest_thumb(
                            &self.values,
                            *self.range.start(),
                            *self.range.end(),
                            bounds,
                            self.orientation,
                            self.inverted,
                            self.props.size.thumb_size(),
                            cursor_position,
                        );
                    } else {
                        state.hovered_thumb = None;
                    }
                }
            }
            Event::Mouse(mouse::Event::WheelScrolled { delta })
                if state.keyboard_modifiers.control() =>
            {
                if cursor.is_over(bounds) {
                    let delta = match delta {
                        mouse::ScrollDelta::Lines { x: _, y } => y,
                        mouse::ScrollDelta::Pixels { x: _, y } => y,
                    };
                    if let Some(index) = state.hovered_thumb.or(state.active_thumb) {
                        let step = self.step_value(state.keyboard_modifiers);
                        let min_distance = self.min_distance(step);
                        let start = (*self.range.start()).into();
                        let end = (*self.range.end()).into();
                        let current = self.values[index].into();
                        let next = if *delta < 0.0 {
                            current - step
                        } else {
                            current + step
                        };
                        self.apply_value(shell, index, next, start, end, min_distance);
                    }
                    shell.capture_event();
                }
            }
            Event::Keyboard(keyboard::Event::KeyPressed { key, .. }) => {
                if cursor.is_over(bounds) {
                    let delta = match key {
                        Key::Named(key::Named::ArrowLeft)
                            if self.orientation == SliderOrientation::Horizontal =>
                        {
                            Some(-1.0)
                        }
                        Key::Named(key::Named::ArrowRight)
                            if self.orientation == SliderOrientation::Horizontal =>
                        {
                            Some(1.0)
                        }
                        Key::Named(key::Named::ArrowDown)
                            if self.orientation == SliderOrientation::Vertical =>
                        {
                            Some(-1.0)
                        }
                        Key::Named(key::Named::ArrowUp)
                            if self.orientation == SliderOrientation::Vertical =>
                        {
                            Some(1.0)
                        }
                        _ => None,
                    };
                    if let Some(delta) = delta {
                        if let Some(index) = state.hovered_thumb.or(state.active_thumb) {
                            let step = self.step_value(state.keyboard_modifiers);
                            let min_distance = self.min_distance(step);
                            let start = (*self.range.start()).into();
                            let end = (*self.range.end()).into();
                            let current = self.values[index].into();
                            let next = current + (step * delta);
                            self.apply_value(shell, index, next, start, end, min_distance);
                        }
                        shell.capture_event();
                    }
                }
            }
            Event::Keyboard(keyboard::Event::ModifiersChanged(modifiers)) => {
                state.keyboard_modifiers = *modifiers;
            }
            _ => {}
        }

        let current_status = if state.is_dragging {
            SliderStatus::Dragged
        } else if cursor.is_over(bounds) {
            SliderStatus::Hovered
        } else {
            SliderStatus::Active
        };

        if let Event::Window(window::Event::RedrawRequested(_now)) = event {
            self.status = Some(current_status);
        } else if self.status.is_some_and(|status| status != current_status) {
            shell.request_redraw();
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

        let status = self.status.unwrap_or(SliderStatus::Active);
        let mut props = self.props;
        if self.on_change.is_none() {
            props.disabled = true;
        }
        let style = slider_style(&self.theme, props, status);
        let thumb_size = self.props.size.thumb_size();
        let track_thickness = self.props.size.rail_height().min(match self.orientation {
            SliderOrientation::Horizontal => bounds.height,
            SliderOrientation::Vertical => bounds.width,
        });

        let (track_bounds, range_bounds) = track_and_range_bounds(
            bounds,
            self.orientation,
            self.inverted,
            thumb_size,
            track_thickness,
            &self.values,
            *self.range.start(),
            *self.range.end(),
        );

        renderer.fill_quad(
            renderer::Quad {
                bounds: track_bounds,
                border: Border {
                    radius: style.track_radius.into(),
                    width: style.track_border_width,
                    color: style.track_border_color,
                },
                shadow: style.track_shadow,
                ..renderer::Quad::default()
            },
            Background::Color(style.track_background),
        );

        renderer.fill_quad(
            renderer::Quad {
                bounds: range_bounds,
                border: Border {
                    radius: style.track_radius.into(),
                    width: style.range_border_width,
                    color: style.range_border_color,
                },
                ..renderer::Quad::default()
            },
            Background::Color(style.range_background),
        );

        let state = tree.state.downcast_ref::<SliderState>();
        let thumb_bounds = thumb_rects(
            bounds,
            self.orientation,
            self.inverted,
            thumb_size,
            &self.values,
            *self.range.start(),
            *self.range.end(),
        );

        for (index, rect) in thumb_bounds.into_iter().enumerate() {
            let show_ring = style.show_ring
                && (state.active_thumb == Some(index) || state.hovered_thumb == Some(index));
            if show_ring {
                let ring_radius = (style.thumb_radius + style.ring_width)
                    .min(rect.width / 2.0 + style.ring_width);
                let ring_bounds = Rectangle {
                    x: rect.x - style.ring_width,
                    y: rect.y - style.ring_width,
                    width: rect.width + style.ring_width * 2.0,
                    height: rect.height + style.ring_width * 2.0,
                };
                renderer.fill_quad(
                    renderer::Quad {
                        bounds: ring_bounds,
                        border: Border {
                            radius: ring_radius.into(),
                            width: 0.0,
                            color: iced::Color::TRANSPARENT,
                        },
                        snap: false,
                        ..renderer::Quad::default()
                    },
                    Background::Color(style.ring_color),
                );
            }

            renderer.fill_quad(
                renderer::Quad {
                    bounds: rect,
                    border: Border {
                        radius: style.thumb_radius.into(),
                        width: style.thumb_border_width,
                        color: style.thumb_border_color,
                    },
                    shadow: style.thumb_shadow,
                    snap: false,
                },
                Background::Color(style.thumb_background),
            );
        }
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        if self.props.disabled || self.on_change.is_none() {
            return mouse::Interaction::default();
        }

        let state = tree.state.downcast_ref::<SliderState>();
        if state.is_dragging {
            if cfg!(target_os = "windows") {
                mouse::Interaction::Pointer
            } else {
                mouse::Interaction::Grabbing
            }
        } else if cursor.is_over(layout.bounds()) {
            if cfg!(target_os = "windows") {
                mouse::Interaction::Pointer
            } else {
                mouse::Interaction::Grab
            }
        } else {
            mouse::Interaction::default()
        }
    }
}

impl<'a, T, Message, Theme, Renderer> From<Slider<'a, T, Message>>
    for Element<'a, Message, Theme, Renderer>
where
    T: Copy + Into<f64> + FromPrimitive + PartialOrd + From<u8> + 'a,
    Message: Clone + 'a,
    Renderer: renderer::Renderer + 'a,
{
    fn from(slider: Slider<'a, T, Message>) -> Element<'a, Message, Theme, Renderer> {
        Element::new(slider)
    }
}

fn normalize_values<T: Copy + PartialOrd>(range: &RangeInclusive<T>, values: Vec<T>) -> Vec<T> {
    let mut values = if values.is_empty() {
        vec![*range.start()]
    } else {
        values
    };

    let start = *range.start();
    let end = *range.end();

    for value in &mut values {
        if *value < start {
            *value = start;
        } else if *value > end {
            *value = end;
        }
    }

    values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
    values
}

fn approx_eq(a: f64, b: f64) -> bool {
    (a - b).abs() <= f64::EPSILON
}

fn position_to_value(
    cursor_position: iced::Point,
    bounds: Rectangle,
    orientation: SliderOrientation,
    inverted: bool,
    start: f64,
    end: f64,
    step: f64,
) -> f64 {
    let min = start.min(end);
    let max = start.max(end);
    let span = (max - min).abs().max(f64::EPSILON);
    let width = bounds.width.max(1.0);
    let height = bounds.height.max(1.0);
    let percent = match orientation {
        SliderOrientation::Horizontal => {
            ((cursor_position.x - bounds.x) / width).clamp(0.0, 1.0) as f64
        }
        SliderOrientation::Vertical => {
            ((cursor_position.y - bounds.y) / height).clamp(0.0, 1.0) as f64
        }
    };

    let base = match orientation {
        SliderOrientation::Horizontal => percent,
        SliderOrientation::Vertical => 1.0 - percent,
    };
    let adjusted = if inverted { 1.0 - base } else { base };
    let raw_value = min + adjusted * span;

    if step > 0.0 {
        let steps = ((raw_value - min) / step).round();
        (steps * step + min).clamp(min, max)
    } else {
        raw_value.clamp(min, max)
    }
}

fn clamp_value_for_thumb<T: Into<f64> + Copy>(
    values: &[T],
    index: usize,
    start: f64,
    end: f64,
    min_distance: f64,
    candidate: f64,
) -> f64 {
    let mut min_value = start.min(end);
    let mut max_value = start.max(end);

    if index > 0 {
        min_value = values[index - 1].into() + min_distance;
    }
    if index + 1 < values.len() {
        max_value = values[index + 1].into() - min_distance;
    }

    candidate.clamp(min_value, max_value)
}

fn nearest_thumb<T: Into<f64> + Copy>(
    values: &[T],
    start: T,
    end: T,
    bounds: Rectangle,
    orientation: SliderOrientation,
    inverted: bool,
    thumb_size: f32,
    cursor_position: iced::Point,
) -> Option<usize> {
    if values.is_empty() {
        return None;
    }

    let positions = thumb_positions(
        bounds,
        orientation,
        inverted,
        thumb_size,
        values,
        start,
        end,
    );
    let cursor_axis = match orientation {
        SliderOrientation::Horizontal => cursor_position.x,
        SliderOrientation::Vertical => cursor_position.y,
    };
    let mut nearest = 0;
    let mut nearest_distance = (positions[0] - cursor_axis).abs();
    for (index, position) in positions.iter().enumerate().skip(1) {
        let distance = (*position - cursor_axis).abs();
        if distance < nearest_distance {
            nearest = index;
            nearest_distance = distance;
        }
    }
    Some(nearest)
}

fn thumb_positions<T: Into<f64> + Copy>(
    bounds: Rectangle,
    orientation: SliderOrientation,
    inverted: bool,
    thumb_size: f32,
    values: &[T],
    start: T,
    end: T,
) -> Vec<f32> {
    let range_start: f64 = start.into();
    let range_end: f64 = end.into();
    let min = range_start.min(range_end);
    let max = range_start.max(range_end);
    let span = (max - min).abs().max(f64::EPSILON);
    let track_length = match orientation {
        SliderOrientation::Horizontal => (bounds.width - thumb_size).max(0.0),
        SliderOrientation::Vertical => (bounds.height - thumb_size).max(0.0),
    };

    values
        .iter()
        .map(|value| {
            let raw: f64 = (*value).into();
            let normalized: f64 = ((raw - min) / span).max(0.0).min(1.0);
            let base = match orientation {
                SliderOrientation::Horizontal => normalized,
                SliderOrientation::Vertical => 1.0 - normalized,
            };
            let percent = if inverted { 1.0 - base } else { base };
            match orientation {
                SliderOrientation::Horizontal => {
                    let offset = track_length * percent as f32;
                    bounds.x + offset + thumb_size / 2.0
                }
                SliderOrientation::Vertical => {
                    let offset = track_length * percent as f32;
                    bounds.y + offset + thumb_size / 2.0
                }
            }
        })
        .collect()
}

fn thumb_rects<T: Into<f64> + Copy>(
    bounds: Rectangle,
    orientation: SliderOrientation,
    inverted: bool,
    thumb_size: f32,
    values: &[T],
    start: T,
    end: T,
) -> Vec<Rectangle> {
    let centers = thumb_positions(
        bounds,
        orientation,
        inverted,
        thumb_size,
        values,
        start,
        end,
    );
    match orientation {
        SliderOrientation::Horizontal => centers
            .into_iter()
            .map(|center| Rectangle {
                x: center - thumb_size / 2.0,
                y: bounds.y + (bounds.height - thumb_size) * 0.5,
                width: thumb_size,
                height: thumb_size,
            })
            .collect(),
        SliderOrientation::Vertical => centers
            .into_iter()
            .map(|center| Rectangle {
                x: bounds.x + (bounds.width - thumb_size) * 0.5,
                y: center - thumb_size / 2.0,
                width: thumb_size,
                height: thumb_size,
            })
            .collect(),
    }
}

fn track_and_range_bounds<T: Into<f64> + Copy>(
    bounds: Rectangle,
    orientation: SliderOrientation,
    inverted: bool,
    thumb_size: f32,
    track_thickness: f32,
    values: &[T],
    start: T,
    end: T,
) -> (Rectangle, Rectangle) {
    let track_bounds = match orientation {
        SliderOrientation::Horizontal => Rectangle {
            x: bounds.x,
            y: bounds.y + (bounds.height - track_thickness) * 0.5,
            width: bounds.width,
            height: track_thickness,
        },
        SliderOrientation::Vertical => Rectangle {
            x: bounds.x + (bounds.width - track_thickness) * 0.5,
            y: bounds.y,
            width: track_thickness,
            height: bounds.height,
        },
    };

    let centers = thumb_positions(
        bounds,
        orientation,
        inverted,
        thumb_size,
        values,
        start,
        end,
    );
    let (min_center, max_center) = centers
        .iter()
        .fold((f32::MAX, f32::MIN), |(min, max), value| {
            (min.min(*value), max.max(*value))
        });

    let (range_start, range_end) = if values.len() <= 1 {
        match orientation {
            SliderOrientation::Horizontal => {
                if inverted {
                    (bounds.x + bounds.width, max_center)
                } else {
                    (bounds.x, max_center)
                }
            }
            SliderOrientation::Vertical => {
                if inverted {
                    (bounds.y, max_center)
                } else {
                    (bounds.y + bounds.height, max_center)
                }
            }
        }
    } else {
        (min_center, max_center)
    };

    let (range_start, range_end) = if range_start <= range_end {
        (range_start, range_end)
    } else {
        (range_end, range_start)
    };

    let range_bounds = match orientation {
        SliderOrientation::Horizontal => Rectangle {
            x: range_start,
            y: track_bounds.y,
            width: range_end - range_start,
            height: track_bounds.height,
        },
        SliderOrientation::Vertical => Rectangle {
            x: track_bounds.x,
            y: range_start,
            width: track_bounds.width,
            height: range_end - range_start,
        },
    };

    (track_bounds, range_bounds)
}

fn slider_style(theme: &Theme, props: SliderProps, status: SliderStatus) -> SliderStyle {
    let palette = theme.palette;
    let accent = accent_color(&palette, props.color);
    let soft_bg = accent_soft(&palette, props.color);
    let range_color = if props.high_contrast {
        palette.foreground
    } else {
        accent
    };
    let soft_track = if is_dark(&palette) {
        apply_opacity(palette.foreground, 0.2)
    } else {
        apply_opacity(palette.foreground, 0.12)
    };

    let mut track_background = match props.variant {
        SliderVariant::Soft => soft_track,
        SliderVariant::Classic | SliderVariant::Surface => palette.muted,
    };
    let mut range_background = match props.variant {
        SliderVariant::Soft => {
            if props.high_contrast {
                palette.foreground
            } else {
                soft_bg
            }
        }
        SliderVariant::Classic | SliderVariant::Surface => range_color,
    };
    let track_border_width = match props.variant {
        SliderVariant::Surface => 1.0,
        SliderVariant::Classic | SliderVariant::Soft => 0.0,
    };
    let mut track_border_color = match props.variant {
        SliderVariant::Surface => apply_opacity(palette.border, 0.8),
        _ => iced::Color::TRANSPARENT,
    };
    let range_border_width = match props.variant {
        SliderVariant::Classic | SliderVariant::Surface => 1.0,
        SliderVariant::Soft => 0.0,
    };
    let mut range_border_color = apply_opacity(palette.border, 0.7);
    let mut track_shadow = match props.variant {
        SliderVariant::Classic => shadow_sm(0.6),
        _ => Shadow::default(),
    };

    let mut thumb_border_color = range_color;
    let mut show_ring = matches!(status, SliderStatus::Hovered | SliderStatus::Dragged);
    let mut thumb_background = iced::Color::WHITE;
    let mut ring_color = apply_opacity(palette.ring, 0.5);
    let mut thumb_shadow = match props.variant {
        SliderVariant::Classic => shadow_md(1.0),
        SliderVariant::Surface => shadow_sm(1.0),
        SliderVariant::Soft => shadow_soft(1.0),
    };

    if props.disabled {
        track_background = apply_opacity(track_background, 0.5);
        range_background = apply_opacity(range_background, 0.5);
        thumb_background = palette.background;
        thumb_border_color = apply_opacity(thumb_border_color, 0.5);
        track_border_color = apply_opacity(track_border_color, 0.5);
        range_border_color = apply_opacity(range_border_color, 0.5);
        track_shadow = shadow_sm(0.0);
        ring_color = apply_opacity(ring_color, 0.5);
        thumb_shadow = shadow_sm(0.5);
        show_ring = false;
    }

    SliderStyle {
        track_background,
        range_background,
        track_radius: slider_radius(theme, props),
        track_border_color,
        track_border_width,
        track_shadow,
        range_border_color,
        range_border_width,
        thumb_background,
        thumb_border_color,
        thumb_border_width: 1.0,
        thumb_radius: slider_thumb_radius(props),
        thumb_shadow,
        ring_color,
        ring_width: 4.0,
        show_ring,
    }
}

fn apply_opacity(color: iced::Color, opacity: f32) -> iced::Color {
    iced::Color {
        a: (color.a * opacity).clamp(0.0, 1.0),
        ..color
    }
}

fn shadow_sm(opacity: f32) -> Shadow {
    Shadow {
        color: apply_opacity(iced::Color::BLACK, 0.05 * opacity),
        offset: Vector::new(0.0, 1.0),
        blur_radius: 2.0,
    }
}

fn shadow_md(opacity: f32) -> Shadow {
    Shadow {
        color: apply_opacity(iced::Color::BLACK, 0.08 * opacity),
        offset: Vector::new(0.0, 1.0),
        blur_radius: 4.0,
    }
}

fn shadow_soft(opacity: f32) -> Shadow {
    Shadow {
        color: apply_opacity(iced::Color::BLACK, 0.06 * opacity),
        offset: Vector::new(0.0, 1.0),
        blur_radius: 3.0,
    }
}
