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
    Direction, FontWeight, NavAction, NavKey, Orientation, SELECT_SIDE_OFFSET_PX,
    resolve_nav_action,
};

use crate::iced_compat::advanced::renderer::Renderer as _;
use crate::iced_compat::advanced::widget::{Tree, tree};
use crate::iced_compat::advanced::{Clipboard, Shell, Widget, layout, overlay, renderer};
use crate::iced_compat::widget::canvas;
use crate::iced_compat::widget::graphics::geometry::Renderer as _;
use crate::iced_compat::{
    Background, Border, Color, Element, Event, Font, Length, Pixels, Point, Rectangle, Renderer,
    Shadow, Size, Theme as IcedTheme, Vector, alignment, mouse,
    time::{Duration, Instant},
    touch, window,
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
    pub(super) max_height: f32,
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

/// Direction of an active scroll-button hold (bits-ui auto-scroll).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ScrollHoldDir {
    Up,
    Down,
}

/// bits-ui select scroll-button default `delay` (`() => 50`).
const SCROLL_HOLD_INTERVAL: Duration = Duration::from_millis(50);

/// Height of `.cn-select-scroll-*-button` (`py-1` + icon).
fn scroll_button_height(recipe: shadcn_common::SelectRecipe) -> f32 {
    recipe.scroll_button_pad_y_px * 2.0 + recipe.icon_size_px
}

/// Typical option row height used as the bits-ui auto-scroll step.
fn typical_option_height(recipe: shadcn_common::SelectRecipe, text_size: f32) -> f32 {
    recipe.item_typography.line_height_px.max(text_size + 6.0) + recipe.item_pad_y_px * 2.0
}

fn row_height_px(
    recipe: shadcn_common::SelectRecipe,
    text_size: f32,
    row: &Row<impl Clone + PartialEq>,
) -> f32 {
    match row {
        Row::Separator => recipe.separator_margin_y_px * 2.0 + 1.0,
        Row::Label { .. } => recipe.label_typography.line_height_px + recipe.label_pad_y_px * 2.0,
        Row::Option { .. } => typical_option_height(recipe, text_size),
    }
}

fn content_total_height(
    rows: &[Row<impl Clone + PartialEq>],
    recipe: shadcn_common::SelectRecipe,
    text_size: f32,
) -> f32 {
    let pad = recipe.content_pad_px * 2.0;
    pad + rows
        .iter()
        .map(|row| row_height_px(recipe, text_size, row))
        .sum::<f32>()
}

/// Resolves scroll-button visibility + viewport height.
///
/// `can_down` is decided against the viewport **without** the down button.
/// Reserving the down button first shrinks the viewport, inflates `max_scroll`,
/// and keeps `can_down` stuck true (self-fulfilling overflow).
fn compute_scroll_metrics(
    content_h: f32,
    button_h: f32,
    budget: f32,
    mut scroll_offset: f32,
) -> (OverlayMetrics, f32) {
    let max_h = budget.max(0.0);

    if content_h <= max_h + 0.5 {
        return (
            OverlayMetrics {
                up_h: 0.0,
                down_h: 0.0,
                viewport_h: content_h,
                outer_h: content_h,
                can_up: false,
                can_down: false,
                max_scroll: 0.0,
            },
            0.0,
        );
    }

    // Pass 1: decide buttons. `can_up` depends only on offset. `can_down` uses
    // the remaining budget after an up button — never after a down button.
    let can_up = scroll_offset > 0.5;
    let up_h = if can_up { button_h } else { 0.0 };
    let max_scroll_without_down = (content_h - (max_h - up_h).max(0.0)).max(0.0);
    let can_down = scroll_offset < max_scroll_without_down - 0.5;

    let down_h = if can_down { button_h } else { 0.0 };
    let viewport_h = (max_h - up_h - down_h).max(0.0);
    let max_scroll = (content_h - viewport_h).max(0.0);
    scroll_offset = scroll_offset.clamp(0.0, max_scroll);

    // Pass 2: offset clamp may clear `can_up`; recompute once.
    let can_up = scroll_offset > 0.5;
    let up_h = if can_up { button_h } else { 0.0 };
    let max_scroll_without_down = (content_h - (max_h - up_h).max(0.0)).max(0.0);
    let can_down = scroll_offset < max_scroll_without_down - 0.5;
    let down_h = if can_down { button_h } else { 0.0 };
    let viewport_h = (max_h - up_h - down_h).max(0.0);
    let max_scroll = (content_h - viewport_h).max(0.0);
    scroll_offset = scroll_offset.clamp(0.0, max_scroll);

    (
        OverlayMetrics {
            up_h,
            down_h,
            viewport_h,
            outer_h: up_h + viewport_h + down_h,
            can_up: scroll_offset > 0.5,
            can_down,
            max_scroll,
        },
        scroll_offset,
    )
}

#[allow(clippy::too_many_arguments)]
fn tick_scroll_hold<Message>(
    scroll_offset: &mut f32,
    scroll_hold: &mut Option<(ScrollHoldDir, Instant)>,
    content_h: f32,
    button_h: f32,
    budget: f32,
    step: f32,
    now: Instant,
    shell: &mut Shell<'_, Message>,
) {
    let Some((dir, last)) = *scroll_hold else {
        return;
    };

    let (metrics, clamped) = compute_scroll_metrics(content_h, button_h, budget, *scroll_offset);
    *scroll_offset = clamped;

    let exhausted = match dir {
        ScrollHoldDir::Up => !metrics.can_up,
        ScrollHoldDir::Down => !metrics.can_down,
    };
    if exhausted {
        *scroll_hold = None;
        shell.request_redraw();
        return;
    }

    let next_tick = last + SCROLL_HOLD_INTERVAL;
    if now < next_tick {
        shell.request_redraw_at(next_tick);
        return;
    }

    let delta = match dir {
        ScrollHoldDir::Up => -step,
        ScrollHoldDir::Down => step,
    };
    let (metrics, next) =
        compute_scroll_metrics(content_h, button_h, budget, *scroll_offset + delta);
    *scroll_offset = next;

    let exhausted = match dir {
        ScrollHoldDir::Up => !metrics.can_up,
        ScrollHoldDir::Down => !metrics.can_down,
    };
    if exhausted {
        *scroll_hold = None;
    } else {
        *scroll_hold = Some((dir, now));
        shell.request_redraw_at(now + SCROLL_HOLD_INTERVAL);
    }
    shell.request_redraw();
}

fn arm_scroll_hold<Message>(
    scroll_hold: &mut Option<(ScrollHoldDir, Instant)>,
    dir: ScrollHoldDir,
    now: Instant,
    shell: &mut Shell<'_, Message>,
) {
    if scroll_hold.is_some_and(|(active, _)| active == dir) {
        if let Some((_, last)) = *scroll_hold {
            shell.request_redraw_at(last + SCROLL_HOLD_INTERVAL);
        }
        return;
    }

    *scroll_hold = Some((dir, now));
    shell.request_redraw_at(now + SCROLL_HOLD_INTERVAL);
}

/// Widget-tree state of the trigger and its dropdown.
struct State {
    is_open: bool,
    hovered_row: Option<usize>,
    /// Vertical scroll of the select viewport (bits-ui `viewport.scrollTop`).
    scroll_offset: f32,
    /// Active hover on `.cn-select-scroll-*-button` for bits-ui auto-scroll.
    scroll_hold: Option<(ScrollHoldDir, Instant)>,
    rows: Vec<paragraph::Plain<ParagraphOf>>,
    placeholder: paragraph::Plain<ParagraphOf>,
}

impl State {
    fn new() -> Self {
        Self {
            is_open: false,
            hovered_row: None,
            scroll_offset: 0.0,
            scroll_hold: None,
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
                    state.scroll_hold = None;

                    if let Some(on_close) = &self.on_close {
                        shell.publish(on_close.clone());
                    }

                    shell.capture_event();
                } else if cursor.is_over(layout.bounds()) && self.is_interactive() {
                    state.is_open = true;
                    state.scroll_offset = 0.0;
                    state.scroll_hold = None;
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

        if let Event::Window(window::Event::RedrawRequested(now)) = event {
            self.last_status = Some(status);

            // Drive scroll-button auto-scroll from the widget: overlay hover
            // arms `scroll_hold`, but redraw scheduling is reliable here.
            if state.is_open && state.scroll_hold.is_some() {
                let recipe = style::recipe(self.theme);
                let text_size = self.resolved_text_size();
                let content_h = content_total_height(&self.rows, recipe, text_size);
                let button_h = scroll_button_height(recipe);
                let step = typical_option_height(recipe, text_size);

                tick_scroll_hold(
                    &mut state.scroll_offset,
                    &mut state.scroll_hold,
                    content_h,
                    button_h,
                    self.max_height,
                    step,
                    *now,
                    shell,
                );
            }
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
        _viewport: &Rectangle,
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
            scroll_offset,
            scroll_hold,
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
                list,
                scroll_offset,
                scroll_hold,
                bounds.width.max(recipe.content_min_width_px),
                bounds.height,
                self.max_height,
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

/// Dropdown overlay: bits-ui select content with scroll buttons + clipped viewport.
struct MenuOverlay<'a, T, Message>
where
    T: Clone + PartialEq,
{
    position: Point,
    list: List<'a, T, Message>,
    scroll_offset: &'a mut f32,
    scroll_hold: &'a mut Option<(ScrollHoldDir, Instant)>,
    width: f32,
    target_height: f32,
    max_height: f32,
    style: SelectContentStyle,
}

#[derive(Debug, Clone, Copy)]
struct OverlayMetrics {
    up_h: f32,
    down_h: f32,
    viewport_h: f32,
    outer_h: f32,
    can_up: bool,
    can_down: bool,
    max_scroll: f32,
}

impl<'a, T, Message> MenuOverlay<'a, T, Message>
where
    T: Clone + PartialEq + 'a,
    Message: Clone + 'a,
{
    #[allow(clippy::too_many_arguments)]
    fn new(
        position: Point,
        list: List<'a, T, Message>,
        scroll_offset: &'a mut f32,
        scroll_hold: &'a mut Option<(ScrollHoldDir, Instant)>,
        width: f32,
        target_height: f32,
        max_height: f32,
        style: SelectContentStyle,
    ) -> Self {
        Self {
            position,
            list,
            scroll_offset,
            scroll_hold,
            width,
            target_height,
            max_height: max_height.max(0.0),
            style,
        }
    }

    fn element(self) -> overlay::Element<'a, Message, IcedTheme, Renderer> {
        overlay::Element::new(Box::new(self))
    }

    fn button_height(&self) -> f32 {
        scroll_button_height(self.list.recipe)
    }

    fn step_height(&self) -> f32 {
        // bits-ui scrolls by the highlighted item height; fall back to a typical option row.
        self.list
            .hovered_row
            .and_then(|index| {
                if index < self.list.rows.len() {
                    Some(self.list.row_height(index))
                } else {
                    None
                }
            })
            .unwrap_or_else(|| typical_option_height(self.list.recipe, self.list.text_size))
    }

    fn resolve_metrics(&mut self, available: f32) -> OverlayMetrics {
        let (metrics, offset) = compute_scroll_metrics(
            self.list.total_height(),
            self.button_height(),
            available.min(self.max_height),
            *self.scroll_offset,
        );
        *self.scroll_offset = offset;
        metrics
    }

    fn peek_metrics(&self, available: f32) -> OverlayMetrics {
        compute_scroll_metrics(
            self.list.total_height(),
            self.button_height(),
            available.min(self.max_height),
            *self.scroll_offset,
        )
        .0
    }

    fn scroll_by(&mut self, delta: f32, metrics: &OverlayMetrics, shell: &mut Shell<'_, Message>) {
        if metrics.max_scroll <= 0.0 {
            return;
        }

        let next = (*self.scroll_offset + delta).clamp(0.0, metrics.max_scroll);
        if (next - *self.scroll_offset).abs() > 0.01 {
            *self.scroll_offset = next;
            let _ = self.resolve_metrics(self.max_height);
            shell.request_redraw();
        }
    }

    fn regions(
        &self,
        bounds: Rectangle,
        metrics: OverlayMetrics,
    ) -> (Rectangle, Rectangle, Rectangle) {
        let up = Rectangle {
            x: bounds.x,
            y: bounds.y,
            width: bounds.width,
            height: metrics.up_h,
        };
        let viewport = Rectangle {
            x: bounds.x,
            y: bounds.y + metrics.up_h,
            width: bounds.width,
            height: metrics.viewport_h,
        };
        let down = Rectangle {
            x: bounds.x,
            y: bounds.y + metrics.up_h + metrics.viewport_h,
            width: bounds.width,
            height: metrics.down_h,
        };
        (up, viewport, down)
    }
}

impl<T, Message> overlay::Overlay<Message, IcedTheme, Renderer> for MenuOverlay<'_, T, Message>
where
    T: Clone + PartialEq,
    Message: Clone,
{
    fn layout(&mut self, _renderer: &Renderer, bounds: Size) -> layout::Node {
        let space_below =
            bounds.height - (self.position.y + self.target_height + SELECT_SIDE_OFFSET_PX);
        let space_above = self.position.y - SELECT_SIDE_OFFSET_PX;
        let open_below = space_below >= space_above;
        let space = if open_below { space_below } else { space_above };

        let metrics = self.resolve_metrics(space.max(0.0));
        let size = Size::new(self.width, metrics.outer_h);

        layout::Node::new(size).move_to(if open_below {
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
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) {
        let bounds = layout.bounds();
        let metrics = self.resolve_metrics(bounds.height);
        let (up_bounds, viewport_bounds, down_bounds) = self.regions(bounds, metrics);

        match event {
            Event::Mouse(mouse::Event::WheelScrolled { delta }) => {
                if cursor.is_over(bounds) {
                    let dy = match delta {
                        mouse::ScrollDelta::Lines { y, .. } => -*y * self.step_height(),
                        mouse::ScrollDelta::Pixels { y, .. } => -*y,
                    };
                    self.scroll_by(dy, &metrics, shell);
                    shell.capture_event();
                }
            }
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                // bits-ui also starts auto-scroll on pointerdown; hover (move) is enough too.
                let now = Instant::now();
                if metrics.can_up && cursor.is_over(up_bounds) {
                    arm_scroll_hold(self.scroll_hold, ScrollHoldDir::Up, now, shell);
                } else if metrics.can_down && cursor.is_over(down_bounds) {
                    arm_scroll_hold(self.scroll_hold, ScrollHoldDir::Down, now, shell);
                } else if let Some(position) = cursor.position_in(viewport_bounds) {
                    let content_pos = Point::new(position.x, position.y + *self.scroll_offset);
                    *self.list.hovered_row = self
                        .list
                        .row_at(content_pos)
                        .filter(|&index| self.list.rows[index].is_selectable());
                    self.list.select_hovered(shell);
                }
            }
            Event::Mouse(mouse::Event::CursorMoved { .. })
            | Event::Touch(touch::Event::FingerMoved { .. }) => {
                // bits-ui: pointermove on the scroll button arms auto-scroll; leave clears it.
                let now = Instant::now();
                if metrics.can_up && cursor.is_over(up_bounds) {
                    arm_scroll_hold(self.scroll_hold, ScrollHoldDir::Up, now, shell);
                } else if metrics.can_down && cursor.is_over(down_bounds) {
                    arm_scroll_hold(self.scroll_hold, ScrollHoldDir::Down, now, shell);
                } else {
                    if self.scroll_hold.is_some() {
                        *self.scroll_hold = None;
                        shell.request_redraw();
                    }

                    if let Some(position) = cursor.position_in(viewport_bounds) {
                        let content_pos = Point::new(position.x, position.y + *self.scroll_offset);
                        let hovered = self
                            .list
                            .row_at(content_pos)
                            .filter(|&index| self.list.rows[index].is_selectable());

                        if *self.list.hovered_row != hovered {
                            *self.list.hovered_row = hovered;
                            shell.request_redraw();
                        }
                    }
                }
            }
            Event::Keyboard(keyboard::Event::KeyPressed { key, .. }) => {
                if let Some(action) = nav_action(key) {
                    match action {
                        NavAction::Next => self.list.move_hover(1, shell),
                        NavAction::Previous => self.list.move_hover(-1, shell),
                        NavAction::First => self.list.move_hover_to_edge(true, shell),
                        NavAction::Last => self.list.move_hover_to_edge(false, shell),
                        NavAction::Activate => self.list.select_hovered(shell),
                        _ => {}
                    }
                    // Keep the highlighted row in view.
                    if let Some(index) = *self.list.hovered_row {
                        let mut y = self.list.recipe.content_pad_px;
                        for i in 0..index {
                            y += self.list.row_height(i);
                        }
                        let row_h = self.list.row_height(index);
                        if y < *self.scroll_offset {
                            *self.scroll_offset = y;
                            let _ = self.resolve_metrics(bounds.height);
                            shell.request_redraw();
                        } else if y + row_h > *self.scroll_offset + metrics.viewport_h {
                            *self.scroll_offset =
                                (y + row_h - metrics.viewport_h).clamp(0.0, metrics.max_scroll);
                            let _ = self.resolve_metrics(bounds.height);
                            shell.request_redraw();
                        }
                    }
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
        let bounds = layout.bounds();
        let metrics = self.peek_metrics(bounds.height);
        let (up_bounds, viewport_bounds, down_bounds) = self.regions(bounds, metrics);

        if (metrics.can_up && cursor.is_over(up_bounds))
            || (metrics.can_down && cursor.is_over(down_bounds))
        {
            return mouse::Interaction::Pointer;
        }

        let selectable_under_cursor = cursor
            .position_in(viewport_bounds)
            .map(|position| Point::new(position.x, position.y + *self.scroll_offset))
            .and_then(|position| self.list.row_at(position))
            .is_some_and(|index| self.list.rows[index].is_selectable());

        if selectable_under_cursor {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::default()
        }
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

        crate::floating_surface::fill_floating_surface(
            renderer,
            bounds,
            self.style.background,
            self.style.radius,
            self.style.shadow,
        );

        let metrics = self.peek_metrics(bounds.height);
        let (up_bounds, viewport_bounds, down_bounds) = self.regions(bounds, metrics);
        let content_h = self.list.total_height();

        let content_node = layout::Node::new(Size::new(bounds.width, content_h)).move_to(
            Point::new(viewport_bounds.x, viewport_bounds.y - *self.scroll_offset),
        );
        let content_layout = layout::Layout::new(&content_node);

        renderer.with_layer(viewport_bounds, |renderer| {
            self.list.draw(
                &Tree::empty(),
                renderer,
                theme,
                defaults,
                content_layout,
                cursor,
                &viewport_bounds,
            );
        });

        // After the list — bits-ui scroll buttons use `z-10`. Caret must be
        // drawn inside the same `with_layer` as the strip fill, or it ends up
        // under that layer and vanishes.
        if metrics.can_up {
            paint_scroll_button(
                renderer,
                up_bounds,
                bounds,
                &self.style,
                self.list.recipe.icon_size_px,
                true,
            );
        }

        if metrics.can_down {
            paint_scroll_button(
                renderer,
                down_bounds,
                bounds,
                &self.style,
                self.list.recipe.icon_size_px,
                false,
            );
        }

        crate::floating_surface::paint_outside_ring(
            renderer,
            bounds,
            self.style.border_color,
            self.style.border_width,
            self.style.radius,
        );
    }
}

/// Scroll strip: full-content rounded fill clipped to the strip, then caret
/// in the **same** layer (otherwise the layer composites over the glyph).
fn paint_scroll_button(
    renderer: &mut Renderer,
    strip: Rectangle,
    content_bounds: Rectangle,
    style: &SelectContentStyle,
    icon_size: f32,
    up: bool,
) {
    if strip.height <= 0.0 || strip.width <= 0.0 {
        return;
    }

    renderer.with_layer(strip, |renderer| {
        renderer.fill_quad(
            renderer::Quad {
                bounds: content_bounds,
                border: Border {
                    radius: style.radius.into(),
                    width: 0.0,
                    color: Color::TRANSPARENT,
                },
                shadow: Shadow::default(),
                ..renderer::Quad::default()
            },
            Background::Color(style.background),
        );

        let center = Point::new(strip.center_x(), strip.center_y());
        if up {
            draw_chevron_up(renderer, center, icon_size, style.muted_color);
        } else {
            draw_chevron(renderer, center, icon_size, style.muted_color);
        }
    });
}

/// Paints the lucide-style `chevron-up` glyph of `.cn-select-scroll-up-button`.
fn draw_chevron_up(renderer: &mut Renderer, center: Point, size: f32, color: Color) {
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
            builder.move_to(Point::new(-reach, arm));
            builder.line_to(Point::new(0.0, -arm));
            builder.line_to(Point::new(reach, arm));
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

/// Inner list widget hosted by the dropdown viewport.
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

#[cfg(test)]
mod scroll_metrics_tests {
    use super::compute_scroll_metrics;

    #[test]
    fn down_button_hides_at_content_end_with_up_visible() {
        // Content taller than budget; scrolled so only the up button should remain.
        let content_h = 500.0;
        let button_h = 24.0;
        let budget = 300.0;
        // End of content with only the up button reserved:
        // max_scroll_without_down = content - (budget - button) = 500 - 276 = 224.
        let offset = 224.0;

        let (metrics, clamped) = compute_scroll_metrics(content_h, button_h, budget, offset);

        assert!(metrics.can_up, "scrolled away from top");
        assert!(
            !metrics.can_down,
            "down must hide once content end fits without it"
        );
        assert_eq!(metrics.down_h, 0.0);
        assert!((clamped - 224.0).abs() < 0.01);
    }

    #[test]
    fn down_button_shows_near_top_when_overflowing() {
        let (metrics, _) = compute_scroll_metrics(500.0, 24.0, 300.0, 0.0);
        assert!(!metrics.can_up);
        assert!(metrics.can_down);
        assert!(metrics.down_h > 0.0);
    }

    #[test]
    fn former_sticky_mid_offset_does_not_keep_down_button() {
        // Old iterative algorithm kept can_down true at this offset because
        // reserving the down button inflated max_scroll by button_h.
        let content_h = 500.0;
        let button_h = 24.0;
        let budget = 300.0;
        let sticky_offset = content_h - budget + button_h; // 224.0

        let (metrics, _) = compute_scroll_metrics(content_h, button_h, budget, sticky_offset);

        assert!(metrics.can_up);
        assert!(!metrics.can_down);
    }
}
