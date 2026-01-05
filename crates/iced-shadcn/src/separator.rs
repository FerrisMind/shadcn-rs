use iced::widget::{container, rule};

use crate::theme::Theme;
use crate::tokens::{AccentColor, accent_color};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SeparatorOrientation {
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SeparatorSize {
    One,
    Two,
    Three,
    Four,
}

#[derive(Clone, Copy, Debug)]
pub struct SeparatorProps {
    pub orientation: SeparatorOrientation,
    pub size: SeparatorSize,
    pub color: AccentColor,
    pub decorative: bool,
}

impl Default for SeparatorProps {
    fn default() -> Self {
        Self {
            orientation: SeparatorOrientation::Horizontal,
            size: SeparatorSize::One,
            color: AccentColor::Gray,
            decorative: true,
        }
    }
}

impl SeparatorProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn orientation(mut self, orientation: SeparatorOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn size(mut self, size: SeparatorSize) -> Self {
        self.size = size;
        self
    }

    pub fn color(mut self, color: AccentColor) -> Self {
        self.color = color;
        self
    }

    pub fn decorative(mut self, decorative: bool) -> Self {
        self.decorative = decorative;
        self
    }
}

fn separator_length(size: SeparatorSize) -> iced::Length {
    match size {
        SeparatorSize::One => iced::Length::Fixed(16.0),
        SeparatorSize::Two => iced::Length::Fixed(32.0),
        SeparatorSize::Three => iced::Length::Fixed(64.0),
        SeparatorSize::Four => iced::Length::Fill,
    }
}

pub fn separator<'a, Message: 'a>(
    props: SeparatorProps,
    theme: &Theme,
) -> container::Container<'a, Message> {
    let palette = theme.palette;
    let radius = theme.radius.sm;
    let color = if matches!(props.color, AccentColor::Gray) {
        palette.border
    } else {
        accent_color(&palette, props.color)
    };

    let style = move |_iced_theme: &iced::Theme| rule::Style {
        color,
        radius: radius.into(),
        fill_mode: rule::FillMode::Full,
        snap: true,
    };

    let length = separator_length(props.size);
    let (rule, width, height) = match props.orientation {
        SeparatorOrientation::Horizontal => (
            rule::horizontal(1).style(style),
            length,
            iced::Length::Shrink,
        ),
        SeparatorOrientation::Vertical => {
            (rule::vertical(1).style(style), iced::Length::Shrink, length)
        }
    };

    container(rule).width(width).height(height)
}
