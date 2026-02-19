use iced::Element;
use iced::widget::{column, text};

use crate::theme::Theme;

/// Validation mode for form fields.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ValidationMode {
    #[default]
    OnSubmit,
    OnChange,
}

/// Value held by a form field.
#[derive(Clone, Debug, PartialEq)]
pub enum FieldValue {
    Text(String),
    Bool(bool),
    Select(Option<String>),
}

impl FieldValue {
    pub fn as_str(&self) -> Option<&str> {
        match self {
            FieldValue::Text(v) => Some(v.as_str()),
            FieldValue::Select(v) => v.as_deref(),
            FieldValue::Bool(_) => None,
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            FieldValue::Text(v) => v.trim().is_empty(),
            FieldValue::Select(v) => v.as_deref().is_none_or(|s| s.trim().is_empty()),
            FieldValue::Bool(v) => !*v,
        }
    }
}

pub type Validator = Box<dyn Fn(&FieldValue) -> Option<String> + 'static>;

/// Returns a no-op validator.
pub fn none() -> Validator {
    Box::new(|_| None)
}

/// Returns a validator that requires a non-empty value.
pub fn required(message: impl Into<String>) -> Validator {
    let msg = message.into();
    Box::new(move |v| {
        if v.is_empty() {
            Some(msg.clone())
        } else {
            None
        }
    })
}

/// Returns a validator that requires a minimum string length.
pub fn min_length(min: usize, message: impl Into<String>) -> Validator {
    let msg = message.into();
    Box::new(move |v| {
        let len = v.as_str().map(|s| s.chars().count()).unwrap_or(0);
        if len < min { Some(msg.clone()) } else { None }
    })
}

/// Composes multiple validators, returning the first error.
pub fn compose(validators: Vec<Validator>) -> Validator {
    Box::new(move |v| {
        for validator in &validators {
            if let Some(err) = validator(v) {
                return Some(err);
            }
        }
        None
    })
}

struct FieldState {
    value: FieldValue,
    error: Option<String>,
    touched: bool,
    dirty: bool,
    validator: Validator,
    initialized: bool,
    initial: FieldValue,
}

impl FieldState {
    fn new(validator: Validator) -> Self {
        Self {
            value: FieldValue::Text(String::new()),
            error: None,
            touched: false,
            dirty: false,
            validator,
            initialized: false,
            initial: FieldValue::Text(String::new()),
        }
    }

    fn set_value(&mut self, value: FieldValue) -> bool {
        if !self.initialized {
            self.initial = value.clone();
            self.initialized = true;
            // First set: changed if value is non-empty (differs from zero-value)
            let changed = !value.is_empty();
            self.value = value;
            self.dirty = changed;
            if changed {
                self.touched = true;
            }
            return changed;
        }
        if self.value == value {
            return false;
        }
        self.value = value;
        self.dirty = self.value != self.initial;
        self.touched = true;
        true
    }

    fn validate(&mut self) -> bool {
        self.error = (self.validator)(&self.value);
        self.error.is_none()
    }
}

/// Form state manager for iced applications.
///
/// Store this in your application state and pass `&mut FormState` to form helpers.
#[derive(Default)]
pub struct FormState {
    pub mode: ValidationMode,
    fields: std::collections::HashMap<String, FieldState>,
    submit_attempted: bool,
}

impl FormState {
    pub fn new(mode: ValidationMode) -> Self {
        Self {
            mode,
            fields: std::collections::HashMap::new(),
            submit_attempted: false,
        }
    }

    /// Register a field with a validator.
    pub fn field(&mut self, name: impl Into<String>, validator: Validator) {
        let name = name.into();
        let entry = self
            .fields
            .entry(name)
            .or_insert_with(|| FieldState::new(none()));
        entry.validator = validator;
    }

    /// Set a text field value. Returns true if changed.
    pub fn set_text(&mut self, name: &str, value: impl Into<String>) -> bool {
        self.set_value(name, FieldValue::Text(value.into()))
    }

    /// Set a bool field value. Returns true if changed.
    pub fn set_bool(&mut self, name: &str, value: bool) -> bool {
        self.set_value(name, FieldValue::Bool(value))
    }

    /// Set a select field value. Returns true if changed.
    pub fn set_select(&mut self, name: &str, value: Option<String>) -> bool {
        self.set_value(name, FieldValue::Select(value))
    }

    /// Validate all fields. Returns true if all valid.
    pub fn validate(&mut self) -> bool {
        self.submit_attempted = true;
        let mut valid = true;
        for field in self.fields.values_mut() {
            if !field.validate() {
                valid = false;
            }
        }
        valid
    }

    /// Returns true if all fields are currently valid.
    pub fn is_valid(&self) -> bool {
        self.fields.values().all(|f| f.error.is_none())
    }

    /// Get error for a specific field.
    pub fn error(&self, name: &str) -> Option<&str> {
        self.fields.get(name).and_then(|f| f.error.as_deref())
    }

    /// Get value for a specific field.
    pub fn value(&self, name: &str) -> Option<&FieldValue> {
        self.fields.get(name).map(|f| &f.value)
    }

    fn set_value(&mut self, name: &str, value: FieldValue) -> bool {
        let mode = self.mode;
        let submit_attempted = self.submit_attempted;
        let field = self
            .fields
            .entry(name.to_string())
            .or_insert_with(|| FieldState::new(none()));
        let changed = field.set_value(value);
        if !changed {
            return false;
        }
        let should_validate = match mode {
            ValidationMode::OnChange => true,
            ValidationMode::OnSubmit => submit_attempted,
        };
        if should_validate {
            field.validate();
        }
        changed
    }
}

/// Render a form field row with label and optional error.
pub fn form_item<'a, Message: 'a>(
    label: &'a str,
    required: bool,
    error: Option<&'a str>,
    control: impl Into<Element<'a, Message>>,
    theme: &Theme,
) -> Element<'a, Message> {
    let fg = theme.palette.foreground;
    let _muted = theme.palette.muted_foreground;
    let destructive = theme.palette.destructive;

    let label_color = if error.is_some() { destructive } else { fg };
    let label_str = if required {
        format!("{label} *")
    } else {
        label.to_string()
    };

    let mut col = column![
        text(label_str)
            .size(14)
            .style(move |_t| iced::widget::text::Style {
                color: Some(label_color)
            }),
        control.into(),
    ]
    .spacing(4);

    if let Some(err) = error {
        col = col.push(
            text(err)
                .size(12)
                .style(move |_t| iced::widget::text::Style {
                    color: Some(destructive),
                }),
        );
    }

    col.into()
}

/// Render a form description text.
pub fn form_description<'a, Message: 'a>(
    description: &'a str,
    theme: &Theme,
) -> Element<'a, Message> {
    let muted = theme.palette.muted_foreground;
    text(description)
        .size(12)
        .style(move |_t| iced::widget::text::Style { color: Some(muted) })
        .into()
}

/// Render a form error message.
pub fn form_message<'a, Message: 'a>(
    error: Option<&'a str>,
    theme: &Theme,
) -> Option<Element<'a, Message>> {
    let err = error?;
    let destructive = theme.palette.destructive;
    Some(
        text(err)
            .size(12)
            .style(move |_t| iced::widget::text::Style {
                color: Some(destructive),
            })
            .into(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn form_state_set_text() {
        let mut state = FormState::new(ValidationMode::OnChange);
        state.field("email", required("Email is required"));
        let changed = state.set_text("email", "test@example.com");
        assert!(changed);
        assert_eq!(
            state.value("email"),
            Some(&FieldValue::Text("test@example.com".to_string()))
        );
    }

    #[test]
    fn form_state_validate_required() {
        let mut state = FormState::new(ValidationMode::OnSubmit);
        state.field("name", required("Name is required"));
        state.set_text("name", "");
        let valid = state.validate();
        assert!(!valid);
        assert_eq!(state.error("name"), Some("Name is required"));
    }

    #[test]
    fn form_state_valid() {
        let mut state = FormState::new(ValidationMode::OnSubmit);
        state.field("name", required("Name is required"));
        state.set_text("name", "Alice");
        let valid = state.validate();
        assert!(valid);
        assert!(state.error("name").is_none());
    }

    #[test]
    fn field_value_is_empty() {
        assert!(FieldValue::Text(String::new()).is_empty());
        assert!(!FieldValue::Text("hello".to_string()).is_empty());
        assert!(FieldValue::Bool(false).is_empty());
        assert!(!FieldValue::Bool(true).is_empty());
    }
}
