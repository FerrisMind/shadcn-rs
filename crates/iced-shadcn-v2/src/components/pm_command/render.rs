//! Layout and semantic styling for [`super::PmCommand`].

use std::time::Duration;

use crate::components::button::{ButtonRadius, ButtonSize, ButtonVariant};
use crate::components::copy_button::{CopyButton, CopyButtonStatus};
use crate::components::scroll_area::{ScrollArea, ScrollAreaOrientation, ScrollAreaScrollbar};
use crate::components::tooltip::Tooltip;
use crate::fonts::iced_font;
use crate::iced_compat::alignment::Vertical;
use crate::iced_compat::widget::button;
use crate::iced_compat::widget::text::{LineHeight, Wrapping};
use crate::iced_compat::widget::{column, container, row, text};
use crate::iced_compat::{Background, Border, Color, Element, Length, Padding, Pixels};
use crate::recipes::{component_radius_px, iced_font_weight};
use crate::theme::Theme;
use shadcn_common::{ComponentRadius, FontWeight};

use super::icon::{ClipboardIcon, TerminalIcon};
use super::types::{PmCommandAgent, PmCommandRadius, PmCommandVariant};

const HEADER_TOP_BOTTOM_PADDING: f32 = 4.0;
const HEADER_RIGHT_PADDING: f32 = 8.0;
const HEADER_GAP: f32 = 8.0;
const LEFT_INSET: f32 = 8.0;
const BODY_PADDING: f32 = 12.0;
const TAB_HEIGHT: f32 = 28.0;
const TAB_PADDING_X: f32 = 8.0;
const TAB_PADDING_Y: f32 = 4.0;
const COMMAND_SIZE: f32 = 14.0;
const COMMAND_LINE_HEIGHT: f32 = 14.0;
const TERMINAL_BOX_SIZE: f32 = 16.0;
const TERMINAL_ICON_SIZE: f32 = 12.0;
const COPY_BUTTON_SIZE: f32 = 24.0;
const COPY_ICON_SIZE: f32 = 12.0;

/// Builds the complete PMCommand body after the public builder has resolved
/// the selected agent and command text.
#[allow(clippy::too_many_arguments)]
pub(super) fn build<'a, Message>(
    theme: &'a Theme,
    variant: PmCommandVariant,
    radius: PmCommandRadius,
    width: Length,
    max_width: Option<f32>,
    agents: &[PmCommandAgent],
    active_agent: &PmCommandAgent,
    command_text: &str,
    copy_status: CopyButtonStatus,
    copy_animation_duration: Duration,
    on_copy: Option<Message>,
    on_agent_change: Option<&dyn Fn(PmCommandAgent) -> Message>,
    style_override: Option<Box<dyn Fn(container::Style) -> container::Style + 'a>>,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let terminal = container(TerminalIcon::element(
        theme.palette.background,
        TERMINAL_ICON_SIZE,
    ))
    // `center_x`/`center_y` also set the container's size. Passing `Fill`
    // here makes the `bg-foreground size-4` terminal tile from the Svelte
    // component expand across the entire header.
    .center(Length::Fixed(TERMINAL_BOX_SIZE))
    .style(move |_iced_theme| container::Style {
        background: Some(Background::Color(with_alpha(theme.palette.foreground, 0.5))),
        ..container::Style::default()
    });

    let tabs = container(row(agents.iter().cloned().map(|agent| {
        let message = on_agent_change.map(|callback| callback(agent.clone()));
        build_agent_tab(theme, agent, active_agent, message)
    })))
    .center_y(Length::Fixed(36.0));

    let left = row![terminal, tabs]
        .spacing(HEADER_GAP)
        .align_y(Vertical::Center)
        .padding(Padding {
            top: 0.0,
            right: 0.0,
            bottom: 0.0,
            left: LEFT_INSET,
        });

    let copy = build_copy_button(
        theme,
        command_text,
        copy_status,
        copy_animation_duration,
        on_copy,
    );

    let header = container(
        row![left, copy]
            .width(Length::Fill)
            .spacing(HEADER_GAP)
            .align_y(Vertical::Center),
    )
    .width(Length::Fill)
    .padding(Padding {
        top: HEADER_TOP_BOTTOM_PADDING,
        right: HEADER_RIGHT_PADDING,
        bottom: HEADER_TOP_BOTTOM_PADDING,
        left: 0.0,
    });

    let separator = container(crate::iced_compat::widget::space())
        .width(Length::Fill)
        .height(Length::Fixed(1.0))
        .style(move |_iced_theme| container::Style {
            background: Some(Background::Color(theme.palette.border)),
            ..container::Style::default()
        });

    let body = build_command_body(theme, command_text);

    let root = container(column![header, separator, body])
        .width(width)
        .style(move |_iced_theme| {
            let mut style = root_style(theme, variant, radius);
            if let Some(ref style_override) = style_override {
                style = style_override(style);
            }
            style
        });

    if let Some(max_width) = max_width {
        root.max_width(max_width)
    } else {
        root
    }
    .into()
}

fn build_agent_tab<'a, Message>(
    theme: &'a Theme,
    agent: PmCommandAgent,
    active_agent: &PmCommandAgent,
    on_press: Option<Message>,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let active = &agent == active_agent;
    let mut font = iced_font(theme.style.font_pack.mono);
    font.weight = iced_font_weight(FontWeight::Light);

    button(
        text(agent.as_str().to_owned())
            .font(font)
            .size(COMMAND_SIZE)
            .line_height(LineHeight::Absolute(Pixels(20.0)))
            .wrapping(Wrapping::None),
    )
    .width(Length::Shrink)
    .height(Length::Fixed(TAB_HEIGHT))
    .padding(Padding {
        top: TAB_PADDING_Y,
        right: TAB_PADDING_X,
        bottom: TAB_PADDING_Y,
        left: TAB_PADDING_X,
    })
    .on_press_maybe(on_press)
    .style(move |_iced_theme, status| tab_style(theme, active, status))
    .into()
}

fn tab_style(theme: &Theme, active: bool, status: button::Status) -> button::Style {
    let hovered = matches!(status, button::Status::Hovered | button::Status::Pressed);
    let text_color = if active || hovered {
        theme.palette.foreground
    } else if theme.is_dark() {
        theme.palette.muted_foreground
    } else {
        with_alpha(theme.palette.foreground, 0.60)
    };

    let background = if active {
        if theme.is_dark() {
            Some(Background::Color(with_alpha(theme.palette.input, 0.30)))
        } else {
            Some(Background::Color(theme.palette.background))
        }
    } else {
        None
    };

    let border = if active && theme.is_dark() {
        Border {
            color: theme.palette.input,
            width: 1.0,
            radius: component_radius_px(theme, ComponentRadius::Sm).into(),
        }
    } else {
        Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: component_radius_px(theme, ComponentRadius::Sm).into(),
        }
    };

    button::Style {
        background,
        text_color,
        border,
        shadow: Default::default(),
        snap: true,
    }
}

fn build_copy_button<'a, Message>(
    theme: &'a Theme,
    command_text: &str,
    status: CopyButtonStatus,
    animation_duration: Duration,
    on_copy: Option<Message>,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let copy = CopyButton::new(command_text.to_owned(), theme)
        .variant(ButtonVariant::Ghost)
        .size(ButtonSize::Icon)
        .radius(ButtonRadius::Small)
        .width(Length::Fixed(COPY_BUTTON_SIZE))
        .height(Length::Fixed(COPY_BUTTON_SIZE))
        .icon(ClipboardIcon::element(
            theme.palette.foreground,
            COPY_ICON_SIZE,
        ))
        .status(status)
        .animation_duration(animation_duration)
        .on_copy_maybe(on_copy)
        .style_override(|mut style, status| {
            // A Rust application may intentionally omit the copy message and
            // handle clipboard work elsewhere. Keep the reference ghost
            // appearance in that case instead of painting an unavailable
            // button as a muted disabled control.
            if matches!(status, button::Status::Disabled) {
                style.background = None;
                style.border.width = 0.0;
                style.border.color = Color::TRANSPARENT;
            }
            style
        });

    Tooltip::text(copy, "Copy to Clipboard", theme).into()
}

fn build_command_body<'a, Message>(theme: &'a Theme, command_text: &str) -> Element<'a, Message>
where
    Message: 'a,
{
    let mut font = iced_font(theme.style.font_pack.mono);
    font.weight = iced_font_weight(FontWeight::Light);

    let command = text(command_text.to_owned())
        .font(font)
        .size(COMMAND_SIZE)
        .line_height(LineHeight::Absolute(Pixels(COMMAND_LINE_HEIGHT)))
        .wrapping(Wrapping::None)
        .color(theme.palette.muted_foreground);

    let scroll = ScrollArea::new(command, theme)
        .orientation(ScrollAreaOrientation::Horizontal)
        .vertical_scrollbar(ScrollAreaScrollbar::hidden())
        .horizontal_scrollbar(ScrollAreaScrollbar::hidden())
        .width(Length::Fill)
        .height(Length::Fixed(20.0));

    container(scroll)
        .width(Length::Fill)
        .padding(BODY_PADDING)
        .into()
}

fn root_style(
    theme: &Theme,
    variant: PmCommandVariant,
    radius: PmCommandRadius,
) -> container::Style {
    let (background, border_color) = match variant {
        PmCommandVariant::Default => (theme.palette.card, theme.palette.border),
        PmCommandVariant::Secondary => (
            with_alpha(theme.palette.secondary, 0.50),
            Color::TRANSPARENT,
        ),
    };

    container::Style {
        background: Some(Background::Color(background)),
        border: Border {
            color: border_color,
            width: 1.0,
            radius: radius_px(theme, radius).into(),
        },
        ..container::Style::default()
    }
}

fn radius_px(theme: &Theme, radius: PmCommandRadius) -> f32 {
    match radius {
        PmCommandRadius::Default => {
            component_radius_px(theme, theme.style.button_type().default_radius)
        }
        PmCommandRadius::None => 0.0,
        PmCommandRadius::Small => component_radius_px(theme, ComponentRadius::Sm),
        PmCommandRadius::Medium => component_radius_px(theme, ComponentRadius::Md),
        PmCommandRadius::Large => component_radius_px(theme, ComponentRadius::Lg),
        PmCommandRadius::Full => 9999.0,
    }
}

fn with_alpha(color: Color, alpha: f32) -> Color {
    Color {
        a: (color.a * alpha).clamp(0.0, 1.0),
        ..color
    }
}
