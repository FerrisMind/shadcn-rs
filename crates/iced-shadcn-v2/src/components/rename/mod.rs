//! Controlled inline rename editor with optional external controls.
//!
//! The component mirrors the two Svelte entry points from
//! `shadcn-svelte-extras`: a single [`Rename`] root and a composable provider
//! with [`rename_edit`], [`rename_save`], and [`rename_cancel`] controls.
//!
//! Rename is deliberately a controlled component. [`RenameState`] belongs to
//! the application; the view only renders it and emits [`RenameAction`]
//! messages. Call [`rename_apply_action`] from the update function and render
//! again with the resulting state.
//!
//! There is no rename-specific style recipe. Editor options are forwarded to
//! [`crate::Input`] or [`crate::Textarea`], and external controls are built
//! with [`crate::Button`]. Consequently the active [`Theme`] style pack,
//! including styles such as `Rhea`, flows through every composed primitive.
//!
//! ```rust,no_run
//! use iced::Element;
//! use iced_shadcn_v2::{Rename, RenameAction, RenameState, Theme};
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     Rename(RenameAction),
//! }
//!
//! fn view<'a>(state: &'a RenameState, theme: &'a Theme) -> Element<'a, Message> {
//!     Rename::new(state, theme)
//!         .on_action(Message::Rename)
//!         .into()
//! }
//! ```

mod render;
mod types;

#[cfg(test)]
mod tests;

use std::fmt;
use std::rc::Rc;

use crate::iced_compat::widget::{self, text_editor};
use crate::iced_compat::{Element, Length};
use crate::theme::Theme;

pub use types::{
    RenameAction, RenameActionHandler, RenameBlurBehavior, RenameButtonProps, RenameContext,
    RenameFallbackSelectionBehavior, RenameInputTag, RenameMode, RenameProviderProps,
    RenameRootProps, RenameSelectionRequest, RenameState, RenameUpdate,
};

/// Builder-first controlled inline rename editor.
///
/// The application owns `state` and applies messages emitted through
/// [`Self::on_action`]. The default root is a full-width single-line editor in
/// edit mode and a full-width text value in view mode; [`Self::props`] and the
/// forwarding methods expose the styling knobs of the composed primitives.
///
/// ```rust,no_run
/// use iced::Element;
/// use iced_shadcn_v2::{Rename, RenameAction, RenameState, Theme};
///
/// #[derive(Debug, Clone)]
/// enum Message {
///     Rename(RenameAction),
/// }
///
/// fn rename<'a>(state: &'a RenameState, theme: &'a Theme) -> Element<'a, Message> {
///     Rename::new(state, theme)
///         .text_size(20.0)
///         .on_action(Message::Rename)
///         .into()
/// }
/// ```
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct Rename<'a, Message> {
    state: &'a RenameState,
    theme: &'a Theme,
    props: RenameRootProps,
    on_action: Option<RenameActionHandler<'a, Message>>,
}

impl<Message> fmt::Debug for Rename<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Rename")
            .field("state", &self.state)
            .field("theme", &self.theme)
            .field("props", &self.props)
            .field("on_action", &self.on_action.is_some())
            .finish()
    }
}

impl<'a, Message> Rename<'a, Message> {
    /// Creates a rename root backed by caller-owned [`RenameState`].
    pub fn new(state: &'a RenameState, theme: &'a Theme) -> Self {
        Self {
            state,
            theme,
            props: RenameRootProps::default(),
            on_action: None,
        }
    }

    /// Replaces the complete root configuration.
    #[must_use = "builder methods return the modified rename"]
    pub fn props(mut self, props: RenameRootProps) -> Self {
        self.props = props;
        self
    }

    /// Alias for [`Self::props`] using the low-level API's terminology.
    #[must_use = "builder methods return the modified rename"]
    pub fn root_props(self, props: RenameRootProps) -> Self {
        self.props(props)
    }

    /// Sets the root container id used by iced widget operations.
    #[must_use = "builder methods return the modified rename"]
    pub fn id(mut self, id: impl Into<widget::Id>) -> Self {
        self.props = self.props.id(id);
        self
    }

    /// Sets the input id used by iced focus and selection operations.
    #[must_use = "builder methods return the modified rename"]
    pub fn input_id(mut self, id: impl Into<widget::Id>) -> Self {
        self.props = self.props.input_id(id);
        self
    }

    /// Selects the single-line input or multiline textarea editor.
    #[must_use = "builder methods return the modified rename"]
    pub fn input_tag(mut self, input_tag: RenameInputTag) -> Self {
        self.props = self.props.input_tag(input_tag);
        self
    }

    /// Sets the blur behavior used by the root.
    #[must_use = "builder methods return the modified rename"]
    pub fn blur_behavior(mut self, behavior: RenameBlurBehavior) -> Self {
        self.props = self.props.blur_behavior(behavior);
        self
    }

    /// Sets the fallback caret or selection behavior on edit start.
    #[must_use = "builder methods return the modified rename"]
    pub fn fallback_selection_behavior(
        mut self,
        behavior: RenameFallbackSelectionBehavior,
    ) -> Self {
        self.props = self.props.fallback_selection_behavior(behavior);
        self
    }

    /// Sets the single-line input size.
    #[must_use = "builder methods return the modified rename"]
    pub fn input_size(mut self, size: crate::InputSize) -> Self {
        self.props = self.props.input_size(size);
        self
    }

    /// Sets the single-line input radius.
    #[must_use = "builder methods return the modified rename"]
    pub fn input_radius(mut self, radius: crate::InputRadius) -> Self {
        self.props = self.props.input_radius(radius);
        self
    }

    /// Sets the multiline textarea size.
    #[must_use = "builder methods return the modified rename"]
    pub fn textarea_size(mut self, size: crate::TextareaSize) -> Self {
        self.props = self.props.textarea_size(size);
        self
    }

    /// Sets the multiline textarea radius.
    #[must_use = "builder methods return the modified rename"]
    pub fn textarea_radius(mut self, radius: crate::TextareaRadius) -> Self {
        self.props = self.props.textarea_radius(radius);
        self
    }

    /// Applies an accent color to the editor's focus ring and selection.
    #[must_use = "builder methods return the modified rename"]
    pub fn color(mut self, color: shadcn_common::AccentColor) -> Self {
        self.props = self.props.color(color);
        self
    }

    /// Applies an accent color only to single-line input mode.
    #[must_use = "builder methods return the modified rename"]
    pub fn input_color(mut self, color: shadcn_common::AccentColor) -> Self {
        self.props = self.props.input_color(color);
        self
    }

    /// Applies an accent color only to multiline textarea mode.
    #[must_use = "builder methods return the modified rename"]
    pub fn textarea_color(mut self, color: shadcn_common::AccentColor) -> Self {
        self.props = self.props.textarea_color(color);
        self
    }

    /// Sets the view text and editor text size in pixels.
    #[must_use = "builder methods return the modified rename"]
    pub fn text_size(mut self, size: f32) -> Self {
        self.props = self.props.text_size(size);
        self
    }

    /// Sets the view text line height in pixels.
    #[must_use = "builder methods return the modified rename"]
    pub fn text_line_height(mut self, line_height: f32) -> Self {
        self.props = self.props.text_line_height(line_height);
        self
    }

    /// Sets the view text color.
    #[must_use = "builder methods return the modified rename"]
    pub fn text_color(mut self, color: crate::iced_compat::Color) -> Self {
        self.props = self.props.text_color(color);
        self
    }

    /// Sets the root/editor width.
    #[must_use = "builder methods return the modified rename"]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.props = self.props.width(width);
        self
    }

    /// Sets a width override for single-line input mode.
    #[must_use = "builder methods return the modified rename"]
    pub fn input_width(mut self, width: impl Into<Length>) -> Self {
        self.props = self.props.input_width(width);
        self
    }

    /// Sets a width override for multiline textarea mode.
    #[must_use = "builder methods return the modified rename"]
    pub fn textarea_width(mut self, width: impl Into<Length>) -> Self {
        self.props = self.props.textarea_width(width);
        self
    }

    /// Disables editor interaction and external actions.
    #[must_use = "builder methods return the modified rename"]
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.props = self.props.disabled(disabled);
        self
    }

    /// Enables or disables clicking the view text to enter edit mode.
    #[must_use = "builder methods return the modified rename"]
    pub fn click_to_edit(mut self, click_to_edit: bool) -> Self {
        self.props = self.props.click_to_edit(click_to_edit);
        self
    }

    /// Maps component actions to application messages.
    #[must_use = "builder methods return the modified rename"]
    pub fn on_action<F>(mut self, on_action: F) -> Self
    where
        F: Fn(RenameAction) -> Message + 'a,
    {
        self.on_action = Some(Rc::new(on_action));
        self
    }

    /// Sets or clears the action mapper.
    #[must_use = "builder methods return the modified rename"]
    pub fn on_action_maybe<F>(mut self, on_action: Option<F>) -> Self
    where
        F: Fn(RenameAction) -> Message + 'a,
    {
        self.on_action = on_action.map(|callback| Rc::new(callback) as _);
        self
    }

    /// Builds the root as an iced element.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        rename_root(self.state, self.on_action, self.props, self.theme)
    }
}

impl<'a, Message: Clone + 'a> From<Rename<'a, Message>> for Element<'a, Message> {
    fn from(rename: Rename<'a, Message>) -> Self {
        rename.into_element()
    }
}

/// Renders a rename root with an optional action mapper.
///
/// This is the low-level counterpart of [`Rename`]. It is useful when a
/// provider composition needs the root and its external controls to share an
/// action handler.
#[must_use = "the returned element must be placed in the iced view"]
pub fn rename_root<'a, Message: Clone + 'a>(
    state: &'a RenameState,
    on_action: Option<RenameActionHandler<'a, Message>>,
    props: RenameRootProps,
    theme: &'a Theme,
) -> Element<'a, Message> {
    let on_action = if props.disabled { None } else { on_action };
    render::root(state, on_action, props, theme)
}

/// Provides the context used by the external rename controls.
///
/// The closure can render the root and controls in any layout. When no root
/// blur behavior was selected explicitly, the provider's default is `None`,
/// matching the Svelte provider's behavior.
///
/// ```rust,no_run
/// use iced::Element;
/// use iced_shadcn_v2::{
///     RenameAction, RenameActionHandler, RenameButtonProps, RenameState, Theme, rename_cancel,
///     rename_edit, rename_provider, rename_root, rename_save,
/// };
///
/// # #[derive(Debug, Clone)]
/// # enum Message { Rename(RenameAction) }
/// # fn example<'a>(state: &'a RenameState, theme: &'a Theme) -> Element<'a, Message> {
/// let handler: RenameActionHandler<'static, Message> = std::rc::Rc::new(Message::Rename);
/// rename_provider(
///     state,
///     Some(handler.clone()),
///     Default::default(),
///     |context| {
///         iced::widget::row![
///             rename_root(state, Some(handler.clone()), context.root_props(Default::default()), theme),
///             rename_edit(context, theme, RenameButtonProps::default()),
///             rename_save(context, theme, RenameButtonProps::default()),
///             rename_cancel(context, theme, RenameButtonProps::default()),
///         ]
///         .into()
///     },
/// )
/// # }
/// ```
#[must_use = "the returned element must be placed in the iced view"]
pub fn rename_provider<'a, Message: Clone + 'a>(
    state: &'a RenameState,
    on_action: Option<RenameActionHandler<'a, Message>>,
    props: RenameProviderProps,
    add_contents: impl FnOnce(&RenameContext<'a, Message>) -> Element<'a, Message>,
) -> Element<'a, Message> {
    let context = RenameContext {
        mode: state.mode(),
        invalid: state.is_invalid(),
        disabled: props.is_disabled(),
        blur_behavior: props.blur_behavior_value(),
        on_action: if props.is_disabled() { None } else { on_action },
    };

    add_contents(&context)
}

/// Renders the provider's external Edit button.
#[must_use = "the returned element must be placed in the iced view"]
pub fn rename_edit<'a, Message: Clone + 'a>(
    context: &RenameContext<'a, Message>,
    theme: &'a Theme,
    props: RenameButtonProps<'a, Message>,
) -> Element<'a, Message> {
    render::control(
        context,
        theme,
        props,
        "Edit",
        crate::ButtonVariant::Outline,
        RenameAction::StartEdit,
        RenameMode::Edit == context.mode(),
    )
}

/// Renders the provider's external Save button.
#[must_use = "the returned element must be placed in the iced view"]
pub fn rename_save<'a, Message: Clone + 'a>(
    context: &RenameContext<'a, Message>,
    theme: &'a Theme,
    props: RenameButtonProps<'a, Message>,
) -> Element<'a, Message> {
    render::control(
        context,
        theme,
        props,
        "Save",
        crate::ButtonVariant::Default,
        RenameAction::SaveRequested,
        RenameMode::View == context.mode(),
    )
}

/// Renders the provider's external Cancel button.
#[must_use = "the returned element must be placed in the iced view"]
pub fn rename_cancel<'a, Message: Clone + 'a>(
    context: &RenameContext<'a, Message>,
    theme: &'a Theme,
    props: RenameButtonProps<'a, Message>,
) -> Element<'a, Message> {
    render::control(
        context,
        theme,
        props,
        "Cancel",
        crate::ButtonVariant::Outline,
        RenameAction::CancelRequested,
        RenameMode::View == context.mode(),
    )
}

/// Applies one emitted rename action to controlled state.
///
/// `validate` returns `true` for an accepted value. A failed save leaves the
/// component in edit mode and sets [`RenameState::is_invalid`] to `true`.
/// [`RenameUpdate::request_focus`] and [`RenameUpdate::selection`] describe
/// the focus operation the application may schedule after the state update.
#[must_use = "the update flags describe focus, selection, and validation results"]
pub fn rename_apply_action<F>(
    state: &mut RenameState,
    action: RenameAction,
    input_tag: RenameInputTag,
    fallback_selection_behavior: RenameFallbackSelectionBehavior,
    blur_behavior: RenameBlurBehavior,
    validate: F,
) -> RenameUpdate
where
    F: Fn(&str) -> bool,
{
    match action {
        RenameAction::StartEdit => {
            state.mode = RenameMode::Edit;
            state.editing_value.clone_from(&state.value);
            state.invalid = !validate(&state.editing_value);
            state.sync_textarea_content();

            if input_tag.is_textarea() {
                apply_textarea_selection(&mut state.textarea_content, fallback_selection_behavior);
                RenameUpdate::new_entered(None)
            } else {
                RenameUpdate::new_entered(Some(map_selection(fallback_selection_behavior)))
            }
        }
        RenameAction::InputChanged(value) => {
            state.editing_value = value;
            state.invalid = !validate(&state.editing_value);
            state.sync_textarea_content();
            RenameUpdate::default()
        }
        RenameAction::TextareaEdited(action) => {
            if input_tag.is_textarea() {
                state.textarea_content.perform(action);
                state.editing_value = state.textarea_content.text();
                state.invalid = !validate(&state.editing_value);
            }
            RenameUpdate::default()
        }
        RenameAction::SaveRequested => {
            if validate(&state.editing_value) {
                state.value.clone_from(&state.editing_value);
                state.mode = RenameMode::View;
                state.invalid = false;
                state.sync_textarea_content();
                RenameUpdate::new_committed()
            } else {
                state.invalid = true;
                RenameUpdate::new_validation_failed()
            }
        }
        RenameAction::CancelRequested | RenameAction::EscapePressed => {
            state.cancel_editing();
            RenameUpdate::new_cancelled()
        }
        RenameAction::BlurDetected => {
            if matches!(blur_behavior, RenameBlurBehavior::Exit) {
                state.cancel_editing();
                RenameUpdate::new_cancelled()
            } else {
                RenameUpdate::default()
            }
        }
    }
}

fn map_selection(
    fallback_selection_behavior: RenameFallbackSelectionBehavior,
) -> RenameSelectionRequest {
    match fallback_selection_behavior {
        RenameFallbackSelectionBehavior::Start => RenameSelectionRequest::Start,
        RenameFallbackSelectionBehavior::End => RenameSelectionRequest::End,
        RenameFallbackSelectionBehavior::All => RenameSelectionRequest::All,
    }
}

fn apply_textarea_selection(
    content: &mut text_editor::Content,
    fallback_selection_behavior: RenameFallbackSelectionBehavior,
) {
    match fallback_selection_behavior {
        RenameFallbackSelectionBehavior::Start => {
            content.perform(text_editor::Action::Move(
                text_editor::Motion::DocumentStart,
            ));
        }
        RenameFallbackSelectionBehavior::End => {
            content.perform(text_editor::Action::Move(text_editor::Motion::DocumentEnd));
        }
        RenameFallbackSelectionBehavior::All => content.perform(text_editor::Action::SelectAll),
    }
}
