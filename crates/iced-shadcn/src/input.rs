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
    Ghost,
}

#[derive(Clone, Copy, Debug)]
pub struct InputProps {
    pub size: InputSize,
    pub variant: InputVariant,
    pub color: AccentColor,
    pub radius: Option<ButtonRadius>,
    pub custom_height: Option<f32>,
    pub padding_x: Option<f32>,
    pub focus_only_border: bool,
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
            custom_height: None,
            padding_x: None,
            focus_only_border: false,
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

    pub fn custom_height(mut self, height: f32) -> Self {
        self.custom_height = Some(height.max(0.0));
        self
    }

    pub fn padding_x(mut self, padding_x: f32) -> Self {
        self.padding_x = Some(padding_x.max(0.0));
        self
    }

    pub fn focus_only_border(mut self, focus_only_border: bool) -> Self {
        self.focus_only_border = focus_only_border;
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
    fn padding(self, theme: &Theme) -> [f32; 2] {
        match self {
            InputSize::Size1 => [
                theme.styles.input.size1_padding_y,
                theme.styles.input.size1_padding_x,
            ],
            InputSize::Size2 => [
                theme.styles.input.size2_padding_y,
                theme.styles.input.size2_padding_x,
            ],
            InputSize::Size3 => [
                theme.styles.input.size3_padding_y,
                theme.styles.input.size3_padding_x,
            ],
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

fn input_padding(theme: &Theme, props: InputProps) -> [f32; 2] {
    let [padding_y, default_padding_x] = props.size.padding(theme);
    let padding_x = props.padding_x.unwrap_or(default_padding_x);
    let Some(custom_height) = props.custom_height else {
        return [padding_y, padding_x];
    };

    let border = theme.styles.input.border_width * 2.0;
    let text_height = props.size.text_size() as f32;
    let adjusted_y = ((custom_height - text_height - border) / 2.0).max(0.0);

    [adjusted_y, padding_x]
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
        .padding(input_padding(&theme, props))
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
        width: theme.styles.input.border_width,
        color: if props.focus_only_border {
            iced::Color::TRANSPARENT
        } else {
            palette.border
        },
    };
    let mut background = match props.variant {
        InputVariant::Classic | InputVariant::Surface => Background::Color(palette.background),
        InputVariant::Soft => Background::Color(soft_bg),
        InputVariant::Ghost => Background::Color(iced::Color::TRANSPARENT),
    };
    let mut value = match props.variant {
        InputVariant::Soft => text_color,
        InputVariant::Ghost => palette.foreground,
        _ => palette.foreground,
    };
    let mut placeholder = match props.variant {
        InputVariant::Soft => text_color,
        InputVariant::Ghost => palette.muted_foreground,
        _ => palette.muted_foreground,
    };

    if matches!(props.variant, InputVariant::Ghost) {
        border.width = 0.0;
        border.color = iced::Color::TRANSPARENT;
    }

    match status {
        text_input::Status::Hovered => {
            if !matches!(props.variant, InputVariant::Ghost) && !props.focus_only_border {
                border.color = palette.ring;
            }
        }
        text_input::Status::Focused { .. } => {
            if !matches!(props.variant, InputVariant::Ghost) {
                border.color = palette.ring;
                border.width = theme.styles.input.focused_border_width;
            }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_props_builder_accepts_custom_height() {
        let props = InputProps::new().custom_height(32.0);
        assert_eq!(props.custom_height, Some(32.0));
    }

    #[test]
    fn input_props_builder_accepts_padding_x() {
        let props = InputProps::new().padding_x(6.0);
        assert_eq!(props.padding_x, Some(6.0));
    }

    #[test]
    fn input_props_builder_accepts_focus_only_border() {
        let props = InputProps::new().focus_only_border(true);
        assert!(props.focus_only_border);
    }
}
