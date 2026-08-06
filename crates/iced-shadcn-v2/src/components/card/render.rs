//! Layout and text rendering for the card component.

use crate::floating_surface::paint_outside_ring;
use crate::iced_compat::advanced::layout::{self, Layout};
use crate::iced_compat::advanced::renderer;
use crate::iced_compat::advanced::widget::{Operation, Tree};
use crate::iced_compat::advanced::{Clipboard, Shell, Widget, overlay};
use crate::iced_compat::alignment::{Horizontal, Vertical};
use crate::iced_compat::widget::text::LineHeight;
use crate::iced_compat::widget::{Space, column, container, row, text as iced_text};
use crate::iced_compat::{
    Background, Color, Element, Event, Font, Length, Padding, Rectangle, Size, Vector, mouse,
};

use super::geometry;
use super::style;
use super::types::{CardFooterAlignment, CardFooterDirection, CardSize};
use super::{
    CardAction, CardContent, CardDescription, CardFooter, CardHeader, CardTextContent, CardTitle,
};
use crate::fonts::iced_font;
use crate::recipes::iced_font_weight;

pub(super) fn build_header<'a, Message>(
    header: CardHeader<'a, Message>,
    spacing: f32,
    card_size: CardSize,
    root_radius: super::types::CardRadius,
) -> Element<'a, Message>
where
    Message: 'a,
{
    let CardHeader {
        theme,
        title,
        description,
        title_element,
        description_element,
        children,
        action,
        spacing: header_spacing,
        border,
        radius,
        style_override,
    } = header;

    let mut main_children = Vec::with_capacity(
        usize::from(title.is_some())
            + usize::from(description.is_some())
            + usize::from(title_element.is_some())
            + usize::from(description_element.is_some())
            + children.len(),
    );

    if let Some(title) = title {
        main_children.push(build_title(title, card_size));
    }
    if let Some(title) = title_element {
        main_children.push(fill_child(title));
    }
    if let Some(description) = description {
        main_children.push(build_description(description));
    }
    if let Some(description) = description_element {
        main_children.push(fill_child(description));
    }
    main_children.extend(children.into_iter().map(fill_child));

    let main = column(main_children)
        .spacing(header_spacing.unwrap_or_else(|| geometry::header_gap(theme)))
        .width(Length::Fill);

    let body: Element<'a, Message> = if let Some(action) = action {
        row![main, container(action).align_x(Horizontal::Right)]
            .spacing(geometry::header_gap(theme))
            .align_y(Vertical::Top)
            .width(Length::Fill)
            .into()
    } else {
        main.into()
    };

    let radius = if radius == super::types::CardRadius::Theme {
        root_radius
    } else {
        radius
    };
    let has_border = style::header_has_border(theme, border);
    let mut section_style = style::resolve_header_style(theme, radius);
    if let Some(override_fn) = style_override.as_ref() {
        section_style = override_fn(section_style);
    }

    let inner = container(body)
        .padding(Padding {
            top: 0.0,
            right: spacing,
            bottom: if has_border { spacing } else { 0.0 },
            left: spacing,
        })
        .width(Length::Fill);

    let section: Element<'a, Message> = if has_border {
        column![inner, horizontal_rule(section_style.border.color)]
            .width(Length::Fill)
            .into()
    } else {
        inner.into()
    };

    container(section)
        .width(Length::Fill)
        .style(move |_| section_style)
        .into()
}

pub(super) fn build_content<'a, Message>(
    content: CardContent<'a, Message>,
    spacing: f32,
) -> Element<'a, Message>
where
    Message: 'a,
{
    let CardContent {
        theme,
        children,
        spacing: content_spacing,
        style_override,
    } = content;

    let body: Element<'a, Message> = column(children)
        .spacing(content_spacing.unwrap_or(0.0))
        .width(Length::Fill)
        .into();

    let mut section_style = style::resolve_content_style(theme);
    if let Some(override_fn) = style_override.as_ref() {
        section_style = override_fn(section_style);
    }

    container(body)
        .padding(Padding {
            top: 0.0,
            right: spacing,
            bottom: 0.0,
            left: spacing,
        })
        .width(Length::Fill)
        .style(move |_| section_style)
        .into()
}

pub(super) fn build_footer<'a, Message>(
    footer: CardFooter<'a, Message>,
    spacing: f32,
    radius: super::types::CardRadius,
) -> Element<'a, Message>
where
    Message: 'a,
{
    let CardFooter {
        theme,
        children,
        direction,
        alignment,
        spacing: footer_spacing,
        border,
        background,
        style_override,
    } = footer;

    let footer_spacing = footer_spacing.unwrap_or(0.0);
    let body: Element<'a, Message> = match direction {
        CardFooterDirection::Row => {
            let row_children = if alignment == CardFooterAlignment::SpaceBetween {
                let mut distributed = Vec::with_capacity(children.len().saturating_mul(2));
                let last = children.len().saturating_sub(1);
                for (index, child) in children.into_iter().enumerate() {
                    distributed.push(child);
                    if index < last {
                        distributed.push(Space::new().width(Length::Fill).into());
                    }
                }
                distributed
            } else {
                children
            };

            let row = row(row_children)
                .spacing(footer_spacing)
                .align_y(Vertical::Center);

            if alignment == CardFooterAlignment::SpaceBetween {
                row.width(Length::Fill).into()
            } else {
                container(row)
                    .width(Length::Fill)
                    .align_x(footer_alignment(alignment))
                    .into()
            }
        }
        CardFooterDirection::Column => column(children.into_iter().map(|child| {
            container(child)
                .width(Length::Fill)
                .align_x(footer_alignment(alignment))
                .into()
        }))
        .spacing(footer_spacing)
        .width(Length::Fill)
        .into(),
    };

    let has_border = style::footer_has_border(theme, border);
    let full_padding = geometry::footer_uses_full_padding(theme);
    let mut section_style = style::resolve_footer_style(theme, radius, background);
    if let Some(override_fn) = style_override.as_ref() {
        section_style = override_fn(section_style);
    }

    let inner = container(body)
        .padding(Padding {
            top: if full_padding || has_border {
                spacing
            } else {
                0.0
            },
            right: spacing,
            bottom: if full_padding { spacing } else { 0.0 },
            left: spacing,
        })
        .width(Length::Fill);

    let section: Element<'a, Message> = if has_border {
        column![horizontal_rule(section_style.border.color), inner]
            .width(Length::Fill)
            .into()
    } else {
        inner.into()
    };

    container(section)
        .width(Length::Fill)
        .clip(true)
        .style(move |_| section_style)
        .into()
}

pub(super) fn build_action<'a, Message>(action: CardAction<'a, Message>) -> Element<'a, Message>
where
    Message: 'a,
{
    let CardAction {
        content,
        width,
        height,
        style_override,
    } = action;

    let mut resolved = container::Style {
        text_color: None,
        ..container::Style::default()
    };
    if let Some(override_fn) = style_override.as_ref() {
        resolved = override_fn(resolved);
    }

    container(content)
        .width(width)
        .height(height)
        .style(move |_| resolved)
        .into()
}

pub(super) fn build_title<'a, Message>(
    title: CardTitle<'a, Message>,
    card_size: CardSize,
) -> Element<'a, Message>
where
    Message: 'a,
{
    let CardTitle {
        content,
        theme,
        text_size,
        line_height,
        color,
        font,
        width,
        style_override,
    } = title;
    let metrics = geometry::title_metrics(theme, card_size);
    build_text(
        content,
        theme,
        TextOptions {
            size_px: text_size.unwrap_or(metrics.size_px),
            line_height_px: line_height.unwrap_or(metrics.line_height_px),
            color: color.unwrap_or(theme.palette.card_foreground),
            custom_font: font,
            metrics,
            width,
            style_override,
        },
    )
}

pub(super) fn build_description<'a, Message>(
    description: CardDescription<'a, Message>,
) -> Element<'a, Message>
where
    Message: 'a,
{
    let CardDescription {
        content,
        theme,
        text_size,
        line_height,
        color,
        font,
        width,
        style_override,
    } = description;
    let metrics = geometry::description_metrics(theme);
    build_text(
        content,
        theme,
        TextOptions {
            size_px: text_size.unwrap_or(metrics.size_px),
            line_height_px: line_height.unwrap_or(metrics.line_height_px),
            color: color.unwrap_or(theme.palette.muted_foreground),
            custom_font: font,
            metrics,
            width,
            style_override,
        },
    )
}

struct TextOptions<'a> {
    size_px: f32,
    line_height_px: f32,
    color: Color,
    custom_font: Option<Font>,
    metrics: geometry::TextMetrics,
    width: Length,
    style_override: Option<Box<dyn Fn(container::Style) -> container::Style + 'a>>,
}

fn build_text<'a, Message>(
    content: CardTextContent<'a, Message>,
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
        metrics,
        width,
        style_override,
    } = options;
    let mut font = custom_font.unwrap_or_else(|| {
        let face = if metrics.semibold || metrics.uppercase {
            theme.font_pack().heading
        } else {
            theme.font_pack().sans
        };
        iced_font(face)
    });
    if custom_font.is_none() {
        font.weight = iced_font_weight(if metrics.semibold {
            shadcn_common::FontWeight::Semibold
        } else {
            shadcn_common::FontWeight::Normal
        });
    }

    let content: Element<'a, Message> = match content {
        CardTextContent::Label(fragment) => {
            if metrics.uppercase {
                iced_text(fragment.as_ref().to_uppercase())
                    .size(size_px)
                    .line_height(LineHeight::Absolute(line_height_px.into()))
                    .font(font)
                    .width(width)
                    .into()
            } else {
                iced_text(fragment)
                    .size(size_px)
                    .line_height(LineHeight::Absolute(line_height_px.into()))
                    .font(font)
                    .width(width)
                    .into()
            }
        }
        CardTextContent::Element(element) => container(element).width(width).into(),
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

pub(super) fn fill_child<'a, Message>(child: Element<'a, Message>) -> Element<'a, Message>
where
    Message: 'a,
{
    container(child).width(Length::Fill).into()
}

/// Wraps card content and paints CSS `ring-1` outside the bounds (see
/// [`crate::floating_surface::paint_outside_ring`]).
pub(super) fn with_outside_ring<'a, Message>(
    content: Element<'a, Message>,
    ring_color: Color,
    ring_width: f32,
    radius: f32,
) -> Element<'a, Message>
where
    Message: 'a,
{
    Element::new(OutsideRing {
        content,
        ring_color,
        ring_width,
        radius,
    })
}

struct OutsideRing<'a, Message> {
    content: Element<'a, Message>,
    ring_color: Color,
    ring_width: f32,
    radius: f32,
}

impl<Message> Widget<Message, crate::iced_compat::Theme, crate::iced_compat::Renderer>
    for OutsideRing<'_, Message>
{
    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.content));
    }

    fn size(&self) -> Size<Length> {
        self.content.as_widget().size()
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &crate::iced_compat::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        self.content
            .as_widget_mut()
            .layout(&mut tree.children[0], renderer, limits)
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &crate::iced_compat::Renderer,
        operation: &mut dyn Operation,
    ) {
        self.content
            .as_widget_mut()
            .operate(&mut tree.children[0], layout, renderer, operation);
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
        self.content.as_widget_mut().update(
            &mut tree.children[0],
            event,
            layout,
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut crate::iced_compat::Renderer,
        theme: &crate::iced_compat::Theme,
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
            layout,
            cursor,
            viewport,
        );
        paint_outside_ring(
            renderer,
            layout.bounds(),
            self.ring_color,
            self.ring_width,
            self.radius,
        );
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &crate::iced_compat::Renderer,
    ) -> mouse::Interaction {
        self.content.as_widget().mouse_interaction(
            &tree.children[0],
            layout,
            cursor,
            viewport,
            renderer,
        )
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
        self.content.as_widget_mut().overlay(
            &mut tree.children[0],
            layout,
            renderer,
            viewport,
            translation,
        )
    }
}

fn footer_alignment(alignment: CardFooterAlignment) -> Horizontal {
    match alignment {
        CardFooterAlignment::Start | CardFooterAlignment::SpaceBetween => Horizontal::Left,
        CardFooterAlignment::Center => Horizontal::Center,
        CardFooterAlignment::End => Horizontal::Right,
    }
}

fn horizontal_rule<'a, Message>(color: Color) -> Element<'a, Message>
where
    Message: 'a,
{
    container(Space::new())
        .width(Length::Fill)
        .height(Length::Fixed(1.0))
        .style(move |_| container::Style {
            background: Some(Background::Color(color)),
            ..container::Style::default()
        })
        .into()
}
