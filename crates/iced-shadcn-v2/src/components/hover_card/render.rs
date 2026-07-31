//! Custom widget and overlay rendering for [`super::HoverCard`].
//!
//! The widget wraps the trigger element and tracks hover, the open/close
//! delays, and the open/close transition in its tree state. Hovering the
//! trigger arms `openDelay`; once open, the card stays up while the cursor
//! is over the trigger or the interactive floating surface, and leaving
//! both arms `closeDelay` — matching the bits-ui link-preview state
//! machine. While visible, an iced overlay positioned with
//! [`shadcn_common::compute_floating`] paints the surface (`bg-popover`,
//! `ring-1`, shadow) and forwards events to the content. Clicks outside
//! the surface and <kbd>Esc</kbd> dismiss it, and the entrance plays the
//! web `fade-in-0 zoom-in-95 slide-in-from-*-2` animation.

use iced_core::keyboard;

use shadcn_common::{
    Easing, FloatingConfig, FloatingRect, FloatingSide, HOVER_CARD_SLIDE_PX, HOVER_CARD_ZOOM_FROM,
    compute_floating,
};

use crate::iced_compat::advanced::renderer::Renderer as _;
use crate::iced_compat::advanced::widget::{Tree, tree};
use crate::iced_compat::advanced::{Clipboard, Shell, Widget, layout, overlay, renderer};
use crate::iced_compat::{
    Element, Event, Length, Point, Rectangle, Renderer, Size, Theme, Transformation, Vector, mouse,
    time::{Duration, Instant},
    touch, window,
};

use super::style::HoverCardStyle;
use super::types::HoverCardState;

/// Frame pacing while the open/close transition runs, matching the other
/// animated components.
const FRAME_INTERVAL: Duration = Duration::from_millis(16);

/// Internal widget produced by the [`super::HoverCard`] builder.
pub(super) struct HoverCardWidget<'a, Message> {
    pub(super) trigger: Element<'a, Message>,
    pub(super) content: Element<'a, Message>,
    pub(super) config: FloatingConfig,
    pub(super) open_delay: Duration,
    pub(super) close_delay: Duration,
    pub(super) duration: Duration,
    pub(super) animated: bool,
    pub(super) disabled: bool,
    pub(super) open_override: Option<bool>,
    pub(super) default_open: bool,
    pub(super) on_open_change: Option<Box<dyn Fn(bool) -> Message + 'a>>,
    pub(super) close_on_click_outside: bool,
    pub(super) close_on_escape: bool,
    pub(super) style: HoverCardStyle,
}

impl<Message> HoverCardWidget<'_, Message> {
    /// Drives the uncontrolled hover state machine: hovering the trigger
    /// arms `openDelay`; leaving both trigger and content arms
    /// `closeDelay`. Re-entering while a timer runs cancels it, exactly
    /// like the bits-ui link preview clears its timeouts.
    fn sync_hover(
        &self,
        state: &mut HoverCardState,
        trigger_hovered: bool,
        now: Instant,
        shell: &mut Shell<'_, Message>,
    ) {
        // The overlay stops reporting once unmounted; drop the stale flag.
        if !state.is_visible() {
            state.content_hovered = false;
        }

        if self.disabled {
            state.hover_started = None;
            state.leave_started = None;
            return;
        }

        let hovered = trigger_hovered || state.content_hovered;

        if state.requested_open {
            state.hover_started = None;

            if hovered {
                state.leave_started = None;
            } else {
                let since = *state.leave_started.get_or_insert(now);

                if now.saturating_duration_since(since) >= self.close_delay {
                    state.requested_open = false;
                    state.leave_started = None;

                    if let Some(on_open_change) = self.on_open_change.as_ref() {
                        shell.publish(on_open_change(false));
                    }
                } else {
                    shell.request_redraw_at(since + self.close_delay);
                }
            }
        } else {
            state.leave_started = None;

            if trigger_hovered {
                let since = *state.hover_started.get_or_insert(now);

                if now.saturating_duration_since(since) >= self.open_delay {
                    state.requested_open = true;
                    state.hover_started = None;

                    if let Some(on_open_change) = self.on_open_change.as_ref() {
                        shell.publish(on_open_change(true));
                    }
                } else {
                    shell.request_redraw_at(since + self.open_delay);
                }
            } else {
                state.hover_started = None;
            }
        }
    }

    /// Synchronizes the effective open target with the uncontrolled intent
    /// and the controlled override, starting the transition on changes.
    fn sync_target(
        &self,
        state: &mut HoverCardState,
        now: Instant,
        shell: &mut Shell<'_, Message>,
    ) {
        let target = self.open_override.unwrap_or(state.requested_open) && !self.disabled;

        if !state.transition.is_initialized() {
            state.open = target;
            state.transition.reset(f32::from(u8::from(target)));
            return;
        }

        if state.open != target {
            state.open = target;

            state.transition.advance(
                f32::from(u8::from(target)),
                self.animated,
                self.duration,
                Easing::EaseInOut,
                now,
            );

            shell.invalidate_layout();
            shell.request_redraw();
        }
    }

    /// Advances the open/close transition for the frame drawn at `now`.
    fn advance(&self, state: &mut HoverCardState, now: Instant, shell: &mut Shell<'_, Message>) {
        let target = f32::from(u8::from(state.open));

        let was_running = state.transition.is_running();
        state
            .transition
            .advance(target, self.animated, self.duration, Easing::EaseInOut, now);

        if state.transition.is_running() {
            shell.request_redraw_at(now + FRAME_INTERVAL);
        } else if was_running && !state.open {
            // The overlay unmounts once the exit animation ends.
            shell.invalidate_layout();
        }
    }
}

impl<Message> Widget<Message, Theme, Renderer> for HoverCardWidget<'_, Message> {
    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.trigger), Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&[self.trigger.as_widget(), self.content.as_widget()]);
    }

    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<HoverCardState>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(HoverCardState::new(self.default_open))
    }

    fn size(&self) -> Size<Length> {
        self.trigger.as_widget().size()
    }

    fn size_hint(&self) -> Size<Length> {
        self.trigger.as_widget().size_hint()
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        self.trigger
            .as_widget_mut()
            .layout(&mut tree.children[0], renderer, limits)
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: layout::Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        match event {
            Event::Mouse(_) => {
                let state = tree.state.downcast_mut::<HoverCardState>();
                let trigger_hovered = cursor.position_over(layout.bounds()).is_some();

                let now = Instant::now();
                self.sync_hover(state, trigger_hovered, now, shell);
                self.sync_target(state, now, shell);
            }
            Event::Window(window::Event::RedrawRequested(now)) => {
                let state = tree.state.downcast_mut::<HoverCardState>();
                let trigger_hovered = cursor.position_over(layout.bounds()).is_some();

                self.sync_hover(state, trigger_hovered, *now, shell);
                self.sync_target(state, *now, shell);
                self.advance(state, *now, shell);
            }
            _ => {}
        }

        self.trigger.as_widget_mut().update(
            &mut tree.children[0],
            event,
            layout,
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: layout::Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.trigger.as_widget().mouse_interaction(
            &tree.children[0],
            layout,
            cursor,
            viewport,
            renderer,
        )
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: layout::Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        self.trigger.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            style,
            layout,
            cursor,
            viewport,
        );
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: layout::Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn crate::iced_compat::advanced::widget::Operation,
    ) {
        operation.container(None, layout.bounds());
        operation.traverse(&mut |operation| {
            self.trigger.as_widget_mut().operate(
                &mut tree.children[0],
                layout,
                renderer,
                operation,
            );
        });
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: layout::Layout<'b>,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        let Tree {
            state, children, ..
        } = tree;
        let state = state.downcast_mut::<HoverCardState>();
        let config = self.config;
        let style = self.style;

        let mut children = children.iter_mut();
        let trigger_tree = children.next().expect("trigger state");
        let content_tree = children.next().expect("content state");

        let trigger = self.trigger.as_widget_mut().overlay(
            trigger_tree,
            layout,
            renderer,
            viewport,
            translation,
        );

        let bounds = layout.bounds();
        let anchor = Rectangle {
            x: bounds.x + translation.x,
            y: bounds.y + translation.y,
            ..bounds
        };

        // `hideWhenDetached`: skip the surface entirely while the trigger
        // is scrolled outside the visible viewport.
        let detached = config.hide_when_detached && !viewport.intersects(&anchor);

        let card = (state.is_visible() && !detached).then(|| {
            overlay::Element::new(Box::new(HoverCardOverlay {
                content: &mut self.content,
                tree: content_tree,
                state,
                anchor,
                config,
                style,
                on_open_change: self.on_open_change.as_deref(),
                close_on_click_outside: self.close_on_click_outside,
                close_on_escape: self.close_on_escape,
            }))
        });

        if trigger.is_some() || card.is_some() {
            Some(overlay::Group::with_children(trigger.into_iter().chain(card).collect()).overlay())
        } else {
            None
        }
    }
}

/// Overlay that lays out, paints, and drives the interactive hover-card
/// surface.
///
/// iced caches the overlay layout node and rebuilds the overlay instance on
/// every pass, so nothing computed in [`Self::layout`] survives until
/// [`Self::draw`]; the final side is re-derived from the laid-out bounds
/// instead.
struct HoverCardOverlay<'a, 'b, Message> {
    content: &'b mut Element<'a, Message>,
    tree: &'b mut Tree,
    state: &'b mut HoverCardState,
    anchor: Rectangle,
    config: FloatingConfig,
    style: HoverCardStyle,
    on_open_change: Option<&'b (dyn Fn(bool) -> Message + 'a)>,
    close_on_click_outside: bool,
    close_on_escape: bool,
}

impl<Message> HoverCardOverlay<'_, '_, Message> {
    /// Requests an immediate close: drops the uncontrolled open intent —
    /// so the exit animation starts on the next frame — and publishes
    /// `onOpenChange(false)` for controlled consumers.
    ///
    /// Only reachable while the card is effectively open.
    fn request_close(&mut self, shell: &mut Shell<'_, Message>) {
        self.state.requested_open = false;
        self.state.hover_started = None;
        self.state.leave_started = None;
        self.state.content_hovered = false;

        if let Some(on_open_change) = self.on_open_change {
            shell.publish(on_open_change(false));
        }

        shell.request_redraw();
    }
}

impl<Message> overlay::Overlay<Message, Theme, Renderer> for HoverCardOverlay<'_, '_, Message> {
    fn layout(&mut self, renderer: &Renderer, bounds: Size) -> layout::Node {
        let limits = layout::Limits::new(Size::ZERO, bounds);
        let node = self
            .content
            .as_widget_mut()
            .layout(self.tree, renderer, &limits);
        let size = node.size();

        let placement = compute_floating(
            FloatingRect::new(
                self.anchor.x,
                self.anchor.y,
                self.anchor.width,
                self.anchor.height,
            ),
            size.width,
            size.height,
            FloatingRect::new(0.0, 0.0, bounds.width, bounds.height),
            &self.config,
        );

        layout::Node::with_children(size, vec![node])
            .translate(Vector::new(placement.x, placement.y))
    }

    fn update(
        &mut self,
        event: &Event,
        layout: layout::Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) {
        // The surface goes inert as soon as it starts closing, like the
        // web layer that unmounts while `animate-out` plays.
        if !self.state.open {
            self.state.content_hovered = false;
            return;
        }

        let bounds = layout.bounds();

        // Keeping the cursor over the surface holds the card open; the
        // widget reads this flag when evaluating `closeDelay`.
        if matches!(event, Event::Mouse(_) | Event::Window(_)) {
            self.state.content_hovered = cursor.is_over(bounds);
        }

        self.content.as_widget_mut().update(
            self.tree,
            event,
            layout.children().next().expect("content layout"),
            cursor,
            renderer,
            clipboard,
            shell,
            &bounds,
        );

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if cursor.is_over(bounds) {
                    // Presses inside the surface stay inside: nothing
                    // underneath may react or dismiss.
                    shell.capture_event();
                } else if self.close_on_click_outside {
                    // Dismiss without capturing, like the bits-ui
                    // dismissable layer: the outside target still receives
                    // the press.
                    self.request_close(shell);
                }
            }
            Event::Keyboard(keyboard::Event::KeyPressed {
                key: keyboard::Key::Named(keyboard::key::Named::Escape),
                ..
            }) if self.close_on_escape => {
                self.request_close(shell);
                shell.capture_event();
            }
            _ => {}
        }
    }

    fn operate(
        &mut self,
        layout: layout::Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn crate::iced_compat::advanced::widget::Operation,
    ) {
        self.content.as_widget_mut().operate(
            self.tree,
            layout.children().next().expect("content layout"),
            renderer,
            operation,
        );
    }

    fn mouse_interaction(
        &self,
        layout: layout::Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        if !self.state.open {
            return mouse::Interaction::None;
        }

        let bounds = layout.bounds();

        self.content.as_widget().mouse_interaction(
            self.tree,
            layout.children().next().expect("content layout"),
            cursor,
            &bounds,
            renderer,
        )
    }

    fn overlay<'c>(
        &'c mut self,
        layout: layout::Layout<'c>,
        renderer: &Renderer,
    ) -> Option<overlay::Element<'c, Message, Theme, Renderer>> {
        self.content.as_widget_mut().overlay(
            self.tree,
            layout.children().next().expect("content layout"),
            renderer,
            &layout.bounds(),
            Vector::ZERO,
        )
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: layout::Layout<'_>,
        cursor: mouse::Cursor,
    ) {
        let progress = self.state.progress().clamp(0.0, 1.0);

        if progress <= 0.0 {
            return;
        }

        let bounds = layout.bounds();
        let side = actual_side(self.config.side, self.anchor, bounds);
        let origin = transform_origin(bounds, side, self.anchor);
        let scale = HOVER_CARD_ZOOM_FROM + (1.0 - HOVER_CARD_ZOOM_FROM) * progress;
        let slide = slide_vector(side, progress);

        let transform = Transformation::translate(slide.x, slide.y)
            * Transformation::translate(origin.x, origin.y)
            * Transformation::scale(scale)
            * Transformation::translate(-origin.x, -origin.y);

        renderer.with_transformation(transform, |renderer| {
            crate::floating_surface::fill_floating_surface(
                renderer,
                bounds,
                self.style.background.scale_alpha(progress),
                self.style.radius,
                crate::iced_compat::Shadow {
                    color: self.style.shadow.color.scale_alpha(progress),
                    ..self.style.shadow
                },
            );

            let defaults = renderer::Style {
                text_color: self.style.text_color.scale_alpha(progress),
            };

            self.content.as_widget().draw(
                self.tree,
                renderer,
                theme,
                &defaults,
                layout.children().next().expect("content layout"),
                cursor,
                &bounds,
            );

            crate::floating_surface::paint_outside_ring(
                renderer,
                bounds,
                self.style.border_color.scale_alpha(progress),
                self.style.border_width,
                self.style.radius,
            );
        });
    }
}

/// Final side of the surface, derived from where the cached layout actually
/// placed it relative to the anchor (the flip may have inverted the
/// preferred side).
fn actual_side(preferred: FloatingSide, anchor: Rectangle, bounds: Rectangle) -> FloatingSide {
    match preferred {
        FloatingSide::Top | FloatingSide::Bottom => {
            if bounds.center_y() < anchor.center_y() {
                FloatingSide::Top
            } else {
                FloatingSide::Bottom
            }
        }
        FloatingSide::Left | FloatingSide::Right => {
            if bounds.center_x() < anchor.center_x() {
                FloatingSide::Left
            } else {
                FloatingSide::Right
            }
        }
    }
}

/// Scale origin on the surface edge facing the trigger, at the projection
/// of the anchor center — mirroring the web `--transform-origin` variable
/// set by the bits-ui floating layer.
fn transform_origin(bounds: Rectangle, side: FloatingSide, anchor: Rectangle) -> Point {
    let cross = if side.is_horizontal() {
        (anchor.center_y() - bounds.y).clamp(0.0, bounds.height)
    } else {
        (anchor.center_x() - bounds.x).clamp(0.0, bounds.width)
    };

    match side {
        FloatingSide::Top => Point::new(bounds.x + cross, bounds.y + bounds.height),
        FloatingSide::Bottom => Point::new(bounds.x + cross, bounds.y),
        FloatingSide::Left => Point::new(bounds.x + bounds.width, bounds.y + cross),
        FloatingSide::Right => Point::new(bounds.x, bounds.y + cross),
    }
}

/// Entrance offset of `slide-in-from-*-2`: the surface starts closer to the
/// trigger and settles into place.
fn slide_vector(side: FloatingSide, progress: f32) -> Vector {
    let distance = (1.0 - progress) * HOVER_CARD_SLIDE_PX;

    match side {
        FloatingSide::Top => Vector::new(0.0, distance),
        FloatingSide::Bottom => Vector::new(0.0, -distance),
        FloatingSide::Left => Vector::new(distance, 0.0),
        FloatingSide::Right => Vector::new(-distance, 0.0),
    }
}
