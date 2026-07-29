//! Layout and rendering for avatar roots, slots, and groups.

use iced::advanced::layout::{self, Layout};
use iced::advanced::widget::{Operation, Tree};
use iced::advanced::{Clipboard, Widget, overlay, renderer};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::image as image_widget;
use iced::widget::text::LineHeight;
use iced::widget::{Space, container, row, stack, text as iced_text};
use iced::{Element, Event, Font, Length, Point, Rectangle, Size, Vector, mouse};

use super::geometry;
use super::style;
use super::types::{AvatarRadius, AvatarSize};
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

    if let Some(badge) = badge {
        let badge_layer = container(build_badge(badge, size))
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Horizontal::Right)
            .align_y(Vertical::Bottom);
        layers.push(badge_layer.into());
    }

    let content = stack(layers).width(width).height(height);
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

    let content = build_text_content(
        content,
        theme,
        metrics.size_px,
        metrics.line_height_px,
        theme.palette.muted_foreground,
        None,
    );

    let content = container(content)
        .width(width.unwrap_or(Length::Fixed(size_px)))
        .height(height.unwrap_or(Length::Fixed(size_px)))
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
        .style(move |_| resolved);

    count_slot(content.into(), size_px, theme)
}

pub(super) fn build_group<'a, Message>(group: AvatarGroup<'a, Message>) -> Element<'a, Message>
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
                let radius = avatar.nominal_radius();
                children.push(group_slot(
                    (*avatar).into_group_element(),
                    size,
                    radius,
                    theme,
                ));
            }
            AvatarGroupItem::Element {
                element,
                size,
                radius,
            } => {
                children.push(group_slot(element, size.pixels(), radius, theme));
            }
            AvatarGroupItem::Count(count) => {
                children.push(build_group_count(count, group_size));
            }
        }
    }

    let content = row(children)
        // CSS `-space-x-2` keeps every child at its nominal footprint and
        // moves the next child left by the overlap amount. Negative row
        // spacing models that geometry directly; shrinking an outer slot and
        // relying on child overflow does not preserve the ring's layout box.
        .spacing(-overlap)
        .height(Length::Fixed(group_size.pixels()));

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
    radius: AvatarRadius,
    theme: &'a crate::theme::Theme,
) -> Element<'a, Message>
where
    Message: 'a,
{
    ring_slot(child, size, radius, theme)
}

fn ring_slot<'a, Message>(
    child: Element<'a, Message>,
    size: f32,
    radius: AvatarRadius,
    theme: &'a crate::theme::Theme,
) -> Element<'a, Message>
where
    Message: 'a,
{
    let ring_size = size + 4.0;
    let ring_radius = geometry::radius_px(theme, radius) + 2.0;
    let content: Element<'a, Message> = container(child)
        .width(Length::Fixed(ring_size))
        .height(Length::Fixed(ring_size))
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
        .clip(false)
        .into();

    let overlay = container(Space::new())
        .width(Length::Fixed(ring_size))
        .height(Length::Fixed(ring_size))
        .style(move |_| style::resolve_group_ring_style(theme, ring_radius));

    let ring = stack![content, overlay]
        .width(Length::Fixed(ring_size))
        .height(Length::Fixed(ring_size))
        .clip(false);

    overflow_slot(ring.into(), size)
}

fn count_slot<'a, Message>(
    content: Element<'a, Message>,
    size: f32,
    theme: &'a crate::theme::Theme,
) -> Element<'a, Message>
where
    Message: 'a,
{
    let ring_size = size + 4.0;
    let base = container(Space::new())
        .width(Length::Fixed(ring_size))
        .height(Length::Fixed(ring_size));
    let surface: Element<'a, Message> = container(content)
        .width(Length::Fixed(ring_size))
        .height(Length::Fixed(ring_size))
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
        .clip(false)
        .into();
    let overlay = container(Space::new())
        .width(Length::Fixed(ring_size))
        .height(Length::Fixed(ring_size))
        .style(move |_| style::resolve_group_ring_style(theme, ring_size));

    let ring = stack![base, surface, overlay]
        .width(Length::Fixed(ring_size))
        .height(Length::Fixed(ring_size))
        .clip(false);

    overflow_slot(ring.into(), size)
}

fn overflow_slot<'a, Message>(content: Element<'a, Message>, size: f32) -> Element<'a, Message>
where
    Message: 'a,
{
    Element::new(OverflowSlot { content, size })
}

struct OverflowSlot<'a, Message> {
    content: Element<'a, Message>,
    size: f32,
}

impl<Message> Widget<Message, iced::Theme, iced::Renderer> for OverflowSlot<'_, Message> {
    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.content));
    }

    fn size(&self) -> Size<Length> {
        Size::new(Length::Fixed(self.size), Length::Fixed(self.size))
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let size = limits.resolve(
            Length::Fixed(self.size),
            Length::Fixed(self.size),
            Size::ZERO,
        );
        let child_size = Size::new(size.width + 4.0, size.height + 4.0);
        let child_limits = layout::Limits::new(Size::ZERO, child_size);
        let child =
            self.content
                .as_widget_mut()
                .layout(&mut tree.children[0], renderer, &child_limits);
        let offset = Point::new(
            (size.width - child.size().width) / 2.0,
            (size.height - child.size().height) / 2.0,
        );

        layout::Node::with_children(size, vec![child.move_to(offset)])
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &iced::Renderer,
        operation: &mut dyn Operation,
    ) {
        self.content.as_widget_mut().operate(
            &mut tree.children[0],
            layout
                .children()
                .next()
                .expect("avatar group slot child layout"),
            renderer,
            operation,
        );
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &iced::Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut iced::advanced::Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        self.content.as_widget_mut().update(
            &mut tree.children[0],
            event,
            layout
                .children()
                .next()
                .expect("avatar group slot child layout"),
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        self.content.as_widget().mouse_interaction(
            &tree.children[0],
            layout
                .children()
                .next()
                .expect("avatar group slot child layout"),
            cursor,
            viewport,
            renderer,
        )
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut iced::Renderer,
        theme: &iced::Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        self.content.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            style,
            layout
                .children()
                .next()
                .expect("avatar group slot child layout"),
            cursor,
            viewport,
        );
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'b>,
        renderer: &iced::Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, iced::Theme, iced::Renderer>> {
        self.content.as_widget_mut().overlay(
            &mut tree.children[0],
            layout
                .children()
                .next()
                .expect("avatar group slot child layout"),
            renderer,
            viewport,
            translation,
        )
    }
}
