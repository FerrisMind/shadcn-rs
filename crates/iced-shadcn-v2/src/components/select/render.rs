//! Custom trigger widget and design-system dropdown for [`super::Select`].
//!
//! The closed trigger paints `.cn-select-trigger` visuals and a lucide-style
//! chevron. The dropdown paints `.cn-select-content` (popover surface, ring,
//! and shadow) with checkable `.cn-select-item` rows, labels, and separators.
//! This is the same design-system popup the web component shows, not the stock
//! OS menu used by [`crate::NativeSelect`].

use iced_core::keyboard;
use iced_core::text::paragraph;
use iced_core::text::{self as core_text, Renderer as _, Text};

use shadcn_common::{
    Direction, FontWeight, NavAction, NavKey, Orientation, SELECT_CONTENT_MAX_HEIGHT_PX,
    SELECT_SIDE_OFFSET_PX, resolve_nav_action,
};

use crate::iced_compat::advanced::renderer::Renderer as _;
use crate::iced_compat::advanced::widget::{Tree, tree};
use crate::iced_compat::advanced::{Clipboard, Shell, Widget, layout, overlay, renderer};
use crate::iced_compat::widget::canvas;
use crate::iced_compat::widget::graphics::geometry::Renderer as _;
use crate::iced_compat::widget::scrollable::Scrollable;
use crate::iced_compat::{
    Background, Border, Color, Element, Event, Font, Length, Pixels, Point, Rectangle, Renderer,
    Shadow, Size, Theme as IcedTheme, Vector, alignment, mouse, touch, window,
};

use super::style::{
    self, SelectContentStyle, SelectStatus, SelectTriggerStyle, pack_icon_size, pack_text_size,
};
use super::types::{Row, SelectRadius, SelectSelection, SelectSize, SelectType};
use crate::fonts::iced_font;
use crate::recipes::iced_font_weight;
use crate::theme::Theme;

type ParagraphOf = <Renderer as core_text::Renderer>::Paragraph;

/// Line box reserved for trigger glyphs.
pub(super) fn line_height_px(text_size: f32) -> f32 {
    text_size + 6.0
}

/// Internal widget produced by the [`super::Select`] builder.
pub(super) struct SelectWidget<'a, T, Message>
where
    T: Clone + PartialEq,
{
    pub(super) theme: &'a Theme,
    pub(super) rows: Vec<Row<T>>,
    pub(super) selection: SelectSelection<T>,
    pub(super) select_type: SelectType,
    pub(super) placeholder: Option<String>,
    pub(super) size: SelectSize,
    pub(super) radius: Option<SelectRadius>,
    pub(super) width: Length,
    pub(super) text_size: Option<f32>,
    pub(super) disabled: bool,
    pub(super) invalid: bool,
    pub(super) deselectable: bool,
    pub(super) on_select: Option<Box<dyn Fn(T) -> Message + 'a>>,
    pub(super) on_selection_change: Option<Box<dyn Fn(SelectSelection<T>) -> Message + 'a>>,
    pub(super) on_open: Option<Message>,
    pub(super) on_close: Option<Message>,
    pub(super) style_override:
        Option<Box<dyn Fn(SelectTriggerStyle, SelectStatus) -> SelectTriggerStyle + 'a>>,
    pub(super) last_status: Option<SelectStatus>,
}

/// Widget-tree state of the trigger and its dropdown.
struct State {
    is_open: bool,
    hovered_row: Option<usize>,
    menu_tree: Tree,
    rows: Vec<paragraph::Plain<ParagraphOf>>,
    placeholder: paragraph::Plain<ParagraphOf>,
}

impl State {
    fn new() -> Self {
        Self {
            is_open: false,
            hovered_row: None,
            menu_tree: Tree::empty(),
            rows: Vec::new(),
            placeholder: paragraph::Plain::default(),
        }
    }
}

impl<T, Message> SelectWidget<'_, T, Message>
where
    T: Clone + PartialEq,
{
    fn is_interactive(&self) -> bool {
        !self.disabled && (self.on_select.is_some() || self.on_selection_change.is_some())
    }

    fn resolved_text_size(&self) -> f32 {
        self.text_size
            .unwrap_or_else(|| pack_text_size(self.theme, self.size))
    }

    fn selected_label(&self) -> Option<&str> {
        match &self.selection {
            SelectSelection::Single(Some(value)) => self.rows.iter().find_map(|row| match row {
                Row::Option {
                    value: option,
                    label,
                    ..
                } if option == value => Some(label.as_str()),
                _ => None,
            }),
            SelectSelection::Multiple(values) if values.len() == 1 => {
                let value = &values[0];
                self.rows.iter().find_map(|row| match row {
                    Row::Option {
                        value: option,
                        label,
                        ..
                    } if option == value => Some(label.as_str()),
                    _ => None,
                })
            }
            _ => None,
        }
    }

    fn multiple_count_label(&self) -> Option<String> {
        match &self.selection {
            SelectSelection::Multiple(values) if values.len() > 1 => {
                Some(format!("{} selected", values.len()))
            }
            _ => None,
        }
    }

    fn status(&self, state: &State, is_hovered: bool) -> SelectStatus {
        if self.disabled {
            SelectStatus::Disabled
        } else if state.is_open {
            SelectStatus::Opened
        } else if is_hovered {
            SelectStatus::Hovered
        } else {
            SelectStatus::Active
        }
    }

    fn resolve_trigger_style(&self, status: SelectStatus) -> SelectTriggerStyle {
        let mut resolved = style::resolve_trigger_style(
            self.theme,
            self.size,
            self.radius,
            self.invalid,
            self.disabled,
            status,
        );

        if let Some(override_fn) = self.style_override.as_ref() {
            resolved = override_fn(resolved, status);
        }

        resolved
    }
}

impl<'a, T, Message> Widget<Message, IcedTheme, Renderer> for SelectWidget<'a, T, Message>
where
    T: Clone + PartialEq + 'a,
    Message: Clone + 'a,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::new())
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: Length::Shrink,
        }
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let state = tree.state.downcast_mut::<State>();
        let recipe = style::recipe(self.theme);
        let text_size = self.resolved_text_size();
        let line_height = core_text::LineHeight::Absolute(Pixels(line_height_px(text_size)));
        let font = iced_font(self.theme.font_pack().sans);

        state.rows.resize_with(self.rows.len(), Default::default);

        let template = Text {
            content: "",
            bounds: Size::new(f32::INFINITY, line_height_px(text_size)),
            size: Pixels(text_size),
            line_height,
            font,
            align_x: core_text::Alignment::Default,
            align_y: alignment::Vertical::Center,
            shaping: core_text::Shaping::default(),
            wrapping: core_text::Wrapping::default(),
        };

        for (row, paragraph) in self.rows.iter().zip(state.rows.iter_mut()) {
            if let Some(label) = row.label() {
                let _ = paragraph.update(Text {
                    content: label,
                    ..template
                });
            }
        }

        if let Some(placeholder) = &self.placeholder {
            let _ = state.placeholder.update(Text {
                content: placeholder,
                ..template
            });
        }

        let labels_width = self
            .rows
            .iter()
            .zip(state.rows.iter())
            .fold(0.0_f32, |width, (row, paragraph)| {
                if row.label().is_some() {
                    width.max(paragraph.min_width())
                } else {
                    width
                }
            })
            .max(
                self.placeholder
                    .as_ref()
                    .map(|_| state.placeholder.min_width())
                    .unwrap_or(0.0),
            );

        let icon_size = pack_icon_size(self.theme, self.size);
        let height = self.size.control_height(self.theme);
        let intrinsic = Size::new(
            labels_width
                + recipe.trigger_pad_left_px
                + recipe.trigger_pad_right_px
                + recipe.trigger_gap_px
                + icon_size,
            height,
        );
        let size = limits
            .width(self.width)
            .height(Length::Fixed(height))
            .resolve(self.width, Length::Fixed(height), intrinsic);

        layout::Node::new(size)
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

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if state.is_open {
                    state.is_open = false;

                    if let Some(on_close) = &self.on_close {
                        shell.publish(on_close.clone());
                    }

                    shell.capture_event();
                } else if cursor.is_over(layout.bounds()) && self.is_interactive() {
                    state.is_open = true;
                    state.hovered_row = self
                        .rows
                        .iter()
                        .enumerate()
                        .find_map(|(index, row)| match row {
                            Row::Option {
                                value,
                                disabled: false,
                                ..
                            } if self.selection.is_selected(value) => Some(index),
                            _ => None,
                        })
                        .or_else(|| self.rows.iter().position(Row::is_selectable));

                    if let Some(on_open) = &self.on_open {
                        shell.publish(on_open.clone());
                    }

                    shell.capture_event();
                }
            }
            Event::Keyboard(keyboard::Event::KeyPressed {
                key: keyboard::Key::Named(keyboard::key::Named::Escape),
                ..
            }) if state.is_open => {
                state.is_open = false;

                if let Some(on_close) = &self.on_close {
                    shell.publish(on_close.clone());
                }

                shell.capture_event();
                shell.request_redraw();
            }
            _ => {}
        }

        let status = self.status(state, cursor.is_over(layout.bounds()));

        if let Event::Window(window::Event::RedrawRequested(_now)) = event {
            self.last_status = Some(status);
        } else if self
            .last_status
            .is_some_and(|last_status| last_status != status)
        {
            shell.request_redraw();
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
        if cursor.is_over(layout.bounds()) && self.is_interactive() {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::default()
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        _theme: &IcedTheme,
        _style: &renderer::Style,
        layout: layout::Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();

        if !bounds.intersects(viewport) {
            return;
        }

        let state = tree.state.downcast_ref::<State>();
        let status = self
            .last_status
            .unwrap_or_else(|| self.status(state, cursor.is_over(bounds)));
        let resolved = self.resolve_trigger_style(status);
        let recipe = style::recipe(self.theme);
        let text_size = self.resolved_text_size();

        if resolved.underline_only {
            renderer.fill_quad(
                renderer::Quad {
                    bounds,
                    border: Border {
                        radius: resolved.radius.into(),
                        ..Border::default()
                    },
                    ..renderer::Quad::default()
                },
                Background::Color(resolved.background),
            );
            renderer.fill_quad(
                renderer::Quad {
                    bounds: Rectangle {
                        x: bounds.x,
                        y: bounds.y + bounds.height - resolved.border_width,
                        width: bounds.width,
                        height: resolved.border_width,
                    },
                    ..renderer::Quad::default()
                },
                Background::Color(resolved.border_color),
            );
        } else {
            renderer.fill_quad(
                renderer::Quad {
                    bounds,
                    border: Border {
                        color: resolved.border_color,
                        width: resolved.border_width,
                        radius: resolved.radius.into(),
                    },
                    ..renderer::Quad::default()
                },
                Background::Color(resolved.background),
            );
        }

        let count_label = self.multiple_count_label();
        let selected = self.selected_label();
        let content = selected
            .map(str::to_owned)
            .or(count_label)
            .or_else(|| self.placeholder.clone());

        let icon_size = pack_icon_size(self.theme, self.size);
        let value_max_width = (bounds.width
            - recipe.trigger_pad_left_px
            - recipe.trigger_pad_right_px
            - recipe.trigger_gap_px
            - icon_size)
            .max(0.0);

        if let Some(content) = content {
            renderer.fill_text(
                Text {
                    content,
                    bounds: Size::new(value_max_width, line_height_px(text_size)),
                    size: Pixels(text_size),
                    line_height: core_text::LineHeight::Absolute(Pixels(line_height_px(text_size))),
                    font: iced_font(self.theme.font_pack().sans),
                    align_x: core_text::Alignment::Default,
                    align_y: alignment::Vertical::Center,
                    shaping: core_text::Shaping::default(),
                    wrapping: core_text::Wrapping::None,
                },
                Point::new(bounds.x + recipe.trigger_pad_left_px, bounds.center_y()),
                if selected.is_some() || self.selection.len() > 1 {
                    resolved.text_color
                } else {
                    resolved.placeholder_color
                },
                *viewport,
            );
        }

        let icon_center = Point::new(
            bounds.x + bounds.width - recipe.trigger_pad_right_px - icon_size / 2.0,
            bounds.center_y(),
        );

        draw_chevron(renderer, icon_center, icon_size, resolved.icon_color);
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

        if !state.is_open || !self.is_interactive() {
            return None;
        }

        let bounds = layout.bounds();
        let content_style = style::resolve_content_style(self.theme);
        let recipe = style::recipe(self.theme);
        let text_size = self.resolved_text_size();
        let font = iced_font(self.theme.font_pack().sans);
        let mut label_font = font;
        label_font.weight = iced_font_weight(FontWeight::Normal);

        let on_select = self.on_select.as_deref();
        let on_selection_change = self.on_selection_change.as_deref();
        let selection = self.selection.clone();
        let select_type = self.select_type;
        let deselectable = self.deselectable;
        let close_on_pick = !select_type.is_multiple();

        let State {
            is_open,
            hovered_row,
            menu_tree,
            ..
        } = state;

        let list = List {
            rows: &self.rows,
            selection,
            select_type,
            deselectable,
            hovered_row,
            on_select,
            on_selection_change,
            on_close_menu: Box::new(move || {
                if close_on_pick {
                    *is_open = false;
                }
            }),
            recipe,
            content_style,
            text_size,
            font,
            label_font,
        };

        Some(
            MenuOverlay::new(
                layout.position() + translation,
                *viewport,
                menu_tree,
                list,
                bounds.width.max(recipe.content_min_width_px),
                bounds.height,
                content_style,
            )
            .element(),
        )
    }
}

/// Paints the lucide-style `chevron-down` glyph of `.cn-select-trigger-icon`.
fn draw_chevron(renderer: &mut Renderer, center: Point, size: f32, color: Color) {
    if size <= 0.0 {
        return;
    }

    let reach = size * 0.25;
    let arm = size * 0.125;
    let stroke_width = (size * 0.10).clamp(1.0, 1.75);

    let mut frame = canvas::Frame::new(renderer, Size::new(size, size));
    frame.translate(Vector::new(size / 2.0, size / 2.0));
    frame.stroke(
        &canvas::Path::new(|builder| {
            builder.move_to(Point::new(-reach, -arm));
            builder.line_to(Point::new(0.0, arm));
            builder.line_to(Point::new(reach, -arm));
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

/// Paints the lucide-style `check` glyph of `.cn-select-item-indicator`.
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
        |renderer| {
            renderer.draw_geometry(geometry);
        },
    );
}

/// Dropdown overlay: scrollable list on the `.cn-select-content` surface.
struct MenuOverlay<'a, Message> {
    position: Point,
    viewport: Rectangle,
    tree: &'a mut Tree,
    list: Scrollable<'a, Message, IcedTheme, Renderer>,
    width: f32,
    target_height: f32,
    style: SelectContentStyle,
}

impl<'a, Message> MenuOverlay<'a, Message>
where
    Message: 'a,
{
    fn new<T>(
        position: Point,
        viewport: Rectangle,
        tree: &'a mut Tree,
        list: List<'a, T, Message>,
        width: f32,
        target_height: f32,
        style: SelectContentStyle,
    ) -> Self
    where
        T: Clone + PartialEq + 'a,
        Message: Clone + 'a,
    {
        let list = Scrollable::new(list);
        tree.diff(&list as &dyn Widget<_, _, _>);

        Self {
            position,
            viewport,
            tree,
            list,
            width,
            target_height,
            style,
        }
    }

    fn element(self) -> overlay::Element<'a, Message, IcedTheme, Renderer> {
        overlay::Element::new(Box::new(self))
    }
}

impl<Message> overlay::Overlay<Message, IcedTheme, Renderer> for MenuOverlay<'_, Message> {
    fn layout(&mut self, renderer: &Renderer, bounds: Size) -> layout::Node {
        let space_below =
            bounds.height - (self.position.y + self.target_height + SELECT_SIDE_OFFSET_PX);
        let space_above = self.position.y - SELECT_SIDE_OFFSET_PX;
        let open_below = space_below >= space_above;
        let space = if open_below { space_below } else { space_above };

        let limits = layout::Limits::new(
            Size::ZERO,
            Size::new(
                (bounds.width - self.position.x).max(self.width),
                space.min(SELECT_CONTENT_MAX_HEIGHT_PX),
            ),
        )
        .width(self.width);

        let node = self.list.layout(self.tree, renderer, &limits);
        let size = node.size();

        node.move_to(if open_below {
            self.position + Vector::new(0.0, self.target_height + SELECT_SIDE_OFFSET_PX)
        } else {
            self.position - Vector::new(0.0, size.height + SELECT_SIDE_OFFSET_PX)
        })
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
        let bounds = layout.bounds();
        self.list.update(
            self.tree, event, layout, cursor, renderer, clipboard, shell, &bounds,
        );
    }

    fn mouse_interaction(
        &self,
        layout: layout::Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.list
            .mouse_interaction(self.tree, layout, cursor, &self.viewport, renderer)
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        theme: &IcedTheme,
        defaults: &renderer::Style,
        layout: layout::Layout<'_>,
        cursor: mouse::Cursor,
    ) {
        let bounds = layout.bounds();

        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border: Border {
                    color: self.style.border_color,
                    width: self.style.border_width,
                    radius: self.style.radius.into(),
                },
                shadow: self.style.shadow,
                ..renderer::Quad::default()
            },
            Background::Color(self.style.background),
        );

        self.list.draw(
            self.tree, renderer, theme, defaults, layout, cursor, &bounds,
        );
    }
}

/// Inner list widget hosted by the dropdown scrollable.
struct List<'a, T, Message>
where
    T: Clone + PartialEq,
{
    rows: &'a [Row<T>],
    selection: SelectSelection<T>,
    select_type: SelectType,
    deselectable: bool,
    hovered_row: &'a mut Option<usize>,
    on_select: Option<&'a dyn Fn(T) -> Message>,
    on_selection_change: Option<&'a dyn Fn(SelectSelection<T>) -> Message>,
    on_close_menu: Box<dyn FnMut() + 'a>,
    recipe: shadcn_common::SelectRecipe,
    content_style: SelectContentStyle,
    text_size: f32,
    font: Font,
    label_font: Font,
}

impl<T, Message> List<'_, T, Message>
where
    T: Clone + PartialEq,
    Message: Clone,
{
    fn row_height(&self, index: usize) -> f32 {
        let recipe = self.recipe;

        match self.rows.get(index) {
            Some(Row::Separator) => recipe.separator_margin_y_px * 2.0 + 1.0,
            Some(Row::Label { .. }) => {
                recipe.label_typography.line_height_px + recipe.label_pad_y_px * 2.0
            }
            _ => {
                recipe
                    .item_typography
                    .line_height_px
                    .max(self.text_size + 6.0)
                    + recipe.item_pad_y_px * 2.0
            }
        }
    }

    fn total_height(&self) -> f32 {
        let pad = self.recipe.content_pad_px * 2.0;
        pad + (0..self.rows.len())
            .map(|index| self.row_height(index))
            .sum::<f32>()
    }

    fn row_at(&self, position: Point) -> Option<usize> {
        let mut y = self.recipe.content_pad_px;

        if position.y < y {
            return None;
        }

        for index in 0..self.rows.len() {
            let height = self.row_height(index);
            if position.y < y + height {
                return Some(index);
            }
            y += height;
        }

        None
    }

    fn select_hovered(&mut self, shell: &mut Shell<'_, Message>) {
        if let Some(index) = *self.hovered_row
            && let Some(Row::Option {
                value,
                disabled: false,
                ..
            }) = self.rows.get(index)
        {
            let next = self
                .selection
                .clone()
                .toggled(self.select_type, value, self.deselectable);

            if let Some(on_select) = self.on_select {
                shell.publish(on_select(value.clone()));
            }

            if let Some(on_selection_change) = self.on_selection_change {
                shell.publish(on_selection_change(next.clone()));
            }

            self.selection = next;
            (self.on_close_menu)();
            shell.capture_event();
            shell.request_redraw();
        }
    }

    fn move_hover(&mut self, direction: isize, shell: &mut Shell<'_, Message>) {
        if let Some(index) =
            shadcn_common::step_index(self.rows, *self.hovered_row, direction, false, |row| {
                row.is_selectable()
            })
            .filter(|&index| *self.hovered_row != Some(index))
        {
            *self.hovered_row = Some(index);
            shell.request_redraw();
        }
    }

    fn move_hover_to_edge(&mut self, first: bool, shell: &mut Shell<'_, Message>) {
        let index = if first {
            shadcn_common::first_enabled_index(self.rows, |row| row.is_selectable())
        } else {
            shadcn_common::last_enabled_index(self.rows, |row| row.is_selectable())
        };

        if *self.hovered_row != index {
            *self.hovered_row = index;
            shell.request_redraw();
        }
    }
}

impl<'a, T, Message> Widget<Message, IcedTheme, Renderer> for List<'a, T, Message>
where
    T: Clone + PartialEq + 'a,
    Message: Clone + 'a,
{
    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Fill,
            height: Length::Shrink,
        }
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let intrinsic = Size::new(0.0, self.total_height());
        layout::Node::new(limits.resolve(Length::Fill, Length::Shrink, intrinsic))
    }

    fn update(
        &mut self,
        _tree: &mut Tree,
        event: &Event,
        layout: layout::Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                if cursor.is_over(layout.bounds()) {
                    self.select_hovered(shell);
                }
            }
            Event::Mouse(mouse::Event::CursorMoved { .. }) => {
                if let Some(position) = cursor.position_in(layout.bounds()) {
                    let hovered = self
                        .row_at(position)
                        .filter(|&index| self.rows[index].is_selectable());

                    if *self.hovered_row != hovered {
                        *self.hovered_row = hovered;
                        shell.request_redraw();
                    }
                }
            }
            Event::Touch(touch::Event::FingerPressed { .. }) => {
                if let Some(position) = cursor.position_in(layout.bounds()) {
                    *self.hovered_row = self
                        .row_at(position)
                        .filter(|&index| self.rows[index].is_selectable());
                    self.select_hovered(shell);
                }
            }
            Event::Keyboard(keyboard::Event::KeyPressed { key, .. }) => {
                if let Some(action) = nav_action(key) {
                    match action {
                        NavAction::Next => self.move_hover(1, shell),
                        NavAction::Previous => self.move_hover(-1, shell),
                        NavAction::First => self.move_hover_to_edge(true, shell),
                        NavAction::Last => self.move_hover_to_edge(false, shell),
                        NavAction::Activate => self.select_hovered(shell),
                        _ => {}
                    }
                    shell.capture_event();
                }
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
        let selectable_under_cursor = cursor
            .position_in(layout.bounds())
            .and_then(|position| self.row_at(position))
            .is_some_and(|index| self.rows[index].is_selectable());

        if selectable_under_cursor {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::default()
        }
    }

    fn draw(
        &self,
        _tree: &Tree,
        renderer: &mut Renderer,
        _theme: &IcedTheme,
        _style: &renderer::Style,
        layout: layout::Layout<'_>,
        _cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        let recipe = self.recipe;
        let mut y = bounds.y + recipe.content_pad_px;

        for (index, row) in self.rows.iter().enumerate() {
            let height = self.row_height(index);
            let row_bounds = Rectangle {
                x: bounds.x + recipe.content_pad_px,
                y,
                width: (bounds.width - recipe.content_pad_px * 2.0).max(0.0),
                height,
            };

            match row {
                Row::Separator => {
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
                        Background::Color(self.content_style.separator_color),
                    );
                }
                Row::Label { text } => {
                    renderer.fill_text(
                        Text {
                            content: text.clone(),
                            bounds: Size::new(
                                (row_bounds.width - recipe.label_pad_x_px * 2.0).max(0.0),
                                recipe.label_typography.line_height_px,
                            ),
                            size: Pixels(recipe.label_typography.size_px),
                            line_height: core_text::LineHeight::Absolute(Pixels(
                                recipe.label_typography.line_height_px,
                            )),
                            font: self.label_font,
                            align_x: core_text::Alignment::Default,
                            align_y: alignment::Vertical::Center,
                            shaping: core_text::Shaping::default(),
                            wrapping: core_text::Wrapping::None,
                        },
                        Point::new(row_bounds.x + recipe.label_pad_x_px, row_bounds.center_y()),
                        self.content_style.muted_color,
                        *viewport,
                    );
                }
                Row::Option {
                    value,
                    label,
                    disabled,
                } => {
                    let selected = self.selection.is_selected(value);
                    let highlighted = *self.hovered_row == Some(index);
                    let mut text_color = if highlighted {
                        self.content_style.item_highlight_text
                    } else {
                        self.content_style.text_color
                    };
                    let mut indicator_color = if highlighted {
                        self.content_style.item_highlight_text
                    } else {
                        self.content_style.item_indicator_color
                    };

                    if *disabled {
                        text_color =
                            text_color.scale_alpha(self.content_style.item_disabled_opacity);
                        indicator_color =
                            indicator_color.scale_alpha(self.content_style.item_disabled_opacity);
                    }

                    if highlighted && !*disabled {
                        renderer.fill_quad(
                            renderer::Quad {
                                bounds: row_bounds,
                                border: Border {
                                    radius: self.content_style.item_radius.into(),
                                    ..Border::default()
                                },
                                shadow: Shadow::default(),
                                ..renderer::Quad::default()
                            },
                            Background::Color(self.content_style.item_highlight_background),
                        );
                    }

                    let mut item_font = self.font;
                    item_font.weight = iced_font_weight(recipe.item_typography.weight);

                    renderer.fill_text(
                        Text {
                            content: label.clone(),
                            bounds: Size::new(
                                (row_bounds.width
                                    - recipe.item_pad_left_px
                                    - recipe.item_pad_right_px)
                                    .max(0.0),
                                recipe.item_typography.line_height_px,
                            ),
                            size: Pixels(recipe.item_typography.size_px),
                            line_height: core_text::LineHeight::Absolute(Pixels(
                                recipe.item_typography.line_height_px,
                            )),
                            font: item_font,
                            align_x: core_text::Alignment::Default,
                            align_y: alignment::Vertical::Center,
                            shaping: core_text::Shaping::default(),
                            wrapping: core_text::Wrapping::None,
                        },
                        Point::new(
                            row_bounds.x + recipe.item_pad_left_px,
                            row_bounds.center_y(),
                        ),
                        text_color,
                        *viewport,
                    );

                    if selected {
                        let indicator_center = Point::new(
                            row_bounds.x + row_bounds.width
                                - recipe.item_indicator_right_px
                                - recipe.item_indicator_size_px / 2.0,
                            row_bounds.center_y(),
                        );
                        draw_check(
                            renderer,
                            indicator_center,
                            recipe.item_indicator_size_px,
                            indicator_color,
                        );
                    }
                }
            }

            y += height;
        }
    }
}

impl<'a, T, Message> From<List<'a, T, Message>> for Element<'a, Message>
where
    T: Clone + PartialEq + 'a,
    Message: Clone + 'a,
{
    fn from(list: List<'a, T, Message>) -> Self {
        Element::new(list)
    }
}

fn nav_action(key: &keyboard::Key) -> Option<NavAction> {
    let nav_key = match key {
        keyboard::Key::Named(keyboard::key::Named::ArrowDown) => NavKey::ArrowDown,
        keyboard::Key::Named(keyboard::key::Named::ArrowUp) => NavKey::ArrowUp,
        keyboard::Key::Named(keyboard::key::Named::Home) => NavKey::Home,
        keyboard::Key::Named(keyboard::key::Named::End) => NavKey::End,
        keyboard::Key::Named(keyboard::key::Named::Enter) => NavKey::Enter,
        keyboard::Key::Named(keyboard::key::Named::Space) => NavKey::Space,
        _ => return None,
    };

    resolve_nav_action(nav_key, Orientation::Vertical, Direction::Ltr)
}
