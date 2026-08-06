//! Custom field widget and dropdown overlay for [`super::NativeSelect`].
//!
//! The closed field paints `.cn-native-select` visuals and a lucide-style
//! chevron. The dropdown deliberately does NOT use shadcn tokens: like the
//! web component — whose popup is OS-rendered — it reuses the stock
//! [`iced_widget::overlay::menu`] styling of the runtime `iced::Theme`
//! (the same look as `pick_list`), extended structurally with `<optgroup>`
//! headings and disabled options that hover and keyboard navigation skip.

use iced_core::keyboard;
use iced_core::text::paragraph;
use iced_core::text::{self as core_text, Renderer as _, Text};

use shadcn_common::{
    AccentColor, Direction, FontWeight, NATIVE_SELECT_MENU_GROUP_INDENT_PX,
    NATIVE_SELECT_MENU_ITEM_PAD_X_PX, NATIVE_SELECT_MENU_ITEM_PAD_Y_PX,
    NATIVE_SELECT_MENU_MAX_HEIGHT_PX, NavAction, NavKey, Orientation, resolve_nav_action,
};

use crate::iced_compat::advanced::renderer::Renderer as _;
use crate::iced_compat::advanced::widget::{Tree, tree};
use crate::iced_compat::advanced::{Clipboard, Shell, Widget, layout, overlay, renderer};
use crate::iced_compat::widget::canvas;
use crate::iced_compat::widget::graphics::geometry::Renderer as _;
use crate::iced_compat::widget::overlay::menu;
use crate::iced_compat::widget::scrollable::Scrollable;
use crate::iced_compat::{
    Background, Border, Color, Element, Event, Font, Length, Pixels, Point, Rectangle, Renderer,
    Size, Theme as IcedTheme, Vector, alignment, border, mouse, touch, window,
};

use super::style::{self, NativeSelectStatus, NativeSelectStyle, pack_icon_size, pack_text_size};
use super::types::{NativeSelectRadius, NativeSelectSize, Row};
use crate::fonts::iced_font;
use crate::recipes::iced_font_weight;
use crate::theme::Theme;

/// Text alpha of a disabled dropdown option, matching how OS menus gray
/// disabled items out.
const MENU_DISABLED_TEXT_ALPHA: f32 = 0.5;

type ParagraphOf = <Renderer as core_text::Renderer>::Paragraph;

/// Line box reserved for the value glyphs; an absolute line height keeps the
/// control height exact regardless of the font's own metrics (same trick as
/// the input).
pub(super) fn line_height_px(text_size: f32) -> f32 {
    text_size + 6.0
}

/// Height of one dropdown row.
fn row_height_px(text_size: f32) -> f32 {
    line_height_px(text_size) + NATIVE_SELECT_MENU_ITEM_PAD_Y_PX * 2.0
}

/// Internal widget produced by the [`super::NativeSelect`] builder.
pub(super) struct NativeSelectWidget<'a, T, Message>
where
    T: Clone + PartialEq,
{
    pub(super) theme: &'a Theme,
    pub(super) rows: Vec<Row<T>>,
    pub(super) selected: Option<T>,
    pub(super) placeholder: Option<String>,
    pub(super) size: NativeSelectSize,
    pub(super) radius: Option<NativeSelectRadius>,
    pub(super) color: Option<AccentColor>,
    pub(super) width: Length,
    pub(super) text_size: Option<f32>,
    pub(super) disabled: bool,
    pub(super) invalid: bool,
    pub(super) on_select: Option<Box<dyn Fn(T) -> Message + 'a>>,
    pub(super) on_open: Option<Message>,
    pub(super) on_close: Option<Message>,
    pub(super) style_override:
        Option<Box<dyn Fn(NativeSelectStyle, NativeSelectStatus) -> NativeSelectStyle + 'a>>,
    pub(super) last_status: Option<NativeSelectStatus>,
}

/// Widget-tree state of the field and its dropdown.
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

impl<T, Message> NativeSelectWidget<'_, T, Message>
where
    T: Clone + PartialEq,
{
    fn is_interactive(&self) -> bool {
        !self.disabled && self.on_select.is_some()
    }

    fn resolved_text_size(&self) -> f32 {
        self.text_size
            .unwrap_or_else(|| pack_text_size(self.theme, self.size))
    }

    /// Index of the row holding the currently selected value.
    fn selected_index(&self) -> Option<usize> {
        let selected = self.selected.as_ref()?;

        self.rows
            .iter()
            .position(|row| matches!(row, Row::Option { value, .. } if value == selected))
    }

    /// Label shown in the closed field.
    fn selected_label(&self) -> Option<&str> {
        self.selected_index().map(|index| self.rows[index].label())
    }

    fn status(&self, state: &State, is_hovered: bool) -> NativeSelectStatus {
        if self.disabled {
            NativeSelectStatus::Disabled
        } else if state.is_open {
            NativeSelectStatus::Opened
        } else if is_hovered {
            NativeSelectStatus::Hovered
        } else {
            NativeSelectStatus::Active
        }
    }

    fn resolve_style(&self, status: NativeSelectStatus) -> NativeSelectStyle {
        let mut resolved = style::resolve_field_style(
            self.theme,
            self.size,
            self.radius,
            self.color,
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

impl<'a, T, Message> Widget<Message, IcedTheme, Renderer> for NativeSelectWidget<'a, T, Message>
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
            let _ = paragraph.update(Text {
                content: row.label(),
                ..template
            });
        }

        if let Some(placeholder) = &self.placeholder {
            let _ = state.placeholder.update(Text {
                content: placeholder,
                ..template
            });
        }

        // Like the native `<select>`, the intrinsic width fits the widest
        // option (indent included) so the field never resizes on selection.
        let labels_width = self
            .rows
            .iter()
            .zip(state.rows.iter())
            .fold(0.0_f32, |width, (row, paragraph)| {
                let indent = if row.is_indented() {
                    NATIVE_SELECT_MENU_GROUP_INDENT_PX
                } else {
                    0.0
                };

                width.max(paragraph.min_width() + indent)
            })
            .max(
                self.placeholder
                    .as_ref()
                    .map(|_| state.placeholder.min_width())
                    .unwrap_or(0.0),
            );

        let height = self.size.control_height(self.theme);
        let intrinsic = Size::new(
            labels_width + recipe.pad_left_px + recipe.pad_right_px,
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
                    // The overlay did not capture the press, so it landed
                    // outside the dropdown (or on an inert row area).
                    state.is_open = false;

                    if let Some(on_close) = &self.on_close {
                        shell.publish(on_close.clone());
                    }

                    shell.capture_event();
                } else if cursor.is_over(layout.bounds()) && self.is_interactive() {
                    state.is_open = true;
                    state.hovered_row = self
                        .selected_index()
                        .filter(|&index| self.rows[index].is_selectable())
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
        let resolved = self.resolve_style(status);
        let recipe = style::recipe(self.theme);
        let text_size = self.resolved_text_size();

        if resolved.underline_only {
            // Sera's `border-b-input`: background box plus a bottom hairline.
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

        let label = self.selected_label();

        if let Some(content) = label.or(self.placeholder.as_deref()) {
            renderer.fill_text(
                Text {
                    content: content.to_owned(),
                    bounds: Size::new(
                        (bounds.width - recipe.pad_left_px - recipe.pad_right_px).max(0.0),
                        line_height_px(text_size),
                    ),
                    size: Pixels(text_size),
                    line_height: core_text::LineHeight::Absolute(Pixels(line_height_px(text_size))),
                    font: iced_font(self.theme.font_pack().sans),
                    align_x: core_text::Alignment::Default,
                    align_y: alignment::Vertical::Center,
                    shaping: core_text::Shaping::default(),
                    wrapping: core_text::Wrapping::None,
                },
                Point::new(bounds.x + recipe.pad_left_px, bounds.center_y()),
                if label.is_some() {
                    resolved.text_color
                } else {
                    resolved.placeholder_color
                },
                *viewport,
            );
        }

        let icon_size = pack_icon_size(self.theme, self.size);
        let icon_center = Point::new(
            bounds.x + bounds.width - recipe.icon_right_px - icon_size / 2.0,
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

        if !state.is_open {
            return None;
        }

        let on_select = self.on_select.as_deref()?;
        let bounds = layout.bounds();

        let State {
            is_open,
            hovered_row,
            menu_tree,
            ..
        } = state;

        let text_size = self
            .text_size
            .unwrap_or_else(|| pack_text_size(self.theme, self.size));
        let font = iced_font(self.theme.font_pack().sans);
        let mut group_font = font;
        group_font.weight = iced_font_weight(FontWeight::Semibold);

        let list = List {
            rows: &self.rows,
            hovered_row,
            on_selected: Box::new(move |value| {
                *is_open = false;

                on_select(value)
            }),
            text_size,
            font,
            group_font,
        };

        Some(
            MenuOverlay::new(
                layout.position() + translation,
                *viewport,
                menu_tree,
                list,
                bounds.width,
                bounds.height,
            )
            .element(),
        )
    }
}

/// Paints the lucide-style `chevron-down` glyph of `.cn-native-select-icon`.
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

/// Dropdown overlay: a scrollable list of rows on the stock iced menu
/// surface.
struct MenuOverlay<'a, Message> {
    position: Point,
    viewport: Rectangle,
    tree: &'a mut Tree,
    list: Scrollable<'a, Message, IcedTheme, Renderer>,
    width: f32,
    target_height: f32,
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
    ) -> Self
    where
        T: Clone + PartialEq + 'a,
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
        }
    }

    fn element(self) -> overlay::Element<'a, Message, IcedTheme, Renderer> {
        overlay::Element::new(Box::new(self))
    }
}

impl<Message> overlay::Overlay<Message, IcedTheme, Renderer> for MenuOverlay<'_, Message> {
    fn layout(&mut self, renderer: &Renderer, bounds: Size) -> layout::Node {
        let space_below = bounds.height - (self.position.y + self.target_height);
        let space_above = self.position.y;
        let space = if space_below > space_above {
            space_below
        } else {
            space_above
        };

        let limits = layout::Limits::new(
            Size::ZERO,
            Size::new(
                bounds.width - self.position.x,
                space.min(NATIVE_SELECT_MENU_MAX_HEIGHT_PX),
            ),
        )
        .width(self.width);

        let node = self.list.layout(self.tree, renderer, &limits);
        let size = node.size();

        node.move_to(if space_below > space_above {
            self.position + Vector::new(0.0, self.target_height)
        } else {
            self.position - Vector::new(0.0, size.height)
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
        // Stock iced menu chrome — the "system" dropdown of this backend.
        let style = menu::default(theme);

        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border: style.border,
                shadow: style.shadow,
                ..renderer::Quad::default()
            },
            style.background,
        );

        self.list.draw(
            self.tree, renderer, theme, defaults, layout, cursor, &bounds,
        );
    }
}

/// Inner list widget hosted by the dropdown scrollable.
struct List<'a, T, Message> {
    rows: &'a [Row<T>],
    hovered_row: &'a mut Option<usize>,
    on_selected: Box<dyn FnMut(T) -> Message + 'a>,
    text_size: f32,
    font: Font,
    group_font: Font,
}

impl<T, Message> List<'_, T, Message>
where
    T: Clone + PartialEq,
{
    fn row_height(&self) -> f32 {
        row_height_px(self.text_size)
    }

    /// Row index under `position` (relative to the list bounds), selectable
    /// or not.
    fn row_at(&self, position: Point) -> Option<usize> {
        if position.y < 0.0 {
            return None;
        }

        let index = (position.y / self.row_height()) as usize;

        (index < self.rows.len()).then_some(index)
    }

    /// Publishes the hovered option and closes the dropdown.
    fn select_hovered(&mut self, shell: &mut Shell<'_, Message>) {
        if let Some(index) = *self.hovered_row
            && let Some(Row::Option {
                value,
                disabled: false,
                ..
            }) = self.rows.get(index)
        {
            shell.publish((self.on_selected)(value.clone()));
            shell.capture_event();
        }
    }

    /// Moves the keyboard hover to the next selectable row in `direction`.
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

impl<T, Message> Widget<Message, IcedTheme, Renderer> for List<'_, T, Message>
where
    T: Clone + PartialEq,
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
        let intrinsic = Size::new(0.0, self.row_height() * self.rows.len() as f32);

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
        theme: &IcedTheme,
        _style: &renderer::Style,
        layout: layout::Layout<'_>,
        _cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        let row_height = self.row_height();
        // Stock iced menu row palette — the "system" dropdown of this
        // backend; shadcn tokens intentionally stay off the popup.
        let style = menu::default(theme);
        let disabled_text = Color {
            a: style.text_color.a * MENU_DISABLED_TEXT_ALPHA,
            ..style.text_color
        };

        for (index, row) in self.rows.iter().enumerate() {
            let row_bounds = Rectangle {
                x: bounds.x,
                y: bounds.y + row_height * index as f32,
                width: bounds.width,
                height: row_height,
            };

            if !row_bounds.intersects(viewport) {
                continue;
            }

            let is_hovered = *self.hovered_row == Some(index);

            if is_hovered {
                renderer.fill_quad(
                    renderer::Quad {
                        bounds: Rectangle {
                            x: row_bounds.x + style.border.width,
                            width: row_bounds.width - style.border.width * 2.0,
                            ..row_bounds
                        },
                        border: border::rounded(style.border.radius),
                        ..renderer::Quad::default()
                    },
                    style.selected_background,
                );
            }

            let (font, color) = match row {
                Row::GroupLabel { .. } => (self.group_font, style.text_color),
                Row::Option { disabled: true, .. } => (self.font, disabled_text),
                Row::Option { .. } if is_hovered => (self.font, style.selected_text_color),
                Row::Option { .. } => (self.font, style.text_color),
            };

            let indent = if row.is_indented() {
                NATIVE_SELECT_MENU_GROUP_INDENT_PX
            } else {
                0.0
            };

            renderer.fill_text(
                Text {
                    content: row.label().to_owned(),
                    bounds: Size::new(f32::INFINITY, row_bounds.height),
                    size: Pixels(self.text_size),
                    line_height: core_text::LineHeight::Absolute(Pixels(line_height_px(
                        self.text_size,
                    ))),
                    font,
                    align_x: core_text::Alignment::Default,
                    align_y: alignment::Vertical::Center,
                    shaping: core_text::Shaping::default(),
                    wrapping: core_text::Wrapping::None,
                },
                Point::new(
                    row_bounds.x + NATIVE_SELECT_MENU_ITEM_PAD_X_PX + indent,
                    row_bounds.center_y(),
                ),
                color,
                *viewport,
            );
        }
    }
}

fn nav_action(key: &keyboard::Key) -> Option<NavAction> {
    let key = match key {
        keyboard::Key::Named(keyboard::key::Named::ArrowUp) => NavKey::ArrowUp,
        keyboard::Key::Named(keyboard::key::Named::ArrowDown) => NavKey::ArrowDown,
        keyboard::Key::Named(keyboard::key::Named::Home) => NavKey::Home,
        keyboard::Key::Named(keyboard::key::Named::End) => NavKey::End,
        keyboard::Key::Named(keyboard::key::Named::Enter) => NavKey::Enter,
        keyboard::Key::Named(keyboard::key::Named::Space) => NavKey::Space,
        _ => return None,
    };

    resolve_nav_action(key, Orientation::Vertical, Direction::Ltr)
}

impl<'a, T, Message> From<List<'a, T, Message>> for Element<'a, Message>
where
    T: Clone + PartialEq + 'a,
    Message: 'a,
{
    fn from(list: List<'a, T, Message>) -> Self {
        Element::new(list)
    }
}
