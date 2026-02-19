use iced::widget::{column, container, text};
use iced::{Element, Length, Padding};

use crate::theme::Theme;

/// Side from which the sheet slides in.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SheetSide {
    Top,
    #[default]
    Right,
    Bottom,
    Left,
}

/// Properties for the Sheet component.
#[derive(Clone, Debug)]
pub struct SheetProps<'a> {
    pub open: bool,
    pub side: SheetSide,
    pub title: Option<&'a str>,
    pub description: Option<&'a str>,
}

impl<'a> SheetProps<'a> {
    pub fn new(open: bool) -> Self {
        Self {
            open,
            side: SheetSide::Right,
            title: None,
            description: None,
        }
    }

    pub fn side(mut self, side: SheetSide) -> Self {
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

/// Render a sheet panel overlay.
///
/// In iced, a full overlay system requires custom widget state management.
/// This renders the sheet content as a positioned container when `open = true`.
/// For production use, integrate with your application's overlay/modal layer.
pub fn sheet<'a, Message: 'a>(
    props: SheetProps<'a>,
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
        SheetSide::Left | SheetSide::Right => (Length::Fixed(384.0), Length::Fill),
        SheetSide::Top | SheetSide::Bottom => (Length::Fill, Length::Fixed(320.0)),
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
                radius: 0.0.into(),
            },
            ..Default::default()
        })
        .into()
}

/// Render sheet header section.
pub fn sheet_header<'a, Message: 'a>(
    content: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    container(content)
        .padding(Padding {
            top: 16.0,
            right: 16.0,
            bottom: 8.0,
            left: 16.0,
        })
        .into()
}

/// Render sheet footer section.
pub fn sheet_footer<'a, Message: 'a>(
    content: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    container(content)
        .padding(Padding {
            top: 8.0,
            right: 16.0,
            bottom: 16.0,
            left: 16.0,
        })
        .into()
}

/// Render sheet title text.
pub fn sheet_title<'a, Message: 'a>(title: impl ToString, theme: &Theme) -> Element<'a, Message> {
    let fg = theme.palette.foreground;
    text(title.to_string())
        .size(16)
        .style(move |_t| iced::widget::text::Style { color: Some(fg) })
        .into()
}

/// Render sheet description text.
pub fn sheet_description<'a, Message: 'a>(
    description: impl ToString,
    theme: &Theme,
) -> Element<'a, Message> {
    let muted = theme.palette.muted_foreground;
    text(description.to_string())
        .size(12)
        .style(move |_t| iced::widget::text::Style { color: Some(muted) })
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sheet_side_default() {
        assert_eq!(SheetSide::default(), SheetSide::Right);
    }

    #[test]
    fn sheet_props_builder() {
        let props = SheetProps::new(true)
            .side(SheetSide::Left)
            .title("Settings")
            .description("Manage your settings here.");

        assert!(props.open);
        assert_eq!(props.side, SheetSide::Left);
        assert_eq!(props.title, Some("Settings"));
        assert_eq!(props.description, Some("Manage your settings here."));
    }

    #[test]
    fn sheet_props_closed() {
        let props = SheetProps::new(false);
        assert!(!props.open);
    }
}
