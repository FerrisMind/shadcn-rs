//! Iced widget and overlay rendering for [`super::NativeSelect`].

use std::f32;

use iced_core::Renderer as CoreRenderer;
use iced_core::alignment;
use iced_core::border;
use iced_core::keyboard;
use iced_core::layout;
use iced_core::mouse;
use iced_core::overlay;
use iced_core::renderer;
use iced_core::text;
use iced_core::text::Renderer as TextRenderer;
use iced_core::text::paragraph;
use iced_core::touch;
use iced_core::widget::operation;
use iced_core::widget::tree::{self, Tree};
use iced_core::window;
use iced_core::{
    Background, Border, Clipboard, Color, Event, Layout, Length, Padding, Pixels, Point, Rectangle,
    Shell, Size, Vector, Widget,
};
use iced_widget::overlay::menu;
use iced_widget::scrollable::Scrollable;

use crate::iced_compat::widget::Id;
use crate::iced_compat::{Element, Renderer};
use crate::theme::Theme;
use crate::{fonts::iced_font, iced_compat::widget::pick_list};

use super::NativeSelect;
use super::style;
use super::types::{NativeSelectItem, NativeSelectRadius, NativeSelectSize};

#[derive(Debug, Clone)]
pub(super) enum Entry<T> {
    Group {
        label: String,
    },
    Option {
        value: T,
        label: String,
        disabled: bool,
        indented: bool,
    },
}

impl<T> Entry<T> {
    fn label(&self) -> &str {
        match self {
            Self::Group { label } | Self::Option { label, .. } => label,
        }
    }

    const fn is_selectable(&self) -> bool {
        matches!(
            self,
            Self::Option {
                disabled: false,
                ..
            }
        )
    }

    fn value(&self) -> Option<&T> {
        match self {
            Self::Option { value, .. } => Some(value),
            Self::Group { .. } => None,
        }
    }
}

pub(super) fn build<'a, T, Message>(select: NativeSelect<'a, T, Message>) -> Element<'a, Message>
where
    T: Clone + PartialEq + ToString + 'a,
    Message: Clone + 'a,
{
    let NativeSelect {
        theme,
        items,
        selected,
        placeholder,
        size,
        radius,
        width,
        menu_height,
        text_size,
        id,
        disabled,
        invalid,
        on_select,
        on_open,
        on_close,
        style_override,
    } = select;

    let entries = flatten_items(items);
    let menu_class: menu::StyleFn<'a, iced_core::Theme> =
        Box::new(move |_| style::menu_style(theme, radius));

    NativeSelectWidget {
        theme,
        entries,
        selected,
        placeholder,
        size,
        radius,
        width,
        menu_height,
        text_size,
        id,
        invalid,
        disabled,
        on_select,
        on_open,
        on_close,
        style_override,
        menu_class,
        last_status: None,
    }
    .into()
}

pub(super) fn flatten_items<T: Clone>(items: Vec<NativeSelectItem<T>>) -> Vec<Entry<T>> {
    let mut entries = Vec::new();

    for item in items {
        match item {
            NativeSelectItem::Option(option) => entries.push(Entry::Option {
                value: option.value().clone(),
                label: option.label().to_owned(),
                disabled: option.is_disabled(),
                indented: false,
            }),
            NativeSelectItem::OptGroup(group) => {
                entries.push(Entry::Group {
                    label: group.label().to_owned(),
                });

                for option in group.options() {
                    entries.push(Entry::Option {
                        value: option.value().clone(),
                        label: option.label().to_owned(),
                        disabled: group.is_disabled() || option.is_disabled(),
                        indented: true,
                    });
                }
            }
        }
    }

    entries
}

pub(super) fn selected_index<T: PartialEq>(
    entries: &[Entry<T>],
    selected: Option<&T>,
) -> Option<usize> {
    selected.and_then(|selected| {
        entries
            .iter()
            .position(|entry| entry.value().is_some_and(|value| value == selected))
    })
}

pub(super) fn next_selectable_index<T>(
    entries: &[Entry<T>],
    selected: Option<usize>,
    forward: bool,
) -> Option<usize> {
    if entries.is_empty() {
        return None;
    }

    match (selected, forward) {
        (None, true) => (0..entries.len()).find(|&index| entries[index].is_selectable()),
        (None, false) => (0..entries.len())
            .rev()
            .find(|&index| entries[index].is_selectable()),
        (Some(index), true) if index + 1 < entries.len() => {
            ((index + 1)..entries.len()).find(|&index| entries[index].is_selectable())
        }
        (Some(index), false) if index > 0 => (0..index)
            .rev()
            .find(|&index| entries[index].is_selectable()),
        (Some(_), _) => None,
    }
}

pub(super) fn next_matching_index<T>(
    entries: &[Entry<T>],
    current: Option<usize>,
    query: &str,
) -> Option<usize> {
    if entries.is_empty() || query.is_empty() {
        return None;
    }

    let start = current.map_or(0, |index| (index + 1) % entries.len());
    (0..entries.len())
        .map(|offset| (start + offset) % entries.len())
        .find(|&index| {
            entries[index].is_selectable()
                && entries[index].label().to_lowercase().starts_with(query)
        })
}

pub(super) fn typeahead_match<T>(
    entries: &[Entry<T>],
    current: Option<usize>,
    buffer: &str,
    typed: &str,
) -> (Option<usize>, String) {
    let query = format!("{buffer}{typed}");

    if let Some(index) = next_matching_index(entries, current, &query) {
        (Some(index), query)
    } else {
        (
            next_matching_index(entries, current, typed),
            typed.to_owned(),
        )
    }
}

struct NativeSelectWidget<'a, T, Message> {
    theme: &'a Theme,
    entries: Vec<Entry<T>>,
    selected: Option<T>,
    placeholder: Option<String>,
    size: NativeSelectSize,
    radius: Option<NativeSelectRadius>,
    width: Length,
    menu_height: Length,
    text_size: Option<Pixels>,
    id: Option<Id>,
    invalid: bool,
    disabled: bool,
    on_select: Option<Box<dyn Fn(T) -> Message + 'a>>,
    on_open: Option<Message>,
    on_close: Option<Message>,
    style_override:
        Option<Box<dyn Fn(pick_list::Style, pick_list::Status) -> pick_list::Style + 'a>>,
    menu_class: menu::StyleFn<'a, iced_core::Theme>,
    last_status: Option<pick_list::Status>,
}

impl<T, Message> Widget<Message, iced_core::Theme, Renderer> for NativeSelectWidget<'_, T, Message>
where
    T: Clone + PartialEq + ToString,
    Message: Clone,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State<<Renderer as text::Renderer>::Paragraph>>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::<<Renderer as text::Renderer>::Paragraph>::new())
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: Length::Fixed(self.size.control_height(self.theme)),
        }
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        _renderer: &Renderer,
        operation: &mut dyn iced_core::widget::Operation,
    ) {
        let state = tree
            .state
            .downcast_mut::<State<<Renderer as text::Renderer>::Paragraph>>();
        if !self.disabled && self.on_select.is_some() {
            operation.focusable(self.id.as_ref(), layout.bounds(), state);
        }
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let state = tree
            .state
            .downcast_mut::<State<<Renderer as text::Renderer>::Paragraph>>();
        let font = iced_font(self.theme.font_pack().sans);
        let text_size = self
            .text_size
            .unwrap_or_else(|| Pixels::from(style::text_size_for(self.theme, self.size)));
        let line_height = text::LineHeight::Absolute(
            style::line_height_px(self.theme, self.size, text_size.0).into(),
        );
        let option_text = text::Text {
            content: "",
            bounds: Size::new(f32::INFINITY, f32::from(line_height.to_absolute(text_size))),
            size: text_size,
            line_height,
            font,
            align_x: text::Alignment::Default,
            align_y: alignment::Vertical::Center,
            shaping: text::Shaping::default(),
            wrapping: text::Wrapping::default(),
        };

        state
            .options
            .resize_with(self.entries.len(), Default::default);
        for (entry, paragraph) in self.entries.iter().zip(state.options.iter_mut()) {
            let _ = paragraph.update(text::Text {
                content: entry.label(),
                ..option_text
            });
        }

        if let Some(placeholder) = &self.placeholder {
            let _ = state.placeholder.update(text::Text {
                content: placeholder.as_str(),
                ..option_text
            });
        } else {
            state.placeholder = paragraph::Plain::default();
        }

        let max_width = match self.width {
            Length::Shrink => state
                .options
                .iter()
                .fold(0.0, |width, paragraph| {
                    f32::max(width, paragraph.min_width())
                })
                .max(state.placeholder.min_width().min(f32::MAX)),
            _ => 0.0,
        };

        let intrinsic = Size::new(
            max_width + self.padding().x(),
            self.size.control_height(self.theme),
        );

        let size = limits
            .width(self.width)
            .height(Length::Fixed(self.size.control_height(self.theme)))
            .resolve(
                self.width,
                Length::Fixed(self.size.control_height(self.theme)),
                intrinsic,
            );

        layout::Node::new(size)
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        let state = tree
            .state
            .downcast_mut::<State<<Renderer as text::Renderer>::Paragraph>>();
        let interactive = !self.disabled && self.on_select.is_some();

        if !shell.is_event_captured() {
            match event {
                Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
                | Event::Touch(touch::Event::FingerPressed { .. }) => {
                    if state.is_open {
                        state.is_open = false;
                        state.typeahead.clear();
                        if let Some(on_close) = &self.on_close {
                            shell.publish(on_close.clone());
                        }
                        shell.capture_event();
                    } else if interactive && cursor.is_over(layout.bounds()) {
                        state.focused = true;
                        state.is_open = true;
                        state.typeahead.clear();
                        state.hovered_option =
                            selected_index(&self.entries, self.selected.as_ref())
                                .or_else(|| next_selectable_index(&self.entries, None, true));
                        if let Some(on_open) = &self.on_open {
                            shell.publish(on_open.clone());
                        }
                        shell.capture_event();
                    } else {
                        state.focused = false;
                    }
                }
                Event::Mouse(mouse::Event::WheelScrolled {
                    delta: mouse::ScrollDelta::Lines { y, .. },
                }) if interactive
                    && state.keyboard_modifiers.command()
                    && cursor.is_over(layout.bounds())
                    && !state.is_open =>
                {
                    let selected = selected_index(&self.entries, self.selected.as_ref());
                    let next = if *y < 0.0 {
                        next_selectable_index(&self.entries, selected, true)
                    } else if *y > 0.0 {
                        next_selectable_index(&self.entries, selected, false)
                    } else {
                        None
                    };

                    if let Some(index) = next
                        && let Some(Entry::Option { value, .. }) = self.entries.get(index)
                        && let Some(on_select) = &self.on_select
                    {
                        shell.publish(on_select(value.clone()));
                    }
                    shell.capture_event();
                }
                Event::Keyboard(keyboard::Event::KeyPressed { key, text, .. }) if interactive => {
                    if state.is_open {
                        let current = state
                            .hovered_option
                            .or_else(|| selected_index(&self.entries, self.selected.as_ref()));
                        let next = match key {
                            keyboard::Key::Named(keyboard::key::Named::ArrowDown)
                            | keyboard::Key::Named(keyboard::key::Named::ArrowRight) => {
                                next_selectable_index(&self.entries, current, true)
                            }
                            keyboard::Key::Named(keyboard::key::Named::ArrowUp)
                            | keyboard::Key::Named(keyboard::key::Named::ArrowLeft) => {
                                next_selectable_index(&self.entries, current, false)
                            }
                            keyboard::Key::Named(keyboard::key::Named::Home) => {
                                next_selectable_index(&self.entries, None, true)
                            }
                            keyboard::Key::Named(keyboard::key::Named::End) => {
                                next_selectable_index(&self.entries, None, false)
                            }
                            keyboard::Key::Named(keyboard::key::Named::Escape) => {
                                state.is_open = false;
                                state.typeahead.clear();
                                if let Some(on_close) = &self.on_close {
                                    shell.publish(on_close.clone());
                                }
                                shell.capture_event();
                                None
                            }
                            keyboard::Key::Named(keyboard::key::Named::Enter)
                            | keyboard::Key::Named(keyboard::key::Named::Space) => {
                                if let Some(index) = state.hovered_option
                                    && let Some(Entry::Option {
                                        value,
                                        disabled: false,
                                        ..
                                    }) = self.entries.get(index)
                                {
                                    if let Some(on_select) = &self.on_select {
                                        shell.publish(on_select(value.clone()));
                                    }
                                    state.is_open = false;
                                    state.typeahead.clear();
                                }
                                shell.capture_event();
                                None
                            }
                            _ => {
                                if !state.keyboard_modifiers.command()
                                    && !state.keyboard_modifiers.alt()
                                    && let Some(text) = text
                                    && text.chars().any(|character| !character.is_control())
                                {
                                    let typed = text.to_lowercase();
                                    let start = state.hovered_option;
                                    let (index, query) = typeahead_match(
                                        &self.entries,
                                        start,
                                        &state.typeahead,
                                        &typed,
                                    );

                                    state.typeahead = query;
                                    if let Some(index) = index {
                                        state.hovered_option = Some(index);
                                        shell.request_redraw();
                                    }
                                    shell.capture_event();
                                    return;
                                }
                                None
                            }
                        };

                        if let Some(next) = next {
                            state.hovered_option = Some(next);
                            state.typeahead.clear();
                            shell.request_redraw();
                            shell.capture_event();
                        } else if matches!(
                            key,
                            keyboard::Key::Named(
                                keyboard::key::Named::ArrowDown
                                    | keyboard::key::Named::ArrowUp
                                    | keyboard::key::Named::ArrowLeft
                                    | keyboard::key::Named::ArrowRight
                                    | keyboard::key::Named::Home
                                    | keyboard::key::Named::End
                            )
                        ) {
                            shell.capture_event();
                        }
                    } else if state.focused
                        && matches!(
                            key,
                            keyboard::Key::Named(
                                keyboard::key::Named::ArrowDown
                                    | keyboard::key::Named::ArrowUp
                                    | keyboard::key::Named::ArrowLeft
                                    | keyboard::key::Named::ArrowRight
                                    | keyboard::key::Named::Enter
                                    | keyboard::key::Named::Space
                            )
                        )
                    {
                        state.is_open = true;
                        state.typeahead.clear();
                        state.hovered_option =
                            selected_index(&self.entries, self.selected.as_ref())
                                .or_else(|| next_selectable_index(&self.entries, None, true));
                        if let Some(on_open) = &self.on_open {
                            shell.publish(on_open.clone());
                        }
                        shell.capture_event();
                    }
                }
                Event::Keyboard(keyboard::Event::ModifiersChanged(modifiers)) => {
                    state.keyboard_modifiers = *modifiers;
                }
                _ => {}
            }
        }

        let is_hovered = cursor.is_over(layout.bounds());
        let status = if state.is_open {
            pick_list::Status::Opened { is_hovered }
        } else if is_hovered {
            pick_list::Status::Hovered
        } else {
            pick_list::Status::Active
        };

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
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        if !self.disabled && self.on_select.is_some() && cursor.is_over(layout.bounds()) {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::default()
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        _iced_theme: &iced_core::Theme,
        _defaults: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        let mut style = style::resolve(
            self.theme,
            self.radius,
            self.invalid,
            self.disabled,
            self.last_status.unwrap_or(pick_list::Status::Active),
        );

        let state = tree
            .state
            .downcast_ref::<State<<Renderer as text::Renderer>::Paragraph>>();
        if state.focused && !self.invalid {
            style.border.color = self.theme.semantic_color(crate::SemanticColor::Ring);
        }

        if let Some(override_fn) = self.style_override.as_ref() {
            style = override_fn(style, self.last_status.unwrap_or(pick_list::Status::Active));
        }

        let bottom_border = style::uses_bottom_border(self.theme);
        let mut frame_border = style.border;
        if bottom_border {
            frame_border.color = Color::TRANSPARENT;
        }

        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border: frame_border,
                ..renderer::Quad::default()
            },
            style.background,
        );

        if bottom_border && style.border.width > 0.0 {
            let thickness = style.border.width.min(bounds.height);
            renderer.fill_quad(
                renderer::Quad {
                    bounds: Rectangle {
                        x: bounds.x,
                        y: bounds.y + bounds.height - thickness,
                        width: bounds.width,
                        height: thickness,
                    },
                    ..renderer::Quad::default()
                },
                Background::Color(style.border.color),
            );
        }

        if (state.is_open || state.focused)
            && !self.disabled
            && style::focus_ring_px(self.theme) > 0.0
        {
            let ring_width = style::focus_ring_px(self.theme);
            let ring_color = if self.invalid {
                let destructive = self.theme.semantic_color(crate::SemanticColor::Destructive);
                with_alpha(destructive, if self.theme.is_dark() { 0.4 } else { 0.2 })
            } else {
                with_alpha(self.theme.semantic_color(crate::SemanticColor::Ring), 0.5)
            };
            let ring = Border {
                color: ring_color,
                width: ring_width,
                radius: style.border.radius,
            };
            renderer.fill_quad(
                renderer::Quad {
                    bounds: Rectangle {
                        x: bounds.x - ring_width,
                        y: bounds.y - ring_width,
                        width: bounds.width + ring_width * 2.0,
                        height: bounds.height + ring_width * 2.0,
                    },
                    border: ring,
                    ..renderer::Quad::default()
                },
                Background::Color(Color::TRANSPARENT),
            );
        }

        let text_size = self
            .text_size
            .unwrap_or_else(|| Pixels::from(style::text_size_for(self.theme, self.size)));
        let line_height = text::LineHeight::Absolute(
            style::line_height_px(self.theme, self.size, text_size.0).into(),
        );
        let label = self
            .selected
            .as_ref()
            .and_then(|selected| {
                self.entries.iter().find_map(|entry| {
                    entry
                        .value()
                        .is_some_and(|value| value == selected)
                        .then(|| entry.label().to_owned())
                })
            })
            .or_else(|| self.selected.as_ref().map(ToString::to_string))
            .or_else(|| self.placeholder.clone())
            .or_else(|| {
                self.entries.iter().find_map(|entry| match entry {
                    Entry::Option { label, .. } => Some(label.clone()),
                    Entry::Group { .. } => None,
                })
            });

        if let Some(label) = label {
            renderer.fill_text(
                text::Text {
                    content: label,
                    size: text_size,
                    line_height,
                    font: iced_font(self.theme.font_pack().sans),
                    bounds: Size::new(
                        bounds.width - self.padding().x(),
                        f32::from(line_height.to_absolute(text_size)),
                    ),
                    align_x: text::Alignment::Default,
                    align_y: alignment::Vertical::Center,
                    shaping: text::Shaping::default(),
                    wrapping: text::Wrapping::default(),
                },
                Point::new(bounds.x + self.padding().left, bounds.center_y()),
                if self.selected.is_some() {
                    style.text_color
                } else {
                    style.placeholder_color
                },
                *viewport,
            );
        }

        let icon_size = Pixels::from(style::icon_size_for(self.theme, self.size));
        renderer.fill_text(
            text::Text {
                content: <Renderer as text::Renderer>::ARROW_DOWN_ICON.to_string(),
                size: icon_size,
                line_height: text::LineHeight::default(),
                font: <Renderer as text::Renderer>::ICON_FONT,
                bounds: Size::new(bounds.width, f32::from(icon_size)),
                align_x: text::Alignment::Right,
                align_y: alignment::Vertical::Center,
                shaping: text::Shaping::Basic,
                wrapping: text::Wrapping::default(),
            },
            Point::new(
                bounds.x + bounds.width - style::icon_right(self.theme),
                bounds.center_y(),
            ),
            style.handle_color,
            *viewport,
        );
    }
    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'_>,
        _renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, iced_core::Theme, Renderer>> {
        let state = tree
            .state
            .downcast_mut::<State<<Renderer as text::Renderer>::Paragraph>>();
        if !state.is_open {
            return None;
        }

        let bounds = layout.bounds();
        let entries = &self.entries;
        let on_select = &self.on_select;
        let is_open = &mut state.is_open;
        let menu_tree = &mut state.menu_tree;
        let hovered_option = &mut state.hovered_option;
        let typeahead = &mut state.typeahead;

        let menu = NativeMenu::new(
            menu_tree,
            entries,
            hovered_option,
            move |index| {
                *is_open = false;
                typeahead.clear();
                let Some(Entry::Option {
                    value, disabled, ..
                }) = entries.get(index)
                else {
                    return None;
                };

                if *disabled {
                    return None;
                }

                on_select.as_ref().map(|on_select| on_select(value.clone()))
            },
            &self.menu_class,
        )
        .width(bounds.width)
        .padding(self.padding())
        .font(iced_font(self.theme.font_pack().sans))
        .text_size(
            self.text_size
                .unwrap_or_else(|| Pixels::from(style::text_size_for(self.theme, self.size))),
        )
        .text_line_height(text::LineHeight::Absolute(
            style::line_height_px(
                self.theme,
                self.size,
                self.text_size
                    .unwrap_or_else(|| Pixels::from(style::text_size_for(self.theme, self.size)))
                    .0,
            )
            .into(),
        ));

        Some(menu.overlay(
            layout.position() + translation,
            *viewport,
            bounds.height,
            self.menu_height,
        ))
    }
}

impl<'a, T, Message> NativeSelectWidget<'a, T, Message> {
    fn padding(&self) -> Padding {
        style::padding(self.theme, self.size)
    }
}

impl<'a, T, Message> From<NativeSelectWidget<'a, T, Message>> for Element<'a, Message>
where
    T: Clone + PartialEq + ToString + 'a,
    Message: Clone + 'a,
{
    fn from(widget: NativeSelectWidget<'a, T, Message>) -> Self {
        Element::new(widget)
    }
}

#[derive(Debug)]
struct State<P: text::Paragraph> {
    menu_tree: Tree,
    keyboard_modifiers: keyboard::Modifiers,
    focused: bool,
    is_open: bool,
    hovered_option: Option<usize>,
    typeahead: String,
    options: Vec<paragraph::Plain<P>>,
    placeholder: paragraph::Plain<P>,
}

impl<P: text::Paragraph> State<P> {
    fn new() -> Self {
        Self {
            menu_tree: Tree::empty(),
            keyboard_modifiers: keyboard::Modifiers::default(),
            focused: false,
            is_open: false,
            hovered_option: None,
            typeahead: String::new(),
            options: Vec::new(),
            placeholder: paragraph::Plain::default(),
        }
    }
}

impl<P: text::Paragraph> operation::Focusable for State<P> {
    fn is_focused(&self) -> bool {
        self.focused
    }

    fn focus(&mut self) {
        self.focused = true;
    }

    fn unfocus(&mut self) {
        self.focused = false;
        self.is_open = false;
        self.typeahead.clear();
    }
}

struct NativeMenu<'a, 'b, T, Message>
where
    'b: 'a,
{
    state: &'a mut Tree,
    entries: &'a [Entry<T>],
    hovered_option: &'a mut Option<usize>,
    on_selected: Box<dyn FnMut(usize) -> Option<Message> + 'a>,
    width: f32,
    padding: Padding,
    text_size: Option<Pixels>,
    text_line_height: text::LineHeight,
    font: Option<iced_core::Font>,
    class: &'a menu::StyleFn<'b, iced_core::Theme>,
}

impl<'a, 'b, T, Message> NativeMenu<'a, 'b, T, Message>
where
    T: Clone + ToString,
    Message: 'a,
    'b: 'a,
{
    fn new(
        state: &'a mut Tree,
        entries: &'a [Entry<T>],
        hovered_option: &'a mut Option<usize>,
        on_selected: impl FnMut(usize) -> Option<Message> + 'a,
        class: &'a menu::StyleFn<'b, iced_core::Theme>,
    ) -> Self {
        Self {
            state,
            entries,
            hovered_option,
            on_selected: Box::new(on_selected),
            width: 0.0,
            padding: Padding::ZERO,
            text_size: None,
            text_line_height: text::LineHeight::default(),
            font: None,
            class,
        }
    }

    fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    fn padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }

    fn text_size(mut self, text_size: Pixels) -> Self {
        self.text_size = Some(text_size);
        self
    }

    fn text_line_height(mut self, line_height: text::LineHeight) -> Self {
        self.text_line_height = line_height;
        self
    }

    fn font(mut self, font: iced_core::Font) -> Self {
        self.font = Some(font);
        self
    }

    fn overlay(
        self,
        position: Point,
        viewport: Rectangle,
        target_height: f32,
        menu_height: Length,
    ) -> overlay::Element<'a, Message, iced_core::Theme, Renderer> {
        overlay::Element::new(Box::new(NativeMenuOverlay::new(
            position,
            viewport,
            self,
            target_height,
            menu_height,
        )))
    }
}

struct NativeMenuOverlay<'a, 'b, Message>
where
    'b: 'a,
{
    position: Point,
    viewport: Rectangle,
    tree: &'a mut Tree,
    list: Scrollable<'a, Message, iced_core::Theme, Renderer>,
    width: f32,
    target_height: f32,
    class: &'a menu::StyleFn<'b, iced_core::Theme>,
}

impl<'a, 'b, Message> NativeMenuOverlay<'a, 'b, Message>
where
    Message: 'a,
    'b: 'a,
{
    fn new<T>(
        position: Point,
        viewport: Rectangle,
        menu: NativeMenu<'a, 'b, T, Message>,
        target_height: f32,
        menu_height: Length,
    ) -> Self
    where
        T: Clone + ToString,
    {
        let NativeMenu {
            state,
            entries,
            hovered_option,
            on_selected,
            width,
            padding,
            text_size,
            text_line_height,
            font,
            class,
            ..
        } = menu;

        let list = Scrollable::new(Element::new(NativeMenuList {
            entries,
            hovered_option,
            on_selected,
            padding,
            text_size,
            text_line_height,
            font,
            class,
        }))
        .height(menu_height);

        state.diff(&list as &dyn Widget<_, _, _>);

        Self {
            position,
            viewport,
            tree: state,
            list,
            width,
            target_height,
            class,
        }
    }
}

impl<Message> iced_core::Overlay<Message, iced_core::Theme, Renderer>
    for NativeMenuOverlay<'_, '_, Message>
{
    fn layout(&mut self, renderer: &Renderer, bounds: Size) -> layout::Node {
        let space_below = bounds.height - (self.position.y + self.target_height);
        let space_above = self.position.y;
        let limits = layout::Limits::new(
            Size::ZERO,
            Size::new(
                bounds.width - self.position.x,
                if space_below > space_above {
                    space_below
                } else {
                    space_above
                },
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
        layout: Layout<'_>,
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
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.list
            .mouse_interaction(self.tree, layout, cursor, &self.viewport, renderer)
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        theme: &iced_core::Theme,
        defaults: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
    ) {
        let bounds = layout.bounds();
        let menu_style = menu::Catalog::style(theme, self.class);
        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border: menu_style.border,
                shadow: menu_style.shadow,
                ..renderer::Quad::default()
            },
            menu_style.background,
        );
        self.list.draw(
            self.tree, renderer, theme, defaults, layout, cursor, &bounds,
        );
    }
}

struct NativeMenuList<'a, 'b, T, Message>
where
    'b: 'a,
{
    entries: &'a [Entry<T>],
    hovered_option: &'a mut Option<usize>,
    on_selected: Box<dyn FnMut(usize) -> Option<Message> + 'a>,
    padding: Padding,
    text_size: Option<Pixels>,
    text_line_height: text::LineHeight,
    font: Option<iced_core::Font>,
    class: &'a menu::StyleFn<'b, iced_core::Theme>,
}

struct NativeMenuListState {
    is_hovered: Option<bool>,
}

impl<T, Message> Widget<Message, iced_core::Theme, Renderer> for NativeMenuList<'_, '_, T, Message>
where
    T: Clone + ToString,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<NativeMenuListState>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(NativeMenuListState { is_hovered: None })
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Fill,
            height: Length::Shrink,
        }
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let text_size = self.text_size.unwrap_or_else(|| renderer.default_size());
        let row_height = f32::from(self.text_line_height.to_absolute(text_size)) + self.padding.y();
        let intrinsic = Size::new(0.0, row_height * self.entries.len() as f32);
        layout::Node::new(limits.resolve(Length::Fill, Length::Shrink, intrinsic))
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        let text_size = self.text_size.unwrap_or_else(|| renderer.default_size());
        let row_height = f32::from(self.text_line_height.to_absolute(text_size)) + self.padding.y();

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                if let Some(cursor_position) = cursor.position_in(layout.bounds()) {
                    let index = (cursor_position.y / row_height) as usize;
                    if let Some(entry) = self.entries.get(index) {
                        *self.hovered_option = Some(index);
                        if entry.is_selectable()
                            && let Some(message) = (self.on_selected)(index)
                        {
                            shell.publish(message);
                        }
                        shell.capture_event();
                    }
                }
            }
            Event::Mouse(mouse::Event::CursorMoved { .. }) => {
                if let Some(cursor_position) = cursor.position_in(layout.bounds()) {
                    let index = (cursor_position.y / row_height) as usize;
                    if index < self.entries.len() {
                        if *self.hovered_option != Some(index) {
                            shell.request_redraw();
                        }
                        *self.hovered_option = Some(index);
                    }
                }
            }
            Event::Touch(touch::Event::FingerPressed { .. }) => {
                if let Some(cursor_position) = cursor.position_in(layout.bounds()) {
                    let index = (cursor_position.y / row_height) as usize;
                    *self.hovered_option = (index < self.entries.len()).then_some(index);
                    if self.entries.get(index).is_some() {
                        if self.entries[index].is_selectable()
                            && let Some(message) = (self.on_selected)(index)
                        {
                            shell.publish(message);
                        }
                        shell.capture_event();
                    }
                }
            }
            _ => {}
        }

        let state = tree.state.downcast_mut::<NativeMenuListState>();
        if let Event::Window(window::Event::RedrawRequested(_now)) = event {
            state.is_hovered = Some(cursor.is_over(layout.bounds()));
        } else if state
            .is_hovered
            .is_some_and(|is_hovered| is_hovered != cursor.is_over(layout.bounds()))
        {
            shell.request_redraw();
        }
    }

    fn mouse_interaction(
        &self,
        _tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        if let Some(position) = cursor.position_in(layout.bounds()) {
            let text_size = self.text_size.unwrap_or_else(|| renderer.default_size());
            let row_height =
                f32::from(self.text_line_height.to_absolute(text_size)) + self.padding.y();
            let index = (position.y / row_height) as usize;
            if self.entries.get(index).is_some_and(Entry::is_selectable) {
                return mouse::Interaction::Pointer;
            }
        }
        mouse::Interaction::default()
    }

    fn draw(
        &self,
        _tree: &Tree,
        renderer: &mut Renderer,
        theme: &iced_core::Theme,
        _defaults: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let menu_style = menu::Catalog::style(theme, self.class);
        let text_size = self.text_size.unwrap_or_else(|| renderer.default_size());
        let row_height = f32::from(self.text_line_height.to_absolute(text_size)) + self.padding.y();
        let bounds = layout.bounds();
        let offset = viewport.y - bounds.y;
        let start = (offset / row_height).max(0.0) as usize;
        let start = start.min(self.entries.len());
        let end = ((offset + viewport.height) / row_height).ceil().max(0.0) as usize;
        let end = end.max(start).min(self.entries.len());

        for (offset, entry) in self.entries[start..end].iter().enumerate() {
            let index = start + offset;
            let row_bounds = Rectangle {
                x: bounds.x,
                y: bounds.y + row_height * index as f32,
                width: bounds.width,
                height: row_height,
            };
            let hovered = *self.hovered_option == Some(index) && entry.is_selectable();
            if hovered {
                renderer.fill_quad(
                    renderer::Quad {
                        bounds: Rectangle {
                            x: row_bounds.x + menu_style.border.width,
                            width: row_bounds.width - menu_style.border.width * 2.0,
                            ..row_bounds
                        },
                        border: border::rounded(menu_style.border.radius),
                        ..renderer::Quad::default()
                    },
                    menu_style.selected_background,
                );
            }

            let (color, left, size) = match entry {
                Entry::Group { .. } => (
                    with_alpha(menu_style.text_color, 0.65),
                    self.padding.left,
                    text_size,
                ),
                Entry::Option {
                    disabled, indented, ..
                } => (
                    if *disabled {
                        with_alpha(menu_style.text_color, 0.45)
                    } else if hovered {
                        menu_style.selected_text_color
                    } else {
                        menu_style.text_color
                    },
                    self.padding.left + if *indented { 8.0 } else { 0.0 },
                    text_size,
                ),
            };

            renderer.fill_text(
                text::Text {
                    content: entry.label().to_owned(),
                    bounds: Size::new(f32::INFINITY, row_bounds.height),
                    size,
                    line_height: self.text_line_height,
                    font: self.font.unwrap_or_else(|| renderer.default_font()),
                    align_x: text::Alignment::Default,
                    align_y: alignment::Vertical::Center,
                    shaping: text::Shaping::default(),
                    wrapping: text::Wrapping::default(),
                },
                Point::new(row_bounds.x + left, row_bounds.center_y()),
                color,
                *viewport,
            );
        }
    }
}

fn with_alpha(color: iced_core::Color, alpha: f32) -> iced_core::Color {
    iced_core::Color {
        a: (color.a * alpha).clamp(0.0, 1.0),
        ..color
    }
}
