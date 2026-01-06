use iced::Background;
use iced::widget::radio as radio_widget;

use crate::theme::Theme;
use crate::tokens::{AccentColor, accent_color, accent_soft, accent_text, is_dark};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RadioSize {
    One,
    Two,
    Three,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RadioVariant {
    Classic,
    Surface,
    Soft,
}

#[derive(Clone, Copy, Debug)]
pub struct RadioProps {
    pub size: RadioSize,
    pub variant: RadioVariant,
    pub color: AccentColor,
    pub high_contrast: bool,
}

impl Default for RadioProps {
    fn default() -> Self {
        Self {
            size: RadioSize::Two,
            variant: RadioVariant::Surface,
            color: AccentColor::Gray,
            high_contrast: false,
        }
    }
}

impl RadioProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn size(mut self, size: RadioSize) -> Self {
        self.size = size;
        self
    }

    pub fn variant(mut self, variant: RadioVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn color(mut self, color: AccentColor) -> Self {
        self.color = color;
        self
    }

    pub fn high_contrast(mut self, high_contrast: bool) -> Self {
        self.high_contrast = high_contrast;
        self
    }
}

impl RadioSize {
    fn dimension(self) -> f32 {
        match self {
            RadioSize::One => 14.0,
            RadioSize::Two => 16.0,
            RadioSize::Three => 20.0,
        }
    }

    fn text_size(self) -> u32 {
        match self {
            RadioSize::One => 12,
            RadioSize::Two => 14,
            RadioSize::Three => 16,
        }
    }
}

pub fn radio<'a, Message: Clone + 'a, F, V>(
    label: impl Into<String>,
    value: V,
    selected: Option<V>,
    on_select: F,
    props: RadioProps,
    theme: &Theme,
) -> radio_widget::Radio<'a, Message>
where
    F: FnOnce(V) -> Message,
    V: Copy + Eq,
{
    let theme = theme.clone();
    radio_widget::Radio::new(label, value, selected, on_select)
        .size(props.size.dimension())
        .spacing(props.size.dimension() * 0.5)
        .text_size(props.size.text_size())
        .style(move |_iced_theme, status| radio_style(&theme, props, status))
}

fn radio_style(theme: &Theme, props: RadioProps, status: radio_widget::Status) -> radio_widget::Style {
    let palette = theme.palette;
    let accent = accent_color(&palette, props.color);
    let text_color = accent_text(&palette, props.color);
    let soft_bg = accent_soft(&palette, props.color);
    let base_bg = if is_dark(&palette) {
        Background::Color(palette.input)
    } else {
        Background::Color(iced::Color::TRANSPARENT)
    };

    let (is_selected, hovered) = match status {
        radio_widget::Status::Active { is_selected } => (is_selected, false),
        radio_widget::Status::Hovered { is_selected } => (is_selected, true),
    };

    let mut background = match props.variant {
        RadioVariant::Soft => Background::Color(soft_bg),
        RadioVariant::Classic | RadioVariant::Surface => base_bg,
    };

    let mut dot_color = accent;
    let mut border_color = palette.input;

    if hovered {
        border_color = palette.ring;
    }

    if props.variant == RadioVariant::Soft {
        background = Background::Color(soft_bg);
        dot_color = if is_selected { text_color } else { accent };
    }

    if is_selected && props.high_contrast {
        background = Background::Color(palette.foreground);
        dot_color = palette.background;
        border_color = palette.foreground;
    }

    radio_widget::Style {
        background,
        dot_color,
        border_width: 1.0,
        border_color,
        text_color: Some(palette.foreground),
    }
}
