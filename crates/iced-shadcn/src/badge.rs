use iced::widget::{button, container, row, text};
use iced::{Alignment, Background, Color, Element, Shadow};
use lucide_icons::Icon as LucideIcon;

use crate::button::ButtonRadius;
use crate::theme::Theme;
use crate::tokens::AccentColor;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum BadgeSize {
    #[default]
    Size1,
    Size2,
    Size3,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum BadgeVariant {
    #[default]
    Default,
    Secondary,
    Outline,
    Destructive,
}

#[derive(Clone, Copy, Debug)]
pub struct BadgeProps<'a, Message> {
    pub size: BadgeSize,
    pub variant: BadgeVariant,
    pub color: Option<AccentColor>,
    pub radius: Option<ButtonRadius>,
    pub high_contrast: bool,
    /// Optional icon to display on the left.
    pub icon: Option<LucideIcon>,
    /// When set, badge renders as a clickable link element.
    pub href: Option<&'a str>,
    /// Optional message to publish when clicked.
    pub on_press: Option<Message>,
}

impl<Message> Default for BadgeProps<'_, Message> {
    fn default() -> Self {
        Self {
            size: BadgeSize::Size1,
            variant: BadgeVariant::Default,
            color: None,
            radius: None,
            high_contrast: false,
            icon: None,
            href: None,
            on_press: None,
        }
    }
}

impl<'a, Message> BadgeProps<'a, Message> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn size(mut self, size: BadgeSize) -> Self {
        self.size = size;
        self
    }

    pub fn variant(mut self, variant: BadgeVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn color(mut self, color: AccentColor) -> Self {
        self.color = Some(color);
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

    /// When set, badge renders as a clickable link element.
    pub fn href(mut self, href: &'a str) -> Self {
        self.href = Some(href);
        self
    }

    pub fn icon(mut self, icon: LucideIcon) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Sets the message to publish when the badge is pressed.
    pub fn on_press(mut self, on_press: Message) -> Self {
        self.on_press = Some(on_press);
        self
    }
}

fn badge_radius<Message>(theme: &Theme, props: &BadgeProps<'_, Message>) -> f32 {
    match props.radius {
        Some(ButtonRadius::None) => 0.0,
        Some(ButtonRadius::Small) => theme.radius.sm,
        Some(ButtonRadius::Medium) => theme.radius.md,
        Some(ButtonRadius::Large) => theme.radius.lg,
        Some(ButtonRadius::Full) => 9999.0,
        None => 9999.0, // Default to pill for badges
    }
}

impl BadgeSize {
    fn padding(self) -> [f32; 2] {
        match self {
            BadgeSize::Size1 => [2.0, 6.0],
            BadgeSize::Size2 => [3.0, 8.0],
            BadgeSize::Size3 => [4.0, 10.0],
        }
    }

    fn text_size(self) -> u16 {
        match self {
            BadgeSize::Size1 => 11,
            BadgeSize::Size2 => 12,
            BadgeSize::Size3 => 13,
        }
    }
}

fn apply_opacity(color: Color, opacity: f32) -> Color {
    Color {
        a: color.a * opacity,
        ..color
    }
}

fn resolve_badge_colors<Message>(
    props: &BadgeProps<'_, Message>,
    theme: &Theme,
) -> (Color, Color, Color) {
    let palette = theme.palette;
    match props.variant {
        BadgeVariant::Default => {
            let color = match props.color {
                Some(c) => crate::tokens::accent_color(&palette, c),
                None => palette.primary,
            };
            (color, palette.primary_foreground, color)
        }
        BadgeVariant::Secondary => {
            let color = match props.color {
                Some(c) => crate::tokens::accent_color(&palette, c),
                None => palette.secondary,
            };
            (color, palette.secondary_foreground, color)
        }
        BadgeVariant::Destructive => {
            let color = match props.color {
                Some(c) => crate::tokens::accent_color(&palette, c),
                None => palette.destructive,
            };
            (color, palette.destructive_foreground, color)
        }
        BadgeVariant::Outline => (Color::TRANSPARENT, palette.foreground, palette.border),
    }
}

pub fn badge_content<'a, Message: Clone + 'a>(
    content: impl Into<Element<'a, Message>>,
    props: BadgeProps<'a, Message>,
    theme: &Theme,
) -> Element<'a, Message> {
    let radius = badge_radius(theme, &props);
    let (background_color, text_color, border_color) = resolve_badge_colors(&props, theme);

    let shadow = if matches!(props.variant, BadgeVariant::Default) && props.high_contrast {
        Shadow {
            color: apply_opacity(Color::BLACK, 0.08),
            offset: iced::Vector::new(0.0, 1.0),
            blur_radius: 6.0,
        }
    } else {
        Shadow::default()
    };

    let style = move |_iced_theme: &iced::Theme| container::Style {
        background: Some(Background::Color(background_color)),
        text_color: Some(text_color),
        border: iced::border::Border {
            color: border_color,
            width: if border_color.a > 0.0 { 1.0 } else { 0.0 },
            radius: radius.into(),
        },
        shadow,
        snap: true,
    };

    let is_interactive = props.on_press.is_some() || props.href.is_some();
    let content = content.into();

    if is_interactive {
        let mut b = button(content).padding(props.size.padding());

        if let Some(msg) = props.on_press {
            b = b.on_press(msg);
        }

        b.style(move |_theme, status| {
            let base = style(&iced::Theme::Light);
            let mut bg = base.background;

            if matches!(status, iced::widget::button::Status::Hovered) {
                bg = Some(Background::Color(crate::tokens::mix(
                    background_color,
                    Color::WHITE,
                    0.1,
                )));
            }

            iced::widget::button::Style {
                background: bg,
                text_color: base.text_color.unwrap_or(text_color),
                border: base.border,
                shadow: base.shadow,
                snap: true,
            }
        })
        .into()
    } else {
        container(content)
            .padding(props.size.padding())
            .style(style)
            .into()
    }
}

pub fn badge<'a, Message: Clone + 'a>(
    label: impl Into<String>,
    props: BadgeProps<'a, Message>,
    theme: &Theme,
) -> Element<'a, Message> {
    let (_, text_color, _) = resolve_badge_colors(&props, theme);
    let text_size = props.size.text_size();

    let mut content = row![].spacing(4).align_y(Alignment::Center);

    if let Some(icon) = props.icon {
        content = content.push(
            text(char::from(icon).to_string())
                .size(text_size as u32)
                .font(iced::Font::with_name("lucide"))
                .style(move |_theme: &iced::Theme| iced::widget::text::Style {
                    color: Some(text_color),
                }),
        );
    }

    content = content.push(text(label.into()).size(text_size as u32).style(
        move |_theme: &iced::Theme| iced::widget::text::Style {
            color: Some(text_color),
        },
    ));

    if props.href.is_some() {
        content = content.push(
            text(char::from(LucideIcon::ExternalLink).to_string())
                .size(text_size as u32 - 1)
                .font(iced::Font::with_name("lucide"))
                .style(move |_theme: &iced::Theme| iced::widget::text::Style {
                    color: Some(apply_opacity(text_color, 0.8)),
                }),
        );
    }

    badge_content(content, props, theme)
}
