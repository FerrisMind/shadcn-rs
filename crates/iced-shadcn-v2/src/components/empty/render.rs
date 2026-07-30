//! Layout, text, and dashed-border rendering for the empty-state component.

use crate::iced_compat::alignment::{Horizontal, Vertical};
use crate::iced_compat::widget::text::LineHeight;
use crate::iced_compat::widget::{canvas, column, container, row, stack, text as iced_text};
use crate::iced_compat::{Color, Element, Font, Length, Padding, Point, Rectangle, Size};

use crate::fonts::iced_font;
use crate::recipes::iced_font_weight;

use super::geometry;
use super::style;
use super::types::{EmptyBorderStyle, EmptyMediaVariant};
use super::{
    Empty, EmptyChild, EmptyContent, EmptyDescription, EmptyHeader, EmptyMedia, EmptyTextContent,
    EmptyTitle,
};

/// Builds the root empty state.
pub(super) fn build_root<'a, Message>(empty: Empty<'a, Message>) -> Element<'a, Message>
where
    Message: 'a,
{
    let Empty {
        theme,
        width,
        height,
        padding: custom_padding,
        spacing: custom_spacing,
        radius,
        border: border_style,
        border_width,
        border_color,
        background,
        align_x,
        align_y,
        children,
        style_override,
    } = empty;

    let metrics = geometry::metrics(theme);
    let spacing = custom_spacing.unwrap_or(metrics.root_gap_px);
    let padding = custom_padding.unwrap_or_else(|| Padding::from(metrics.root_padding_px));

    let children = children
        .into_iter()
        .map(|child| match child {
            EmptyChild::Element(element) => fill_child(element),
            EmptyChild::Header(header) => build_header(*header),
            EmptyChild::Content(content) => build_content(*content),
        })
        .collect::<Vec<_>>();

    let body = column(children)
        .spacing(spacing)
        .width(Length::Fill)
        .align_x(align_x);

    let mut resolved = style::resolve_root_style(
        theme,
        radius,
        border_style,
        border_width,
        border_color,
        background,
    );
    if let Some(override_fn) = style_override.as_ref() {
        resolved = override_fn(resolved);
    }

    let canvas_border = resolved.border;
    let base_style = if border_style == EmptyBorderStyle::Dashed {
        let mut base_style = resolved;
        base_style.border.width = 0.0;
        base_style
    } else {
        resolved
    };
    let base = container(body)
        .padding(padding)
        .width(width)
        .height(height)
        .align_x(align_x)
        .align_y(align_y)
        .clip(true)
        .style(move |_| base_style);

    if border_style == EmptyBorderStyle::Dashed {
        let dashed_width = canvas_border.width;
        let overlay = canvas(DashedBorder {
            color: canvas_border.color,
            width: dashed_width,
            radius: canvas_border.radius,
        })
        .width(Length::Fill)
        .height(Length::Fill);

        stack![base, overlay]
            .width(width)
            .height(height)
            .clip(true)
            .into()
    } else {
        base.into()
    }
}

/// Builds a centered header section.
pub(super) fn build_header<'a, Message>(header: EmptyHeader<'a, Message>) -> Element<'a, Message>
where
    Message: 'a,
{
    let EmptyHeader {
        theme,
        media,
        title,
        description,
        title_element,
        description_element,
        children,
        width,
        max_width,
        spacing: custom_spacing,
        style_override,
    } = header;

    let metrics = geometry::metrics(theme);
    let spacing = custom_spacing.unwrap_or(metrics.header_gap_px);
    let mut rows = Vec::with_capacity(
        usize::from(media.is_some())
            + usize::from(title.is_some())
            + usize::from(description.is_some())
            + usize::from(title_element.is_some())
            + usize::from(description_element.is_some())
            + children.len(),
    );

    if let Some(media) = media {
        rows.push(build_media(media));
    }
    if let Some(title) = title {
        rows.push(build_title_with_width(title, max_width));
    }
    if let Some(title) = title_element {
        rows.push(fill_centered(title));
    }
    if let Some(description) = description {
        rows.push(build_description_with_width(description, max_width));
    }
    if let Some(description) = description_element {
        rows.push(fill_centered(description));
    }
    rows.extend(children.into_iter().map(fill_centered));

    let body = column(rows)
        .spacing(spacing)
        .width(Length::Fill)
        .align_x(Horizontal::Center);

    let mut resolved = style::resolve_text_style(theme.palette.foreground);
    if let Some(override_fn) = style_override.as_ref() {
        resolved = override_fn(resolved);
    }

    container(body)
        .width(width)
        .max_width(max_width)
        .align_x(Horizontal::Center)
        .style(move |_| resolved)
        .into()
}

/// Builds the media slot.
pub(super) fn build_media<'a, Message>(media: EmptyMedia<'a, Message>) -> Element<'a, Message>
where
    Message: 'a,
{
    let EmptyMedia {
        theme,
        variant,
        children,
        width,
        height,
        size,
        radius,
        spacing: custom_spacing,
        style_override,
    } = media;

    let metrics = geometry::metrics(theme);
    let spacing = custom_spacing.unwrap_or(8.0);
    let body = row(children).spacing(spacing).align_y(Vertical::Center);

    let radius = radius.map(|radius| geometry::media_radius_px(theme, radius));
    let mut resolved = style::resolve_media_style(theme, variant, radius);
    if let Some(override_fn) = style_override.as_ref() {
        resolved = override_fn(resolved);
    }

    let width = if let Some(size) = size {
        Length::Fixed(size)
    } else if variant == EmptyMediaVariant::Icon && width == Length::Shrink {
        Length::Fixed(metrics.media_size_px)
    } else {
        width
    };
    let height = if let Some(size) = size {
        Length::Fixed(size)
    } else if variant == EmptyMediaVariant::Icon && height == Length::Shrink {
        Length::Fixed(metrics.media_size_px)
    } else {
        height
    };

    let media = container(body)
        .width(width)
        .height(height)
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
        .clip(variant == EmptyMediaVariant::Icon)
        .style(move |_| resolved);

    container(media)
        .padding(Padding {
            top: 0.0,
            right: 0.0,
            bottom: metrics.media_margin_bottom_px,
            left: 0.0,
        })
        .into()
}

/// Builds the content section.
pub(super) fn build_content<'a, Message>(content: EmptyContent<'a, Message>) -> Element<'a, Message>
where
    Message: 'a,
{
    let EmptyContent {
        theme,
        children,
        width,
        max_width,
        spacing: custom_spacing,
        style_override,
    } = content;

    let metrics = geometry::metrics(theme);
    let spacing = custom_spacing.unwrap_or(metrics.content_gap_px);
    let body = column(children)
        .spacing(spacing)
        .width(Length::Fill)
        .align_x(Horizontal::Center);

    let mut resolved = style::resolve_text_style(theme.palette.foreground);
    if let Some(override_fn) = style_override.as_ref() {
        resolved = override_fn(resolved);
    }

    container(body)
        .width(width)
        .max_width(max_width)
        .align_x(Horizontal::Center)
        .style(move |_| resolved)
        .into()
}

/// Builds a typed title.
pub(super) fn build_title<'a, Message>(title: EmptyTitle<'a, Message>) -> Element<'a, Message>
where
    Message: 'a,
{
    build_title_with_width(title, 384.0)
}

fn build_title_with_width<'a, Message>(
    title: EmptyTitle<'a, Message>,
    balance_width_px: f32,
) -> Element<'a, Message>
where
    Message: 'a,
{
    let EmptyTitle {
        content,
        theme,
        text_size,
        line_height,
        color,
        font,
        width,
        align_x,
        style_override,
    } = title;
    let metrics = geometry::title_metrics(theme);

    build_text(
        content,
        TextOptions {
            size_px: text_size.unwrap_or(metrics.size_px),
            line_height_px: line_height.unwrap_or(metrics.line_height_px),
            color: color.unwrap_or(theme.palette.foreground),
            custom_font: font,
            default_font: iced_font(theme.font_pack().heading),
            width,
            align_x,
            uppercase: metrics.uppercase,
            top_padding_px: metrics.top_padding_px,
            weight: metrics.weight,
            balance_width_px,
            style_override,
        },
    )
}

/// Builds a typed description.
pub(super) fn build_description<'a, Message>(
    description: EmptyDescription<'a, Message>,
) -> Element<'a, Message>
where
    Message: 'a,
{
    build_description_with_width(description, 384.0)
}

fn build_description_with_width<'a, Message>(
    description: EmptyDescription<'a, Message>,
    balance_width_px: f32,
) -> Element<'a, Message>
where
    Message: 'a,
{
    let EmptyDescription {
        content,
        theme,
        text_size,
        line_height,
        color,
        font,
        width,
        align_x,
        style_override,
    } = description;
    let metrics = geometry::description_metrics(theme);

    build_text(
        content,
        TextOptions {
            size_px: text_size.unwrap_or(metrics.size_px),
            line_height_px: line_height.unwrap_or(metrics.line_height_px),
            color: color.unwrap_or(theme.palette.muted_foreground),
            custom_font: font,
            default_font: iced_font(theme.font_pack().sans),
            width,
            align_x,
            uppercase: metrics.uppercase,
            top_padding_px: metrics.top_padding_px,
            weight: metrics.weight,
            balance_width_px,
            style_override,
        },
    )
}

struct TextOptions<'a> {
    size_px: f32,
    line_height_px: f32,
    color: Color,
    custom_font: Option<Font>,
    default_font: Font,
    width: Length,
    align_x: Horizontal,
    uppercase: bool,
    top_padding_px: f32,
    weight: shadcn_common::FontWeight,
    balance_width_px: f32,
    style_override: Option<Box<dyn Fn(container::Style) -> container::Style + 'a>>,
}

fn build_text<'a, Message>(
    content: EmptyTextContent<'a, Message>,
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
        default_font,
        width,
        align_x,
        uppercase,
        top_padding_px,
        weight,
        balance_width_px,
        style_override,
    } = options;

    let font = custom_font.unwrap_or_else(|| {
        let mut font = default_font;
        font.weight = iced_font_weight(weight);
        font
    });

    let content: Element<'a, Message> = match content {
        EmptyTextContent::Label(fragment) => {
            let label = if uppercase {
                balance_text(&fragment.as_ref().to_uppercase(), balance_width_px, size_px)
            } else {
                balance_text(fragment.as_ref(), balance_width_px, size_px)
            };
            iced_text(label)
                .size(size_px)
                .line_height(LineHeight::Absolute(line_height_px.into()))
                .font(font)
                .width(width)
                .align_x(align_x)
                .into()
        }
        EmptyTextContent::Element(element) => element,
    };

    let mut resolved = style::resolve_text_style(color);
    if let Some(override_fn) = style_override.as_ref() {
        resolved = override_fn(resolved);
    }

    let content = container(content)
        .width(width)
        .align_x(align_x)
        .style(move |_| resolved);

    if top_padding_px > 0.0 {
        container(content)
            .width(width)
            .padding(Padding {
                top: top_padding_px,
                right: 0.0,
                bottom: 0.0,
                left: 0.0,
            })
            .into()
    } else {
        content.into()
    }
}

/// Approximates CSS `text-wrap: balance` for typed slot labels.
///
/// Iced's regular text widget wraps to the available width, while the source
/// component balances short paragraphs into similarly wide lines. Word-based
/// balancing keeps the source demo's two-line description stable without
/// changing arbitrary user-provided elements.
pub(super) fn balance_text(label: &str, max_width_px: f32, size_px: f32) -> String {
    if label.contains('\n') || label.split_whitespace().count() < 2 {
        return label.to_owned();
    }

    let approximate_character_width = (size_px * 0.5).max(1.0);
    let characters_per_line = (max_width_px / approximate_character_width)
        .floor()
        .max(1.0) as usize;
    let total_characters = label.chars().count();
    let line_count = total_characters.div_ceil(characters_per_line);
    if line_count < 2 {
        return label.to_owned();
    }

    let target_line_length = (total_characters / line_count).max(1);
    let words = label.split_whitespace().collect::<Vec<_>>();
    let mut lines = Vec::with_capacity(line_count);
    let mut current = String::new();

    let last_balanced_line = line_count - 1;
    for word in words {
        let would_exceed = lines.len() < last_balanced_line
            && !current.is_empty()
            && current.chars().count() + 1 + word.chars().count() > target_line_length;
        if would_exceed {
            lines.push(current);
            current = String::new();
        }

        if !current.is_empty() {
            current.push(' ');
        }
        current.push_str(word);
    }

    if !current.is_empty() {
        lines.push(current);
    }

    lines.join("\n")
}

fn fill_child<'a, Message>(child: Element<'a, Message>) -> Element<'a, Message>
where
    Message: 'a,
{
    fill_centered(child)
}

fn fill_centered<'a, Message>(child: Element<'a, Message>) -> Element<'a, Message>
where
    Message: 'a,
{
    container(child)
        .width(Length::Fill)
        .align_x(Horizontal::Center)
        .into()
}

/// Canvas program used for a real rounded dashed outline.
#[derive(Debug, Clone, Copy)]
struct DashedBorder {
    color: Color,
    width: f32,
    radius: crate::iced_compat::border::Radius,
}

impl<Message> canvas::Program<Message> for DashedBorder {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &crate::iced_compat::Renderer,
        _theme: &crate::iced_compat::Theme,
        bounds: Rectangle,
        _cursor: crate::iced_compat::mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let width = if self.width.is_finite() {
            self.width.max(0.0)
        } else {
            0.0
        };
        if width <= 0.0 || bounds.width <= width || bounds.height <= width {
            return Vec::new();
        }

        let inset = width / 2.0;
        let size = Size::new(bounds.width - width, bounds.height - width);
        let max_radius = (size.width.min(size.height) / 2.0).max(0.0);
        let radius = crate::iced_compat::border::radius(
            self.radius
                .top_left
                .min(max_radius)
                .max(0.0)
                .min(self.radius.top_right.min(max_radius).max(0.0))
                .min(self.radius.bottom_right.min(max_radius).max(0.0))
                .min(self.radius.bottom_left.min(max_radius).max(0.0)),
        );
        let path = canvas::Path::rounded_rectangle(Point::new(inset, inset), size, radius);

        let mut stroke = canvas::Stroke::default()
            .with_color(self.color)
            .with_width(width)
            .with_line_join(canvas::LineJoin::Round);
        stroke.line_dash = canvas::LineDash {
            segments: &[6.0, 4.0],
            offset: 0,
        };

        let mut frame = canvas::Frame::new(renderer, bounds.size());
        frame.stroke(&path, stroke);
        vec![frame.into_geometry()]
    }
}
