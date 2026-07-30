//! Custom widget and overlay rendering for [`super::Sheet`].
//!
//! The widget wraps the trigger element and stores the open state and the
//! open/close transition in its tree state. A click on the trigger opens
//! the sheet; while visible, an iced overlay covering the whole window
//! paints the dimmed backdrop (`.cn-sheet-overlay`), the edge-docked
//! surface (`.cn-sheet-content`), and the ghost close button, and forwards
//! events to the interactive content. Backdrop clicks and <kbd>Esc</kbd>
//! dismiss it. The entrance plays the web `fade-in-0 slide-in-from-*-10`
//! animation (`duration-200`).

use iced_core::keyboard;

use shadcn_common::{Easing, SheetSide, sheet_panel_metrics};

use crate::iced_compat::advanced::renderer::Renderer as _;
use crate::iced_compat::advanced::widget::{Tree, tree};
use crate::iced_compat::advanced::{Clipboard, Shell, Widget, layout, overlay, renderer};
use crate::iced_compat::{
    Border, Element, Event, Length, Point, Rectangle, Renderer, Size, Theme, Transformation,
    Vector, mouse,
    time::{Duration, Instant},
    touch, window,
};

use super::style::SheetStyle;
use super::types::SheetState;

/// Frame pacing while the open/close transition runs.
const FRAME_INTERVAL: Duration = Duration::from_millis(16);

/// Internal widget produced by the [`super::Sheet`] builder.
pub(super) struct SheetWidget<'a, Message> {
    pub(super) trigger: Element<'a, Message>,
    pub(super) surface: Element<'a, Message>,
    pub(super) close: Option<Element<'a, Message>>,
    pub(super) side: SheetSide,
    pub(super) max_width: f32,
    pub(super) max_height: Option<f32>,
    pub(super) close_size: f32,
    pub(super) close_offset: f32,
    pub(super) duration: Duration,
    pub(super) animated: bool,
    pub(super) disabled: bool,
    pub(super) open_override: Option<bool>,
    pub(super) default_open: bool,
    pub(super) on_open_change: Option<Box<dyn Fn(bool) -> Message + 'a>>,
    pub(super) close_on_click_outside: bool,
    pub(super) close_on_escape: bool,
    pub(super) modal: bool,
    pub(super) style: SheetStyle,
}

impl<Message> SheetWidget<'_, Message> {
    fn sync_target(&self, state: &mut SheetState, now: Instant, shell: &mut Shell<'_, Message>) {
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

    fn advance(&self, state: &mut SheetState, now: Instant, shell: &mut Shell<'_, Message>) {
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

    fn handle_trigger_press(&self, state: &mut SheetState, shell: &mut Shell<'_, Message>) {
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
        let mut children = vec![&self.trigger, &self.surface];

        if let Some(close) = &self.close {
            children.push(close);
        }

        children
    }
}

impl<Message> Widget<Message, Theme, Renderer> for SheetWidget<'_, Message> {
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
        tree::Tag::of::<SheetState>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(SheetState::new(self.default_open))
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
                    let state = tree.state.downcast_mut::<SheetState>();
                    self.handle_trigger_press(state, shell);
                }
            }
            Event::Window(window::Event::RedrawRequested(now)) => {
                let state = tree.state.downcast_mut::<SheetState>();

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
        let state = state.downcast_mut::<SheetState>();
        let style = self.style;

        let mut children = children.iter_mut();
        let trigger_tree = children.next().expect("trigger state");
        let surface_tree = children.next().expect("surface state");
        let close_tree = children.next();

        let trigger = self.trigger.as_widget_mut().overlay(
            trigger_tree,
            layout,
            renderer,
            viewport,
            translation,
        );

        let sheet = state.is_visible().then(|| {
            overlay::Element::new(Box::new(SheetOverlay {
                surface: &mut self.surface,
                surface_tree,
                close: self
                    .close
                    .as_mut()
                    .map(|close| (close, close_tree.expect("close state"))),
                state,
                side: self.side,
                max_width: self.max_width,
                max_height: self.max_height,
                close_size: self.close_size,
                close_offset: self.close_offset,
                style,
                on_open_change: self.on_open_change.as_deref(),
                close_on_click_outside: self.close_on_click_outside,
                close_on_escape: self.close_on_escape,
                modal: self.modal,
            }))
        });

        if trigger.is_some() || sheet.is_some() {
            Some(
                overlay::Group::with_children(trigger.into_iter().chain(sheet).collect()).overlay(),
            )
        } else {
            None
        }
    }
}

/// Overlay that lays out, paints, and drives the modal sheet.
struct SheetOverlay<'a, 'b, Message> {
    surface: &'b mut Element<'a, Message>,
    surface_tree: &'b mut Tree,
    close: Option<(&'b mut Element<'a, Message>, &'b mut Tree)>,
    state: &'b mut SheetState,
    side: SheetSide,
    max_width: f32,
    max_height: Option<f32>,
    close_size: f32,
    close_offset: f32,
    style: SheetStyle,
    on_open_change: Option<&'b (dyn Fn(bool) -> Message + 'a)>,
    close_on_click_outside: bool,
    close_on_escape: bool,
    modal: bool,
}

impl<Message> SheetOverlay<'_, '_, Message> {
    fn request_close(&mut self, shell: &mut Shell<'_, Message>) {
        self.state.requested_open = false;

        if let Some(on_open_change) = self.on_open_change {
            shell.publish(on_open_change(false));
        }

        shell.request_redraw();
    }

    fn close_bounds(&self, surface: Rectangle) -> Option<Rectangle> {
        self.close.is_some().then_some(Rectangle {
            x: surface.x + surface.width - self.close_offset - self.close_size,
            y: surface.y + self.close_offset,
            width: self.close_size,
            height: self.close_size,
        })
    }

    /// Inner-edge border strip (`border-l` for right sheets, …).
    fn edge_strip(&self, surface: Rectangle) -> Rectangle {
        let w = self.style.border_width;
        match self.side {
            SheetSide::Right => Rectangle {
                x: surface.x,
                y: surface.y,
                width: w,
                height: surface.height,
            },
            SheetSide::Left => Rectangle {
                x: surface.x + surface.width - w,
                y: surface.y,
                width: w,
                height: surface.height,
            },
            SheetSide::Top => Rectangle {
                x: surface.x,
                y: surface.y + surface.height - w,
                width: surface.width,
                height: w,
            },
            SheetSide::Bottom => Rectangle {
                x: surface.x,
                y: surface.y,
                width: surface.width,
                height: w,
            },
        }
    }
}

impl<Message> overlay::Overlay<Message, Theme, Renderer> for SheetOverlay<'_, '_, Message> {
    fn layout(&mut self, renderer: &Renderer, bounds: Size) -> layout::Node {
        let metrics_for = |content_height: f32| {
            sheet_panel_metrics(
                bounds.width,
                bounds.height,
                self.side,
                self.max_width,
                content_height,
                self.max_height,
            )
        };

        let (surface_node, metrics) = if self.side.is_vertical_edge() {
            // Left/right: `h-full w-3/4 sm:max-w-sm`.
            let metrics = metrics_for(bounds.height);
            let limits = layout::Limits::new(Size::ZERO, Size::new(metrics.width, metrics.height));
            let node = self
                .surface
                .as_widget_mut()
                .layout(self.surface_tree, renderer, &limits);
            (node, metrics)
        } else {
            // Top/bottom: `h-auto` shrink-wrap, optionally capped.
            let max_h = self
                .max_height
                .unwrap_or(bounds.height)
                .min(bounds.height)
                .max(0.0);
            let measure_limits = layout::Limits::new(Size::ZERO, Size::new(bounds.width, max_h));
            let measured =
                self.surface
                    .as_widget_mut()
                    .layout(self.surface_tree, renderer, &measure_limits);
            let metrics = metrics_for(measured.size().height);
            let limits = layout::Limits::new(Size::ZERO, Size::new(metrics.width, metrics.height));
            let node = self
                .surface
                .as_widget_mut()
                .layout(self.surface_tree, renderer, &limits);
            (node, metrics)
        };

        let x = metrics.x;
        let y = metrics.y;
        let panel_w = metrics.width;
        let surface_node = surface_node.move_to(Point::new(x, y));

        let mut children = vec![surface_node];

        if let Some((close, close_tree)) = &mut self.close {
            let close_limits =
                layout::Limits::new(Size::ZERO, Size::new(self.close_size, self.close_size));
            let node = close
                .as_widget_mut()
                .layout(close_tree, renderer, &close_limits);
            let glyph = node.size();

            let button_x = x + panel_w - self.close_offset - self.close_size;
            let button_y = y + self.close_offset;

            children.push(node.move_to(Point::new(
                button_x + (self.close_size - glyph.width) / 2.0,
                button_y + (self.close_size - glyph.height) / 2.0,
            )));
        }

        layout::Node::with_children(bounds, children)
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
        if !self.state.open {
            return;
        }

        let surface_layout = layout.children().next().expect("surface layout");
        let surface_bounds = surface_layout.bounds();
        let close_bounds = self.close_bounds(surface_bounds);

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
                if close_bounds.is_some_and(|bounds| cursor.is_over(bounds)) {
                    self.request_close(shell);
                    shell.capture_event();
                } else if cursor.is_over(surface_bounds) {
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
        if !self.state.open {
            return mouse::Interaction::None;
        }

        let surface_layout = layout.children().next().expect("surface layout");
        let surface_bounds = surface_layout.bounds();

        if self
            .close_bounds(surface_bounds)
            .is_some_and(|bounds| cursor.is_over(bounds))
        {
            return mouse::Interaction::Pointer;
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

        if progress <= 0.0 {
            return;
        }

        let bounds = layout.bounds();

        // `.cn-sheet-overlay`: `fixed inset-0 bg-black/N fade-in-0`.
        renderer.fill_quad(
            renderer::Quad {
                bounds,
                ..renderer::Quad::default()
            },
            self.style.overlay.scale_alpha(progress),
        );

        let mut children = layout.children();
        let surface_layout = children.next().expect("surface layout");
        let close_layout = children.next();

        let surface_bounds = surface_layout.bounds();
        let metrics = sheet_panel_metrics(
            bounds.width,
            bounds.height,
            self.side,
            self.max_width,
            surface_bounds.height,
            self.max_height,
        );

        // `slide-in-from-*-10`: surface starts `SHEET_SLIDE_PX` off its dock.
        let slide = Vector::new(
            metrics.slide_from_x * (1.0 - progress),
            metrics.slide_from_y * (1.0 - progress),
        );
        let transform = Transformation::translate(slide.x, slide.y);

        renderer.with_transformation(transform, |renderer| {
            renderer.fill_quad(
                renderer::Quad {
                    bounds: surface_bounds,
                    border: Border::default(),
                    shadow: crate::iced_compat::Shadow {
                        color: self.style.shadow.color.scale_alpha(progress),
                        ..self.style.shadow
                    },
                    ..renderer::Quad::default()
                },
                self.style.background.scale_alpha(progress),
            );

            // Inner-edge hairline (`border-l` / `border-r` / …).
            renderer.fill_quad(
                renderer::Quad {
                    bounds: self.edge_strip(surface_bounds),
                    ..renderer::Quad::default()
                },
                self.style.border_color.scale_alpha(progress),
            );

            let defaults = renderer::Style {
                text_color: self.style.text_color.scale_alpha(progress),
            };

            self.surface.as_widget().draw(
                self.surface_tree,
                renderer,
                theme,
                &defaults,
                surface_layout,
                cursor,
                &surface_bounds,
            );

            if let (Some((close, close_tree)), Some(close_layout)) = (&self.close, close_layout) {
                let button = self
                    .close_bounds(surface_bounds)
                    .expect("close bounds exist alongside the close element");

                let background = if cursor.is_over(button) {
                    self.style.close_hover_background
                } else {
                    self.style.close_background
                };

                if background.a > 0.0 {
                    renderer.fill_quad(
                        renderer::Quad {
                            bounds: button,
                            border: Border {
                                radius: self.style.close_radius.into(),
                                ..Border::default()
                            },
                            ..renderer::Quad::default()
                        },
                        background.scale_alpha(progress),
                    );
                }

                close.as_widget().draw(
                    close_tree,
                    renderer,
                    theme,
                    &defaults,
                    close_layout,
                    cursor,
                    &button,
                );
            }
        });
    }
}
