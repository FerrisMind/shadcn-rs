use iced::Font;
use iced::font::Weight;
use iced::widget::Text;
use iced::widget::text::{IntoFragment, LineHeight, Style};

use crate::theme::Theme;

/// Label styling options.
#[derive(Clone, Copy, Debug, Default)]
pub struct LabelProps {
    pub disabled: bool,
}

impl LabelProps {
    /// Creates default label props.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the disabled state.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

/// Shadcn-style label text.
pub fn label<'a>(content: impl IntoFragment<'a>, theme: &Theme) -> Text<'a> {
    label_with_props(content, LabelProps::default(), theme)
}

/// Shadcn-style label text with custom props.
pub fn label_with_props<'a>(
    content: impl IntoFragment<'a>,
    props: LabelProps,
    theme: &Theme,
) -> Text<'a> {
    let mut color = theme.palette.foreground;
    if props.disabled {
        color = apply_opacity(color, 0.5);
    }

    Text::new(content)
        .size(14)
        .line_height(LineHeight::Absolute(14.0.into()))
        .font(Font {
            weight: Weight::Medium,
            ..Font::DEFAULT
        })
        .style(move |_theme| Style { color: Some(color) })
}

fn apply_opacity(color: iced::Color, opacity: f32) -> iced::Color {
    iced::Color {
        a: color.a * opacity,
        ..color
    }
}
