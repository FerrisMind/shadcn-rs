use iced::Background;
use iced::border::Border;
use iced::widget::text_input;

use crate::button::ButtonRadius;
use crate::theme::Theme;
use crate::tokens::{AccentColor, accent_color, accent_soft, accent_text};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextFieldSize {
    One,
    Two,
    Three,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextFieldVariant {
    Classic,
    Surface,
    Soft,
}

#[derive(Clone, Copy, Debug)]
pub struct TextFieldProps {
    pub size: TextFieldSize,
    pub variant: TextFieldVariant,
    pub color: AccentColor,
    pub radius: Option<ButtonRadius>,
    pub disabled: bool,
    pub read_only: bool,
}

impl Default for TextFieldProps {
    fn default() -> Self {
        Self {
            size: TextFieldSize::Two,
            variant: TextFieldVariant::Surface,
            color: AccentColor::Gray,
            radius: None,
            disabled: false,
            read_only: false,
        }
    }
}

impl TextFieldProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn size(mut self, size: TextFieldSize) -> Self {
        self.size = size;
        self
    }

    pub fn variant(mut self, variant: TextFieldVariant) -> Self {
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

impl TextFieldSize {
    fn padding(self) -> [f32; 2] {
        match self {
            TextFieldSize::One => [6.0, 10.0],
            TextFieldSize::Two => [8.0, 12.0],
            TextFieldSize::Three => [10.0, 14.0],
        }
    }

    fn text_size(self) -> u32 {
        match self {
            TextFieldSize::One => 14,
            TextFieldSize::Two => 14,
            TextFieldSize::Three => 16,
        }
    }
}

pub fn text_field<'a, Message: Clone + 'a, F>(
    value: &'a str,
    placeholder: &'a str,
    on_input: Option<F>,
    props: TextFieldProps,
    theme: &Theme,
) -> text_input::TextInput<'a, Message>
where
    F: Fn(String) -> Message + 'a,
{
    let theme = theme.clone();
    let mut widget = text_input::TextInput::new(placeholder, value)
        .padding(props.size.padding())
        .size(props.size.text_size())
        .style(move |_iced_theme, status| text_field_style(&theme, props, status));

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

fn text_field_radius(theme: &Theme, props: TextFieldProps) -> f32 {
    match props.radius {
        Some(ButtonRadius::None) => 0.0,
        Some(ButtonRadius::Small) => theme.radius.sm,
        Some(ButtonRadius::Medium) => theme.radius.md,
        Some(ButtonRadius::Large) => theme.radius.lg,
        Some(ButtonRadius::Full) => 9999.0,
        None => theme.radius.sm,
    }
}

fn text_field_style(
    theme: &Theme,
    props: TextFieldProps,
    status: text_input::Status,
) -> text_input::Style {
    let palette = theme.palette;
    let radius = text_field_radius(theme, props);
    let accent = accent_color(&palette, props.color);
    let text_color = accent_text(&palette, props.color);
    let soft_bg = accent_soft(&palette, props.color);

    let mut border = Border {
        radius: radius.into(),
        width: 1.0,
        color: palette.border,
    };
    let mut background = match props.variant {
        TextFieldVariant::Classic | TextFieldVariant::Surface => {
            Background::Color(palette.background)
        }
        TextFieldVariant::Soft => Background::Color(soft_bg),
    };
    let mut value = match props.variant {
        TextFieldVariant::Soft => text_color,
        _ => palette.foreground,
    };
    let mut placeholder = match props.variant {
        TextFieldVariant::Soft => text_color,
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
