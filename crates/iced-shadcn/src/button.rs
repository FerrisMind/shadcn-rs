use iced::alignment::{Horizontal, Vertical};
use iced::border::Border;
use iced::widget::text::IntoFragment;
use iced::widget::{
    button as button_widget, button as iced_button, container, stack, text as iced_text,
};
use iced::{Background, Color, Element, Length, Shadow};

use crate::spinner::{Spinner, SpinnerSize, spinner};
use crate::theme::Theme;
use crate::tokens::{
    AccentColor, accent_color, accent_foreground, accent_soft, accent_soft_foreground, accent_text,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ButtonVariant {
    Classic,
    Solid,
    Soft,
    Surface,
    Outline,
    Ghost,
    Link,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ButtonSize {
    One,
    Two,
    Three,
    Four,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ButtonRadius {
    None,
    Small,
    Medium,
    Large,
    Full,
}

#[derive(Clone, Copy, Debug)]
pub struct ButtonProps {
    pub variant: ButtonVariant,
    pub size: ButtonSize,
    pub color: AccentColor,
    pub radius: Option<ButtonRadius>,
    pub high_contrast: bool,
    pub loading: bool,
    pub disabled: bool,
}

impl Default for ButtonProps {
    fn default() -> Self {
        Self {
            variant: ButtonVariant::Solid,
            size: ButtonSize::Two,
            color: AccentColor::Gray,
            radius: None,
            high_contrast: false,
            loading: false,
            disabled: false,
        }
    }
}

impl ButtonProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
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

    pub fn high_contrast(mut self, high_contrast: bool) -> Self {
        self.high_contrast = high_contrast;
        self
    }

    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl ButtonSize {
    fn padding(self) -> [f32; 2] {
        match self {
            ButtonSize::One => [6.0, 12.0],
            ButtonSize::Two => [8.0, 16.0],
            ButtonSize::Three => [10.0, 24.0],
            ButtonSize::Four => [12.0, 28.0],
        }
    }

    fn height(self) -> f32 {
        match self {
            ButtonSize::One => 32.0,
            ButtonSize::Two => 36.0,
            ButtonSize::Three => 40.0,
            ButtonSize::Four => 48.0,
        }
    }

    fn text_size(self) -> u32 {
        match self {
            ButtonSize::One => 14,
            ButtonSize::Two => 14,
            ButtonSize::Three => 14,
            ButtonSize::Four => 16,
        }
    }
}

pub fn button<'a, Message: Clone + 'a>(
    label: impl IntoFragment<'a>,
    on_press: Option<Message>,
    props: ButtonProps,
    theme: &Theme,
) -> button_widget::Button<'a, Message> {
    let content = iced_text(label).size(props.size.text_size());
    button_content(content, on_press, props, theme)
}

pub fn button_content<'a, Message: Clone + 'a>(
    content: impl Into<Element<'a, Message>>,
    on_press: Option<Message>,
    props: ButtonProps,
    theme: &Theme,
) -> button_widget::Button<'a, Message> {
    button_content_aligned(content, on_press, props, theme, false)
}

fn button_content_aligned<'a, Message: Clone + 'a>(
    content: impl Into<Element<'a, Message>>,
    on_press: Option<Message>,
    props: ButtonProps,
    theme: &Theme,
    center_x: bool,
) -> button_widget::Button<'a, Message> {
    let content: Element<'a, Message> = if props.loading {
        loading_overlay(content.into(), props, theme)
    } else {
        content.into()
    };
    let mut wrapper = container(content)
        .height(Length::Fixed(props.size.height()))
        .align_y(Vertical::Center);
    if center_x {
        wrapper = wrapper.width(Length::Fill).align_x(Horizontal::Center);
    }
    let content: Element<'a, Message> = wrapper.into();

    let mut widget = iced_button(content)
        .padding(props.size.padding())
        .height(Length::Fixed(props.size.height()));

    let disabled = props.disabled || props.loading || on_press.is_none();
    if let Some(message) = on_press
        && !disabled
    {
        widget = widget.on_press(message);
    }

    let theme = theme.clone();
    widget.style(move |_iced_theme, status| button_style(&theme, props, status))
}

pub fn icon_button<'a, Message: Clone + 'a>(
    content: impl Into<Element<'a, Message>>,
    on_press: Option<Message>,
    props: ButtonProps,
    theme: &Theme,
) -> button_widget::Button<'a, Message> {
    let size = props.size.height();
    button_content_aligned(content, on_press, props, theme, true)
        .padding(0)
        .width(Length::Fixed(size))
        .height(Length::Fixed(size))
}

fn loading_overlay<'a, Message: Clone + 'a>(
    content: Element<'a, Message>,
    props: ButtonProps,
    theme: &Theme,
) -> Element<'a, Message> {
    let spinner_size = match props.size {
        ButtonSize::One => SpinnerSize::One,
        ButtonSize::Two => SpinnerSize::Two,
        ButtonSize::Three | ButtonSize::Four => SpinnerSize::Three,
    };
    let spinner_color = accent_text(&theme.palette, props.color);
    let spinner = spinner(Spinner::new(theme).size(spinner_size).color(spinner_color));
    let spinner_layer = container(spinner)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill);
    stack![container(content), spinner_layer]
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

fn button_radius(theme: &Theme, props: ButtonProps) -> f32 {
    match props.radius {
        Some(ButtonRadius::None) => 0.0,
        Some(ButtonRadius::Small) => theme.radius.sm,
        Some(ButtonRadius::Medium) => theme.radius.md,
        Some(ButtonRadius::Large) => theme.radius.lg,
        Some(ButtonRadius::Full) => 9999.0,
        None => theme.radius.sm,
    }
}

fn mix(a: Color, b: Color, t: f32) -> Color {
    let t = t.clamp(0.0, 1.0);
    Color {
        r: a.r + (b.r - a.r) * t,
        g: a.g + (b.g - a.g) * t,
        b: a.b + (b.b - a.b) * t,
        a: a.a + (b.a - a.a) * t,
    }
}

fn button_style(
    theme: &Theme,
    props: ButtonProps,
    status: button_widget::Status,
) -> button_widget::Style {
    let palette = theme.palette;
    let radius = button_radius(theme, props);

    let accent = accent_color(&palette, props.color);
    let accent_fg = accent_foreground(&palette, props.color);
    let accent_txt = accent_text(&palette, props.color);
    let soft_bg = accent_soft(&palette, props.color);
    let soft_fg = accent_soft_foreground(&palette, props.color);

    let (mut background, mut text_color, mut border_color) = match props.variant {
        ButtonVariant::Classic | ButtonVariant::Solid => {
            (Some(Background::Color(accent)), accent_fg, accent)
        }
        ButtonVariant::Soft => (Some(Background::Color(soft_bg)), soft_fg, soft_bg),
        ButtonVariant::Surface => (
            Some(Background::Color(palette.background)),
            accent_txt,
            palette.border,
        ),
        ButtonVariant::Outline => (None, accent_txt, palette.border),
        ButtonVariant::Ghost => (None, accent_txt, Color::TRANSPARENT),
        ButtonVariant::Link => (None, accent, Color::TRANSPARENT),
    };

    if props.high_contrast {
        text_color = palette.foreground;
    }

    match status {
        button_widget::Status::Hovered => {
            background = match props.variant {
                ButtonVariant::Classic | ButtonVariant::Solid => {
                    Some(Background::Color(mix(accent, palette.background, 0.1)))
                }
                ButtonVariant::Soft
                | ButtonVariant::Surface
                | ButtonVariant::Outline
                | ButtonVariant::Ghost => Some(Background::Color(palette.muted)),
                ButtonVariant::Link => None,
            };
        }
        button_widget::Status::Pressed => {
            background = match props.variant {
                ButtonVariant::Classic | ButtonVariant::Solid => {
                    Some(Background::Color(mix(accent, palette.background, 0.2)))
                }
                ButtonVariant::Soft
                | ButtonVariant::Surface
                | ButtonVariant::Outline
                | ButtonVariant::Ghost => Some(Background::Color(palette.muted)),
                ButtonVariant::Link => None,
            };
        }
        button_widget::Status::Disabled => {
            text_color = palette.muted_foreground;
            background = Some(Background::Color(palette.muted));
            border_color = palette.border;
        }
        button_widget::Status::Active => {}
    }

    button_widget::Style {
        background,
        text_color,
        border: Border {
            radius: radius.into(),
            width: if matches!(props.variant, ButtonVariant::Outline) {
                1.0
            } else {
                0.0
            },
            color: border_color,
        },
        shadow: Shadow::default(),
        snap: true,
    }
}
