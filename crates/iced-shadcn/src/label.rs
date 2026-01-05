use crate::theme::Theme;
use crate::typography::{TextProps, TextSize, TextWeight, text};

pub fn label<'a>(
    content: impl iced::widget::text::IntoFragment<'a>,
    theme: &Theme,
) -> iced::widget::Text<'a> {
    text(
        content,
        TextProps::new()
            .size(TextSize::Two)
            .weight(TextWeight::Medium),
        theme,
    )
}
