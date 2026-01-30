use iced::border::Border;
use iced::widget::{container, container as container_widget};
use iced::{Background, Color, Element, Shadow};

use crate::theme::Theme;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CardVariant {
    Surface,
    Classic,
    Ghost,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CardSize {
    One,
    Two,
    Three,
    Four,
    Five,
}

#[derive(Clone, Copy, Debug)]
pub struct CardProps {
    pub variant: CardVariant,
    pub size: CardSize,
}

impl Default for CardProps {
    fn default() -> Self {
        Self {
            variant: CardVariant::Surface,
            size: CardSize::One,
        }
    }
}

impl CardProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn variant(mut self, variant: CardVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: CardSize) -> Self {
        self.size = size;
        self
    }
}

impl CardSize {
    fn padding(self) -> f32 {
        match self {
            CardSize::One => 12.0,
            CardSize::Two => 16.0,
            CardSize::Three => 20.0,
            CardSize::Four => 24.0,
            CardSize::Five => 32.0,
        }
    }

    fn radius(self, theme: &Theme) -> f32 {
        match self {
            CardSize::One | CardSize::Two => theme.radius.sm,
            CardSize::Three | CardSize::Four => theme.radius.md,
            CardSize::Five => theme.radius.lg,
        }
    }
}

fn apply_opacity(color: Color, opacity: f32) -> Color {
    Color {
        a: color.a * opacity,
        ..color
    }
}

fn card_style(theme: &Theme, props: CardProps) -> container_widget::Style {
    let palette = theme.palette;
    let radius = props.size.radius(theme);

    let (background, border_color, border_width, shadow) = match props.variant {
        CardVariant::Surface => (
            Some(Background::Color(palette.card)),
            palette.border,
            1.0,
            Shadow {
                color: apply_opacity(Color::BLACK, 0.08),
                offset: iced::Vector::new(0.0, 2.0),
                blur_radius: 10.0,
            },
        ),
        CardVariant::Classic => (
            Some(Background::Color(palette.card)),
            palette.border,
            1.0,
            Shadow {
                color: apply_opacity(Color::BLACK, 0.12),
                offset: iced::Vector::new(0.0, 4.0),
                blur_radius: 14.0,
            },
        ),
        CardVariant::Ghost => (None, Color::TRANSPARENT, 0.0, Shadow::default()),
    };

    container_widget::Style {
        background,
        text_color: Some(palette.card_foreground),
        border: Border {
            color: border_color,
            width: border_width,
            radius: radius.into(),
        },
        shadow,
        snap: true,
    }
}

pub fn card<'a, Message: 'a>(
    content: impl Into<Element<'a, Message>>,
    props: CardProps,
    theme: &Theme,
) -> container::Container<'a, Message> {
    let padding = props.size.padding();
    let theme = theme.clone();

    container(content)
        .padding(padding)
        .style(move |_iced_theme: &iced::Theme| card_style(&theme, props))
}
