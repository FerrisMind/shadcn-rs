use iced::border::Border;
use iced::widget::scrollable::{Direction, Scrollbar, Status, Style};
use iced::widget::{container, scrollable};
use iced::{Background, Color, Element, Length, Shadow};

use crate::button::ButtonRadius;
use crate::theme::Theme;
use crate::tokens::{AccentColor, accent_high, accent_low, is_dark};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScrollAreaSize {
    Size1,
    Size2,
    Size3,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScrollAreaScrollbars {
    Vertical,
    Horizontal,
    Both,
}

#[derive(Clone, Copy, Debug)]
pub struct ScrollAreaProps {
    pub size: ScrollAreaSize,
    pub radius: Option<ButtonRadius>,
    pub scrollbars: ScrollAreaScrollbars,
}

impl Default for ScrollAreaProps {
    fn default() -> Self {
        Self {
            size: ScrollAreaSize::Size1,
            radius: None,
            scrollbars: ScrollAreaScrollbars::Both,
        }
    }
}

impl ScrollAreaProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn size(mut self, size: ScrollAreaSize) -> Self {
        self.size = size;
        self
    }

    pub fn radius(mut self, radius: ButtonRadius) -> Self {
        self.radius = Some(radius);
        self
    }

    pub fn scrollbars(mut self, scrollbars: ScrollAreaScrollbars) -> Self {
        self.scrollbars = scrollbars;
        self
    }
}

fn apply_opacity(color: Color, opacity: f32) -> Color {
    Color {
        a: color.a * opacity,
        ..color
    }
}

fn scroll_area_radius(theme: &Theme, props: ScrollAreaProps) -> f32 {
    match props.radius {
        Some(ButtonRadius::None) => 0.0,
        Some(ButtonRadius::Small) => theme.radius.sm,
        Some(ButtonRadius::Medium) => theme.radius.md,
        Some(ButtonRadius::Large) => theme.radius.lg,
        Some(ButtonRadius::Full) => 9999.0,
        None => theme.radius.md,
    }
}

impl ScrollAreaSize {
    fn scrollbar_width(self) -> f32 {
        match self {
            ScrollAreaSize::Size1 => 4.0,
            ScrollAreaSize::Size2 => 8.0,
            ScrollAreaSize::Size3 => 12.0,
        }
    }
}

fn scroll_area_style(theme: &Theme, props: ScrollAreaProps, _status: Status) -> Style {
    let palette = theme.palette;
    let radius = scroll_area_radius(theme, props);

    let rail_bg = if is_dark(&palette) {
        apply_opacity(accent_low(&palette, AccentColor::Gray), 0.7)
    } else {
        apply_opacity(accent_low(&palette, AccentColor::Gray), 0.6)
    };
    let scroller_bg = if is_dark(&palette) {
        apply_opacity(accent_high(&palette, AccentColor::Gray), 0.9)
    } else {
        apply_opacity(accent_high(&palette, AccentColor::Gray), 0.8)
    };

    let rail = scrollable::Rail {
        background: Some(Background::Color(rail_bg)),
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: (radius.min(9999.0)).into(),
        },
        scroller: scrollable::Scroller {
            background: Background::Color(scroller_bg),
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: (radius.min(9999.0)).into(),
            },
        },
    };

    Style {
        container: container::Style {
            background: Some(Background::Color(palette.card)),
            text_color: Some(palette.card_foreground),
            border: Border {
                color: palette.border,
                width: 1.0,
                radius: radius.into(),
            },
            shadow: Shadow::default(),
            snap: true,
        },
        vertical_rail: rail,
        horizontal_rail: rail,
        gap: Some(Background::Color(rail_bg)),
        auto_scroll: scrollable::AutoScroll {
            background: Background::Color(apply_opacity(palette.background, 0.85)),
            border: Border {
                color: palette.border,
                width: 1.0,
                radius: radius.into(),
            },
            shadow: Shadow::default(),
            icon: palette.muted_foreground,
        },
    }
}

pub fn scroll_area<'a, Message: 'a>(
    content: impl Into<Element<'a, Message>>,
    props: ScrollAreaProps,
    theme: &Theme,
) -> scrollable::Scrollable<'a, Message> {
    let scrollbar_width = props.size.scrollbar_width();
    let scrollbar = Scrollbar::new()
        .width(scrollbar_width)
        .scroller_width(scrollbar_width)
        .margin(4.0);

    let direction = match props.scrollbars {
        ScrollAreaScrollbars::Vertical => Direction::Vertical(scrollbar),
        ScrollAreaScrollbars::Horizontal => Direction::Horizontal(scrollbar),
        ScrollAreaScrollbars::Both => Direction::Both {
            vertical: scrollbar,
            horizontal: scrollbar,
        },
    };

    let theme = theme.clone();
    scrollable(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .direction(direction)
        .style(move |_iced_theme, status| scroll_area_style(&theme, props, status))
}
