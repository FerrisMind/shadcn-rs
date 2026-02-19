use iced::widget::{column, container, text};
use iced::{Element, Length};

use crate::theme::Theme;

/// Properties for the Empty state component.
#[derive(Clone, Debug)]
pub struct EmptyProps<'a> {
    pub title: &'a str,
    pub description: Option<&'a str>,
    pub icon: Option<&'a str>,
}

impl<'a> EmptyProps<'a> {
    pub fn new(title: &'a str) -> Self {
        Self {
            title,
            description: None,
            icon: None,
        }
    }

    pub fn description(mut self, description: &'a str) -> Self {
        self.description = Some(description);
        self
    }

    pub fn icon(mut self, icon: &'a str) -> Self {
        self.icon = Some(icon);
        self
    }
}

/// Render an empty state placeholder.
pub fn empty<'a, Message: 'a>(
    props: EmptyProps<'a>,
    theme: &Theme,
) -> Element<'a, Message> {
    let fg = theme.palette.foreground;
    let muted = theme.palette.muted_foreground;

    let mut col = column![].spacing(8).align_x(iced::alignment::Horizontal::Center);

    if let Some(icon) = props.icon {
        col = col.push(
            text(icon)
                .size(32)
                .style(move |_t| iced::widget::text::Style { color: Some(muted) }),
        );
    }

    col = col.push(
        text(props.title)
            .size(16)
            .style(move |_t| iced::widget::text::Style { color: Some(fg) }),
    );

    if let Some(desc) = props.description {
        col = col.push(
            text(desc)
                .size(14)
                .style(move |_t| iced::widget::text::Style { color: Some(muted) }),
        );
    }

    container(col)
        .width(Length::Fill)
        .padding([32, 16])
        .center_x(Length::Fill)
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_props_builder() {
        let props = EmptyProps::new("No results")
            .description("Try adjusting your search.")
            .icon("🔍");

        assert_eq!(props.title, "No results");
        assert_eq!(props.description, Some("Try adjusting your search."));
        assert_eq!(props.icon, Some("🔍"));
    }
}
