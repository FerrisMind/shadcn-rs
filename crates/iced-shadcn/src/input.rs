use iced::Background;
use iced::border::Border;
use iced::widget::text_input;

use crate::theme::Theme;

#[derive(Clone, Copy, Debug)]
pub enum InputSize {
    Sm,
    Md,
    Lg,
}

impl InputSize {
    fn padding(self) -> [f32; 2] {
        match self {
            InputSize::Sm => [6.0, 10.0],
            InputSize::Md => [8.0, 12.0],
            InputSize::Lg => [10.0, 14.0],
        }
    }

    fn text_size(self) -> u32 {
        match self {
            InputSize::Sm => 12,
            InputSize::Md => 14,
            InputSize::Lg => 16,
        }
    }
}

pub fn input<'a, Message: Clone>(
    value: &'a str,
    placeholder: &'a str,
    on_input: impl Fn(String) -> Message + 'a,
    size: InputSize,
    theme: &Theme,
) -> text_input::TextInput<'a, Message> {
    let theme = theme.clone();
    text_input::TextInput::new(placeholder, value)
        .on_input(on_input)
        .padding(size.padding())
        .size(size.text_size())
        .style(move |_iced_theme, status| input_style(&theme, status))
}

fn input_style(theme: &Theme, status: text_input::Status) -> text_input::Style {
    let palette = theme.palette;
    let radius = theme.radius.sm;

    let mut border = Border {
        radius: radius.into(),
        width: 1.0,
        color: palette.border,
    };
    let mut background = Background::Color(palette.background);
    let mut value = palette.foreground;

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
        }
        text_input::Status::Active => {}
    }

    text_input::Style {
        background,
        border,
        icon: palette.muted_foreground,
        placeholder: palette.muted_foreground,
        value,
        selection: palette.primary,
    }
}
