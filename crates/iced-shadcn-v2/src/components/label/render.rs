//! Content composition for labels (text, icons, click wrapper).

use crate::iced_compat::alignment::Vertical;
use crate::iced_compat::widget::button as button_widget;
use crate::iced_compat::widget::button as iced_button;
use crate::iced_compat::widget::text::{Fragment, LineHeight};
use crate::iced_compat::widget::{container, row, text as iced_text};
use crate::iced_compat::{Color, Element, Font, Length, Padding};
use shadcn_common::{LabelRecipe, TypeRecipe};

use super::style::SIDECAR_PX;
use super::{LabelContent, LabelStyleOverride};
use crate::fonts::iced_font;
use crate::recipes::iced_font_weight;
use crate::theme::Theme;

/// iced text glyphs sit above the geometric center of their layout box; nudge
/// when a sidecar icon shares the row.
const LABEL_OPTICAL_NUDGE_TOP: f32 = 1.5;

pub(super) fn build_content<'a, Message>(
    content: LabelContent<'a, Message>,
    icon_start: Option<Element<'a, Message>>,
    icon_end: Option<Element<'a, Message>>,
    recipe: LabelRecipe,
    text_color: Color,
    theme: &Theme,
) -> Element<'a, Message>
where
    Message: 'a,
{
    let typography = recipe.typography;
    let mut font = iced_font(theme.font_pack().sans);
    font.weight = iced_font_weight(typography.weight);

    let has_sidecar = icon_start.is_some() || icon_end.is_some();
    let nudge = has_sidecar;

    let label = match content {
        LabelContent::Text(fragment) => plain_label(fragment, typography, font, text_color, nudge),
        LabelContent::Element(content) => maybe_nudge(content, nudge),
    };

    if !has_sidecar {
        return label;
    }

    let mut children: Vec<Element<'a, Message>> = Vec::with_capacity(3);

    if let Some(icon) = icon_start {
        children.push(sidecar_slot(icon));
    }

    children.push(label);

    if let Some(icon) = icon_end {
        children.push(sidecar_slot(icon));
    }

    row(children)
        .spacing(recipe.gap_px)
        .align_y(Vertical::Center)
        .into()
}

fn plain_label<'a, Message: 'a>(
    fragment: Fragment<'a>,
    typography: TypeRecipe,
    font: Font,
    text_color: Color,
    nudge: bool,
) -> Element<'a, Message> {
    let size = typography.size_px;
    let line_height = LineHeight::Absolute(typography.line_height_px.into());

    // iced has no CSS `text-transform`; own the string so Sera can uppercase.
    let widget = if typography.uppercase {
        iced_text(fragment.as_ref().to_uppercase())
            .size(size)
            .font(font)
            .line_height(line_height)
            .color(text_color)
    } else {
        iced_text(fragment)
            .size(size)
            .font(font)
            .line_height(line_height)
            .color(text_color)
    };

    maybe_nudge(widget, nudge)
}

fn maybe_nudge<'a, Message: 'a>(
    content: impl Into<Element<'a, Message>>,
    nudge: bool,
) -> Element<'a, Message> {
    if nudge {
        container(content.into())
            .padding(Padding {
                top: LABEL_OPTICAL_NUDGE_TOP,
                ..Padding::ZERO
            })
            .into()
    } else {
        content.into()
    }
}

fn sidecar_slot<'a, Message: 'a>(content: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    container(content.into())
        .width(Length::Fixed(SIDECAR_PX))
        .height(Length::Fixed(SIDECAR_PX))
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
}

pub(super) fn wrap_interactive<'a, Message>(
    body: Element<'a, Message>,
    width: Length,
    text_color: Color,
    on_press: Message,
    disabled: bool,
    style_override: Option<LabelStyleOverride<'a>>,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let mut widget = iced_button(body)
        .padding(Padding::ZERO)
        .width(width)
        .on_press_maybe((!disabled).then_some(on_press));

    widget = widget.style(move |_iced_theme, status| {
        let mut style = transparent_button_style(text_color, status);

        if let Some(LabelStyleOverride::Button(override_fn)) = style_override.as_ref() {
            style = override_fn(style, status);
        }

        style
    });

    widget.into()
}

pub(super) fn wrap_static<'a, Message: 'a>(
    body: Element<'a, Message>,
    width: Length,
    style_override: Option<LabelStyleOverride<'a>>,
) -> Element<'a, Message> {
    let mut widget = container(body).width(width).padding(Padding::ZERO);

    widget = widget.style(move |_iced_theme| {
        let mut style = container::Style::default();

        if let Some(LabelStyleOverride::Container(override_fn)) = style_override.as_ref() {
            style = override_fn(style);
        }

        style
    });

    widget.into()
}

fn transparent_button_style(
    text_color: Color,
    status: button_widget::Status,
) -> button_widget::Style {
    let _ = status;
    button_widget::Style {
        background: None,
        text_color,
        border: crate::iced_compat::Border::default(),
        shadow: crate::iced_compat::Shadow::default(),
        snap: false,
    }
}
