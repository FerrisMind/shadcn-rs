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
    Size1,
    Size2,
    Size3,
    Size4,
    Five,
}

#[derive(Clone, Copy, Debug)]
pub struct CardProps {
    pub variant: CardVariant,
    pub size: CardSize,
    pub show_shadow: bool,
}

impl Default for CardProps {
    fn default() -> Self {
        Self {
            variant: CardVariant::Surface,
            size: CardSize::Size1,
            show_shadow: true,
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

    pub fn show_shadow(mut self, show_shadow: bool) -> Self {
        self.show_shadow = show_shadow;
        self
    }
}

impl CardSize {
    fn padding(self) -> f32 {
        match self {
            CardSize::Size1 => 12.0,
            CardSize::Size2 => 16.0,
            CardSize::Size3 => 20.0,
            CardSize::Size4 => 24.0,
            CardSize::Five => 32.0,
        }
    }

    fn radius(self, theme: &Theme) -> f32 {
        match self {
            CardSize::Size1 | CardSize::Size2 => theme.radius.sm,
            CardSize::Size3 | CardSize::Size4 => theme.radius.md,
            CardSize::Five => theme.radius.lg,
        }
    }
}

fn card_style(theme: &Theme, props: CardProps) -> container_widget::Style {
    let palette = theme.palette;
    let radius = props.size.radius(theme);
    let border_color = palette.border;
    let border_width = 1.0;

    let (background, default_shadow) = match props.variant {
        CardVariant::Surface => (
            Some(Background::Color(palette.card)),
            Shadow {
                color: apply_opacity(Color::BLACK, 0.08),
                offset: iced::Vector::new(0.0, 2.0),
                blur_radius: 10.0,
            },
        ),
        CardVariant::Classic => (
            Some(Background::Color(palette.card)),
            Shadow {
                color: apply_opacity(Color::BLACK, 0.12),
                offset: iced::Vector::new(0.0, 4.0),
                blur_radius: 14.0,
            },
        ),
        CardVariant::Ghost => (None, Shadow::default()),
    };
    let shadow = if props.show_shadow {
        default_shadow
    } else {
        Shadow::default()
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

fn apply_opacity(mut color: Color, opacity: f32) -> Color {
    color.a *= opacity;
    color
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
