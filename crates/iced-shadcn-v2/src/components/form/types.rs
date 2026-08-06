//! Public metadata types used by the iced form builders.

use shadcn_common::FormFieldState;

/// Renderer-neutral properties for one form control.
///
/// The properties connect a control to the ids of its label, description, and
/// validation messages. The iced backend has no DOM accessibility tree, so
/// these values are exposed to applications and to [`FormControlExt`] adapters
/// instead of being written as HTML attributes.
///
/// [`FormControlExt`]: super::FormControlExt
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[must_use = "builders do nothing unless turned into a form control"]
pub struct FormControlProps {
    control_id: Option<String>,
    label_id: Option<String>,
    description_id: Option<String>,
    errors_id: Option<String>,
    described_by: Vec<String>,
    invalid: bool,
    disabled: bool,
    required: bool,
}

impl FormControlProps {
    /// Creates unassociated control properties.
    pub const fn new() -> Self {
        Self {
            control_id: None,
            label_id: None,
            description_id: None,
            errors_id: None,
            described_by: Vec::new(),
            invalid: false,
            disabled: false,
            required: false,
        }
    }

    /// Creates control properties from a shared field state.
    pub fn from_field(field: &FormFieldState) -> Self {
        let ids = field.ids();
        let mut props = Self {
            control_id: Some(ids.control().to_owned()),
            label_id: Some(ids.label().to_owned()),
            description_id: Some(ids.description().to_owned()),
            errors_id: Some(ids.errors().to_owned()),
            described_by: vec![ids.description().to_owned(), ids.errors().to_owned()],
            invalid: field.is_invalid(),
            disabled: field.is_disabled(),
            required: field.constraints().is_required(),
        };
        props.described_by.shrink_to_fit();
        props
    }

    /// Sets the control id.
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.control_id = Some(id.into());
        self
    }

    /// Associates a label id.
    pub fn with_label_id(mut self, id: impl Into<String>) -> Self {
        self.label_id = Some(id.into());
        self
    }

    /// Associates a description id.
    pub fn with_description_id(mut self, id: impl Into<String>) -> Self {
        self.description_id = Some(id.into());
        self
    }

    /// Associates a validation-message id.
    pub fn with_errors_id(mut self, id: impl Into<String>) -> Self {
        self.errors_id = Some(id.into());
        self
    }

    /// Replaces the ids announced by the control as supporting text.
    pub fn described_by<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.described_by = ids.into_iter().map(Into::into).collect();
        self
    }

    /// Sets the invalid state.
    pub const fn invalid(mut self, invalid: bool) -> Self {
        self.invalid = invalid;
        self
    }

    /// Sets the disabled state.
    pub const fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Sets whether the control is required.
    pub const fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    /// Returns the control id, if associated.
    #[must_use]
    pub fn control_id(&self) -> Option<&str> {
        self.control_id.as_deref()
    }

    /// Returns the label id, if associated.
    #[must_use]
    pub fn label_id(&self) -> Option<&str> {
        self.label_id.as_deref()
    }

    /// Returns the description id, if associated.
    #[must_use]
    pub fn description_id(&self) -> Option<&str> {
        self.description_id.as_deref()
    }

    /// Returns the validation-message id, if associated.
    #[must_use]
    pub fn errors_id(&self) -> Option<&str> {
        self.errors_id.as_deref()
    }

    /// Returns ids of supporting text announced by the control.
    #[must_use]
    pub fn described_by_ids(&self) -> &[String] {
        &self.described_by
    }

    /// Returns whether the control is invalid.
    #[must_use]
    pub const fn is_invalid(&self) -> bool {
        self.invalid
    }

    /// Returns whether the control is disabled.
    #[must_use]
    pub const fn is_disabled(&self) -> bool {
        self.disabled
    }

    /// Returns whether the control is required.
    #[must_use]
    pub const fn is_required(&self) -> bool {
        self.required
    }
}
