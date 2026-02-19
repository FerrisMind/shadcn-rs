use iced::widget::{column, container, text};
use iced::{Element, Length};

use crate::theme::Theme;

/// Side from which the drawer slides in.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DrawerSide {
    Top,
    #[default]
    Right,
    Bottom,
    Left,
}

/// Properties for the Drawer component.
#[derive(Clone, Debug)]
pub struct DrawerProps<'a> {
    pub open: bool,
    pub side: DrawerSide,
    pub title: Option<&'a str>,
    pub description: Option<&'a str>,
}

impl<'a> DrawerProps<'a> {
    pub fn new(open: bool) -> Self {
        Self {
            open,
            side: DrawerSide::Bottom,
            title: None,
            description: None,
        }
    }

    pub fn side(mut self, side: DrawerSide) -> Self {
        self.side = side;
        self
    }

    pub fn title(mut self, title: &'a str) -> Self {
        self.title = Some(title);
        self
    }

    pub fn description(mut self, description: &'a str) -> Self {
        self.description = Some(description);
        self
    }
}

/// Render a drawer panel.
///
/// Drawer is similar to Sheet but defaults to Bottom side and is typically
/// used for mobile-style bottom sheets. In iced, renders as a positioned
/// container when `open = true`.
pub fn drawer<'a, Message: 'a>(
    props: DrawerProps<'a>,
    content: impl Into<Element<'a, Message>>,
    theme: &Theme,
) -> Element<'a, Message> {
    if !props.open {
        return container(column![])
            .width(Length::Shrink)
            .height(Length::Shrink)
            .into();
    }

    let bg = theme.palette.background;
    let border_color = theme.palette.border;
    let fg = theme.palette.foreground;
    let muted = theme.palette.muted_foreground;

    let mut header_col = column![].spacing(4);

    if let Some(title) = props.title {
        header_col = header_col.push(
            text(title)
                .size(16)
                .style(move |_t| iced::widget::text::Style { color: Some(fg) }),
        );
    }

    if let Some(desc) = props.description {
        header_col = header_col.push(
            text(desc)
                .size(12)
                .style(move |_t| iced::widget::text::Style { color: Some(muted) }),
        );
    }

    let body = column![header_col, content.into()].spacing(16);

    let (width, height) = match props.side {
        DrawerSide::Left | DrawerSide::Right => (Length::Fixed(320.0), Length::Fill),
        DrawerSide::Top | DrawerSide::Bottom => (Length::Fill, Length::Fixed(280.0)),
    };

    container(body)
        .width(width)
        .height(height)
        .padding(24)
        .style(move |_t: &iced::Theme| iced::widget::container::Style {
            background: Some(iced::Background::Color(bg)),
            border: iced::border::Border {
                color: border_color,
                width: 1.0,
                radius: 12.0.into(),
            },
            ..Default::default()
        })
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drawer_side_default() {
        assert_eq!(DrawerSide::default(), DrawerSide::Right);
    }

    #[test]
    fn drawer_props_builder() {
        let props = DrawerProps::new(true)
            .side(DrawerSide::Bottom)
            .title("Actions")
            .description("Choose an action.");

        assert!(props.open);
        assert_eq!(props.side, DrawerSide::Bottom);
        assert_eq!(props.title, Some("Actions"));
    }
}
