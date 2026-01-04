use iced::widget::rule;

use crate::theme::Theme;

#[derive(Clone, Copy, Debug)]
pub enum SeparatorOrientation {
    Horizontal,
    Vertical,
}

pub fn separator<'a>(orientation: SeparatorOrientation, theme: &Theme) -> rule::Rule<'a> {
    let palette = theme.palette;
    let radius = theme.radius.sm;

    let style = move |_iced_theme: &iced::Theme| rule::Style {
        color: palette.border,
        radius: radius.into(),
        fill_mode: rule::FillMode::Full,
        snap: true,
    };

    match orientation {
        SeparatorOrientation::Horizontal => rule::horizontal(1).style(style),
        SeparatorOrientation::Vertical => rule::vertical(1).style(style),
    }
}
