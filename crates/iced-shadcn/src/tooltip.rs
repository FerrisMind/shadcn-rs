use iced::border::Border;
use iced::widget::{container, tooltip as tooltip_widget};
use iced::{Background, Element};

use crate::theme::Theme;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TooltipPosition {
    Top,
    Bottom,
    Left,
    Right,
    FollowCursor,
}

impl From<TooltipPosition> for tooltip_widget::Position {
    fn from(value: TooltipPosition) -> Self {
        match value {
            TooltipPosition::Top => tooltip_widget::Position::Top,
            TooltipPosition::Bottom => tooltip_widget::Position::Bottom,
            TooltipPosition::Left => tooltip_widget::Position::Left,
            TooltipPosition::Right => tooltip_widget::Position::Right,
            TooltipPosition::FollowCursor => tooltip_widget::Position::FollowCursor,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct TooltipProps {
    pub position: TooltipPosition,
    pub gap: f32,
    pub delay_ms: u64,
    pub snap_within_viewport: bool,
    pub max_width: u32,
}

impl Default for TooltipProps {
    fn default() -> Self {
        Self {
            position: TooltipPosition::Top,
            gap: 4.0,
            delay_ms: 0,
            snap_within_viewport: true,
            max_width: 360,
        }
    }
}

impl TooltipProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn position(mut self, position: TooltipPosition) -> Self {
        self.position = position;
        self
    }

    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap.max(0.0);
        self
    }

    pub fn delay_ms(mut self, delay_ms: u64) -> Self {
        self.delay_ms = delay_ms;
        self
    }

    pub fn snap_within_viewport(mut self, snap: bool) -> Self {
        self.snap_within_viewport = snap;
        self
    }

    pub fn max_width(mut self, max_width: u32) -> Self {
        self.max_width = max_width.max(1);
        self
    }
}

pub fn tooltip<'a, Message: 'a>(
    content: impl Into<Element<'a, Message>>,
    tip: impl Into<Element<'a, Message>>,
    props: TooltipProps,
    theme: &Theme,
) -> tooltip_widget::Tooltip<'a, Message> {
    let theme = theme.clone();
    let tooltip_content = container(tip).padding(8).max_width(props.max_width).style(
        move |_iced_theme: &iced::Theme| iced::widget::container::Style {
            background: Some(Background::Color(theme.palette.foreground)),
            text_color: Some(theme.palette.background),
            border: Border::default(),
            shadow: iced::Shadow {
                color: iced::Color {
                    a: 0.18,
                    ..iced::Color::BLACK
                },
                offset: iced::Vector::new(0.0, 6.0),
                blur_radius: 18.0,
            },
            snap: true,
        },
    );

    tooltip_widget::Tooltip::new(content, tooltip_content, props.position.into())
        .gap(props.gap)
        .padding(0)
        .delay(iced::time::Duration::from_millis(props.delay_ms))
        .snap_within_viewport(props.snap_within_viewport)
}
