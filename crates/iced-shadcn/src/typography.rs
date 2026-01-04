use iced::Color;
use iced::widget::Text;
use iced::widget::text::{IntoFragment, Style};

use crate::theme::Theme;

#[derive(Clone, Copy, Debug)]
pub enum TextVariant {
    H1,
    H2,
    H3,
    H4,
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
            TextVariant::Large => 18,
            TextVariant::Body => 16,
            TextVariant::Small => 14,
            TextVariant::Muted => 14,
            TextVariant::Label => 14,
        }
    }

    fn color(self, theme: &Theme) -> Color {
        match self {
            TextVariant::Muted => theme.palette.muted_foreground,
            _ => theme.palette.foreground,
        }
    }
}

pub fn text<'a>(content: impl IntoFragment<'a>, variant: TextVariant, theme: &Theme) -> Text<'a> {
    let color = variant.color(theme);
    Text::new(content)
        .size(variant.size())
        .style(move |_theme| Style { color: Some(color) })
}
