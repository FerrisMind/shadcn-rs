//! Custom widget and overlay rendering for [`super::Drawer`].
//!
//! The widget wraps the trigger element and stores the open state and the
//! open/close transition in its tree state. A click on the trigger opens
//! the drawer; while visible, an iced overlay covering the whole window
//! paints the dimmed backdrop (`.cn-drawer-overlay`), the edge-docked
//! surface (`.cn-drawer-content`), and the muted drag handle on bottom
//! drawers, and forwards events to the interactive content. Backdrop
//! clicks, drag-dismiss, and <kbd>Esc</kbd> dismiss it. The entrance plays
//! a full-panel vaul slide.

use iced_core::keyboard;

use shadcn_common::{DrawerDirection, Easing, drawer_panel_metrics};

use crate::iced_compat::advanced::renderer::Renderer as _;
use crate::iced_compat::advanced::widget::{Tree, tree};
use crate::iced_compat::advanced::{Clipboard, Shell, Widget, layout, overlay, renderer};
use crate::iced_compat::{
    Border, Element, Event, Length, Point, Rectangle, Renderer, Size, Theme, Transformation,
    Vector, mouse,
    time::{Duration, Instant},
    touch, window,
};

use super::style::DrawerStyle;
use super::types::DrawerState;

/// Frame pacing while the open/close transition runs.
const FRAME_INTERVAL: Duration = Duration::from_millis(16);

/// Drag distance (px) past which a bottom drawer dismisses on release.
const DISMISS_DRAG_THRESHOLD_PX: f32 = 80.0;

/// Internal widget produced by the [`super::Drawer`] builder.
pub(super) struct DrawerWidget<'a, Message> {
    pub(super) trigger: Element<'a, Message>,
    pub(super) surface: Element<'a, Message>,
    pub(super) direction: DrawerDirection,
    pub(super) max_width: f32,
    pub(super) max_height: Option<f32>,
    pub(super) handle_width: f32,
    pub(super) handle_margin_top: f32,
    pub(super) duration: Duration,
    pub(super) animated: bool,
    pub(super) disabled: bool,
    pub(super) open_override: Option<bool>,
    pub(super) default_open: bool,
    pub(super) on_open_change: Option<Box<dyn Fn(bool) -> Message + 'a>>,
    pub(super) close_on_click_outside: bool,
    pub(super) close_on_escape: bool,
    pub(super) modal: bool,
    pub(super) should_scale_background: bool,
    pub(super) show_handle: bool,
    pub(super) snap_points: Vec<f32>,
    pub(super) active_snap_point: Option<f32>,
    pub(super) on_snap_point_change: Option<Box<dyn Fn(Option<f32>) -> Message + 'a>>,
    pub(super) nested: bool,
    pub(super) style: DrawerStyle,
}

impl<Message> DrawerWidget<'_, Message> {
    fn sync_target(&self, state: &mut DrawerState, now: Instant, shell: &mut Shell<'_, Message>) {
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

    fn advance(&self, state: &mut DrawerState, now: Instant, shell: &mut Shell<'_, Message>) {
        let target = f32::from(u8::from(state.open));

        let was_running = state.transition.is_running();
        state
            .transition
            .advance(target, self.animated, self.duration, Easing::EaseInOut, now);

        if state.transition.is_running() {
            shell.request_redraw_at(now + FRAME_INTERVAL);
        } else if was_running && !state.open {
            shell.invalidate_layout();
        }
    }

    fn handle_trigger_press(&self, state: &mut DrawerState, shell: &mut Shell<'_, Message>) {
        if self.disabled || state.open {
            return;
        }

        state.requested_open = true;

        if let Some(on_open_change) = self.on_open_change.as_ref() {
            shell.publish(on_open_change(true));
        }

        self.sync_target(state, Instant::now(), shell);
    }

    fn child_widgets(&self) -> Vec<&Element<'_, Message>> {
        vec![&self.trigger, &self.surface]
    }

    fn resolved_snap(&self) -> Option<f32> {
        self.active_snap_point
            .or_else(|| self.snap_points.first().copied())
    }
}

impl<Message> Widget<Message, Theme, Renderer> for DrawerWidget<'_, Message> {
    fn children(&self) -> Vec<Tree> {
        self.child_widgets().into_iter().map(Tree::new).collect()
    }

    fn diff(&self, tree: &mut Tree) {
        let children: Vec<_> = self
            .child_widgets()
            .into_iter()
            .map(Element::as_widget)
            .collect();

        tree.diff_children(&children);
    }

    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<DrawerState>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(DrawerState::new(self.default_open))
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
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if cursor.is_over(layout.bounds()) {
                    let state = tree.state.downcast_mut::<DrawerState>();
                    self.handle_trigger_press(state, shell);
                }
            }
            Event::Window(window::Event::RedrawRequested(now)) => {
                let state = tree.state.downcast_mut::<DrawerState>();

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
        let state = state.downcast_mut::<DrawerState>();
        let style = self.style;

        let mut children = children.iter_mut();
        let trigger_tree = children.next().expect("trigger state");
        let surface_tree = children.next().expect("surface state");

        let active_snap_point = self.resolved_snap();
        let direction = self.direction;
        let max_width = self.max_width;
        let max_height = self.max_height;
        let handle_width = self.handle_width;
        let handle_margin_top = self.handle_margin_top;
        let on_open_change = self.on_open_change.as_deref();
        let close_on_click_outside = self.close_on_click_outside;
        let close_on_escape = self.close_on_escape;
        let modal = self.modal;
        let should_scale_background = self.should_scale_background;
        let show_handle = self.show_handle;
        let snap_points = self.snap_points.as_slice();
        let on_snap_point_change = self.on_snap_point_change.as_deref();
        let nested = self.nested;
        let visible = state.is_visible();

        let trigger = self.trigger.as_widget_mut().overlay(
            trigger_tree,
            layout,
            renderer,
            viewport,
            translation,
        );

        let drawer = visible.then(|| {
            overlay::Element::new(Box::new(DrawerOverlay {
                surface: &mut self.surface,
                surface_tree,
                state,
                direction,
                max_width,
                max_height,
                handle_width,
                handle_margin_top,
                style,
                on_open_change,
                close_on_click_outside,
                close_on_escape,
                modal,
                should_scale_background,
                show_handle,
                snap_points,
                active_snap_point,
                on_snap_point_change,
                nested,
            }))
        });

        if trigger.is_some() || drawer.is_some() {
            Some(
                overlay::Group::with_children(trigger.into_iter().chain(drawer).collect())
                    .overlay(),
            )
        } else {
            None
        }
    }
}

/// Overlay that lays out, paints, and drives the modal drawer.
struct DrawerOverlay<'a, 'b, Message> {
    surface: &'b mut Element<'a, Message>,
    surface_tree: &'b mut Tree,
    state: &'b mut DrawerState,
    direction: DrawerDirection,
    max_width: f32,
    max_height: Option<f32>,
    handle_width: f32,
    handle_margin_top: f32,
    style: DrawerStyle,
    on_open_change: Option<&'b (dyn Fn(bool) -> Message + 'a)>,
    close_on_click_outside: bool,
    close_on_escape: bool,
    modal: bool,
    should_scale_background: bool,
    show_handle: bool,
    snap_points: &'b [f32],
    active_snap_point: Option<f32>,
    on_snap_point_change: Option<&'b (dyn Fn(Option<f32>) -> Message + 'a)>,
    nested: bool,
}

impl<Message> DrawerOverlay<'_, '_, Message> {
    fn request_close(&mut self, shell: &mut Shell<'_, Message>) {
        self.state.requested_open = false;
        self.state.clear_drag();

        if let Some(on_open_change) = self.on_open_change {
            shell.publish(on_open_change(false));
        }

        shell.request_redraw();
    }

    fn metrics_for(
        &self,
        viewport: Size,
        content_height: f32,
    ) -> shadcn_common::DrawerPanelMetrics {
        let mut metrics = drawer_panel_metrics(
            viewport.width,
            viewport.height,
            self.direction,
            self.max_width,
            content_height,
            self.max_height,
            self.active_snap_point,
        );

        if self.nested {
            let inset = 12.0;
            match self.direction {
                DrawerDirection::Bottom => {
                    metrics.height = (metrics.height - inset).max(0.0);
                    metrics.y += inset / 2.0;
                    metrics.slide_from_y = metrics.height;
                }
                DrawerDirection::Top => {
                    metrics.height = (metrics.height - inset).max(0.0);
                    metrics.slide_from_y = -metrics.height;
                }
                DrawerDirection::Left => {
                    metrics.width = (metrics.width - inset).max(0.0);
                    metrics.slide_from_x = -metrics.width;
                }
                DrawerDirection::Right => {
                    metrics.width = (metrics.width - inset).max(0.0);
                    metrics.x += inset;
                    metrics.slide_from_x = metrics.width;
                }
            }
        }

        metrics
    }

    fn handle_bounds(&self, surface: Rectangle) -> Option<Rectangle> {
        self.show_handle.then_some(Rectangle {
            x: surface.x + (surface.width - self.handle_width) / 2.0,
            y: surface.y + self.handle_margin_top,
            width: self.handle_width,
            height: self.style.handle_height_px,
        })
    }

    fn content_origin(&self, metrics: &shadcn_common::DrawerPanelMetrics) -> Point {
        let pad = self.style.floating_pad_px;
        let handle_reserve = if self.show_handle {
            self.handle_margin_top + self.style.handle_height_px
        } else {
            0.0
        };

        Point::new(metrics.x + pad, metrics.y + pad + handle_reserve)
    }

    fn content_size(&self, metrics: &shadcn_common::DrawerPanelMetrics) -> Size {
        let pad = self.style.floating_pad_px * 2.0;
        let handle_reserve = if self.show_handle {
            self.handle_margin_top + self.style.handle_height_px
        } else {
            0.0
        };

        Size::new(
            (metrics.width - pad).max(0.0),
            (metrics.height - pad - handle_reserve).max(0.0),
        )
    }

    fn nearest_snap(&self, fraction: f32) -> Option<f32> {
        self.snap_points.iter().copied().min_by(|a, b| {
            (a - fraction)
                .abs()
                .partial_cmp(&(b - fraction).abs())
                .unwrap_or(std::cmp::Ordering::Equal)
        })
    }

    fn begin_drag(&mut self, cursor: mouse::Cursor) {
        if !self.show_handle {
            return;
        }

        if let Some(position) = cursor.position() {
            self.state.dragging = true;
            self.state.drag_origin = Some((position.x, position.y));
            self.state.drag_offset = 0.0;
        }
    }

    fn update_drag(&mut self, cursor: mouse::Cursor, shell: &mut Shell<'_, Message>) {
        if !self.state.dragging {
            return;
        }

        let Some(origin) = self.state.drag_origin else {
            return;
        };
        let Some(position) = cursor.position() else {
            return;
        };

        // Bottom drawers dismiss by dragging down (positive Y).
        self.state.drag_offset = (position.y - origin.1).max(0.0);
        shell.request_redraw();
    }

    fn end_drag(&mut self, viewport_height: f32, shell: &mut Shell<'_, Message>) {
        if !self.state.dragging {
            return;
        }

        let offset = self.state.drag_offset;
        self.state.dragging = false;
        self.state.drag_origin = None;

        if offset >= DISMISS_DRAG_THRESHOLD_PX {
            self.state.drag_offset = 0.0;
            self.request_close(shell);
            return;
        }

        if !self.snap_points.is_empty() && viewport_height > 0.0 {
            let current = self.active_snap_point.unwrap_or(0.5);
            let visual_height = (current * viewport_height - offset).max(0.0);
            let fraction = (visual_height / viewport_height).clamp(0.0, 1.0);
            if let Some(nearest) = self.nearest_snap(fraction)
                && let Some(on_snap) = self.on_snap_point_change
            {
                shell.publish(on_snap(Some(nearest)));
            }
        }

        self.state.drag_offset = 0.0;
        shell.request_redraw();
    }
}

impl<Message> overlay::Overlay<Message, Theme, Renderer> for DrawerOverlay<'_, '_, Message> {
    fn layout(&mut self, renderer: &Renderer, bounds: Size) -> layout::Node {
        let (surface_node, metrics) = if self.direction.is_vertical_edge() {
            let metrics = self.metrics_for(bounds, bounds.height);
            let content = self.content_size(&metrics);
            let limits = layout::Limits::new(Size::ZERO, content);
            let node = self
                .surface
                .as_widget_mut()
                .layout(self.surface_tree, renderer, &limits);
            (node, metrics)
        } else {
            let max_h = self
                .max_height
                .unwrap_or(bounds.height)
                .min(bounds.height)
                .max(0.0);
            let provisional = self.metrics_for(bounds, max_h);
            let content = self.content_size(&provisional);
            let measure_limits = layout::Limits::new(Size::ZERO, content);
            let measured =
                self.surface
                    .as_widget_mut()
                    .layout(self.surface_tree, renderer, &measure_limits);

            let handle_reserve = if self.show_handle {
                self.handle_margin_top + self.style.handle_height_px
            } else {
                0.0
            };
            let pad = self.style.floating_pad_px * 2.0;
            let total_height = measured.size().height + handle_reserve + pad;
            let metrics = self.metrics_for(bounds, total_height);
            let content = self.content_size(&metrics);
            let limits = layout::Limits::new(Size::ZERO, content);
            let node = self
                .surface
                .as_widget_mut()
                .layout(self.surface_tree, renderer, &limits);
            (node, metrics)
        };

        let origin = self.content_origin(&metrics);
        let surface_node = surface_node.move_to(origin);

        layout::Node::with_children(bounds, vec![surface_node])
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
        if !self.state.open && !self.state.dragging {
            return;
        }

        let surface_layout = layout.children().next().expect("surface layout");
        let surface_bounds = surface_layout.bounds();
        let panel = self.metrics_for(
            layout.bounds().size(),
            surface_bounds.height
                + if self.show_handle {
                    self.handle_margin_top + self.style.handle_height_px
                } else {
                    0.0
                }
                + self.style.floating_pad_px * 2.0,
        );
        let panel_bounds = Rectangle {
            x: panel.x,
            y: panel.y + self.state.drag_offset,
            width: panel.width,
            height: panel.height,
        };
        let handle_bounds = self.handle_bounds(panel_bounds);

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if handle_bounds.is_some_and(|bounds| cursor.is_over(bounds)) {
                    self.begin_drag(cursor);
                    shell.capture_event();
                    return;
                }
            }
            Event::Mouse(mouse::Event::CursorMoved { .. })
            | Event::Touch(touch::Event::FingerMoved { .. }) => {
                if self.state.dragging {
                    self.update_drag(cursor, shell);
                    shell.capture_event();
                    return;
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. })
            | Event::Touch(touch::Event::FingerLost { .. })
                if self.state.dragging =>
            {
                self.end_drag(layout.bounds().height, shell);
                shell.capture_event();
                return;
            }
            _ => {}
        }

        if !self.state.open {
            return;
        }

        self.surface.as_widget_mut().update(
            self.surface_tree,
            event,
            surface_layout,
            cursor,
            renderer,
            clipboard,
            shell,
            &surface_bounds,
        );

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if cursor.is_over(panel_bounds) {
                    shell.capture_event();
                } else {
                    if self.close_on_click_outside {
                        self.request_close(shell);
                    }

                    if self.modal {
                        shell.capture_event();
                    }
                }
            }
            Event::Keyboard(keyboard::Event::KeyPressed {
                key: keyboard::Key::Named(keyboard::key::Named::Escape),
                ..
            }) => {
                if self.close_on_escape {
                    self.request_close(shell);
                    shell.capture_event();
                } else if self.modal {
                    shell.capture_event();
                }
            }
            Event::Mouse(mouse::Event::CursorMoved { .. }) => {
                shell.request_redraw();

                if self.modal {
                    shell.capture_event();
                }
            }
            Event::Mouse(_) | Event::Touch(_) | Event::Keyboard(_) if self.modal => {
                shell.capture_event();
            }
            Event::Mouse(_) | Event::Touch(_) | Event::Keyboard(_) => {}
            _ => {}
        }
    }

    fn operate(
        &mut self,
        layout: layout::Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn crate::iced_compat::advanced::widget::Operation,
    ) {
        self.surface.as_widget_mut().operate(
            self.surface_tree,
            layout.children().next().expect("surface layout"),
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
        if !self.state.open && !self.state.dragging {
            return mouse::Interaction::None;
        }

        let surface_layout = layout.children().next().expect("surface layout");
        let surface_bounds = surface_layout.bounds();
        let panel = self.metrics_for(
            layout.bounds().size(),
            surface_bounds.height
                + if self.show_handle {
                    self.handle_margin_top + self.style.handle_height_px
                } else {
                    0.0
                }
                + self.style.floating_pad_px * 2.0,
        );
        let panel_bounds = Rectangle {
            x: panel.x,
            y: panel.y + self.state.drag_offset,
            width: panel.width,
            height: panel.height,
        };

        if self
            .handle_bounds(panel_bounds)
            .is_some_and(|bounds| cursor.is_over(bounds))
        {
            return mouse::Interaction::Grab;
        }

        self.surface.as_widget().mouse_interaction(
            self.surface_tree,
            surface_layout,
            cursor,
            &surface_bounds,
            renderer,
        )
    }

    fn overlay<'c>(
        &'c mut self,
        layout: layout::Layout<'c>,
        renderer: &Renderer,
    ) -> Option<overlay::Element<'c, Message, Theme, Renderer>> {
        let surface_layout = layout.children().next().expect("surface layout");

        self.surface.as_widget_mut().overlay(
            self.surface_tree,
            surface_layout,
            renderer,
            &surface_layout.bounds(),
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

        if progress <= 0.0 && self.state.drag_offset <= 0.0 {
            return;
        }

        let bounds = layout.bounds();

        // `.cn-drawer-overlay`: `fixed inset-0 bg-black/N fade-in-0`.
        let overlay_alpha = if self.should_scale_background {
            // Slightly stronger dim suggests vaul's scaled background.
            (self.style.overlay.a * 1.25).min(0.92)
        } else {
            self.style.overlay.a
        };
        let mut overlay = self.style.overlay;
        overlay.a = overlay_alpha;

        renderer.fill_quad(
            renderer::Quad {
                bounds,
                ..renderer::Quad::default()
            },
            overlay.scale_alpha(progress),
        );

        let surface_layout = layout.children().next().expect("surface layout");
        let surface_bounds = surface_layout.bounds();

        let handle_reserve = if self.show_handle {
            self.handle_margin_top + self.style.handle_height_px
        } else {
            0.0
        };
        let metrics = self.metrics_for(
            bounds.size(),
            surface_bounds.height + handle_reserve + self.style.floating_pad_px * 2.0,
        );

        let slide = Vector::new(
            metrics.slide_from_x * (1.0 - progress),
            metrics.slide_from_y * (1.0 - progress) + self.state.drag_offset,
        );
        let transform = Transformation::translate(slide.x, slide.y);

        let pad = self.style.floating_pad_px;
        let panel_bounds = if pad > 0.0 {
            Rectangle {
                x: metrics.x + pad,
                y: metrics.y + pad,
                width: (metrics.width - pad * 2.0).max(0.0),
                height: (metrics.height - pad * 2.0).max(0.0),
            }
        } else {
            Rectangle {
                x: metrics.x,
                y: metrics.y,
                width: metrics.width,
                height: metrics.height,
            }
        };

        renderer.with_transformation(transform, |renderer| {
            // One rounded fill + hairline border (dialog-style). A separate
            // full-width edge strip under `rounded-*-xl` left transparent
            // corner gaps that looked like shadows / empty bands.
            renderer.fill_quad(
                renderer::Quad {
                    bounds: panel_bounds,
                    border: self.style.surface_border(progress),
                    ..renderer::Quad::default()
                },
                self.style.background.scale_alpha(progress),
            );

            if let Some(handle) = self.handle_bounds(panel_bounds) {
                renderer.fill_quad(
                    renderer::Quad {
                        bounds: handle,
                        border: Border {
                            radius: self.style.handle_radius_px.into(),
                            ..Border::default()
                        },
                        ..renderer::Quad::default()
                    },
                    self.style.handle_color.scale_alpha(progress),
                );
            }

            let defaults = renderer::Style {
                text_color: self.style.text_color.scale_alpha(progress),
            };

            renderer.with_layer(panel_bounds, |renderer| {
                self.surface.as_widget().draw(
                    self.surface_tree,
                    renderer,
                    theme,
                    &defaults,
                    surface_layout,
                    cursor,
                    &surface_bounds,
                );
            });
        });
    }
}
