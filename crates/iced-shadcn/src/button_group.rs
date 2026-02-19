use iced::border::{Border, Radius};
use iced::widget::{button as button_widget, column, row};
use iced::{Alignment, Background, Color, Element};

use crate::button::{ButtonProps, ButtonRadius, ButtonVariant, button_content};
use crate::theme::Theme;
use crate::tokens::{
    accent_color, accent_foreground, accent_soft, accent_soft_foreground, accent_text,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ButtonGroupOrientation {
    #[default]
    Horizontal,
    Vertical,
}

pub struct ButtonGroupItem<'a, Message> {
    pub content: Element<'a, Message>,
    pub on_press: Option<Message>,
    pub props: ButtonProps,
}

impl<'a, Message> ButtonGroupItem<'a, Message> {
    pub fn new(
        content: impl Into<Element<'a, Message>>,
        on_press: Option<Message>,
        props: ButtonProps,
    ) -> Self {
        Self {
            content: content.into(),
            on_press,
            props,
        }
    }
}

pub struct ButtonGroup<'a, Message> {
    orientation: ButtonGroupOrientation,
    radius: ButtonRadius,
    items: Vec<ButtonGroupItem<'a, Message>>,
}

impl<'a, Message> ButtonGroup<'a, Message> {
    pub fn new(items: Vec<ButtonGroupItem<'a, Message>>) -> Self {
        Self {
            orientation: ButtonGroupOrientation::Horizontal,
            radius: ButtonRadius::Medium,
            items,
        }
    }

    pub fn orientation(mut self, orientation: ButtonGroupOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn radius(mut self, radius: ButtonRadius) -> Self {
        self.radius = radius;
        self
    }

    pub fn show(self, theme: &Theme) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        let count = self.items.len();
        let theme = theme.clone();

        let mut children: Vec<Element<'a, Message>> = Vec::with_capacity(count);
        for (index, item) in self.items.into_iter().enumerate() {
            let radius = group_radius(
                &theme,
                item.props.radius.unwrap_or(self.radius),
                self.orientation,
                index,
                count,
            );
            children.push(button_group_button(item, theme.clone(), radius));
        }

        match self.orientation {
            ButtonGroupOrientation::Horizontal => {
                row(children).spacing(0).align_y(Alignment::Center).into()
            }
            ButtonGroupOrientation::Vertical => column(children)
                .spacing(0)
                .align_x(Alignment::Center)
                .into(),
        }
    }
}

pub fn button_group<'a, Message: Clone + 'a>(
    items: Vec<ButtonGroupItem<'a, Message>>,
    theme: &Theme,
) -> Element<'a, Message> {
    ButtonGroup::new(items).show(theme)
}

fn group_radius(
    theme: &Theme,
    radius: ButtonRadius,
    orientation: ButtonGroupOrientation,
    index: usize,
    count: usize,
) -> Radius {
    let base = button_radius(theme, radius);
    if count <= 1 {
        return Radius::new(base);
    }

    match orientation {
        ButtonGroupOrientation::Horizontal => {
            if index == 0 {
                Radius {
                    top_left: base,
                    bottom_left: base,
                    ..Radius::default()
                }
            } else if index + 1 == count {
                Radius {
                    top_right: base,
                    bottom_right: base,
                    ..Radius::default()
                }
            } else {
                Radius::new(0.0)
            }
        }
        ButtonGroupOrientation::Vertical => {
            if index == 0 {
                Radius {
                    top_left: base,
                    top_right: base,
                    ..Radius::default()
                }
            } else if index + 1 == count {
                Radius {
                    bottom_left: base,
                    bottom_right: base,
                    ..Radius::default()
                }
            } else {
                Radius::new(0.0)
            }
        }
    }
}

fn button_group_button<'a, Message: Clone + 'a>(
    item: ButtonGroupItem<'a, Message>,
    theme: Theme,
    radius: Radius,
) -> Element<'a, Message> {
    let ButtonGroupItem {
        content,
        on_press,
        props,
    } = item;

    let mut button = button_content(content, on_press, props, &theme);
    button = button.style(move |_t, status| button_group_style(&theme, props, status, radius));
    button.into()
}

fn button_radius(theme: &Theme, radius: ButtonRadius) -> f32 {
    match radius {
        ButtonRadius::None => 0.0,
        ButtonRadius::Small => theme.radius.sm,
        ButtonRadius::Medium => theme.radius.md,
        ButtonRadius::Large => theme.radius.lg,
        ButtonRadius::Full => 9999.0,
    }
}

use crate::tokens::mix;

fn button_group_style(
    theme: &Theme,
    props: ButtonProps,
    status: button_widget::Status,
    radius: Radius,
) -> button_widget::Style {
    let palette = theme.palette;

    let accent = accent_color(&palette, props.color);
    let accent_fg = accent_foreground(&palette, props.color);
    let accent_txt = accent_text(&palette, props.color);
    let soft_bg = accent_soft(&palette, props.color);
    let soft_fg = accent_soft_foreground(&palette, props.color);

    let (mut background, mut text_color, mut border_color) = match props.variant {
        ButtonVariant::Default | ButtonVariant::Classic | ButtonVariant::Solid => {
            (Some(Background::Color(accent)), accent_fg, accent)
        }
        ButtonVariant::Secondary => {
            let color = palette.secondary;
            let fg = palette.secondary_foreground;
            (Some(Background::Color(color)), fg, color)
        }
        ButtonVariant::Destructive => {
            let color = palette.destructive;
            let fg = palette.destructive_foreground;
            (Some(Background::Color(color)), fg, color)
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
                ButtonVariant::Default | ButtonVariant::Classic | ButtonVariant::Solid => {
                    Some(Background::Color(mix(accent, palette.background, 0.1)))
                }
                ButtonVariant::Secondary => Some(Background::Color(mix(
                    palette.secondary,
                    palette.background,
                    0.1,
                ))),
                ButtonVariant::Destructive => Some(Background::Color(mix(
                    palette.destructive,
                    palette.background,
                    0.1,
                ))),
                ButtonVariant::Soft
                | ButtonVariant::Surface
                | ButtonVariant::Outline
                | ButtonVariant::Ghost => Some(Background::Color(palette.muted)),
                ButtonVariant::Link => None,
            };
        }
        button_widget::Status::Pressed => {
            background = match props.variant {
                ButtonVariant::Default | ButtonVariant::Classic | ButtonVariant::Solid => {
                    Some(Background::Color(mix(accent, palette.background, 0.2)))
                }
                ButtonVariant::Secondary => Some(Background::Color(mix(
                    palette.secondary,
                    palette.background,
                    0.2,
                ))),
                ButtonVariant::Destructive => Some(Background::Color(mix(
                    palette.destructive,
                    palette.background,
                    0.2,
                ))),
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
            radius,
            width: if matches!(props.variant, ButtonVariant::Outline) {
                1.0
            } else {
                0.0
            },
            color: border_color,
        },
        shadow: Default::default(),
        snap: true,
    }
}
