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
    pub background: Option<Color>,
    pub text_color: Option<Color>,
    pub border_color: Option<Color>,
    pub padding: Option<f32>,
    pub radius: Option<f32>,
}

impl Default for CardProps {
    fn default() -> Self {
        Self {
            variant: CardVariant::Surface,
            size: CardSize::Size1,
            show_shadow: true,
            background: None,
            text_color: None,
            border_color: None,
            padding: None,
            radius: None,
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

    pub fn background(mut self, background: Color) -> Self {
        self.background = Some(background);
        self
    }

    pub fn text_color(mut self, text_color: Color) -> Self {
        self.text_color = Some(text_color);
        self
    }

    pub fn border_color(mut self, border_color: Color) -> Self {
        self.border_color = Some(border_color);
        self
    }

    pub fn padding(mut self, padding: f32) -> Self {
        self.padding = Some(padding.max(0.0));
        self
    }

    pub fn radius(mut self, radius: f32) -> Self {
        self.radius = Some(radius.max(0.0));
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
    let radius = props.radius.unwrap_or_else(|| props.size.radius(theme));
    let mut border_color = palette.border;
    let mut text_color = palette.card_foreground;
    let border_width = 1.0;

    let is_dark_theme = crate::tokens::is_dark(&palette);

    let (mut background, default_shadow) = match props.variant {
        CardVariant::Surface => (
            Some(Background::Color(palette.card)),
            if is_dark_theme {
                Shadow::default()
            } else {
                Shadow {
                    color: apply_opacity(Color::BLACK, 0.08),
                    offset: iced::Vector::new(0.0, 2.0),
                    blur_radius: 10.0,
                }
            },
        ),
        CardVariant::Classic => (
            Some(Background::Color(palette.card)),
            if is_dark_theme {
                Shadow::default()
            } else {
                Shadow {
                    color: apply_opacity(Color::BLACK, 0.12),
                    offset: iced::Vector::new(0.0, 4.0),
                    blur_radius: 14.0,
                }
            },
        ),
        CardVariant::Ghost => (None, Shadow::default()),
    };

    if let Some(custom_background) = props.background {
        background = Some(Background::Color(custom_background));
    }
    if let Some(custom_text_color) = props.text_color {
        text_color = custom_text_color;
    }
    if let Some(custom_border_color) = props.border_color {
        border_color = custom_border_color;
    }

    let shadow = if props.show_shadow {
        default_shadow
    } else {
        Shadow::default()
    };

    container_widget::Style {
        background,
        text_color: Some(text_color),
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
    let padding = props.padding.unwrap_or_else(|| props.size.padding());
    let theme = theme.clone();

    container(content)
        .padding(padding)
        .style(move |_iced_theme: &iced::Theme| card_style(&theme, props))
}
