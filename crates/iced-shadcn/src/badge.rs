use iced::border::Border;
use iced::widget::{container, text};
use iced::{Background, Color, Shadow};

use crate::button::ButtonRadius;
use crate::theme::Theme;
use crate::tokens::{
    AccentColor, accent_color, accent_foreground, accent_soft, accent_soft_foreground, accent_text,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BadgeSize {
    One,
    Two,
    Three,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BadgeVariant {
    Solid,
    Soft,
    Surface,
    Outline,
}

#[derive(Clone, Copy, Debug)]
pub struct BadgeProps {
    pub size: BadgeSize,
    pub variant: BadgeVariant,
    pub color: AccentColor,
    pub radius: Option<ButtonRadius>,
    pub high_contrast: bool,
}

impl Default for BadgeProps {
    fn default() -> Self {
        Self {
            size: BadgeSize::One,
            variant: BadgeVariant::Soft,
            color: AccentColor::Gray,
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

fn badge_radius(theme: &Theme, props: BadgeProps) -> f32 {
    match props.radius {
        Some(ButtonRadius::None) => 0.0,
        Some(ButtonRadius::Small) => theme.radius.sm,
        Some(ButtonRadius::Medium) => theme.radius.md,
        Some(ButtonRadius::Large) => theme.radius.lg,
        Some(ButtonRadius::Full) => 9999.0,
        None => theme.radius.sm,
    }
}

impl BadgeSize {
    fn padding(self) -> [f32; 2] {
        match self {
            BadgeSize::One => [2.0, 6.0],
            BadgeSize::Two => [3.0, 8.0],
            BadgeSize::Three => [4.0, 10.0],
        }
    }

    fn text_size(self) -> u32 {
        match self {
            BadgeSize::One => 11,
            BadgeSize::Two => 12,
            BadgeSize::Three => 13,
        }
    }
}

fn apply_opacity(color: Color, opacity: f32) -> Color {
    Color { a: color.a * opacity, ..color }
}

pub fn badge<'a, Message: 'a>(
    label: impl Into<String>,
    props: BadgeProps,
    theme: &Theme,
) -> container::Container<'a, Message> {
    let palette = theme.palette;
    let radius = badge_radius(theme, props);

    let accent = accent_color(&palette, props.color);
    let accent_fg = accent_foreground(&palette, props.color);
    let accent_text_color = accent_text(&palette, props.color);
    let soft_bg = accent_soft(&palette, props.color);
    let soft_fg = accent_soft_foreground(&palette, props.color);

    let (background, text_color, border_color) = match props.variant {
        BadgeVariant::Solid => (Background::Color(accent), accent_fg, accent),
        BadgeVariant::Soft => (Background::Color(soft_bg), soft_fg, Color::TRANSPARENT),
        BadgeVariant::Surface => (
            Background::Color(apply_opacity(palette.muted, 0.6)),
            accent_text_color,
            palette.border,
        ),
        BadgeVariant::Outline => (Background::Color(Color::TRANSPARENT), accent_text_color, accent),
    };

    let shadow = if matches!(props.variant, BadgeVariant::Solid) && props.high_contrast {
        Shadow {
            color: apply_opacity(Color::BLACK, 0.08),
            offset: iced::Vector::new(0.0, 1.0),
            blur_radius: 6.0,
        }
    } else {
        Shadow::default()
    };

    let style = move |_iced_theme: &iced::Theme| container::Style {
        background: Some(background),
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
            .size(props.size.text_size())
            .style(move |_theme: &iced::Theme| iced::widget::text::Style {
                color: Some(text_color),
            }),
    )
    .padding(props.size.padding())
    .style(style)
}
