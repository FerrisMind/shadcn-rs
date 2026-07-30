//! Custom widget and overlay rendering for [`super::Tooltip`].
//!
//! The widget wraps the trigger element and tracks hover, delay, and the
//! open/close transition in its tree state. While visible, it mounts an
//! iced overlay positioned with [`shadcn_common::compute_floating`] that
//! paints the bubble, the rotated-square arrow, and the content with the
//! `fade-in-0 zoom-in-95 slide-in-from-*-2` entrance of the web component.

use shadcn_common::{
    Easing, FloatingConfig, FloatingRect, FloatingSide, TOOLTIP_SLIDE_PX, TOOLTIP_ZOOM_FROM,
    compute_floating,
};

use crate::iced_compat::advanced::renderer::Renderer as _;
use crate::iced_compat::advanced::widget::{Tree, tree};
use crate::iced_compat::advanced::{Clipboard, Shell, Widget, layout, overlay, renderer};
use crate::iced_compat::widget::canvas;
use crate::iced_compat::widget::graphics::geometry::Renderer as _;
use crate::iced_compat::{
    Border, Element, Event, Length, Point, Radians, Rectangle, Renderer, Size, Theme,
    Transformation, Vector, mouse,
    time::{Duration, Instant},
    window,
};

use super::style::TooltipStyle;
use super::types::TooltipState;

/// Frame pacing while the open/close transition runs, matching the other
/// animated components.
const FRAME_INTERVAL: Duration = Duration::from_millis(16);

/// Distance the diamond center is pulled inside the bubble edge. Measured
/// from the web component: the arrow center sits ≈1 px inside the content
/// box, leaving a ≈6 px visible tip.
const ARROW_INSET: f32 = 1.0;

/// Internal widget produced by the [`super::Tooltip`] builder.
pub(super) struct TooltipWidget<'a, Message> {
    pub(super) trigger: Element<'a, Message>,
    pub(super) content: Element<'a, Message>,
    pub(super) config: FloatingConfig,
    pub(super) delay: Duration,
    pub(super) duration: Duration,
    pub(super) animated: bool,
    pub(super) disabled: bool,
    pub(super) open_override: Option<bool>,
    pub(super) on_open_change: Option<Box<dyn Fn(bool) -> Message + 'a>>,
    pub(super) arrow: bool,
    pub(super) style: TooltipStyle,
}

impl<Message> TooltipWidget<'_, Message> {
    /// Gap the arrow adds between the trigger and the bubble. The web
    /// floating layer offsets the content by the measured arrow height:
    /// with the 10 px diamond the trigger→bubble gap is exactly 10 px.
    fn arrow_gap(&self) -> f32 {
        if self.arrow {
            self.style.arrow_size
        } else {
            0.0
        }
    }

    /// Floating config with the arrow gap folded into the side offset.
    fn effective_config(&self) -> FloatingConfig {
        self.config
            .side_offset(self.config.side_offset + self.arrow_gap())
    }

    /// Synchronizes the logical open target with hover, delay, and the
    /// controlled override, publishing `on_open_change` on transitions.
    fn sync_target(
        &self,
        state: &mut TooltipState,
        hovered: bool,
        now: Instant,
        shell: &mut Shell<'_, Message>,
    ) {
        if hovered {
            if state.hover_started.is_none() {
                state.hover_started = Some(now);

                if !self.delay.is_zero() {
                    shell.request_redraw_at(now + self.delay);
                }
            }
        } else {
            state.hover_started = None;
        }

        let delay_satisfied = self.delay.is_zero()
            || state
                .hover_started
                .is_some_and(|at| now.saturating_duration_since(at) >= self.delay);

        if hovered
            && !delay_satisfied
            && let Some(at) = state.hover_started
        {
            shell.request_redraw_at(at + self.delay);
        }

        let target = match self.open_override {
            Some(forced) => forced && !self.disabled,
            None => !self.disabled && hovered && delay_satisfied,
        };

        if !state.transition.is_initialized() {
            state.open = target;
            state.transition.reset(f32::from(u8::from(target)));
            return;
        }

        if state.open != target {
            state.open = target;

            if let Some(on_open_change) = self.on_open_change.as_ref() {
                shell.publish(on_open_change(target));
            }

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
    fn advance(&self, state: &mut TooltipState, now: Instant, shell: &mut Shell<'_, Message>) {
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

impl<Message> Widget<Message, Theme, Renderer> for TooltipWidget<'_, Message> {
    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.trigger), Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&[self.trigger.as_widget(), self.content.as_widget()]);
    }

    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<TooltipState>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(TooltipState::default())
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
                let state = tree.state.downcast_mut::<TooltipState>();
                let hovered = cursor.position_over(layout.bounds()).is_some();

                self.sync_target(state, hovered, Instant::now(), shell);
            }
            Event::Window(window::Event::RedrawRequested(now)) => {
                let state = tree.state.downcast_mut::<TooltipState>();
                let hovered = cursor.position_over(layout.bounds()).is_some();

                self.sync_target(state, hovered, *now, shell);
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
        let state = *tree.state.downcast_ref::<TooltipState>();
        let config = self.effective_config();
        let style = self.style;
        let arrow = self.arrow;

        let mut children = tree.children.iter_mut();

        let trigger = self.trigger.as_widget_mut().overlay(
            children.next().expect("trigger state"),
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

        // `hideWhenDetached`: skip the bubble entirely while the trigger is
        // scrolled outside the visible viewport.
        let detached = config.hide_when_detached && !viewport.intersects(&anchor);

        let tooltip = (state.is_visible() && !detached).then(|| {
            overlay::Element::new(Box::new(TooltipOverlay {
                content: &mut self.content,
                tree: children.next().expect("content state"),
                anchor,
                config,
                style,
                arrow,
                progress: state.progress(),
            }))
        });

        if trigger.is_some() || tooltip.is_some() {
            Some(
                overlay::Group::with_children(trigger.into_iter().chain(tooltip).collect())
                    .overlay(),
            )
        } else {
            None
        }
    }
}

/// Overlay that lays out and paints the tooltip bubble.
///
/// iced caches the overlay layout node and rebuilds the overlay instance on
/// every pass, so nothing computed in [`Self::layout`] survives until
/// [`Self::draw`]; the final side and arrow position are re-derived from the
/// laid-out bounds instead.
struct TooltipOverlay<'a, 'b, Message> {
    content: &'b mut Element<'a, Message>,
    tree: &'b mut Tree,
    anchor: Rectangle,
    config: FloatingConfig,
    style: TooltipStyle,
    arrow: bool,
    progress: f32,
}

impl<Message> overlay::Overlay<Message, Theme, Renderer> for TooltipOverlay<'_, '_, Message> {
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

    fn draw(
        &self,
        renderer: &mut Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: layout::Layout<'_>,
        cursor: mouse::Cursor,
    ) {
        let progress = self.progress.clamp(0.0, 1.0);

        if progress <= 0.0 {
            return;
        }

        let bounds = layout.bounds();
        let side = actual_side(self.config.side, self.anchor, bounds);
        let arrow_offset = arrow_offset(side, self.anchor, bounds, self.config.arrow_padding);
        let background = self.style.background.scale_alpha(progress);
        let origin = transform_origin(bounds, side, arrow_offset);
        let scale = TOOLTIP_ZOOM_FROM + (1.0 - TOOLTIP_ZOOM_FROM) * progress;
        let slide = slide_vector(side, progress);

        let transform = Transformation::translate(slide.x, slide.y)
            * Transformation::translate(origin.x, origin.y)
            * Transformation::scale(scale)
            * Transformation::translate(-origin.x, -origin.y);

        renderer.with_transformation(transform, |renderer| {
            if self.arrow {
                draw_arrow(
                    renderer,
                    bounds,
                    side,
                    arrow_offset,
                    &self.style,
                    background,
                );
            }

            renderer.fill_quad(
                renderer::Quad {
                    bounds,
                    border: Border {
                        radius: self.style.radius.into(),
                        ..Border::default()
                    },
                    ..renderer::Quad::default()
                },
                background,
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
                &Rectangle::with_size(Size::INFINITE),
            );
        });
    }
}

/// Final side of the bubble, derived from where the cached layout actually
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

/// Arrow anchor along the bubble edge facing the trigger, relative to the
/// bubble origin, pointing at the anchor center.
fn arrow_offset(side: FloatingSide, anchor: Rectangle, bounds: Rectangle, padding: f32) -> f32 {
    let (target, origin, extent) = if side.is_horizontal() {
        (anchor.center_y(), bounds.y, bounds.height)
    } else {
        (anchor.center_x(), bounds.x, bounds.width)
    };

    let min = padding;
    let max = extent - padding;

    if max < min {
        extent / 2.0
    } else {
        (target - origin).clamp(min, max)
    }
}

/// Scale origin at the arrow anchor, mirroring the web transform origin of
/// the shadcn-svelte tooltip content.
fn transform_origin(bounds: Rectangle, side: FloatingSide, arrow_offset: f32) -> Point {
    match side {
        FloatingSide::Top => Point::new(bounds.x + arrow_offset, bounds.y + bounds.height),
        FloatingSide::Bottom => Point::new(bounds.x + arrow_offset, bounds.y),
        FloatingSide::Left => Point::new(bounds.x + bounds.width, bounds.y + arrow_offset),
        FloatingSide::Right => Point::new(bounds.x, bounds.y + arrow_offset),
    }
}

/// Entrance offset of `slide-in-from-*-2`: the bubble starts closer to the
/// trigger and settles into place.
fn slide_vector(side: FloatingSide, progress: f32) -> Vector {
    let distance = (1.0 - progress) * TOOLTIP_SLIDE_PX;

    match side {
        FloatingSide::Top => Vector::new(0.0, distance),
        FloatingSide::Bottom => Vector::new(0.0, -distance),
        FloatingSide::Left => Vector::new(distance, 0.0),
        FloatingSide::Right => Vector::new(-distance, 0.0),
    }
}

/// Paints the rotated-square arrow on the bubble edge facing the trigger.
fn draw_arrow(
    renderer: &mut Renderer,
    bounds: Rectangle,
    side: FloatingSide,
    arrow_offset: f32,
    style: &TooltipStyle,
    background: crate::iced_compat::Color,
) {
    let size = style.arrow_size;

    if size <= 0.0 {
        return;
    }

    let center = match side {
        FloatingSide::Top => Point::new(
            bounds.x + arrow_offset,
            bounds.y + bounds.height - ARROW_INSET,
        ),
        FloatingSide::Bottom => Point::new(bounds.x + arrow_offset, bounds.y + ARROW_INSET),
        FloatingSide::Left => Point::new(
            bounds.x + bounds.width - ARROW_INSET,
            bounds.y + arrow_offset,
        ),
        FloatingSide::Right => Point::new(bounds.x + ARROW_INSET, bounds.y + arrow_offset),
    };

    // The frame is sized to fit the rotated square, drawn around its center.
    let mut frame = canvas::Frame::new(renderer, Size::new(size * 2.0, size * 2.0));
    frame.translate(Vector::new(size, size));
    frame.rotate(Radians(std::f32::consts::FRAC_PI_4));
    frame.fill(
        &canvas::Path::rounded_rectangle(
            Point::new(-size / 2.0, -size / 2.0),
            Size::new(size, size),
            style.arrow_radius.into(),
        ),
        background,
    );
    let geometry = frame.into_geometry();

    renderer.with_translation(Vector::new(center.x - size, center.y - size), |renderer| {
        renderer.draw_geometry(geometry);
    });
}
