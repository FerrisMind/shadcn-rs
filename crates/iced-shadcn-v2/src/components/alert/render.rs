//! Alert layout and text rendering.

use crate::iced_compat::alignment::{Horizontal, Vertical};
use crate::iced_compat::widget::text::LineHeight;
use crate::iced_compat::widget::{Space, column, container, row, stack, text as iced_text};
use crate::iced_compat::{Background, Color, Element, Font, Length, Padding};

use super::geometry;
use super::style;
use super::types::AlertVariant;
use super::{Alert, AlertAction, AlertDescription, AlertItem, AlertTextContent, AlertTitle};
use crate::fonts::iced_font;
use crate::recipes::iced_font_weight;

pub(super) fn build_alert<'a, Message>(alert: Alert<'a, Message>) -> Element<'a, Message>
where
    Message: 'a,
{
    let Alert {
        theme,
        variant,
        radius,
        width,
        height,
        padding: custom_padding,
        spacing: custom_spacing,
        icon,
        items,
        action,
        style_override,
    } = alert;

    let metrics = geometry::metrics(theme);
    let spacing = custom_spacing.unwrap_or(metrics.gap_px);
    let padding = custom_padding.unwrap_or(Padding {
        top: metrics.padding_y_px,
        right: metrics.padding_x_px,
        bottom: metrics.padding_y_px,
        left: metrics.padding_x_px,
    });

    let mut body_children = Vec::with_capacity(items.len());
    for item in items {
        let child = match item {
            AlertItem::Element(element) => fill_child(element),
            AlertItem::Title(title) => build_title(title, variant),
            AlertItem::Description(description) => build_description(description, variant),
        };
        body_children.push(child);
    }

    let body: Element<'a, Message> = column(body_children)
        .spacing(spacing)
        .width(Length::Fill)
        .into();

    let content = if let Some(icon) = icon {
        let icon_color = style::foreground_color(theme, variant);
        let icon = container(icon)
            .width(Length::Fixed(metrics.icon_size_px))
            .height(Length::Fixed(metrics.icon_size_px))
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .style(move |_| container::Style {
                text_color: Some(icon_color),
                ..container::Style::default()
            });
        let icon = container(icon)
            .padding(Padding {
                top: metrics.icon_offset_y_px,
                right: 0.0,
                bottom: 0.0,
                left: 0.0,
            })
            .height(Length::Shrink);

        row![icon, body]
            .spacing(metrics.icon_gap_px)
            .align_y(Vertical::Top)
            .width(Length::Fill)
            .into()
    } else {
        body
    };

    let mut resolved = style::resolve_root_style(theme, variant, radius);
    if let Some(override_fn) = style_override.as_ref() {
        resolved = override_fn(resolved);
    }

    let right_padding = if action.is_some() {
        padding.right.max(72.0)
    } else {
        padding.right
    };
    let base = container(content)
        .padding(Padding {
            top: padding.top,
            right: right_padding,
            bottom: padding.bottom,
            left: padding.left,
        })
        .width(width)
        .height(height)
        .clip(true)
        .style(move |_| resolved);

    let accent_bar = style::accent_bar_color(theme, variant);
    if accent_bar.is_none() && action.is_none() {
        return base.into();
    }

    let mut layers = stack(vec![base.into()]).width(width).height(height);

    if let Some(color) = accent_bar {
        let rail = container(Space::new())
            .width(Length::Fixed(2.0))
            .height(Length::Fill)
            .style(move |_| container::Style {
                background: Some(Background::Color(color)),
                ..container::Style::default()
            });
        let rail_layer = container(row![rail, Space::new().width(Length::Fill)])
            .width(Length::Fill)
            .height(Length::Fill);
        layers = layers.push(rail_layer);
    }

    if let Some(action) = action {
        let action = build_action(action);
        let action_layer = container(action)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Horizontal::Right)
            .align_y(Vertical::Top)
            .padding(Padding {
                top: metrics.action_top_px,
                right: metrics.action_right_px,
                bottom: 0.0,
                left: 0.0,
            });
        layers = layers.push(action_layer);
    }

    layers.into()
}

pub(super) fn build_title<'a, Message>(
    title: AlertTitle<'a, Message>,
    variant: AlertVariant,
) -> Element<'a, Message>
where
    Message: 'a,
{
    let AlertTitle {
        content,
        theme,
        text_size,
        line_height,
        color,
        font,
        width,
        style_override,
    } = title;
    let metrics = geometry::metrics(theme);
    build_text(
        content,
        theme,
        TextOptions {
            size_px: text_size.unwrap_or(metrics.title_size_px),
            line_height_px: line_height.unwrap_or(metrics.title_line_height_px),
            color: color.unwrap_or_else(|| style::foreground_color(theme, variant)),
            custom_font: font,
            use_heading_font: true,
            weight: if metrics.title_is_semibold {
                shadcn_common::FontWeight::Semibold
            } else {
                shadcn_common::FontWeight::Medium
            },
            width,
            style_override,
        },
    )
}

pub(super) fn build_description<'a, Message>(
    description: AlertDescription<'a, Message>,
    variant: AlertVariant,
) -> Element<'a, Message>
where
    Message: 'a,
{
    let AlertDescription {
        content,
        theme,
        text_size,
        line_height,
        color,
        font,
        width,
        style_override,
    } = description;
    let metrics = geometry::metrics(theme);
    build_text(
        content,
        theme,
        TextOptions {
            size_px: text_size.unwrap_or(metrics.description_size_px),
            line_height_px: line_height.unwrap_or(metrics.description_line_height_px),
            color: color.unwrap_or_else(|| style::description_color(theme, variant)),
            custom_font: font,
            use_heading_font: false,
            weight: shadcn_common::FontWeight::Normal,
            width,
            style_override,
        },
    )
}

pub(super) fn build_action<'a, Message>(action: AlertAction<'a, Message>) -> Element<'a, Message>
where
    Message: 'a,
{
    let AlertAction {
        content,
        width,
        height,
        style_override,
    } = action;

    let mut resolved = container::Style::default();
    if let Some(override_fn) = style_override.as_ref() {
        resolved = override_fn(resolved);
    }

    container(content)
        .width(width)
        .height(height)
        .style(move |_| resolved)
        .into()
}

struct TextOptions<'a> {
    size_px: f32,
    line_height_px: f32,
    color: Color,
    custom_font: Option<Font>,
    use_heading_font: bool,
    weight: shadcn_common::FontWeight,
    width: Length,
    style_override: Option<Box<dyn Fn(container::Style) -> container::Style + 'a>>,
}

fn build_text<'a, Message>(
    content: AlertTextContent<'a, Message>,
    theme: &'a crate::theme::Theme,
    options: TextOptions<'a>,
) -> Element<'a, Message>
where
    Message: 'a,
{
    let TextOptions {
        size_px,
        line_height_px,
        color,
        custom_font,
        use_heading_font,
        weight,
        width,
        style_override,
    } = options;

    let mut font = custom_font.unwrap_or_else(|| {
        let face = if use_heading_font {
            theme.font_pack().heading
        } else {
            theme.font_pack().sans
        };
        iced_font(face)
    });
    if custom_font.is_none() {
        font.weight = iced_font_weight(weight);
    }

    let content: Element<'a, Message> = match content {
        AlertTextContent::Label(fragment) => iced_text(fragment)
            .size(size_px)
            .line_height(LineHeight::Absolute(line_height_px.into()))
            .font(font)
            .width(width)
            .into(),
        AlertTextContent::Element(element) => container(element).width(width).into(),
    };

    let mut resolved = container::Style {
        text_color: Some(color),
        ..container::Style::default()
    };
    if let Some(override_fn) = style_override.as_ref() {
        resolved = override_fn(resolved);
    }

    container(content)
        .width(width)
        .style(move |_| resolved)
        .into()
}

fn fill_child<'a, Message>(child: Element<'a, Message>) -> Element<'a, Message>
where
    Message: 'a,
{
    container(child).width(Length::Fill).into()
}
