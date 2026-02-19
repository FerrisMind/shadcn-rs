use iced::border::Border;
use iced::widget::{column, container, row, text};
use iced::{Background, Color, Element, Length};

use crate::theme::Theme;

/// Visual style variants for the alert.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AlertVariant {
    #[default]
    Default,
    Destructive,
    Warning,
    Success,
    Info,
}

/// Properties for the Alert component.
#[derive(Clone, Debug)]
pub struct AlertProps<'a> {
    pub variant: AlertVariant,
    pub title: Option<&'a str>,
    pub description: &'a str,
}

impl<'a> AlertProps<'a> {
    pub fn new(description: &'a str) -> Self {
        Self {
            variant: AlertVariant::Default,
            title: None,
            description,
        }
    }

    pub fn variant(mut self, variant: AlertVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn title(mut self, title: &'a str) -> Self {
        self.title = Some(title);
        self
    }
}

fn alert_colors(variant: AlertVariant, theme: &Theme) -> (Color, Color, Color, &'static str) {
    match variant {
        AlertVariant::Default => (
            Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.05,
            },
            theme.palette.border,
            theme.palette.foreground,
            "ℹ",
        ),
        AlertVariant::Destructive => (
            Color {
                r: 0.996,
                g: 0.792,
                b: 0.792,
                a: 0.3,
            },
            Color {
                r: 0.937,
                g: 0.267,
                b: 0.267,
                a: 1.0,
            },
            Color {
                r: 0.937,
                g: 0.267,
                b: 0.267,
                a: 1.0,
            },
            "⚠",
        ),
        AlertVariant::Warning => (
            Color {
                r: 0.996,
                g: 0.953,
                b: 0.780,
                a: 0.5,
            },
            Color {
                r: 0.961,
                g: 0.620,
                b: 0.043,
                a: 1.0,
            },
            Color {
                r: 0.961,
                g: 0.620,
                b: 0.043,
                a: 1.0,
            },
            "⚠",
        ),
        AlertVariant::Success => (
            Color {
                r: 0.733,
                g: 0.969,
                b: 0.816,
                a: 0.3,
            },
            Color {
                r: 0.133,
                g: 0.773,
                b: 0.369,
                a: 1.0,
            },
            Color {
                r: 0.133,
                g: 0.773,
                b: 0.369,
                a: 1.0,
            },
            "✓",
        ),
        AlertVariant::Info => (
            Color {
                r: 0.749,
                g: 0.859,
                b: 0.996,
                a: 0.3,
            },
            Color {
                r: 0.231,
                g: 0.510,
                b: 0.965,
                a: 1.0,
            },
            Color {
                r: 0.231,
                g: 0.510,
                b: 0.965,
                a: 1.0,
            },
            "ℹ",
        ),
    }
}

/// Render an alert message.
pub fn alert<'a, Message: 'a>(props: AlertProps<'a>, theme: &Theme) -> Element<'a, Message> {
    let (bg, border_color, icon_color, icon) = alert_colors(props.variant, theme);

    let icon_widget = text(icon)
        .size(18)
        .style(move |_t| iced::widget::text::Style {
            color: Some(icon_color),
        });

    let mut content_col = column![].spacing(4);

    if let Some(title) = props.title {
        let fg = theme.palette.foreground;
        content_col = content_col.push(
            text(title)
                .size(14)
                .style(move |_t| iced::widget::text::Style { color: Some(fg) }),
        );
    }

    let muted = theme.palette.muted_foreground;
    content_col = content_col.push(
        text(props.description)
            .size(14)
            .style(move |_t| iced::widget::text::Style { color: Some(muted) }),
    );

    let inner = row![icon_widget, content_col].spacing(12);

    let style = move |_t: &iced::Theme| container::Style {
        background: Some(Background::Color(bg)),
        border: Border {
            color: border_color,
            width: 1.0,
            radius: 6.0.into(),
        },
        ..Default::default()
    };

    container(inner)
        .padding([12, 16])
        .width(Length::Fill)
        .style(style)
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alert_variant_default() {
        assert_eq!(AlertVariant::default(), AlertVariant::Default);
    }

    #[test]
    fn alert_props_builder() {
        let props = AlertProps::new("Test message")
            .variant(AlertVariant::Success)
            .title("Success!");

        assert_eq!(props.variant, AlertVariant::Success);
        assert_eq!(props.title, Some("Success!"));
        assert_eq!(props.description, "Test message");
    }
}
