//! Field component - form field wrapper with label, description, and error.
//!
//! # Example
//! ```ignore
//! field(ui, &theme, FieldProps::new().label("Email").required(true), |ui| {
//!     input(ui, &theme, ...);
//! });
//! ```

use crate::theme::Theme;
use egui::{RichText, Ui};

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
pub fn field<R>(
    ui: &mut Ui,
    theme: &Theme,
    props: FieldProps<'_>,
    add_control: impl FnOnce(&mut Ui) -> R,
) -> R {
    ui.vertical(|ui| {
        ui.spacing_mut().item_spacing.y = 4.0;

        if let Some(label) = props.label {
            let label_color = if props.error.is_some() {
                theme.palette.destructive
            } else if props.disabled {
                theme.palette.muted_foreground
            } else {
                theme.palette.foreground
            };

            let label_text = if props.required {
                format!("{label} *")
            } else {
                label.to_string()
            };

            ui.label(RichText::new(label_text).size(14.0).color(label_color));
        }

        let result = add_control(ui);

        if let Some(desc) = props.description {
            ui.label(
                RichText::new(desc)
                    .size(12.0)
                    .color(theme.palette.muted_foreground),
            );
        }

        if let Some(err) = props.error {
            ui.label(
                RichText::new(err)
                    .size(12.0)
                    .color(theme.palette.destructive),
            );
        }

        result
    })
    .inner
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
