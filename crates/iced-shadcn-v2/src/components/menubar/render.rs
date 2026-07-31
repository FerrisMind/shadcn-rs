//! Custom widget and overlay rendering for [`super::Menubar`].
//!
//! The widget wraps a trigger element. While open, an iced overlay paints the
//! `.cn-menubar-content` surface with rows (items, checkboxes, radios,
//! labels, separators, sub-triggers) and optional nested
//! `.cn-menubar-sub-content` panels. Keyboard navigation matches bits-ui
//! (arrow keys, Enter/Space, Esc, ArrowRight/Left for submenus).

use iced_core::keyboard;
use iced_core::text::paragraph;
use iced_core::text::{self as core_text, Renderer as _, Text};

use shadcn_common::{
    Direction, DropdownMenuRecipe, MENU_SUB_SIDE_OFFSET_PX, MenuItemVariant, MenubarRecipe,
    NavAction, NavKey, Orientation, resolve_nav_action,
};

use crate::iced_compat::advanced::renderer::Renderer as _;
use crate::iced_compat::advanced::widget::{Tree, tree};
use crate::iced_compat::advanced::{Clipboard, Shell, Widget, layout, overlay, renderer};
use crate::iced_compat::widget::canvas;
use crate::iced_compat::widget::graphics::geometry::Renderer as _;
use crate::iced_compat::{
    Background, Border, Color, Event, Font, Length, Pixels, Point, Rectangle, Renderer, Size,
    Theme as IcedTheme, Vector, alignment, mouse, touch,
};

type ParagraphOf = <Renderer as core_text::Renderer>::Paragraph;

use super::style::{self, MenubarContentStyle, item_colors, item_highlight_fill};
use super::types::{Entry, MenubarMenu};
use crate::fonts::iced_font;
use crate::recipes::{component_radius_px, iced_font_weight};
use crate::theme::Theme;

/// Internal widget produced by the [`super::Menubar`] builder.
pub(super) struct MenubarWidget<'a, Message> {
    pub(super) theme: &'a Theme,
    pub(super) menus: Vec<MenubarMenu<Message>>,
    pub(super) width: Option<f32>,
    pub(super) side_offset: f32,
    pub(super) align_offset: f32,
    pub(super) disabled: bool,
    pub(super) open_override: Option<usize>,
    pub(super) default_open_menu: Option<usize>,
    pub(super) on_open: Option<Message>,
    pub(super) on_close: Option<Message>,
    pub(super) on_open_change: Option<Box<dyn Fn(bool) -> Message + 'a>>,
    pub(super) style_override: Option<Box<dyn Fn(MenubarContentStyle) -> MenubarContentStyle + 'a>>,
}

/// Widget-tree state of the bar and its open menu.
struct State {
    /// Index of the open top-level menu, or `None` when closed.
    open_menu: Option<usize>,
    hovered_trigger: Option<usize>,
    hovered: Option<usize>,
    open_path: Vec<usize>,
    hovered_sub: Option<usize>,
    suppress_next_trigger_press: bool,
}

impl State {
    fn new(default_open_menu: Option<usize>) -> Self {
        Self {
            open_menu: default_open_menu,
            hovered_trigger: None,
            hovered: None,
            open_path: Vec::new(),
            hovered_sub: None,
            suppress_next_trigger_press: false,
        }
    }
}

impl<Message> MenubarWidget<'_, Message>
where
    Message: Clone,
{
    fn open_index(&self, state: &State) -> Option<usize> {
        if self.disabled {
            return None;
        }
        self.open_override.or(state.open_menu)
    }

    fn is_open(&self, state: &State) -> bool {
        self.open_index(state).is_some()
    }

    fn resolve_style(&self, submenu: bool) -> MenubarContentStyle {
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

    fn set_open_menu(
        &self,
        state: &mut State,
        index: Option<usize>,
        shell: &mut Shell<'_, Message>,
    ) {
        if self.open_override.is_some() {
            self.publish_open_change(index.is_some(), shell);
            return;
        }

        if state.open_menu == index {
            return;
        }

        let was_open = state.open_menu.is_some();
        let now_open = index.is_some();
        state.open_menu = index;
        state.open_path.clear();
        state.hovered_sub = None;
        if let Some(i) = index {
            state.hovered = self
                .menus
                .get(i)
                .and_then(|menu| menu.entries.iter().position(Entry::is_selectable));
        } else {
            state.hovered = None;
        }

        if was_open != now_open {
            self.publish_open_change(now_open, shell);
        }
        shell.request_redraw();
    }

    fn handle_trigger_press(
        &self,
        state: &mut State,
        index: usize,
        shell: &mut Shell<'_, Message>,
    ) {
        if state.suppress_next_trigger_press {
            state.suppress_next_trigger_press = false;
            return;
        }

        if self.disabled {
            return;
        }

        let Some(menu) = self.menus.get(index) else {
            return;
        };
        if menu.disabled {
            return;
        }

        let next = if state.open_menu == Some(index) {
            None
        } else {
            Some(index)
        };
        self.set_open_menu(state, next, shell);
    }

    fn measure_trigger(
        &self,
        _renderer: &Renderer,
        label: &str,
        recipe: MenubarRecipe,
        font: Font,
    ) -> Size {
        // Measure with the same paragraph path as select/button so trigger
        // width matches glyph advance (char*0.55 under-sized and left-flush).
        let line_height = recipe.trigger_typography.line_height_px;
        let mut paragraph = paragraph::Plain::<ParagraphOf>::default();
        let _ = paragraph.update(Text {
            content: label,
            bounds: Size::new(f32::INFINITY, line_height),
            size: Pixels(recipe.trigger_typography.size_px),
            line_height: core_text::LineHeight::Absolute(Pixels(line_height)),
            font,
            align_x: core_text::Alignment::Default,
            align_y: alignment::Vertical::Center,
            shaping: core_text::Shaping::Advanced,
            wrapping: core_text::Wrapping::None,
        });
        Size::new(
            paragraph.min_width() + recipe.trigger_pad_x_px * 2.0,
            line_height + recipe.trigger_pad_y_px * 2.0,
        )
    }
}

impl<'a, Message> Widget<Message, IcedTheme, Renderer> for MenubarWidget<'a, Message>
where
    Message: Clone + 'a,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::new(self.default_open_menu))
    }

    fn size(&self) -> Size<Length> {
        Size::new(Length::Shrink, Length::Shrink)
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let recipe = style::recipe(self.theme);
        let font = iced_font(self.theme.font_pack().sans);
        let mut trigger_font = font;
        trigger_font.weight = iced_font_weight(recipe.trigger_typography.weight);

        // `.cn-menubar` is `flex items-center`: triggers size to content and
        // sit vertically centered in the padded bar (they must not overflow).
        let mut trigger_nodes = Vec::with_capacity(self.menus.len());
        let mut x = recipe.bar_pad_px;
        let inner_height = (recipe.bar_height_px - recipe.bar_pad_px * 2.0).max(0.0);

        for menu in &self.menus {
            let size = self.measure_trigger(renderer, &menu.trigger, recipe, trigger_font);
            let h = size.height.min(inner_height);
            let node = layout::Node::new(Size::new(size.width, h)).move_to(Point::new(
                x,
                recipe.bar_pad_px + ((inner_height - h) * 0.5).max(0.0),
            ));
            x += size.width + recipe.bar_gap_px;
            trigger_nodes.push(node);
        }

        if !self.menus.is_empty() {
            x -= recipe.bar_gap_px;
        }
        x += recipe.bar_pad_px;

        let resolved = limits.resolve(
            Length::Shrink,
            Length::Shrink,
            Size::new(x, recipe.bar_height_px),
        );
        layout::Node::with_children(
            Size::new(resolved.width.max(x), recipe.bar_height_px),
            trigger_nodes,
        )
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: layout::Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_mut::<State>();
        let children: Vec<_> = layout.children().collect();

        match event {
            Event::Mouse(mouse::Event::CursorMoved { .. })
            | Event::Touch(touch::Event::FingerMoved { .. }) => {
                let mut next_hover = None;
                if let Some(cursor_pos) = cursor.position() {
                    for (index, child) in children.iter().enumerate() {
                        if child.bounds().contains(cursor_pos) {
                            next_hover = Some(index);
                            break;
                        }
                    }
                }
                if state.hovered_trigger != next_hover {
                    state.hovered_trigger = next_hover;
                    shell.request_redraw();
                }

                if let (Some(open), Some(hover)) = (self.open_index(state), next_hover)
                    && open != hover
                    && self.menus.get(hover).is_some_and(|menu| !menu.disabled)
                {
                    self.set_open_menu(state, Some(hover), shell);
                }
            }
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                let mut hit = None;
                if let Some(cursor_pos) = cursor.position() {
                    for (index, child) in children.iter().enumerate() {
                        if child.bounds().contains(cursor_pos) {
                            hit = Some(index);
                            break;
                        }
                    }
                }

                if let Some(index) = hit {
                    self.handle_trigger_press(state, index, shell);
                    shell.capture_event();
                } else if self.is_open(state) {
                    state.suppress_next_trigger_press = false;
                    self.set_open_menu(state, None, shell);
                } else {
                    state.suppress_next_trigger_press = false;
                }
            }
            Event::Keyboard(keyboard::Event::KeyPressed {
                key: keyboard::Key::Named(keyboard::key::Named::Escape),
                ..
            }) => {
                if self.is_open(state) {
                    if !state.open_path.is_empty() {
                        state.open_path.pop();
                        state.hovered_sub = None;
                        shell.capture_event();
                        shell.request_redraw();
                    } else {
                        self.set_open_menu(state, None, shell);
                        shell.capture_event();
                    }
                }
            }
            Event::Keyboard(keyboard::Event::KeyPressed { key, .. }) => {
                if !self.is_open(state) {
                    return;
                }
                let nav_key = match key {
                    keyboard::Key::Named(keyboard::key::Named::ArrowLeft) => NavKey::ArrowLeft,
                    keyboard::Key::Named(keyboard::key::Named::ArrowRight) => NavKey::ArrowRight,
                    _ => return,
                };
                let Some(nav) =
                    resolve_nav_action(nav_key, Orientation::Horizontal, Direction::Ltr)
                else {
                    return;
                };

                let Some(current) = self.open_index(state) else {
                    return;
                };
                let enabled: Vec<usize> = self
                    .menus
                    .iter()
                    .enumerate()
                    .filter(|(_, menu)| !menu.disabled)
                    .map(|(i, _)| i)
                    .collect();
                if enabled.is_empty() {
                    return;
                }
                let pos = enabled.iter().position(|&i| i == current).unwrap_or(0);
                let next = match nav {
                    NavAction::Previous => enabled[(pos + enabled.len() - 1) % enabled.len()],
                    NavAction::Next => enabled[(pos + 1) % enabled.len()],
                    NavAction::First => enabled[0],
                    NavAction::Last => *enabled.last().unwrap_or(&current),
                    NavAction::Activate | _ => return,
                };
                self.set_open_menu(state, Some(next), shell);
                shell.capture_event();
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
        if cursor.is_over(layout.bounds()) {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::None
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        _theme: &IcedTheme,
        _style: &renderer::Style,
        layout: layout::Layout<'_>,
        _cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_ref::<State>();
        let recipe = style::recipe(self.theme);
        let bar_style = style::resolve_bar_style(self.theme);
        let font = iced_font(self.theme.font_pack().sans);
        let mut trigger_font = font;
        trigger_font.weight = iced_font_weight(recipe.trigger_typography.weight);

        let bounds = layout.bounds();
        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border: Border {
                    color: bar_style.border_color,
                    width: bar_style.border_width,
                    radius: bar_style.radius.into(),
                },
                shadow: bar_style.shadow.unwrap_or_default(),
                ..renderer::Quad::default()
            },
            Background::Color(bar_style.background),
        );

        let open = self.open_index(state);
        for (index, (menu, child)) in self.menus.iter().zip(layout.children()).enumerate() {
            let trigger_bounds = child.bounds();
            let active = open == Some(index) || state.hovered_trigger == Some(index);
            if active && !menu.disabled {
                renderer.fill_quad(
                    renderer::Quad {
                        bounds: trigger_bounds,
                        border: Border {
                            radius: component_radius_px(self.theme, recipe.trigger_radius).into(),
                            ..Border::default()
                        },
                        ..renderer::Quad::default()
                    },
                    Background::Color(bar_style.trigger_muted),
                );
            }

            let text_color = if menu.disabled || self.disabled {
                bar_style
                    .trigger_text
                    .scale_alpha(bar_style.disabled_opacity)
            } else {
                bar_style.trigger_text
            };

            renderer.fill_text(
                Text {
                    content: menu.trigger.clone(),
                    bounds: Size::new(
                        (trigger_bounds.width - recipe.trigger_pad_x_px * 2.0).max(0.0),
                        recipe.trigger_typography.line_height_px,
                    ),
                    size: Pixels(recipe.trigger_typography.size_px),
                    line_height: core_text::LineHeight::Absolute(Pixels(
                        recipe.trigger_typography.line_height_px,
                    )),
                    font: trigger_font,
                    align_x: core_text::Alignment::Default,
                    align_y: alignment::Vertical::Center,
                    shaping: core_text::Shaping::Advanced,
                    wrapping: core_text::Wrapping::None,
                },
                Point::new(
                    trigger_bounds.x + recipe.trigger_pad_x_px,
                    trigger_bounds.center_y(),
                ),
                text_color,
                *viewport,
            );
        }
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: layout::Layout<'b>,
        _renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, IcedTheme, Renderer>> {
        let state = tree.state.downcast_mut::<State>();
        let open = self.open_index(state)?;
        let children: Vec<_> = layout.children().collect();
        let trigger_layout = children.get(open)?;
        let trigger_bounds = trigger_layout.bounds() + translation;

        let content_style = self.resolve_style(false);
        let sub_style = self.resolve_style(true);
        let menubar_recipe = style::recipe(self.theme);
        let recipe = menubar_recipe.menu;
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

        let menu = self.menus.get_mut(open)?;

        let mut sub_recipe = recipe;
        sub_recipe.content_pad_px = menubar_recipe.sub_content_pad_px;

        Some(
            MenuOverlay {
                position: Point::new(trigger_bounds.x + self.align_offset, trigger_bounds.y),
                trigger_size: Size::new(trigger_bounds.width, trigger_bounds.height),
                viewport: *viewport,
                entries: &mut menu.entries,
                width,
                side_offset: self.side_offset,
                recipe,
                sub_recipe,
                indicator_leading: menubar_recipe.indicator_leading,
                item_indicator_left_px: menubar_recipe.item_indicator_left_px,
                checkable_item_radius: content_style.checkable_item_radius,
                content_style,
                sub_style,
                item_font,
                label_font,
                shortcut_font,
                hovered: &mut state.hovered,
                open_path: &mut state.open_path,
                hovered_sub: &mut state.hovered_sub,
                close_menu_slot: &mut state.open_menu,
                open_override: self.open_override,
                on_close_msg: self.on_close.clone(),
                on_open_change: self.on_open_change.as_deref(),
                suppress_next_trigger_press: &mut state.suppress_next_trigger_press,
            }
            .element(),
        )
    }
}

/// Root + nested submenu overlay.
struct MenuOverlay<'a, Message> {
    position: Point,
    trigger_size: Size,
    viewport: Rectangle,
    entries: &'a mut [Entry<Message>],
    width: f32,
    side_offset: f32,
    /// Root panel tokens (`menubar-content.svelte` hardcode for pad/radius).
    recipe: DropdownMenuRecipe,
    /// Nested panel tokens (`.cn-menubar-sub-content` pad; shared item metrics).
    sub_recipe: DropdownMenuRecipe,
    indicator_leading: bool,
    item_indicator_left_px: f32,
    checkable_item_radius: f32,
    content_style: MenubarContentStyle,
    sub_style: MenubarContentStyle,
    item_font: Font,
    label_font: Font,
    shortcut_font: Font,
    hovered: &'a mut Option<usize>,
    open_path: &'a mut Vec<usize>,
    hovered_sub: &'a mut Option<usize>,
    close_menu_slot: &'a mut Option<usize>,
    open_override: Option<usize>,
    on_close_msg: Option<Message>,
    on_open_change: Option<&'a dyn Fn(bool) -> Message>,
    suppress_next_trigger_press: &'a mut bool,
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
            *self.close_menu_slot = None;
        }
        self.open_path.clear();
        *self.hovered = None;
        *self.hovered_sub = None;
        *self.suppress_next_trigger_press = true;

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

    fn root_origin(&self, bounds: Size) -> Point {
        let space_below =
            bounds.height - (self.position.y + self.trigger_size.height + self.side_offset);
        let space_above = self.position.y - self.side_offset;
        let open_below = space_below >= space_above;
        let available = if open_below { space_below } else { space_above };
        // Prefer full content height; only shrink to the free viewport strip
        // (bits-ui `max-h-(--available-height)`). Do not apply a fixed
        // `max-h-96` without a scrollport — that clipped the last rows while
        // they were still painted.
        let height = fitted_menu_height(self.root_height(), available);

        if open_below {
            Point::new(
                self.position.x,
                self.position.y + self.trigger_size.height + self.side_offset,
            )
        } else {
            Point::new(
                self.position.x,
                (self.position.y - self.side_offset - height).max(0.0),
            )
        }
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
                fitted_menu_height(menu_height(&sub.entries, self.sub_recipe), sub_available);
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
                let nested_row_y = row_offset_y(&sub.entries, nested_index, self.sub_recipe);
                let nested_available = (bounds.height - (sub_origin.y + nested_row_y)).max(0.0);
                let nested_height = fitted_menu_height(
                    menu_height(&nested.entries, self.sub_recipe),
                    nested_available,
                );
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
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                let path = self.open_path.clone();

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
                    let open_sub = maybe_open_submenu(
                        self.entries,
                        &activate_path,
                        pos,
                        self.recipe,
                        self.sub_recipe,
                    );
                    if let Some(next_path) = open_sub {
                        *self.open_path = next_path;
                        *self.hovered = self.open_path.first().copied().or(*self.hovered);
                        *self.hovered_sub = self.open_path.get(1).copied();
                        shell.request_redraw();
                    } else {
                        let close = activate_at_path_pos(
                            self.entries,
                            &activate_path,
                            pos,
                            self.recipe,
                            self.sub_recipe,
                            shell,
                        );
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
                    let hovered = row_at(&sub.entries, pos, self.sub_recipe)
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
            if let Some(pos) = cursor.position_in(child.bounds()) {
                // Approximate: any menu surface uses pointer over selectable area.
                let _ = pos;
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
            self.indicator_leading,
            self.item_indicator_left_px,
            self.checkable_item_radius,
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
                self.sub_recipe,
                self.sub_style,
                self.item_font,
                self.label_font,
                self.shortcut_font,
                &self.viewport,
                self.indicator_leading,
                self.item_indicator_left_px,
                self.checkable_item_radius,
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
                    self.sub_recipe,
                    self.sub_style,
                    self.item_font,
                    self.label_font,
                    self.shortcut_font,
                    &self.viewport,
                    self.indicator_leading,
                    self.item_indicator_left_px,
                    self.checkable_item_radius,
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

pub(super) fn menu_height<Message>(entries: &[Entry<Message>], recipe: DropdownMenuRecipe) -> f32 {
    recipe.content_pad_px * 2.0
        + (0..entries.len())
            .map(|index| row_height(entries, index, recipe))
            .sum::<f32>()
}

/// Panel height: grow with every row, capped only by the free viewport strip.
///
/// Web `max-h-96` ships with `overflow-y-auto`. Applying that ceiling without a
/// scrollport clipped the surface while rows kept painting underneath — the
/// last item floated outside the white panel. Until a scrollport exists, size
/// to content and only shrink when the window itself runs out of space.
fn fitted_menu_height(content_height: f32, available: f32) -> f32 {
    content_height.min(available.max(0.0))
}

fn row_height<Message>(
    entries: &[Entry<Message>],
    index: usize,
    recipe: DropdownMenuRecipe,
) -> f32 {
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
    recipe: DropdownMenuRecipe,
) -> f32 {
    recipe.content_pad_px
        + (0..index)
            .map(|i| row_height(entries, i, recipe))
            .sum::<f32>()
}

fn row_at<Message>(
    entries: &[Entry<Message>],
    position: Point,
    recipe: DropdownMenuRecipe,
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
    root_recipe: DropdownMenuRecipe,
    sub_recipe: DropdownMenuRecipe,
) -> Option<Vec<usize>> {
    match path {
        [] => {
            let index = row_at(entries, position, root_recipe)?;
            entries[index].is_submenu().then(|| vec![index])
        }
        &[sub_index] => {
            let Entry::Sub(sub) = entries.get(sub_index)? else {
                return None;
            };
            let index = row_at(&sub.entries, position, sub_recipe)?;
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
    root_recipe: DropdownMenuRecipe,
    sub_recipe: DropdownMenuRecipe,
    shell: &mut Shell<'_, Message>,
) -> bool {
    match path {
        [] => {
            if let Some(index) = row_at(entries, position, root_recipe) {
                activate_index(entries, index, shell)
            } else {
                false
            }
        }
        &[sub_index] => {
            if let Some(Entry::Sub(sub)) = entries.get_mut(sub_index)
                && let Some(index) = row_at(&sub.entries, position, sub_recipe)
            {
                activate_index(&mut sub.entries, index, shell)
            } else {
                false
            }
        }
        &[sub_index, nested_index, ..] => {
            if let Some(Entry::Sub(sub)) = entries.get_mut(sub_index)
                && let Some(Entry::Sub(nested)) = sub.entries.get_mut(nested_index)
                && let Some(index) = row_at(&nested.entries, position, sub_recipe)
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
    recipe: DropdownMenuRecipe,
    style: MenubarContentStyle,
    item_font: Font,
    label_font: Font,
    shortcut_font: Font,
    viewport: &Rectangle,
    indicator_leading: bool,
    item_indicator_left_px: f32,
    checkable_item_radius: f32,
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
                    indicator_leading,
                    item_indicator_left_px,
                    style.item_radius,
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
                    indicator_leading,
                    item_indicator_left_px,
                    checkable_item_radius,
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
                    indicator_leading,
                    item_indicator_left_px,
                    checkable_item_radius,
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
                    indicator_leading,
                    item_indicator_left_px,
                    style.item_radius,
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
    recipe: DropdownMenuRecipe,
    style: MenubarContentStyle,
    item_font: Font,
    shortcut_font: Font,
    viewport: &Rectangle,
    indicator_leading: bool,
    item_indicator_left_px: f32,
    item_radius: f32,
) {
    if let Some(fill) = item_highlight_fill(style, variant, highlighted) {
        renderer.fill_quad(
            renderer::Quad {
                bounds: row_bounds,
                border: Border {
                    radius: item_radius.into(),
                    ..Border::default()
                },
                ..renderer::Quad::default()
            },
            Background::Color(fill),
        );
    }

    let (text_color, muted_color, icon_color) = item_colors(style, variant, highlighted, disabled);
    // Leading indicators use `pl-*` / `data-inset:pl-*` (= item_inset_pad_left_px),
    // not the trailing `pr-*` slot used by dropdown-menu checkmarks on the right.
    let (pad_left, pad_right) = if has_indicator && indicator_leading {
        (recipe.item_inset_pad_left_px, recipe.item_pad_x_px)
    } else if has_indicator {
        (
            if inset {
                recipe.item_inset_pad_left_px
            } else {
                recipe.item_pad_x_px
            },
            recipe.item_indicator_pad_right_px,
        )
    } else if inset {
        (recipe.item_inset_pad_left_px, recipe.item_pad_x_px)
    } else {
        (recipe.item_pad_x_px, recipe.item_pad_x_px)
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
            if indicator_leading {
                row_bounds.x + item_indicator_left_px + size / 2.0
            } else {
                row_bounds.x + row_bounds.width - recipe.item_indicator_right_px - size / 2.0
            },
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
