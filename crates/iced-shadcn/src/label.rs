use crate::theme::Theme;
use crate::typography::{TextVariant, text};

pub fn label<'a>(
    content: impl iced::widget::text::IntoFragment<'a>,
    theme: &Theme,
) -> iced::widget::Text<'a> {
    text(content, TextVariant::Label, theme)
}
