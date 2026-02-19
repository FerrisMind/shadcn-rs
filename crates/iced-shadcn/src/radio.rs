use iced::Background;
use iced::widget::radio as radio_widget;

use crate::theme::Theme;
use crate::tokens::{
    AccentColor, ControlSize, ControlVariant, accent_color, accent_soft, accent_text, is_dark,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum RadioDirection {
    #[default]
    Vertical,
    Horizontal,
}

#[derive(Clone, Debug)]
pub struct RadioItem<V> {
    pub label: String,
    pub value: V,
    pub description: Option<String>,
    pub disabled: bool,
}

impl<V> RadioItem<V> {
    pub fn new(label: impl Into<String>, value: V) -> Self {
        Self {
            label: label.into(),
            value,
            description: None,
            disabled: false,
        }
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

#[derive(Clone, Copy, Debug)]
pub struct RadioGroupProps {
    pub size: ControlSize,
    pub variant: ControlVariant,
    pub direction: RadioDirection,
    pub color: AccentColor,
    pub high_contrast: bool,
    pub disabled: bool,
}

impl Default for RadioGroupProps {
    fn default() -> Self {
        Self {
            size: ControlSize::Md,
            variant: ControlVariant::Primary,
            direction: RadioDirection::Vertical,
            color: AccentColor::Gray,
            high_contrast: false,
            disabled: false,
        }
    }
}

impl RadioGroupProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    pub fn variant(mut self, variant: ControlVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn direction(mut self, direction: RadioDirection) -> Self {
        self.direction = direction;
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

impl ControlSize {
    fn dimension(self) -> f32 {
        match self {
            ControlSize::Sm | ControlSize::IconSm => 14.0,
            ControlSize::Md | ControlSize::Icon => 16.0,
            ControlSize::Lg | ControlSize::IconLg => 20.0,
        }
    }

    fn text_size(self) -> u32 {
        match self {
            ControlSize::Sm | ControlSize::IconSm => 12,
            ControlSize::Md | ControlSize::Icon => 14,
            ControlSize::Lg | ControlSize::IconLg => 16,
        }
    }
}

pub fn radio_group<'a, Message: Clone + 'a, V>(
    selected: Option<V>,
    options: Vec<RadioItem<V>>,
    on_select: impl Fn(V) -> Message + 'a,
    props: RadioGroupProps,
    theme: &'a Theme,
) -> iced::Element<'a, Message>
where
    V: Copy + Eq + 'a,
{
    use iced::widget::{column, container, row, text};

    let spacing = match props.size {
        ControlSize::Sm | ControlSize::IconSm => 8.0,
        ControlSize::Md | ControlSize::Icon => 10.0,
        ControlSize::Lg | ControlSize::IconLg => 12.0,
    };

    let items = options.into_iter().map(|item| {
        let item_disabled = props.disabled || item.disabled;
        let r = radio_widget::Radio::new(item.label, item.value, selected, &on_select)
            .size(props.size.dimension())
            .spacing(props.size.dimension() * 0.5)
            .text_size(props.size.text_size())
            .style(move |_iced_theme, status| {
                let mut s = radio_style(theme, props, status);
                if item_disabled {
                    s.text_color = Some(theme.palette.muted_foreground);
                    s.dot_color = theme.palette.muted;
                    s.border_color = theme.palette.border;
                }
                s
            });

        if let Some(desc) = item.description {
            column![
                r,
                container(text(desc).size(props.size.text_size() as f32 * 0.9)).padding(
                    iced::Padding {
                        left: props.size.dimension() * 1.5,
                        ..Default::default()
                    }
                )
            ]
            .spacing(2)
            .into()
        } else {
            r.into()
        }
    });

    match props.direction {
        RadioDirection::Vertical => column(items).spacing(spacing).into(),
        RadioDirection::Horizontal => row(items).spacing(spacing * 2.0).into(),
    }
}

fn radio_style(
    theme: &Theme,
    props: RadioGroupProps,
    status: radio_widget::Status,
) -> radio_widget::Style {
    let palette = theme.palette;

    let (accent, _text_color) = match props.variant {
        ControlVariant::Primary => (
            accent_color(&palette, props.color),
            accent_text(&palette, props.color),
        ),
        ControlVariant::Secondary => (palette.secondary, palette.secondary_foreground),
        ControlVariant::Destructive => (palette.destructive, palette.destructive_foreground),
    };

    let _soft_bg = accent_soft(&palette, props.color);
    let base_bg = if is_dark(&palette) {
        Background::Color(palette.input)
    } else {
        Background::Color(iced::Color::TRANSPARENT)
    };

    let (is_selected, hovered) = match status {
        radio_widget::Status::Active { is_selected } => (is_selected, false),
        radio_widget::Status::Hovered { is_selected } => (is_selected, true),
    };

    // Placeholder for "Soft" style if we want to keep parity with previous iced implementation
    // But aligning with shadcn/egui variants:
    let mut background = base_bg;
    let mut dot_color = accent;
    let mut border_color = palette.input;

    if hovered {
        border_color = palette.ring;
    }

    if is_selected && props.high_contrast {
        background = Background::Color(palette.foreground);
        dot_color = palette.background;
        border_color = palette.foreground;
    } else if is_selected {
        border_color = accent;
    }

    radio_widget::Style {
        background,
        dot_color,
        border_width: 1.0,
        border_color,
        text_color: Some(palette.foreground),
    }
}
