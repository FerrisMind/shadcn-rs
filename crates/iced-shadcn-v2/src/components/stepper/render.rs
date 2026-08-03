//! Layout, state, and keyboard interaction for the stepper.

use crate::components::button::Button;
use crate::fonts::iced_font;
use crate::iced_compat::advanced::layout::{self, Layout};
use crate::iced_compat::advanced::renderer::Renderer as _;
use crate::iced_compat::advanced::widget::{Operation, Tree, tree};
use crate::iced_compat::advanced::{Clipboard, Shell, Widget, overlay, renderer};
use crate::iced_compat::widget::{button as button_widget, column, container, row, space, text};
use crate::iced_compat::{
    Background, Border, Color, Element, Event, Font, Length, Padding, Point, Rectangle, Size,
    Vector, mouse,
};
use crate::theme::Theme;
use iced_core::keyboard;
use shadcn_common::{Direction, NavAction, NavKey, Orientation, resolve_nav_action};

use super::geometry::{self, StepperMetrics};
use super::types::{
    Stepper, StepperButtonContent, StepperContent, StepperDescription, StepperIndicator,
    StepperItem, StepperItemState, StepperNav, StepperNext, StepperOrientation, StepperPrevious,
    StepperSeparator, StepperTitle, StepperTrigger,
};

#[derive(Debug, Clone, Copy)]
struct SeparatorMeta {
    offset: f32,
    thickness: f32,
    color: Option<Color>,
    completed_color: Option<Color>,
    custom: bool,
}

#[derive(Debug, Clone)]
struct StepperItemMeta {
    disabled: bool,
    indicator_size: f32,
    indicator_ring: Option<f32>,
    indicator_ring_color: Color,
    separator: SeparatorMeta,
}

struct TextContentOptions<'a> {
    color: Color,
    size: f32,
    line_height: f32,
    custom_font: Option<Font>,
    weight: shadcn_common::FontWeight,
    orientation: StepperOrientation,
    style_override: Option<Box<dyn Fn(container::Style) -> container::Style + 'a>>,
}

struct ControlButtonOptions<'a, Message> {
    variant: super::super::button::ButtonVariant,
    size: super::super::button::ButtonSize,
    color: Option<shadcn_common::AccentColor>,
    disabled: bool,
    message: Option<Message>,
    style_override: Option<
        Box<dyn Fn(button_widget::Style, button_widget::Status) -> button_widget::Style + 'a>,
    >,
}

struct TriggerBuildOptions<Message> {
    disabled: bool,
    message: Option<Message>,
    external_ring: bool,
}

#[derive(Debug, Default)]
struct StepperNavState {
    focused: bool,
    focus_visible: bool,
    focused_index: Option<usize>,
    active_index: Option<usize>,
    trigger_bounds: Vec<Rectangle>,
}

struct StepperNavWidget<'a, Message> {
    children: Vec<Element<'a, Message>>,
    trigger_count: usize,
    items: Vec<StepperItemMeta>,
    active_step: usize,
    orientation: StepperOrientation,
    width: Length,
    height: Length,
    padding: Padding,
    gap: f32,
    theme: &'a Theme,
    disabled: bool,
    on_step_change: Option<Box<dyn Fn(usize) -> Message + 'a>>,
    metrics: StepperMetrics,
    style_override: Option<Box<dyn Fn(container::Style) -> container::Style + 'static>>,
}

pub(super) fn build_stepper<'a, Message>(stepper: Stepper<'a, Message>) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let Stepper {
        theme,
        items,
        nav,
        step,
        previous,
        next,
        spacing,
        width,
        height,
        padding,
        disabled,
        on_step_change,
        style_override,
    } = stepper;

    let active_step = geometry::resolve_step(step, items.len());
    let next_step = geometry::next_step(active_step, items.len());
    let previous_step = geometry::previous_step(active_step);
    let root_spacing = spacing.unwrap_or(theme.style.spacing_unit_px * 2.0);

    let previous = previous.map(|control| {
        let StepperPrevious {
            theme: control_theme,
            content,
            variant,
            size,
            color,
            disabled: control_disabled,
            on_press,
            style_override,
        } = control;
        let message = on_press.or_else(|| {
            previous_step.and_then(|step| on_step_change.as_ref().map(|callback| callback(step)))
        });
        build_control_button(
            content,
            control_theme,
            ControlButtonOptions {
                variant,
                size,
                color,
                disabled: control_disabled
                    || disabled
                    || previous_step.is_none()
                    || message.is_none(),
                message,
                style_override,
            },
        )
    });

    let next = next.map(|control| {
        let StepperNext {
            theme: control_theme,
            content,
            variant,
            size,
            color,
            disabled: control_disabled,
            on_press,
            style_override,
        } = control;
        let message = on_press.or_else(|| {
            next_step.and_then(|step| on_step_change.as_ref().map(|callback| callback(step)))
        });
        build_control_button(
            content,
            control_theme,
            ControlButtonOptions {
                variant,
                size,
                color,
                disabled: control_disabled || disabled || next_step.is_none() || message.is_none(),
                message,
                style_override,
            },
        )
    });

    let nav_element = build_nav(theme, nav, items, active_step, disabled, on_step_change);

    let content: Element<'a, Message> = match (previous, next) {
        (Some(previous), Some(next)) => column![
            nav_element,
            row![previous, space().width(Length::Fill), next].width(Length::Fill)
        ]
        .spacing(root_spacing)
        .into(),
        (Some(previous), None) => column![nav_element, row![previous].width(Length::Fill)]
            .spacing(root_spacing)
            .into(),
        (None, Some(next)) => column![nav_element, row![next].width(Length::Fill)]
            .spacing(root_spacing)
            .into(),
        (None, None) => nav_element,
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

fn build_nav<'a, Message>(
    theme: &'a Theme,
    nav: StepperNav,
    items: Vec<StepperItem<'a, Message>>,
    active_step: usize,
    disabled: bool,
    on_step_change: Option<Box<dyn Fn(usize) -> Message + 'a>>,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let StepperNav {
        orientation,
        width,
        height,
        padding,
        gap,
        style_override,
    } = nav;
    let metrics = StepperMetrics::for_theme(theme);
    let nav_gap = gap.unwrap_or(metrics.vertical_gap);
    let mut trigger_children = Vec::with_capacity(items.len());
    let mut separator_children = Vec::with_capacity(items.len());
    let mut item_meta = Vec::with_capacity(items.len());

    for (index, item) in items.into_iter().enumerate() {
        let StepperItem {
            id: _,
            trigger,
            separator,
            disabled: item_disabled,
        } = item;
        let effective_disabled = disabled || item_disabled || trigger.disabled;
        let item_state = geometry::state_for_step(index + 1, active_step);
        let (indicator_size, indicator_ring, indicator_ring_color) = trigger
            .indicator
            .as_ref()
            .map(|indicator| {
                (
                    indicator.size.unwrap_or(metrics.indicator_size),
                    Some(metrics.indicator_ring),
                    indicator.ring_color.unwrap_or(theme.palette.background),
                )
            })
            .unwrap_or((metrics.indicator_size, None, theme.palette.background));
        let trigger_message = trigger.on_press.clone().or_else(|| {
            (!effective_disabled)
                .then(|| on_step_change.as_ref().map(|callback| callback(index + 1)))
                .flatten()
        });
        let trigger = build_trigger(
            trigger,
            item_state,
            orientation,
            theme,
            &metrics,
            TriggerBuildOptions {
                disabled: effective_disabled,
                message: trigger_message,
                external_ring: true,
            },
        );
        trigger_children.push(trigger);

        let separator = separator.unwrap_or_else(|| StepperSeparator::new(theme));
        let SeparatorMeta {
            offset,
            thickness,
            color,
            completed_color,
            custom,
        } = separator_meta(&separator);
        let separator_element = if custom {
            build_custom_separator(separator)
        } else {
            space().into()
        };
        separator_children.push(separator_element);
        item_meta.push(StepperItemMeta {
            disabled: effective_disabled,
            indicator_size,
            indicator_ring,
            indicator_ring_color,
            separator: SeparatorMeta {
                offset,
                thickness,
                color,
                completed_color,
                custom,
            },
        });
    }

    // `StepperNavWidget` lays out these as two contiguous child ranges:
    // triggers first, then their separators. Keep that ordering even though
    // the public composition is item-by-item.
    trigger_children.extend(separator_children);

    Element::new(StepperNavWidget {
        trigger_count: item_meta.len(),
        children: trigger_children,
        items: item_meta,
        active_step,
        orientation,
        width,
        height,
        padding,
        gap: nav_gap,
        theme,
        disabled,
        on_step_change,
        metrics,
        style_override,
    })
}

fn separator_meta<Message>(separator: &StepperSeparator<'_, Message>) -> SeparatorMeta {
    SeparatorMeta {
        offset: separator.offset,
        thickness: separator.thickness,
        color: separator.color,
        completed_color: separator.completed_color,
        custom: separator.content.is_some() || separator.style_override.is_some(),
    }
}

fn build_custom_separator<'a, Message>(
    separator: StepperSeparator<'a, Message>,
) -> Element<'a, Message>
where
    Message: 'a,
{
    let StepperSeparator {
        content,
        style_override,
        ..
    } = separator;
    let content: Element<'a, Message> = content
        .map(|content| match content {
            StepperContent::Label(label) => text(label.into_owned()).into(),
            StepperContent::Element(element) => element,
        })
        .unwrap_or_else(|| space().into());
    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(move |_iced_theme| {
            let mut style = container::Style::default();
            if let Some(override_fn) = style_override.as_ref() {
                style = override_fn(style);
            }
            style
        })
        .into()
}

fn build_trigger<'a, Message>(
    trigger: StepperTrigger<'a, Message>,
    item_state: StepperItemState,
    orientation: StepperOrientation,
    theme: &'a Theme,
    metrics: &StepperMetrics,
    options: TriggerBuildOptions<Message>,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let TriggerBuildOptions {
        disabled,
        message,
        external_ring,
    } = options;
    let StepperTrigger {
        indicator,
        title,
        description,
        children,
        width,
        height,
        gap,
        style_override,
        ..
    } = trigger;

    let indicator =
        indicator.map(|indicator| build_indicator(indicator, item_state, *metrics, external_ring));
    let title = title.map(|title| build_title(title, orientation));
    let description = description.map(|description| build_description(description, orientation));
    let has_labels = title.is_some() || description.is_some();
    let mut labels = column![];
    if let Some(title) = title {
        labels = labels.push(title);
    }
    if let Some(description) = description {
        labels = labels.push(description);
    }
    let content: Element<'a, Message> = match orientation {
        StepperOrientation::Horizontal => {
            let mut content = column![]
                .align_x(crate::iced_compat::alignment::Horizontal::Center)
                .spacing(gap.unwrap_or(0.0));
            if let Some(indicator) = indicator {
                content = content.push(indicator);
            }
            if has_labels {
                content = content.push(labels);
            }
            children
                .into_iter()
                .fold(content, |content, child| content.push(child))
                .into()
        }
        StepperOrientation::Vertical => {
            let mut content = row![]
                .align_y(crate::iced_compat::alignment::Vertical::Top)
                .spacing(gap.unwrap_or(metrics.vertical_trigger_gap));
            if let Some(indicator) = indicator {
                content = content.push(indicator);
            }
            if has_labels {
                content = content.push(labels);
            }
            children
                .into_iter()
                .fold(content, |content, child| content.push(child))
                .into()
        }
    };

    let mut widget = button_widget(content)
        .width(width)
        .height(height)
        .padding(Padding::default())
        .style(move |_iced_theme, status| {
            let mut style = button_widget::Style {
                background: None,
                text_color: theme.palette.foreground,
                border: Border::default(),
                shadow: Default::default(),
                snap: true,
            };
            if let Some(override_fn) = style_override.as_ref() {
                style = override_fn(style, status);
            }
            style
        });
    if !disabled && let Some(message) = message {
        widget = widget.on_press(message);
    }
    widget.into()
}

fn build_indicator<'a, Message>(
    indicator: StepperIndicator<'a, Message>,
    item_state: StepperItemState,
    metrics: StepperMetrics,
    external_ring: bool,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let StepperIndicator {
        content,
        theme,
        size,
        foreground,
        background,
        ring_color,
        style_override,
    } = indicator;
    let size = size.unwrap_or(metrics.indicator_size);
    let (default_background, default_foreground) = match item_state {
        StepperItemState::Completed | StepperItemState::Active => {
            (theme.palette.primary, theme.palette.primary_foreground)
        }
        StepperItemState::Inactive => (theme.palette.muted, theme.palette.muted_foreground),
    };
    let content = match content {
        StepperContent::Label(label) => text(label.into_owned())
            .size(16.0)
            .font(iced_font(theme.font_pack().sans))
            .into(),
        StepperContent::Element(content) => content,
    };
    container(content)
        .width(Length::Fixed(size))
        .height(Length::Fixed(size))
        .align_x(crate::iced_compat::alignment::Horizontal::Center)
        .align_y(crate::iced_compat::alignment::Vertical::Center)
        .style(move |_iced_theme| {
            let mut style = container::Style {
                background: Some(Background::Color(background.unwrap_or(default_background))),
                text_color: Some(foreground.unwrap_or(default_foreground)),
                border: Border {
                    color: ring_color.unwrap_or(theme.palette.background),
                    width: if external_ring {
                        0.0
                    } else {
                        metrics.indicator_ring
                    },
                    radius: (size / 2.0).into(),
                },
                ..container::Style::default()
            };
            if let Some(override_fn) = style_override.as_ref() {
                style = override_fn(style);
            }
            style
        })
        .into()
}

pub(super) fn build_standalone_indicator<'a, Message>(
    indicator: StepperIndicator<'a, Message>,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let theme = indicator.theme;
    build_indicator(
        indicator,
        StepperItemState::Active,
        StepperMetrics::for_theme(theme),
        false,
    )
}

pub(super) fn build_title<'a, Message>(
    title: StepperTitle<'a, Message>,
    orientation: StepperOrientation,
) -> Element<'a, Message>
where
    Message: 'a,
{
    let StepperTitle {
        content,
        theme,
        color,
        text_size,
        line_height,
        font,
        style_override,
    } = title;
    let metrics = StepperMetrics::for_theme(theme);
    build_text_content(
        content,
        theme,
        TextContentOptions {
            color: color.unwrap_or(theme.palette.foreground),
            size: text_size.unwrap_or(metrics.title_size),
            line_height: line_height.unwrap_or(metrics.title_line_height),
            custom_font: font,
            weight: metrics.title_weight,
            orientation,
            style_override,
        },
    )
}

pub(super) fn build_description<'a, Message>(
    description: StepperDescription<'a, Message>,
    orientation: StepperOrientation,
) -> Element<'a, Message>
where
    Message: 'a,
{
    let StepperDescription {
        content,
        theme,
        color,
        text_size,
        line_height,
        font,
        style_override,
    } = description;
    let metrics = StepperMetrics::for_theme(theme);
    build_text_content(
        content,
        theme,
        TextContentOptions {
            color: color.unwrap_or(theme.palette.muted_foreground),
            size: text_size.unwrap_or(metrics.description_size),
            line_height: line_height.unwrap_or(metrics.description_line_height),
            custom_font: font,
            weight: shadcn_common::FontWeight::Normal,
            orientation,
            style_override,
        },
    )
}

fn build_text_content<'a, Message>(
    content: StepperContent<'a, Message>,
    theme: &'a Theme,
    options: TextContentOptions<'a>,
) -> Element<'a, Message>
where
    Message: 'a,
{
    let TextContentOptions {
        color,
        size,
        line_height,
        custom_font,
        weight,
        orientation,
        style_override,
    } = options;
    let mut font = custom_font.unwrap_or_else(|| iced_font(theme.font_pack().sans));
    font.weight = crate::recipes::iced_font_weight(weight);
    let content: Element<'a, Message> = match content {
        StepperContent::Label(label) => text(label.into_owned())
            .size(size)
            .font(font)
            .line_height(crate::iced_compat::widget::text::LineHeight::Absolute(
                line_height.into(),
            ))
            .color(color)
            .into(),
        StepperContent::Element(content) => content,
    };
    container(content)
        .width(Length::Shrink)
        .height(Length::Shrink)
        .align_x(if orientation.is_vertical() {
            crate::iced_compat::alignment::Horizontal::Left
        } else {
            crate::iced_compat::alignment::Horizontal::Center
        })
        .style(move |_iced_theme| {
            let mut style = container::Style {
                text_color: Some(color),
                ..container::Style::default()
            };
            if let Some(override_fn) = style_override.as_ref() {
                style = override_fn(style);
            }
            style
        })
        .into()
}

pub(super) fn build_standalone_trigger<'a, Message>(
    trigger: StepperTrigger<'a, Message>,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let theme = trigger.theme;
    build_trigger(
        trigger,
        StepperItemState::Active,
        StepperOrientation::Horizontal,
        theme,
        &StepperMetrics::for_theme(theme),
        TriggerBuildOptions {
            disabled: false,
            message: None,
            external_ring: false,
        },
    )
}

pub(super) fn build_standalone_separator<'a, Message>(
    separator: StepperSeparator<'a, Message>,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let StepperSeparator {
        content,
        theme,
        thickness,
        color,
        style_override,
        ..
    } = separator;
    let content: Element<'a, Message> = content
        .map(|content| match content {
            StepperContent::Label(label) => text(label.into_owned()).into(),
            StepperContent::Element(element) => element,
        })
        .unwrap_or_else(|| space().into());
    container(content)
        .width(Length::Fill)
        .height(Length::Fixed(thickness))
        .style(move |_iced_theme| {
            let mut style = container::Style {
                background: color.or(Some(theme.palette.muted)).map(Background::Color),
                ..container::Style::default()
            };
            if let Some(override_fn) = style_override.as_ref() {
                style = override_fn(style);
            }
            style
        })
        .into()
}

fn build_control_button<'a, Message>(
    content: StepperButtonContent<'a, Message>,
    theme: &'a Theme,
    options: ControlButtonOptions<'a, Message>,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let ControlButtonOptions {
        variant,
        size,
        color,
        disabled,
        message,
        style_override,
    } = options;
    let mut button = match content {
        StepperButtonContent::Label(label) => Button::text(label, theme),
        StepperButtonContent::Element(content) => Button::new(content, theme),
    }
    .variant(variant)
    .size(size)
    .disabled(disabled);
    if let Some(color) = color {
        button = button.color(color);
    }
    if let Some(style_override) = style_override {
        button = button.style_override(style_override);
    }
    if let Some(message) = message {
        button = button.on_press(message);
    }
    button.into()
}

pub(super) fn build_standalone_next<'a, Message>(
    next: StepperNext<'a, Message>,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let StepperNext {
        theme,
        content,
        variant,
        size,
        color,
        disabled,
        on_press,
        style_override,
    } = next;
    build_control_button(
        content,
        theme,
        ControlButtonOptions {
            variant,
            size,
            color,
            disabled: disabled || on_press.is_none(),
            message: on_press,
            style_override,
        },
    )
}

pub(super) fn build_standalone_previous<'a, Message>(
    previous: StepperPrevious<'a, Message>,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let StepperPrevious {
        theme,
        content,
        variant,
        size,
        color,
        disabled,
        on_press,
        style_override,
    } = previous;
    build_control_button(
        content,
        theme,
        ControlButtonOptions {
            variant,
            size,
            color,
            disabled: disabled || on_press.is_none(),
            message: on_press,
            style_override,
        },
    )
}

impl<Message> Widget<Message, crate::iced_compat::Theme, crate::iced_compat::Renderer>
    for StepperNavWidget<'_, Message>
where
    Message: Clone,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<StepperNavState>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(StepperNavState::default())
    }

    fn children(&self) -> Vec<Tree> {
        self.children.iter().map(Tree::new).collect()
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&self.children);
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
        let horizontal = !self.orientation.is_vertical();
        let left_right = self.padding.left + self.padding.right;
        let top_bottom = self.padding.top + self.padding.bottom;
        let mut child_nodes = Vec::with_capacity(self.children.len());

        for (index, child) in self.children.iter_mut().enumerate() {
            let child_limits = if index < self.trigger_count {
                layout::Limits::new(
                    Size::ZERO,
                    Size::new(
                        (max.width - left_right).max(0.0),
                        (max.height - top_bottom).max(0.0),
                    ),
                )
            } else {
                layout::Limits::new(Size::ZERO, max)
            };
            child_nodes.push(child.as_widget_mut().layout(
                &mut tree.children[index],
                renderer,
                &child_limits,
            ));
        }

        let natural_trigger_width = child_nodes[..self.trigger_count]
            .iter()
            .map(|node| node.size().width)
            .fold(0.0, f32::max);
        let natural_trigger_height = child_nodes[..self.trigger_count]
            .iter()
            .map(|node| node.size().height)
            .fold(0.0, f32::max);
        let natural_width = if horizontal {
            child_nodes[..self.trigger_count]
                .iter()
                .map(|node| node.size().width)
                .sum::<f32>()
        } else {
            natural_trigger_width
        } + left_right;
        let natural_height = if horizontal {
            natural_trigger_height
        } else {
            child_nodes[..self.trigger_count]
                .iter()
                .map(|node| node.size().height)
                .sum::<f32>()
                + self.gap * self.trigger_count.saturating_sub(1) as f32
        } + top_bottom;
        let width = geometry::resolve_length(self.width, natural_width, min.width, max.width);
        let height = geometry::resolve_length(self.height, natural_height, min.height, max.height);
        let content_width = (width - left_right).max(0.0);
        let mut trigger_bounds = vec![Rectangle::default(); self.trigger_count];
        let mut separator_bounds = vec![Rectangle::default(); self.trigger_count];

        if horizontal {
            let last_width = child_nodes
                .get(self.trigger_count.saturating_sub(1))
                .map_or(0.0, |node| node.size().width);
            let flex_count = self.trigger_count.saturating_sub(1);
            let flex_width = if flex_count == 0 {
                0.0
            } else {
                ((content_width - last_width).max(0.0)) / flex_count as f32
            };
            let mut x = self.padding.left;
            for index in 0..self.trigger_count {
                let cell_width = if index + 1 == self.trigger_count {
                    last_width
                } else {
                    flex_width
                };
                let node =
                    std::mem::replace(&mut child_nodes[index], layout::Node::new(Size::ZERO))
                        .move_to(Point::new(x, self.padding.top));
                trigger_bounds[index] = node.bounds();
                child_nodes[index] = node;
                if index + 1 < self.trigger_count {
                    let separator = &self.items[index].separator;
                    separator_bounds[index] = Rectangle {
                        x: x + separator.offset.min(cell_width),
                        y: self.padding.top + self.metrics.separator_top,
                        width: (cell_width - separator.offset).max(0.0),
                        height: separator.thickness,
                    };
                }
                x += cell_width;
            }
        } else {
            let mut y = self.padding.top;
            for index in 0..self.trigger_count {
                let node =
                    std::mem::replace(&mut child_nodes[index], layout::Node::new(Size::ZERO))
                        .move_to(Point::new(self.padding.left, y));
                let item_height = node.size().height;
                trigger_bounds[index] = node.bounds();
                child_nodes[index] = node;
                if index + 1 < self.trigger_count {
                    let separator = &self.items[index].separator;
                    separator_bounds[index] = Rectangle {
                        x: self.padding.left + self.metrics.separator_left,
                        y: y + self.metrics.indicator_size,
                        width: separator.thickness,
                        height: item_height,
                    };
                }
                y += item_height + self.gap;
            }
        }

        for index in 0..self.trigger_count {
            let bounds = separator_bounds[index];
            child_nodes[self.trigger_count + index] = if bounds.width > 0.0 && bounds.height > 0.0 {
                self.children[self.trigger_count + index]
                    .as_widget_mut()
                    .layout(
                        &mut tree.children[self.trigger_count + index],
                        renderer,
                        &layout::Limits::new(
                            Size::new(bounds.width, bounds.height),
                            Size::new(bounds.width, bounds.height),
                        ),
                    )
                    .move_to(Point::new(bounds.x, bounds.y))
            } else {
                layout::Node::new(Size::ZERO)
            };
        }

        let state = tree.state.downcast_mut::<StepperNavState>();
        state.trigger_bounds = trigger_bounds;
        if state.active_index != (self.active_step.checked_sub(1)) {
            state.active_index = self.active_step.checked_sub(1);
            if !state.focused {
                state.focused_index = state.active_index;
            }
        }

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
            .children
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
        let navigation_enabled = !self.disabled && self.on_step_change.is_some();
        {
            let state = tree.state.downcast_mut::<StepperNavState>();
            if let Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(crate::iced_compat::touch::Event::FingerPressed { .. }) = event
            {
                if cursor.is_over(layout.bounds()) {
                    state.focused = true;
                    state.focus_visible = false;
                    if let Some(position) = cursor.position_in(layout.bounds()) {
                        state.focused_index = state
                            .trigger_bounds
                            .iter()
                            .enumerate()
                            .find(|(_, bounds)| {
                                position.x >= bounds.x
                                    && position.x <= bounds.x + bounds.width
                                    && position.y >= bounds.y
                                    && position.y <= bounds.y + bounds.height
                            })
                            .map(|(index, _)| index);
                    }
                } else {
                    state.focused = false;
                }
            }

            if navigation_enabled
                && state.focused
                && let Event::Keyboard(keyboard::Event::KeyPressed { key, .. }) = event
                && let Some(action) = nav_action(key, self.orientation)
                && let Some(next_index) = self.next_index(action, state.focused_index)
            {
                state.focused_index = Some(next_index);
                state.focus_visible = true;
                if let Some(callback) = self.on_step_change.as_ref() {
                    shell.publish(callback(next_index + 1));
                }
                shell.capture_event();
                return;
            }
        }

        let mut layouts = layout.children();
        for index in 0..self.trigger_count {
            let Some(child_layout) = layouts.next() else {
                break;
            };
            self.children[index].as_widget_mut().update(
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
        for ((child, child_tree), child_layout) in self
            .children
            .iter()
            .zip(&tree.children)
            .take(self.trigger_count)
            .zip(layout.children().take(self.trigger_count))
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
        mouse::Interaction::default()
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

        let mut nav_style = container::Style::default();
        if let Some(override_fn) = self.style_override.as_ref() {
            nav_style = override_fn(nav_style);
        }
        if let Some(background) = nav_style.background {
            renderer.fill_quad(
                renderer::Quad {
                    bounds,
                    border: nav_style.border,
                    shadow: nav_style.shadow,
                    ..renderer::Quad::default()
                },
                background,
            );
        }

        let state = tree.state.downcast_ref::<StepperNavState>();
        for index in 0..self.trigger_count.saturating_sub(1) {
            let separator = &self.items[index].separator;
            if separator.custom {
                continue;
            }
            let local = state
                .trigger_bounds
                .get(index)
                .map(|_| {
                    separator_bounds(
                        &state.trigger_bounds,
                        index,
                        self.orientation,
                        separator,
                        &self.metrics,
                    )
                })
                .unwrap_or_default();
            if local.width <= 0.0 || local.height <= 0.0 {
                continue;
            }
            let item_state = geometry::state_for_step(index + 1, self.active_step);
            let color = match item_state {
                StepperItemState::Completed => separator
                    .completed_color
                    .unwrap_or(self.theme.palette.primary),
                StepperItemState::Active | StepperItemState::Inactive => {
                    separator.color.unwrap_or(self.theme.palette.muted)
                }
            };
            renderer.fill_quad(
                renderer::Quad {
                    bounds: Rectangle {
                        x: bounds.x + local.x,
                        y: bounds.y + local.y,
                        ..local
                    },
                    ..renderer::Quad::default()
                },
                Background::Color(color),
            );
        }

        // Svelte's `ring-3` is an external box-shadow. Paint that layer here
        // so the indicator keeps its 28px layout footprint while the ring
        // masks the rail behind it.
        for index in 0..self.trigger_count {
            let Some(ring_width) = self.items[index].indicator_ring else {
                continue;
            };
            let Some(trigger) = state.trigger_bounds.get(index).copied() else {
                continue;
            };
            let indicator =
                indicator_bounds(trigger, self.orientation, self.items[index].indicator_size);
            let ring = Rectangle {
                x: bounds.x + indicator.x - ring_width,
                y: bounds.y + indicator.y - ring_width,
                width: indicator.width + ring_width * 2.0,
                height: indicator.height + ring_width * 2.0,
            };
            renderer.fill_quad(
                renderer::Quad {
                    bounds: ring,
                    border: Border {
                        radius: (indicator.width / 2.0 + ring_width).into(),
                        ..Border::default()
                    },
                    ..renderer::Quad::default()
                },
                Background::Color(self.items[index].indicator_ring_color),
            );
        }

        let mut child_layouts = layout.children();
        for index in 0..self.trigger_count {
            let Some(child_layout) = child_layouts.next() else {
                break;
            };
            self.children[index].as_widget().draw(
                &tree.children[index],
                renderer,
                iced_theme,
                &renderer::Style {
                    text_color: nav_style.text_color.unwrap_or(inherited_style.text_color),
                },
                child_layout,
                cursor,
                viewport,
            );
        }
        for index in 0..self.trigger_count {
            let Some(child_layout) = child_layouts.next() else {
                break;
            };
            if self.items[index].separator.custom {
                self.children[self.trigger_count + index].as_widget().draw(
                    &tree.children[self.trigger_count + index],
                    renderer,
                    iced_theme,
                    inherited_style,
                    child_layout,
                    cursor,
                    viewport,
                );
            }
        }

        if state.focused
            && state.focus_visible
            && let Some(index) = state.focused_index
            && let Some(trigger) = state.trigger_bounds.get(index)
        {
            let indicator =
                indicator_bounds(*trigger, self.orientation, self.items[index].indicator_size);
            let focus = Rectangle {
                x: bounds.x + indicator.x - 2.0,
                y: bounds.y + indicator.y - 2.0,
                width: indicator.width + 4.0,
                height: indicator.height + 4.0,
            };
            renderer.fill_quad(
                renderer::Quad {
                    bounds: focus,
                    border: Border {
                        color: self.theme.palette.ring.scale_alpha(0.50),
                        width: 2.0,
                        radius: (indicator.width / 2.0 + 2.0).into(),
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
            &mut self.children,
            tree,
            layout,
            renderer,
            viewport,
            translation,
        )
    }
}

fn indicator_bounds(
    trigger: Rectangle,
    orientation: StepperOrientation,
    indicator_size: f32,
) -> Rectangle {
    let indicator_size = indicator_size.max(1.0);
    match orientation {
        StepperOrientation::Horizontal => Rectangle {
            x: trigger.x + (trigger.width - indicator_size).max(0.0) / 2.0,
            y: trigger.y,
            width: indicator_size,
            height: indicator_size,
        },
        StepperOrientation::Vertical => Rectangle {
            x: trigger.x,
            y: trigger.y,
            width: indicator_size,
            height: indicator_size,
        },
    }
}

impl<'a, Message: Clone + 'a> StepperNavWidget<'a, Message> {
    fn next_index(&self, action: NavAction, current: Option<usize>) -> Option<usize> {
        match action {
            NavAction::Next => current.and_then(|index| {
                let next = index + 1;
                (next < self.items.len() && !self.items[next].disabled).then_some(next)
            }),
            NavAction::Previous => current.and_then(|index| {
                index
                    .checked_sub(1)
                    .filter(|next| !self.items[*next].disabled)
            }),
            NavAction::First => self.items.iter().position(|item| !item.disabled),
            NavAction::Last => self.items.iter().rposition(|item| !item.disabled),
            NavAction::Activate => None,
            _ => None,
        }
    }
}

fn separator_bounds(
    trigger_bounds: &[Rectangle],
    index: usize,
    orientation: StepperOrientation,
    separator: &SeparatorMeta,
    metrics: &StepperMetrics,
) -> Rectangle {
    let current = trigger_bounds.get(index).copied().unwrap_or_default();
    let next = trigger_bounds.get(index + 1).copied().unwrap_or_default();
    match orientation {
        StepperOrientation::Horizontal => Rectangle {
            x: current.x,
            y: current.y + metrics.separator_top,
            width: (next.x - current.x - separator.offset).max(0.0),
            height: separator.thickness,
        },
        StepperOrientation::Vertical => Rectangle {
            x: current.x + metrics.separator_left,
            y: current.y + metrics.indicator_size,
            width: separator.thickness,
            height: current.height,
        },
    }
}

fn nav_action(key: &keyboard::Key, orientation: StepperOrientation) -> Option<NavAction> {
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
