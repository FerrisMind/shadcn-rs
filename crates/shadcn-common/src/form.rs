//! Backend-agnostic form state and validation primitives.
//!
//! The state model mirrors the data that `formsnap` exposes to
//! `shadcn-svelte` controls: values stay owned by the application, while the
//! form keeps validation messages, touched/dirty status, submission state,
//! constraints, and stable ids available to either an iced or egui renderer.

use std::collections::HashMap;
use std::fmt;

use regex::Regex;

/// A value that can be validated by the shared form helpers.
///
/// Applications with richer controls can still use [`Validator`] directly and
/// map their value into one of these variants, or keep richer values beside
/// the form state and only store the validation result with
/// [`FormState::set_errors`].
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FieldValue {
    /// Text input value.
    Text(String),
    /// Boolean control value, where `false` is considered empty by
    /// [`FieldValue::is_empty`].
    Bool(bool),
    /// Optional selection value.
    Select(Option<String>),
}

impl FieldValue {
    /// Returns the string representation when this is a text or selection
    /// value.
    #[must_use]
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::Text(value) => Some(value),
            Self::Bool(_) => None,
            Self::Select(value) => value.as_deref(),
        }
    }

    /// Returns whether this value is empty for required-field validation.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Text(value) => value.trim().is_empty(),
            Self::Bool(value) => !value,
            Self::Select(value) => value.as_deref().is_none_or(|value| value.trim().is_empty()),
        }
    }
}

/// Determines when a registered validator runs.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ValidationMode {
    /// Validate all fields when [`FormState::validate`] is called.
    #[default]
    OnSubmit,
    /// Validate a field immediately after its value changes.
    OnChange,
    /// Validate a field when [`FormState::blur`] marks it unfocused.
    OnBlur,
    /// Validate after a touched field changes or is blurred.
    OnTouched,
    /// Run validation on change, blur, and submit.
    All,
}

/// Constraint metadata that a renderer can pass to its native control.
///
/// The metadata is deliberately separate from [`Validator`]. It lets iced and
/// egui expose the same semantic information to their controls without
/// requiring either backend to understand the other backend's widget types.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FieldConstraints {
    required: bool,
    min_length: Option<usize>,
    max_length: Option<usize>,
    pattern: Option<String>,
}

impl FieldConstraints {
    /// Creates constraints with no requirements.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            required: false,
            min_length: None,
            max_length: None,
            pattern: None,
        }
    }

    /// Marks a field as required.
    #[must_use]
    pub const fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    /// Sets the minimum number of Unicode scalar values accepted.
    #[must_use]
    pub const fn min_length(mut self, min_length: usize) -> Self {
        self.min_length = Some(min_length);
        self
    }

    /// Sets the maximum number of Unicode scalar values accepted.
    #[must_use]
    pub const fn max_length(mut self, max_length: usize) -> Self {
        self.max_length = Some(max_length);
        self
    }

    /// Stores a regular-expression constraint for a native control.
    ///
    /// This method intentionally does not compile the expression: constraint
    /// metadata is allowed to be forwarded to a renderer that has its own
    /// validation implementation. Use [`pattern`] for a validator that
    /// reports invalid expressions as validation failures instead of panicking.
    #[must_use]
    pub fn pattern(mut self, pattern: impl Into<String>) -> Self {
        self.pattern = Some(pattern.into());
        self
    }

    /// Returns whether the field is required.
    #[must_use]
    pub const fn is_required(&self) -> bool {
        self.required
    }

    /// Returns the minimum length, if configured.
    #[must_use]
    pub const fn min_length_value(&self) -> Option<usize> {
        self.min_length
    }

    /// Returns the maximum length, if configured.
    #[must_use]
    pub const fn max_length_value(&self) -> Option<usize> {
        self.max_length
    }

    /// Returns the configured regular-expression constraint, if any.
    #[must_use]
    pub fn pattern_value(&self) -> Option<&str> {
        self.pattern.as_deref()
    }
}

/// Stable ids connecting a field's label, control, description, and errors.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FormFieldIds {
    control: String,
    label: String,
    description: String,
    errors: String,
}

impl FormFieldIds {
    /// Creates ids from an application-level field name.
    ///
    /// Punctuation becomes `-`, repeated separators collapse, and a name that
    /// does not contain an ASCII letter is prefixed with `field-` so the
    /// resulting id remains useful in CSS selectors and test tooling.
    #[must_use]
    pub fn new(name: &str) -> Self {
        let mut base = String::with_capacity(name.len());
        let mut pending_separator = false;

        for character in name.chars() {
            if character.is_ascii_alphanumeric() {
                if pending_separator && !base.is_empty() {
                    base.push('-');
                }
                pending_separator = false;
                base.push(character.to_ascii_lowercase());
            } else {
                pending_separator = !base.is_empty();
            }
        }

        while base.ends_with('-') {
            base.pop();
        }

        if base.is_empty() {
            base.push_str("field");
        } else if base
            .chars()
            .next()
            .is_some_and(|character| character.is_ascii_digit())
        {
            base.insert_str(0, "field-");
        }

        Self {
            control: base.clone(),
            label: format!("{base}-label"),
            description: format!("{base}-description"),
            errors: format!("{base}-errors"),
        }
    }

    /// Returns the control id.
    #[must_use]
    pub fn control(&self) -> &str {
        &self.control
    }

    /// Returns the label id.
    #[must_use]
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Returns the description id.
    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Returns the validation-message id.
    #[must_use]
    pub fn errors(&self) -> &str {
        &self.errors
    }
}

/// A validator callback owned by a [`FormFieldState`].
pub type Validator = Box<dyn Fn(&FieldValue) -> Option<String> + Send + Sync + 'static>;

/// Runtime state for one registered form field.
///
/// The validator itself is intentionally hidden from the public struct layout;
/// callers configure it through [`FormState::field`] or
/// [`FormState::field_with_constraints`].
pub struct FormFieldState {
    value: FieldValue,
    errors: Vec<String>,
    touched: bool,
    dirty: bool,
    disabled: bool,
    constraints: FieldConstraints,
    ids: FormFieldIds,
    validator: Validator,
    initialized: bool,
    initial: FieldValue,
}

impl fmt::Debug for FormFieldState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("FormFieldState")
            .field("value", &self.value)
            .field("errors", &self.errors)
            .field("touched", &self.touched)
            .field("dirty", &self.dirty)
            .field("disabled", &self.disabled)
            .field("constraints", &self.constraints)
            .field("ids", &self.ids)
            .field("initialized", &self.initialized)
            .finish()
    }
}

impl FormFieldState {
    fn new(name: &str, validator: Validator, constraints: FieldConstraints) -> Self {
        Self {
            value: FieldValue::Text(String::new()),
            errors: Vec::new(),
            touched: false,
            dirty: false,
            disabled: false,
            constraints,
            ids: FormFieldIds::new(name),
            validator,
            initialized: false,
            initial: FieldValue::Text(String::new()),
        }
    }

    /// Returns the current field value.
    #[must_use]
    pub fn value(&self) -> &FieldValue {
        &self.value
    }

    /// Returns the first validation message, if any.
    #[must_use]
    pub fn error(&self) -> Option<&str> {
        self.errors.first().map(String::as_str)
    }

    /// Returns all validation messages in display order.
    #[must_use]
    pub fn errors(&self) -> &[String] {
        &self.errors
    }

    /// Returns whether at least one validation message is present.
    #[must_use]
    pub fn is_invalid(&self) -> bool {
        !self.errors.is_empty()
    }

    /// Returns whether the field has been focused or edited.
    #[must_use]
    pub const fn is_touched(&self) -> bool {
        self.touched
    }

    /// Returns whether the value differs from its initial value.
    #[must_use]
    pub const fn is_dirty(&self) -> bool {
        self.dirty
    }

    /// Returns whether the control should reject user interaction.
    #[must_use]
    pub const fn is_disabled(&self) -> bool {
        self.disabled
    }

    /// Sets the disabled state used by renderer adapters.
    pub fn set_disabled(&mut self, disabled: bool) {
        self.disabled = disabled;
    }

    /// Returns the renderer-neutral constraint metadata.
    #[must_use]
    pub const fn constraints(&self) -> &FieldConstraints {
        &self.constraints
    }

    /// Returns stable ids for this field's related controls.
    #[must_use]
    pub const fn ids(&self) -> &FormFieldIds {
        &self.ids
    }

    /// Replaces validation messages supplied by a server or custom validator.
    pub fn set_errors<I, E>(&mut self, errors: I)
    where
        I: IntoIterator<Item = E>,
        E: Into<String>,
    {
        self.errors = errors
            .into_iter()
            .map(Into::into)
            .filter(|message| !message.is_empty())
            .collect();
    }

    /// Removes all validation messages without changing the field value.
    pub fn clear_errors(&mut self) {
        self.errors.clear();
    }

    /// Runs this field's validator and replaces its messages.
    ///
    /// The boolean result is an intermediate result: `true` means no errors
    /// remain, while `false` means at least one error is available through
    /// [`Self::errors`].
    pub fn validate(&mut self) -> bool {
        self.errors.clear();
        if let Some(error) = (self.validator)(&self.value) {
            self.errors.push(error);
        }
        self.errors.is_empty()
    }

    fn set_value(&mut self, value: FieldValue) -> bool {
        if !self.initialized {
            let changed = self.value != value;
            self.initial = value.clone();
            self.value = value;
            self.initialized = true;
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

    fn reset(&mut self) {
        self.value = self.initial.clone();
        self.errors.clear();
        self.touched = false;
        self.dirty = false;
    }
}

/// Application-owned form state shared by backend renderers.
pub struct FormState {
    mode: ValidationMode,
    fields: HashMap<String, FormFieldState>,
    submit_attempted: bool,
    submitting: bool,
}

impl fmt::Debug for FormState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("FormState")
            .field("mode", &self.mode)
            .field("fields", &self.fields)
            .field("submit_attempted", &self.submit_attempted)
            .field("submitting", &self.submitting)
            .finish()
    }
}

impl Default for FormState {
    fn default() -> Self {
        Self::new(ValidationMode::default())
    }
}

impl FormState {
    /// Creates an empty form with the selected validation mode.
    #[must_use]
    pub fn new(mode: ValidationMode) -> Self {
        Self {
            mode,
            fields: HashMap::new(),
            submit_attempted: false,
            submitting: false,
        }
    }

    /// Returns the active validation mode.
    #[must_use]
    pub const fn mode(&self) -> ValidationMode {
        self.mode
    }

    /// Changes the active validation mode without discarding field state.
    pub fn set_mode(&mut self, mode: ValidationMode) {
        self.mode = mode;
    }

    /// Registers or updates a field validator.
    pub fn field(&mut self, name: impl Into<String>, validator: Validator) {
        self.field_with_constraints(name, validator, FieldConstraints::default());
    }

    /// Registers or updates a field validator and its native-control metadata.
    pub fn field_with_constraints(
        &mut self,
        name: impl Into<String>,
        validator: Validator,
        constraints: FieldConstraints,
    ) {
        let name = name.into();
        let field = self
            .fields
            .entry(name.clone())
            .or_insert_with(|| FormFieldState::new(&name, none(), FieldConstraints::default()));
        field.validator = validator;
        field.constraints = constraints;
    }

    /// Returns the state for a field, if it has been registered or touched.
    #[must_use]
    pub fn field_state(&self, name: &str) -> Option<&FormFieldState> {
        self.fields.get(name)
    }

    /// Returns mutable state for a field, if it has been registered or touched.
    pub fn field_state_mut(&mut self, name: &str) -> Option<&mut FormFieldState> {
        self.fields.get_mut(name)
    }

    /// Iterates over registered fields and their state.
    pub fn iter(&self) -> impl Iterator<Item = (&str, &FormFieldState)> {
        self.fields
            .iter()
            .map(|(name, field)| (name.as_str(), field))
    }

    /// Sets a text field value and returns whether the value changed.
    pub fn set_text(&mut self, name: &str, value: impl Into<String>) -> bool {
        self.set_value(name, FieldValue::Text(value.into()))
    }

    /// Sets a boolean field value and returns whether the value changed.
    pub fn set_bool(&mut self, name: &str, value: bool) -> bool {
        self.set_value(name, FieldValue::Bool(value))
    }

    /// Sets a selection value and returns whether the value changed.
    pub fn set_select(&mut self, name: &str, value: Option<String>) -> bool {
        self.set_value(name, FieldValue::Select(value))
    }

    /// Returns the current value for a field.
    #[must_use]
    pub fn value(&self, name: &str) -> Option<&FieldValue> {
        self.field_state(name).map(FormFieldState::value)
    }

    /// Marks a field as blurred and validates it according to [`Self::mode`].
    pub fn blur(&mut self, name: &str) {
        let mode = self.mode;
        let field = self.ensure_field(name);
        field.touched = true;

        if matches!(
            mode,
            ValidationMode::OnBlur | ValidationMode::OnTouched | ValidationMode::All
        ) {
            field.validate();
        }
    }

    /// Validates all fields and returns whether the form is valid.
    pub fn validate(&mut self) -> bool {
        self.submit_attempted = true;
        self.fields.values_mut().all(FormFieldState::validate)
    }

    /// Returns whether no registered field currently has validation errors.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.fields.values().all(|field| !field.is_invalid())
    }

    /// Returns whether the form has been submitted or submission is in flight.
    #[must_use]
    pub const fn is_submitting(&self) -> bool {
        self.submitting
    }

    /// Marks submission as in flight or complete.
    pub fn set_submitting(&mut self, submitting: bool) {
        self.submitting = submitting;
    }

    /// Returns whether validation has been requested at least once.
    #[must_use]
    pub const fn submit_attempted(&self) -> bool {
        self.submit_attempted
    }

    /// Returns the first error for a named field.
    #[must_use]
    pub fn error(&self, name: &str) -> Option<&str> {
        self.field_state(name).and_then(FormFieldState::error)
    }

    /// Returns all errors for a named field.
    #[must_use]
    pub fn errors(&self, name: &str) -> Option<&[String]> {
        self.field_state(name).map(FormFieldState::errors)
    }

    /// Replaces errors for a named field, creating an unvalidated field if
    /// necessary.
    pub fn set_errors<I, E>(&mut self, name: &str, errors: I)
    where
        I: IntoIterator<Item = E>,
        E: Into<String>,
    {
        self.ensure_field(name).set_errors(errors);
    }

    /// Clears errors for a named field.
    pub fn clear_errors(&mut self, name: &str) {
        if let Some(field) = self.field_state_mut(name) {
            field.clear_errors();
        }
    }

    /// Restores all fields to their initial values and clears interaction
    /// state and submission state.
    pub fn reset(&mut self) {
        for field in self.fields.values_mut() {
            field.reset();
        }
        self.submit_attempted = false;
        self.submitting = false;
    }

    fn set_value(&mut self, name: &str, value: FieldValue) -> bool {
        let mode = self.mode;
        let submit_attempted = self.submit_attempted;
        let field = self.ensure_field(name);
        let changed = field.set_value(value);
        if !changed {
            return false;
        }

        let should_validate = match mode {
            ValidationMode::OnChange | ValidationMode::All => true,
            ValidationMode::OnTouched => field.touched,
            ValidationMode::OnSubmit | ValidationMode::OnBlur => submit_attempted,
        };
        if should_validate {
            field.validate();
        }
        true
    }

    fn ensure_field(&mut self, name: &str) -> &mut FormFieldState {
        self.fields
            .entry(name.to_owned())
            .or_insert_with(|| FormFieldState::new(name, none(), FieldConstraints::default()))
    }
}

/// Returns a validator that always succeeds.
#[must_use]
pub fn none() -> Validator {
    Box::new(|_| None)
}

/// Composes validators and reports the first failure.
#[must_use]
pub fn compose(validators: Vec<Validator>) -> Validator {
    Box::new(move |value| validators.iter().find_map(|validator| validator(value)))
}

/// Returns a validator that rejects empty text, false booleans, and empty
/// selections.
#[must_use]
pub fn required(message: impl Into<String>) -> Validator {
    let message = message.into();
    Box::new(move |value| value.is_empty().then(|| message.clone()))
}

/// Returns a validator that rejects strings shorter than `minimum`.
#[must_use]
pub fn min_length(minimum: usize, message: impl Into<String>) -> Validator {
    let message = message.into();
    Box::new(move |value| {
        let length = value.as_str().map_or(0, |text| text.chars().count());
        (length < minimum).then(|| message.clone())
    })
}

/// Returns a validator that rejects strings longer than `maximum`.
#[must_use]
pub fn max_length(maximum: usize, message: impl Into<String>) -> Validator {
    let message = message.into();
    Box::new(move |value| {
        let length = value.as_str().map_or(0, |text| text.chars().count());
        (length > maximum).then(|| message.clone())
    })
}

/// Returns a validator backed by a regular expression.
///
/// Empty optional values pass, matching browser constraint-validation
/// semantics. An invalid expression is treated as a validation failure rather
/// than causing a panic.
#[must_use]
pub fn pattern(expression: &str, message: impl Into<String>) -> Validator {
    let message = message.into();
    let compiled = Regex::new(expression).ok();
    Box::new(move |value| {
        let text = value.as_str()?;
        if text.is_empty() {
            return None;
        }

        match &compiled {
            Some(regex) if regex.is_match(text) => None,
            _ => Some(message.clone()),
        }
    })
}

/// Returns a validator for a conventional email address.
#[must_use]
pub fn email(message: impl Into<String>) -> Validator {
    pattern(r"^[^@\s]+@[^@\s]+\.[^@\s]+$", message)
}
