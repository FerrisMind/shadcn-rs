//! Custom widget and overlay rendering for [`super::AlertDialog`].
//!
//! The widget wraps the trigger element and stores the open state and the
//! open/close transition in its tree state. A click on the trigger opens
//! the dialog; while visible, an iced overlay covering the whole window
//! paints the dimmed backdrop (`.cn-alert-dialog-overlay`), the centered
//! surface (`.cn-alert-dialog-content`), and the footer buttons, and
//! forwards events to the interactive content. Action/cancel clicks and
//! <kbd>Esc</kbd> dismiss it, backdrop clicks are ignored by default
//! (`interactOutsideBehavior: "ignore"`), everything underneath stays
//! inert while open, and the entrance plays the web `fade-in-0 zoom-in-95`
//! animation.

use iced_core::keyboard;

use shadcn_common::{DIALOG_ZOOM_FROM, Easing};

use crate::iced_compat::advanced::renderer::Renderer as _;
use crate::iced_compat::advanced::widget::{Tree, tree};
use crate::iced_compat::advanced::{Clipboard, Shell, Widget, layout, overlay, renderer};
use crate::iced_compat::{
    Element, Event, Length, Point, Rectangle, Renderer, Size, Theme, Transformation, Vector, mouse,
    time::{Duration, Instant},
    touch, window,
};

use super::FooterChild;
use super::style::AlertDialogStyle;
use super::types::{AlertDialogSize, AlertDialogState};

/// Frame pacing while the open/close transition runs, matching the other
/// animated components.
const FRAME_INTERVAL: Duration = Duration::from_millis(16);

/// Internal widget produced by the [`super::AlertDialog`] builder.
pub(super) struct AlertDialogWidget<'a, Message> {
    pub(super) trigger: Element<'a, Message>,
    pub(super) surface: Element<'a, Message>,
    pub(super) footer: Vec<FooterChild<'a, Message>>,
    pub(super) footer_gap: f32,
    pub(super) size: AlertDialogSize,
    pub(super) max_width: f32,
    pub(super) margin: f32,
    pub(super) pad: f32,
    pub(super) gap: f32,
    pub(super) duration: Duration,
    pub(super) animated: bool,
    pub(super) disabled: bool,
    pub(super) open_override: Option<bool>,
    pub(super) default_open: bool,
    pub(super) on_open_change: Option<Box<dyn Fn(bool) -> Message + 'a>>,
    pub(super) on_open_change_complete: Option<Box<dyn Fn(bool) -> Message + 'a>>,
    pub(super) close_on_click_outside: bool,
    pub(super) close_on_escape: bool,
    pub(super) style: AlertDialogStyle,
}

impl<Message> AlertDialogWidget<'_, Message> {
    /// Synchronizes the effective open target with the uncontrolled intent
    /// and the controlled override, starting the transition on changes.
    fn sync_target(
        &self,
        state: &mut AlertDialogState,
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

            // A disabled animation settles instantly; report completion
            // here since `advance` will never observe a running frame.
            if !state.transition.is_running()
                && let Some(on_complete) = self.on_open_change_complete.as_ref()
            {
                shell.publish(on_complete(target));
            }

            shell.invalidate_layout();
            shell.request_redraw();
        }
    }

    /// Advances the open/close transition for the frame drawn at `now`.
    fn advance(&self, state: &mut AlertDialogState, now: Instant, shell: &mut Shell<'_, Message>) {
        let target = f32::from(u8::from(state.open));

        let was_running = state.transition.is_running();
        state
            .transition
            .advance(target, self.animated, self.duration, Easing::EaseInOut, now);

        if state.transition.is_running() {
            shell.request_redraw_at(now + FRAME_INTERVAL);
        } else if was_running {
            if !state.open {
                // The overlay unmounts once the exit animation ends.
                shell.invalidate_layout();
            }

            // `onOpenChangeComplete`: the animation just settled.
            if let Some(on_complete) = self.on_open_change_complete.as_ref() {
                shell.publish(on_complete(state.open));
            }
        }
    }

    /// Handles a press on the trigger: opens the dialog. While the dialog
    /// is open the modal overlay captures presses, so this only ever runs
    /// against a closed (or closing) dialog.
    fn handle_trigger_press(&self, state: &mut AlertDialogState, shell: &mut Shell<'_, Message>) {
        if self.disabled || state.open {
            return;
        }

        state.requested_open = true;

        if let Some(on_open_change) = self.on_open_change.as_ref() {
            shell.publish(on_open_change(true));
        }

        self.sync_target(state, Instant::now(), shell);
    }

    /// Widgets backing the trigger, surface, and footer buttons.
    fn child_widgets(&self) -> Vec<&Element<'_, Message>> {
        let mut children = vec![&self.trigger, &self.surface];
        children.extend(self.footer.iter().map(|child| &child.element));
        children
    }
}

impl<Message> Widget<Message, Theme, Renderer> for AlertDialogWidget<'_, Message> {
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
        tree::Tag::of::<AlertDialogState>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(AlertDialogState::new(self.default_open))
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
                    let state = tree.state.downcast_mut::<AlertDialogState>();
                    self.handle_trigger_press(state, shell);
                }
            }
            Event::Window(window::Event::RedrawRequested(now)) => {
                let state = tree.state.downcast_mut::<AlertDialogState>();

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
        let state = state.downcast_mut::<AlertDialogState>();
        let style = self.style;

        let (trigger_tree, rest) = children.split_first_mut().expect("trigger state");
        let (surface_tree, footer_trees) = rest.split_first_mut().expect("surface state");

        let trigger = self.trigger.as_widget_mut().overlay(
            trigger_tree,
            layout,
            renderer,
            viewport,
            translation,
        );

        let dialog = state.is_visible().then(|| {
            overlay::Element::new(Box::new(AlertDialogOverlay {
                surface: &mut self.surface,
                surface_tree,
                footer: self
                    .footer
                    .iter_mut()
                    .zip(footer_trees.iter_mut())
                    .collect(),
                footer_gap: self.footer_gap,
                size: self.size,
                state,
                max_width: self.max_width,
                margin: self.margin,
                pad: self.pad,
                gap: self.gap,
                style,
                on_open_change: self.on_open_change.as_deref(),
                close_on_click_outside: self.close_on_click_outside,
                close_on_escape: self.close_on_escape,
            }))
        });

        if trigger.is_some() || dialog.is_some() {
            Some(
                overlay::Group::with_children(trigger.into_iter().chain(dialog).collect())
                    .overlay(),
            )
        } else {
            None
        }
    }
}

/// Overlay that lays out, paints, and drives the modal alert dialog:
/// backdrop, centered surface, footer buttons, and interactive content.
struct AlertDialogOverlay<'a, 'b, Message> {
    surface: &'b mut Element<'a, Message>,
    surface_tree: &'b mut Tree,
    footer: Vec<(&'b mut FooterChild<'a, Message>, &'b mut Tree)>,
    footer_gap: f32,
    size: AlertDialogSize,
    state: &'b mut AlertDialogState,
    max_width: f32,
    margin: f32,
    pad: f32,
    gap: f32,
    style: AlertDialogStyle,
    on_open_change: Option<&'b (dyn Fn(bool) -> Message + 'a)>,
    close_on_click_outside: bool,
    close_on_escape: bool,
}

impl<Message> AlertDialogOverlay<'_, '_, Message> {
    /// Requests a close: drops the uncontrolled open intent — so the exit
    /// animation starts on the next frame — and publishes
    /// `onOpenChange(false)` for controlled consumers unless the dismissal
    /// source already did.
    ///
    /// Only reachable while the dialog is effectively open.
    fn request_close(&mut self, publish: bool, shell: &mut Shell<'_, Message>) {
        self.state.requested_open = false;

        if publish && let Some(on_open_change) = self.on_open_change {
            shell.publish(on_open_change(false));
        }

        shell.request_redraw();
    }
}

impl<Message> overlay::Overlay<Message, Theme, Renderer> for AlertDialogOverlay<'_, '_, Message> {
    fn layout(&mut self, renderer: &Renderer, bounds: Size) -> layout::Node {
        let width = (bounds.width - 2.0 * self.margin)
            .min(self.max_width)
            .max(0.0);
        let max_height = (bounds.height - 2.0 * self.margin).max(0.0);

        // Content column: `p-6` on top/left/right; the bottom padding is
        // deferred until after the footer block.
        let limits = layout::Limits::new(Size::ZERO, Size::new(width, max_height));
        let content_node =
            self.surface
                .as_widget_mut()
                .layout(self.surface_tree, renderer, &limits);
        let content_size = content_node.size();

        // Footer block: `sm:flex-row sm:justify-end gap-2`, or the
        // `grid grid-cols-2 gap-2` of `size="sm"`.
        let inner_width = (width - 2.0 * self.pad).max(0.0);
        let mut footer_nodes = Vec::with_capacity(self.footer.len());
        let mut footer_height = 0.0f32;

        match self.size {
            AlertDialogSize::Default => {
                for (child, tree) in &mut self.footer {
                    let limits =
                        layout::Limits::new(Size::ZERO, Size::new(inner_width, max_height));
                    let node = child
                        .element
                        .as_widget_mut()
                        .layout(tree, renderer, &limits);

                    footer_height = footer_height.max(node.size().height);
                    footer_nodes.push(node);
                }

                // Right-aligned row.
                let total: f32 = footer_nodes.iter().map(|node| node.size().width).sum();
                let gaps = self.footer_gap * footer_nodes.len().saturating_sub(1) as f32;
                let mut x = (width - self.pad - total - gaps).max(self.pad);
                let y = content_size.height + self.gap;

                footer_nodes = footer_nodes
                    .into_iter()
                    .map(|node| {
                        let node_size = node.size();
                        let node = node
                            .move_to(Point::new(x, y + (footer_height - node_size.height) / 2.0));
                        x += node_size.width + self.footer_gap;
                        node
                    })
                    .collect();
            }
            AlertDialogSize::Sm => {
                let column_width = ((inner_width - self.footer_gap) / 2.0).max(0.0);

                for (child, tree) in &mut self.footer {
                    let limits = layout::Limits::new(
                        Size::new(column_width, 0.0),
                        Size::new(column_width, max_height),
                    );
                    let node = child
                        .element
                        .as_widget_mut()
                        .layout(tree, renderer, &limits);

                    footer_height = footer_height.max(node.size().height);
                    footer_nodes.push(node);
                }

                // Two-column grid; extra items wrap onto new rows.
                let rows = footer_nodes.len().div_ceil(2);
                let y = content_size.height + self.gap;

                footer_nodes = footer_nodes
                    .into_iter()
                    .enumerate()
                    .map(|(index, node)| {
                        let row = (index / 2) as f32;
                        let column = (index % 2) as f32;

                        node.move_to(Point::new(
                            self.pad + column * (column_width + self.footer_gap),
                            row.mul_add(footer_height + self.footer_gap, y),
                        ))
                    })
                    .collect();

                if rows > 1 {
                    footer_height =
                        (rows as f32).mul_add(footer_height, self.footer_gap * (rows - 1) as f32);
                }
            }
        }

        let surface_height = if footer_nodes.is_empty() {
            content_size.height
        } else {
            content_size.height + self.gap + footer_height + self.pad
        };

        // `fixed top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2`.
        let x = ((bounds.width - width) / 2.0).max(0.0);
        let y = ((bounds.height - surface_height) / 2.0).max(0.0);

        let mut children = vec![content_node];
        children.extend(footer_nodes);

        let surface_node = layout::Node::with_children(Size::new(width, surface_height), children)
            .move_to(Point::new(x, y));

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
        // The dialog goes inert as soon as it starts closing, like the
        // web layer that unmounts while `animate-out` plays.
        if !self.state.open {
            return;
        }

        let surface_layout = layout.children().next().expect("surface layout");
        let surface_bounds = surface_layout.bounds();

        let mut inner = surface_layout.children();
        let content_layout = inner.next().expect("content layout");
        let footer_layouts: Vec<_> = inner.collect();

        self.surface.as_widget_mut().update(
            self.surface_tree,
            event,
            content_layout,
            cursor,
            renderer,
            clipboard,
            shell,
            &content_layout.bounds(),
        );

        for ((child, tree), child_layout) in self.footer.iter_mut().zip(&footer_layouts) {
            child.element.as_widget_mut().update(
                tree,
                event,
                *child_layout,
                cursor,
                renderer,
                clipboard,
                shell,
                &child_layout.bounds(),
            );
        }

        let footer_under_cursor = footer_layouts
            .iter()
            .position(|child_layout| cursor.is_over(child_layout.bounds()));

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if let Some(index) = footer_under_cursor {
                    self.state.pressed_footer = Some(index);
                    shell.capture_event();
                } else if cursor.is_over(surface_bounds) {
                    // Presses inside the surface stay inside: nothing
                    // underneath may react or dismiss.
                    shell.capture_event();
                } else {
                    // `interactOutsideBehavior`: ignored by default.
                    if self.close_on_click_outside {
                        self.request_close(true, shell);
                    }

                    shell.capture_event();
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. }) => {
                let pressed = self.state.pressed_footer.take();

                // Mirror the button click contract: the press and the
                // release must both land on the same footer child. The
                // child's own `on_press` published during the forwarding
                // above; dismissal follows here.
                if let Some(index) = pressed
                    && footer_under_cursor == Some(index)
                    && let Some((child, _)) = self.footer.get(index)
                    && child.dismisses
                {
                    let publish = !child.publishes_open_change;
                    self.request_close(publish, shell);
                }

                shell.capture_event();
            }
            Event::Keyboard(keyboard::Event::KeyPressed {
                key: keyboard::Key::Named(keyboard::key::Named::Escape),
                ..
            }) => {
                if self.close_on_escape {
                    self.request_close(true, shell);
                }

                shell.capture_event();
            }
            Event::Mouse(mouse::Event::CursorMoved { .. }) => {
                // Footer button hover fills follow the cursor directly.
                shell.request_redraw();
                shell.capture_event();
            }
            Event::Mouse(_) | Event::Touch(_) | Event::Keyboard(_) => {
                // Modal: scrolling, releases, and typing never reach the
                // window underneath while the dialog is open.
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
        let surface_layout = layout.children().next().expect("surface layout");
        let mut inner = surface_layout.children();
        let content_layout = inner.next().expect("content layout");

        self.surface.as_widget_mut().operate(
            self.surface_tree,
            content_layout,
            renderer,
            operation,
        );

        for ((child, tree), child_layout) in self.footer.iter_mut().zip(inner) {
            child
                .element
                .as_widget_mut()
                .operate(tree, child_layout, renderer, operation);
        }
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
        let mut inner = surface_layout.children();
        let content_layout = inner.next().expect("content layout");

        for ((child, tree), child_layout) in self.footer.iter().zip(inner) {
            if cursor.is_over(child_layout.bounds()) {
                return child.element.as_widget().mouse_interaction(
                    tree,
                    child_layout,
                    cursor,
                    &child_layout.bounds(),
                    renderer,
                );
            }
        }

        self.surface.as_widget().mouse_interaction(
            self.surface_tree,
            content_layout,
            cursor,
            &content_layout.bounds(),
            renderer,
        )
    }

    fn overlay<'c>(
        &'c mut self,
        layout: layout::Layout<'c>,
        renderer: &Renderer,
    ) -> Option<overlay::Element<'c, Message, Theme, Renderer>> {
        let surface_layout = layout.children().next().expect("surface layout");
        let content_layout = surface_layout.children().next().expect("content layout");

        self.surface.as_widget_mut().overlay(
            self.surface_tree,
            content_layout,
            renderer,
            &content_layout.bounds(),
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

        // `.cn-alert-dialog-overlay`: `fixed inset-0 bg-black/N fade-in-0`.
        renderer.fill_quad(
            renderer::Quad {
                bounds,
                ..renderer::Quad::default()
            },
            self.style.overlay.scale_alpha(progress),
        );

        let surface_layout = layout.children().next().expect("surface layout");
        let surface_bounds = surface_layout.bounds();

        let mut inner = surface_layout.children();
        let content_layout = inner.next().expect("content layout");

        let origin = Point::new(surface_bounds.center_x(), surface_bounds.center_y());
        let scale = DIALOG_ZOOM_FROM + (1.0 - DIALOG_ZOOM_FROM) * progress;

        // `zoom-in-95` about the surface center (`--transform-origin`).
        let transform = Transformation::translate(origin.x, origin.y)
            * Transformation::scale(scale)
            * Transformation::translate(-origin.x, -origin.y);

        renderer.with_transformation(transform, |renderer| {
            crate::floating_surface::fill_floating_surface(
                renderer,
                surface_bounds,
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

            self.surface.as_widget().draw(
                self.surface_tree,
                renderer,
                theme,
                &defaults,
                content_layout,
                cursor,
                &content_layout.bounds(),
            );

            for ((child, tree), child_layout) in self.footer.iter().zip(inner) {
                child.element.as_widget().draw(
                    tree,
                    renderer,
                    theme,
                    &defaults,
                    child_layout,
                    cursor,
                    &child_layout.bounds(),
                );
            }

            crate::floating_surface::paint_outside_ring(
                renderer,
                surface_bounds,
                self.style.border_color.scale_alpha(progress),
                self.style.border_width,
                self.style.radius,
            );
        });
    }
}
