use iced::Background;
use iced::Font;
use iced::border::Border;
use iced::widget::checkbox as checkbox_widget;
use iced::widget::text;
use iced::widget::text::IntoFragment;
use lucide_icons::Icon as LucideIcon;

use crate::theme::Theme;
use crate::tokens::{
    AccentColor, accent_color, accent_foreground, accent_soft, accent_text, is_dark,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CheckboxSize {
    One,
    Two,
    Three,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CheckboxVariant {
    Classic,
    Surface,
    Soft,
}

#[derive(Clone, Copy, Debug)]
pub struct CheckboxProps {
    pub size: CheckboxSize,
    pub variant: CheckboxVariant,
    pub color: AccentColor,
    pub high_contrast: bool,
    pub disabled: bool,
}

impl Default for CheckboxProps {
    fn default() -> Self {
        Self {
            size: CheckboxSize::Two,
            variant: CheckboxVariant::Surface,
            color: AccentColor::Gray,
            high_contrast: false,
            disabled: false,
        }
    }
}

impl CheckboxProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn size(mut self, size: CheckboxSize) -> Self {
        self.size = size;
        self
    }

    pub fn variant(mut self, variant: CheckboxVariant) -> Self {
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

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl CheckboxSize {
    fn dimension(self) -> f32 {
        match self {
            CheckboxSize::One => 14.0,
            CheckboxSize::Two => 16.0,
            CheckboxSize::Three => 20.0,
        }
    }

    fn text_size(self) -> u32 {
        match self {
            CheckboxSize::One => 12,
            CheckboxSize::Two => 14,
            CheckboxSize::Three => 16,
        }
    }

    fn icon_size(self) -> f32 {
        self.dimension() - 2.0
    }
}

fn checkbox_radius(props: CheckboxProps) -> f32 {
    props.size.dimension() * 0.25
}

pub fn checkbox<'a, Message: Clone + 'a, F>(
    label: impl IntoFragment<'a>,
    is_checked: bool,
    on_toggle: Option<F>,
    props: CheckboxProps,
    theme: &Theme,
) -> checkbox_widget::Checkbox<'a, Message>
where
    F: Fn(bool) -> Message + 'a,
{
    let theme = theme.clone();
    let icon = checkbox_icon(props.size);
    let mut widget = checkbox_widget::Checkbox::new(is_checked)
        .label(label)
        .size(props.size.dimension())
        .icon(icon)
        .spacing(props.size.dimension() * 0.5)
        .text_size(props.size.text_size())
        .style(move |_iced_theme, status| checkbox_style(&theme, props, status));

    if props.disabled {
        widget = widget.on_toggle_maybe(None::<fn(bool) -> Message>);
    } else {
        widget = widget.on_toggle_maybe(on_toggle);
    }

    widget
}

fn checkbox_icon(size: CheckboxSize) -> checkbox_widget::Icon<Font> {
    checkbox_widget::Icon {
        font: Font::with_name("lucide"),
        code_point: char::from(LucideIcon::Check),
        size: Some(size.icon_size().into()),
        line_height: text::LineHeight::default(),
        shaping: text::Shaping::Basic,
    }
}

fn checkbox_style(
    theme: &Theme,
    props: CheckboxProps,
    status: checkbox_widget::Status,
) -> checkbox_widget::Style {
    let palette = theme.palette;
    let radius = checkbox_radius(props);
    let accent = accent_color(&palette, props.color);
    let accent_fg = accent_foreground(&palette, props.color);
    let soft_bg = accent_soft(&palette, props.color);
    let text_color = accent_text(&palette, props.color);
    let base_bg = if is_dark(&palette) {
        Background::Color(palette.input)
    } else {
        Background::Color(iced::Color::TRANSPARENT)
    };

    let is_checked = match status {
        checkbox_widget::Status::Active { is_checked }
        | checkbox_widget::Status::Hovered { is_checked }
        | checkbox_widget::Status::Disabled { is_checked } => is_checked,
    };

    let mut background = match props.variant {
        CheckboxVariant::Soft => Background::Color(soft_bg),
        CheckboxVariant::Classic | CheckboxVariant::Surface => base_bg,
    };
    let mut border_color = palette.input;
    let mut icon_color = accent_fg;
    let mut label_color = palette.foreground;

    if is_checked {
        match props.variant {
            CheckboxVariant::Soft => {
                background = Background::Color(soft_bg);
                border_color = accent;
                icon_color = text_color;
            }
            CheckboxVariant::Classic | CheckboxVariant::Surface => {
                background = Background::Color(accent);
                border_color = accent;
                icon_color = accent_fg;
            }
        }

        if props.high_contrast {
            background = Background::Color(palette.foreground);
            border_color = palette.foreground;
            icon_color = palette.background;
        }
    }

    match status {
        checkbox_widget::Status::Hovered { .. } => {
            border_color = palette.ring;
        }
        checkbox_widget::Status::Disabled { .. } => {
            if is_checked {
                background = Background::Color(palette.muted);
            } else {
                background = base_bg;
            }
            border_color = palette.border;
            icon_color = palette.muted_foreground;
            label_color = palette.muted_foreground;
        }
        checkbox_widget::Status::Active { .. } => {}
    }

    checkbox_widget::Style {
        background,
        icon_color,
        border: Border {
            radius: radius.into(),
            width: 1.0,
            color: border_color,
        },
        text_color: Some(label_color),
    }
}
