//! Content composition for badges (label, icons, loading spinner).

use crate::iced_compat::alignment::Vertical;
use crate::iced_compat::widget::text::{Fragment, LineHeight, Rich, Span};
use crate::iced_compat::widget::{container, hover, row, text as iced_text};
use crate::iced_compat::{Element, Font, Length, Padding};

use shadcn_common::AccentColor;

use super::super::spinner::{Spinner, SpinnerSize, spinner};
use super::style::{accent_text, label_color};
use super::{BadgeContent, BadgeVariant};
use crate::fonts::iced_font;
use crate::recipes::iced_font_weight;
use crate::theme::Theme;

/// iced text glyphs sit above the geometric center of their layout box; when
/// placed next to a canvas spinner / square icon that *is* geometrically
/// centered, the label reads high. Nudge the label down to match.
const LABEL_OPTICAL_NUDGE_TOP: f32 = 1.5;

pub(super) fn build_content<'a, Message>(
    content: BadgeContent<'a, Message>,
    icon_start: Option<Element<'a, Message>>,
    icon_end: Option<Element<'a, Message>>,
    variant: BadgeVariant,
    loading: bool,
    color: Option<AccentColor>,
    theme: &Theme,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let recipe = theme.style.badge();
    let mut font = iced_font(theme.font_pack().sans);
    font.weight = iced_font_weight(recipe.typography.weight);
    let size_px = recipe.typography.size_px;
    let text_color = label_color(theme, variant, color);
    let nudge = loading || icon_start.is_some() || icon_end.is_some() || color.is_some();
    let has_sidecar = loading || icon_start.is_some() || icon_end.is_some();
    let sidecar_px = recipe.icon_px;

    let label = match content {
        BadgeContent::Label(label) => {
            let text = if recipe.typography.uppercase {
                label.as_ref().to_uppercase()
            } else {
                label.into_owned()
            };
            if variant == BadgeVariant::Link {
                link_label(text.into(), size_px, font, nudge)
            } else {
                plain_label(text.into(), size_px, font, text_color, nudge)
            }
        }
        BadgeContent::Element(content) => maybe_nudge(content, nudge),
    };

    if !has_sidecar {
        return label;
    }

    let mut children: Vec<Element<'a, Message>> = Vec::with_capacity(3);

    if loading {
        children.push(sidecar_slot(
            spinner(
                Spinner::from_color(accent_text(theme, color))
                    .size(SpinnerSize::Xs)
                    .animated(true)
                    .loading(true),
            ),
            sidecar_px,
        ));
    } else if let Some(icon) = icon_start {
        children.push(sidecar_slot(icon, sidecar_px));
    }

    children.push(label);

    if let Some(icon) = icon_end {
        children.push(sidecar_slot(icon, sidecar_px));
    }

    row(children)
        .spacing(recipe.gap_px)
        .align_y(Vertical::Center)
        .into()
}

fn plain_label<'a, Message: 'a>(
    label: Fragment<'a>,
    size_px: f32,
    font: Font,
    text_color: crate::iced_compat::Color,
    nudge: bool,
) -> Element<'a, Message> {
    maybe_nudge(
        iced_text(label)
            .size(size_px)
            .font(font)
            .line_height(LineHeight::Absolute(size_px.into()))
            .color(text_color),
        nudge,
    )
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

fn sidecar_slot<'a, Message: 'a>(
    content: impl Into<Element<'a, Message>>,
    sidecar_px: f32,
) -> Element<'a, Message> {
    container(content.into())
        .width(Length::Fixed(sidecar_px))
        .height(Length::Fixed(sidecar_px))
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
}

fn link_label<'a, Message: 'a>(
    label: Fragment<'a>,
    size_px: f32,
    font: Font,
    nudge: bool,
) -> Element<'a, Message> {
    let size = size_px;
    let line_height = LineHeight::Absolute((size + 3.0).into());

    let base = Rich::<(), Message>::with_spans(vec![Span::new(label.clone())])
        .size(size)
        .font(font)
        .line_height(line_height);
    let underlined = Rich::<(), Message>::with_spans(vec![Span::new(label).underline(true)])
        .size(size)
        .font(font)
        .line_height(line_height);

    maybe_nudge(hover(base, underlined), nudge)
}

pub(super) fn build_wrapper<'a, Message: 'a>(
    content: Element<'a, Message>,
) -> Element<'a, Message> {
    container(content)
        .width(Length::Shrink)
        .center_y(Length::Fill)
        .into()
}
