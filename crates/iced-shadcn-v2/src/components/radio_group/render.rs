//! Content composition for the radio group (indicator, label, click wrapper).

use crate::iced_compat::alignment::Vertical;
use crate::iced_compat::widget::button as button_widget;
use crate::iced_compat::widget::text::{Fragment, LineHeight};
use crate::iced_compat::widget::{
    button as iced_button, column, container, row, space, text as iced_text,
};
use crate::iced_compat::{Background, Border, Element, Length, Padding, Shadow};
use shadcn_common::LabelContext;

use super::ItemContent;
use super::types::{RadioGroupOrientation, RadioGroupStyle};
use crate::components::label::Label;
use crate::fonts::iced_font;
use crate::recipes::iced_font_weight;
use crate::theme::Theme;

/// Gap between a label and its description (`gap-1` on the web).
const DESCRIPTION_GAP_PX: f32 = 4.0;

/// Layout metrics shared by every item of one group.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct ItemLayout {
    /// Indicator footprint including the reserved ring width.
    pub(super) footprint: f32,
    /// Gap between the indicator and the label column.
    pub(super) label_gap: f32,
    /// Width of the whole item.
    pub(super) width: Length,
}

/// Builds the ring / indicator / dot stack of one item.
///
/// The ring is painted as a disc behind the indicator and its width is always
/// reserved, so focusing an item never reflows the group.
pub(super) fn build_indicator<'a, Message: 'a>(
    footprint: f32,
    style: RadioGroupStyle,
) -> Element<'a, Message> {
    let dot = container(space())
        .width(Length::Fixed(style.dot_size))
        .height(Length::Fixed(style.dot_size))
        .style(move |_iced_theme| container::Style {
            background: (style.dot_size > 0.0).then_some(Background::Color(style.dot)),
            border: Border {
                radius: (style.dot_size / 2.0).into(),
                ..Border::default()
            },
            ..container::Style::default()
        });

    let indicator = container(dot)
        .width(Length::Fixed(style.indicator_size))
        .height(Length::Fixed(style.indicator_size))
        .align_x(crate::iced_compat::alignment::Horizontal::Center)
        .align_y(Vertical::Center)
        .style(move |_iced_theme| container::Style {
            background: Some(Background::Color(style.indicator)),
            border: Border {
                color: style.border,
                width: style.border_width,
                radius: style.radius.into(),
            },
            ..container::Style::default()
        });

    container(indicator)
        .width(Length::Fixed(footprint))
        .height(Length::Fixed(footprint))
        .padding(style.ring_width)
        .style(move |_iced_theme| container::Style {
            background: style.ring.map(Background::Color),
            border: Border {
                radius: (style.radius + style.ring_width).into(),
                ..Border::default()
            },
            ..container::Style::default()
        })
        .into()
}

/// Builds one clickable item: indicator, label, and optional description.
pub(super) fn build_item<'a, Message>(
    theme: &'a Theme,
    content: ItemContent<'a, Message>,
    description: Option<Fragment<'a>>,
    style: RadioGroupStyle,
    layout: ItemLayout,
    on_press: Option<Message>,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let has_description = description.is_some();
    let mut lines: Vec<Element<'a, Message>> = Vec::with_capacity(2);

    match content {
        ItemContent::Empty => {}
        ItemContent::Text(label) => lines.push(
            Label::text(label, theme)
                .context(LabelContext::AdjacentControl)
                .color(style.label)
                .into(),
        ),
        ItemContent::Element(content) => lines.push(content),
    }

    if let Some(description) = description {
        lines.push(description_text(theme, description, style));
    }

    let mut children: Vec<Element<'a, Message>> = Vec::with_capacity(2);
    children.push(build_indicator(layout.footprint, style));

    if !lines.is_empty() {
        children.push(column(lines).spacing(DESCRIPTION_GAP_PX).into());
    }

    // A description makes the item multi-line, so the indicator aligns with the
    // first line instead of the block's optical center.
    let body = row(children)
        .spacing(layout.label_gap)
        .align_y(if has_description {
            Vertical::Top
        } else {
            Vertical::Center
        });

    iced_button(body)
        .padding(Padding::ZERO)
        .width(layout.width)
        .on_press_maybe(on_press)
        .style(move |_iced_theme, _status| button_widget::Style {
            background: None,
            text_color: style.label,
            border: Border::default(),
            shadow: Shadow::default(),
            snap: false,
        })
        .into()
}

/// Builds the group root: a stack (or row) of items with the pack's gap.
pub(super) fn build_group<'a, Message: 'a>(
    items: Vec<Element<'a, Message>>,
    orientation: RadioGroupOrientation,
    gap: f32,
    width: Length,
    height: Length,
    style_override: Option<Box<dyn Fn(container::Style) -> container::Style + 'a>>,
) -> Element<'a, Message> {
    let content: Element<'a, Message> = if orientation.is_horizontal() {
        row(items).spacing(gap).align_y(Vertical::Center).into()
    } else {
        column(items).spacing(gap).into()
    };

    container(content)
        .width(width)
        .height(height)
        .style(move |_iced_theme| {
            let mut style = container::Style::default();

            if let Some(override_fn) = style_override.as_ref() {
                style = override_fn(style);
            }

            style
        })
        .into()
}

/// Muted secondary line under a label, at the pack's adjacent-label size.
fn description_text<'a, Message: 'a>(
    theme: &Theme,
    description: Fragment<'a>,
    style: RadioGroupStyle,
) -> Element<'a, Message> {
    let typography = theme.style.label(LabelContext::AdjacentControl).typography;
    let mut font = iced_font(theme.font_pack().sans);
    font.weight = iced_font_weight(shadcn_common::FontWeight::Normal);

    iced_text(description)
        .size(typography.size_px)
        .font(font)
        .line_height(LineHeight::Absolute(typography.line_height_px.into()))
        .color(style.description)
        .into()
}
