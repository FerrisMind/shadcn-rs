//! Form state management and layout helpers.

use crate::label::{Label, LabelVariant};
use crate::theme::Theme;
use crate::tokens::ControlSize;
use egui::{Id, Response, Ui, WidgetText, vec2};
use regex::Regex;
use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;

pub type Validator = Box<dyn Fn(&FieldValue) -> Option<String> + 'static>;

#[derive(Clone, Debug, PartialEq)]
pub enum FieldValue {
    Text(String),
    Bool(bool),
    Select(Option<String>),
}

impl FieldValue {
    fn as_str(&self) -> Option<&str> {
        match self {
            FieldValue::Text(value) => Some(value.as_str()),
            FieldValue::Select(value) => value.as_deref(),
            FieldValue::Bool(_) => None,
        }
    }

    fn is_empty(&self) -> bool {
        match self {
            FieldValue::Text(value) => value.trim().is_empty(),
            FieldValue::Select(value) => value.as_deref().is_none_or(|val| val.trim().is_empty()),
            FieldValue::Bool(value) => !*value,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ValidationMode {
    #[default]
    OnSubmit,
    OnChange,
    OnBlur,
    OnTouched,
    All,
}

pub struct FieldState {
    pub value: FieldValue,
    pub error: Option<String>,
    pub touched: bool,
    pub dirty: bool,
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
            self.value = value.clone();
            self.initial = value;
            self.initialized = true;
            self.dirty = false;
            return false;
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

impl fmt::Debug for FieldState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FieldState")
            .field("value", &self.value)
            .field("error", &self.error)
            .field("touched", &self.touched)
            .field("dirty", &self.dirty)
            .finish()
    }
}

#[derive(Default)]
pub struct FormState {
    pub mode: ValidationMode,
    fields: HashMap<String, FieldState>,
    submit_attempted: bool,
}

impl FormState {
    pub fn new(mode: ValidationMode) -> Self {
        Self {
            mode,
            fields: HashMap::new(),
            submit_attempted: false,
        }
    }

    pub fn field(&mut self, name: impl Into<String>, validator: Validator) {
        let name = name.into();
        let entry = self
            .fields
            .entry(name)
            .or_insert_with(|| FieldState::new(none()));
        entry.validator = validator;
    }

    pub fn set_text(&mut self, name: &str, value: impl Into<String>) -> bool {
        self.set_value(name, FieldValue::Text(value.into()))
    }

    pub fn set_bool(&mut self, name: &str, value: bool) -> bool {
        self.set_value(name, FieldValue::Bool(value))
    }

    pub fn set_select(&mut self, name: &str, value: Option<String>) -> bool {
        self.set_value(name, FieldValue::Select(value))
    }

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

    pub fn is_valid(&self) -> bool {
        self.fields.values().all(|field| field.error.is_none())
    }

    pub fn errors(&self) -> HashMap<String, String> {
        self.fields
            .iter()
            .filter_map(|(name, field)| field.error.clone().map(|err| (name.clone(), err)))
            .collect()
    }

    pub fn error(&self, name: &str) -> Option<&str> {
        self.fields
            .get(name)
            .and_then(|field| field.error.as_deref())
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
            ValidationMode::OnSubmit => submit_attempted,
            ValidationMode::OnBlur => false,
        };

        if should_validate {
            field.validate();
        }

        changed
    }

    fn ensure_field(&mut self, name: &str) -> &mut FieldState {
        self.fields
            .entry(name.to_string())
            .or_insert_with(|| FieldState::new(none()))
    }
}

pub fn none() -> Validator {
    Box::new(|_| None)
}

pub fn compose(validators: Vec<Validator>) -> Validator {
    Box::new(move |value| {
        for validator in validators.iter() {
            if let Some(error) = validator(value) {
                return Some(error);
            }
        }
        None
    })
}

pub fn required(message: impl Into<String>) -> Validator {
    let message = message.into();
    Box::new(move |value| {
        if value.is_empty() {
            Some(message.clone())
        } else {
            None
        }
    })
}

pub fn min_length(min: usize, message: impl Into<String>) -> Validator {
    let message = message.into();
    Box::new(move |value| {
        let len = value.as_str().map(|val| val.chars().count()).unwrap_or(0);
        if len < min {
            Some(message.clone())
        } else {
            None
        }
    })
}

pub fn max_length(max: usize, message: impl Into<String>) -> Validator {
    let message = message.into();
    Box::new(move |value| {
        let len = value.as_str().map(|val| val.chars().count()).unwrap_or(0);
        if len > max {
            Some(message.clone())
        } else {
            None
        }
    })
}

pub fn pattern(pattern: &str, message: impl Into<String>) -> Validator {
    let message = message.into();
    let compiled = Regex::new(pattern);
    Box::new(move |value| {
        let text = value.as_str()?;

        if text.is_empty() {
            return None;
        }

        match &compiled {
            Ok(regex) => {
                if regex.is_match(text) {
                    None
                } else {
                    Some(message.clone())
                }
            }
            Err(_) => Some(message.clone()),
        }
    })
}

pub fn email(message: impl Into<String>) -> Validator {
    let message = message.into();
    let regex = Regex::new(r"^[^@\\s]+@[^@\\s]+\\.[^@\\s]+$");
    Box::new(move |value| {
        let text = value.as_str()?;

        if text.is_empty() {
            return None;
        }

        match &regex {
            Ok(regex) => {
                if regex.is_match(text) {
                    None
                } else {
                    Some(message.clone())
                }
            }
            Err(_) => Some(message.clone()),
        }
    })
}

#[derive(Clone, Copy, Debug)]
pub struct FormItemContext {
    pub id: Id,
}

pub struct FormItemProps<IdSource> {
    pub id_source: IdSource,
    pub spacing: f32,
}

impl<IdSource> FormItemProps<IdSource> {
    pub fn new(id_source: IdSource) -> Self {
        Self {
            id_source,
            spacing: 6.0,
        }
    }

    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }
}

pub struct FormItem<IdSource> {
    props: FormItemProps<IdSource>,
}

impl<IdSource> FormItem<IdSource> {
    pub fn new(id_source: IdSource) -> Self {
        Self {
            props: FormItemProps::new(id_source),
        }
    }

    pub fn spacing(mut self, spacing: f32) -> Self {
        self.props.spacing = spacing;
        self
    }

    pub fn show<R>(
        self,
        ui: &mut Ui,
        add_contents: impl FnOnce(&mut Ui, &FormItemContext) -> R,
    ) -> R
    where
        IdSource: Hash,
    {
        form_item(ui, self.props, add_contents)
    }
}

pub fn form_item<R, IdSource>(
    ui: &mut Ui,
    props: FormItemProps<IdSource>,
    add_contents: impl FnOnce(&mut Ui, &FormItemContext) -> R,
) -> R
where
    IdSource: Hash,
{
    let id = ui.make_persistent_id(&props.id_source);
    let ctx = FormItemContext { id };

    ui.vertical(|item_ui| {
        item_ui.spacing_mut().item_spacing = vec2(0.0, props.spacing);
        add_contents(item_ui, &ctx)
    })
    .inner
}

#[derive(Clone, Debug)]
pub struct FormLabelProps {
    pub text: WidgetText,
    pub size: ControlSize,
    pub required: bool,
    pub disabled: bool,
    pub error: bool,
}

impl FormLabelProps {
    pub fn new(text: impl Into<WidgetText>) -> Self {
        Self {
            text: text.into(),
            size: ControlSize::Sm,
            required: false,
            disabled: false,
            error: false,
        }
    }

    pub fn size(mut self, size: ControlSize) -> Self {
        self.size = size;
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

    pub fn error(mut self, error: bool) -> Self {
        self.error = error;
        self
    }
}

pub struct FormLabel {
    props: FormLabelProps,
}

impl FormLabel {
    pub fn new(text: impl Into<WidgetText>) -> Self {
        Self {
            props: FormLabelProps::new(text),
        }
    }

    pub fn size(mut self, size: ControlSize) -> Self {
        self.props.size = size;
        self
    }

    pub fn required(mut self, required: bool) -> Self {
        self.props.required = required;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.props.disabled = disabled;
        self
    }

    pub fn error(mut self, error: bool) -> Self {
        self.props.error = error;
        self
    }

    pub fn show(self, ui: &mut Ui, theme: &Theme, ctx: &FormItemContext) -> Response {
        form_label(ui, theme, ctx, self.props)
    }
}

pub fn form_label(
    ui: &mut Ui,
    theme: &Theme,
    ctx: &FormItemContext,
    props: FormLabelProps,
) -> Response {
    let variant = if props.error {
        LabelVariant::Destructive
    } else {
        LabelVariant::Default
    };
    Label::new(props.text)
        .size(props.size)
        .required(props.required)
        .disabled(props.disabled)
        .variant(variant)
        .for_id(ctx.id)
        .show(ui, theme)
}

#[derive(Default)]
pub struct FormControl;

impl FormControl {
    pub fn new() -> Self {
        Self
    }

    pub fn show<R>(
        self,
        ui: &mut Ui,
        ctx: &FormItemContext,
        add_contents: impl FnOnce(&mut Ui, Id) -> R,
    ) -> R {
        form_control(ui, ctx, add_contents)
    }
}

pub fn form_control<R>(
    ui: &mut Ui,
    ctx: &FormItemContext,
    add_contents: impl FnOnce(&mut Ui, Id) -> R,
) -> R {
    add_contents(ui, ctx.id)
}

#[derive(Clone, Debug)]
pub struct FormDescriptionProps {
    pub text: WidgetText,
}

impl FormDescriptionProps {
    pub fn new(text: impl Into<WidgetText>) -> Self {
        Self { text: text.into() }
    }
}

pub struct FormDescription {
    props: FormDescriptionProps,
}

impl FormDescription {
    pub fn new(text: impl Into<WidgetText>) -> Self {
        Self {
            props: FormDescriptionProps::new(text),
        }
    }

    pub fn show(self, ui: &mut Ui, theme: &Theme) -> Response {
        form_description(ui, theme, self.props)
    }
}

pub fn form_description(ui: &mut Ui, theme: &Theme, props: FormDescriptionProps) -> Response {
    let text = props.text.color(theme.palette.muted_foreground);
    ui.add(egui::Label::new(text).wrap())
}

#[derive(Clone, Debug)]
pub struct FormMessageProps {
    pub text: Option<WidgetText>,
}

impl FormMessageProps {
    pub fn new(text: impl Into<WidgetText>) -> Self {
        Self {
            text: Some(text.into()),
        }
    }

    pub fn empty() -> Self {
        Self { text: None }
    }
}

pub struct FormMessage {
    props: FormMessageProps,
}

impl FormMessage {
    pub fn new(text: impl Into<WidgetText>) -> Self {
        Self {
            props: FormMessageProps::new(text),
        }
    }

    pub fn from_error(error: Option<&str>) -> Self {
        let text = error.map(|msg| WidgetText::from(msg.to_string()));
        Self {
            props: FormMessageProps { text },
        }
    }

    pub fn show(self, ui: &mut Ui, theme: &Theme) -> Option<Response> {
        form_message(ui, theme, self.props)
    }
}

pub fn form_message(ui: &mut Ui, theme: &Theme, props: FormMessageProps) -> Option<Response> {
    let text = props.text?;
    let text = text.color(theme.palette.destructive);
    Some(ui.add(egui::Label::new(text).wrap()))
}
