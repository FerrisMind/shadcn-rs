use iced::alignment::{Horizontal, Vertical};
use iced::border::Border;
use iced::widget::{button as button_widget, button as iced_button, column, container, text};
use iced::{Alignment, Background, Color, Element, Length, Shadow};

use crate::theme::Theme;
use crate::tokens::{AccentColor, accent_color, accent_low, accent_text, is_dark};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TabsVariant {
    Underline,
    Soft,
    Outline,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TabsOrientation {
    Horizontal,
    Vertical,
}

pub type TabsDirection = TabsOrientation;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TabsActivationMode {
    Automatic,
    Manual,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TabsListLoop {
    Enabled,
    Disabled,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TabsSize {
    One,
    Two,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TabsWrap {
    NoWrap,
    Wrap,
    WrapReverse,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TabsJustify {
    Start,
    Center,
    End,
}

#[derive(Clone, Debug)]
pub struct TabItem {
    pub id: String,
    pub label: String,
    pub disabled: bool,
}

impl TabItem {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            disabled: false,
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

#[derive(Clone, Copy, Debug)]
pub struct TabsProps {
    pub variant: TabsVariant,
    pub orientation: TabsDirection,
    pub activation_mode: TabsActivationMode,
    pub list_loop: TabsListLoop,
    pub size: TabsSize,
    pub wrap: TabsWrap,
    pub justify: TabsJustify,
    pub color: AccentColor,
    pub high_contrast: bool,
    pub full_width: bool,
}

impl Default for TabsProps {
    fn default() -> Self {
        Self {
            variant: TabsVariant::Underline,
            orientation: TabsDirection::Horizontal,
            activation_mode: TabsActivationMode::Automatic,
            list_loop: TabsListLoop::Enabled,
            size: TabsSize::Two,
            wrap: TabsWrap::NoWrap,
            justify: TabsJustify::Start,
            color: AccentColor::Gray,
            high_contrast: false,
            full_width: false,
        }
    }
}

impl TabsProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn variant(mut self, variant: TabsVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn orientation(mut self, orientation: TabsDirection) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn activation_mode(mut self, activation_mode: TabsActivationMode) -> Self {
        self.activation_mode = activation_mode;
        self
    }

    pub fn list_loop(mut self, list_loop: TabsListLoop) -> Self {
        self.list_loop = list_loop;
        self
    }

    pub fn size(mut self, size: TabsSize) -> Self {
        self.size = size;
        self
    }

    pub fn wrap(mut self, wrap: TabsWrap) -> Self {
        self.wrap = wrap;
        self
    }

    pub fn justify(mut self, justify: TabsJustify) -> Self {
        self.justify = justify;
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

    pub fn full_width(mut self, full_width: bool) -> Self {
        self.full_width = full_width;
        self
    }
}

impl TabsSize {
    fn padding(self) -> [f32; 2] {
        match self {
            TabsSize::One => [6.0, 10.0],
            TabsSize::Two => [8.0, 12.0],
        }
    }

    fn text_size(self) -> u32 {
        match self {
            TabsSize::One => 12,
            TabsSize::Two => 13,
        }
    }
}

fn apply_opacity(color: Color, opacity: f32) -> Color {
    Color { a: color.a * opacity, ..color }
}

fn trigger_style(
    theme: &Theme,
    props: TabsProps,
    is_active: bool,
    status: button_widget::Status,
) -> button_widget::Style {
    let palette = theme.palette;
    let accent = accent_color(&palette, props.color);
    let active_txt = accent_text(&palette, props.color);

    let is_hovered = matches!(status, button_widget::Status::Hovered);
    let is_pressed = matches!(status, button_widget::Status::Pressed);

    let muted_bg = if is_dark(&palette) {
        apply_opacity(accent_low(&palette, AccentColor::Gray), 0.75)
    } else {
        apply_opacity(accent_low(&palette, AccentColor::Gray), 0.55)
    };

    let (background, border) = match props.variant {
        TabsVariant::Underline => {
            let bg = if is_pressed {
                Background::Color(apply_opacity(muted_bg, 0.9))
            } else if is_hovered {
                Background::Color(muted_bg)
            } else {
                Background::Color(Color::TRANSPARENT)
            };

            let border_color = if is_active { accent } else { Color::TRANSPARENT };
            let border_width = if is_active { 2.0 } else { 0.0 };

            (
                bg,
                Border {
                    color: border_color,
                    width: border_width,
                    radius: 0.0.into(),
                },
            )
        }
        TabsVariant::Soft => {
            let bg = if is_active {
                Background::Color(muted_bg)
            } else if is_hovered {
                Background::Color(apply_opacity(muted_bg, 0.7))
            } else {
                Background::Color(Color::TRANSPARENT)
            };

            (
                bg,
                Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: theme.radius.sm.into(),
                },
            )
        }
        TabsVariant::Outline => {
            let bg = if is_active {
                Background::Color(apply_opacity(palette.card, 1.0))
            } else if is_hovered {
                Background::Color(apply_opacity(palette.card, 0.9))
            } else {
                Background::Color(Color::TRANSPARENT)
            };
            let border_color = if is_active { accent } else { palette.border };
            (bg, Border {
                color: border_color,
                width: 1.0,
                radius: theme.radius.sm.into(),
            })
        }
    };

    let text_color = if is_active { active_txt } else { palette.muted_foreground };

    button_widget::Style {
        background: Some(background),
        text_color,
        border,
        shadow: Shadow::default(),
        snap: true,
    }
}

pub fn tabs<'a, Message: Clone + 'a, F>(
    items: Vec<TabItem>,
    active: &'a str,
    on_value_change: Option<F>,
    props: TabsProps,
    theme: &Theme,
    content: impl Into<Element<'a, Message>>,
) -> Element<'a, Message>
where
    F: Fn(String) -> Message + 'a,
{
    let theme = theme.clone();
    let on_value_change = on_value_change.map(|f| Box::new(f) as Box<dyn Fn(String) -> Message + 'a>);

    let active_id = items
        .iter()
        .find(|t| t.id == active && !t.disabled)
        .or_else(|| items.iter().find(|t| !t.disabled))
        .map(|t| t.id.as_str())
        .unwrap_or(active);

    let mut trigger_items: Vec<Element<'a, Message>> = Vec::new();

    for item in &items {
        let is_active = item.id == active_id;
        let is_disabled = item.disabled || on_value_change.is_none();

        let label = text(item.label.clone()).size(props.size.text_size());
        let mut trigger = iced_button(label)
            .padding(props.size.padding())
            .style({
                let theme = theme.clone();
                move |_iced_theme, status| trigger_style(&theme, props, is_active, status)
            });

        if !is_disabled && let Some(on_change) = on_value_change.as_ref() {
            trigger = trigger.on_press((on_change)(item.id.clone()));
        }

        let trigger: Element<'a, Message> = trigger.into();
        trigger_items.push(if props.full_width {
            container(trigger).width(Length::Fill).into()
        } else {
            trigger
        });
    }

    let justify = match props.justify {
        TabsJustify::Start => Horizontal::Left,
        TabsJustify::Center => Horizontal::Center,
        TabsJustify::End => Horizontal::Right,
    };

    let triggers: Element<'a, Message> = match props.orientation {
        TabsOrientation::Horizontal => {
            let mut row = iced::widget::Row::new()
                .spacing(6)
                .align_y(Alignment::Center);
            for item in trigger_items {
                row = row.push(item);
            }
            container(row)
                .width(if props.full_width { Length::Fill } else { Length::Shrink })
                .align_x(justify)
                .align_y(Vertical::Center)
                .into()
        }
        TabsOrientation::Vertical => {
            let mut column = iced::widget::Column::new().spacing(6);
            for item in trigger_items {
                column = column.push(item);
            }
            container(column).into()
        }
    };

    column![triggers, content.into()]
        .spacing(12)
        .width(Length::Fill)
        .into()
}
