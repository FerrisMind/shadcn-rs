use iced::widget::{column, container, text};
use iced::{Element, Length};

use crate::theme::Theme;

/// A generic list/grid item component.
#[derive(Clone, Debug)]
pub struct ItemProps<'a> {
    pub title: &'a str,
    pub description: Option<&'a str>,
    pub disabled: bool,
    pub selected: bool,
}

impl<'a> ItemProps<'a> {
    pub fn new(title: &'a str) -> Self {
        Self {
            title,
            description: None,
            disabled: false,
            selected: false,
        }
    }

    pub fn description(mut self, description: &'a str) -> Self {
        self.description = Some(description);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
}

/// Render a generic list item.
pub fn item<'a, Message: 'a>(props: ItemProps<'a>, theme: &Theme) -> Element<'a, Message> {
    let fg = if props.disabled {
        theme.palette.muted_foreground
    } else {
        theme.palette.foreground
    };
    let muted = theme.palette.muted_foreground;

    let bg = if props.selected {
        theme.palette.accent
    } else {
        iced::Color::TRANSPARENT
    };

    let mut col = column![
        text(props.title)
            .size(14)
            .style(move |_t| iced::widget::text::Style { color: Some(fg) })
    ]
    .spacing(2);

    if let Some(desc) = props.description {
        col = col.push(
            text(desc)
                .size(12)
                .style(move |_t| iced::widget::text::Style { color: Some(muted) }),
        );
    }

    container(col)
        .width(Length::Fill)
        .padding([8, 12])
        .style(move |_t: &iced::Theme| iced::widget::container::Style {
            background: Some(iced::Background::Color(bg)),
            border: iced::border::Border {
                color: iced::Color::TRANSPARENT,
                width: 0.0,
                radius: 6.0.into(),
            },
            ..Default::default()
        })
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item_props_builder() {
        let props = ItemProps::new("Item title")
            .description("Item description")
            .disabled(false)
            .selected(true);

        assert_eq!(props.title, "Item title");
        assert_eq!(props.description, Some("Item description"));
        assert!(!props.disabled);
        assert!(props.selected);
    }
}
