//! Content composition for typography blocks.

use crate::iced_compat::alignment;
use crate::iced_compat::widget::text::{Fragment, LineHeight};
use crate::iced_compat::widget::{column, container, row, text as iced_text};
use crate::iced_compat::{Background, Color, Element, Font, Length, Padding};

use super::style::{
    BLOCKQUOTE_BAR_PX, BLOCKQUOTE_INSET_PX, H2_UNDERLINE_GAP_PX, INLINE_CODE_PADDING_X_PX,
    INLINE_CODE_PADDING_Y_PX, RULE_PX,
};
use super::types::TypographyVariant;
use crate::recipes::component_radius_px;
use crate::theme::Theme;
use shadcn_common::ComponentRadius;

/// Resolved text metrics shared by every render path.
#[derive(Clone, Copy, Debug)]
pub(super) struct ResolvedText {
    pub size_px: f32,
    pub line_height_px: f32,
    pub color: Color,
    pub font: Font,
    pub align_x: alignment::Horizontal,
}

/// Renders a plain text fragment with the resolved metrics.
pub(super) fn text_block<'a, Message: 'a>(
    fragment: Fragment<'a>,
    resolved: ResolvedText,
    width: Length,
) -> Element<'a, Message> {
    iced_text(fragment)
        .size(resolved.size_px)
        .line_height(LineHeight::Absolute(resolved.line_height_px.into()))
        .font(resolved.font)
        .color(resolved.color)
        .width(width)
        .align_x(resolved.align_x)
        .into()
}

/// Wraps a body with the variant chrome (`h2` underline, blockquote bar,
/// inline-code chip). Plain variants pass through untouched.
pub(super) fn apply_chrome<'a, Message: 'a>(
    variant: TypographyVariant,
    body: Element<'a, Message>,
    theme: &Theme,
    width: Length,
) -> Element<'a, Message> {
    match variant {
        TypographyVariant::H2 => column![body, horizontal_rule(theme.palette.border)]
            .spacing(H2_UNDERLINE_GAP_PX)
            .width(width)
            .into(),
        TypographyVariant::Blockquote => {
            let bar_color = theme.palette.border;
            let bar = container(crate::iced_compat::widget::Space::new())
                .width(Length::Fixed(BLOCKQUOTE_BAR_PX))
                .height(Length::Fill)
                .style(move |_| solid(bar_color));

            row![bar, body]
                .spacing(BLOCKQUOTE_INSET_PX)
                .width(width)
                .into()
        }
        TypographyVariant::InlineCode => {
            let background = theme.palette.muted;
            let radius_token = if theme.style_id().locks_radius() {
                ComponentRadius::None
            } else {
                ComponentRadius::Sm
            };
            let radius = component_radius_px(theme, radius_token);

            container(body)
                .padding(Padding {
                    top: INLINE_CODE_PADDING_Y_PX,
                    bottom: INLINE_CODE_PADDING_Y_PX,
                    left: INLINE_CODE_PADDING_X_PX,
                    right: INLINE_CODE_PADDING_X_PX,
                })
                .style(move |_| container::Style {
                    background: Some(Background::Color(background)),
                    border: crate::iced_compat::Border {
                        radius: radius.into(),
                        ..crate::iced_compat::Border::default()
                    },
                    ..container::Style::default()
                })
                .into()
        }
        _ => body,
    }
}

/// Applies an optional top margin (`mt-*` in the web demo flow).
pub(super) fn apply_margin_top<'a, Message: 'a>(
    body: Element<'a, Message>,
    margin_top_px: f32,
) -> Element<'a, Message> {
    if margin_top_px > 0.0 {
        container(body)
            .padding(Padding {
                top: margin_top_px,
                ..Padding::ZERO
            })
            .into()
    } else {
        body
    }
}

/// 1 px full-width rule in `color` (`border-b` / table grid lines).
pub(super) fn horizontal_rule<'a, Message: 'a>(color: Color) -> Element<'a, Message> {
    container(crate::iced_compat::widget::Space::new())
        .width(Length::Fill)
        .height(Length::Fixed(RULE_PX))
        .style(move |_| solid(color))
        .into()
}

/// 1 px full-height rule in `color` (table column separators).
pub(super) fn vertical_rule<'a, Message: 'a>(color: Color) -> Element<'a, Message> {
    container(crate::iced_compat::widget::Space::new())
        .width(Length::Fixed(RULE_PX))
        .height(Length::Fill)
        .style(move |_| solid(color))
        .into()
}

fn solid(color: Color) -> container::Style {
    container::Style {
        background: Some(Background::Color(color)),
        ..container::Style::default()
    }
}
