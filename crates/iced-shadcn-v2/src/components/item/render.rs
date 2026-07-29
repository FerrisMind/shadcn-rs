//! Layout and text rendering for the item component.

use crate::iced_compat::alignment::Vertical;
use crate::iced_compat::widget::text::LineHeight;
use crate::iced_compat::widget::{
    Space, button as iced_button, column, container, row, text as iced_text,
};
use crate::iced_compat::{Color, Element, Font, Length, Padding};

use super::geometry;
use super::style;
use super::types::{ItemMediaVariant, ItemSize};
use super::{
    Item, ItemActions, ItemContent, ItemDescription, ItemGroup, ItemGroupChild, ItemMedia,
    ItemRowChild, ItemSeparator, ItemTextContent, ItemTitle,
};
use crate::fonts::iced_font;
use crate::recipes::iced_font_weight;

pub(super) fn build_root<'a, Message>(item: Item<'a, Message>) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let Item {
        theme,
        variant,
        size,
        radius,
        width,
        spacing,
        padding_x,
        padding_y,
        children,
        on_press,
        style_override,
    } = item;

    let metrics = geometry::size_metrics(theme, size);
    let gap = spacing.unwrap_or(metrics.gap);
    let has_description = children
        .iter()
        .any(|child| matches!(child, ItemRowChild::Content(content) if content.has_description()));

    let mut header = None;
    let mut footer = None;
    let mut content_seen = false;
    let mut main_children = Vec::with_capacity(children.len());
    for child in children {
        match child {
            ItemRowChild::Media(media) => {
                main_children.push(build_media(*media, size, has_description));
            }
            ItemRowChild::Content(content) => {
                let fill = !content_seen;
                content_seen = true;
                main_children.push(build_content(*content, size, fill));
            }
            ItemRowChild::Actions(actions) => main_children.push(build_actions(*actions)),
            ItemRowChild::Header(section) => {
                header = Some(build_section(section.children, section.spacing));
            }
            ItemRowChild::Footer(section) => {
                footer = Some(build_section(section.children, section.spacing));
            }
            ItemRowChild::Element(element) => main_children.push(element),
        }
    }

    let main: Element<'a, Message> = row(main_children)
        .spacing(gap)
        .align_y(if has_description {
            Vertical::Top
        } else {
            Vertical::Center
        })
        .width(Length::Fill)
        .into();

    let mut rows = Vec::with_capacity(3);
    if let Some(header) = header {
        rows.push(header);
    }
    rows.push(main);
    if let Some(footer) = footer {
        rows.push(footer);
    }

    let body = column(rows).spacing(gap).width(Length::Fill);
    let padding = Padding {
        top: padding_y.unwrap_or(metrics.padding_y),
        right: padding_x.unwrap_or(metrics.padding_x),
        bottom: padding_y.unwrap_or(metrics.padding_y),
        left: padding_x.unwrap_or(metrics.padding_x),
    };

    let mut resting = style::resolve_root_style(theme, variant, radius);
    if let Some(override_fn) = style_override.as_ref() {
        resting = override_fn(resting);
    }

    if let Some(message) = on_press {
        let mut hovered = style::resolve_hover_style(theme, variant, radius);
        if let Some(override_fn) = style_override.as_ref() {
            hovered = override_fn(hovered);
        }
        let fallback_text = theme.palette.foreground;

        iced_button(body)
            .padding(padding)
            .width(width)
            .on_press(message)
            .style(move |_iced_theme, status| {
                let resolved = match status {
                    iced_button::Status::Hovered | iced_button::Status::Pressed => hovered,
                    iced_button::Status::Active | iced_button::Status::Disabled => resting,
                };
                style::to_button_style(resolved, fallback_text)
            })
            .into()
    } else {
        container(body)
            .padding(padding)
            .width(width)
            .style(move |_iced_theme| resting)
            .into()
    }
}

pub(super) fn build_media<'a, Message>(
    media: ItemMedia<'a, Message>,
    item_size: ItemSize,
    beside_description: bool,
) -> Element<'a, Message>
where
    Message: 'a,
{
    let ItemMedia {
        theme,
        variant,
        children,
        spacing,
        image_size,
        image_radius,
        style_override,
    } = media;

    let body: Element<'a, Message> = row(children)
        .spacing(spacing.unwrap_or_else(geometry::media_gap))
        .align_y(Vertical::Center)
        .into();

    let mut resolved = container::Style::default();
    let wrapper = if variant == ItemMediaVariant::Image {
        let edge = image_size.unwrap_or_else(|| geometry::media_image_size_px(theme, item_size));
        let radius =
            image_radius.unwrap_or_else(|| geometry::media_image_radius_px(theme, item_size));
        resolved.border.radius = radius.into();
        resolved.snap = true;
        container(body).center(Length::Fixed(edge)).clip(true)
    } else {
        container(body)
    };

    if let Some(override_fn) = style_override.as_ref() {
        resolved = override_fn(resolved);
    }

    let media: Element<'a, Message> = wrapper.style(move |_iced_theme| resolved).into();

    if beside_description {
        container(media)
            .padding(Padding {
                top: geometry::media_description_offset(),
                right: 0.0,
                bottom: 0.0,
                left: 0.0,
            })
            .into()
    } else {
        media
    }
}

pub(super) fn build_content<'a, Message>(
    content: ItemContent<'a, Message>,
    size: ItemSize,
    fill: bool,
) -> Element<'a, Message>
where
    Message: 'a,
{
    let ItemContent {
        theme,
        title,
        title_element,
        description,
        description_element,
        children,
        spacing,
        style_override,
    } = content;

    let mut rows = Vec::with_capacity(
        usize::from(title.is_some())
            + usize::from(title_element.is_some())
            + usize::from(description.is_some())
            + usize::from(description_element.is_some())
            + children.len(),
    );
    if let Some(title) = title {
        rows.push(build_title(title));
    }
    if let Some(title) = title_element {
        rows.push(title);
    }
    if let Some(description) = description {
        rows.push(build_description(description, size));
    }
    if let Some(description) = description_element {
        rows.push(description);
    }
    rows.extend(children);

    let width = if fill { Length::Fill } else { Length::Shrink };
    let body = column(rows)
        .spacing(spacing.unwrap_or_else(|| geometry::content_gap(theme, size)))
        .width(width);

    let mut resolved = container::Style::default();
    if let Some(override_fn) = style_override.as_ref() {
        resolved = override_fn(resolved);
    }

    container(body)
        .width(width)
        .style(move |_iced_theme| resolved)
        .into()
}

pub(super) fn build_actions<'a, Message>(actions: ItemActions<'a, Message>) -> Element<'a, Message>
where
    Message: 'a,
{
    let ItemActions {
        children, spacing, ..
    } = actions;

    row(children)
        .spacing(spacing.unwrap_or_else(geometry::section_gap))
        .align_y(Vertical::Center)
        .into()
}

/// Full-width header/footer row with space-between distribution.
pub(super) fn build_section<'a, Message>(
    children: Vec<Element<'a, Message>>,
    spacing: Option<f32>,
) -> Element<'a, Message>
where
    Message: 'a,
{
    let last = children.len().saturating_sub(1);
    let mut distributed = Vec::with_capacity(children.len().saturating_mul(2));
    for (index, child) in children.into_iter().enumerate() {
        distributed.push(child);
        if index < last {
            distributed.push(Space::new().width(Length::Fill).into());
        }
    }

    row(distributed)
        .spacing(spacing.unwrap_or_else(geometry::section_gap))
        .align_y(Vertical::Center)
        .width(Length::Fill)
        .into()
}

pub(super) fn build_title<'a, Message>(title: ItemTitle<'a, Message>) -> Element<'a, Message>
where
    Message: 'a,
{
    let ItemTitle {
        content,
        theme,
        text_size,
        line_height,
        color,
        font,
        width,
    } = title;
    let metrics = geometry::title_metrics(theme);
    build_text(
        content,
        TextOptions {
            size_px: text_size.unwrap_or(metrics.size_px),
            line_height_px: line_height.unwrap_or(metrics.line_height_px),
            color: color.unwrap_or(theme.palette.foreground),
            font: font.unwrap_or_else(|| {
                let mut font = iced_font(theme.font_pack().heading);
                font.weight = iced_font_weight(metrics.weight);
                font
            }),
            uppercase: metrics.uppercase,
            width,
        },
    )
}

pub(super) fn build_description<'a, Message>(
    description: ItemDescription<'a, Message>,
    size: ItemSize,
) -> Element<'a, Message>
where
    Message: 'a,
{
    let ItemDescription {
        content,
        theme,
        text_size,
        line_height,
        color,
        font,
        width,
    } = description;
    let metrics = geometry::description_metrics(theme, size);
    build_text(
        content,
        TextOptions {
            size_px: text_size.unwrap_or(metrics.size_px),
            line_height_px: line_height.unwrap_or(metrics.line_height_px),
            color: color.unwrap_or(theme.palette.muted_foreground),
            font: font.unwrap_or_else(|| {
                let mut font = iced_font(theme.font_pack().sans);
                font.weight = iced_font_weight(metrics.weight);
                font
            }),
            uppercase: metrics.uppercase,
            width,
        },
    )
}

pub(super) fn build_group<'a, Message>(group: ItemGroup<'a, Message>) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let ItemGroup {
        children,
        spacing,
        width,
        has_sm,
        has_xs,
        ..
    } = group;

    let gap = spacing.unwrap_or_else(|| geometry::group_gap(has_sm, has_xs));
    let children = children.into_iter().map(|child| match child {
        ItemGroupChild::Item(item) => build_root(*item),
        ItemGroupChild::Element(element) => element,
    });

    column(children).spacing(gap).width(width).into()
}

pub(super) fn build_separator<'a, Message>(separator: ItemSeparator) -> Element<'a, Message>
where
    Message: 'a,
{
    let margin = separator.margin_y;
    container(Element::from(separator.separator))
        .padding(Padding {
            top: margin,
            right: 0.0,
            bottom: margin,
            left: 0.0,
        })
        .width(Length::Fill)
        .into()
}

struct TextOptions {
    size_px: f32,
    line_height_px: f32,
    color: Color,
    font: Font,
    uppercase: bool,
    width: Length,
}

fn build_text<'a, Message>(
    content: ItemTextContent<'a, Message>,
    options: TextOptions,
) -> Element<'a, Message>
where
    Message: 'a,
{
    let TextOptions {
        size_px,
        line_height_px,
        color,
        font,
        uppercase,
        width,
    } = options;

    let content: Element<'a, Message> = match content {
        ItemTextContent::Label(fragment) => {
            let text = if uppercase {
                iced_text(fragment.as_ref().to_uppercase())
            } else {
                iced_text(fragment)
            };
            text.size(size_px)
                .line_height(LineHeight::Absolute(line_height_px.into()))
                .font(font)
                .width(width)
                .into()
        }
        ItemTextContent::Element(element) => container(element).width(width).into(),
    };

    container(content)
        .width(width)
        .style(move |_iced_theme| container::Style {
            text_color: Some(color),
            ..container::Style::default()
        })
        .into()
}
