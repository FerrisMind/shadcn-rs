use iced::Background;
use iced::Shadow;
use iced::Vector;
use iced::border;
use iced::widget::{button as button_widget, container, row, space, stack};
use iced::widget::button;
use iced::Length;

use crate::theme::Theme;
use crate::tokens::{AccentColor, accent_color, accent_foreground, accent_soft, is_dark};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwitchSize {
    One,
    Two,
    Three,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwitchVariant {
    Classic,
    Surface,
    Soft,
}

#[derive(Clone, Copy, Debug)]
pub struct SwitchProps {
    pub size: SwitchSize,
    pub variant: SwitchVariant,
    pub color: AccentColor,
    pub radius: Option<crate::button::ButtonRadius>,
    pub high_contrast: bool,
    pub disabled: bool,
}

impl Default for SwitchProps {
    fn default() -> Self {
        Self {
            size: SwitchSize::Two,
            variant: SwitchVariant::Surface,
            color: AccentColor::Gray,
            radius: None,
            high_contrast: false,
            disabled: false,
        }
    }
}

impl SwitchProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn size(mut self, size: SwitchSize) -> Self {
        self.size = size;
        self
    }

    pub fn variant(mut self, variant: SwitchVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn color(mut self, color: AccentColor) -> Self {
        self.color = color;
        self
    }

    pub fn radius(mut self, radius: crate::button::ButtonRadius) -> Self {
        self.radius = Some(radius);
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

impl SwitchSize {
    fn metrics(self) -> SwitchMetrics {
        let scale = match self {
            SwitchSize::One => 0.8,
            SwitchSize::Two => 1.0,
            SwitchSize::Three => 1.2,
        };
        let height = 18.4 * scale;
        let thumb = 16.0 * scale;
        let width = 32.0 * scale;
        let padding_ratio = ((height - thumb) / 2.0) / height;
        let thumb_offset_checked = width - thumb - 2.0;

        SwitchMetrics {
            height,
            width,
            thumb,
            padding_ratio,
            thumb_offset_checked,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct SwitchMetrics {
    height: f32,
    width: f32,
    thumb: f32,
    padding_ratio: f32,
    thumb_offset_checked: f32,
}

#[derive(Clone, Copy, Debug)]
struct SwitchColors {
    track: iced::Color,
    thumb: iced::Color,
}

fn switch_radius(theme: &Theme, props: SwitchProps) -> f32 {
    let height = props.size.metrics().height;
    match props.radius {
        Some(crate::button::ButtonRadius::None) => 0.0,
        Some(crate::button::ButtonRadius::Small) => theme.radius.sm,
        Some(crate::button::ButtonRadius::Medium) => theme.radius.md,
        Some(crate::button::ButtonRadius::Large) => theme.radius.lg,
        Some(crate::button::ButtonRadius::Full) => height / 2.0,
        None => height / 2.0,
    }
}

pub fn switch<'a, Message: Clone + 'a, F>(
    is_checked: bool,
    on_toggle: Option<F>,
    props: SwitchProps,
    theme: &Theme,
) -> button_widget::Button<'a, Message>
where
    F: Fn(bool) -> Message + 'a,
{
    let metrics = props.size.metrics();
    let radius = switch_radius(theme, props);
    let padding = metrics.padding_ratio * metrics.height;
    let thumb_radius = (radius - padding).max(0.0);
    let dark_mode = is_dark(&theme.palette);
    let disabled = props.disabled;
    let colors = switch_colors(theme, props, is_checked, disabled, dark_mode);
    let track_shadow = shadow_xs(if disabled { 0.5 } else { 1.0 });

    let track = container(space())
        .width(Length::Fixed(metrics.width))
        .height(Length::Fixed(metrics.height))
        .style(move |_theme| iced::widget::container::Style {
            background: Some(Background::Color(colors.track)),
            border: border::Border {
                radius: radius.into(),
                width: 1.0,
                color: iced::Color::TRANSPARENT,
            },
            shadow: track_shadow,
            ..iced::widget::container::Style::default()
        });

    let thumb = container(space())
        .width(Length::Fixed(metrics.thumb))
        .height(Length::Fixed(metrics.thumb))
        .style(move |_theme| iced::widget::container::Style {
            background: Some(Background::Color(colors.thumb)),
            border: border::Border {
                radius: thumb_radius.into(),
                width: 0.0,
                color: iced::Color::TRANSPARENT,
            },
            ..iced::widget::container::Style::default()
        });

    let offset = if is_checked {
        metrics.thumb_offset_checked
    } else {
        0.0
    };
    let thumb_layer = row![
        space().width(Length::Fixed(offset)),
        thumb
    ]
    .width(Length::Fixed(metrics.width))
    .height(Length::Fixed(metrics.height))
    .align_y(iced::Alignment::Center);

    let content = stack![track, thumb_layer];

    let mut widget = button_widget(content).style(|_theme, _status| button::Style {
        background: None,
        text_color: iced::Color::TRANSPARENT,
        border: border::Border::default(),
        shadow: Shadow::default(),
        snap: true,
    });

    if !disabled {
        if let Some(on_toggle) = on_toggle {
            widget = widget.on_press(on_toggle(!is_checked));
        }
    }

    widget
}

fn switch_colors(
    theme: &Theme,
    props: SwitchProps,
    is_checked: bool,
    disabled: bool,
    dark_mode: bool,
) -> SwitchColors {
    let palette = theme.palette;
    let accent = accent_color(&palette, props.color);
    let accent_fg = accent_foreground(&palette, props.color);
    let soft_bg = accent_soft(&palette, props.color);

    let mut track_unchecked = palette.input;
    if dark_mode {
        track_unchecked = apply_opacity(track_unchecked, 0.8);
    }
    let checked_track = match props.variant {
        SwitchVariant::Soft => soft_bg,
        SwitchVariant::Classic | SwitchVariant::Surface => accent,
    };
    let mut track = if is_checked {
        checked_track
    } else {
        track_unchecked
    };

    let mut thumb = if dark_mode {
        if is_checked {
            accent_fg
        } else {
            palette.foreground
        }
    } else {
        palette.background
    };

    if is_checked && props.high_contrast {
        track = palette.foreground;
        thumb = palette.background;
    }

    if disabled {
        track = apply_opacity(track, 0.5);
        thumb = apply_opacity(thumb, 0.5);
    }

    SwitchColors { track, thumb }
}

fn apply_opacity(color: iced::Color, opacity: f32) -> iced::Color {
    iced::Color {
        a: (color.a * opacity).clamp(0.0, 1.0),
        ..color
    }
}

fn shadow_xs(opacity: f32) -> Shadow {
    Shadow {
        color: apply_opacity(iced::Color::BLACK, 0.05 * opacity),
        offset: Vector::new(0.0, 1.0),
        blur_radius: 2.0,
    }
}
