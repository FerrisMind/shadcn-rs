//! Public state and configuration types for the rename component.

use std::fmt;
use std::rc::Rc;

use crate::iced_compat::widget::{self, text_editor};
use crate::iced_compat::{Color, Element, Length};

use super::super::button::{ButtonRadius, ButtonSize, ButtonVariant};
use super::super::input::{InputRadius, InputSize};
use super::super::textarea::{TextareaRadius, TextareaSize};
use shadcn_common::AccentColor;

/// The externally controlled display mode of [`super::Rename`].
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RenameMode {
    /// Show the accepted value as text.
    #[default]
    View,
    /// Show the configured input or textarea.
    Edit,
}

impl RenameMode {
    /// Returns `true` when the edit control should be visible.
    pub const fn is_edit(self) -> bool {
        matches!(self, Self::Edit)
    }
}

/// The editing control rendered by [`super::Rename`].
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RenameInputTag {
    /// Use the single-line [`crate::Input`] primitive.
    #[default]
    Input,
    /// Use the multiline [`crate::Textarea`] primitive.
    Textarea,
}

impl RenameInputTag {
    /// Returns `true` when the multiline editor is selected.
    pub const fn is_textarea(self) -> bool {
        matches!(self, Self::Textarea)
    }
}

/// What the rename control does when it loses focus.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RenameBlurBehavior {
    /// Leave edit mode and discard the uncommitted value.
    #[default]
    Exit,
    /// Keep edit mode active until Save or Cancel is requested.
    None,
}

/// Where to place the caret when editing starts without a more specific range.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RenameFallbackSelectionBehavior {
    /// Place the caret at the beginning.
    Start,
    /// Place the caret at the end.
    #[default]
    End,
    /// Select the complete value.
    All,
}

/// An application action emitted by [`super::Rename`] and its control buttons.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq)]
pub enum RenameAction {
    /// Enter edit mode and restore the accepted value into the editor.
    StartEdit,
    /// Replace the current single-line editing buffer.
    InputChanged(String),
    /// Apply an iced text-editor action to the multiline editing buffer.
    TextareaEdited(text_editor::Action),
    /// Try to commit the editing buffer.
    SaveRequested,
    /// Discard the editing buffer.
    CancelRequested,
    /// Cancel editing from the Escape key.
    EscapePressed,
    /// Report that the pointer or keyboard focus left the root control.
    BlurDetected,
}

/// A cloneable action callback shared by the root and external controls.
pub type RenameActionHandler<'a, Message> = Rc<dyn Fn(RenameAction) -> Message + 'a>;

/// Controlled state for a [`super::Rename`].
///
/// Keep this value in application state. The component never mutates it while
/// rendering; call [`super::rename_apply_action`] from the application's
/// update function and feed the resulting state back into the next view.
/// Keeping the fields private preserves the invariant that the text-editor
/// content and `editing_value` stay synchronized.
#[derive(Clone, Debug)]
pub struct RenameState {
    pub(super) mode: RenameMode,
    pub(super) value: String,
    pub(super) editing_value: String,
    pub(super) textarea_content: text_editor::Content,
    pub(super) invalid: bool,
}

impl RenameState {
    /// Creates view-mode state whose accepted and editing values are `value`.
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        let value = value.into();
        Self {
            mode: RenameMode::View,
            editing_value: value.clone(),
            textarea_content: text_editor::Content::with_text(&value),
            value,
            invalid: false,
        }
    }

    /// Returns the accepted value currently shown in view mode.
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Returns the uncommitted value currently shown by the editor.
    pub fn editing_value(&self) -> &str {
        &self.editing_value
    }

    /// Returns the current display mode.
    pub const fn mode(&self) -> RenameMode {
        self.mode
    }

    /// Returns whether the current editing value failed validation.
    pub const fn is_invalid(&self) -> bool {
        self.invalid
    }

    /// Returns the caller-owned text-editor content used by textarea mode.
    pub fn textarea_content(&self) -> &text_editor::Content {
        &self.textarea_content
    }

    /// Replaces the accepted value and keeps the editing buffer synchronized.
    ///
    /// If the state is already in edit mode, the current uncommitted buffer is
    /// preserved. Use [`super::rename_apply_action`] with `StartEdit` to reset
    /// it deliberately.
    pub fn set_value(&mut self, value: impl Into<String>) {
        self.value = value.into();
        if self.mode == RenameMode::View {
            self.editing_value = self.value.clone();
            self.sync_textarea_content();
        }
    }

    /// Sets the externally controlled mode and restores editor invariants.
    pub fn set_mode(&mut self, mode: RenameMode) {
        match mode {
            RenameMode::View => self.cancel_editing(),
            RenameMode::Edit => {
                self.mode = RenameMode::Edit;
                self.editing_value = self.value.clone();
                self.invalid = false;
                self.sync_textarea_content();
            }
        }
    }

    pub(super) fn sync_textarea_content(&mut self) {
        self.textarea_content = text_editor::Content::with_text(&self.editing_value);
    }

    pub(super) fn cancel_editing(&mut self) {
        self.mode = RenameMode::View;
        self.editing_value = self.value.clone();
        self.invalid = false;
        self.sync_textarea_content();
    }
}

impl Default for RenameState {
    fn default() -> Self {
        Self::new("")
    }
}

/// Which selection operation should follow a successful focus request.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum RenameSelectionRequest {
    /// Move the caret to the beginning.
    Start,
    /// Move the caret to the end.
    #[default]
    End,
    /// Select all text in the single-line input.
    All,
}

/// Result flags returned by [`super::rename_apply_action`].
///
/// The application can use these flags to schedule focus and selection
/// operations with `iced::widget::operation` after it updates its state.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct RenameUpdate {
    entered_edit_mode: bool,
    committed: bool,
    cancelled: bool,
    validation_failed: bool,
    request_focus: bool,
    selection: Option<RenameSelectionRequest>,
}

impl RenameUpdate {
    pub(super) fn new_entered(selection: Option<RenameSelectionRequest>) -> Self {
        Self {
            entered_edit_mode: true,
            request_focus: true,
            selection,
            ..Self::default()
        }
    }

    pub(super) fn new_committed() -> Self {
        Self {
            committed: true,
            ..Self::default()
        }
    }

    pub(super) fn new_cancelled() -> Self {
        Self {
            cancelled: true,
            ..Self::default()
        }
    }

    pub(super) fn new_validation_failed() -> Self {
        Self {
            validation_failed: true,
            ..Self::default()
        }
    }

    /// Returns `true` when the action entered edit mode.
    pub const fn entered_edit_mode(self) -> bool {
        self.entered_edit_mode
    }

    /// Returns `true` when the editing value was accepted.
    pub const fn committed(self) -> bool {
        self.committed
    }

    /// Returns `true` when editing was cancelled.
    pub const fn cancelled(self) -> bool {
        self.cancelled
    }

    /// Returns `true` when Save was rejected by validation.
    pub const fn validation_failed(self) -> bool {
        self.validation_failed
    }

    /// Returns `true` when the application should focus the configured input.
    pub const fn request_focus(self) -> bool {
        self.request_focus
    }

    /// Returns the single-line selection operation requested on edit start.
    pub const fn selection(self) -> Option<RenameSelectionRequest> {
        self.selection
    }
}

/// Root configuration shared by the builder and the low-level free function.
///
/// `Rename` does not define its own style recipe. `input_*` and
/// `textarea_*` options are forwarded to the corresponding v2 primitives, so
/// changing `Theme::with_style` changes every composed control consistently.
#[derive(Clone, Debug)]
pub struct RenameRootProps {
    pub(super) id: widget::Id,
    pub(super) input_id: widget::Id,
    pub(super) input_tag: RenameInputTag,
    pub(super) blur_behavior: RenameBlurBehavior,
    pub(super) blur_behavior_explicit: bool,
    pub(super) fallback_selection_behavior: RenameFallbackSelectionBehavior,
    pub(super) input_size: InputSize,
    pub(super) input_radius: Option<InputRadius>,
    pub(super) textarea_size: TextareaSize,
    pub(super) textarea_radius: Option<TextareaRadius>,
    pub(super) color: Option<AccentColor>,
    pub(super) input_color: Option<AccentColor>,
    pub(super) textarea_color: Option<AccentColor>,
    pub(super) text_size: Option<f32>,
    pub(super) text_line_height: Option<f32>,
    pub(super) text_color: Option<Color>,
    pub(super) width: Length,
    pub(super) input_width: Option<Length>,
    pub(super) textarea_width: Option<Length>,
    pub(super) disabled: bool,
    pub(super) click_to_edit: bool,
    pub(super) click_to_edit_explicit: bool,
}

impl RenameRootProps {
    /// Creates default single-line rename configuration.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the root container id used by iced widget operations.
    #[must_use]
    pub fn id(mut self, id: impl Into<widget::Id>) -> Self {
        self.id = id.into();
        self
    }

    /// Sets the id used by iced focus and selection operations.
    #[must_use]
    pub fn input_id(mut self, input_id: impl Into<widget::Id>) -> Self {
        self.input_id = input_id.into();
        self
    }

    /// Returns a clone of the configured editor id for `iced::widget::operation`.
    #[must_use]
    pub fn input_id_value(&self) -> widget::Id {
        self.input_id.clone()
    }

    /// Sets the editing primitive.
    #[must_use]
    pub fn input_tag(mut self, input_tag: RenameInputTag) -> Self {
        self.input_tag = input_tag;
        self
    }

    /// Sets the blur policy.
    #[must_use]
    pub fn blur_behavior(mut self, blur_behavior: RenameBlurBehavior) -> Self {
        self.blur_behavior = blur_behavior;
        self.blur_behavior_explicit = true;
        self
    }

    /// Sets the fallback caret/selection policy.
    #[must_use]
    pub fn fallback_selection_behavior(
        mut self,
        fallback_selection_behavior: RenameFallbackSelectionBehavior,
    ) -> Self {
        self.fallback_selection_behavior = fallback_selection_behavior;
        self
    }

    /// Sets the v2 input size used in single-line mode.
    #[must_use]
    pub fn input_size(mut self, size: InputSize) -> Self {
        self.input_size = size;
        self
    }

    /// Sets the v2 input radius used in single-line mode.
    #[must_use]
    pub fn input_radius(mut self, radius: InputRadius) -> Self {
        self.input_radius = Some(radius);
        self
    }

    /// Sets the v2 textarea size used in multiline mode.
    #[must_use]
    pub fn textarea_size(mut self, size: TextareaSize) -> Self {
        self.textarea_size = size;
        self
    }

    /// Sets the v2 textarea radius used in multiline mode.
    #[must_use]
    pub fn textarea_radius(mut self, radius: TextareaRadius) -> Self {
        self.textarea_radius = Some(radius);
        self
    }

    /// Applies one semantic accent to both editor variants.
    #[must_use]
    pub fn color(mut self, color: AccentColor) -> Self {
        self.color = Some(color);
        self
    }

    /// Applies an accent color only to the single-line input.
    #[must_use]
    pub fn input_color(mut self, color: AccentColor) -> Self {
        self.input_color = Some(color);
        self
    }

    /// Applies an accent color only to the multiline textarea.
    #[must_use]
    pub fn textarea_color(mut self, color: AccentColor) -> Self {
        self.textarea_color = Some(color);
        self
    }

    /// Sets the view text and editor text size in pixels.
    #[must_use]
    pub fn text_size(mut self, size: f32) -> Self {
        self.text_size = Some(finite_positive(size));
        self
    }

    /// Sets an explicit view text line height in pixels.
    #[must_use]
    pub fn text_line_height(mut self, line_height: f32) -> Self {
        self.text_line_height = Some(finite_positive(line_height));
        self
    }

    /// Overrides the view text color.
    #[must_use]
    pub fn text_color(mut self, color: Color) -> Self {
        self.text_color = Some(color);
        self
    }

    /// Sets the root/editor width (`Length::Fill` by default).
    #[must_use]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets a width override for single-line input mode.
    #[must_use]
    pub fn input_width(mut self, width: impl Into<Length>) -> Self {
        self.input_width = Some(width.into());
        self
    }

    /// Sets a width override for multiline textarea mode.
    #[must_use]
    pub fn textarea_width(mut self, width: impl Into<Length>) -> Self {
        self.textarea_width = Some(width.into());
        self
    }

    /// Disables editing and external control actions.
    #[must_use]
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Enables or disables clicking the view text to enter edit mode.
    #[must_use]
    pub fn click_to_edit(mut self, click_to_edit: bool) -> Self {
        self.click_to_edit = click_to_edit;
        self.click_to_edit_explicit = true;
        self
    }
}

impl Default for RenameRootProps {
    fn default() -> Self {
        Self {
            id: widget::Id::unique(),
            input_id: widget::Id::unique(),
            input_tag: RenameInputTag::Input,
            blur_behavior: RenameBlurBehavior::Exit,
            fallback_selection_behavior: RenameFallbackSelectionBehavior::End,
            input_size: InputSize::Default,
            input_radius: None,
            textarea_size: TextareaSize::Default,
            textarea_radius: None,
            color: None,
            input_color: None,
            textarea_color: None,
            blur_behavior_explicit: false,
            text_size: None,
            text_line_height: None,
            text_color: None,
            width: Length::Fill,
            input_width: None,
            textarea_width: None,
            disabled: false,
            click_to_edit: true,
            click_to_edit_explicit: false,
        }
    }
}

/// Configuration for the external Edit, Save, and Cancel controls.
pub struct RenameButtonProps<'a, Message> {
    pub(super) label: String,
    pub(super) content: Option<Element<'a, Message>>,
    pub(super) variant: Option<ButtonVariant>,
    pub(super) size: ButtonSize,
    pub(super) radius: Option<ButtonRadius>,
    pub(super) color: Option<AccentColor>,
    pub(super) width: Length,
    pub(super) height: Option<Length>,
    pub(super) disabled: bool,
}

impl<Message> fmt::Debug for RenameButtonProps<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RenameButtonProps")
            .field("label", &self.label)
            .field("content", &self.content.is_some())
            .field("variant", &self.variant)
            .field("size", &self.size)
            .field("radius", &self.radius)
            .field("color", &self.color)
            .field("width", &self.width)
            .field("height", &self.height)
            .field("disabled", &self.disabled)
            .finish()
    }
}

impl<'a, Message> RenameButtonProps<'a, Message> {
    /// Creates a button configuration with a custom label.
    #[must_use]
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            content: None,
            variant: None,
            size: ButtonSize::Sm,
            radius: None,
            color: None,
            width: Length::Shrink,
            height: None,
            disabled: false,
        }
    }

    /// Sets the label. An empty label makes the control use its component
    /// default (`Edit`, `Save`, or `Cancel`).
    #[must_use]
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }

    /// Replaces the default text with arbitrary button content.
    ///
    /// The control still owns the action and the composed [`crate::Button`]
    /// still owns its theme-derived styling. For a completely different
    /// trigger widget, use [`RenameContext::action_message`] directly in the
    /// provider composition closure.
    #[must_use]
    pub fn content(mut self, content: impl Into<Element<'a, Message>>) -> Self {
        self.content = Some(content.into());
        self
    }

    /// Sets the composed [`crate::Button`] variant.
    #[must_use]
    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = Some(variant);
        self
    }

    /// Sets the composed [`crate::Button`] size.
    #[must_use]
    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    /// Sets the composed button radius.
    #[must_use]
    pub fn radius(mut self, radius: ButtonRadius) -> Self {
        self.radius = Some(radius);
        self
    }

    /// Applies an accent to the composed button.
    #[must_use]
    pub fn color(mut self, color: AccentColor) -> Self {
        self.color = Some(color);
        self
    }

    /// Sets the composed button width.
    #[must_use]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the composed button height.
    #[must_use]
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = Some(height.into());
        self
    }

    /// Disables this control independently from the root.
    #[must_use]
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl<'a, Message> Default for RenameButtonProps<'a, Message> {
    fn default() -> Self {
        Self::new("")
    }
}

/// Configuration for [`super::rename_provider`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RenameProviderProps {
    disabled: bool,
    blur_behavior: RenameBlurBehavior,
}

impl Default for RenameProviderProps {
    fn default() -> Self {
        Self::new()
    }
}

impl RenameProviderProps {
    /// Creates an enabled provider whose default blur policy is `None`, like
    /// the Svelte provider context.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            disabled: false,
            blur_behavior: RenameBlurBehavior::None,
        }
    }

    /// Disables actions emitted by the provider's controls.
    #[must_use]
    pub const fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Sets the provider's default blur policy.
    #[must_use]
    pub const fn blur_behavior(mut self, blur_behavior: RenameBlurBehavior) -> Self {
        self.blur_behavior = blur_behavior;
        self
    }

    pub(super) const fn is_disabled(self) -> bool {
        self.disabled
    }

    pub(super) const fn blur_behavior_value(self) -> RenameBlurBehavior {
        self.blur_behavior
    }
}

/// Context passed to the low-level provider composition closure.
pub struct RenameContext<'a, Message> {
    pub(super) mode: RenameMode,
    pub(super) invalid: bool,
    pub(super) disabled: bool,
    pub(super) blur_behavior: RenameBlurBehavior,
    pub(super) on_action: Option<RenameActionHandler<'a, Message>>,
}

impl<Message> fmt::Debug for RenameContext<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RenameContext")
            .field("mode", &self.mode)
            .field("invalid", &self.invalid)
            .field("disabled", &self.disabled)
            .field("blur_behavior", &self.blur_behavior)
            .field("on_action", &self.on_action.is_some())
            .finish()
    }
}

impl<'a, Message> RenameContext<'a, Message> {
    /// Returns the current mode exposed to external controls.
    pub const fn mode(&self) -> RenameMode {
        self.mode
    }

    /// Returns whether the current value is invalid.
    pub const fn is_invalid(&self) -> bool {
        self.invalid
    }

    /// Returns whether the provider disables the external controls.
    pub const fn is_disabled(&self) -> bool {
        self.disabled
    }

    /// Returns the provider's default blur policy.
    pub const fn blur_behavior(&self) -> RenameBlurBehavior {
        self.blur_behavior
    }

    /// Applies the provider's blur policy to root props.
    #[must_use]
    pub fn root_props(&self, mut props: RenameRootProps) -> RenameRootProps {
        if !props.blur_behavior_explicit {
            props.blur_behavior = self.blur_behavior;
        }
        if !props.click_to_edit_explicit {
            props.click_to_edit = false;
        }
        props
    }

    /// Converts an action into the configured application message.
    #[must_use]
    pub fn action_message(&self, action: RenameAction) -> Option<Message> {
        self.on_action.as_ref().map(|on_action| on_action(action))
    }
}

fn finite_positive(value: f32) -> f32 {
    if value.is_finite() {
        value.max(1.0)
    } else {
        1.0
    }
}
