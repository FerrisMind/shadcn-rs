use iced::widget::{container, rule};

use crate::theme::Theme;
use crate::tokens::{AccentColor, accent_color, mix};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SeparatorOrientation {
    #[default]
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SeparatorSize {
    Size1,
    Size2,
    Size3,
    #[default]
    Size4,
}

#[derive(Clone, Copy, Debug)]
pub struct SeparatorProps {
    pub orientation: SeparatorOrientation,
    pub size: SeparatorSize,
    pub color: Option<AccentColor>,
    pub custom_color: Option<iced::Color>,
    pub thickness: f32,
    pub gap: f32,
    pub length: Option<f32>,
    pub high_contrast: bool,
    pub decorative: bool,
    pub as_child: bool,
}

impl Default for SeparatorProps {
    fn default() -> Self {
        Self {
            orientation: SeparatorOrientation::Horizontal,
            size: SeparatorSize::default(),
            color: None,
            custom_color: None,
            thickness: 1.0,
            gap: 0.0,
            length: None,
            high_contrast: false,
            decorative: true,
            as_child: false,
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
        self.color = Some(color);
        self
    }

    pub fn custom_color(mut self, color: iced::Color) -> Self {
        self.custom_color = Some(color);
        self
    }

    pub fn thickness(mut self, thickness: f32) -> Self {
        self.thickness = thickness;
        self
    }

    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }

    pub fn length(mut self, length: f32) -> Self {
        self.length = Some(length);
        self
    }

    pub fn high_contrast(mut self, high_contrast: bool) -> Self {
        self.high_contrast = high_contrast;
        self
    }

    pub fn decorative(mut self, decorative: bool) -> Self {
        self.decorative = decorative;
        self
    }

    pub fn as_child(mut self, as_child: bool) -> Self {
        self.as_child = as_child;
        self
    }
}

fn separator_length(size: SeparatorSize) -> iced::Length {
    match size {
        SeparatorSize::Size1 => iced::Length::Fixed(16.0),
        SeparatorSize::Size2 => iced::Length::Fixed(24.0),
        SeparatorSize::Size3 => iced::Length::Fixed(48.0),
        SeparatorSize::Size4 => iced::Length::Fill,
    }
}

pub fn separator<'a, Message: 'a>(
    props: SeparatorProps,
    theme: &Theme,
) -> container::Container<'a, Message> {
    let palette = theme.palette;
    let radius = theme.radius.sm;

    let base_color = if let Some(color) = props.custom_color {
        color
    } else if let Some(c) = props.color {
        accent_color(&palette, c)
    } else if props.high_contrast {
        mix(palette.border, palette.foreground, 0.2)
    } else {
        palette.border
    };

    let style = move |_iced_theme: &iced::Theme| rule::Style {
        color: base_color,
        radius: radius.into(),
        fill_mode: rule::FillMode::Full,
        snap: true,
    };

    let length = if let Some(l) = props.length {
        iced::Length::Fixed(l)
    } else {
        separator_length(props.size)
    };
    let thickness = props.thickness;
    let (rule, width, height) = match props.orientation {
        SeparatorOrientation::Horizontal => (
            rule::horizontal(thickness).style(style),
            length,
            iced::Length::Fixed(thickness + props.gap * 2.0),
        ),
        SeparatorOrientation::Vertical => (
            rule::vertical(thickness).style(style),
            iced::Length::Fixed(thickness + props.gap * 2.0),
            length,
        ),
    };

    container(rule)
        .width(width)
        .height(height)
        .center_x(width)
        .center_y(height)
}
