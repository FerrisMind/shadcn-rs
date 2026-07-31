//! Track widget (clipped strip, settle animation, drag, loop, autoplay) and
//! root composition with the prev/next controls.

use std::time::Duration;

use iced_core::keyboard;

use crate::iced_compat::advanced::layout::{self, Layout};
use crate::iced_compat::advanced::renderer::Renderer as _;
use crate::iced_compat::advanced::widget::{Operation, Tree, tree};
use crate::iced_compat::advanced::{Clipboard, Shell, Widget, overlay, renderer};
use crate::iced_compat::widget::canvas;
use crate::iced_compat::{
    Color, Element, Event, Length, Point, Rectangle, Size, Vector, mouse, time, touch, window,
};

use shadcn_common::{
    CAROUSEL_DRAG_THRESHOLD_FRACTION, Easing, TransitionValue, carousel_loop_target,
    carousel_next_snap, carousel_step_snap, carousel_wrap_position,
};

use super::geometry::Strip;
use super::types::CarouselOrientation;

/// Frame pacing while the strip settles, matching the other animated components.
const FRAME_INTERVAL: Duration = Duration::from_millis(16);

/// Pointer travel (px) after which a press is treated as a drag, not a click.
const DRAG_SLOP_PX: f32 = 4.0;

/// Resistance applied when dragging past the edges of a non-looped strip.
const RUBBER_BAND: f32 = 0.3;

/// Autoplay settings threaded into the track.
#[derive(Debug, Clone, Copy)]
pub(super) struct Autoplay {
    pub(super) delay: Duration,
    pub(super) stop_on_interaction: bool,
    pub(super) pause_on_hover: bool,
}

/// The scrollable strip of slides.
pub(super) struct Track<'a, Message> {
    pub(super) children: Vec<Element<'a, Message>>,
    pub(super) strip: Strip,
    pub(super) selected: usize,
    pub(super) orientation: CarouselOrientation,
    pub(super) looped: bool,
    pub(super) gap: f32,
    pub(super) main: Length,
    pub(super) cross: Length,
    pub(super) animated: bool,
    pub(super) duration: Duration,
    pub(super) drag_enabled: bool,
    pub(super) keyboard_enabled: bool,
    pub(super) autoplay: Option<Autoplay>,
    pub(super) on_select: Option<Box<dyn Fn(usize) -> Message + 'a>>,
}

/// Per-instance interaction state stored in the widget tree.
#[derive(Debug, Default, Clone, Copy)]
struct TrackState {
    /// Animated strip offset in normalized units.
    offset: TransitionValue,
    drag: Option<Drag>,
    hovered: bool,
    autoplay_deadline: Option<time::Instant>,
    autoplay_stopped: bool,
    autoplay_expected: Option<usize>,
    last_selected: Option<usize>,
}

#[derive(Debug, Clone, Copy)]
struct Drag {
    origin: Point,
    delta_px: f32,
    moved: bool,
}

impl<Message> Track<'_, Message> {
    /// Normalized snap offset of the controlled selection.
    fn selected_snap(&self) -> f32 {
        self.strip.snap_offset(self.selected)
    }

    /// Normalized settle target, continuing along the loop's shortest path.
    fn target_offset(&self, state: &TrackState) -> f32 {
        let snap = self.selected_snap();

        if self.looped && state.offset.is_initialized() {
            carousel_loop_target(state.offset.current(), snap, self.strip.period)
        } else {
            snap
        }
    }

    /// Normalized offset to paint this frame, including an active drag.
    fn displayed_offset(&self, state: &TrackState, unit_px: f32) -> f32 {
        let mut offset = state.offset.displayed(self.target_offset(state));

        if let Some(drag) = state.drag
            && unit_px > 0.0
        {
            offset -= drag.delta_px / unit_px;

            if !self.looped {
                let max = self.strip.snaps.last().copied().unwrap_or(0.0);

                if offset < 0.0 {
                    offset *= RUBBER_BAND;
                } else if offset > max {
                    offset = max + (offset - max) * RUBBER_BAND;
                }
            }
        }

        offset
    }

    /// Main-axis pixel length of the viewport from the laid-out bounds.
    fn main_px(&self, bounds: Rectangle) -> f32 {
        match self.orientation {
            CarouselOrientation::Horizontal => bounds.width,
            CarouselOrientation::Vertical => bounds.height,
        }
    }

    /// One normalized unit in pixels: viewport main length plus one gap.
    fn unit_px(&self, bounds: Rectangle) -> f32 {
        self.main_px(bounds) + self.gap
    }

    /// Main-axis screen shift of slide `index` relative to its static layout
    /// position, choosing the loop copy with the larger viewport overlap.
    fn child_shift(&self, index: usize, offset_norm: f32, bounds: Rectangle) -> f32 {
        let unit = self.unit_px(bounds);
        let start_px = self.strip.starts[index] * unit;
        let offset_px = offset_norm * unit;

        if !self.looped {
            return -offset_px;
        }

        let period_px = self.strip.period * unit;
        if period_px <= 0.0 {
            return -offset_px;
        }

        let content_px = (self.strip.bases[index] * unit - self.gap).max(0.0);
        let main_px = self.main_px(bounds);
        let wrapped = carousel_wrap_position(start_px - offset_px, period_px);
        let overlap = |position: f32| (position + content_px).min(main_px) - position.max(0.0);
        let position = if overlap(wrapped) >= overlap(wrapped - period_px) {
            wrapped
        } else {
            wrapped - period_px
        };

        position - start_px
    }

    /// Shift of slide `index` as a translation vector.
    fn shift_vector(&self, index: usize, offset_norm: f32, bounds: Rectangle) -> Vector {
        let shift = self.child_shift(index, offset_norm, bounds);

        match self.orientation {
            CarouselOrientation::Horizontal => Vector::new(shift, 0.0),
            CarouselOrientation::Vertical => Vector::new(0.0, shift),
        }
    }

    /// Cursor translated into a shifted slide's coordinate space.
    fn shifted_cursor(cursor: mouse::Cursor, shift: Vector) -> mouse::Cursor {
        match cursor.position() {
            Some(position) => mouse::Cursor::Available(position - shift),
            None => cursor,
        }
    }

    /// Main-axis component of a pointer position.
    fn main_component(&self, position: Point) -> f32 {
        match self.orientation {
            CarouselOrientation::Horizontal => position.x,
            CarouselOrientation::Vertical => position.y,
        }
    }

    /// Publishes a snap selection and registers the interaction for autoplay.
    fn publish_select(&self, state: &mut TrackState, shell: &mut Shell<'_, Message>, snap: usize) {
        if let Some(on_select) = self.on_select.as_ref() {
            if snap != self.selected {
                shell.publish(on_select(snap));
            }

            self.note_interaction(state);
        }
    }

    /// Applies `stopOnInteraction` / delay-reset semantics to autoplay.
    fn note_interaction(&self, state: &mut TrackState) {
        if let Some(autoplay) = self.autoplay {
            if autoplay.stop_on_interaction {
                state.autoplay_stopped = true;
            } else {
                state.autoplay_deadline = None;
            }
        }
    }

    /// Whether the widget reacts to pointers and keys at all.
    fn is_interactive(&self) -> bool {
        self.on_select.is_some() && self.strip.snap_count() > 1
    }

    /// Advances animation and autoplay for the frame drawn at `now`.
    fn on_frame(&self, state: &mut TrackState, now: time::Instant, shell: &mut Shell<'_, Message>) {
        // External selection changes (controls, app logic) count as
        // interactions for `stopOnInteraction`, autoplay's own advances do not.
        if state.last_selected != Some(self.selected) {
            if state.last_selected.is_some() && state.autoplay_expected != Some(self.selected) {
                self.note_interaction(state);
            }

            state.autoplay_expected = None;
            state.last_selected = Some(self.selected);
        }

        let target = self.target_offset(state);
        state.offset.advance(
            target,
            self.animated && state.drag.is_none(),
            self.duration,
            Easing::EaseOut,
            now,
        );

        if state.offset.is_running() {
            shell.request_redraw_at(now + FRAME_INTERVAL);
        }

        let Some(autoplay) = self.autoplay else {
            return;
        };

        if !self.is_interactive() || state.autoplay_stopped {
            return;
        }

        if autoplay.pause_on_hover && (state.hovered || state.drag.is_some()) {
            state.autoplay_deadline = Some(now + autoplay.delay);
            return;
        }

        match state.autoplay_deadline {
            None => {
                let deadline = now + autoplay.delay;
                state.autoplay_deadline = Some(deadline);
                shell.request_redraw_at(deadline);
            }
            Some(deadline) if now >= deadline => {
                // The autoplay plugin wraps to the first snap even without loop.
                if let Some(next) = carousel_next_snap(self.selected, self.strip.snap_count(), true)
                    && let Some(on_select) = self.on_select.as_ref()
                {
                    state.autoplay_expected = Some(next);
                    shell.publish(on_select(next));
                }

                let deadline = now + autoplay.delay;
                state.autoplay_deadline = Some(deadline);
                shell.request_redraw_at(deadline);
            }
            Some(deadline) => {
                shell.request_redraw_at(deadline);
            }
        }
    }

    /// Ends an active drag: settles back or commits whole snap steps.
    fn finish_drag(
        &self,
        state: &mut TrackState,
        bounds: Rectangle,
        shell: &mut Shell<'_, Message>,
    ) {
        let Some(drag) = state.drag.take() else {
            return;
        };

        let unit = self.unit_px(bounds);
        // Freeze the dragged position so the settle animation starts there.
        let dragged = {
            let mut with_drag = *state;
            with_drag.drag = Some(drag);
            self.displayed_offset(&with_drag, unit)
        };
        state.offset.reset(dragged);

        if !drag.moved {
            shell.request_redraw();
            return;
        }

        let count = self.strip.snap_count();
        let average_slot_px = if count > 0 {
            (self.strip.period / count as f32) * unit
        } else {
            0.0
        };
        let steps = shadcn_common::carousel_drag_steps(
            -drag.delta_px,
            average_slot_px,
            CAROUSEL_DRAG_THRESHOLD_FRACTION,
        );
        let target = carousel_step_snap(self.selected, count, self.looped, steps);

        self.publish_select(state, shell, target);
        shell.request_redraw();
    }

    /// Snap step for a navigation key, honoring the scroll axis.
    fn key_step(&self, key: &keyboard::Key) -> Option<isize> {
        use keyboard::key::Named;

        match key {
            keyboard::Key::Named(Named::ArrowLeft) => Some(-1),
            keyboard::Key::Named(Named::ArrowRight) => Some(1),
            keyboard::Key::Named(Named::ArrowUp) if self.orientation.is_vertical() => Some(-1),
            keyboard::Key::Named(Named::ArrowDown) if self.orientation.is_vertical() => Some(1),
            _ => None,
        }
    }
}

impl Default for Drag {
    fn default() -> Self {
        Self {
            origin: Point::ORIGIN,
            delta_px: 0.0,
            moved: false,
        }
    }
}

impl<Message> Widget<Message, crate::iced_compat::Theme, crate::iced_compat::Renderer>
    for Track<'_, Message>
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<TrackState>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(TrackState::default())
    }

    fn children(&self) -> Vec<Tree> {
        self.children.iter().map(Tree::new).collect()
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&self.children);
    }

    fn size(&self) -> Size<Length> {
        match self.orientation {
            CarouselOrientation::Horizontal => Size {
                width: self.main,
                height: self.cross,
            },
            CarouselOrientation::Vertical => Size {
                width: self.cross,
                height: self.main,
            },
        }
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &crate::iced_compat::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let max = limits.max();
        let (main_limit, cross_limit) = match self.orientation {
            CarouselOrientation::Horizontal => (max.width, max.height),
            CarouselOrientation::Vertical => (max.height, max.width),
        };
        let main_px = resolve_axis(self.main, main_limit).max(0.0);
        let unit = main_px + self.gap;

        let mut nodes = Vec::with_capacity(self.children.len());
        let mut natural_cross = 0.0_f32;

        for ((child, state), (basis, start)) in self
            .children
            .iter_mut()
            .zip(&mut tree.children)
            .zip(self.strip.bases.iter().zip(&self.strip.starts))
        {
            let content_main = (basis * unit - self.gap).max(0.0);
            let child_max = match self.orientation {
                CarouselOrientation::Horizontal => Size::new(content_main, cross_limit),
                CarouselOrientation::Vertical => Size::new(cross_limit, content_main),
            };
            let child_limits = layout::Limits::new(Size::ZERO, child_max);
            let node = child.as_widget_mut().layout(state, renderer, &child_limits);
            let child_size = node.size();
            let position = start * unit;

            natural_cross = natural_cross.max(match self.orientation {
                CarouselOrientation::Horizontal => child_size.height,
                CarouselOrientation::Vertical => child_size.width,
            });
            nodes.push(node.move_to(match self.orientation {
                CarouselOrientation::Horizontal => Point::new(position, 0.0),
                CarouselOrientation::Vertical => Point::new(0.0, position),
            }));
        }

        let cross_px = match self.cross {
            Length::Shrink => natural_cross,
            length => resolve_axis(length, cross_limit).max(natural_cross.min(cross_limit)),
        };
        let bounds = match self.orientation {
            CarouselOrientation::Horizontal => Size::new(main_px, cross_px),
            CarouselOrientation::Vertical => Size::new(cross_px, main_px),
        };

        layout::Node::with_children(bounds, nodes)
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &crate::iced_compat::Renderer,
        operation: &mut dyn Operation,
    ) {
        for ((child, state), child_layout) in self
            .children
            .iter_mut()
            .zip(&mut tree.children)
            .zip(layout.children())
        {
            child
                .as_widget_mut()
                .operate(state, child_layout, renderer, operation);
        }
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &crate::iced_compat::Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();

        if let Event::Window(window::Event::RedrawRequested(now)) = event {
            let state = tree.state.downcast_mut::<TrackState>();
            self.on_frame(state, *now, shell);
        }

        // Forward to slides in their shifted coordinate spaces first, so
        // interactive slide content keeps priority over the strip drag.
        let offset = {
            let state = tree.state.downcast_ref::<TrackState>();
            self.displayed_offset(state, self.unit_px(bounds))
        };

        let shifts: Vec<Vector> = (0..self.children.len())
            .map(|index| self.shift_vector(index, offset, bounds))
            .collect();

        for (((child, state), child_layout), shift) in self
            .children
            .iter_mut()
            .zip(&mut tree.children)
            .zip(layout.children())
            .zip(shifts.iter())
        {
            child.as_widget_mut().update(
                state,
                event,
                child_layout,
                Self::shifted_cursor(cursor, *shift),
                renderer,
                clipboard,
                shell,
                viewport,
            );
        }

        if !self.is_interactive() {
            return;
        }

        let state = tree.state.downcast_mut::<TrackState>();

        match event {
            Event::Mouse(mouse::Event::CursorMoved { .. })
            | Event::Touch(touch::Event::FingerMoved { .. }) => {
                state.hovered = cursor.is_over(bounds);

                if let Some(drag) = state.drag.as_mut()
                    && let Some(position) = cursor.position()
                {
                    let delta = self.main_component(position) - self.main_component(drag.origin);
                    drag.delta_px = delta;
                    drag.moved = drag.moved || delta.abs() > DRAG_SLOP_PX;
                    shell.request_redraw();
                }
            }
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if self.drag_enabled
                    && !shell.is_event_captured()
                    && state.drag.is_none()
                    && let Some(position) = cursor.position_over(bounds)
                {
                    state.drag = Some(Drag {
                        origin: position,
                        ..Drag::default()
                    });
                    self.note_interaction(state);
                    shell.capture_event();
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. })
            | Event::Touch(touch::Event::FingerLost { .. }) => {
                if state.drag.is_some() {
                    self.finish_drag(state, bounds, shell);
                    shell.capture_event();
                }
            }
            Event::Keyboard(keyboard::Event::KeyPressed { key, .. }) => {
                if self.keyboard_enabled
                    && state.hovered
                    && !shell.is_event_captured()
                    && let Some(step) = self.key_step(key)
                {
                    let count = self.strip.snap_count();
                    let target = carousel_step_snap(self.selected, count, self.looped, step);

                    self.publish_select(state, shell, target);
                    shell.capture_event();
                    shell.request_redraw();
                }
            }
            _ => {}
        }
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &crate::iced_compat::Renderer,
    ) -> mouse::Interaction {
        let bounds = layout.bounds();
        let state = tree.state.downcast_ref::<TrackState>();

        if state.drag.map(|drag| drag.moved).unwrap_or(false) {
            return mouse::Interaction::Grabbing;
        }

        let offset = self.displayed_offset(state, self.unit_px(bounds));

        self.children
            .iter()
            .zip(&tree.children)
            .zip(layout.children())
            .enumerate()
            .map(|(index, ((child, state), child_layout))| {
                let shift = self.shift_vector(index, offset, bounds);

                child.as_widget().mouse_interaction(
                    state,
                    child_layout,
                    Self::shifted_cursor(cursor, shift),
                    viewport,
                    renderer,
                )
            })
            .max()
            .unwrap_or(mouse::Interaction::None)
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut crate::iced_compat::Renderer,
        theme: &crate::iced_compat::Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        let Some(clipped) = bounds.intersection(viewport) else {
            return;
        };

        let state = tree.state.downcast_ref::<TrackState>();
        let offset = self.displayed_offset(state, self.unit_px(bounds));

        // `overflow-hidden` of the web content wrapper: slides paint only
        // inside the viewport band.
        renderer.with_layer(clipped, |renderer| {
            for (index, ((child, state), child_layout)) in self
                .children
                .iter()
                .zip(&tree.children)
                .zip(layout.children())
                .enumerate()
            {
                let shift = self.shift_vector(index, offset, bounds);
                let shifted_bounds = child_layout.bounds() + shift;

                if shifted_bounds.intersection(&clipped).is_none() {
                    continue;
                }

                renderer.with_translation(shift, |renderer| {
                    child.as_widget().draw(
                        state,
                        renderer,
                        theme,
                        style,
                        child_layout,
                        Self::shifted_cursor(cursor, shift),
                        &(clipped - shift),
                    );
                });
            }
        });
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'b>,
        renderer: &crate::iced_compat::Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<
        overlay::Element<'b, Message, crate::iced_compat::Theme, crate::iced_compat::Renderer>,
    > {
        let bounds = layout.bounds();
        let offset = {
            let state = tree.state.downcast_ref::<TrackState>();
            self.displayed_offset(state, self.unit_px(bounds))
        };
        let shifts: Vec<Vector> = (0..self.children.len())
            .map(|index| self.shift_vector(index, offset, bounds))
            .collect();

        let children: Vec<_> = self
            .children
            .iter_mut()
            .zip(&mut tree.children)
            .zip(layout.children())
            .zip(shifts)
            .filter_map(|(((child, state), child_layout), shift)| {
                child.as_widget_mut().overlay(
                    state,
                    child_layout,
                    renderer,
                    viewport,
                    translation + shift,
                )
            })
            .collect();

        (!children.is_empty()).then(|| overlay::Group::with_children(children).overlay())
    }
}

impl<'a, Message: 'a> From<Track<'a, Message>> for Element<'a, Message> {
    fn from(track: Track<'a, Message>) -> Self {
        Element::new(track)
    }
}

/// Resolves a main/cross axis [`Length`] against the available limit.
fn resolve_axis(length: Length, limit: f32) -> f32 {
    match length {
        Length::Fixed(px) => px,
        _ => {
            if limit.is_finite() {
                limit
            } else {
                0.0
            }
        }
    }
}

/// Direction of a control chevron glyph.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ChevronDirection {
    Left,
    Right,
    Up,
    Down,
}

/// Lucide-style chevron painted on a small canvas, used as the default
/// prev/next control glyph.
#[derive(Debug, Clone, Copy)]
struct ControlChevron {
    direction: ChevronDirection,
    color: Color,
}

/// Canvas element with the default control glyph.
pub(super) fn control_glyph<'a, Message: 'a>(
    direction: ChevronDirection,
    color: Color,
    size: f32,
) -> Element<'a, Message> {
    let side = Length::Fixed(size);

    canvas::Canvas::new(ControlChevron { direction, color })
        .width(side)
        .height(side)
        .into()
}

impl<Message> canvas::Program<Message> for ControlChevron {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &crate::iced_compat::Renderer,
        _theme: &crate::iced_compat::Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let size = bounds.size();
        if size.width <= 0.0 || size.height <= 0.0 {
            return Vec::new();
        }

        let extent = size.width.min(size.height);
        let reach = extent * 0.18;
        let arm = extent * 0.30;
        let width = (extent * 0.125).clamp(1.0, 2.0);
        let (a, b, c) = match self.direction {
            ChevronDirection::Left => (
                Point::new(reach, -arm),
                Point::new(-reach, 0.0),
                Point::new(reach, arm),
            ),
            ChevronDirection::Right => (
                Point::new(-reach, -arm),
                Point::new(reach, 0.0),
                Point::new(-reach, arm),
            ),
            ChevronDirection::Up => (
                Point::new(-arm, reach),
                Point::new(0.0, -reach),
                Point::new(arm, reach),
            ),
            ChevronDirection::Down => (
                Point::new(-arm, -reach),
                Point::new(0.0, reach),
                Point::new(arm, -reach),
            ),
        };

        let mut frame = canvas::Frame::new(renderer, size);
        frame.translate(Vector::new(size.width / 2.0, size.height / 2.0));
        frame.stroke(
            &canvas::Path::new(|builder| {
                builder.move_to(a);
                builder.line_to(b);
                builder.line_to(c);
            }),
            canvas::Stroke::default()
                .with_width(width)
                .with_color(self.color)
                .with_line_cap(canvas::LineCap::Round)
                .with_line_join(canvas::LineJoin::Round),
        );

        vec![frame.into_geometry()]
    }
}
