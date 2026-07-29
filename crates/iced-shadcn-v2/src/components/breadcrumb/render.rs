//! Breadcrumb layout assembly and text rendering.

use crate::iced_compat::alignment::{Horizontal, Vertical};
use crate::iced_compat::widget::button as iced_button;
use crate::iced_compat::widget::text::LineHeight;
use crate::iced_compat::widget::{container, row, text as iced_text};
use crate::iced_compat::{Color, Element, Font, Length, Padding};

use shadcn_common::FontWeight;

use super::{
    Breadcrumb, BreadcrumbEllipsis, BreadcrumbEntry, BreadcrumbItem, BreadcrumbLink,
    BreadcrumbList, BreadcrumbPage, BreadcrumbSeparator, EllipsisStyleOverride, EntryKind,
    TextContent, geometry, icon, style,
};
use crate::fonts::iced_font;
use crate::recipes::iced_font_weight;
use crate::theme::Theme;

/// Tokens the web `<ol>` hands to its descendants through CSS inheritance.
///
/// Every entry falls back to these values for the properties it does not set
/// itself, which is what makes [`BreadcrumbList::color`] and friends behave
/// like the class they port.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct Inherited {
    pub(super) color: Color,
    pub(super) text_size_px: f32,
    pub(super) line_height_px: f32,
    pub(super) uppercase: bool,
}

/// Inherited tokens of a standalone part, taken straight from the style pack.
pub(super) fn inherited(theme: &Theme) -> Inherited {
    let metrics = geometry::metrics(theme);

    Inherited {
        color: style::muted_color(theme),
        text_size_px: metrics.text_size_px,
        line_height_px: metrics.line_height_px,
        uppercase: metrics.uppercase,
    }
}

pub(super) fn build_breadcrumb<'a, Message>(
    breadcrumb: Breadcrumb<'a, Message>,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let Breadcrumb {
        theme: _,
        list,
        width,
        height,
        padding,
        aria_label: _,
        style_override,
    } = breadcrumb;

    let content = build_list(list);

    let mut resolved = container::Style::default();
    if let Some(override_fn) = style_override.as_ref() {
        resolved = override_fn(resolved);
    }

    container(content)
        .width(width)
        .height(height)
        .padding(padding)
        .style(move |_| resolved)
        .into()
}

pub(super) fn build_list<'a, Message>(list: BreadcrumbList<'a, Message>) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let BreadcrumbList {
        theme,
        entries,
        spacing,
        line_spacing,
        wrap,
        width,
        color,
        text_size,
        line_height,
        style_override,
    } = list;

    let metrics = geometry::metrics(theme);
    let mut context = inherited(theme);
    if let Some(color) = color {
        context.color = color;
    }
    if let Some(text_size) = text_size {
        context.text_size_px = text_size;
    }
    if let Some(line_height) = line_height {
        context.line_height_px = line_height;
    }

    let spacing = spacing.unwrap_or(metrics.list_gap_px);
    let line_spacing = line_spacing.unwrap_or(spacing);

    let children: Vec<Element<'a, Message>> = entries
        .into_iter()
        .map(|entry| build_entry(entry, context))
        .collect();
    let entries = row(children)
        .spacing(spacing)
        .align_y(Vertical::Center)
        .width(width);

    // `flex-wrap` on the web list; a wrapping row breaks against the width its
    // parent offers, so a shrinking list still wraps.
    let content: Element<'a, Message> = if wrap {
        entries.wrap().vertical_spacing(line_spacing).into()
    } else {
        entries.into()
    };

    let mut resolved = style::text_color_surface(context.color);
    if let Some(override_fn) = style_override.as_ref() {
        resolved = override_fn(resolved);
    }

    container(content)
        .width(width)
        .style(move |_| resolved)
        .into()
}

fn build_entry<'a, Message>(
    entry: BreadcrumbEntry<'a, Message>,
    context: Inherited,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    match entry.kind {
        EntryKind::Item(item) => build_item(*item, context),
        EntryKind::Separator(separator) => build_separator(*separator, context),
        EntryKind::Link(link) => build_link(*link, context),
        EntryKind::Page(page) => build_page(*page, context),
        EntryKind::Ellipsis(ellipsis) => build_ellipsis(*ellipsis, context),
        EntryKind::Element(element) => element,
    }
}

pub(super) fn build_item<'a, Message>(
    item: BreadcrumbItem<'a, Message>,
    context: Inherited,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let BreadcrumbItem {
        theme,
        children,
        spacing,
        width,
        style_override,
    } = item;

    let spacing = spacing.unwrap_or_else(|| geometry::metrics(theme).item_gap_px);
    let children: Vec<Element<'a, Message>> = children
        .into_iter()
        .map(|child| build_entry(child, context))
        .collect();

    let content = row(children)
        .spacing(spacing)
        .align_y(Vertical::Center)
        .width(width);

    let mut resolved = container::Style::default();
    if let Some(override_fn) = style_override.as_ref() {
        resolved = override_fn(resolved);
    }

    container(content)
        .width(width)
        .style(move |_| resolved)
        .into()
}

pub(super) fn build_link<'a, Message>(
    link: BreadcrumbLink<'a, Message>,
    context: Inherited,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let BreadcrumbLink {
        content,
        theme,
        href: _,
        color,
        hover_color,
        text_size,
        line_height,
        font,
        width,
        disabled,
        on_press,
        style_override,
    } = link;

    let resting = color.unwrap_or(context.color);
    let hovered = hover_color.unwrap_or_else(|| style::current_color(theme));
    let body = build_text(
        content,
        theme,
        font,
        TextOptions {
            size_px: text_size.unwrap_or(context.text_size_px),
            line_height_px: line_height.unwrap_or(context.line_height_px),
            uppercase: context.uppercase,
            width,
        },
    );
    let on_press = if disabled { None } else { on_press };

    iced_button(body)
        .padding(Padding::ZERO)
        .width(width)
        .on_press_maybe(on_press)
        .style(move |_iced_theme, status| {
            let mut resolved = style::resolve_link_style(resting, hovered, status);

            if let Some(override_fn) = style_override.as_ref() {
                resolved = override_fn(resolved, status);
            }

            resolved
        })
        .into()
}

pub(super) fn build_page<'a, Message>(
    page: BreadcrumbPage<'a, Message>,
    context: Inherited,
) -> Element<'a, Message>
where
    Message: 'a,
{
    let BreadcrumbPage {
        content,
        theme,
        color,
        text_size,
        line_height,
        font,
        width,
        style_override,
    } = page;

    // `.cn-breadcrumb-page` sets `text-foreground`, overriding the list color.
    let text_color = color.unwrap_or_else(|| style::current_color(theme));
    let body = build_text(
        content,
        theme,
        font,
        TextOptions {
            size_px: text_size.unwrap_or(context.text_size_px),
            line_height_px: line_height.unwrap_or(context.line_height_px),
            uppercase: context.uppercase,
            width,
        },
    );

    let mut resolved = style::text_color_surface(text_color);
    if let Some(override_fn) = style_override.as_ref() {
        resolved = override_fn(resolved);
    }

    container(body).width(width).style(move |_| resolved).into()
}

pub(super) fn build_separator<'a, Message>(
    separator: BreadcrumbSeparator<'a, Message>,
    context: Inherited,
) -> Element<'a, Message>
where
    Message: 'a,
{
    let BreadcrumbSeparator {
        content,
        theme,
        color,
        icon_size,
        text_size,
        line_height,
        font,
        style_override,
    } = separator;

    let metrics = geometry::metrics(theme);
    let tint = color.unwrap_or(context.color);

    // Canvas geometry cannot inherit a text color, so the glyph is tinted
    // explicitly with the same value the surface propagates.
    let body: Element<'a, Message> = match content {
        None => icon::icon(icon::Icon::new(
            icon::Glyph::ChevronRight,
            icon_size.unwrap_or(metrics.separator_icon_px),
            tint,
        )),
        Some(content) => build_text(
            content,
            theme,
            font,
            TextOptions {
                size_px: text_size.unwrap_or(context.text_size_px),
                line_height_px: line_height.unwrap_or(context.line_height_px),
                uppercase: context.uppercase,
                width: Length::Shrink,
            },
        ),
    };

    let mut resolved = style::text_color_surface(tint);
    if let Some(override_fn) = style_override.as_ref() {
        resolved = override_fn(resolved);
    }

    container(body)
        .align_y(Vertical::Center)
        .style(move |_| resolved)
        .into()
}

pub(super) fn build_ellipsis<'a, Message>(
    ellipsis: BreadcrumbEllipsis<'a, Message>,
    context: Inherited,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let BreadcrumbEllipsis {
        theme,
        color,
        size,
        icon_size,
        screen_reader_label: _,
        on_press,
        style_override,
    } = ellipsis;

    let metrics = geometry::metrics(theme);
    let tint = color.unwrap_or(context.color);
    let box_px = size.unwrap_or(metrics.ellipsis_box_px);
    let glyph_px = icon_size.unwrap_or(metrics.ellipsis_icon_px);

    let glyph = icon::icon(icon::Icon::new(icon::Glyph::Ellipsis, glyph_px, tint));
    let body = container(glyph)
        .width(Length::Fixed(box_px))
        .height(Length::Fixed(box_px))
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center);

    match on_press {
        Some(message) => iced_button(body)
            .padding(Padding::ZERO)
            .on_press(message)
            .style(move |_iced_theme, status| {
                // The web ellipsis has no hover treatment of its own; the
                // wrapping trigger owns it.
                let mut resolved = style::resolve_link_style(tint, tint, status);

                if let Some(EllipsisStyleOverride::Button(override_fn)) = style_override.as_ref() {
                    resolved = override_fn(resolved, status);
                }

                resolved
            })
            .into(),
        None => {
            let mut resolved = style::text_color_surface(tint);

            if let Some(EllipsisStyleOverride::Container(override_fn)) = style_override.as_ref() {
                resolved = override_fn(resolved);
            }

            container(body).style(move |_| resolved).into()
        }
    }
}

/// Typography of one breadcrumb text part, after inheritance is applied.
struct TextOptions {
    size_px: f32,
    line_height_px: f32,
    uppercase: bool,
    width: Length,
}

fn build_text<'a, Message: 'a>(
    content: TextContent<'a, Message>,
    theme: &Theme,
    font: Option<Font>,
    options: TextOptions,
) -> Element<'a, Message> {
    let font = font.unwrap_or_else(|| {
        let mut font = iced_font(theme.font_pack().sans);
        font.weight = iced_font_weight(FontWeight::Normal);
        font
    });

    match content {
        TextContent::Label(fragment) => {
            // iced has no CSS `text-transform`; own the string so Sera can
            // uppercase its trail.
            let label = if options.uppercase {
                iced_text(fragment.as_ref().to_uppercase())
            } else {
                iced_text(fragment)
            };

            label
                .size(options.size_px)
                .line_height(LineHeight::Absolute(options.line_height_px.into()))
                .font(font)
                .width(options.width)
                .into()
        }
        // Text colors reach nested widgets through the surface style, so an
        // element slot only needs the width applied.
        TextContent::Element(element) => container(element).width(options.width).into(),
    }
}
