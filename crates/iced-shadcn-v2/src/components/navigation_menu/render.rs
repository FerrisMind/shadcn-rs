//! Custom widget and overlay rendering for [`super::NavigationMenu`].
//!
//! Ports the bits-ui / shadcn-svelte navigation menu state machine: hover
//! opens a trigger after `delayDuration`, switching between open items uses
//! a shortened delay, leaving both the trigger list and the floating panel
//! arms `closeDelay`, click toggles, Esc closes, and arrow keys move roving
//! focus. Content is painted either in a shared viewport panel or as a
//! per-item floating surface (`viewport=false`).

use iced_core::keyboard;
use iced_core::keyboard::key::{self, Key};

use shadcn_common::{
    NAVIGATION_MENU_INDICATOR_ANIM_MS, NAVIGATION_MENU_MOTION_ANIM_MS, NavRect,
    first_enabled_index, last_enabled_index, motion_offset_x, place_navigation_menu_content,
    place_navigation_menu_viewport, step_index,
};

use crate::iced_compat::advanced::renderer::{self, Renderer as _};
use crate::iced_compat::advanced::widget::{Operation, Tree, tree};
use crate::iced_compat::advanced::{Clipboard, Shell, Widget, layout, overlay};
use crate::iced_compat::widget::button;
use crate::iced_compat::widget::canvas;
use crate::iced_compat::widget::graphics::geometry::Renderer as _;
use crate::iced_compat::{
    Background, Border, Color, Element, Event, Length, Padding, Point, Rectangle, Renderer, Size,
    Theme as IcedTheme, Vector, mouse, time::Duration, time::Instant, touch, window,
};
use crate::theme::Theme;

use super::style::{
    NavigationMenuViewportStyle, metrics, paint_item_surface, paint_viewport_ring,
    paint_viewport_surface, resolve_link_style,
};
use super::types::{
    Motion, NavigationMenuContentProps, NavigationMenuJustify, NavigationMenuLinkProps,
    NavigationMenuLinkState, NavigationMenuListProps, NavigationMenuOrientation,
    NavigationMenuProps, NavigationMenuState, NavigationMenuTriggerState, NavigationMenuWrap,
    PendingOpen,
};

/// Item kind stored alongside each top-level entry.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum NavItemKind {
    Trigger,
    Link,
}

/// Metadata for one top-level list entry.
pub(super) struct NavItemMeta<Message> {
    pub(super) value: String,
    pub(super) kind: NavItemKind,
    pub(super) disabled: bool,
    pub(super) content_index: Option<usize>,
    pub(super) content_props: NavigationMenuContentProps,
    pub(super) link_message: Option<Message>,
}

/// Root widget produced by [`super::NavigationMenu`].
pub(super) struct NavigationMenuWidget<'a, Message> {
    pub(super) triggers: Vec<Element<'a, Message>>,
    pub(super) contents: Vec<Element<'a, Message>>,
    pub(super) items: Vec<NavItemMeta<Message>>,
    pub(super) value: Option<String>,
    pub(super) on_value_change: Option<Box<dyn Fn(String) -> Message + 'a>>,
    pub(super) root_props: NavigationMenuProps,
    pub(super) list_props: NavigationMenuListProps,
    pub(super) theme: Theme,
    pub(super) viewport_style: NavigationMenuViewportStyle,
    pub(super) content_style: NavigationMenuViewportStyle,
}

impl<'a, Message> NavigationMenuWidget<'a, Message> {
    fn is_controlled(&self) -> bool {
        self.value.is_some()
    }

    fn current_value<'b>(&'b self, state: &'b NavigationMenuState) -> Option<&'b str> {
        self.value
            .as_deref()
            .or(state.open_value.as_deref())
            .filter(|val| !val.is_empty())
    }

    fn open_index(&self, state: &NavigationMenuState) -> Option<usize> {
        resolve_open_index(&self.items, self.current_value(state))
    }

    fn set_open_value(
        &self,
        state: &mut NavigationMenuState,
        shell: &mut Shell<'_, Message>,
        next: Option<String>,
    ) {
        let next_value = next.clone().unwrap_or_default();
        let current = self.current_value(state).unwrap_or("");
        if current == next_value {
            return;
        }

        if !self.is_controlled() {
            state.open_value = next;
        }

        if let Some(on_change) = self.on_value_change.as_ref() {
            shell.publish((on_change)(next_value));
        }
    }

    fn elapsed_since_close_ms(&self, state: &NavigationMenuState, now: Instant) -> Option<u64> {
        state
            .last_close_at
            .map(|last| now.saturating_duration_since(last).as_millis() as u64)
    }
}

impl<Message> Widget<Message, IcedTheme, Renderer> for NavigationMenuWidget<'_, Message>
where
    Message: Clone,
{
    fn children(&self) -> Vec<Tree> {
        let mut children: Vec<Tree> = self.triggers.iter().map(Tree::new).collect();
        children.extend(self.contents.iter().map(Tree::new));
        children
    }

    fn diff(&self, tree: &mut Tree) {
        let mut children: Vec<&dyn Widget<Message, IcedTheme, Renderer>> = self
            .triggers
            .iter()
            .map(|child| child.as_widget())
            .collect();
        children.extend(self.contents.iter().map(|child| child.as_widget()));
        tree.diff_children(children.as_slice());
    }

    fn state(&self) -> tree::State {
        tree::State::new(NavigationMenuState::default())
    }

    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<NavigationMenuState>()
    }

    fn size(&self) -> Size<Length> {
        Size::new(Length::Fill, Length::Shrink)
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let metrics = metrics(self.list_props.padding, self.list_props.gap, &self.theme);
        let max = limits.max();
        let count = self.triggers.len();
        let mut child_nodes = Vec::with_capacity(count);

        let full_width_each = if self.list_props.full_width
            && matches!(
                self.root_props.orientation,
                NavigationMenuOrientation::Horizontal
            )
            && matches!(self.list_props.wrap, NavigationMenuWrap::NoWrap)
            && count > 0
        {
            let available = (max.width
                - metrics.list_padding * 2.0
                - metrics.gap * (count.saturating_sub(1) as f32))
                .max(0.0);
            available / count as f32
        } else {
            0.0
        };

        for (index, trigger) in self.triggers.iter_mut().enumerate() {
            let child_limits = if full_width_each > 0.0 {
                layout::Limits::new(
                    Size::new(full_width_each, 0.0),
                    Size::new(full_width_each, max.height),
                )
            } else {
                layout::Limits::new(Size::ZERO, max)
            };
            let node =
                trigger
                    .as_widget_mut()
                    .layout(&mut tree.children[index], renderer, &child_limits);
            child_nodes.push(node);
        }

        let mut lines: Vec<Line> = Vec::new();
        let content_width = (max.width - metrics.list_padding * 2.0).max(0.0);

        match self.root_props.orientation {
            NavigationMenuOrientation::Horizontal => {
                let mut current = Line::default();
                for (index, node) in child_nodes.iter().enumerate() {
                    let size = node.size();
                    let next_width = if current.indices.is_empty() {
                        size.width
                    } else {
                        current.width + metrics.gap + size.width
                    };

                    let wrap = matches!(
                        self.list_props.wrap,
                        NavigationMenuWrap::Wrap | NavigationMenuWrap::WrapReverse
                    ) && !current.indices.is_empty()
                        && next_width > content_width
                        && content_width > 0.0;

                    if wrap {
                        lines.push(std::mem::take(&mut current));
                    }

                    if current.indices.is_empty() {
                        current.width = size.width;
                    } else {
                        current.width += metrics.gap + size.width;
                    }
                    current.height = current.height.max(size.height);
                    current.indices.push(index);
                }
                if !current.indices.is_empty() {
                    lines.push(current);
                }

                if matches!(self.list_props.wrap, NavigationMenuWrap::WrapReverse) {
                    lines.reverse();
                }
            }
            NavigationMenuOrientation::Vertical => {
                for (index, node) in child_nodes.iter().enumerate() {
                    let size = node.size();
                    lines.push(Line {
                        indices: vec![index],
                        width: size.width,
                        height: size.height,
                    });
                }
            }
        }

        let mut total_height = metrics.list_padding * 2.0;
        let mut total_width = metrics.list_padding * 2.0;
        for (line_index, line) in lines.iter().enumerate() {
            total_height += line.height;
            total_width = total_width.max(line.width + metrics.list_padding * 2.0);
            if line_index + 1 < lines.len() {
                total_height += match self.root_props.orientation {
                    NavigationMenuOrientation::Horizontal => metrics.line_gap,
                    NavigationMenuOrientation::Vertical => metrics.gap,
                };
            }
        }

        let size = Size::new(
            if matches!(
                self.root_props.orientation,
                NavigationMenuOrientation::Horizontal
            ) {
                max.width
            } else {
                total_width.min(max.width)
            },
            total_height.min(max.height),
        );

        let mut trigger_bounds = vec![Rectangle::default(); count];
        let mut y = metrics.list_padding;

        for (line_index, line) in lines.iter().enumerate() {
            let line_space = (size.width - metrics.list_padding * 2.0 - line.width).max(0.0);
            let offset = match self.list_props.justify {
                NavigationMenuJustify::Start => 0.0,
                NavigationMenuJustify::Center => line_space / 2.0,
                NavigationMenuJustify::End => line_space,
            };

            let mut x = metrics.list_padding + offset;
            for (pos, index) in line.indices.iter().enumerate() {
                let node_size = child_nodes[*index].size();
                let center_offset = (line.height - node_size.height).max(0.0) / 2.0;
                let child_y = y + center_offset;

                let node =
                    std::mem::replace(&mut child_nodes[*index], layout::Node::new(Size::ZERO));
                child_nodes[*index] = node.move_to(Point::new(x, child_y));
                trigger_bounds[*index] = child_nodes[*index].bounds();

                if pos + 1 < line.indices.len() {
                    x += node_size.width + metrics.gap;
                }
            }

            if line_index + 1 < lines.len() {
                let gap = match self.root_props.orientation {
                    NavigationMenuOrientation::Horizontal => metrics.line_gap,
                    NavigationMenuOrientation::Vertical => metrics.gap,
                };
                y += line.height + gap;
            }
        }

        let state = tree.state.downcast_mut::<NavigationMenuState>();
        state.trigger_bounds = trigger_bounds;

        layout::Node::with_children(size, child_nodes)
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
        let state = tree.state.downcast_mut::<NavigationMenuState>();

        if !state.initialized {
            state.initialized = true;
            if !self.is_controlled()
                && state.open_value.is_none()
                && let Some(default) = self.root_props.default_value
            {
                state.open_value = Some(default.to_string());
            }
        }

        for (index, child) in self.triggers.iter_mut().enumerate() {
            if let Some(child_layout) = layout.children().nth(index) {
                child.as_widget_mut().update(
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

        let bounds = layout.bounds();
        let open_index = self.open_index(state);
        if open_index.is_none() {
            state.viewport_hovered = false;
            state.viewport_bounds = None;
            state.viewport_size = None;
        }

        let hovered_index = hit_test_trigger(&state.trigger_bounds, bounds, cursor);
        state.hovered_index = hovered_index;
        let resolved_viewport_bounds =
            resolve_viewport_bounds(self, state, bounds, *viewport).or(state.viewport_bounds);
        let over_viewport = resolved_viewport_bounds
            .map(|rect| cursor.is_over(rect))
            .unwrap_or(false)
            || state.viewport_hovered;
        let over_bridge = bridge_hovered(bounds, resolved_viewport_bounds, cursor);

        if over_viewport || over_bridge {
            state.pending_close = None;
        }

        match event {
            Event::Mouse(mouse::Event::CursorMoved { .. })
            | Event::Touch(touch::Event::FingerMoved { .. }) => {
                handle_hover(
                    self,
                    state,
                    hovered_index,
                    bounds,
                    resolved_viewport_bounds,
                    cursor,
                    shell,
                );
            }
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if cursor.is_over(bounds) {
                    state.focused = true;
                    state.focus_visible = false;
                    state.focused_index = hovered_index.or(open_index);
                } else if state.focused {
                    state.focused = false;
                }

                if let Some(index) = hovered_index
                    && let Some(item) = self.items.get(index)
                {
                    match item.kind {
                        NavItemKind::Trigger if !item.disabled => {
                            let current = self.current_value(state).unwrap_or("");
                            if current == item.value {
                                state.pending_open = None;
                                self.set_open_value(state, shell, None);
                                state.last_close_at = Some(Instant::now());
                            } else {
                                state.pending_open = None;
                                self.set_open_value(state, shell, Some(item.value.clone()));
                            }
                            shell.capture_event();
                        }
                        NavItemKind::Link => {
                            state.pending_open = None;
                            self.set_open_value(state, shell, None);
                            state.last_close_at = Some(Instant::now());
                        }
                        NavItemKind::Trigger => {}
                    }
                }
            }
            Event::Keyboard(keyboard::Event::KeyPressed { key, .. }) => {
                if !state.focused {
                    return;
                }

                state.focus_visible = true;
                let current = state.focused_index.or(open_index);
                let horizontal = matches!(
                    self.root_props.orientation,
                    NavigationMenuOrientation::Horizontal
                );

                let next = match key {
                    Key::Named(key::Named::ArrowRight) if horizontal => {
                        step_index(&self.items, current, 1, true, |item| !item.disabled)
                    }
                    Key::Named(key::Named::ArrowLeft) if horizontal => {
                        step_index(&self.items, current, -1, true, |item| !item.disabled)
                    }
                    Key::Named(key::Named::ArrowDown) if !horizontal => {
                        step_index(&self.items, current, 1, true, |item| !item.disabled)
                    }
                    Key::Named(key::Named::ArrowUp) if !horizontal => {
                        step_index(&self.items, current, -1, true, |item| !item.disabled)
                    }
                    Key::Named(key::Named::Home) => {
                        first_enabled_index(&self.items, |item| !item.disabled)
                    }
                    Key::Named(key::Named::End) => {
                        last_enabled_index(&self.items, |item| !item.disabled)
                    }
                    Key::Named(key::Named::Escape) => {
                        self.set_open_value(state, shell, None);
                        state.last_close_at = Some(Instant::now());
                        shell.capture_event();
                        None
                    }
                    Key::Named(key::Named::Enter) | Key::Named(key::Named::Space) => current,
                    _ => None,
                };

                if let Some(next_index) = next {
                    state.focused_index = Some(next_index);
                    if matches!(
                        key,
                        Key::Named(key::Named::Enter) | Key::Named(key::Named::Space)
                    ) {
                        if let Some(item) = self.items.get(next_index) {
                            match item.kind {
                                NavItemKind::Trigger if !item.disabled => {
                                    let current_value = self.current_value(state).unwrap_or("");
                                    if current_value == item.value {
                                        self.set_open_value(state, shell, None);
                                        state.last_close_at = Some(Instant::now());
                                    } else {
                                        self.set_open_value(state, shell, Some(item.value.clone()));
                                    }
                                }
                                NavItemKind::Link => {
                                    if let Some(message) = item.link_message.clone() {
                                        shell.publish(message);
                                    }
                                }
                                NavItemKind::Trigger => {}
                            }
                        }
                        shell.capture_event();
                    }
                }
            }
            Event::Window(window::Event::RedrawRequested(now)) => {
                state.last_redraw = Some(*now);
                let elapsed_close = self.elapsed_since_close_ms(state, *now);

                if let Some(pending) = state.pending_open {
                    let elapsed = now.saturating_duration_since(pending.started_at);
                    let delay_ms = self
                        .root_props
                        .timing
                        .derived_open_delay_ms(state.open_index.is_some(), elapsed_close);
                    if elapsed >= Duration::from_millis(delay_ms) {
                        if let Some(item) = self.items.get(pending.index)
                            && item.kind == NavItemKind::Trigger
                            && !item.disabled
                        {
                            self.set_open_value(state, shell, Some(item.value.clone()));
                        }
                        state.pending_open = None;
                    } else {
                        shell.request_redraw_at(
                            pending.started_at + Duration::from_millis(delay_ms),
                        );
                    }
                }

                if let Some(pending_close) = state.pending_close {
                    let elapsed = now.saturating_duration_since(pending_close);
                    let delay_ms = self
                        .root_props
                        .timing
                        .derived_close_delay_ms(state.open_index.is_some(), elapsed_close);
                    if elapsed >= Duration::from_millis(delay_ms) {
                        self.set_open_value(state, shell, None);
                        state.pending_close = None;
                        state.last_close_at = Some(*now);
                    } else {
                        shell.request_redraw_at(pending_close + Duration::from_millis(delay_ms));
                    }
                }

                let over_viewport = state
                    .viewport_bounds
                    .map(|rect| cursor.is_over(rect))
                    .unwrap_or(false)
                    || state.viewport_hovered;
                if (state.hovered_index.is_some() || over_viewport) && state.pending_close.is_some()
                {
                    state.pending_close = None;
                }

                if let Some(started) = state.indicator_started {
                    let elapsed = now.saturating_duration_since(started);
                    if elapsed < Duration::from_millis(NAVIGATION_MENU_INDICATOR_ANIM_MS) {
                        shell.request_redraw();
                    } else {
                        state.indicator_started = None;
                        state.indicator_from = None;
                    }
                }

                if let Some(motion) = state.motion {
                    let elapsed = now.saturating_duration_since(motion.started_at);
                    if elapsed < Duration::from_millis(NAVIGATION_MENU_MOTION_ANIM_MS) {
                        shell.request_redraw();
                    } else {
                        state.motion = None;
                    }
                }
            }
            _ => {}
        }

        let next_open_index = self.open_index(state);
        if next_open_index != state.open_index {
            let now = state.last_redraw.unwrap_or_else(Instant::now);
            state.indicator_from = state
                .open_index
                .and_then(|idx| state.trigger_bounds.get(idx).copied());
            state.indicator_to =
                next_open_index.and_then(|idx| state.trigger_bounds.get(idx).copied());
            state.indicator_started =
                if state.indicator_from.is_some() && state.indicator_to.is_some() {
                    Some(now)
                } else {
                    None
                };

            if let (Some(prev), Some(next)) = (state.open_index, next_open_index)
                && prev != next
            {
                let direction = if next > prev { 1 } else { -1 };
                state.motion = Some(Motion {
                    direction,
                    started_at: now,
                });
            }

            state.open_index = next_open_index;
            state.focused_index = state.focused_index.or(next_open_index);
            if state.indicator_started.is_some() {
                shell.request_redraw();
            }
        }

        for (index, child_tree) in tree
            .children
            .iter_mut()
            .take(self.triggers.len())
            .enumerate()
        {
            if let Some(item) = self.items.get(index)
                && item.kind == NavItemKind::Trigger
            {
                let trigger_state = child_tree
                    .state
                    .downcast_mut::<NavigationMenuTriggerState>();
                trigger_state.is_open = state.open_index == Some(index);
            }
        }
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: layout::Layout<'_>,
        _renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, IcedTheme, Renderer>> {
        let state = tree.state.downcast_mut::<NavigationMenuState>();
        let open_index = state.open_index?;
        let item = self.items.get(open_index)?;
        let content_index = item.content_index?;
        let content_tree_index = self.triggers.len() + content_index;
        let content = &mut self.contents[content_index];
        let content_tree = &mut tree.children[content_tree_index];
        let anchor_position = layout.position() + translation;
        let trigger_bounds = state
            .trigger_bounds
            .get(open_index)
            .copied()
            .map(|rect| Rectangle {
                x: rect.x + anchor_position.x,
                y: rect.y + anchor_position.y,
                width: rect.width,
                height: rect.height,
            })
            .unwrap_or(Rectangle::default());

        let surface = if self.root_props.viewport {
            self.viewport_style
        } else {
            self.content_style
        };

        Some(overlay::Element::new(Box::new(NavigationMenuOverlay {
            content,
            tree: content_tree,
            root_props: self.root_props,
            content_props: item.content_props,
            state,
            trigger_bounds,
            viewport: *viewport,
            surface,
        })))
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: layout::Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        for (index, child) in self.triggers.iter().enumerate() {
            if let Some(child_layout) = layout.children().nth(index) {
                let interaction = child.as_widget().mouse_interaction(
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
        }

        if cursor.is_over(layout.bounds()) {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::default()
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &IcedTheme,
        style: &renderer::Style,
        layout: layout::Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        if !bounds.intersects(viewport) {
            return;
        }

        let metrics = metrics(self.list_props.padding, self.list_props.gap, &self.theme);
        let state = tree.state.downcast_ref::<NavigationMenuState>();

        for (index, child) in self.triggers.iter().enumerate() {
            if let Some(child_layout) = layout.children().nth(index) {
                child.as_widget().draw(
                    &tree.children[index],
                    renderer,
                    theme,
                    style,
                    child_layout,
                    cursor,
                    viewport,
                );
            }
        }

        // Indicator diamond is painted by the overlay (between trigger and
        // panel) so parents cannot clip it; see NavigationMenuOverlay::draw.

        if state.focused
            && state.focus_visible
            && let Some(focus_index) = state.focused_index
            && let Some(rect) = state.trigger_bounds.get(focus_index)
        {
            let focus_rect = Rectangle {
                x: rect.x + bounds.x - 2.0,
                y: rect.y + bounds.y - 2.0,
                width: rect.width + 4.0,
                height: rect.height + 4.0,
            };

            renderer.fill_quad(
                renderer::Quad {
                    bounds: focus_rect,
                    border: Border {
                        color: self.theme.palette.ring.scale_alpha(0.50),
                        width: 3.0,
                        radius: metrics.radius.into(),
                    },
                    ..renderer::Quad::default()
                },
                Background::Color(Color::TRANSPARENT),
            );
        }
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: layout::Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation,
    ) {
        operation.container(None, layout.bounds());
        operation.traverse(&mut |operation| {
            for (index, child) in self.triggers.iter_mut().enumerate() {
                if let Some(child_layout) = layout.children().nth(index) {
                    child.as_widget_mut().operate(
                        &mut tree.children[index],
                        child_layout,
                        renderer,
                        operation,
                    );
                }
            }
        });
    }
}

struct NavigationMenuOverlay<'a, 'b, Message> {
    content: &'a mut Element<'b, Message>,
    tree: &'a mut Tree,
    root_props: NavigationMenuProps,
    content_props: NavigationMenuContentProps,
    state: &'a mut NavigationMenuState,
    trigger_bounds: Rectangle,
    viewport: Rectangle,
    surface: NavigationMenuViewportStyle,
}

impl<Message> overlay::Overlay<Message, IcedTheme, Renderer>
    for NavigationMenuOverlay<'_, '_, Message>
where
    Message: Clone,
{
    fn layout(&mut self, renderer: &Renderer, bounds: Size) -> layout::Node {
        // Cap width when the content declares one; otherwise keep the window
        // max but children that use Length::Shrink still size intrinsically.
        // Without this, Length::Fill links expand to the full window width.
        let max = match self.content_props.width {
            Some(width) => Size::new(width.max(0.0).min(bounds.width), bounds.height),
            None => bounds,
        };
        let limits = layout::Limits::new(Size::ZERO, max);
        let content_node = self
            .content
            .as_widget_mut()
            .layout(self.tree, renderer, &limits);
        let content_size = content_node.size();

        let trigger = NavRect::new(
            self.trigger_bounds.x,
            self.trigger_bounds.y,
            self.trigger_bounds.width,
            self.trigger_bounds.height,
        );

        let (x, y) = if self.root_props.viewport {
            place_navigation_menu_viewport(
                trigger,
                content_size.width,
                content_size.height,
                self.content_props.align,
                self.content_props.align_offset,
                self.content_props.side_offset,
                self.content_props.collision_padding,
                bounds.width,
                bounds.height,
            )
        } else {
            place_navigation_menu_content(
                trigger,
                content_size.width,
                content_size.height,
                self.content_props.side,
                self.content_props.align,
                self.content_props.side_offset,
                self.content_props.align_offset,
                self.content_props.collision_padding,
                bounds.width,
                bounds.height,
            )
        };

        let motion = current_motion_offset(self.state, self.root_props.viewport);
        let position = Point::new(x + motion.x, y + motion.y);

        let mut root = layout::Node::with_children(content_node.size(), vec![content_node]);
        root = root.move_to(position);
        self.state.viewport_bounds = Some(root.bounds());
        self.state.viewport_size = Some(content_size);
        root
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
        let content_layout = layout.children().next().unwrap_or(layout);
        let bounds = content_layout.bounds();
        self.state.viewport_bounds = Some(bounds);
        self.state.viewport_hovered = cursor.is_over(bounds);
        if self.state.viewport_hovered {
            self.state.pending_close = None;
        }
        self.content.as_widget_mut().update(
            self.tree,
            event,
            content_layout,
            cursor,
            renderer,
            clipboard,
            shell,
            &bounds,
        );
    }

    fn mouse_interaction(
        &self,
        layout: layout::Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        let content_layout = layout.children().next().unwrap_or(layout);
        self.content.as_widget().mouse_interaction(
            self.tree,
            content_layout,
            cursor,
            &self.viewport,
            renderer,
        )
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        theme: &IcedTheme,
        style: &renderer::Style,
        layout: layout::Layout<'_>,
        cursor: mouse::Cursor,
    ) {
        let content_layout = layout.children().next().unwrap_or(layout);
        let bounds = content_layout.bounds();
        paint_viewport_surface(renderer, bounds, self.surface);

        self.content.as_widget().draw(
            self.tree,
            renderer,
            theme,
            &renderer::Style {
                text_color: self.surface.text_color,
            },
            content_layout,
            cursor,
            &self.viewport,
        );

        // bits-ui / shadcn: rotated square at the panel top edge under the
        // open trigger. Painted after content so the toggle is obvious.
        if self.root_props.indicator {
            let size = 10.0;
            let diamond = Rectangle {
                x: self.trigger_bounds.x + (self.trigger_bounds.width - size) / 2.0,
                y: bounds.y - size * 0.5,
                width: size,
                height: size,
            };
            paint_indicator_diamond(
                renderer,
                diamond,
                self.surface.background,
                self.surface.border_color,
            );
        }

        paint_viewport_ring(renderer, bounds, self.surface);
        let _ = style;
    }
}

/// Gap between trigger label and chevron (`ml-1` in shadcn-svelte).
const TRIGGER_CHEVRON_GAP_PX: f32 = 4.0;

/// Trigger chip with optional chevron.
pub(super) struct NavigationMenuTriggerWidget<'a, Message> {
    pub(super) content: Element<'a, Message>,
    pub(super) show_chevron: bool,
    pub(super) icon_size: f32,
    pub(super) pad_y: f32,
    pub(super) pad_x: f32,
    pub(super) disabled: bool,
    pub(super) theme: Theme,
    pub(super) link_props: NavigationMenuLinkProps,
}

impl<Message> Widget<Message, IcedTheme, Renderer> for NavigationMenuTriggerWidget<'_, Message>
where
    Message: Clone,
{
    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&[self.content.as_widget()]);
    }

    fn state(&self) -> tree::State {
        tree::State::new(NavigationMenuTriggerState::default())
    }

    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<NavigationMenuTriggerState>()
    }

    fn size(&self) -> Size<Length> {
        Size::new(Length::Shrink, Length::Shrink)
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let padding = Padding {
            top: self.pad_y,
            right: self.pad_x,
            bottom: self.pad_y,
            left: self.pad_x,
        };
        // Reserve `ml-1` + `size-3` inside the padded row (not outside it).
        let caret_slot = if self.show_chevron {
            TRIGGER_CHEVRON_GAP_PX + self.icon_size
        } else {
            0.0
        };

        layout::padded(limits, Length::Shrink, Length::Shrink, padding, |limits| {
            let max = limits.max();
            let content_limits = layout::Limits::new(
                Size::ZERO,
                Size::new((max.width - caret_slot).max(0.0), max.height),
            );
            let content_node = self.content.as_widget_mut().layout(
                &mut tree.children[0],
                renderer,
                &content_limits,
            );
            let size = content_node.size();
            let height = size.height.max(self.icon_size);
            // `items-center` — vertically center the label in the row.
            let content_node =
                content_node.move_to(Point::new(0.0, ((height - size.height) / 2.0).max(0.0)));

            layout::Node::with_children(
                Size::new(size.width + caret_slot, height),
                vec![content_node],
            )
        })
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
        let label_layout = trigger_label_layout(layout);
        self.content.as_widget_mut().update(
            &mut tree.children[0],
            event,
            label_layout,
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );

        if self.disabled {
            return;
        }

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. })
                if cursor.is_over(layout.bounds()) =>
            {
                tree.state
                    .downcast_mut::<NavigationMenuTriggerState>()
                    .is_pressed = true;
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. }) => {
                tree.state
                    .downcast_mut::<NavigationMenuTriggerState>()
                    .is_pressed = false;
            }
            Event::Touch(touch::Event::FingerLost { .. }) => {
                tree.state
                    .downcast_mut::<NavigationMenuTriggerState>()
                    .is_pressed = false;
            }
            _ => {}
        }
    }

    fn mouse_interaction(
        &self,
        _tree: &Tree,
        layout: layout::Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        if self.disabled {
            return mouse::Interaction::default();
        }
        if cursor.is_over(layout.bounds()) {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::default()
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &IcedTheme,
        _style: &renderer::Style,
        layout: layout::Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        let label_layout = trigger_label_layout(layout);
        let state = tree.state.downcast_ref::<NavigationMenuTriggerState>();

        let status = if self.disabled {
            button::Status::Disabled
        } else if cursor.is_over(bounds) {
            if state.is_pressed {
                button::Status::Pressed
            } else {
                button::Status::Hovered
            }
        } else {
            button::Status::Active
        };

        let resolved = resolve_link_style(&self.theme, self.link_props, status, state.is_open);
        paint_item_surface(renderer, bounds, resolved);

        self.content.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            &renderer::Style {
                text_color: resolved.text_color,
            },
            label_layout,
            cursor,
            viewport,
        );

        if self.show_chevron {
            let label_bounds = label_layout.bounds();
            // Sit in the reserved caret slot: after label + `ml-1`, still inside
            // the padded trigger (matches `.cn-navigation-menu-trigger-icon`).
            let center = Point::new(
                label_bounds.x + label_bounds.width + TRIGGER_CHEVRON_GAP_PX + self.icon_size / 2.0,
                bounds.y + bounds.height / 2.0,
            );
            draw_chevron(
                renderer,
                center,
                self.icon_size,
                resolved.text_color,
                state.is_open,
            );
        }
    }
}

/// `layout::padded` → row (label + caret slot) → label. Returns the label node.
fn trigger_label_layout(layout: layout::Layout<'_>) -> layout::Layout<'_> {
    let row = layout.children().next().unwrap_or(layout);
    row.children().next().unwrap_or(row)
}

/// Interactive navigation-menu link widget.
pub(super) struct NavigationMenuLinkWidget<'a, Message> {
    pub(super) content: Element<'a, Message>,
    pub(super) on_press: Option<Message>,
    pub(super) props: NavigationMenuLinkProps,
    pub(super) theme: Theme,
    pub(super) width: Length,
    pub(super) height: Length,
}

impl<Message> Widget<Message, IcedTheme, Renderer> for NavigationMenuLinkWidget<'_, Message>
where
    Message: Clone,
{
    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&[self.content.as_widget()]);
    }

    fn state(&self) -> tree::State {
        tree::State::new(NavigationMenuLinkState::default())
    }

    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<NavigationMenuLinkState>()
    }

    fn size(&self) -> Size<Length> {
        Size::new(self.width, self.height)
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let padding = self.props.resolved_padding();
        layout::padded(limits, self.width, self.height, padding, |limits| {
            self.content
                .as_widget_mut()
                .layout(&mut tree.children[0], renderer, limits)
        })
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
        self.content.as_widget_mut().update(
            &mut tree.children[0],
            event,
            layout.children().next().unwrap_or(layout),
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );

        if shell.is_event_captured() || self.props.disabled || self.on_press.is_none() {
            tree.state
                .downcast_mut::<NavigationMenuLinkState>()
                .is_pressed = false;
            return;
        }

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. })
                if cursor.is_over(layout.bounds()) =>
            {
                tree.state
                    .downcast_mut::<NavigationMenuLinkState>()
                    .is_pressed = true;
                shell.capture_event();
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. }) => {
                let state = tree.state.downcast_mut::<NavigationMenuLinkState>();
                if state.is_pressed {
                    state.is_pressed = false;
                    if cursor.is_over(layout.bounds())
                        && let Some(message) = self.on_press.clone()
                    {
                        shell.publish(message);
                    }
                    shell.capture_event();
                }
            }
            Event::Touch(touch::Event::FingerLost { .. }) => {
                tree.state
                    .downcast_mut::<NavigationMenuLinkState>()
                    .is_pressed = false;
            }
            _ => {}
        }
    }

    fn mouse_interaction(
        &self,
        _tree: &Tree,
        layout: layout::Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        if self.props.disabled {
            return mouse::Interaction::default();
        }
        if cursor.is_over(layout.bounds()) {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::default()
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &IcedTheme,
        _style: &renderer::Style,
        layout: layout::Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        let content_layout = layout.children().next().unwrap_or(layout);
        let state = tree.state.downcast_ref::<NavigationMenuLinkState>();

        let status = if self.props.disabled {
            button::Status::Disabled
        } else if cursor.is_over(bounds) {
            if state.is_pressed {
                button::Status::Pressed
            } else {
                button::Status::Hovered
            }
        } else {
            button::Status::Active
        };

        let resolved = resolve_link_style(&self.theme, self.props, status, false);
        paint_item_surface(renderer, bounds, resolved);

        self.content.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            &renderer::Style {
                text_color: resolved.text_color,
            },
            content_layout,
            cursor,
            viewport,
        );
    }
}

#[derive(Default)]
struct Line {
    indices: Vec<usize>,
    width: f32,
    height: f32,
}

fn resolve_open_index<Message>(
    items: &[NavItemMeta<Message>],
    value: Option<&str>,
) -> Option<usize> {
    let value = value?;
    items
        .iter()
        .position(|item| item.kind == NavItemKind::Trigger && !item.disabled && item.value == value)
}

fn hit_test_trigger(
    bounds: &[Rectangle],
    list_bounds: Rectangle,
    cursor: mouse::Cursor,
) -> Option<usize> {
    bounds.iter().position(|rect| {
        let global = Rectangle {
            x: rect.x + list_bounds.x,
            y: rect.y + list_bounds.y,
            width: rect.width,
            height: rect.height,
        };
        cursor.is_over(global)
    })
}

fn bridge_hovered(
    list_bounds: Rectangle,
    viewport_bounds: Option<Rectangle>,
    cursor: mouse::Cursor,
) -> bool {
    let Some(viewport) = viewport_bounds else {
        return false;
    };
    let list_bottom = list_bounds.y + list_bounds.height;
    let viewport_top = viewport.y;
    let (y1, y2) = if viewport_top >= list_bottom {
        (list_bottom, viewport_top)
    } else {
        (viewport_top, list_bottom)
    };
    let x1 = list_bounds.x.min(viewport.x);
    let x2 = (list_bounds.x + list_bounds.width).max(viewport.x + viewport.width);
    let bridge = Rectangle {
        x: x1,
        y: y1,
        width: (x2 - x1).max(0.0),
        height: (y2 - y1).max(0.0),
    };
    cursor.is_over(bridge)
}

fn current_motion_offset(state: &NavigationMenuState, viewport: bool) -> Vector {
    let Some(motion) = state.motion else {
        return Vector::default();
    };
    let now = state.last_redraw.unwrap_or_else(Instant::now);
    let elapsed = now.saturating_duration_since(motion.started_at);
    let progress =
        (elapsed.as_secs_f32() / (NAVIGATION_MENU_MOTION_ANIM_MS as f32 / 1000.0)).clamp(0.0, 1.0);
    Vector::new(motion_offset_x(progress, motion.direction, viewport), 0.0)
}

fn resolve_viewport_bounds<Message>(
    menu: &NavigationMenuWidget<'_, Message>,
    state: &NavigationMenuState,
    list_bounds: Rectangle,
    viewport: Rectangle,
) -> Option<Rectangle> {
    let open_index = menu.open_index(state)?;
    let trigger = state.trigger_bounds.get(open_index).copied()?;
    let content_size = state.viewport_size?;
    let item = menu.items.get(open_index)?;

    let trigger_bounds = NavRect::new(
        trigger.x + list_bounds.x,
        trigger.y + list_bounds.y,
        trigger.width,
        trigger.height,
    );

    let (x, y) = if menu.root_props.viewport {
        place_navigation_menu_viewport(
            trigger_bounds,
            content_size.width,
            content_size.height,
            item.content_props.align,
            item.content_props.align_offset,
            item.content_props.side_offset,
            item.content_props.collision_padding,
            viewport.width,
            viewport.height,
        )
    } else {
        place_navigation_menu_content(
            trigger_bounds,
            content_size.width,
            content_size.height,
            item.content_props.side,
            item.content_props.align,
            item.content_props.side_offset,
            item.content_props.align_offset,
            item.content_props.collision_padding,
            viewport.width,
            viewport.height,
        )
    };

    let motion = current_motion_offset(state, menu.root_props.viewport);
    Some(Rectangle {
        x: x + motion.x,
        y: y + motion.y,
        width: content_size.width,
        height: content_size.height,
    })
}

fn handle_hover<Message: Clone>(
    menu: &NavigationMenuWidget<'_, Message>,
    state: &mut NavigationMenuState,
    hovered_index: Option<usize>,
    list_bounds: Rectangle,
    viewport_bounds: Option<Rectangle>,
    cursor: mouse::Cursor,
    shell: &mut Shell<'_, Message>,
) {
    let now = Instant::now();
    let open_index = menu.open_index(state);
    let over_viewport = viewport_bounds
        .map(|rect| cursor.is_over(rect))
        .unwrap_or(false)
        || state.viewport_hovered;
    let over_bridge = bridge_hovered(list_bounds, viewport_bounds, cursor);

    if let Some(index) = hovered_index
        && let Some(item) = menu.items.get(index)
        && item.kind == NavItemKind::Trigger
        && !item.disabled
    {
        state.pending_close = None;

        if open_index != Some(index)
            && state
                .pending_open
                .map(|pending| pending.index != index)
                .unwrap_or(true)
        {
            state.pending_open = Some(PendingOpen {
                index,
                started_at: now,
            });
            shell.request_redraw();
        }
    } else {
        state.pending_open = None;
        if open_index.is_some() && !(over_viewport || over_bridge) {
            if state.pending_close.is_none() {
                state.pending_close = Some(now);
                shell.request_redraw();
            }
        } else if open_index.is_some() && (over_viewport || over_bridge) {
            state.pending_close = None;
        }
    }
}

fn paint_indicator_diamond(renderer: &mut Renderer, bounds: Rectangle, fill: Color, border: Color) {
    if bounds.width <= 0.0 || bounds.height <= 0.0 {
        return;
    }

    let size = bounds.width.max(bounds.height);
    let mut frame = canvas::Frame::new(renderer, Size::new(size, size));
    let half = size / 2.0;
    // CSS `rotate-45` square → diamond pointing up toward the trigger.
    let path = canvas::Path::new(|builder| {
        builder.move_to(Point::new(half, 0.0));
        builder.line_to(Point::new(size, half));
        builder.line_to(Point::new(half, size));
        builder.line_to(Point::new(0.0, half));
        builder.close();
    });
    if fill.a > f32::EPSILON {
        frame.fill(&path, fill);
    }
    if border.a > f32::EPSILON {
        frame.stroke(
            &path,
            canvas::Stroke::default()
                .with_width(1.0)
                .with_color(border)
                .with_line_join(canvas::LineJoin::Miter),
        );
    }
    let geometry = frame.into_geometry();
    renderer.with_translation(Vector::new(bounds.x, bounds.y), |renderer| {
        renderer.draw_geometry(geometry);
    });
}

fn draw_chevron(renderer: &mut Renderer, center: Point, size: f32, color: Color, open: bool) {
    if size <= 0.0 {
        return;
    }

    let reach = size * 0.25;
    let arm = size * 0.125;
    let stroke_width = (size * 0.10).clamp(1.0, 1.75);

    let mut frame = canvas::Frame::new(renderer, Size::new(size, size));
    frame.translate(Vector::new(size / 2.0, size / 2.0));
    // Flip vertically when open (`group-data-open:rotate-180`).
    let y_sign = if open { -1.0 } else { 1.0 };
    frame.stroke(
        &canvas::Path::new(|builder| {
            builder.move_to(Point::new(-reach, -arm * y_sign));
            builder.line_to(Point::new(0.0, arm * y_sign));
            builder.line_to(Point::new(reach, -arm * y_sign));
        }),
        canvas::Stroke::default()
            .with_width(stroke_width)
            .with_color(color)
            .with_line_cap(canvas::LineCap::Round)
            .with_line_join(canvas::LineJoin::Round),
    );
    let geometry = frame.into_geometry();

    renderer.with_translation(
        Vector::new(center.x - size / 2.0, center.y - size / 2.0),
        |renderer| {
            renderer.draw_geometry(geometry);
        },
    );
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn resolve_open_index_matches_trigger() {
        let items = vec![
            NavItemMeta::<()> {
                value: "home".to_string(),
                kind: NavItemKind::Trigger,
                disabled: false,
                content_index: Some(0),
                content_props: NavigationMenuContentProps::new(),
                link_message: None,
            },
            NavItemMeta::<()> {
                value: "docs".to_string(),
                kind: NavItemKind::Link,
                disabled: false,
                content_index: None,
                content_props: NavigationMenuContentProps::new(),
                link_message: None,
            },
        ];

        assert_eq!(resolve_open_index(&items, Some("home")), Some(0));
        assert_eq!(resolve_open_index(&items, Some("docs")), None);
    }
}
