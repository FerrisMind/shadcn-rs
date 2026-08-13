//! Layout, interaction, and rendering for the tabs builders.

use crate::components::button::Button;
use crate::fonts::iced_font;
use crate::iced_compat::advanced::layout::{self, Layout};
use crate::iced_compat::advanced::renderer::Renderer as _;
use crate::iced_compat::advanced::widget::{Operation, Tree, tree};
use crate::iced_compat::advanced::{Clipboard, Shell, Widget, overlay, renderer};
use crate::iced_compat::widget::{column, container, row, text};
use crate::iced_compat::{
    Background, Border, Color, Element, Event, Length, Padding, Point, Rectangle, Size, Vector,
    mouse,
};
use crate::theme::Theme;
use iced_core::keyboard;
use shadcn_common::{Direction, NavAction, NavKey, Orientation, resolve_nav_action};

use super::geometry::{self, TabsMetrics};
use super::style;
use super::types::{
    Tabs, TabsActivationMode, TabsContent, TabsContentValue, TabsHover, TabsJustify, TabsList,
    TabsListLoop, TabsListVariant, TabsOrientation, TabsSize, TabsTrigger, TabsTriggerContent,
    TabsWrap,
};

#[derive(Clone, Debug)]
pub(crate) struct TabsTriggerMeta {
    pub(crate) value: String,
    pub(crate) disabled: bool,
}

#[derive(Debug, Default)]
struct TabsListState {
    focused: bool,
    focus_visible: bool,
    focused_index: Option<usize>,
    active_index: Option<usize>,
    trigger_bounds: Vec<Rectangle>,
}

struct TabsListWidget<'a, Message> {
    triggers: Vec<Element<'a, Message>>,
    items: Vec<TabsTriggerMeta>,
    active: String,
    on_value_change: Option<Box<dyn Fn(String) -> Message + 'a>>,
    orientation: TabsOrientation,
    activation_mode: TabsActivationMode,
    list_loop: TabsListLoop,
    variant: TabsListVariant,
    wrap: TabsWrap,
    justify: TabsJustify,
    full_width: bool,
    width: Length,
    height: Length,
    theme: &'a Theme,
    style_override: Option<Box<dyn Fn(container::Style) -> container::Style + 'a>>,
    metrics: TabsMetrics,
    disabled: bool,
}

struct TriggerOptions<'a, Message> {
    theme: &'a Theme,
    metrics: TabsMetrics,
    variant: TabsListVariant,
    hover: TabsHover,
    size: TabsSize,
    orientation: TabsOrientation,
    full_width: bool,
    active: bool,
    disabled: bool,
    on_press: Option<Message>,
}

/// Builds a complete root, resolving only the active content panel.
pub(super) fn build_tabs<'a, Message>(tabs: Tabs<'a, Message>) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let Tabs {
        theme: _,
        list,
        contents,
        value,
        orientation,
        activation_mode,
        list_loop,
        spacing,
        width,
        height,
        padding,
        disabled,
        on_value_change,
        style_override,
    } = tabs;

    let active = resolve_active_value(&list.triggers, &value);
    let has_panels = !contents.is_empty();
    let panel = contents
        .into_iter()
        .find(|content| content.value == active)
        .map(build_content)
        .unwrap_or_else(|| crate::iced_compat::widget::space().into());

    let list_element = build_list(
        list,
        active.clone(),
        orientation,
        activation_mode,
        list_loop,
        on_value_change,
        disabled,
    );

    let content: Element<'a, Message> = if !has_panels {
        // Chrome-style tab strips often use the list alone; skip the empty
        // panel column so a fixed-height parent does not squeeze triggers.
        list_element
    } else if orientation.is_vertical() {
        row![list_element, panel]
            .spacing(spacing)
            .align_y(crate::iced_compat::alignment::Vertical::Top)
            .into()
    } else {
        column![list_element, panel].spacing(spacing).into()
    };

    container(content)
        .width(width)
        .height(height)
        .padding(padding)
        .style(move |_iced_theme| {
            let mut resolved = container::Style::default();
            if let Some(override_fn) = style_override.as_ref() {
                resolved = override_fn(resolved);
            }
            resolved
        })
        .into()
}

/// Builds a list and wires its trigger callbacks and keyboard policy.
pub(super) fn build_list<'a, Message>(
    list: TabsList<'a, Message>,
    active: String,
    orientation: TabsOrientation,
    activation_mode: TabsActivationMode,
    list_loop: TabsListLoop,
    on_value_change: Option<Box<dyn Fn(String) -> Message + 'a>>,
    disabled: bool,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let TabsList {
        theme,
        triggers: trigger_definitions,
        variant,
        size,
        wrap,
        justify,
        hover,
        full_width,
        width,
        height,
        gap,
        list_padding,
        style_override,
    } = list;

    let active = resolve_active_value(&trigger_definitions, &active);
    let mut metrics = geometry::resolve_metrics(theme, size, orientation, variant);
    if let Some(gap) = gap {
        metrics.gap = gap;
    }
    if let Some(list_padding) = list_padding {
        metrics.list_padding = list_padding;
        metrics.vertical_list_padding = list_padding;
    }
    let mut triggers = Vec::with_capacity(trigger_definitions.len());
    let mut items = Vec::with_capacity(trigger_definitions.len());

    for trigger in trigger_definitions {
        let is_active = trigger.value == active && !trigger.disabled;
        let value = trigger.value.clone();
        let effective_disabled = disabled || trigger.disabled;
        let message = (!effective_disabled)
            .then(|| {
                on_value_change
                    .as_ref()
                    .map(|callback| callback(value.clone()))
            })
            .flatten();

        triggers.push(build_trigger(
            trigger,
            TriggerOptions {
                theme,
                metrics,
                variant,
                hover,
                size,
                orientation,
                full_width,
                active: is_active,
                disabled: effective_disabled,
                on_press: message,
            },
        ));
        items.push(TabsTriggerMeta {
            value,
            disabled: effective_disabled,
        });
    }

    Element::new(TabsListWidget {
        triggers,
        items,
        active,
        on_value_change,
        orientation,
        activation_mode,
        list_loop,
        variant,
        wrap,
        justify,
        full_width,
        width,
        height,
        theme,
        style_override,
        metrics,
        disabled,
    })
}

pub(super) fn build_standalone_trigger<'a, Message>(
    trigger: TabsTrigger<'a, Message>,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let theme = trigger.theme;
    let metrics = geometry::resolve_metrics(
        theme,
        TabsSize::Default,
        TabsOrientation::Horizontal,
        TabsListVariant::Default,
    );
    build_trigger(
        trigger,
        TriggerOptions {
            theme,
            metrics,
            variant: TabsListVariant::Default,
            hover: TabsHover::Subtle,
            size: TabsSize::Default,
            orientation: TabsOrientation::Horizontal,
            full_width: false,
            active: false,
            disabled: false,
            on_press: None,
        },
    )
}

fn build_trigger<'a, Message>(
    trigger: TabsTrigger<'a, Message>,
    options: TriggerOptions<'a, Message>,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let TriggerOptions {
        theme,
        metrics,
        variant,
        hover,
        size,
        orientation,
        full_width,
        active,
        disabled,
        on_press,
    } = options;

    let TabsTrigger {
        content,
        width,
        height,
        padding,
        style_override,
        ..
    } = trigger;

    let button = match content {
        TabsTriggerContent::Label(label) => Button::text(label, theme),
        TabsTriggerContent::Element(content) => Button::new(content, theme),
    };

    let resolved_width = width.unwrap_or(if full_width || orientation.is_vertical() {
        Length::Fill
    } else {
        Length::Shrink
    });
    let resolved_height = height.unwrap_or(Length::Fixed(metrics.trigger_height));
    let (trigger_pad_x, trigger_pad_y) = metrics.trigger_padding_for(orientation);
    let resolved_padding = padding.unwrap_or(Padding {
        top: trigger_pad_y,
        right: trigger_pad_x,
        bottom: trigger_pad_y,
        left: trigger_pad_x,
    });

    let mut button = button
        .size(geometry::button_size(size))
        .width(resolved_width)
        .height(resolved_height)
        .disabled(disabled)
        .style_override(move |_base, status| {
            let mut resolved = style::resolve_trigger_style(
                theme, metrics, variant, hover, active, disabled, status,
            );
            if let Some(override_fn) = style_override.as_ref() {
                resolved = override_fn(resolved, status);
            }
            resolved
        })
        .into_button();

    button = button
        .width(resolved_width)
        .height(resolved_height)
        .padding(resolved_padding);

    if let Some(message) = on_press {
        button = button.on_press(message);
    }

    button.into()
}

pub(super) fn build_content<'a, Message>(content: TabsContent<'a, Message>) -> Element<'a, Message>
where
    Message: 'a,
{
    let TabsContent {
        theme,
        content,
        width,
        height,
        padding,
        style_override,
        ..
    } = content;

    let content: Element<'a, Message> = match content {
        TabsContentValue::Label(label) => {
            let metrics = geometry::resolve_metrics(
                theme,
                TabsSize::Default,
                TabsOrientation::Horizontal,
                TabsListVariant::Default,
            );
            let mut font = iced_font(theme.font_pack().sans);
            font.weight = crate::recipes::iced_font_weight(shadcn_common::FontWeight::Normal);
            let label = label.into_owned();

            text(label)
                .size(metrics.content_text_size)
                .font(font)
                .line_height(crate::iced_compat::widget::text::LineHeight::Absolute(
                    metrics.content_line_height.into(),
                ))
                .into()
        }
        TabsContentValue::Element(content) => content,
    };

    container(content)
        .width(width)
        .height(height)
        .padding(padding)
        .style(move |_iced_theme| {
            let mut resolved = style::resolve_content_style(theme);
            if let Some(override_fn) = style_override.as_ref() {
                resolved = override_fn(resolved);
            }
            resolved
        })
        .into()
}

impl<Message> Widget<Message, crate::iced_compat::Theme, crate::iced_compat::Renderer>
    for TabsListWidget<'_, Message>
where
    Message: Clone,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<TabsListState>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(TabsListState::default())
    }

    fn children(&self) -> Vec<Tree> {
        self.triggers.iter().map(Tree::new).collect()
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&self.triggers);
    }

    fn size(&self) -> Size<Length> {
        Size::new(self.width, self.height)
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &crate::iced_compat::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let max = limits.max();
        let min = limits.min();
        let count = self.triggers.len();
        let horizontal = !self.orientation.is_vertical();
        let list_padding = self.metrics.list_padding_for(self.orientation);
        let target_width = geometry::resolve_length(self.width, max.width, min.width, max.width);
        let full_width_each = if self.full_width && horizontal && self.wrap == TabsWrap::NoWrap {
            let gaps = self.gap_width(count);
            Some(((target_width - list_padding * 2.0 - gaps) / count.max(1) as f32).max(0.0))
        } else {
            None
        };

        let mut child_nodes = Vec::with_capacity(count);
        for (index, child) in self.triggers.iter_mut().enumerate() {
            let child_limits = if let Some(each_width) = full_width_each {
                layout::Limits::new(
                    Size::new(each_width, 0.0),
                    Size::new(each_width, max.height),
                )
            } else {
                layout::Limits::new(Size::ZERO, max).width(Length::Shrink)
            };
            child_nodes.push(child.as_widget_mut().layout(
                &mut tree.children[index],
                renderer,
                &child_limits,
            ));
        }

        let mut lines = Vec::new();
        match self.orientation {
            TabsOrientation::Horizontal => {
                let mut current = Line::default();
                let wrap_width = target_width;
                for (index, node) in child_nodes.iter().enumerate() {
                    let node_width = node.size().width;
                    let proposed = if current.indices.is_empty() {
                        node_width
                    } else {
                        current.width + self.metrics.gap + node_width
                    };
                    let should_wrap = self.wrap != TabsWrap::NoWrap
                        && self.width != Length::Shrink
                        && !current.indices.is_empty()
                        && list_padding * 2.0 + proposed > wrap_width;

                    if should_wrap {
                        lines.push(current);
                        current = Line::default();
                    }

                    current.indices.push(index);
                    current.width = if current.width == 0.0 {
                        node_width
                    } else {
                        current.width + self.metrics.gap + node_width
                    };
                    current.height = current.height.max(node.size().height);
                }
                if !current.indices.is_empty() {
                    lines.push(current);
                }
            }
            TabsOrientation::Vertical => {
                for (index, node) in child_nodes.iter().enumerate() {
                    lines.push(Line {
                        indices: vec![index],
                        width: node.size().width,
                        height: node.size().height,
                    });
                }
            }
        }

        if lines.is_empty() {
            lines.push(Line::default());
        }

        let content_width = lines.iter().map(|line| line.width).fold(0.0, f32::max);
        let content_height = match self.orientation {
            TabsOrientation::Horizontal => {
                lines.iter().map(|line| line.height).sum::<f32>()
                    + self.metrics.line_gap * lines.len().saturating_sub(1) as f32
            }
            TabsOrientation::Vertical => {
                lines.iter().map(|line| line.height).sum::<f32>()
                    + self.metrics.gap * lines.len().saturating_sub(1) as f32
            }
        };

        let natural_width = content_width + list_padding * 2.0;
        let natural_height = content_height + list_padding * 2.0;
        let width = geometry::resolve_length(self.width, natural_width, min.width, max.width);
        let height = geometry::resolve_length(self.height, natural_height, min.height, max.height);

        if self.orientation.is_vertical() {
            let child_width = (width - list_padding * 2.0).max(0.0);
            for (index, child) in self.triggers.iter_mut().enumerate() {
                child_nodes[index] = child.as_widget_mut().layout(
                    &mut tree.children[index],
                    renderer,
                    &layout::Limits::new(
                        Size::new(child_width, 0.0),
                        Size::new(child_width, max.height),
                    ),
                );
            }
        }

        let mut y = list_padding;
        if self.wrap == TabsWrap::WrapReverse && horizontal {
            y = (height - list_padding - content_height).max(0.0);
        }

        let mut trigger_bounds = vec![Rectangle::default(); child_nodes.len()];
        for (line_index, line) in lines.iter().enumerate() {
            let line_space = (width - list_padding * 2.0 - line.width).max(0.0);
            let offset = match self.justify {
                TabsJustify::Start => 0.0,
                TabsJustify::Center => line_space / 2.0,
                TabsJustify::End => line_space,
            };
            let mut x = list_padding + offset;

            for index in &line.indices {
                let node_size = child_nodes[*index].size();
                let child_y = y + (line.height - node_size.height).max(0.0) / 2.0;
                let node =
                    std::mem::replace(&mut child_nodes[*index], layout::Node::new(Size::ZERO))
                        .move_to(Point::new(x, child_y));
                trigger_bounds[*index] = node.bounds();
                child_nodes[*index] = node;
                x += node_size.width + self.metrics.gap;
            }

            if line_index + 1 < lines.len() {
                let line_gap = if horizontal {
                    self.metrics.line_gap
                } else {
                    self.metrics.gap
                };
                y += line.height + line_gap;
            }
        }

        tree.state.downcast_mut::<TabsListState>().trigger_bounds = trigger_bounds;
        layout::Node::with_children(Size::new(width, height), child_nodes)
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &crate::iced_compat::Renderer,
        operation: &mut dyn Operation,
    ) {
        for ((child, child_tree), child_layout) in self
            .triggers
            .iter_mut()
            .zip(&mut tree.children)
            .zip(layout.children())
        {
            child
                .as_widget_mut()
                .operate(child_tree, child_layout, renderer, operation);
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
        let active_index = resolve_active_index(&self.items, &self.active);
        let state = tree.state.downcast_mut::<TabsListState>();
        if active_index != state.active_index {
            state.active_index = active_index;
            state.focused_index = active_index;
        }

        let interactive = !self.disabled && self.on_value_change.is_some();
        if interactive {
            match event {
                Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
                | Event::Touch(crate::iced_compat::touch::Event::FingerPressed { .. }) => {
                    if cursor.is_over(layout.bounds()) {
                        state.focused = true;
                        state.focus_visible = false;
                        state.focused_index = state.focused_index.or(active_index);
                    } else {
                        state.focused = false;
                    }
                }
                Event::Keyboard(keyboard::Event::KeyPressed { key, .. }) if state.focused => {
                    let current = state.focused_index.or(active_index);
                    let action = nav_action(key, self.orientation);
                    if let Some(next_index) = self.next_index(action, current) {
                        state.focused_index = Some(next_index);
                        state.focus_visible = true;

                        let activate = matches!(action, Some(NavAction::Activate))
                            || matches!(self.activation_mode, TabsActivationMode::Automatic);
                        if activate && let Some(callback) = self.on_value_change.as_ref() {
                            shell.publish(callback(self.items[next_index].value.clone()));
                        }
                        shell.capture_event();
                        return;
                    }
                }
                _ => {}
            }
        }

        for ((child, child_tree), child_layout) in self
            .triggers
            .iter_mut()
            .zip(&mut tree.children)
            .zip(layout.children())
        {
            child.as_widget_mut().update(
                child_tree,
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
        for ((child, child_tree), child_layout) in self
            .triggers
            .iter()
            .zip(&tree.children)
            .zip(layout.children())
        {
            let interaction = child.as_widget().mouse_interaction(
                child_tree,
                child_layout,
                cursor,
                viewport,
                renderer,
            );
            if interaction != mouse::Interaction::default() {
                return interaction;
            }
        }

        if self.on_value_change.is_some() && !self.disabled && cursor.is_over(layout.bounds()) {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::default()
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut crate::iced_compat::Renderer,
        iced_theme: &crate::iced_compat::Theme,
        inherited_style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        if !bounds.intersects(viewport) {
            return;
        }

        let mut list_style =
            style::resolve_list_style(self.theme, self.metrics, self.variant, self.orientation);
        if let Some(override_fn) = self.style_override.as_ref() {
            list_style = override_fn(list_style);
        }

        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border: list_style.border,
                shadow: list_style.shadow,
                ..renderer::Quad::default()
            },
            list_style
                .background
                .unwrap_or(Background::Color(Color::TRANSPARENT)),
        );

        for ((child, child_tree), child_layout) in self
            .triggers
            .iter()
            .zip(&tree.children)
            .zip(layout.children())
        {
            child.as_widget().draw(
                child_tree,
                renderer,
                iced_theme,
                &renderer::Style {
                    text_color: list_style.text_color.unwrap_or(inherited_style.text_color),
                },
                child_layout,
                cursor,
                viewport,
            );
        }

        let state = tree.state.downcast_ref::<TabsListState>();
        if self.variant == TabsListVariant::Line
            && let Some(index) = state.active_index
            && let Some(trigger) = state.trigger_bounds.get(index)
        {
            let indicator_color = self.theme.palette.foreground;
            let indicator = if self.orientation.is_vertical() {
                Rectangle {
                    x: bounds.x + trigger.x + trigger.width - 1.0,
                    y: bounds.y + trigger.y,
                    width: 2.0,
                    height: trigger.height,
                }
            } else {
                Rectangle {
                    x: bounds.x + trigger.x,
                    y: bounds.y + trigger.y + trigger.height + 3.0,
                    width: trigger.width,
                    height: 2.0,
                }
            };
            renderer.fill_quad(
                renderer::Quad {
                    bounds: indicator,
                    ..renderer::Quad::default()
                },
                Background::Color(indicator_color),
            );
        }

        if state.focused
            && state.focus_visible
            && let Some(index) = state.focused_index
            && let Some(trigger) = state.trigger_bounds.get(index)
        {
            let focus = Rectangle {
                x: bounds.x + trigger.x - 2.0,
                y: bounds.y + trigger.y - 2.0,
                width: trigger.width + 4.0,
                height: trigger.height + 4.0,
            };
            renderer.fill_quad(
                renderer::Quad {
                    bounds: focus,
                    border: Border {
                        color: self.theme.palette.ring,
                        width: 2.0,
                        radius: self.metrics.trigger_radius.into(),
                    },
                    ..renderer::Quad::default()
                },
                Background::Color(Color::TRANSPARENT),
            );
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
            &mut self.triggers,
            tree,
            layout,
            renderer,
            viewport,
            translation,
        )
    }
}

impl<'a, Message: Clone + 'a> TabsListWidget<'a, Message> {
    fn gap_width(&self, count: usize) -> f32 {
        self.metrics.gap * count.saturating_sub(1) as f32
    }

    fn next_index(&self, action: Option<NavAction>, current: Option<usize>) -> Option<usize> {
        match action {
            Some(NavAction::First) => first_enabled_index(&self.items),
            Some(NavAction::Last) => last_enabled_index(&self.items),
            Some(NavAction::Activate) => current,
            Some(NavAction::Next | NavAction::Previous) => current.and_then(|current| {
                let delta = if matches!(action, Some(NavAction::Next)) {
                    1
                } else {
                    -1
                };
                next_enabled_index(
                    &self.items,
                    current,
                    delta,
                    matches!(self.list_loop, TabsListLoop::Enabled),
                )
            }),
            None => None,
            Some(_) => None,
        }
    }
}

#[derive(Default)]
struct Line {
    indices: Vec<usize>,
    width: f32,
    height: f32,
}

pub(crate) fn resolve_active_value<Message>(
    items: &[TabsTrigger<'_, Message>],
    active: &str,
) -> String {
    items
        .iter()
        .find(|item| item.value == active && !item.disabled)
        .or_else(|| items.iter().find(|item| !item.disabled))
        .map_or_else(|| active.to_owned(), |item| item.value.clone())
}

pub(crate) fn resolve_active_index(items: &[TabsTriggerMeta], active: &str) -> Option<usize> {
    items
        .iter()
        .position(|item| item.value == active && !item.disabled)
        .or_else(|| first_enabled_index(items))
}

pub(crate) fn first_enabled_index(items: &[TabsTriggerMeta]) -> Option<usize> {
    shadcn_common::first_enabled_index(items, |item| !item.disabled)
}

fn last_enabled_index(items: &[TabsTriggerMeta]) -> Option<usize> {
    shadcn_common::last_enabled_index(items, |item| !item.disabled)
}

pub(crate) fn next_enabled_index(
    items: &[TabsTriggerMeta],
    start: usize,
    delta: isize,
    looping: bool,
) -> Option<usize> {
    shadcn_common::step_index(items, Some(start), delta, looping, |item| !item.disabled)
}

fn nav_action(key: &keyboard::Key, orientation: TabsOrientation) -> Option<NavAction> {
    let key = match key {
        keyboard::Key::Named(keyboard::key::Named::ArrowLeft) => NavKey::ArrowLeft,
        keyboard::Key::Named(keyboard::key::Named::ArrowRight) => NavKey::ArrowRight,
        keyboard::Key::Named(keyboard::key::Named::ArrowUp) => NavKey::ArrowUp,
        keyboard::Key::Named(keyboard::key::Named::ArrowDown) => NavKey::ArrowDown,
        keyboard::Key::Named(keyboard::key::Named::Home) => NavKey::Home,
        keyboard::Key::Named(keyboard::key::Named::End) => NavKey::End,
        keyboard::Key::Named(keyboard::key::Named::Enter) => NavKey::Enter,
        keyboard::Key::Named(keyboard::key::Named::Space) => NavKey::Space,
        _ => return None,
    };

    let orientation = if orientation.is_vertical() {
        Orientation::Vertical
    } else {
        Orientation::Horizontal
    };

    resolve_nav_action(key, orientation, Direction::Ltr)
}
