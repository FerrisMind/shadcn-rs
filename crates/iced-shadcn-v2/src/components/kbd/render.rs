//! Content composition for kbds (label, icon slots).

use crate::iced_compat::alignment::Vertical;
use crate::iced_compat::widget::text::{Fragment, LineHeight};
use crate::iced_compat::widget::{container, row, text as iced_text};
use crate::iced_compat::{Element, Length};

use super::KbdContent;
use super::geometry::{gap, icon_px};
use crate::fonts::iced_font;
use crate::theme::Theme;

pub(super) fn build_content<'a, Message>(
    content: KbdContent<'a, Message>,
    icon_start: Option<Element<'a, Message>>,
    icon_end: Option<Element<'a, Message>>,
    text_size: f32,
    theme: &Theme,
) -> Element<'a, Message>
where
    Message: 'a,
{
    let has_sidecar = icon_start.is_some() || icon_end.is_some();

    let label = match content {
        KbdContent::Label(label) => plain_label(label, text_size, theme),
        KbdContent::Element(content) => content,
    };

    if !has_sidecar {
        return label;
    }

    let mut children: Vec<Element<'a, Message>> = Vec::with_capacity(3);

    if let Some(icon) = icon_start {
        children.push(sidecar_slot(icon, icon_px(theme)));
    }

    children.push(label);

    if let Some(icon) = icon_end {
        children.push(sidecar_slot(icon, icon_px(theme)));
    }

    // Shrink-height row, vertically centered as a unit inside the kbd
    // (Tailwind `inline-flex items-center gap-1`).
    row(children)
        .spacing(gap(theme))
        .align_y(Vertical::Center)
        .into()
}

/// Key label — `font-sans text-xs`.
///
/// The color is intentionally not set: it is inherited from the styled kbd
/// container (`container::Style::text_color`), so surface styles and
/// [`super::Kbd::style_override`] both reach the glyphs.
fn plain_label<'a, Message: 'a>(
    label: Fragment<'a>,
    text_size: f32,
    theme: &Theme,
) -> Element<'a, Message> {
    iced_text(label)
        .size(text_size)
        .font(iced_font(theme.font_pack().sans))
        .line_height(LineHeight::Absolute(text_size.into()))
        .into()
}

/// Fixed square for icons so they share one geometric center
/// (shadcn `[&_svg]:size-3`).
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

/// Centers the composed content on the kbd midline (same approach as the
/// badge: the content box is `Fill`-height and centered, not top-padded).
pub(super) fn build_wrapper<'a, Message: 'a>(
    content: Element<'a, Message>,
) -> Element<'a, Message> {
    container(content)
        .width(Length::Shrink)
        .center_y(Length::Fill)
        .into()
}
