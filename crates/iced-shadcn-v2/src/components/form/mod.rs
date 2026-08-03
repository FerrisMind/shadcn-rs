//! Form composition for `iced-shadcn-v2`.
//!
//! The web form component combines field structure, labels, descriptions,
//! validation messages, and a submit button. Iced has no DOM form element, so
//! [`Form`] is the visual/layout root and the application owns submission
//! messages through [`FormButton`] (an alias of [`Button`]). Validation and
//! field lifecycle state live in `shadcn-common::FormState`.
//!
//! **Style packs:** shadcn-svelte ships the same `form.json` for every pack.
//! Choosing Rhea (or Nova, …) on the shared [`Theme`] styles Form by styling
//! its parts — [`FormLabel`] → Label recipe, Input / select / … → their
//! recipes, [`FormButton`] → Button recipe — all via `theme.style_id()`.
//!
//! ```rust,no_run
//! use iced::Element;
//! use iced_shadcn_v2::{
//!     Form, FormButton, FormControlExt, FormControlProps, FormDescription, FormField,
//!     FormFieldErrors, FormLabel, Input, Theme,
//! };
//! use shadcn_common::FormState;
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     UsernameChanged(String),
//!     Submit,
//! }
//!
//! fn view<'a>(
//!     theme: &'a Theme,
//!     form_state: &'a FormState,
//!     username: &'a str,
//! ) -> Element<'a, Message> {
//!     let field = form_state
//!         .field_state("username")
//!         .expect("the example registers username before rendering");
//!     let control_props = FormControlProps::from_field(field);
//!
//!     Form::new(theme)
//!         .push(
//!             FormField::new(theme)
//!                 .form_state("username", form_state)
//!                 .push(FormLabel::from_field("Username", field, theme))
//!                 .push(
//!                     Input::new(theme)
//!                         .value(username)
//!                         .placeholder("shadcn")
//!                         .form_control(&control_props)
//!                         .on_input(Message::UsernameChanged),
//!                 )
//!                 .push(FormDescription::text(
//!                     "This is your public display name.",
//!                     theme,
//!                 ))
//!                 .push(FormFieldErrors::from_field(field, theme)),
//!         )
//!         .push(FormButton::text("Submit", theme).on_press(Message::Submit))
//!         .into()
//! }
//! ```

mod render;
mod types;

#[cfg(test)]
mod tests;

pub use types::FormControlProps;

use std::borrow::Cow;
use std::fmt;

use shadcn_common::{FormFieldState, FormState};

use crate::components::button::Button;
use crate::components::field::{
    Field, FieldContent, FieldErrorItem, FieldGroup, FieldLabel, FieldLegendVariant,
    FieldOrientation, FieldSet, FieldTitle,
};
use crate::components::input::Input;
use crate::components::input_otp::InputOtp;
use crate::components::label::LabelContext;
use crate::components::native_select::NativeSelect;
use crate::components::radio_group::RadioGroup;
use crate::components::select::Select;
use crate::components::switch::Switch;
use crate::components::textarea::Textarea;
use crate::iced_compat::widget;
use crate::iced_compat::widget::text::IntoFragment;
use crate::iced_compat::{Color, Element, Length};
use crate::theme::Theme;

/// A vertical form root with the active style pack's form spacing.
///
/// `Form` only controls layout. Submit behavior is explicit in iced: attach a
/// message to [`FormButton`] and call [`FormState::validate`] in the update
/// path.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct Form<'a, Message> {
    theme: &'a Theme,
    children: Vec<Element<'a, Message>>,
    width: Length,
    spacing: f32,
}

impl<Message> fmt::Debug for Form<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Form")
            .field("theme", &self.theme)
            .field("children", &self.children.len())
            .field("width", &self.width)
            .field("spacing", &self.spacing)
            .finish()
    }
}

impl<'a, Message> Form<'a, Message> {
    /// Creates an empty form using the active form recipe.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            children: Vec::new(),
            width: Length::Fill,
            spacing: theme.style.form().form_gap_px,
        }
    }

    /// Creates a form and appends all supplied children.
    pub fn with_children<I, E>(theme: &'a Theme, children: I) -> Self
    where
        I: IntoIterator<Item = E>,
        E: Into<Element<'a, Message>>,
    {
        let mut form = Self::new(theme);
        form.children.extend(children.into_iter().map(Into::into));
        form
    }

    /// Appends one child to the form.
    #[must_use = "builder methods return the modified Form"]
    pub fn push(mut self, child: impl Into<Element<'a, Message>>) -> Self {
        self.children.push(child.into());
        self
    }

    /// Appends a collection of children to the form.
    #[must_use = "builder methods return the modified Form"]
    pub fn extend<I>(mut self, children: I) -> Self
    where
        I: IntoIterator<Item = Element<'a, Message>>,
    {
        self.children.extend(children);
        self
    }

    /// Sets the form width.
    #[must_use = "builder methods return the modified Form"]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the gap between top-level fields in pixels.
    #[must_use = "builder methods return the modified Form"]
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing.max(0.0);
        self
    }

    /// Builds the form as an iced element.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        widget::column(self.children)
            .spacing(self.spacing)
            .width(self.width)
            .into()
    }
}

impl<'a, Message> From<Form<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(form: Form<'a, Message>) -> Self {
        form.into_element()
    }
}

/// A form field that can derive invalid and disabled state from
/// [`shadcn_common::FormState`].
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct FormField<'a, Message> {
    inner: Field<'a, Message>,
    name: Option<String>,
    form_state: Option<&'a FormState>,
    field_state: Option<&'a FormFieldState>,
    invalid: Option<bool>,
    disabled: Option<bool>,
}

impl<Message> fmt::Debug for FormField<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("FormField")
            .field("name", &self.name)
            .field("form_state", &self.form_state.is_some())
            .field("field_state", &self.field_state.is_some())
            .field("invalid", &self.invalid)
            .field("disabled", &self.disabled)
            .finish()
    }
}

impl<'a, Message> FormField<'a, Message> {
    /// Creates an empty form field.
    pub fn new(theme: &'a Theme) -> Self {
        let field_gap = theme.style.form().field_gap_px;
        Self {
            inner: Field::new(theme).spacing(field_gap),
            name: None,
            form_state: None,
            field_state: None,
            invalid: None,
            disabled: None,
        }
    }

    /// Creates a named field that can later resolve state from a form.
    pub fn named(name: impl Into<String>, theme: &'a Theme) -> Self {
        Self::new(theme).name(name)
    }

    /// Creates a field bound to a shared form state.
    pub fn from_state(
        name: impl Into<String>,
        form_state: &'a FormState,
        theme: &'a Theme,
    ) -> Self {
        let name = name.into();
        Self::named(name.clone(), theme).form_state(name, form_state)
    }

    /// Appends one child.
    #[must_use = "builder methods return the modified FormField"]
    pub fn push(mut self, child: impl Into<Element<'a, Message>>) -> Self {
        self.inner = self.inner.push(child);
        self
    }

    /// Appends a collection of children.
    #[must_use = "builder methods return the modified FormField"]
    pub fn extend<I>(mut self, children: I) -> Self
    where
        I: IntoIterator<Item = Element<'a, Message>>,
    {
        self.inner = self.inner.extend(children);
        self
    }

    /// Sets the shared state source and resolves it by this field's name.
    #[must_use = "builder methods return the modified FormField"]
    pub fn form_state(mut self, name: impl Into<String>, form_state: &'a FormState) -> Self {
        self.name = Some(name.into());
        self.form_state = Some(form_state);
        self
    }

    /// Sets the field name used when resolving state from [`FormState`].
    #[must_use = "builder methods return the modified FormField"]
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Binds a field state directly, avoiding a name lookup during rendering.
    #[must_use = "builder methods return the modified FormField"]
    pub fn field_state(mut self, field_state: &'a FormFieldState) -> Self {
        self.field_state = Some(field_state);
        self
    }

    /// Overrides the derived invalid state.
    #[must_use = "builder methods return the modified FormField"]
    pub fn invalid(mut self, invalid: bool) -> Self {
        self.invalid = Some(invalid);
        self
    }

    /// Overrides the derived disabled state.
    #[must_use = "builder methods return the modified FormField"]
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = Some(disabled);
        self
    }

    /// Sets the field orientation.
    #[must_use = "builder methods return the modified FormField"]
    pub fn orientation(mut self, orientation: FieldOrientation) -> Self {
        self.inner = self.inner.orientation(orientation);
        self
    }

    /// Sets the gap between direct field children in pixels.
    #[must_use = "builder methods return the modified FormField"]
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.inner = self.inner.spacing(spacing);
        self
    }

    /// Sets the field width.
    #[must_use = "builder methods return the modified FormField"]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.inner = self.inner.width(width);
        self
    }

    /// Sets the responsive orientation breakpoint.
    #[must_use = "builder methods return the modified FormField"]
    pub fn responsive_breakpoint(mut self, breakpoint: f32) -> Self {
        self.inner = self.inner.responsive_breakpoint(breakpoint);
        self
    }

    /// Builds the field after applying state-derived flags.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        let resolved_state = self.field_state.or_else(|| {
            self.name
                .as_deref()
                .and_then(|name| self.form_state.and_then(|form| form.field_state(name)))
        });
        let invalid = self
            .invalid
            .or_else(|| resolved_state.map(FormFieldState::is_invalid))
            .unwrap_or(false);
        let disabled = self
            .disabled
            .or_else(|| resolved_state.map(FormFieldState::is_disabled))
            .unwrap_or(false);

        self.inner
            .invalid(invalid)
            .disabled(disabled)
            .into_element()
    }
}

impl<'a, Message> From<FormField<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(field: FormField<'a, Message>) -> Self {
        field.into_element()
    }
}

/// Validation messages sourced from a shared form field.
///
/// Unlike [`FieldError`], this renderer uses the form recipe's `font-medium`
/// typography to match shadcn-svelte's `Form.FieldErrors`.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct FormFieldErrors<'a, Message> {
    theme: &'a Theme,
    content: Option<Element<'a, Message>>,
    errors: Vec<FieldErrorItem>,
    width: Length,
}

impl<Message> fmt::Debug for FormFieldErrors<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("FormFieldErrors")
            .field("theme", &self.theme)
            .field("has_content", &self.content.is_some())
            .field("errors", &self.errors)
            .field("width", &self.width)
            .finish()
    }
}

impl<'a, Message> FormFieldErrors<'a, Message> {
    /// Creates an empty validation-message renderer.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            content: None,
            errors: Vec::new(),
            width: Length::Fill,
        }
    }

    /// Creates validation messages from a shared form state.
    pub fn from_state(name: &str, form_state: &FormState, theme: &'a Theme) -> Self {
        form_state
            .field_state(name)
            .map_or_else(|| Self::new(theme), |field| Self::from_field(field, theme))
    }

    /// Creates validation messages from an already-resolved field.
    pub fn from_field(field: &FormFieldState, theme: &'a Theme) -> Self {
        Self::new(theme).errors(field.errors().iter().map(FieldErrorItem::new))
    }

    /// Creates custom error content.
    pub fn with_content(content: impl Into<Element<'a, Message>>, theme: &'a Theme) -> Self {
        Self {
            theme,
            content: Some(content.into()),
            errors: Vec::new(),
            width: Length::Fill,
        }
    }

    /// Creates one text error message.
    pub fn text(content: impl Into<String>, theme: &'a Theme) -> Self {
        Self::new(theme).errors([FieldErrorItem::new(content)])
    }

    /// Replaces the error list.
    #[must_use = "builder methods return the modified FormFieldErrors"]
    pub fn errors<I, E>(mut self, errors: I) -> Self
    where
        I: IntoIterator<Item = E>,
        E: Into<FieldErrorItem>,
    {
        self.errors = errors.into_iter().map(Into::into).collect();
        self
    }

    /// Appends one error item.
    #[must_use = "builder methods return the modified FormFieldErrors"]
    pub fn push_error(mut self, error: impl Into<FieldErrorItem>) -> Self {
        self.errors.push(error.into());
        self
    }

    /// Sets the error width.
    #[must_use = "builder methods return the modified FormFieldErrors"]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Builds the validation messages as an iced element.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        if let Some(content) = self.content {
            return content;
        }
        render::build_errors(self.errors, self.theme, self.width)
    }
}

impl<'a, Message> From<FormFieldErrors<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(errors: FormFieldErrors<'a, Message>) -> Self {
        errors.into_element()
    }
}

/// Control metadata adapter for arbitrary iced content.
///
/// For controls that do not implement [`FormControlExt`], construct this
/// wrapper and keep the properties available to an application-owned widget.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct FormControl<'a, Message> {
    child: Element<'a, Message>,
    props: FormControlProps,
    width: Length,
}

impl<Message> fmt::Debug for FormControl<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("FormControl")
            .field("props", &self.props)
            .field("width", &self.width)
            .finish_non_exhaustive()
    }
}

impl<'a, Message> FormControl<'a, Message> {
    /// Wraps arbitrary content with form-control metadata.
    pub fn new(child: impl Into<Element<'a, Message>>) -> Self {
        Self {
            child: child.into(),
            props: FormControlProps::new(),
            width: Length::Fill,
        }
    }

    /// Wraps content and associates it with a field state.
    pub fn from_field(child: impl Into<Element<'a, Message>>, field: &FormFieldState) -> Self {
        Self {
            child: child.into(),
            props: FormControlProps::from_field(field),
            width: Length::Fill,
        }
    }

    /// Replaces the metadata.
    #[must_use = "builder methods return the modified FormControl"]
    pub fn props(mut self, props: FormControlProps) -> Self {
        self.props = props;
        self
    }

    /// Returns the configured metadata.
    pub const fn control_props(&self) -> &FormControlProps {
        &self.props
    }

    /// Sets the wrapper width.
    #[must_use = "builder methods return the modified FormControl"]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Returns the wrapped content and metadata.
    pub fn into_parts(self) -> (Element<'a, Message>, FormControlProps) {
        (self.child, self.props)
    }

    /// Builds the wrapper as an iced element.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        let Self {
            child,
            props: _props,
            width,
        } = self;
        widget::container(child).width(width).into()
    }
}

impl<'a, Message> From<FormControl<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(control: FormControl<'a, Message>) -> Self {
        control.into_element()
    }
}

/// Applies [`FormControlProps`] to controls owned by this crate.
pub trait FormControlExt: Sized {
    /// Applies ids and state flags to the control.
    fn form_control(self, props: &FormControlProps) -> Self;
}

impl<'a, Message> FormControlExt for Input<'a, Message> {
    fn form_control(self, props: &FormControlProps) -> Self {
        let input = if let Some(id) = props.control_id() {
            self.id(id.to_owned())
        } else {
            self
        };
        input
            .invalid(props.is_invalid())
            .disabled(props.is_disabled())
    }
}

impl<'a, Message> FormControlExt for Textarea<'a, Message> {
    fn form_control(self, props: &FormControlProps) -> Self {
        let textarea = if let Some(id) = props.control_id() {
            self.id(id.to_owned())
        } else {
            self
        };
        textarea
            .invalid(props.is_invalid())
            .disabled(props.is_disabled())
    }
}

impl<'a, Message> FormControlExt for InputOtp<'a, Message> {
    fn form_control(self, props: &FormControlProps) -> Self {
        let input = if let Some(id) = props.control_id() {
            self.id(id.to_owned())
        } else {
            self
        };
        input
            .invalid(props.is_invalid())
            .disabled(props.is_disabled())
    }
}

impl<'a, T, Message> FormControlExt for NativeSelect<'a, T, Message>
where
    T: Clone + PartialEq,
{
    fn form_control(self, props: &FormControlProps) -> Self {
        self.invalid(props.is_invalid())
            .disabled(props.is_disabled())
    }
}

impl<'a, T, Message> FormControlExt for Select<'a, T, Message>
where
    T: Clone + PartialEq,
{
    fn form_control(self, props: &FormControlProps) -> Self {
        self.invalid(props.is_invalid())
            .disabled(props.is_disabled())
    }
}

impl<'a, Message> FormControlExt for Switch<'a, Message> {
    fn form_control(self, props: &FormControlProps) -> Self {
        self.invalid(props.is_invalid())
            .disabled(props.is_disabled())
    }
}

impl<'a, Message> FormControlExt for RadioGroup<'a, Message> {
    fn form_control(self, props: &FormControlProps) -> Self {
        self.invalid(props.is_invalid())
            .disabled(props.is_disabled())
    }
}

/// Button used as a form submit action.
///
/// This is an alias instead of a second button implementation, so every
/// `Button` capability (variant, size, loading, disabled, icon content, and
/// style overrides) is available. Iced has no HTML `type="submit"`; attach
/// the submit message with [`Button::on_press`].
pub type FormButton<'a, Message> = Button<'a, Message>;

/// Form description text using [`FormRecipe::description`](shadcn_common::FormRecipe).
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct FormDescription<'a, Message> {
    content: Option<crate::iced_compat::widget::text::Fragment<'a>>,
    element: Option<Element<'a, Message>>,
    theme: &'a Theme,
    width: Length,
    color: Option<Color>,
}

impl<Message> fmt::Debug for FormDescription<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("FormDescription")
            .field("theme", &self.theme)
            .field("width", &self.width)
            .field("color", &self.color)
            .field("has_element", &self.element.is_some())
            .finish_non_exhaustive()
    }
}

impl<'a, Message> FormDescription<'a, Message> {
    /// Creates a description from arbitrary iced content.
    pub fn new(content: impl Into<Element<'a, Message>>, theme: &'a Theme) -> Self {
        Self {
            content: None,
            element: Some(content.into()),
            theme,
            width: Length::Fill,
            color: None,
        }
    }

    /// Creates a text description.
    pub fn text(content: impl IntoFragment<'a>, theme: &'a Theme) -> Self {
        Self {
            content: Some(content.into_fragment()),
            element: None,
            theme,
            width: Length::Fill,
            color: None,
        }
    }

    /// Sets the description width.
    #[must_use = "builder methods return the modified FormDescription"]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Overrides the description color.
    #[must_use = "builder methods return the modified FormDescription"]
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Builds the description as an iced element.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        if let Some(element) = self.element {
            return widget::container(element)
                .width(self.width)
                .style(move |_| {
                    let color = self.color.unwrap_or(self.theme.palette.muted_foreground);
                    widget::container::Style {
                        text_color: Some(color),
                        ..widget::container::Style::default()
                    }
                })
                .into();
        }

        render::build_description(
            self.content.unwrap_or_else(|| "".into()),
            self.theme,
            self.width,
            self.color,
        )
    }
}

impl<'a, Message> From<FormDescription<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(description: FormDescription<'a, Message>) -> Self {
        description.into_element()
    }
}

/// Form label that paints destructive text when the field is invalid.
///
/// Mirrors shadcn-svelte's `data-[fs-error]:text-destructive` on `Form.Label`.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct FormLabel<'a, Message> {
    inner: FieldLabel<'a, Message>,
    theme: &'a Theme,
    invalid: bool,
    color_override: Option<Color>,
}

impl<Message> fmt::Debug for FormLabel<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("FormLabel")
            .field("theme", &self.theme)
            .field("invalid", &self.invalid)
            .field("color_override", &self.color_override)
            .finish_non_exhaustive()
    }
}

impl<'a, Message> FormLabel<'a, Message> {
    /// Creates a form label from arbitrary iced content.
    pub fn new(content: impl Into<Element<'a, Message>>, theme: &'a Theme) -> Self {
        Self {
            inner: FieldLabel::new(content, theme),
            theme,
            invalid: false,
            color_override: None,
        }
    }

    /// Creates a text form label.
    pub fn text(content: impl IntoFragment<'a>, theme: &'a Theme) -> Self {
        Self {
            inner: FieldLabel::text(content, theme),
            theme,
            invalid: false,
            color_override: None,
        }
    }

    /// Creates a label wired to a shared field's invalid state and control id.
    pub fn from_field(
        content: impl IntoFragment<'a>,
        field: &FormFieldState,
        theme: &'a Theme,
    ) -> Self {
        Self::text(content, theme)
            .invalid(field.is_invalid())
            .for_id(field.ids().control().to_owned())
            .disabled(field.is_disabled())
    }

    /// Sets the label context used by the active style pack's [`LabelRecipe`].
    ///
    /// Form itself has no style-pack variants in shadcn-svelte; label typography
    /// comes from the composed [`Label`](crate::Label) / Field label recipes.
    #[must_use = "builder methods return the modified FormLabel"]
    pub fn context(mut self, context: LabelContext) -> Self {
        self.inner = self.inner.context(context);
        self
    }

    /// Applies the disabled label treatment.
    #[must_use = "builder methods return the modified FormLabel"]
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.inner = self.inner.disabled(disabled);
        self
    }

    /// Associates the label with a control identifier.
    #[must_use = "builder methods return the modified FormLabel"]
    pub fn for_id(mut self, id: impl Into<Cow<'a, str>>) -> Self {
        self.inner = self.inner.for_id(id);
        self
    }

    /// Sets the label width.
    #[must_use = "builder methods return the modified FormLabel"]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.inner = self.inner.width(width);
        self
    }

    /// Overrides the label color (wins over the invalid treatment).
    #[must_use = "builder methods return the modified FormLabel"]
    pub fn color(mut self, color: Color) -> Self {
        self.color_override = Some(color);
        self
    }

    /// Applies the `data-[fs-error]:text-destructive` treatment.
    #[must_use = "builder methods return the modified FormLabel"]
    pub fn invalid(mut self, invalid: bool) -> Self {
        self.invalid = invalid;
        self
    }

    /// Sets the message emitted when the label is pressed.
    #[must_use = "builder methods return the modified FormLabel"]
    pub fn on_press(mut self, message: Message) -> Self {
        self.inner = self.inner.on_press(message);
        self
    }

    /// Sets or clears the message emitted when the label is pressed.
    #[must_use = "builder methods return the modified FormLabel"]
    pub fn on_press_maybe(mut self, message: Option<Message>) -> Self {
        self.inner = self.inner.on_press_maybe(message);
        self
    }

    /// Builds the label as an iced element.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        let color = self
            .color_override
            .or_else(|| self.invalid.then_some(self.theme.palette.destructive));
        let label = if let Some(color) = color {
            self.inner.color(color)
        } else {
            self.inner
        };
        label.into_element()
    }
}

impl<'a, Message> From<FormLabel<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(label: FormLabel<'a, Message>) -> Self {
        label.into_element()
    }
}

/// Form legend using [`FormRecipe::legend`](shadcn_common::FormRecipe).
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct FormLegend<'a> {
    content: crate::iced_compat::widget::text::Fragment<'a>,
    theme: &'a Theme,
    width: Length,
    color: Option<Color>,
    invalid: bool,
}

impl fmt::Debug for FormLegend<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("FormLegend")
            .field("theme", &self.theme)
            .field("width", &self.width)
            .field("color", &self.color)
            .field("invalid", &self.invalid)
            .finish_non_exhaustive()
    }
}

impl<'a> FormLegend<'a> {
    /// Creates a text legend.
    pub fn text(content: impl IntoFragment<'a>, theme: &'a Theme) -> Self {
        Self {
            content: content.into_fragment(),
            theme,
            width: Length::Fill,
            color: None,
            invalid: false,
        }
    }

    /// Sets the legend width.
    #[must_use = "builder methods return the modified FormLegend"]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Overrides the legend color.
    #[must_use = "builder methods return the modified FormLegend"]
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Applies the `data-[fs-error]:text-destructive` treatment.
    #[must_use = "builder methods return the modified FormLegend"]
    pub fn invalid(mut self, invalid: bool) -> Self {
        self.invalid = invalid;
        self
    }

    /// Builds the legend as an iced element.
    pub fn into_element<Message>(self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        let color = self
            .color
            .or_else(|| self.invalid.then_some(self.theme.palette.destructive));
        render::build_legend(self.content, self.theme, self.width, color)
    }
}

impl<'a, Message> From<FormLegend<'a>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(legend: FormLegend<'a>) -> Self {
        legend.into_element()
    }
}

/// Form legend variant (re-export for API parity with Field).
pub type FormLegendVariant = FieldLegendVariant;

/// Group of form fields.
pub type FormFieldGroup<'a, Message> = FieldGroup<'a, Message>;

/// Group of controls inside a form field.
pub type FormFieldContent<'a, Message> = FieldContent<'a, Message>;

/// Fieldset-like form section with the active form recipe's fieldset gap.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct FormFieldset<'a, Message> {
    inner: FieldSet<'a, Message>,
}

impl<Message> fmt::Debug for FormFieldset<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("FormFieldset")
            .field("inner", &self.inner)
            .finish()
    }
}

impl<'a, Message> FormFieldset<'a, Message> {
    /// Creates an empty fieldset using the active form recipe spacing.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            inner: FieldSet::new().spacing(theme.style.form().fieldset_gap_px),
        }
    }

    /// Creates a fieldset and appends all supplied children.
    pub fn with_children<I, E>(theme: &'a Theme, children: I) -> Self
    where
        I: IntoIterator<Item = E>,
        E: Into<Element<'a, Message>>,
    {
        let mut set = Self::new(theme);
        set.inner = set.inner.extend(children.into_iter().map(Into::into));
        set
    }

    /// Appends one child.
    #[must_use = "builder methods return the modified FormFieldset"]
    pub fn push(mut self, child: impl Into<Element<'a, Message>>) -> Self {
        self.inner = self.inner.push(child);
        self
    }

    /// Appends a collection of children.
    #[must_use = "builder methods return the modified FormFieldset"]
    pub fn extend<I>(mut self, children: I) -> Self
    where
        I: IntoIterator<Item = Element<'a, Message>>,
    {
        self.inner = self.inner.extend(children);
        self
    }

    /// Sets the gap between fieldset children in pixels.
    #[must_use = "builder methods return the modified FormFieldset"]
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.inner = self.inner.spacing(spacing);
        self
    }

    /// Sets the fieldset width.
    #[must_use = "builder methods return the modified FormFieldset"]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.inner = self.inner.width(width);
        self
    }

    /// Builds the fieldset as an iced element.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        self.inner.into_element()
    }
}

impl<'a, Message> From<FormFieldset<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(set: FormFieldset<'a, Message>) -> Self {
        set.into_element()
    }
}

/// Alternate spelling for [`FormFieldset`].
pub type FormFieldSet<'a, Message> = FormFieldset<'a, Message>;

/// Compact form field title.
pub type FormFieldTitle<'a, Message> = FieldTitle<'a, Message>;

/// Alias for the state-aware [`FormField`] builder.
pub type FormElementField<'a, Message> = FormField<'a, Message>;
