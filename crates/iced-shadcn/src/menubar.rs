use iced::widget::{container, row, text};
use iced::{Background, Element, Length};

use crate::theme::Theme;

/// A single menubar item.
#[derive(Clone, Debug)]
pub struct MenubarItem<'a> {
    pub label: &'a str,
    pub disabled: bool,
}

impl<'a> MenubarItem<'a> {
    pub fn new(label: &'a str) -> Self {
        Self {
            label,
            disabled: false,
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

/// Properties for the Menubar component.
#[derive(Clone, Debug, Default)]
pub struct MenubarProps {
    pub disabled: bool,
}

impl MenubarProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

/// Render a menubar with items.
pub fn menubar<'a, Message: 'a>(
    items: Vec<MenubarItem<'a>>,
    props: MenubarProps,
    theme: &Theme,
) -> Element<'a, Message> {
    let bg = theme.palette.background;
    let border_color = theme.palette.border;
    let fg = theme.palette.foreground;
    let muted = theme.palette.muted_foreground;

    let mut item_row = row![].spacing(0);

    for item in items {
        let color = if item.disabled || props.disabled {
            muted
        } else {
            fg
        };
        let item_widget = container(
            text(item.label)
                .size(14)
                .style(move |_t| iced::widget::text::Style { color: Some(color) }),
        )
        .padding([6, 12]);

        item_row = item_row.push(item_widget);
    }

    container(item_row)
        .width(Length::Fill)
        .style(move |_t: &iced::Theme| iced::widget::container::Style {
            background: Some(Background::Color(bg)),
            border: iced::border::Border {
                color: border_color,
                width: 1.0,
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
    fn menubar_props_default() {
        let props = MenubarProps::new();
        assert!(!props.disabled);
    }

    #[test]
    fn menubar_item_builder() {
        let item = MenubarItem::new("File").disabled(true);
        assert_eq!(item.label, "File");
        assert!(item.disabled);
    }
}
