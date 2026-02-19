use iced::Background;
use iced::border::Border;
use iced::widget::text_input;

use crate::button::ButtonRadius;
use crate::theme::Theme;
use crate::tokens::{AccentColor, accent_color, accent_soft, accent_text};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InputSize {
    Size1,
    Size2,
    Size3,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InputVariant {
    Classic,
    Surface,
    Soft,
}

#[derive(Clone, Copy, Debug)]
pub struct InputProps {
    pub size: InputSize,
    pub variant: InputVariant,
    pub color: AccentColor,
    pub radius: Option<ButtonRadius>,
    pub disabled: bool,
    pub read_only: bool,
}

impl Default for InputProps {
    fn default() -> Self {
        Self {
            size: InputSize::Size2,
            variant: InputVariant::Surface,
            color: AccentColor::Gray,
            radius: None,
            disabled: false,
            read_only: false,
        }
    }
}

impl InputProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn size(mut self, size: InputSize) -> Self {
        self.size = size;
        self
    }

    pub fn variant(mut self, variant: InputVariant) -> Self {
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

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn read_only(mut self, read_only: bool) -> Self {
        self.read_only = read_only;
        self
    }
}

impl InputSize {
    fn padding(self) -> [f32; 2] {
        match self {
            InputSize::Size1 => [6.0, 10.0],
            InputSize::Size2 => [8.0, 12.0],
            InputSize::Size3 => [10.0, 14.0],
        }
    }

    fn text_size(self) -> u32 {
        match self {
            InputSize::Size1 => 14,
            InputSize::Size2 => 14,
            InputSize::Size3 => 16,
        }
    }
}

pub fn input<'a, Message: Clone + 'a, F>(
    value: &'a str,
    placeholder: &'a str,
    on_input: Option<F>,
    props: InputProps,
    theme: &Theme,
) -> text_input::TextInput<'a, Message>
where
    F: Fn(String) -> Message + 'a,
{
    let theme = theme.clone();
    let mut widget = text_input::TextInput::new(placeholder, value)
        .padding(props.size.padding())
        .size(props.size.text_size())
        .style(move |_iced_theme, status| input_style(&theme, props, status));

    if let Some(on_input) = on_input {
        if props.disabled {
            widget = widget.on_input_maybe(None::<fn(String) -> Message>);
        } else {
            widget = widget.on_input(on_input);
        }
    } else {
        widget = widget.on_input_maybe(None::<fn(String) -> Message>);
    }

    widget
}

fn input_radius(theme: &Theme, props: InputProps) -> f32 {
    match props.radius {
        Some(ButtonRadius::None) => 0.0,
        Some(ButtonRadius::Small) => theme.radius.sm,
        Some(ButtonRadius::Medium) => theme.radius.md,
        Some(ButtonRadius::Large) => theme.radius.lg,
        Some(ButtonRadius::Full) => 9999.0,
        None => theme.radius.sm,
    }
}

fn input_style(theme: &Theme, props: InputProps, status: text_input::Status) -> text_input::Style {
    let palette = theme.palette;
    let radius = input_radius(theme, props);
    let accent = accent_color(&palette, props.color);
    let text_color = accent_text(&palette, props.color);
    let soft_bg = accent_soft(&palette, props.color);

    let mut border = Border {
        radius: radius.into(),
        width: 1.0,
        color: palette.border,
    };
    let mut background = match props.variant {
        InputVariant::Classic | InputVariant::Surface => Background::Color(palette.background),
        InputVariant::Soft => Background::Color(soft_bg),
    };
    let mut value = match props.variant {
        InputVariant::Soft => text_color,
        _ => palette.foreground,
    };
    let mut placeholder = match props.variant {
        InputVariant::Soft => text_color,
        _ => palette.muted_foreground,
    };

    match status {
        text_input::Status::Hovered => {
            border.color = palette.ring;
        }
        text_input::Status::Focused { .. } => {
            border.color = palette.ring;
            border.width = 1.5;
        }
        text_input::Status::Disabled => {
            background = Background::Color(palette.muted);
            value = palette.muted_foreground;
            placeholder = palette.muted_foreground;
        }
        text_input::Status::Active => {}
    }

    text_input::Style {
        background,
        border,
        icon: palette.muted_foreground,
        placeholder,
        value,
        selection: accent,
    }
}
