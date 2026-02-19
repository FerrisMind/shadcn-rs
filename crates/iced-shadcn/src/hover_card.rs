use iced::border::Border;
use iced::widget::{container, tooltip as tooltip_widget};
use iced::{Background, Element};

use crate::theme::Theme;
use crate::tooltip::TooltipPosition;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HoverCardSize {
    Size1,
    Size2,
    Size3,
}

#[derive(Clone, Copy, Debug)]
pub struct HoverCardProps {
    pub size: HoverCardSize,
    pub position: TooltipPosition,
    pub gap: f32,
    pub open_delay_ms: u64,
    pub snap_within_viewport: bool,
    pub max_width: u32,
}

impl Default for HoverCardProps {
    fn default() -> Self {
        Self {
            size: HoverCardSize::Size2,
            position: TooltipPosition::Bottom,
            gap: 8.0,
            open_delay_ms: 200,
            snap_within_viewport: true,
            max_width: 480,
        }
    }
}

impl HoverCardProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn size(mut self, size: HoverCardSize) -> Self {
        self.size = size;
        self
    }

    pub fn position(mut self, position: TooltipPosition) -> Self {
        self.position = position;
        self
    }

    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap.max(0.0);
        self
    }

    pub fn open_delay_ms(mut self, open_delay_ms: u64) -> Self {
        self.open_delay_ms = open_delay_ms;
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

fn padding_px(theme: &Theme, size: HoverCardSize) -> u16 {
    let px = match size {
        HoverCardSize::Size1 => theme.spacing.md,
        HoverCardSize::Size2 => theme.spacing.lg,
        HoverCardSize::Size3 => theme.spacing.lg + theme.spacing.xs,
    };
    px.round().max(0.0) as u16
}

fn radius_px(theme: &Theme, size: HoverCardSize) -> f32 {
    match size {
        HoverCardSize::Size1 | HoverCardSize::Size2 => theme.radius.md,
        HoverCardSize::Size3 => theme.radius.lg,
    }
}

pub fn hover_card<'a, Message: 'a>(
    content: impl Into<Element<'a, Message>>,
    card: impl Into<Element<'a, Message>>,
    props: HoverCardProps,
    theme: &Theme,
) -> tooltip_widget::Tooltip<'a, Message> {
    let theme = theme.clone();
    let padding = padding_px(&theme, props.size);
    let radius = radius_px(&theme, props.size);

    let hover_content = container(card)
        .padding(padding)
        .max_width(props.max_width)
        .style(
            move |_iced_theme: &iced::Theme| iced::widget::container::Style {
                background: Some(Background::Color(theme.palette.popover)),
                text_color: Some(theme.palette.popover_foreground),
                border: Border {
                    color: theme.palette.border,
                    width: 1.0,
                    radius: radius.into(),
                },
                shadow: iced::Shadow {
                    color: iced::Color {
                        a: 0.18,
                        ..iced::Color::BLACK
                    },
                    offset: iced::Vector::new(0.0, 8.0),
                    blur_radius: 22.0,
                },
                snap: true,
            },
        );

    tooltip_widget::Tooltip::new(content, hover_content, props.position.into())
        .gap(props.gap)
        .padding(0)
        .delay(iced::time::Duration::from_millis(props.open_delay_ms))
        .snap_within_viewport(props.snap_within_viewport)
}
