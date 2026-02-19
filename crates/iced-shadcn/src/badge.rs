use iced::border::Border;
use iced::widget::{container, text};
use iced::{Background, Color, Shadow};

use crate::button::ButtonRadius;
use crate::theme::Theme;
use crate::tokens::AccentColor;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum BadgeSize {
    #[default]
    Size1,
    Size2,
    Size3,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum BadgeVariant {
    #[default]
    Default,
    Secondary,
    Outline,
    Destructive,
}

#[derive(Clone, Copy, Debug)]
pub struct BadgeProps {
    pub size: BadgeSize,
    pub variant: BadgeVariant,
    pub color: Option<AccentColor>,
    pub radius: Option<ButtonRadius>,
    pub high_contrast: bool,
}

impl Default for BadgeProps {
    fn default() -> Self {
        Self {
            size: BadgeSize::Size1,
            variant: BadgeVariant::Default,
            color: None,
            radius: None,
            high_contrast: false,
        }
    }
}

impl BadgeProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn size(mut self, size: BadgeSize) -> Self {
        self.size = size;
        self
    }

    pub fn variant(mut self, variant: BadgeVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn color(mut self, color: AccentColor) -> Self {
        self.color = Some(color);
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

fn badge_radius(theme: &Theme, props: BadgeProps) -> f32 {
    match props.radius {
        Some(ButtonRadius::None) => 0.0,
        Some(ButtonRadius::Small) => theme.radius.sm,
        Some(ButtonRadius::Medium) => theme.radius.md,
        Some(ButtonRadius::Large) => theme.radius.lg,
        Some(ButtonRadius::Full) => 9999.0,
        None => 9999.0, // Default to pill for badges
    }
}

impl BadgeSize {
    fn padding(self) -> [f32; 2] {
        match self {
            BadgeSize::Size1 => [2.0, 6.0],
            BadgeSize::Size2 => [3.0, 8.0],
            BadgeSize::Size3 => [4.0, 10.0],
        }
    }

    fn text_size(self) -> u16 {
        match self {
            BadgeSize::Size1 => 11,
            BadgeSize::Size2 => 12,
            BadgeSize::Size3 => 13,
        }
    }
}

fn apply_opacity(color: Color, opacity: f32) -> Color {
    Color {
        a: color.a * opacity,
        ..color
    }
}

pub fn badge<'a, Message: 'a>(
    label: impl Into<String>,
    props: BadgeProps,
    theme: &Theme,
) -> container::Container<'a, Message> {
    let palette = theme.palette;
    let radius = badge_radius(theme, props);

    let (background_color, text_color, border_color) = match props.variant {
        BadgeVariant::Default => {
            let color = match props.color {
                Some(c) => crate::tokens::accent_color(&palette, c),
                None => palette.primary,
            };
            (color, palette.primary_foreground, color)
        }
        BadgeVariant::Secondary => {
            let color = match props.color {
                Some(c) => crate::tokens::accent_color(&palette, c),
                None => palette.secondary,
            };
            (color, palette.secondary_foreground, color)
        }
        BadgeVariant::Destructive => {
            let color = match props.color {
                Some(c) => crate::tokens::accent_color(&palette, c),
                None => palette.destructive,
            };
            (color, palette.destructive_foreground, color)
        }
        BadgeVariant::Outline => (
            Color::TRANSPARENT,
            palette.foreground,
            palette.border,
        ),
    };

    let shadow = if matches!(props.variant, BadgeVariant::Default) && props.high_contrast {
        Shadow {
            color: apply_opacity(Color::BLACK, 0.08),
            offset: iced::Vector::new(0.0, 1.0),
            blur_radius: 6.0,
        }
    } else {
        Shadow::default()
    };

    let style = move |_iced_theme: &iced::Theme| container::Style {
        background: Some(Background::Color(background_color)),
        text_color: Some(text_color),
        border: Border {
            color: border_color,
            width: if border_color.a > 0.0 { 1.0 } else { 0.0 },
            radius: radius.into(),
        },
        shadow,
        snap: true,
    };

    container(
        text(label.into())
            .size(props.size.text_size() as u32)
            .style(move |_theme: &iced::Theme| iced::widget::text::Style {
                color: Some(text_color),
            }),
    )
    .padding(props.size.padding())
    .style(style)
}
