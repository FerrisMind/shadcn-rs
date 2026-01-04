use iced::Color;
use iced::Font;
use iced::font::Weight;
use iced::widget::Text;
use iced::widget::text::{IntoFragment, LineHeight, Style};

use crate::theme::Theme;

#[derive(Clone, Copy, Debug)]
pub enum TextVariant {
    H1,
    H2,
    H3,
    H4,
    Lead,
    Large,
    Body,
    Small,
    Muted,
    Label,
}

impl TextVariant {
    fn size(self) -> u32 {
        match self {
            TextVariant::H1 => 36,
            TextVariant::H2 => 30,
            TextVariant::H3 => 24,
            TextVariant::H4 => 20,
            TextVariant::Lead => 20,
            TextVariant::Large => 18,
            TextVariant::Body => 16,
            TextVariant::Small => 14,
            TextVariant::Muted => 14,
            TextVariant::Label => 14,
        }
    }

    fn color(self, theme: &Theme) -> Color {
        match self {
            TextVariant::Lead | TextVariant::Muted => theme.palette.muted_foreground,
            _ => theme.palette.foreground,
        }
    }

    fn font(self) -> Option<Font> {
        let weight = match self {
            TextVariant::H1 => Weight::ExtraBold,
            TextVariant::H2 | TextVariant::H3 | TextVariant::H4 => Weight::Semibold,
            TextVariant::Large => Weight::Semibold,
            TextVariant::Small | TextVariant::Label => Weight::Medium,
            _ => return None,
        };

        Some(Font {
            weight,
            ..Font::DEFAULT
        })
    }

    fn line_height(self) -> Option<LineHeight> {
        match self {
            TextVariant::Lead | TextVariant::Body | TextVariant::Muted => {
                Some(LineHeight::Relative(1.75))
            }
            _ => None,
        }
    }
}

pub fn text<'a>(content: impl IntoFragment<'a>, variant: TextVariant, theme: &Theme) -> Text<'a> {
    let color = variant.color(theme);
    let mut widget = Text::new(content)
        .size(variant.size())
        .style(move |_theme| Style { color: Some(color) });

    if let Some(line_height) = variant.line_height() {
        widget = widget.line_height(line_height);
    }

    if let Some(font) = variant.font() {
        widget = widget.font(font);
    }

    widget
}
