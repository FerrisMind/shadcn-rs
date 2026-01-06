use iced::Background;
use iced::border::Border;
use iced::widget::slider as slider_widget;
use iced::widget::vertical_slider as vertical_slider_widget;

use crate::button::ButtonRadius;
use crate::theme::Theme;
use crate::tokens::{AccentColor, accent_color, accent_soft};

#[derive(Clone, Copy, Debug)]
pub enum SliderSize {
    One,
    Two,
    Three,
}

#[derive(Clone, Copy, Debug)]
pub enum SliderVariant {
    Classic,
    Surface,
    Soft,
}

#[derive(Clone, Copy, Debug)]
pub struct SliderProps {
    pub size: SliderSize,
    pub variant: SliderVariant,
    pub color: AccentColor,
    pub radius: Option<ButtonRadius>,
    pub high_contrast: bool,
}

impl Default for SliderProps {
    fn default() -> Self {
        Self {
            size: SliderSize::Two,
            variant: SliderVariant::Surface,
            color: AccentColor::Gray,
            radius: None,
            high_contrast: false,
        }
    }
}

impl SliderProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn size(mut self, size: SliderSize) -> Self {
        self.size = size;
        self
    }

    pub fn variant(mut self, variant: SliderVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn color(mut self, color: AccentColor) -> Self {
        self.color = color;
        self
    }

    pub fn radius(mut self, radius: ButtonRadius) -> Self {
        self.radius = Some(radius);
        self
    }

    pub fn high_contrast(mut self, high_contrast: bool) -> Self {
        self.high_contrast = high_contrast;
        self
    }
}

impl SliderSize {
    fn rail_height(self) -> f32 {
        match self {
            SliderSize::One => 4.0,
            SliderSize::Two => 6.0,
            SliderSize::Three => 8.0,
        }
    }

    fn handle_radius(self) -> f32 {
        match self {
            SliderSize::One => 6.0,
            SliderSize::Two => 8.0,
            SliderSize::Three => 10.0,
        }
    }
}

fn slider_radius(theme: &Theme, props: SliderProps) -> f32 {
    match props.radius {
        Some(ButtonRadius::None) => 0.0,
        Some(ButtonRadius::Small) => theme.radius.sm,
        Some(ButtonRadius::Medium) => theme.radius.md,
        Some(ButtonRadius::Large) => theme.radius.lg,
        Some(ButtonRadius::Full) => 9999.0,
        None => 9999.0,
    }
}

pub fn slider<'a, Message: Clone + 'a, T, F>(
    range: std::ops::RangeInclusive<T>,
    value: T,
    on_change: F,
    props: SliderProps,
    theme: &Theme,
) -> slider_widget::Slider<'a, T, Message>
where
    T: Copy + From<u8> + PartialOrd,
    F: Fn(T) -> Message + 'a,
{
    let theme = theme.clone();
    slider_widget::Slider::new(range, value, on_change)
        .style(move |_iced_theme, status| slider_style(&theme, props, status))
}

pub fn vertical_slider<'a, Message: Clone + 'a, T, F>(
    range: std::ops::RangeInclusive<T>,
    value: T,
    on_change: F,
    props: SliderProps,
    theme: &Theme,
) -> vertical_slider_widget::VerticalSlider<'a, T, Message>
where
    T: Copy + From<u8> + PartialOrd,
    F: Fn(T) -> Message + 'a,
{
    let theme = theme.clone();
    vertical_slider_widget::VerticalSlider::new(range, value, on_change)
        .style(move |_iced_theme, status| slider_style(&theme, props, status))
}

fn slider_style(
    theme: &Theme,
    props: SliderProps,
    status: slider_widget::Status,
) -> slider_widget::Style {
    let palette = theme.palette;
    let accent = accent_color(&palette, props.color);
    let soft_bg = accent_soft(&palette, props.color);
    let radius = slider_radius(theme, props);
    let range_color = if props.high_contrast {
        palette.foreground
    } else {
        accent
    };

    let rail_background = match props.variant {
        SliderVariant::Soft => soft_bg,
        SliderVariant::Classic | SliderVariant::Surface => palette.muted,
    };

    let rail = slider_widget::Rail {
        backgrounds: (Background::Color(rail_background), Background::Color(range_color)),
        width: props.size.rail_height(),
        border: Border {
            radius: radius.into(),
            width: 0.0,
            color: palette.border,
        },
    };

    let (handle_bg, handle_border) = match status {
        slider_widget::Status::Hovered | slider_widget::Status::Dragged => {
            (Background::Color(iced::Color::WHITE), palette.ring)
        }
        slider_widget::Status::Active => (Background::Color(iced::Color::WHITE), range_color),
    };

    slider_widget::Style {
        rail,
        handle: slider_widget::Handle {
            shape: slider_widget::HandleShape::Circle {
                radius: props.size.handle_radius(),
            },
            background: handle_bg,
            border_width: 1.0,
            border_color: handle_border,
        },
    }
}
