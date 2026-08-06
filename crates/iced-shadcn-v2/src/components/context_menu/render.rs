//! Custom widget and overlay rendering for [`super::ContextMenu`].
//!
//! The widget wraps a trigger element. A secondary (right) click over the
//! trigger opens an iced overlay anchored at the cursor position (bits-ui
//! `position` + `--bits-context-menu-content-available-height`). The overlay
//! paints the `.cn-context-menu-content` surface with rows (items, checkboxes,
//! radios, labels, separators, sub-triggers) and optional nested
//! `.cn-context-menu-sub-content` panels. Keyboard navigation matches bits-ui
//! (arrow keys, Enter/Space, Esc, ArrowRight/Left for submenus).

use iced_core::keyboard;
use iced_core::text::{self as core_text, Renderer as _, Text};

use shadcn_common::{
    CONTEXT_MENU_FLIP_SLACK_PX, ContextMenuRecipe, Direction, FloatingAlign, FloatingConfig,
    FloatingPadding, FloatingRect, FloatingSide, MENU_SUB_SIDE_OFFSET_PX, MenuItemVariant,
    NavAction, NavKey, Orientation, compute_floating, resolve_nav_action,
};

use crate::iced_compat::advanced::renderer::Renderer as _;
use crate::iced_compat::advanced::widget::{Tree, tree};
use crate::iced_compat::advanced::{Clipboard, Shell, Widget, layout, overlay, renderer};
use crate::iced_compat::widget::canvas;
use crate::iced_compat::widget::graphics::geometry::Renderer as _;
use crate::iced_compat::{
    Background, Border, Color, Element, Event, Font, Length, Pixels, Point, Rectangle, Renderer,
    Size, Theme as IcedTheme, Vector, alignment, mouse, touch, window,
};

use super::style::{self, ContextMenuContentStyle, item_colors, item_highlight_fill};
use super::types::Entry;
use crate::fonts::iced_font;
use crate::recipes::iced_font_weight;
use crate::theme::Theme;

/// Internal widget produced by the [`super::ContextMenu`] builder.
pub(super) struct ContextMenuWidget<'a, Message> {
    pub(super) theme: &'a Theme,
    pub(super) trigger: Element<'a, Message>,
    pub(super) entries: Vec<Entry<Message>>,
    pub(super) width: Option<f32>,
    pub(super) side: Option<FloatingSide>,
    pub(super) side_offset: f32,
    pub(super) disabled: bool,
    pub(super) open_override: Option<bool>,
    pub(super) default_open: bool,
    pub(super) on_open: Option<Message>,
    pub(super) on_close: Option<Message>,
    pub(super) on_open_change: Option<Box<dyn Fn(bool) -> Message + 'a>>,
    pub(super) style_override:
        Option<Box<dyn Fn(ContextMenuContentStyle) -> ContextMenuContentStyle + 'a>>,
}

/// Widget-tree state of the trigger and its menu.
struct State {
    requested_open: bool,
    /// Anchor point (cursor) for the open root, in viewport coordinates.
    anchor: Point,
    hovered: Option<usize>,
    /// Indices of the open submenu chain from the root.
    open_path: Vec<usize>,
    /// Hovered row inside the deepest open submenu (`None` = root menu).
    hovered_sub: Option<usize>,
}

impl State {
    fn new(default_open: bool) -> Self {
        Self {
            requested_open: default_open,
            anchor: Point::ORIGIN,
            hovered: None,
            open_path: Vec::new(),
            hovered_sub: None,
        }
    }
}

impl<Message> ContextMenuWidget<'_, Message>
where
    Message: Clone,
{
    fn is_open(&self, state: &State) -> bool {
        !self.disabled && self.open_override.unwrap_or(state.requested_open)
    }

    fn resolve_style(&self, submenu: bool) -> ContextMenuContentStyle {
        let mut resolved = style::resolve_content_style(self.theme, submenu);
        if let Some(override_fn) = self.style_override.as_ref() {
            resolved = override_fn(resolved);
        }
        resolved
    }

    fn publish_open_change(&self, open: bool, shell: &mut Shell<'_, Message>) {
        if open {
            if let Some(on_open) = &self.on_open {
                shell.publish(on_open.clone());
            }
        } else if let Some(on_close) = &self.on_close {
            shell.publish(on_close.clone());
        }

        if let Some(on_open_change) = self.on_open_change.as_ref() {
            shell.publish(on_open_change(open));
        }
    }

    fn set_open(&self, state: &mut State, open: bool, shell: &mut Shell<'_, Message>) {
        if self.open_override.is_some() {
            self.publish_open_change(open, shell);
            return;
        }

        if state.requested_open == open {
            return;
        }

        state.requested_open = open;
        if !open {
            state.open_path.clear();
            state.hovered = None;
            state.hovered_sub = None;
        }
        self.publish_open_change(open, shell);
        shell.request_redraw();
    }

    fn open_at(&self, state: &mut State, anchor: Point, shell: &mut Shell<'_, Message>) {
        if self.disabled {
            return;
        }
        state.anchor = anchor;
        state.hovered = self.entries.iter().position(Entry::is_selectable);
        state.open_path.clear();
        state.hovered_sub = None;
        self.set_open(state, true, shell);
    }

    fn handle_trigger_secondary_press(
        &self,
        state: &mut State,
        position: Point,
        shell: &mut Shell<'_, Message>,
    ) {
        if self.disabled {
            return;
        }

        if self.is_open(state) {
            // Re-anchor on a fresh right-click while already open (matches the
            // web behaviour: the menu jumps to the new cursor position).
            state.anchor = position;
            state.hovered = self.entries.iter().position(Entry::is_selectable);
            state.open_path.clear();
            state.hovered_sub = None;
            shell.request_redraw();
        } else {
            self.open_at(state, position, shell);
        }
    }
}

impl<'a, Message> Widget<Message, IcedTheme, Renderer> for ContextMenuWidget<'a, Message>
where
    Message: Clone + 'a,
{
    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.trigger)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&[self.trigger.as_widget()]);
    }

    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::new(self.default_open))
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
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Right))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                let state = tree.state.downcast_mut::<State>();

                if let Some(position) = cursor.position_in(layout.bounds()) {
                    // bits-ui opens on `contextmenu` (right click) over the
                    // trigger. Touch uses a long-press which surfaces as a
                    // press on iced; we treat it the same.
                    let position = Point::new(
                        layout.bounds().x + position.x,
                        layout.bounds().y + position.y,
                    );
                    self.handle_trigger_secondary_press(state, position, shell);
                    shell.capture_event();
                } else if self.is_open(state)
                    && let Some(_position) = cursor.position()
                {
                    // Right-click outside the trigger but inside the overlay is
                    // handled by the overlay itself; if it lands fully outside,
                    // the overlay's `update` closes the menu.
                }
            }
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                let state = tree.state.downcast_mut::<State>();
                if !cursor.is_over(layout.bounds()) && self.is_open(state) {
                    // Left click outside the trigger closes (the overlay also
                    // closes on outside press; this guards the case where the
                    // overlay did not capture it).
                    self.set_open(state, false, shell);
                }
            }
            Event::Keyboard(keyboard::Event::KeyPressed {
                key: keyboard::Key::Named(keyboard::key::Named::Escape),
                ..
            }) => {
                let state = tree.state.downcast_mut::<State>();
                if self.is_open(state) {
                    if !state.open_path.is_empty() {
                        state.open_path.pop();
                        state.hovered_sub = None;
                        shell.capture_event();
                        shell.request_redraw();
                    } else {
                        self.set_open(state, false, shell);
                        shell.capture_event();
                    }
                }
            }
            Event::Window(window::Event::RedrawRequested(_)) => {}
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
        theme: &IcedTheme,
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
    ) -> Option<overlay::Element<'b, Message, IcedTheme, Renderer>> {
        let state = tree.state.downcast_mut::<State>();

        if !self.is_open(state) {
            return self.trigger.as_widget_mut().overlay(
                &mut tree.children[0],
                layout,
                renderer,
                viewport,
                translation,
            );
        }

        let content_style = self.resolve_style(false);
        let sub_style = self.resolve_style(true);
        let recipe = style::recipe(self.theme);
        let font = iced_font(self.theme.font_pack().sans);
        let mut label_font = font;
        label_font.weight = iced_font_weight(recipe.label_typography.weight);
        let mut item_font = font;
        item_font.weight = iced_font_weight(recipe.item_typography.weight);
        let mut shortcut_font = font;
        shortcut_font.weight = iced_font_weight(recipe.shortcut_typography.weight);

        let width = self
            .width
            .unwrap_or(recipe.content_min_width_px)
            .max(recipe.content_min_width_px);

        Some(
            MenuOverlay {
                anchor: state.anchor + translation,
                viewport: *viewport,
                entries: &mut self.entries,
                width,
                side: self.side,
                side_offset: self.side_offset,
                recipe,
                content_style,
                sub_style,
                item_font,
                label_font,
                shortcut_font,
                hovered: &mut state.hovered,
                open_path: &mut state.open_path,
                hovered_sub: &mut state.hovered_sub,
                close_requested: &mut state.requested_open,
                open_override: self.open_override,
                on_close_msg: self.on_close.clone(),
                on_open_change: self.on_open_change.as_deref(),
            }
            .element(),
        )
    }
}

/// Root + nested submenu overlay anchored at the cursor.
struct MenuOverlay<'a, Message> {
    anchor: Point,
    viewport: Rectangle,
    entries: &'a mut [Entry<Message>],
    width: f32,
    side: Option<FloatingSide>,
    side_offset: f32,
    recipe: ContextMenuRecipe,
    content_style: ContextMenuContentStyle,
    sub_style: ContextMenuContentStyle,
    item_font: Font,
    label_font: Font,
    shortcut_font: Font,
    hovered: &'a mut Option<usize>,
    open_path: &'a mut Vec<usize>,
    hovered_sub: &'a mut Option<usize>,
    close_requested: &'a mut bool,
    open_override: Option<bool>,
    on_close_msg: Option<Message>,
    on_open_change: Option<&'a dyn Fn(bool) -> Message>,
}

impl<'a, Message> MenuOverlay<'a, Message>
where
    Message: Clone + 'a,
{
    fn element(self) -> overlay::Element<'a, Message, IcedTheme, Renderer> {
        overlay::Element::new(Box::new(self))
    }

    fn close_menu(&mut self, shell: &mut Shell<'_, Message>) {
        if self.open_override.is_none() {
            *self.close_requested = false;
        }
        self.open_path.clear();
        *self.hovered = None;
        *self.hovered_sub = None;

        if let Some(on_close) = &self.on_close_msg {
            shell.publish(on_close.clone());
        }
        if let Some(on_open_change) = self.on_open_change {
            shell.publish(on_open_change(false));
        }
        shell.request_redraw();
    }

    fn root_height(&self) -> f32 {
        menu_height(self.entries, self.recipe)
    }

    /// Resolve the root origin from the cursor anchor and the requested side.
    ///
    /// When `side` is `None` the menu prefers below the cursor and flips via
    /// [`compute_floating`]. Explicit `Left` / `Right` / `Top` / `Bottom` place
    /// relative to the cursor point (bits-ui `side` on context-menu content).
    fn root_origin(&self, bounds: Size) -> Point {
        cursor_menu_origin(
            self.anchor,
            self.width,
            fitted_menu_height(self.root_height(), bounds.height),
            bounds,
            self.side,
            self.side_offset,
        )
    }
}

impl<Message> overlay::Overlay<Message, IcedTheme, Renderer> for MenuOverlay<'_, Message>
where
    Message: Clone,
{
    fn layout(&mut self, _renderer: &Renderer, bounds: Size) -> layout::Node {
        let origin = self.root_origin(bounds);
        let space_below = (bounds.height - origin.y).max(0.0);
        let height = fitted_menu_height(self.root_height(), space_below);
        let root = layout::Node::new(Size::new(self.width, height)).move_to(origin);

        let mut children = vec![root];

        if let Some(&sub_index) = self.open_path.first()
            && let Some(Entry::Sub(sub)) = self.entries.get(sub_index)
        {
            let row_y = row_offset_y(self.entries, sub_index, self.recipe);
            let sub_available = (bounds.height - (origin.y + row_y)).max(0.0);
            let sub_height =
                fitted_menu_height(menu_height(&sub.entries, self.recipe), sub_available);
            let sub_width = self.width.max(self.recipe.sub_content_min_width_px);
            let sub_gap = MENU_SUB_SIDE_OFFSET_PX;
            let mut sub_x = origin.x + self.width + sub_gap;
            if sub_x + sub_width > bounds.width {
                sub_x = (origin.x - sub_gap - sub_width).max(0.0);
            }
            let sub_origin = Point::new(sub_x, origin.y + row_y);
            children.push(layout::Node::new(Size::new(sub_width, sub_height)).move_to(sub_origin));

            // Nested submenu (second level).
            if let Some(&nested_index) = self.open_path.get(1)
                && let Some(Entry::Sub(nested)) = sub.entries.get(nested_index)
            {
                let nested_row_y = row_offset_y(&sub.entries, nested_index, self.recipe);
                let nested_available = (bounds.height - (sub_origin.y + nested_row_y)).max(0.0);
                let nested_height =
                    fitted_menu_height(menu_height(&nested.entries, self.recipe), nested_available);
                let nested_width = sub_width.max(self.recipe.sub_content_min_width_px);
                let mut nested_x = sub_origin.x + sub_width + sub_gap;
                if nested_x + nested_width > bounds.width {
                    nested_x = (sub_origin.x - sub_gap - nested_width).max(0.0);
                }
                children.push(
                    layout::Node::new(Size::new(nested_width, nested_height))
                        .move_to(Point::new(nested_x, sub_origin.y + nested_row_y)),
                );
            }
        }

        layout::Node::with_children(bounds, children)
    }

    fn update(
        &mut self,
        event: &Event,
        layout: layout::Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) {
        let mut children = layout.children();
        let Some(root_layout) = children.next() else {
            return;
        };
        let sub_layout = children.next();
        let nested_layout = children.next();

        let root_bounds = root_layout.bounds();
        let over_root = cursor.is_over(root_bounds);
        let over_sub = sub_layout.is_some_and(|layout| cursor.is_over(layout.bounds()));
        let over_nested = nested_layout.is_some_and(|layout| cursor.is_over(layout.bounds()));

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Right))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                let path = self.open_path.clone();
                let recipe = self.recipe;

                let press = if over_nested {
                    nested_layout
                        .and_then(|layout| cursor.position_in(layout.bounds()))
                        .map(|pos| (path, pos))
                } else if over_sub {
                    sub_layout
                        .and_then(|layout| cursor.position_in(layout.bounds()))
                        .map(|pos| (path, pos))
                } else if over_root {
                    cursor.position_in(root_bounds).map(|pos| (Vec::new(), pos))
                } else {
                    None
                };

                if let Some((activate_path, pos)) = press {
                    let open_sub = maybe_open_submenu(self.entries, &activate_path, pos, recipe);
                    if let Some(next_path) = open_sub {
                        *self.open_path = next_path;
                        *self.hovered = self.open_path.first().copied().or(*self.hovered);
                        *self.hovered_sub = self.open_path.get(1).copied();
                        shell.request_redraw();
                    } else {
                        let close =
                            activate_at_path_pos(self.entries, &activate_path, pos, recipe, shell);
                        if close {
                            self.close_menu(shell);
                        }
                    }
                    shell.capture_event();
                } else {
                    self.close_menu(shell);
                    shell.capture_event();
                }
            }
            Event::Mouse(mouse::Event::CursorMoved { .. }) => {
                if let Some(pos) = cursor.position_in(root_bounds) {
                    let hovered = row_at(self.entries, pos, self.recipe)
                        .filter(|&index| self.entries[index].is_selectable());
                    if *self.hovered != hovered {
                        *self.hovered = hovered;
                        if let Some(index) =
                            hovered.filter(|&index| self.entries[index].is_submenu())
                        {
                            *self.open_path = vec![index];
                            *self.hovered_sub = None;
                        } else if hovered.is_some_and(|index| !self.entries[index].is_submenu())
                            && !over_sub
                            && !over_nested
                        {
                            self.open_path.clear();
                            *self.hovered_sub = None;
                        }
                        shell.request_redraw();
                    }
                } else if over_sub
                    && let Some(layout) = sub_layout
                    && let Some(pos) = cursor.position_in(layout.bounds())
                    && let Some(&sub_index) = self.open_path.first()
                    && let Some(Entry::Sub(sub)) = self.entries.get(sub_index)
                {
                    let hovered = row_at(&sub.entries, pos, self.recipe)
                        .filter(|&index| sub.entries[index].is_selectable());
                    if *self.hovered_sub != hovered {
                        *self.hovered_sub = hovered;
                        if let Some(index) =
                            hovered.filter(|&index| sub.entries[index].is_submenu())
                        {
                            self.open_path.truncate(1);
                            self.open_path.push(index);
                        }
                        shell.request_redraw();
                    }
                }
            }
            Event::Keyboard(keyboard::Event::KeyPressed { key, .. }) => {
                if let Some(action) = nav_action(key) {
                    match action {
                        NavAction::Next => move_hover(self.entries, self.hovered, 1, shell),
                        NavAction::Previous => move_hover(self.entries, self.hovered, -1, shell),
                        NavAction::First => {
                            *self.hovered = shadcn_common::first_enabled_index(
                                self.entries,
                                Entry::is_selectable,
                            );
                            shell.request_redraw();
                        }
                        NavAction::Last => {
                            *self.hovered = shadcn_common::last_enabled_index(
                                self.entries,
                                Entry::is_selectable,
                            );
                            shell.request_redraw();
                        }
                        NavAction::Activate => {
                            if let Some(index) = *self.hovered {
                                if self.entries[index].is_submenu() {
                                    *self.open_path = vec![index];
                                    shell.request_redraw();
                                } else {
                                    let close = activate_index(self.entries, index, shell);
                                    if close {
                                        self.close_menu(shell);
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                    shell.capture_event();
                } else if matches!(key, keyboard::Key::Named(keyboard::key::Named::ArrowRight)) {
                    if let Some(index) = *self.hovered
                        && self.entries[index].is_submenu()
                    {
                        *self.open_path = vec![index];
                        *self.hovered_sub = None;
                        shell.request_redraw();
                        shell.capture_event();
                    }
                } else if matches!(key, keyboard::Key::Named(keyboard::key::Named::ArrowLeft))
                    && !self.open_path.is_empty()
                {
                    self.open_path.pop();
                    *self.hovered_sub = None;
                    shell.request_redraw();
                    shell.capture_event();
                }
            }
            _ => {}
        }
    }

    fn mouse_interaction(
        &self,
        layout: layout::Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        for child in layout.children() {
            if cursor.is_over(child.bounds()) {
                return mouse::Interaction::Pointer;
            }
        }
        mouse::Interaction::default()
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        _theme: &IcedTheme,
        _defaults: &renderer::Style,
        layout: layout::Layout<'_>,
        _cursor: mouse::Cursor,
    ) {
        let mut children = layout.children();
        let Some(root_layout) = children.next() else {
            return;
        };

        draw_menu_surface(
            renderer,
            root_layout.bounds(),
            self.entries,
            *self.hovered,
            self.open_path.first().copied(),
            self.recipe,
            self.content_style,
            self.item_font,
            self.label_font,
            self.shortcut_font,
            &self.viewport,
        );

        if let (Some(sub_layout), Some(&sub_i)) = (children.next(), self.open_path.first())
            && let Some(Entry::Sub(sub)) = self.entries.get(sub_i)
        {
            let nested_open = self.open_path.get(1).copied();
            draw_menu_surface(
                renderer,
                sub_layout.bounds(),
                &sub.entries,
                *self.hovered_sub,
                nested_open,
                self.recipe,
                self.sub_style,
                self.item_font,
                self.label_font,
                self.shortcut_font,
                &self.viewport,
            );

            if let (Some(nested_layout), Some(nested_i)) = (children.next(), nested_open)
                && let Some(Entry::Sub(nested)) = sub.entries.get(nested_i)
            {
                draw_menu_surface(
                    renderer,
                    nested_layout.bounds(),
                    &nested.entries,
                    None,
                    None,
                    self.recipe,
                    self.sub_style,
                    self.item_font,
                    self.label_font,
                    self.shortcut_font,
                    &self.viewport,
                );
            }
        }
    }
}

fn nav_action(key: &keyboard::Key) -> Option<NavAction> {
    let nav_key = match key {
        keyboard::Key::Named(keyboard::key::Named::ArrowDown) => NavKey::ArrowDown,
        keyboard::Key::Named(keyboard::key::Named::ArrowUp) => NavKey::ArrowUp,
        keyboard::Key::Named(keyboard::key::Named::Home) => NavKey::Home,
        keyboard::Key::Named(keyboard::key::Named::End) => NavKey::End,
        keyboard::Key::Named(keyboard::key::Named::Enter)
        | keyboard::Key::Named(keyboard::key::Named::Space) => NavKey::Enter,
        _ => return None,
    };

    resolve_nav_action(nav_key, Orientation::Vertical, Direction::Ltr)
}

fn move_hover<Message>(
    entries: &[Entry<Message>],
    hovered: &mut Option<usize>,
    direction: isize,
    shell: &mut Shell<'_, Message>,
) {
    if let Some(index) =
        shadcn_common::step_index(entries, *hovered, direction, true, Entry::is_selectable)
            .filter(|&index| *hovered != Some(index))
    {
        *hovered = Some(index);
        shell.request_redraw();
    }
}

/// Cursor-anchored placement for the context-menu root panel.
///
/// Uses [`compute_floating`] so `Left` / `Right` / `Top` / `Bottom` all shift
/// the panel on the correct axis (the previous hand-rolled path only offset Y
/// for top/bottom and left X alone unchanged for left/right).
pub(super) fn cursor_menu_origin(
    anchor: Point,
    width: f32,
    height: f32,
    viewport: Size,
    side: Option<FloatingSide>,
    side_offset: f32,
) -> Point {
    let preferred = side.unwrap_or(FloatingSide::Bottom);
    let config = FloatingConfig::default()
        .side(preferred)
        .align(FloatingAlign::Start)
        .side_offset(side_offset)
        .avoid_collisions(true)
        .collision_padding(FloatingPadding::all(CONTEXT_MENU_FLIP_SLACK_PX));

    let placement = compute_floating(
        FloatingRect::new(anchor.x, anchor.y, 0.0, 0.0),
        width,
        height,
        FloatingRect::new(0.0, 0.0, viewport.width, viewport.height),
        &config,
    );

    Point::new(placement.x, placement.y)
}

pub(super) fn menu_height<Message>(entries: &[Entry<Message>], recipe: ContextMenuRecipe) -> f32 {
    recipe.content_pad_px * 2.0
        + (0..entries.len())
            .map(|index| row_height(entries, index, recipe))
            .sum::<f32>()
}

/// Panel height: grow with every row, capped only by the free viewport strip.
fn fitted_menu_height(content_height: f32, available: f32) -> f32 {
    content_height.min(available.max(0.0))
}

fn row_height<Message>(entries: &[Entry<Message>], index: usize, recipe: ContextMenuRecipe) -> f32 {
    match entries.get(index) {
        Some(Entry::Separator) => recipe.separator_margin_y_px * 2.0 + 1.0,
        Some(Entry::Label(_)) => {
            recipe.label_typography.line_height_px + recipe.label_pad_y_px * 2.0
        }
        _ => {
            let body = recipe
                .item_typography
                .line_height_px
                .max(recipe.item_typography.size_px + 6.0)
                + recipe.item_pad_y_px * 2.0;
            body.max(recipe.item_min_height_px)
        }
    }
}

fn row_offset_y<Message>(
    entries: &[Entry<Message>],
    index: usize,
    recipe: ContextMenuRecipe,
) -> f32 {
    recipe.content_pad_px
        + (0..index)
            .map(|i| row_height(entries, i, recipe))
            .sum::<f32>()
}

fn row_at<Message>(
    entries: &[Entry<Message>],
    position: Point,
    recipe: ContextMenuRecipe,
) -> Option<usize> {
    let mut y = recipe.content_pad_px;
    if position.y < y {
        return None;
    }
    for index in 0..entries.len() {
        let height = row_height(entries, index, recipe);
        if position.y < y + height {
            return Some(index);
        }
        y += height;
    }
    None
}

fn activate_index<Message: Clone>(
    entries: &mut [Entry<Message>],
    index: usize,
    shell: &mut Shell<'_, Message>,
) -> bool {
    match entries.get_mut(index) {
        Some(Entry::Item(item)) if !item.disabled => {
            if let Some(message) = item.on_select.clone() {
                shell.publish(message);
            }
            item.close_on_select
        }
        Some(Entry::Checkbox(item)) if !item.disabled => {
            if let Some(message) = item.on_toggle.clone() {
                shell.publish(message);
            }
            false
        }
        Some(Entry::Radio(item)) if !item.disabled => {
            if let Some(message) = item.on_select.clone() {
                shell.publish(message);
            }
            item.close_on_select
        }
        _ => false,
    }
}

/// Returns a new open-path when the press lands on a submenu trigger.
fn maybe_open_submenu<Message>(
    entries: &[Entry<Message>],
    path: &[usize],
    position: Point,
    recipe: ContextMenuRecipe,
) -> Option<Vec<usize>> {
    match path {
        [] => {
            let index = row_at(entries, position, recipe)?;
            entries[index].is_submenu().then(|| vec![index])
        }
        &[sub_index] => {
            let Entry::Sub(sub) = entries.get(sub_index)? else {
                return None;
            };
            let index = row_at(&sub.entries, position, recipe)?;
            sub.entries[index]
                .is_submenu()
                .then(|| vec![sub_index, index])
        }
        _ => None,
    }
}

fn activate_at_path_pos<Message: Clone>(
    entries: &mut [Entry<Message>],
    path: &[usize],
    position: Point,
    recipe: ContextMenuRecipe,
    shell: &mut Shell<'_, Message>,
) -> bool {
    match path {
        [] => {
            if let Some(index) = row_at(entries, position, recipe) {
                activate_index(entries, index, shell)
            } else {
                false
            }
        }
        &[sub_index] => {
            if let Some(Entry::Sub(sub)) = entries.get_mut(sub_index)
                && let Some(index) = row_at(&sub.entries, position, recipe)
            {
                activate_index(&mut sub.entries, index, shell)
            } else {
                false
            }
        }
        &[sub_index, nested_index, ..] => {
            if let Some(Entry::Sub(sub)) = entries.get_mut(sub_index)
                && let Some(Entry::Sub(nested)) = sub.entries.get_mut(nested_index)
                && let Some(index) = row_at(&nested.entries, position, recipe)
            {
                activate_index(&mut nested.entries, index, shell)
            } else {
                false
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn draw_menu_surface<Message>(
    renderer: &mut Renderer,
    bounds: Rectangle,
    entries: &[Entry<Message>],
    hovered: Option<usize>,
    open_sub: Option<usize>,
    recipe: ContextMenuRecipe,
    style: ContextMenuContentStyle,
    item_font: Font,
    label_font: Font,
    shortcut_font: Font,
    viewport: &Rectangle,
) {
    crate::floating_surface::fill_floating_surface(
        renderer,
        bounds,
        style.background,
        style.radius,
        style.shadow,
    );

    let mut y = bounds.y + recipe.content_pad_px;

    for (index, entry) in entries.iter().enumerate() {
        let height = row_height(entries, index, recipe);
        let row_bounds = Rectangle {
            x: bounds.x + recipe.content_pad_px,
            y,
            width: (bounds.width - recipe.content_pad_px * 2.0).max(0.0),
            height,
        };

        match entry {
            Entry::Separator => {
                let sep_y = y + recipe.separator_margin_y_px;
                renderer.fill_quad(
                    renderer::Quad {
                        bounds: Rectangle {
                            x: bounds.x + recipe.content_pad_px - recipe.separator_margin_x_px,
                            y: sep_y,
                            width: (bounds.width - recipe.content_pad_px * 2.0
                                + recipe.separator_margin_x_px * 2.0)
                                .max(0.0),
                            height: 1.0,
                        },
                        ..renderer::Quad::default()
                    },
                    Background::Color(style.separator_color),
                );
            }
            Entry::Label(label) => {
                let pad_left = if label.inset {
                    recipe.label_inset_pad_left_px
                } else {
                    recipe.label_pad_x_px
                };
                renderer.fill_text(
                    Text {
                        content: label.text.clone(),
                        bounds: Size::new(
                            (row_bounds.width - pad_left - recipe.label_pad_x_px).max(0.0),
                            recipe.label_typography.line_height_px,
                        ),
                        size: Pixels(recipe.label_typography.size_px),
                        line_height: core_text::LineHeight::Absolute(Pixels(
                            recipe.label_typography.line_height_px,
                        )),
                        font: label_font,
                        align_x: core_text::Alignment::Default,
                        align_y: alignment::Vertical::Center,
                        shaping: core_text::Shaping::Advanced,
                        wrapping: core_text::Wrapping::None,
                    },
                    Point::new(row_bounds.x + pad_left, row_bounds.center_y()),
                    style.muted_color,
                    *viewport,
                );
            }
            Entry::Item(item) => {
                let highlighted = hovered == Some(index);
                draw_item_row(
                    renderer,
                    row_bounds,
                    &item.label,
                    item.shortcut.as_deref(),
                    item.variant,
                    item.disabled,
                    item.inset,
                    false,
                    false,
                    false,
                    highlighted,
                    recipe,
                    style,
                    item_font,
                    shortcut_font,
                    viewport,
                );
            }
            Entry::Checkbox(item) => {
                let highlighted = hovered == Some(index);
                draw_item_row(
                    renderer,
                    row_bounds,
                    &item.label,
                    None,
                    MenuItemVariant::Default,
                    item.disabled,
                    item.inset,
                    true,
                    item.checked,
                    false,
                    highlighted,
                    recipe,
                    style,
                    item_font,
                    shortcut_font,
                    viewport,
                );
            }
            Entry::Radio(item) => {
                let highlighted = hovered == Some(index);
                draw_item_row(
                    renderer,
                    row_bounds,
                    &item.label,
                    None,
                    MenuItemVariant::Default,
                    item.disabled,
                    item.inset,
                    true,
                    item.selected,
                    true,
                    highlighted,
                    recipe,
                    style,
                    item_font,
                    shortcut_font,
                    viewport,
                );
            }
            Entry::Sub(sub) => {
                let highlighted = hovered == Some(index) || open_sub == Some(index);
                draw_item_row(
                    renderer,
                    row_bounds,
                    &sub.label,
                    None,
                    MenuItemVariant::Default,
                    sub.disabled,
                    sub.inset,
                    false,
                    false,
                    false,
                    highlighted,
                    recipe,
                    style,
                    item_font,
                    shortcut_font,
                    viewport,
                );
                // Chevron-right
                let (_text, _muted, icon_color) =
                    item_colors(style, MenuItemVariant::Default, highlighted, sub.disabled);
                let icon_size = recipe.item_icon_size_px;
                let center = Point::new(
                    row_bounds.x + row_bounds.width - recipe.item_pad_x_px - icon_size / 2.0,
                    row_bounds.center_y(),
                );
                draw_chevron_right(renderer, center, icon_size, icon_color);
            }
        }

        y += height;
    }

    crate::floating_surface::paint_outside_ring(
        renderer,
        bounds,
        style.border_color,
        style.border_width,
        style.radius,
    );
}

#[allow(clippy::too_many_arguments)]
fn draw_item_row(
    renderer: &mut Renderer,
    row_bounds: Rectangle,
    label: &str,
    shortcut: Option<&str>,
    variant: MenuItemVariant,
    disabled: bool,
    inset: bool,
    has_indicator: bool,
    indicator_on: bool,
    radio: bool,
    highlighted: bool,
    recipe: ContextMenuRecipe,
    style: ContextMenuContentStyle,
    item_font: Font,
    shortcut_font: Font,
    viewport: &Rectangle,
) {
    if let Some(fill) = item_highlight_fill(style, variant, highlighted) {
        renderer.fill_quad(
            renderer::Quad {
                bounds: row_bounds,
                border: Border {
                    radius: style.item_radius.into(),
                    ..Border::default()
                },
                ..renderer::Quad::default()
            },
            Background::Color(fill),
        );
    }

    let (text_color, muted_color, icon_color) = item_colors(style, variant, highlighted, disabled);
    let pad_left = if inset {
        recipe.item_inset_pad_left_px
    } else {
        recipe.item_pad_x_px
    };
    let pad_right = if has_indicator {
        recipe.item_indicator_pad_right_px
    } else {
        recipe.item_pad_x_px
    };

    let mut label_max = (row_bounds.width - pad_left - pad_right).max(0.0);
    if shortcut.is_some() {
        label_max = (label_max - 48.0).max(0.0);
    }

    renderer.fill_text(
        Text {
            content: if recipe.item_typography.uppercase {
                label.to_uppercase()
            } else {
                label.to_owned()
            },
            bounds: Size::new(label_max, recipe.item_typography.line_height_px),
            size: Pixels(recipe.item_typography.size_px),
            line_height: core_text::LineHeight::Absolute(Pixels(
                recipe.item_typography.line_height_px,
            )),
            font: item_font,
            align_x: core_text::Alignment::Default,
            align_y: alignment::Vertical::Center,
            shaping: core_text::Shaping::Advanced,
            wrapping: core_text::Wrapping::None,
        },
        Point::new(row_bounds.x + pad_left, row_bounds.center_y()),
        text_color,
        *viewport,
    );

    if let Some(shortcut) = shortcut {
        renderer.fill_text(
            Text {
                content: shortcut.to_owned(),
                bounds: Size::new(
                    (row_bounds.width / 2.0).max(0.0),
                    recipe.shortcut_typography.line_height_px,
                ),
                size: Pixels(recipe.shortcut_typography.size_px),
                line_height: core_text::LineHeight::Absolute(Pixels(
                    recipe.shortcut_typography.line_height_px,
                )),
                font: shortcut_font,
                align_x: core_text::Alignment::Right,
                align_y: alignment::Vertical::Center,
                shaping: core_text::Shaping::Advanced,
                wrapping: core_text::Wrapping::None,
            },
            Point::new(
                row_bounds.x + row_bounds.width - recipe.item_pad_x_px,
                row_bounds.center_y(),
            ),
            muted_color,
            *viewport,
        );
    }

    if has_indicator && indicator_on {
        let size = recipe.item_indicator_size_px;
        let center = Point::new(
            row_bounds.x + row_bounds.width - recipe.item_indicator_right_px - size / 2.0,
            row_bounds.center_y(),
        );
        if radio {
            draw_radio_dot(renderer, center, size, icon_color);
        } else {
            draw_check(renderer, center, size, icon_color);
        }
    }
}

fn draw_check(renderer: &mut Renderer, center: Point, size: f32, color: Color) {
    if size <= 0.0 {
        return;
    }
    let stroke_width = (size * 0.12).clamp(1.0, 2.0);
    let mut frame = canvas::Frame::new(renderer, Size::new(size, size));
    frame.translate(Vector::new(size / 2.0, size / 2.0));
    frame.stroke(
        &canvas::Path::new(|builder| {
            builder.move_to(Point::new(-size * 0.28, 0.0));
            builder.line_to(Point::new(-size * 0.06, size * 0.22));
            builder.line_to(Point::new(size * 0.30, -size * 0.24));
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
        |renderer| renderer.draw_geometry(geometry),
    );
}

fn draw_radio_dot(renderer: &mut Renderer, center: Point, size: f32, color: Color) {
    if size <= 0.0 {
        return;
    }
    let radius = size * 0.22;
    renderer.fill_quad(
        renderer::Quad {
            bounds: Rectangle {
                x: center.x - radius,
                y: center.y - radius,
                width: radius * 2.0,
                height: radius * 2.0,
            },
            border: Border {
                radius: radius.into(),
                ..Border::default()
            },
            ..renderer::Quad::default()
        },
        Background::Color(color),
    );
}

fn draw_chevron_right(renderer: &mut Renderer, center: Point, size: f32, color: Color) {
    if size <= 0.0 {
        return;
    }
    let reach = size * 0.18;
    let arm = size * 0.28;
    let stroke_width = (size * 0.10).clamp(1.0, 1.75);
    let mut frame = canvas::Frame::new(renderer, Size::new(size, size));
    frame.translate(Vector::new(size / 2.0, size / 2.0));
    frame.stroke(
        &canvas::Path::new(|builder| {
            builder.move_to(Point::new(-reach, -arm));
            builder.line_to(Point::new(reach, 0.0));
            builder.line_to(Point::new(-reach, arm));
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
        |renderer| renderer.draw_geometry(geometry),
    );
}
