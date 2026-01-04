use iced::border::Border;
use iced::widget::text::IntoFragment;
use iced::widget::{button as button_widget, button as iced_button, text as iced_text};
use iced::{Background, Color, Shadow};

use crate::theme::Theme;

#[derive(Clone, Copy, Debug)]
pub enum ButtonVariant {
    Default,
    Secondary,
    Destructive,
    Outline,
    Ghost,
    Link,
}

#[derive(Clone, Copy, Debug)]
pub enum ButtonSize {
    Sm,
    Md,
    Lg,
    IconSm,
    Icon,
    IconLg,
}

impl ButtonSize {
    fn padding(self) -> [f32; 2] {
        match self {
            ButtonSize::Sm => [6.0, 12.0],
            ButtonSize::Md => [8.0, 16.0],
            ButtonSize::Lg => [10.0, 20.0],
            ButtonSize::IconSm => [6.0, 6.0],
            ButtonSize::Icon => [8.0, 8.0],
            ButtonSize::IconLg => [10.0, 10.0],
        }
    }

    fn text_size(self) -> u32 {
        match self {
            ButtonSize::Sm => 12,
            ButtonSize::Md => 14,
            ButtonSize::Lg => 16,
            ButtonSize::IconSm => 12,
            ButtonSize::Icon => 14,
            ButtonSize::IconLg => 16,
        }
    }
}

pub fn button<'a, Message: Clone>(
    label: impl IntoFragment<'a>,
    on_press: Option<Message>,
    variant: ButtonVariant,
    size: ButtonSize,
    theme: &Theme,
) -> button_widget::Button<'a, Message> {
    let content = iced_text(label).size(size.text_size());
    button_content(content, on_press, variant, size, theme)
}

pub fn button_content<'a, Message: Clone>(
    content: impl Into<iced::Element<'a, Message>>,
    on_press: Option<Message>,
    variant: ButtonVariant,
    size: ButtonSize,
    theme: &Theme,
) -> button_widget::Button<'a, Message> {
    let mut widget = iced_button(content).padding(size.padding());

    if let Some(message) = on_press {
        widget = widget.on_press(message);
    }

    let theme = theme.clone();
    widget.style(move |_iced_theme, status| button_style(&theme, variant, status))
}

fn button_style(
    theme: &Theme,
    variant: ButtonVariant,
    status: button_widget::Status,
) -> button_widget::Style {
    let palette = theme.palette;
    let radius = theme.radius.md;

    let (mut background, mut text_color, mut border_color) = match variant {
        ButtonVariant::Default => (
            Some(Background::Color(palette.primary)),
            palette.primary_foreground,
            palette.primary,
        ),
        ButtonVariant::Secondary => (
            Some(Background::Color(palette.secondary)),
            palette.secondary_foreground,
            palette.secondary,
        ),
        ButtonVariant::Destructive => (
            Some(Background::Color(palette.destructive)),
            palette.destructive_foreground,
            palette.destructive,
        ),
        ButtonVariant::Outline => (None, palette.foreground, palette.border),
        ButtonVariant::Ghost => (None, palette.foreground, Color::TRANSPARENT),
        ButtonVariant::Link => (None, palette.primary, Color::TRANSPARENT),
    };

    match status {
        button_widget::Status::Hovered => {
            if matches!(variant, ButtonVariant::Default) {
                background = Some(Background::Color(palette.foreground));
            } else if matches!(
                variant,
                ButtonVariant::Secondary | ButtonVariant::Outline | ButtonVariant::Ghost
            ) {
                background = Some(Background::Color(palette.muted));
            }
        }
        button_widget::Status::Pressed => {
            if matches!(variant, ButtonVariant::Default) {
                background = Some(Background::Color(palette.foreground));
            } else if matches!(
                variant,
                ButtonVariant::Secondary | ButtonVariant::Outline | ButtonVariant::Ghost
            ) {
                background = Some(Background::Color(palette.muted));
            }
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
            width: if matches!(variant, ButtonVariant::Outline) {
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
