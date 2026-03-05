use iced::Element;
use iced::widget::{column, text};

use crate::theme::Theme;

/// Properties for a form field wrapper.
#[derive(Clone, Debug)]
pub struct FieldProps<'a> {
    pub label: Option<&'a str>,
    pub description: Option<&'a str>,
    pub error: Option<&'a str>,
    pub required: bool,
    pub disabled: bool,
}

impl<'a> FieldProps<'a> {
    pub fn new() -> Self {
        Self {
            label: None,
            description: None,
            error: None,
            required: false,
            disabled: false,
        }
    }

    pub fn label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }

    pub fn description(mut self, description: &'a str) -> Self {
        self.description = Some(description);
        self
    }

    pub fn error(mut self, error: &'a str) -> Self {
        self.error = Some(error);
        self
    }

    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl<'a> Default for FieldProps<'a> {
    fn default() -> Self {
        Self::new()
    }
}

/// Render a form field wrapper with label, description, control, and error.
pub fn field<'a, Message: 'a>(
    props: FieldProps<'a>,
    control: impl Into<Element<'a, Message>>,
    theme: &Theme,
) -> Element<'a, Message> {
    let fg = theme.palette.foreground;
    let muted = theme.palette.muted_foreground;
    let destructive = theme.palette.destructive;

    let mut col = column![].spacing(theme.styles.field.spacing);

    if let Some(label) = props.label {
        let label_color = if props.error.is_some() {
            destructive
        } else if props.disabled {
            muted
        } else {
            fg
        };

        let label_str = if props.required {
            format!("{label} *")
        } else {
            label.to_string()
        };

        col = col.push(
            text(label_str)
                .size(theme.styles.field.label_size)
                .style(move |_t| iced::widget::text::Style {
                    color: Some(label_color),
                }),
        );
    }

    col = col.push(control.into());

    if let Some(desc) = props.description {
        col = col.push(
            text(desc)
                .size(theme.styles.field.description_size)
                .style(move |_t| iced::widget::text::Style { color: Some(muted) }),
        );
    }

    if let Some(err) = props.error {
        col = col.push(
            text(err)
                .size(theme.styles.field.error_size)
                .style(move |_t| iced::widget::text::Style {
                    color: Some(destructive),
                }),
        );
    }

    col.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn field_props_builder() {
        let props = FieldProps::new()
            .label("Email")
            .description("Enter your email address.")
            .error("Invalid email")
            .required(true)
            .disabled(false);

        assert_eq!(props.label, Some("Email"));
        assert_eq!(props.description, Some("Enter your email address."));
        assert_eq!(props.error, Some("Invalid email"));
        assert!(props.required);
        assert!(!props.disabled);
    }

    #[test]
    fn field_props_default() {
        let props = FieldProps::default();
        assert!(props.label.is_none());
        assert!(!props.required);
    }
}
