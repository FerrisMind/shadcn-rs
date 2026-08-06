//! Content composition for the toggle component.

use crate::iced_compat::alignment::{Horizontal, Vertical};
use crate::iced_compat::widget::text::LineHeight;
use crate::iced_compat::widget::{container, row, text as iced_text};
use crate::iced_compat::{Element, Length};

use super::{ToggleContent, ToggleSize};
use crate::fonts::iced_font;
use crate::theme::Theme;

pub(super) fn build_content<'a, Message>(
    content: ToggleContent<'a, Message>,
    icon_start: Option<Element<'a, Message>>,
    icon_end: Option<Element<'a, Message>>,
    size: ToggleSize,
    theme: &Theme,
) -> Element<'a, Message>
where
    Message: 'a,
{
    let recipe = theme.style.toggle();
    let icon_px = size.recipe(theme).icon_px;

    let label = match content {
        ToggleContent::Label(label) => {
            let size_px = size.label_text_size(theme);
            let mut font = iced_font(theme.font_pack().sans);
            font.weight = crate::recipes::iced_font_weight(recipe.typography.weight);
            let text = if recipe.typography.uppercase {
                label.as_ref().to_uppercase()
            } else {
                label.into_owned()
            };

            iced_text(text)
                .size(size_px)
                .font(font)
                .line_height(LineHeight::Absolute(size_px.into()))
                .into()
        }
        ToggleContent::Element(content) => content,
        ToggleContent::Icon(content) => {
            return container(content)
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .into();
        }
    };

    if icon_start.is_none() && icon_end.is_none() {
        return label;
    }

    // `[&_svg]:size-*` + `[&_svg]:shrink-0`: icon slots keep a square footprint
    // so a label never squeezes them, and `gap-*` separates them from the text.
    let mut children: Vec<Element<'a, Message>> = Vec::with_capacity(3);

    if let Some(icon) = icon_start {
        children.push(icon_slot(icon, icon_px));
    }

    children.push(label);

    if let Some(icon) = icon_end {
        children.push(icon_slot(icon, icon_px));
    }

    row(children)
        .spacing(recipe.gap_px)
        .align_y(Vertical::Center)
        .into()
}

fn icon_slot<'a, Message: 'a>(
    content: impl Into<Element<'a, Message>>,
    icon_px: f32,
) -> Element<'a, Message> {
    container(content.into())
        .width(Length::Fixed(icon_px))
        .height(Length::Fixed(icon_px))
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
}

pub(super) fn build_wrapper<'a, Message: 'a>(
    content: Element<'a, Message>,
    full_width: bool,
    icon: bool,
) -> Element<'a, Message> {
    let mut wrapper = container(content)
        .width(Length::Shrink)
        .height(Length::Fill)
        .align_y(Vertical::Center);

    if full_width || icon {
        wrapper = wrapper.width(Length::Fill).align_x(Horizontal::Center);
    }

    wrapper.into()
}
