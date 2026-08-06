//! Accordion composition, trigger painting, and content delegation.

use std::f32::consts::PI;
use std::rc::Rc;

use crate::components::button::Button;
use crate::components::collapsible::{Collapsible, CollapsibleContent};
use crate::fonts::iced_font;
use crate::iced_compat::widget::canvas::{self, LineCap, LineJoin, Path, Stroke};
use crate::iced_compat::widget::text::{Fragment, LineHeight, Rich, Span};
use crate::iced_compat::widget::{Space, column, container, hover, row, text as iced_text};
use crate::iced_compat::{
    Background, Color, Element, Length, Point, Rectangle, Vector, alignment, mouse,
};
use crate::recipes::iced_font_weight;

use super::geometry;
use super::style;
use super::types::{
    Accordion, AccordionContent, AccordionItem, AccordionTrigger, AccordionTriggerContent,
    AccordionType, AccordionValue,
};

struct RenderContext<'value, 'callback, Message> {
    accordion_type: AccordionType,
    value: &'value AccordionValue,
    root_disabled: bool,
    animated: bool,
    duration: std::time::Duration,
    on_value_change: Option<Rc<dyn Fn(AccordionValue) -> Message + 'callback>>,
    on_press: Option<Message>,
}

/// Builds a complete accordion from its controlled builder state.
pub(super) fn build_accordion<'a, Message>(
    accordion: Accordion<'a, Message>,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let Accordion {
        theme,
        items,
        accordion_type,
        value,
        orientation: _orientation,
        loop_navigation: _loop_navigation,
        spacing,
        disabled,
        animated,
        duration,
        width,
        height,
        padding,
        background,
        bordered,
        radius,
        on_value_change,
        on_press,
        style_override,
    } = accordion;

    let value = value.for_type(accordion_type);
    let item_count = items.len();
    let mut children = Vec::with_capacity(item_count);
    let context = RenderContext {
        accordion_type,
        value: &value,
        root_disabled: disabled,
        animated,
        duration,
        on_value_change,
        on_press,
    };

    for (index, item) in items.into_iter().enumerate() {
        children.push(build_item(item, index, index + 1 < item_count, &context));
    }

    let body: Element<'a, Message> = column(children).spacing(spacing).width(Length::Fill).into();

    let mut resolved = style::resolve_surface(
        theme,
        style::Surface {
            background,
            bordered: bordered.unwrap_or_else(|| geometry::default_root_bordered(theme)),
            radius: Some(radius.unwrap_or_else(|| geometry::default_root_radius(theme))),
        },
    );

    if let Some(style_override) = style_override.as_ref() {
        resolved = style_override(resolved);
    }

    container(body)
        .width(width)
        .height(height)
        .padding(padding.unwrap_or_default())
        .style(move |_| resolved)
        .into()
}

fn build_item<'a, Message>(
    item: AccordionItem<'a, Message>,
    index: usize,
    show_divider: bool,
    context: &RenderContext<'_, 'a, Message>,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let AccordionItem {
        theme: item_theme,
        value: item_value,
        trigger,
        content,
        disabled: item_disabled,
        padding,
        background,
        bordered,
        radius,
        style_override,
    } = item;

    let item_value = item_value.unwrap_or_else(|| format!("item-{}", index + 1));
    let open = context.value.is_open(&item_value);
    let disabled = context.root_disabled || item_disabled;
    let next_value = style::next_value(context.value, context.accordion_type, &item_value);

    let trigger = trigger.map(|trigger| {
        let trigger_message = if disabled {
            None
        } else {
            trigger.on_press.clone().or_else(|| {
                context
                    .on_value_change
                    .as_ref()
                    .map(|callback| callback(next_value.clone()))
                    .or_else(|| context.on_press.clone())
            })
        };

        build_trigger(trigger, open, disabled, trigger_message)
    });

    let mut root = Collapsible::new(item_theme)
        .open(open)
        .spacing(0.0)
        .width(Length::Fill)
        .animated(context.animated)
        .duration(context.duration);

    if let Some(trigger) = trigger {
        root = root.push(trigger);
    }

    if let Some(content) = content {
        root = root.content(build_content(content));
    }

    let item_body: Element<'a, Message> = if show_divider {
        column![root, divider(item_theme)]
            .spacing(0.0)
            .width(Length::Fill)
            .into()
    } else {
        root.into()
    };

    let mut resolved = style::resolve_item_surface(item_theme, background, bordered, radius, open);
    if let Some(style_override) = style_override.as_ref() {
        resolved = style_override(resolved);
    }

    container(item_body)
        .width(Length::Fill)
        .padding(padding.unwrap_or_default())
        .style(move |_| resolved)
        .into()
}

pub(super) fn build_trigger<'a, Message>(
    trigger: AccordionTrigger<'a, Message>,
    open: bool,
    root_disabled: bool,
    message: Option<Message>,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let AccordionTrigger {
        theme,
        content,
        variant,
        size,
        radius,
        color,
        width,
        height,
        full_width,
        disabled: trigger_disabled,
        level: _level,
        padding,
        gap,
        on_press: _on_press,
        style_override,
    } = trigger;

    let disabled = root_disabled || trigger_disabled;
    let label = trigger_content(content, theme, size);
    let indicator = indicator_canvas(Indicator {
        open,
        color: style::indicator_color(theme, disabled),
        size: geometry::trigger_icon_size_px(theme, size),
    });

    let gap = gap.unwrap_or_else(|| geometry::trigger_gap_px(theme, size));
    let trigger_content: Element<'a, Message> =
        row![container(label).width(Length::Fill), indicator]
            .spacing(gap)
            .align_y(alignment::Vertical::Center)
            .into();

    let mut button = Button::new(trigger_content, theme)
        .variant(variant)
        .size(size)
        .width(width)
        .height(height)
        .disabled(disabled);

    if full_width {
        button = button.full_width();
    }

    button = button.radius(radius.unwrap_or_else(|| geometry::default_trigger_radius(theme)));

    if let Some(color) = color {
        button = button.color(color);
    }

    if let Some(message) = message {
        button = button.on_press(message);
    }

    let user_style = style_override;
    button = button.style_override(move |mut resolved, status| {
        if variant == crate::components::button::ButtonVariant::Ghost {
            style::normalize_ghost_trigger_style(theme, &mut resolved, status, disabled);
        }

        if let Some(user_style) = user_style.as_ref() {
            resolved = user_style(resolved, status);
        }

        resolved
    });

    let padding = padding.unwrap_or_else(|| geometry::default_trigger_padding(theme));
    // `AccordionTrigger::padding` validates the same twill value before it is
    // stored. The default is made exclusively from fixed scale values, so an
    // error here would indicate a bug in this module rather than user input.
    button = button
        .padding(padding)
        .expect("accordion trigger padding was validated before rendering");

    button.into()
}

fn trigger_content<'a, Message: 'a>(
    content: AccordionTriggerContent<'a, Message>,
    theme: &crate::theme::Theme,
    size: crate::components::button::ButtonSize,
) -> Element<'a, Message> {
    match content {
        AccordionTriggerContent::Label(label) => trigger_label(label, theme, size),
        AccordionTriggerContent::Element(element) | AccordionTriggerContent::Icon(element) => {
            element
        }
    }
}

fn trigger_label<'a, Message: 'a>(
    label: Fragment<'a>,
    theme: &crate::theme::Theme,
    size: crate::components::button::ButtonSize,
) -> Element<'a, Message> {
    let size_px = geometry::trigger_text_size_px(theme, size);
    let weight = geometry::trigger_weight(theme);
    let mut font = iced_font(theme.font_pack().sans);
    font.weight = iced_font_weight(weight);
    let line_height = LineHeight::Absolute(
        geometry::trigger_line_height_px(theme, size)
            .max(size_px)
            .into(),
    );
    let text = label.into_owned();

    let base = Rich::<(), Message>::with_spans(vec![Span::new(text.clone())])
        .size(size_px)
        .font(font)
        .line_height(line_height);
    let underlined = Rich::<(), Message>::with_spans(vec![Span::new(text).underline(true)])
        .size(size_px)
        .font(font)
        .line_height(line_height);

    container(hover(base, underlined))
        .width(Length::Fill)
        .height(Length::Shrink)
        .into()
}

pub(super) fn build_content<'a, Message>(
    content: AccordionContent<'a, Message>,
) -> CollapsibleContent<'a, Message>
where
    Message: 'a,
{
    let AccordionContent {
        theme,
        children,
        spacing,
        padding,
        width,
        height,
        background,
        bordered,
        radius,
        force_mount,
        hidden_until_found: _hidden_until_found,
        style_override,
    } = content;

    let body: Element<'a, Message> = column(children).spacing(spacing).width(Length::Fill).into();

    let content_padding = padding.unwrap_or_else(|| geometry::default_content_padding(theme));

    let mut resolved = style::resolve_surface(
        theme,
        style::Surface {
            background,
            bordered,
            radius,
        },
    );
    if let Some(style_override) = style_override.as_ref() {
        resolved = style_override(resolved);
    }

    let inner = container(body)
        .width(Length::Fill)
        .padding(content_padding)
        .style(move |_| resolved);

    CollapsibleContent::new(theme)
        .width(width)
        .height(height)
        .force_mount(force_mount)
        .push(inner)
}

pub(super) fn paragraph_text<'a, Message: 'a>(
    content: Fragment<'a>,
    theme: &crate::theme::Theme,
) -> Element<'a, Message> {
    let (size, line_height) = geometry::content_text_metrics(theme);
    let font = iced_font(theme.font_pack().sans);
    iced_text(content)
        .size(size)
        .font(font)
        .line_height(LineHeight::Absolute(line_height.into()))
        .color(theme.palette.foreground)
        .width(Length::Fill)
        .into()
}

fn divider<'a, Message: 'a>(theme: &crate::theme::Theme) -> Element<'a, Message> {
    let color = theme.semantic_color(twill_core::prelude::theme::SemanticColor::Border);
    container(Space::new())
        .width(Length::Fill)
        .height(Length::Fixed(1.0))
        .style(move |_| container::Style {
            background: Some(Background::Color(color)),
            ..container::Style::default()
        })
        .into()
}

/// Static down/up chevron painted as a canvas glyph.
#[derive(Debug, Clone, Copy)]
struct Indicator {
    open: bool,
    color: Color,
    size: f32,
}

fn indicator_canvas<'a, Message: 'a>(indicator: Indicator) -> Element<'a, Message> {
    canvas::Canvas::new(indicator)
        .width(Length::Fixed(indicator.size))
        .height(Length::Fixed(indicator.size))
        .into()
}

impl<Message> canvas::Program<Message> for Indicator {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &crate::iced_compat::Renderer,
        _theme: &crate::iced_compat::Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let size = bounds.size();
        if !size.width.is_finite()
            || !size.height.is_finite()
            || size.width <= 0.0
            || size.height <= 0.0
        {
            return Vec::new();
        }

        let mut frame = canvas::Frame::new(renderer, size);
        let center = frame.center();
        let extent = size.width.min(size.height);
        let reach = extent * 0.25;
        let arm = extent * 0.125;
        let stroke_width = (extent * 0.10).clamp(1.0, 1.75);
        let chevron = Path::new(|builder| {
            builder.move_to(Point::new(-reach, -arm));
            builder.line_to(Point::new(0.0, arm));
            builder.line_to(Point::new(reach, -arm));
        });

        frame.with_save(|frame| {
            frame.translate(Vector::new(center.x, center.y));
            if self.open {
                frame.rotate(PI);
            }
            frame.stroke(
                &chevron,
                Stroke::default()
                    .with_width(stroke_width)
                    .with_color(self.color)
                    .with_line_cap(LineCap::Round)
                    .with_line_join(LineJoin::Round),
            );
        });

        vec![frame.into_geometry()]
    }
}
