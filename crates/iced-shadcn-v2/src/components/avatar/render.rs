//! Layout and rendering for avatar roots, slots, and groups.

use iced::alignment::{Horizontal, Vertical};
use iced::widget::image as image_widget;
use iced::widget::text::LineHeight;
use iced::widget::{Space, container, row, stack, text as iced_text};
use iced::{Element, Font, Length};

use super::geometry;
use super::style;
use super::types::AvatarSize;
use super::{
    Avatar, AvatarBadge, AvatarFallback, AvatarGroup, AvatarGroupCount, AvatarGroupItem,
    AvatarImage, AvatarTextContent,
};
use crate::fonts::iced_font;

pub(super) fn build_avatar<'a, Message>(avatar: Avatar<'a, Message>) -> Element<'a, Message>
where
    Message: 'a,
{
    let Avatar {
        theme,
        size,
        radius,
        width,
        height,
        image,
        fallback,
        badge,
        style_override,
    } = avatar;

    let nominal_size = size.pixels();
    let width = width.unwrap_or(Length::Fixed(nominal_size));
    let height = height.unwrap_or(Length::Fixed(nominal_size));
    let radius_px = geometry::radius_px(theme, radius);

    let mut layers: Vec<Element<'a, Message>> = Vec::with_capacity(3);

    if let Some(fallback) = fallback {
        layers.push(build_fallback(fallback, size, width, height, radius_px));
    }

    if let Some(image) = image {
        layers.push(build_image(image, width, height, radius_px));
    }

    if layers.is_empty() {
        layers.push(Space::new().into());
    }

    let content = stack(layers).width(width).height(height);

    if let Some(badge) = badge {
        let badge = build_badge(badge, size);
        layers_with_badge(content, badge, width, height)
    } else {
        let mut resolved = style::resolve_root_style(theme, radius);
        if let Some(override_fn) = style_override.as_ref() {
            resolved = override_fn(resolved);
        }

        container(content)
            .width(width)
            .height(height)
            .clip(true)
            .style(move |_| resolved)
            .into()
    }
}

fn layers_with_badge<'a, Message>(
    content: iced::widget::Stack<'a, Message>,
    badge: Element<'a, Message>,
    width: Length,
    height: Length,
) -> Element<'a, Message>
where
    Message: 'a,
{
    let badge_layer = container(badge)
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Horizontal::Right)
        .align_y(Vertical::Bottom);

    let content = content.push(badge_layer);
    container(content)
        .width(width)
        .height(height)
        .clip(true)
        .into()
}

fn build_image<'a, Message>(
    image: AvatarImage,
    width: Length,
    height: Length,
    radius_px: f32,
) -> Element<'a, Message>
where
    Message: 'a,
{
    let AvatarImage {
        handle,
        content_fit,
        filter_method,
        opacity,
        scale,
    } = image;

    image_widget::Image::new(handle)
        .width(width)
        .height(height)
        .content_fit(content_fit)
        .filter_method(filter_method)
        .opacity(opacity)
        .scale(scale)
        .border_radius(iced::border::Radius::from(radius_px))
        .into()
}

pub(super) fn build_fallback<'a, Message>(
    fallback: AvatarFallback<'a, Message>,
    size: AvatarSize,
    width: Length,
    height: Length,
    radius_px: f32,
) -> Element<'a, Message>
where
    Message: 'a,
{
    let AvatarFallback {
        content,
        theme,
        text_size,
        line_height,
        color,
        background,
        font,
        style_override,
    } = fallback;

    let metrics = geometry::fallback_metrics(size);
    let content = build_text_content(
        content,
        theme,
        text_size.unwrap_or(metrics.size_px),
        line_height.unwrap_or(metrics.line_height_px),
        color.unwrap_or(theme.palette.muted_foreground),
        font,
    );

    let mut resolved = style::resolve_fallback_style(theme, radius_px);
    if let Some(background) = background {
        resolved.background = Some(iced::Background::Color(background));
    }
    if let Some(color) = color {
        resolved.text_color = Some(color);
    }
    if let Some(override_fn) = style_override.as_ref() {
        resolved = override_fn(resolved);
    }

    container(content)
        .width(width)
        .height(height)
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
        .clip(true)
        .style(move |_| resolved)
        .into()
}

fn build_text_content<'a, Message>(
    content: AvatarTextContent<'a, Message>,
    theme: &'a crate::theme::Theme,
    text_size: f32,
    line_height: f32,
    color: iced::Color,
    custom_font: Option<Font>,
) -> Element<'a, Message>
where
    Message: 'a,
{
    let font = custom_font.unwrap_or_else(|| iced_font(theme.font_pack().sans));

    match content {
        AvatarTextContent::Label(fragment) => iced_text(fragment)
            .size(text_size)
            .line_height(LineHeight::Absolute(line_height.into()))
            .font(font)
            .color(color)
            .into(),
        AvatarTextContent::Element(element) => element,
    }
}

pub(super) fn build_badge<'a, Message>(
    badge: AvatarBadge<'a, Message>,
    avatar_size: AvatarSize,
) -> Element<'a, Message>
where
    Message: 'a,
{
    let AvatarBadge {
        content,
        theme,
        width,
        height,
        style_override,
    } = badge;

    let badge_size = geometry::badge_size(avatar_size);
    let content = content.unwrap_or_else(|| Space::new().into());
    let mut resolved = style::resolve_badge_style(theme, avatar_size);

    if let Some(override_fn) = style_override.as_ref() {
        resolved = override_fn(resolved);
    }

    container(content)
        .width(width.unwrap_or(Length::Fixed(badge_size)))
        .height(height.unwrap_or(Length::Fixed(badge_size)))
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
        .clip(true)
        .style(move |_| resolved)
        .into()
}

pub(super) fn build_group_count<'a, Message>(
    count: AvatarGroupCount<'a, Message>,
    group_size: AvatarSize,
) -> Element<'a, Message>
where
    Message: 'a,
{
    let AvatarGroupCount {
        content,
        theme,
        width,
        height,
        style_override,
    } = count;

    let size_px = group_size.pixels();
    let metrics = geometry::group_count_metrics(theme, group_size);
    let mut resolved = style::resolve_group_count_style(theme, size_px);
    if let Some(override_fn) = style_override.as_ref() {
        resolved = override_fn(resolved);
    }

    let content = container(
        iced_text("")
            .size(metrics.size_px)
            .line_height(LineHeight::Absolute(metrics.line_height_px.into()))
            .font(iced_font(theme.font_pack().sans))
            .color(theme.palette.muted_foreground),
    )
    .width(Length::Shrink)
    .height(Length::Shrink)
    .align_x(Horizontal::Center)
    .align_y(Vertical::Center);

    let content = container(content)
        .width(width.unwrap_or(Length::Fixed(size_px)))
        .height(height.unwrap_or(Length::Fixed(size_px)))
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
        .style(move |_| resolved);

    // The source count accepts arbitrary children. The outer style provides
    // the semantic surface; the supplied child remains responsible for its
    // own typography and icon painting.
    let _ = content;
    container(content)
        .width(width.unwrap_or(Length::Fixed(size_px)))
        .height(height.unwrap_or(Length::Fixed(size_px)))
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
        .style(move |_| resolved)
        .into()
}

pub(super) fn build_group<'a, Message>(
    group: AvatarGroup<'a, Message>,
) -> Element<'a, Message>
where
    Message: 'a,
{
    let AvatarGroup {
        theme,
        items,
        overlap,
        style_override,
    } = group;

    let group_size = items
        .iter()
        .filter_map(|item| match item {
            AvatarGroupItem::Avatar(avatar) => Some(avatar.nominal_size()),
            AvatarGroupItem::Element { size, .. } => Some(size.pixels()),
            AvatarGroupItem::Count(_) => None,
        })
        .max_by(f32::total_cmp)
        .unwrap_or(AvatarSize::Default.pixels());

    let group_size = AvatarSize::Custom(group_size);
    let mut children = Vec::with_capacity(items.len());

    for item in items {
        match item {
            AvatarGroupItem::Avatar(avatar) => {
                let size = avatar.nominal_size();
                children.push(group_slot(
                    avatar.into_group_element(),
                    size,
                    overlap,
                    theme,
                ));
            }
            AvatarGroupItem::Element { element, size } => {
                children.push(group_slot(element, size.pixels(), overlap, theme));
            }
            AvatarGroupItem::Count(count) => {
                children.push(group_slot(
                    build_group_count(count, group_size),
                    group_size.pixels(),
                    overlap,
                    theme,
                ));
            }
        }
    }

    let content = row(children)
        .spacing(0.0)
        .height(Length::Fixed(group_size.pixels()))
        .align_y(Vertical::Center);

    let mut resolved = style::resolve_group_style(theme);
    if let Some(override_fn) = style_override.as_ref() {
        resolved = override_fn(resolved);
    }

    container(content)
        .width(Length::Shrink)
        .height(Length::Fixed(group_size.pixels()))
        .style(move |_| resolved)
        .into()
}

fn group_slot<'a, Message>(
    child: Element<'a, Message>,
    size: f32,
    overlap: f32,
    theme: &'a crate::theme::Theme,
) -> Element<'a, Message>
where
    Message: 'a,
{
    let ring = container(child)
        .width(Length::Fixed(size))
        .height(Length::Fixed(size))
        .style(move |_| style::resolve_group_ring_style(theme, size));

    container(ring)
        .width(Length::Fixed((size - overlap).max(1.0)))
        .height(Length::Fixed(size))
        .clip(false)
        .into()
}
