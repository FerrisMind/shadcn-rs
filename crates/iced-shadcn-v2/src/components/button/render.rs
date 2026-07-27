//! Content composition and loading-state rendering.

use iced::alignment::{Horizontal, Vertical};
use iced::widget::text::{Fragment, LineHeight, Rich, Span};
use iced::widget::{container, hover, row, text as iced_text};
use iced::{Element, Font, Length};

use shadcn_common::AccentColor;

use super::super::spinner::{Spinner, SpinnerSize, spinner};
use super::style::accent_text;
use super::{ButtonContent, ButtonSize};
use crate::fonts::iced_font;
use crate::theme::Theme;

pub(super) fn build_content<'a, Message>(
    content: ButtonContent<'a, Message>,
    variant: super::ButtonVariant,
    size: ButtonSize,
    loading: bool,
    color: Option<AccentColor>,
    theme: &Theme,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let is_icon = matches!(content, ButtonContent::Icon(_));

    let content = match content {
        ButtonContent::Label(label) => {
            let size_px = size.label_text_size();
            let font = iced_font(theme.font_pack().sans);
            let line_height = LineHeight::Absolute(f32::from(size_px).into());

            if variant == super::ButtonVariant::Link {
                link_label(label, size_px, font)
            } else {
                iced_text(label)
                    .size(u32::from(size_px))
                    .font(font)
                    .line_height(line_height)
                    .into()
            }
        }
        ButtonContent::Element(content) => content,
        ButtonContent::Icon(content) => container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into(),
    };

    if loading {
        loading_content(content, size, color, theme, is_icon)
    } else {
        content
    }
}

fn link_label<'a, Message: 'a>(
    label: Fragment<'a>,
    size_px: u16,
    font: Font,
) -> Element<'a, Message> {
    let size = f32::from(size_px);
    // Leave room under the glyphs — iced `hover` layers clip to layout bounds,
    // so a tight Absolute line-height would crop `Span::underline`.
    let line_height = LineHeight::Absolute((size + 3.0).into());

    let base = Rich::<(), Message>::with_spans(vec![Span::new(label.clone())])
        .size(size)
        .font(font)
        .line_height(line_height);
    let underlined = Rich::<(), Message>::with_spans(vec![Span::new(label).underline(true)])
        .size(size)
        .font(font)
        .line_height(line_height);

    // Fill the button content box so hover tracks the whole control, not just
    // the tight text metrics (padding / vertical centering still apply outside).
    container(hover(base, underlined))
        .width(Length::Shrink)
        .height(Length::Fill)
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

/// Loading UI matching shadcn: animated spinner inline-start of the label.
///
/// Icon-only buttons replace their glyph with the spinner (same footprint).
fn loading_content<'a, Message>(
    content: Element<'a, Message>,
    size: ButtonSize,
    color: Option<AccentColor>,
    theme: &Theme,
    is_icon: bool,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let spinner_size = match size {
        ButtonSize::Size0 | ButtonSize::Size1 => SpinnerSize::Size1,
        ButtonSize::Size2 => SpinnerSize::Size2,
        ButtonSize::Size3 | ButtonSize::Size4 => SpinnerSize::Size3,
    };

    let indicator = spinner(
        Spinner::from_color(accent_text(theme, color))
            .size(spinner_size)
            .animated(true)
            .loading(true),
    );

    if is_icon {
        return container(indicator)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into();
    }

    let gap = match size {
        ButtonSize::Size0 | ButtonSize::Size1 => 6.0,
        ButtonSize::Size2 => 8.0,
        ButtonSize::Size3 | ButtonSize::Size4 => 8.0,
    };

    row![indicator, content]
        .spacing(gap)
        .align_y(Vertical::Center)
        .into()
}
