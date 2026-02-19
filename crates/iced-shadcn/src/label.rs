use iced::font::Weight;
use iced::widget::{column, container, row, text};
use iced::{Color, Element, Font};

use crate::theme::Theme;
use crate::tokens::{ControlSize, mix};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum LabelVariant {
    #[default]
    Default,
    Secondary,
    Muted,
    Destructive,
}

/// Label styling options.
#[derive(Clone, Debug)]
pub struct LabelProps<'a> {
    pub variant: LabelVariant,
    pub size: ControlSize,
    pub disabled: bool,
    pub required: bool,
    pub description: Option<&'a str>,
    pub as_child: bool,
}

impl<'a> Default for LabelProps<'a> {
    fn default() -> Self {
        Self {
            variant: LabelVariant::Default,
            size: ControlSize::Md,
            disabled: false,
            required: false,
            description: None,
            as_child: false,
        }
    }
}

impl<'a> LabelProps<'a> {
    /// Creates default label props.
    pub fn new() -> Self {
        Self::default()
    }

    pub fn variant(mut self, variant: LabelVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    /// Sets the disabled state.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    pub fn description(mut self, description: &'a str) -> Self {
        self.description = Some(description);
        self
    }

    pub fn as_child(mut self, as_child: bool) -> Self {
        self.as_child = as_child;
        self
    }
}

/// Shadcn-style label with description and variants.
pub fn label<'a, Message: 'a>(content: impl Into<String>, theme: &Theme) -> Element<'a, Message> {
    label_with_props(content, LabelProps::default(), theme)
}

/// Shadcn-style label with custom props.
pub fn label_with_props<'a, Message: 'a>(
    content: impl Into<String>,
    props: LabelProps<'a>,
    theme: &Theme,
) -> Element<'a, Message> {
    let palette = theme.palette;

    let base_color = match props.variant {
        LabelVariant::Default => palette.foreground,
        LabelVariant::Secondary => palette.secondary_foreground,
        LabelVariant::Muted => palette.muted_foreground,
        LabelVariant::Destructive => palette.destructive,
    };

    let desc_color = mix(palette.muted_foreground, palette.foreground, 0.35);
    let required_color = palette.destructive;

    let (color, final_desc_color) = if props.disabled {
        (
            apply_opacity(base_color, 0.5),
            apply_opacity(desc_color, 0.5),
        )
    } else {
        (base_color, desc_color)
    };

    let font_size = match props.size {
        ControlSize::Sm | ControlSize::IconSm => 13,
        ControlSize::Md | ControlSize::Icon => 14,
        ControlSize::Lg | ControlSize::IconLg => 15,
    };

    let mut label_row = row![
        text(content.into())
            .size(font_size)
            .font(Font {
                weight: Weight::Medium,
                ..Font::DEFAULT
            })
            .style(move |_theme| text::Style { color: Some(color) })
    ]
    .spacing(4)
    .align_y(iced::Alignment::Start);

    if props.required {
        label_row = label_row.push(text("*").size(font_size).style(move |_theme| text::Style {
            color: Some(required_color),
        }));
    }

    let mut content_col = column![label_row].spacing(2);

    if let Some(desc) = props.description {
        content_col = content_col.push(text(desc).size(font_size as f32 * 0.9).style(
            move |_theme| text::Style {
                color: Some(final_desc_color),
            },
        ));
    }

    container(content_col).into()
}

fn apply_opacity(color: Color, opacity: f32) -> Color {
    Color {
        a: color.a * opacity,
        ..color
    }
}
