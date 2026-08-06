//! Custom layout widget and pointer handling for [`super::ResizablePaneGroup`].

use crate::iced_compat::advanced::layout::{self, Layout};
use crate::iced_compat::advanced::widget::{Operation, Tree, tree};
use crate::iced_compat::advanced::{Clipboard, Shell, Widget, overlay, renderer};
use crate::iced_compat::widget::container as container_widget;
use crate::iced_compat::{
    Element, Event, Length, Point, Rectangle, Size, Vector, mouse, touch, window,
};

use super::geometry::{self, HANDLE_LAYOUT_PX, hit_bounds, pane_area_px, resize_pair};
use super::style::{self, HandleStyle};
use super::types::{HandleConfig, PaneConstraints, ResizableDirection, ResizableLayout};
use super::{ResizableBuildError, ResizablePaneGroup};
use crate::theme::Theme;

/// Builds a resizable pane group as an iced [`Element`](iced_core::Element).
pub fn resizable_pane_group<'a, Message>(
    group: ResizablePaneGroup<'a, Message>,
) -> Result<Element<'a, Message>, ResizableBuildError>
where
    Message: Clone + 'a,
{
    let ResizablePaneGroup {
        theme,
        direction,
        sizes,
        slots,
        width,
        height,
        padding,
        bordered,
        radius,
        on_layout_change,
        on_dragging_change,
        style_override,
    } = group;

    let (panes, constraints, handles) = parse_slots(slots)?;

    let layout_sizes = sizes
        .map(|layout| layout.0)
        .unwrap_or_else(|| geometry::default_layout(&constraints));

    if layout_sizes.len() != panes.len() {
        return Err(ResizableBuildError::InvalidSlotSequence);
    }

    let widget = ResizablePaneGroupWidget {
        theme,
        direction,
        panes,
        constraints,
        handles,
        sizes: layout_sizes,
        on_layout_change,
        on_dragging_change,
    };

    let mut element = Element::new(widget);

    let frame_style = style::resolve_frame_style(theme, bordered, radius);
    let style_override = style_override;

    element = container_widget(element)
        .width(width)
        .height(height)
        .padding(padding.unwrap_or_default())
        .style(move |_iced_theme| {
            let mut resolved = frame_style;
            if let Some(override_fn) = style_override.as_ref() {
                resolved = override_fn(resolved);
            }
            resolved
        })
        .into();

    Ok(element)
}

type ParsedSlots<'a, Message> = (
    Vec<Element<'a, Message>>,
    Vec<PaneConstraints>,
    Vec<HandleConfig>,
);

fn parse_slots<'a, Message>(
    slots: Vec<super::ResizableSlot<'a, Message>>,
) -> Result<ParsedSlots<'a, Message>, ResizableBuildError> {
    if slots.is_empty() {
        return Err(ResizableBuildError::EmptyPaneGroup);
    }

    if slots.len() == 1 {
        let only = slots.into_iter().next().expect("length checked");
        return match only {
            super::ResizableSlot::Pane(pane) => {
                Ok((vec![pane.content], vec![pane.constraints], Vec::new()))
            }
            super::ResizableSlot::Handle(_) => Err(ResizableBuildError::InvalidSlotSequence),
        };
    }

    let mut panes = Vec::new();
    let mut constraints = Vec::new();
    let mut handles = Vec::new();
    let mut iter = slots.into_iter();

    while let Some(slot) = iter.next() {
        let super::ResizableSlot::Pane(pane) = slot else {
            return Err(ResizableBuildError::InvalidSlotSequence);
        };

        constraints.push(pane.constraints);
        panes.push(pane.content);

        match iter.next() {
            Some(super::ResizableSlot::Handle(handle)) => handles.push(handle.config),
            None => break,
            Some(super::ResizableSlot::Pane(_)) => {
                return Err(ResizableBuildError::InvalidSlotSequence);
            }
        }
    }

    if panes.is_empty() {
        return Err(ResizableBuildError::EmptyPaneGroup);
    }

    if panes.len() > 1 && handles.len() != panes.len() - 1 {
        return Err(ResizableBuildError::InvalidSlotSequence);
    }

    Ok((panes, constraints, handles))
}

struct ResizablePaneGroupWidget<'a, Message> {
    theme: &'a Theme,
    direction: ResizableDirection,
    panes: Vec<Element<'a, Message>>,
    constraints: Vec<PaneConstraints>,
    handles: Vec<HandleConfig>,
    sizes: Vec<f32>,
    on_layout_change: Option<Box<dyn Fn(ResizableLayout) -> Message + 'a>>,
    on_dragging_change: Option<Box<dyn Fn(bool) -> Message + 'a>>,
}

#[derive(Debug, Default)]
struct ResizableState {
    dragging: Option<usize>,
    last_pointer: Option<Point>,
    hovered_handle: Option<usize>,
    dragging_active: bool,
}

impl<'a, Message> ResizablePaneGroupWidget<'a, Message> {
    fn handle_style(&self) -> HandleStyle {
        style::resolve_handle_style(self.theme)
    }

    fn layout_children(&self, size: Size) -> (Vec<Rectangle>, Vec<Rectangle>, f32) {
        let handle_count = self.handles.len();
        let axis = if self.direction.is_horizontal() {
            size.width
        } else {
            size.height
        };
        let pane_area = pane_area_px(axis, handle_count);

        let mut pane_bounds = Vec::with_capacity(self.panes.len());
        let mut handle_bounds = Vec::with_capacity(handle_count);
        let mut offset = 0.0;

        for (index, &portion) in self.sizes.iter().enumerate() {
            let extent = pane_area * portion / 100.0;

            let bounds = match self.direction {
                ResizableDirection::Horizontal => Rectangle {
                    x: offset,
                    y: 0.0,
                    width: extent,
                    height: size.height,
                },
                ResizableDirection::Vertical => Rectangle {
                    x: 0.0,
                    y: offset,
                    width: size.width,
                    height: extent,
                },
            };

            pane_bounds.push(bounds);
            offset += extent;

            if index < handle_count {
                let handle = match self.direction {
                    ResizableDirection::Horizontal => Rectangle {
                        x: offset,
                        y: 0.0,
                        width: HANDLE_LAYOUT_PX,
                        height: size.height,
                    },
                    ResizableDirection::Vertical => Rectangle {
                        x: 0.0,
                        y: offset,
                        width: size.width,
                        height: HANDLE_LAYOUT_PX,
                    },
                };
                handle_bounds.push(handle);
                offset += HANDLE_LAYOUT_PX;
            }
        }

        (pane_bounds, handle_bounds, pane_area)
    }

    fn handle_at(
        &self,
        cursor: mouse::Cursor,
        layout: Layout<'_>,
        viewport: &Rectangle,
    ) -> Option<usize> {
        let bounds = layout.bounds();
        let position = cursor.position().and_then(|point| {
            if bounds.contains(point) {
                Some(Point::new(point.x - bounds.x, point.y - bounds.y))
            } else {
                None
            }
        })?;

        let (_, handle_bounds, _) = self.layout_children(bounds.size());

        for (index, visual) in handle_bounds.iter().enumerate() {
            if self.handles[index].disabled {
                continue;
            }

            let hit = hit_bounds(*visual, self.direction);
            if hit.contains(position) {
                return Some(index);
            }
        }

        let _ = viewport;
        None
    }

    fn publish_layout(&self, shell: &mut Shell<'_, Message>, sizes: Vec<f32>) {
        if let Some(callback) = self.on_layout_change.as_ref() {
            shell.publish(callback(ResizableLayout(sizes)));
        }
    }

    fn set_dragging(&self, shell: &mut Shell<'_, Message>, dragging: bool) {
        if let Some(callback) = self.on_dragging_change.as_ref() {
            shell.publish(callback(dragging));
        }
    }
}

impl<Message> Widget<Message, crate::iced_compat::Theme, crate::iced_compat::Renderer>
    for ResizablePaneGroupWidget<'_, Message>
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<ResizableState>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(ResizableState::default())
    }

    fn children(&self) -> Vec<Tree> {
        self.panes.iter().map(Tree::new).collect()
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&self.panes);
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Fill,
            height: Length::Fill,
        }
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &crate::iced_compat::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let size = self.size();
        let bounds = limits.resolve(size.width, size.height, Size::ZERO);
        let (pane_bounds, _, _) = self.layout_children(bounds);

        let mut children = Vec::with_capacity(self.panes.len());
        for (index, pane) in self.panes.iter_mut().enumerate() {
            let pane_limits = layout::Limits::new(
                Size::new(pane_bounds[index].width, pane_bounds[index].height),
                Size::new(pane_bounds[index].width, pane_bounds[index].height),
            );
            let child = pane
                .as_widget_mut()
                .layout(&mut tree.children[index], renderer, &pane_limits)
                .move_to(Point::new(pane_bounds[index].x, pane_bounds[index].y));
            children.push(child);
        }

        layout::Node::with_children(bounds, children)
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &crate::iced_compat::Renderer,
        operation: &mut dyn Operation,
    ) {
        for (index, child_layout) in layout.children().enumerate() {
            self.panes[index].as_widget_mut().operate(
                &mut tree.children[index],
                child_layout,
                renderer,
                operation,
            );
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
        let state = tree.state.downcast_mut::<ResizableState>();
        let bounds = layout.bounds();
        let (_, handle_bounds, pane_area) = self.layout_children(bounds.size());

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                if let Some(index) = self.handle_at(cursor, layout, viewport)
                    && let Some(position) = cursor.position()
                {
                    state.dragging = Some(index);
                    state.last_pointer = Some(position);
                    if !state.dragging_active {
                        state.dragging_active = true;
                        self.set_dragging(shell, true);
                    }
                    shell.capture_event();
                    return;
                }
            }
            Event::Touch(touch::Event::FingerPressed { position, .. }) => {
                if bounds.contains(*position) {
                    let local = Point::new(position.x - bounds.x, position.y - bounds.y);
                    for (index, visual) in handle_bounds.iter().enumerate() {
                        if self.handles[index].disabled {
                            continue;
                        }
                        if hit_bounds(*visual, self.direction).contains(local) {
                            state.dragging = Some(index);
                            state.last_pointer = Some(*position);
                            if !state.dragging_active {
                                state.dragging_active = true;
                                self.set_dragging(shell, true);
                            }
                            shell.capture_event();
                            return;
                        }
                    }
                }
            }
            Event::Mouse(mouse::Event::CursorMoved { .. })
            | Event::Touch(touch::Event::FingerMoved { .. }) => {
                if let Some(index) = state.dragging {
                    let current = cursor.position().or(state.last_pointer);
                    if let (Some(current), Some(previous)) = (current, state.last_pointer) {
                        let delta = match self.direction {
                            ResizableDirection::Horizontal => current.x - previous.x,
                            ResizableDirection::Vertical => current.y - previous.y,
                        };

                        let mut next = self.sizes.clone();
                        if resize_pair(&mut next, &self.constraints, index, delta, pane_area) {
                            self.sizes = next.clone();
                            self.publish_layout(shell, next);
                        }

                        state.last_pointer = Some(current);
                        shell.request_redraw();
                        return;
                    }
                }

                let hovered = self.handle_at(cursor, layout, viewport);
                if state.hovered_handle != hovered {
                    state.hovered_handle = hovered;
                    shell.request_redraw();
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. })
            | Event::Touch(touch::Event::FingerLost { .. }) => {
                if state.dragging.is_some() {
                    state.dragging = None;
                    state.last_pointer = None;
                    if state.dragging_active {
                        state.dragging_active = false;
                        self.set_dragging(shell, false);
                    }
                    shell.request_redraw();
                    return;
                }
            }
            Event::Window(window::Event::RedrawRequested(_)) => {
                state.hovered_handle = self.handle_at(cursor, layout, viewport);
            }
            _ => {}
        }

        for (index, child_layout) in layout.children().enumerate() {
            self.panes[index].as_widget_mut().update(
                &mut tree.children[index],
                event,
                child_layout,
                cursor,
                renderer,
                clipboard,
                shell,
                viewport,
            );
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
        let state = tree.state.downcast_ref::<ResizableState>();

        if state.dragging.is_some() {
            return match self.direction {
                ResizableDirection::Horizontal => mouse::Interaction::ResizingHorizontally,
                ResizableDirection::Vertical => mouse::Interaction::ResizingVertically,
            };
        }

        if self.handle_at(cursor, layout, viewport).is_some() {
            return match self.direction {
                ResizableDirection::Horizontal => mouse::Interaction::ResizingHorizontally,
                ResizableDirection::Vertical => mouse::Interaction::ResizingVertically,
            };
        }

        for (index, child_layout) in layout.children().enumerate() {
            let interaction = self.panes[index].as_widget().mouse_interaction(
                &tree.children[index],
                child_layout,
                cursor,
                viewport,
                renderer,
            );

            if interaction != mouse::Interaction::default() {
                return interaction;
            }
        }

        mouse::Interaction::default()
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
        let state = tree.state.downcast_ref::<ResizableState>();
        let bounds = layout.bounds();

        if !bounds.intersects(viewport) {
            return;
        }

        let (_, handle_bounds, _) = self.layout_children(bounds.size());
        let handle_style = self.handle_style();

        for (index, child_layout) in layout.children().enumerate() {
            self.panes[index].as_widget().draw(
                &tree.children[index],
                renderer,
                theme,
                style,
                child_layout,
                cursor,
                viewport,
            );
        }

        for (index, visual) in handle_bounds.into_iter().enumerate() {
            let absolute = Rectangle {
                x: bounds.x + visual.x,
                y: bounds.y + visual.y,
                width: visual.width,
                height: visual.height,
            };
            style::draw_divider(renderer, absolute, &handle_style);

            if self.handles[index].with_handle {
                style::draw_grip(renderer, absolute, self.direction, &handle_style);
            }

            if state.hovered_handle == Some(index) || state.dragging == Some(index) {
                let hit = hit_bounds(absolute, self.direction);
                style::draw_focus_ring(renderer, hit, &handle_style);
            }
        }
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
        overlay::from_children(
            &mut self.panes,
            tree,
            layout,
            renderer,
            viewport,
            translation,
        )
    }
}
