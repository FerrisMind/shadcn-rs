use iced::alignment::{Horizontal, Vertical};
use iced::border::Border;
use iced::widget::{container, text};
use iced::{Background, Color, Element, Length};

use crate::theme::Theme;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AvatarSize {
    Size1,
    Size2,
    #[default]
    Size3,
    Size4,
    Size5,
    Size6,
    Size7,
    Size8,
    Size9,
}

impl AvatarSize {
    fn to_pixels(self) -> f32 {
        match self {
            AvatarSize::Size1 => 16.0,
            AvatarSize::Size2 => 20.0,
            AvatarSize::Size3 => 24.0,
            AvatarSize::Size4 => 32.0,
            AvatarSize::Size5 => 40.0,
            AvatarSize::Size6 => 48.0,
            AvatarSize::Size7 => 64.0,
            AvatarSize::Size8 => 80.0,
            AvatarSize::Size9 => 96.0,
        }
    }

    fn font_size(self) -> f32 {
        self.to_pixels() * 0.4
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AvatarVariant {
    Solid,
    #[default]
    Soft,
}

#[derive(Clone, Debug)]
pub struct AvatarProps<'a> {
    pub fallback: &'a str,
    pub size: AvatarSize,
    pub variant: AvatarVariant,
    pub color: Option<Color>,
}

impl<'a> AvatarProps<'a> {
    pub fn new(fallback: &'a str) -> Self {
        Self {
            fallback,
            size: AvatarSize::Size3,
            variant: AvatarVariant::Soft,
            color: None,
        }
    }

    pub fn size(mut self, size: AvatarSize) -> Self {
        self.size = size;
        self
    }

    pub fn variant(mut self, variant: AvatarVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }
}

pub fn avatar<'a, Message: Clone + 'a>(
    props: AvatarProps<'a>,
    theme: &Theme,
) -> Element<'a, Message> {
    let size = props.size.to_pixels();
    let font_size = props.size.font_size();
    let accent = props.color.unwrap_or(theme.palette.primary);

    let (bg_color, text_color) = match props.variant {
        AvatarVariant::Solid => (accent, theme.palette.primary_foreground),
        AvatarVariant::Soft => (apply_opacity(accent, 0.18), accent),
    };

    let text_value = props
        .fallback
        .chars()
        .take(2)
        .collect::<String>()
        .to_uppercase();

    let content = text(text_value)
        .size(font_size)
        .style(move |_t| iced::widget::text::Style {
            color: Some(text_color),
        });

    container(content)
        .width(Length::Fixed(size))
        .height(Length::Fixed(size))
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
        .style(move |_t| iced::widget::container::Style {
            background: Some(Background::Color(bg_color)),
            border: Border {
                radius: (size / 2.0).into(),
                width: 0.0,
                color: Color::TRANSPARENT,
            },
            ..Default::default()
        })
        .into()
}

fn apply_opacity(color: Color, opacity: f32) -> Color {
    Color {
        a: color.a * opacity,
        ..color
    }
}
