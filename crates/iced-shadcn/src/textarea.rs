use iced::Background;
use iced::border::Border;
use iced::widget::text_editor;

use crate::button::ButtonRadius;
use crate::theme::Theme;
use crate::tokens::{AccentColor, accent_color, accent_soft, accent_text, is_dark};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextareaSize {
    One,
    Two,
    Three,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextareaVariant {
    Classic,
    Surface,
    Soft,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextareaResize {
    None,
    Vertical,
    Horizontal,
    Both,
}

#[derive(Clone, Copy, Debug)]
pub struct TextareaProps {
    pub size: TextareaSize,
    pub variant: TextareaVariant,
    pub resize: TextareaResize,
    pub color: AccentColor,
    pub radius: Option<ButtonRadius>,
    pub invalid: bool,
    pub disabled: bool,
}

impl Default for TextareaProps {
    fn default() -> Self {
        Self {
            size: TextareaSize::Two,
            variant: TextareaVariant::Surface,
            resize: TextareaResize::None,
            color: AccentColor::Gray,
            radius: None,
            invalid: false,
            disabled: false,
        }
    }
}

impl TextareaProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn size(mut self, size: TextareaSize) -> Self {
        self.size = size;
        self
    }

    pub fn variant(mut self, variant: TextareaVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn resize(mut self, resize: TextareaResize) -> Self {
        self.resize = resize;
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

    pub fn invalid(mut self, invalid: bool) -> Self {
        self.invalid = invalid;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl TextareaSize {
    fn padding(self) -> [f32; 2] {
        match self {
            TextareaSize::One => [6.0, 10.0],
            TextareaSize::Two => [8.0, 12.0],
            TextareaSize::Three => [10.0, 14.0],
        }
    }

    fn text_size(self) -> u32 {
        match self {
            TextareaSize::One => 14,
            TextareaSize::Two => 14,
            TextareaSize::Three => 16,
        }
    }

    fn min_height(self) -> f32 {
        match self {
            TextareaSize::One => 64.0,
            TextareaSize::Two => 96.0,
            TextareaSize::Three => 128.0,
        }
    }
}

fn textarea_radius(theme: &Theme, props: TextareaProps) -> f32 {
    match props.radius {
        Some(ButtonRadius::None) => 0.0,
        Some(ButtonRadius::Small) => theme.radius.sm,
        Some(ButtonRadius::Medium) => theme.radius.md,
        Some(ButtonRadius::Large) => theme.radius.lg,
        Some(ButtonRadius::Full) => 9999.0,
        None => theme.radius.sm,
    }
}

pub fn textarea<'a, Message: Clone + 'a, F>(
    content: &'a text_editor::Content,
    placeholder: &'a str,
    on_action: Option<F>,
    props: TextareaProps,
    theme: &Theme,
) -> text_editor::TextEditor<'a, iced::advanced::text::highlighter::PlainText, Message>
where
    F: Fn(text_editor::Action) -> Message + 'a,
{
    let theme = theme.clone();
    let mut widget = text_editor::TextEditor::new(content)
        .placeholder(placeholder)
        .padding(props.size.padding())
        .size(props.size.text_size())
        .height(iced::Length::Fixed(props.size.min_height()))
        .style(move |_iced_theme, status| textarea_style(&theme, props, status));

    if !props.disabled
        && let Some(on_action) = on_action
    {
        widget = widget.on_action(on_action);
    }

    widget
}

fn textarea_style(
    theme: &Theme,
    props: TextareaProps,
    status: text_editor::Status,
) -> text_editor::Style {
    let palette = theme.palette;
    let radius = textarea_radius(theme, props);
    let accent = accent_color(&palette, props.color);
    let text_color = accent_text(&palette, props.color);
    let soft_bg = accent_soft(&palette, props.color);

    let mut border = Border {
        radius: radius.into(),
        width: 1.0,
        color: palette.input,
    };
    let base_bg = if is_dark(&palette) {
        Background::Color(palette.input)
    } else {
        Background::Color(iced::Color::TRANSPARENT)
    };
    let mut background = match props.variant {
        TextareaVariant::Classic | TextareaVariant::Surface => base_bg,
        TextareaVariant::Soft => Background::Color(soft_bg),
    };
    let mut value = match props.variant {
        TextareaVariant::Soft => text_color,
        _ => palette.foreground,
    };
    let mut placeholder = match props.variant {
        TextareaVariant::Soft => text_color,
        _ => palette.muted_foreground,
    };

    match status {
        text_editor::Status::Hovered => {
            border.color = if props.invalid {
                palette.destructive
            } else {
                palette.ring
            };
        }
        text_editor::Status::Focused { .. } => {
            border.color = if props.invalid {
                palette.destructive
            } else {
                palette.ring
            };
            border.width = 1.5;
        }
        text_editor::Status::Disabled => {
            background = Background::Color(palette.muted);
            value = palette.muted_foreground;
            placeholder = palette.muted_foreground;
        }
        text_editor::Status::Active => {}
    }

    if props.invalid && matches!(status, text_editor::Status::Active) {
        border.color = palette.destructive;
    }

    text_editor::Style {
        background,
        border,
        placeholder,
        value,
        selection: accent,
    }
}
