use std::rc::Rc;

use iced::advanced::layout;
use iced::advanced::renderer;
use iced::advanced::widget::Tree;
use iced::advanced::{Clipboard, Layout, Shell, Widget};
use iced::keyboard::key::{self, Key};
use iced::mouse;
use iced::touch;
use iced::widget::{Id, column, container, row, text};
use iced::{Alignment, Background, Element, Event, Font, Length, Rectangle, Size, Task, keyboard};
use lucide_icons::Icon as LucideIcon;

use crate::badge::{BadgeProps, BadgeSize, BadgeVariant, badge_content};
use crate::button::{ButtonProps, ButtonRadius, ButtonSize, ButtonVariant, icon_button};
use crate::input::{InputProps, InputVariant, input};
use crate::select::{SelectProps, select};
use crate::theme::Theme;

pub type TagsInputValidate = fn(&str, &[String]) -> Option<String>;
pub type TagsInputFilter = fn(&str, &[String]) -> Vec<String>;
pub type TagsInputActionHandler<'a, Message> = Rc<dyn Fn(TagsInputAction) -> Message + 'a>;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TagsInputState {
    pub tags: Vec<String>,
    pub input_value: String,
    pub filtered_suggestions: Vec<String>,
    pub active_tag: Option<usize>,
    pub highlighted_suggestion: Option<usize>,
    pub input_focused: bool,
    pub invalid: bool,
}

impl TagsInputState {
    #[must_use]
    pub fn new(tags: impl Into<Vec<String>>) -> Self {
        Self {
            tags: tags.into(),
            ..Self::default()
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TagsInputAction {
    InputChanged(String),
    AddRequested,
    RemoveAt(usize),
    RemoveValue(String),
    SuggestionSelected(usize),
    HighlightNextSuggestion,
    HighlightPreviousSuggestion,
    HighlightPreviousTag,
    HighlightNextTag,
    SelectTag(usize),
    BackspacePressed,
    DeletePressed,
    Focus,
    Blur,
    EscapePressed,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TagsInputEffects {
    pub value_changed: bool,
    pub request_focus: bool,
}

#[derive(Clone, Debug)]
pub struct TagsInputProps<'a> {
    pub id: Id,
    pub input_id: Id,
    pub placeholder: &'a str,
    pub disabled: bool,
    pub suggestions: &'a [String],
    pub validate: TagsInputValidate,
    pub filter_suggestions: TagsInputFilter,
    pub restrict_to_suggestions: bool,
    pub input_props: InputProps,
    pub badge_size: BadgeSize,
    pub badge_variant: BadgeVariant,
    pub active_badge_variant: BadgeVariant,
    pub badge_radius: ButtonRadius,
    pub suggestion_button_props: ButtonProps,
    pub suggestion_select_props: SelectProps,
    pub remove_button_props: ButtonProps,
    pub suggestions_max_height: f32,
}

impl<'a> Default for TagsInputProps<'a> {
    fn default() -> Self {
        Self {
            id: Id::unique(),
            input_id: Id::unique(),
            placeholder: "",
            disabled: false,
            suggestions: &[],
            validate: default_validate,
            filter_suggestions: default_filter_suggestions,
            restrict_to_suggestions: false,
            input_props: InputProps::default(),
            badge_size: BadgeSize::Size1,
            badge_variant: BadgeVariant::Secondary,
            active_badge_variant: BadgeVariant::Default,
            badge_radius: ButtonRadius::Small,
            suggestion_button_props: ButtonProps::new()
                .variant(ButtonVariant::Ghost)
                .size(ButtonSize::Size1),
            suggestion_select_props: SelectProps::new(),
            remove_button_props: ButtonProps::new()
                .variant(ButtonVariant::Ghost)
                .size(ButtonSize::Size0),
            suggestions_max_height: 200.0,
        }
    }
}

impl<'a> TagsInputProps<'a> {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn id(mut self, id: impl Into<Id>) -> Self {
        self.id = id.into();
        self
    }

    #[must_use]
    pub fn input_id(mut self, input_id: impl Into<Id>) -> Self {
        self.input_id = input_id.into();
        self
    }

    #[must_use]
    pub fn placeholder(mut self, placeholder: &'a str) -> Self {
        self.placeholder = placeholder;
        self
    }

    #[must_use]
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    #[must_use]
    pub fn suggestions(mut self, suggestions: &'a [String]) -> Self {
        self.suggestions = suggestions;
        self
    }

    #[must_use]
    pub fn validate(mut self, validate: TagsInputValidate) -> Self {
        self.validate = validate;
        self
    }

    #[must_use]
    pub fn filter_suggestions(mut self, filter_suggestions: TagsInputFilter) -> Self {
        self.filter_suggestions = filter_suggestions;
        self
    }

    #[must_use]
    pub fn restrict_to_suggestions(mut self, restrict_to_suggestions: bool) -> Self {
        self.restrict_to_suggestions = restrict_to_suggestions;
        self
    }

    #[must_use]
    pub fn input_props(mut self, input_props: InputProps) -> Self {
        self.input_props = input_props;
        self
    }

    #[must_use]
    pub fn badge_size(mut self, badge_size: BadgeSize) -> Self {
        self.badge_size = badge_size;
        self
    }

    #[must_use]
    pub fn badge_variant(mut self, badge_variant: BadgeVariant) -> Self {
        self.badge_variant = badge_variant;
        self
    }

    #[must_use]
    pub fn active_badge_variant(mut self, active_badge_variant: BadgeVariant) -> Self {
        self.active_badge_variant = active_badge_variant;
        self
    }

    #[must_use]
    pub fn badge_radius(mut self, badge_radius: ButtonRadius) -> Self {
        self.badge_radius = badge_radius;
        self
    }

    #[must_use]
    pub fn suggestion_button_props(mut self, suggestion_button_props: ButtonProps) -> Self {
        self.suggestion_button_props = suggestion_button_props;
        self
    }

    #[must_use]
    pub fn suggestion_select_props(mut self, suggestion_select_props: SelectProps) -> Self {
        self.suggestion_select_props = suggestion_select_props;
        self
    }

    #[must_use]
    pub fn remove_button_props(mut self, remove_button_props: ButtonProps) -> Self {
        self.remove_button_props = remove_button_props;
        self
    }

    #[must_use]
    pub fn suggestions_max_height(mut self, suggestions_max_height: f32) -> Self {
        self.suggestions_max_height = suggestions_max_height.max(40.0);
        self
    }
}

pub fn default_validate(value: &str, tags: &[String]) -> Option<String> {
    let transformed = value.trim();
    if transformed.is_empty() {
        return None;
    }
    if tags.iter().any(|tag| tag.eq_ignore_ascii_case(transformed)) {
        return None;
    }
    Some(transformed.to_string())
}

pub fn default_filter_suggestions(input_value: &str, suggestions: &[String]) -> Vec<String> {
    let needle = input_value.to_lowercase();
    suggestions
        .iter()
        .filter(|suggestion| suggestion.to_lowercase().contains(&needle))
        .cloned()
        .collect()
}

#[must_use]
pub fn filtered_suggestions(state: &TagsInputState, props: &TagsInputProps<'_>) -> Vec<String> {
    if props.suggestions.is_empty() {
        return Vec::new();
    }

    let available: Vec<String> = props
        .suggestions
        .iter()
        .filter(|suggestion| {
            !state
                .tags
                .iter()
                .any(|tag| tag.eq_ignore_ascii_case(suggestion.as_str()))
        })
        .cloned()
        .collect();

    let trimmed = state.input_value.trim();
    if trimmed.is_empty() {
        return available;
    }

    (props.filter_suggestions)(trimmed, &available)
}

fn show_suggestions(state: &TagsInputState, suggestions: &[String]) -> bool {
    state.input_focused && state.active_tag.is_none() && !suggestions.is_empty()
}

fn reset_suggestion_highlight(state: &mut TagsInputState, props: &TagsInputProps<'_>) {
    state.filtered_suggestions = filtered_suggestions(state, props);
    state.highlighted_suggestion = if show_suggestions(state, &state.filtered_suggestions) {
        Some(0)
    } else {
        None
    };
}

fn push_tag(
    state: &mut TagsInputState,
    props: &TagsInputProps<'_>,
    value: String,
    effects: &mut TagsInputEffects,
) {
    state.tags.push(value);
    state.input_value.clear();
    state.active_tag = None;
    state.input_focused = true;
    state.invalid = false;
    effects.value_changed = true;
    effects.request_focus = true;
    reset_suggestion_highlight(state, props);
}

fn remove_tag_at(state: &mut TagsInputState, index: usize, effects: &mut TagsInputEffects) -> bool {
    if index >= state.tags.len() {
        return false;
    }

    state.tags.remove(index);
    effects.value_changed = true;

    if let Some(active) = state.active_tag {
        if state.tags.is_empty() {
            state.active_tag = None;
        } else if active == index {
            state.active_tag = Some(active.min(state.tags.len() - 1));
        } else if active > index {
            state.active_tag = Some(active - 1);
        }
    }

    true
}

#[must_use]
pub fn tags_input_reduce(
    state: &mut TagsInputState,
    action: TagsInputAction,
    props: &TagsInputProps<'_>,
) -> TagsInputEffects {
    let mut effects = TagsInputEffects::default();

    if props.disabled && !matches!(action, TagsInputAction::Blur) {
        return effects;
    }

    match action {
        TagsInputAction::InputChanged(value) => {
            state.input_value = value;
            state.active_tag = None;
            state.input_focused = true;
            state.invalid = false;
            reset_suggestion_highlight(state, props);
        }
        TagsInputAction::AddRequested => {
            let current_suggestions = filtered_suggestions(state, props);
            let can_pick_suggestion = show_suggestions(state, &current_suggestions);

            if can_pick_suggestion
                && let Some(index) = state.highlighted_suggestion
                && let Some(value) = current_suggestions.get(index)
            {
                push_tag(state, props, value.clone(), &mut effects);
                return effects;
            }

            if props.restrict_to_suggestions && !props.suggestions.is_empty() {
                let raw = state.input_value.trim();
                let suggestion = props
                    .suggestions
                    .iter()
                    .find(|suggestion| suggestion.eq_ignore_ascii_case(raw));

                if let Some(suggestion) = suggestion {
                    push_tag(state, props, suggestion.clone(), &mut effects);
                } else {
                    state.invalid = true;
                }
                return effects;
            }

            if let Some(value) = (props.validate)(&state.input_value, &state.tags) {
                push_tag(state, props, value, &mut effects);
            } else {
                state.invalid = true;
            }
        }
        TagsInputAction::RemoveAt(index) => {
            remove_tag_at(state, index, &mut effects);
            reset_suggestion_highlight(state, props);
        }
        TagsInputAction::RemoveValue(value) => {
            if let Some(index) = state.tags.iter().position(|tag| tag == &value) {
                remove_tag_at(state, index, &mut effects);
                reset_suggestion_highlight(state, props);
            }
        }
        TagsInputAction::SuggestionSelected(index) => {
            let suggestions = filtered_suggestions(state, props);
            if let Some(value) = suggestions.get(index) {
                push_tag(state, props, value.clone(), &mut effects);
            }
        }
        TagsInputAction::HighlightNextSuggestion => {
            if show_suggestions(state, &state.filtered_suggestions) {
                state.highlighted_suggestion = match state.highlighted_suggestion {
                    Some(current) => Some((current + 1) % state.filtered_suggestions.len()),
                    None => Some(0),
                };
            }
        }
        TagsInputAction::HighlightPreviousSuggestion => {
            if show_suggestions(state, &state.filtered_suggestions) {
                state.highlighted_suggestion = match state.highlighted_suggestion {
                    Some(current) => Some(
                        (current + state.filtered_suggestions.len() - 1)
                            % state.filtered_suggestions.len(),
                    ),
                    None => Some(state.filtered_suggestions.len() - 1),
                };
            }
        }
        TagsInputAction::HighlightPreviousTag => {
            if state.input_value.is_empty() {
                state.highlighted_suggestion = None;
                state.active_tag = match state.active_tag {
                    Some(index) => Some(index.saturating_sub(1)),
                    None => state.tags.len().checked_sub(1),
                };
            }
        }
        TagsInputAction::HighlightNextTag => {
            if state.input_value.is_empty() {
                state.highlighted_suggestion = None;
                state.active_tag = match state.active_tag {
                    Some(index) if index + 1 < state.tags.len() => Some(index + 1),
                    _ => None,
                };
            }
        }
        TagsInputAction::SelectTag(index) => {
            state.highlighted_suggestion = None;
            state.active_tag = if index < state.tags.len() {
                Some(index)
            } else {
                None
            };
        }
        TagsInputAction::BackspacePressed => {
            if state.input_value.is_empty() {
                state.highlighted_suggestion = None;
                if let Some(index) = state.active_tag {
                    if remove_tag_at(state, index, &mut effects) {
                        state.active_tag = index.checked_sub(1);
                    }
                } else {
                    state.active_tag = state.tags.len().checked_sub(1);
                }
            }
        }
        TagsInputAction::DeletePressed => {
            if state.input_value.is_empty() {
                state.highlighted_suggestion = None;
                if let Some(index) = state.active_tag
                    && remove_tag_at(state, index, &mut effects)
                    && state.tags.is_empty()
                {
                    state.active_tag = None;
                }
            }
        }
        TagsInputAction::Focus => {
            state.input_focused = true;
            state.active_tag = None;
            reset_suggestion_highlight(state, props);
        }
        TagsInputAction::Blur => {
            state.input_focused = false;
            state.active_tag = None;
            state.highlighted_suggestion = None;
        }
        TagsInputAction::EscapePressed => {
            state.input_focused = false;
            state.active_tag = None;
            state.highlighted_suggestion = None;
        }
    }

    effects
}

pub fn tags_input_update_task<Message: 'static>(
    props: &TagsInputProps<'_>,
    effects: TagsInputEffects,
) -> Task<Message> {
    if !effects.request_focus {
        return Task::none();
    }

    Task::batch(vec![
        iced::widget::operation::focus(props.input_id.clone()),
        iced::widget::operation::move_cursor_to_end(props.input_id.clone()),
    ])
}

#[must_use]
pub fn tags_input<'a, Message: Clone + 'a>(
    state: &'a TagsInputState,
    on_action: Option<TagsInputActionHandler<'a, Message>>,
    props: TagsInputProps<'a>,
    theme: &'a Theme,
) -> Element<'a, Message> {
    let suggestion_items = state.filtered_suggestions.as_slice();
    let suggestions_open = show_suggestions(state, suggestion_items);

    let mut tags_row = row![]
        .spacing(6)
        .align_y(Alignment::Center)
        .width(Length::Fill);

    for (index, tag) in state.tags.iter().enumerate() {
        let badge_variant = if state.active_tag == Some(index) {
            props.active_badge_variant
        } else {
            props.badge_variant
        };

        let badge_props = BadgeProps::new()
            .size(props.badge_size)
            .variant(badge_variant)
            .radius(props.badge_radius);

        let remove_message = on_action
            .as_ref()
            .map(|on_action| (on_action)(TagsInputAction::RemoveAt(index)));

        let remove_icon = text(char::from(LucideIcon::X).to_string())
            .font(Font::with_name("lucide"))
            .size(10);

        let remove_button = icon_button(
            remove_icon,
            remove_message,
            props
                .remove_button_props
                .size(ButtonSize::Size0)
                .disabled(props.disabled),
            theme,
        )
        .width(Length::Fixed(16.0))
        .height(Length::Fixed(16.0));

        let chip_content = row![text(tag.clone()), remove_button]
            .spacing(2)
            .align_y(Alignment::Center);
        let chip_badge: Element<'a, Message> = badge_content(chip_content, badge_props, theme);
        tags_row = tags_row.push(chip_badge);
    }

    let on_input = on_action.as_ref().map(|on_action| {
        let on_action = Rc::clone(on_action);
        move |value: String| (on_action)(TagsInputAction::InputChanged(value))
    });

    let mut input_widget = input(
        &state.input_value,
        props.placeholder,
        on_input,
        props
            .input_props
            .variant(InputVariant::Ghost)
            .disabled(props.disabled),
        theme,
    )
    .id(props.input_id.clone())
    .width(Length::Fill);

    if let Some(on_action) = on_action.as_ref()
        && !props.disabled
    {
        input_widget = input_widget.on_submit((on_action)(TagsInputAction::AddRequested));
    }

    tags_row = tags_row.push(input_widget);

    let border_color = if state.invalid {
        theme.palette.destructive
    } else {
        theme.palette.input
    };
    let background_color = if props.disabled {
        theme.palette.muted
    } else {
        theme.palette.background
    };

    let trigger = container(tags_row)
        .id(props.id.clone())
        .width(Length::Fill)
        .padding([4.0, 6.0])
        .style(move |_theme: &iced::Theme| iced::widget::container::Style {
            background: Some(Background::Color(background_color)),
            border: iced::border::Border {
                color: border_color,
                width: 1.0,
                radius: theme.radius.sm.into(),
            },
            ..iced::widget::container::Style::default()
        });

    let body: Element<'a, Message> = if suggestions_open {
        if let Some(on_action) = on_action.as_ref() {
            let suggestion_snapshot = suggestion_items.to_vec();
            let selected_value = state
                .highlighted_suggestion
                .and_then(|index| suggestion_items.get(index))
                .cloned();
            let on_select = {
                let on_action = Rc::clone(on_action);
                move |selected: String| {
                    let index = suggestion_snapshot
                        .iter()
                        .position(|candidate| candidate == &selected)
                        .unwrap_or_default();
                    (on_action)(TagsInputAction::SuggestionSelected(index))
                }
            };

            let suggestions_select = select(
                suggestion_items,
                selected_value,
                "",
                on_select,
                props.suggestion_select_props.disabled(props.disabled),
                theme,
            )
            .width(Length::Fill)
            .menu_height(Length::Fixed(props.suggestions_max_height));

            column![trigger, suggestions_select]
                .spacing(4)
                .width(Length::Fill)
                .into()
        } else {
            trigger.into()
        }
    } else {
        trigger.into()
    };

    TagsInputRoot {
        content: body,
        on_action,
        disabled: props.disabled,
        input_focused: state.input_focused,
        input_empty: state.input_value.is_empty(),
        show_suggestions: suggestions_open,
    }
    .into()
}

struct TagsInputRoot<'a, Message> {
    content: Element<'a, Message>,
    on_action: Option<TagsInputActionHandler<'a, Message>>,
    disabled: bool,
    input_focused: bool,
    input_empty: bool,
    show_suggestions: bool,
}

impl<Message> Widget<Message, iced::Theme, iced::Renderer> for TagsInputRoot<'_, Message>
where
    Message: Clone,
{
    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&[self.content.as_widget()]);
    }

    fn size(&self) -> Size<Length> {
        self.content.as_widget().size()
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        self.content
            .as_widget_mut()
            .layout(&mut tree.children[0], renderer, limits)
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &iced::Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        self.content.as_widget_mut().update(
            &mut tree.children[0],
            event,
            layout,
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );

        if self.disabled {
            return;
        }

        let Some(on_action) = self.on_action.as_ref() else {
            return;
        };

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if cursor.is_over(layout.bounds()) {
                    shell.publish((on_action)(TagsInputAction::Focus));
                } else {
                    shell.publish((on_action)(TagsInputAction::Blur));
                }
            }
            Event::Keyboard(keyboard::Event::KeyPressed {
                key: Key::Named(key::Named::Escape),
                ..
            }) if self.input_focused => {
                shell.publish((on_action)(TagsInputAction::EscapePressed));
                shell.capture_event();
            }
            Event::Keyboard(keyboard::Event::KeyPressed {
                key: Key::Named(key::Named::Enter),
                ..
            }) if self.input_focused => {
                shell.publish((on_action)(TagsInputAction::AddRequested));
                shell.capture_event();
            }
            Event::Keyboard(keyboard::Event::KeyPressed {
                key: Key::Named(key::Named::ArrowDown),
                ..
            }) if self.input_focused && self.show_suggestions => {
                shell.publish((on_action)(TagsInputAction::HighlightNextSuggestion));
                shell.capture_event();
            }
            Event::Keyboard(keyboard::Event::KeyPressed {
                key: Key::Named(key::Named::ArrowUp),
                ..
            }) if self.input_focused && self.show_suggestions => {
                shell.publish((on_action)(TagsInputAction::HighlightPreviousSuggestion));
                shell.capture_event();
            }
            Event::Keyboard(keyboard::Event::KeyPressed {
                key: Key::Named(key::Named::ArrowLeft),
                ..
            }) if self.input_focused && self.input_empty => {
                shell.publish((on_action)(TagsInputAction::HighlightPreviousTag));
                shell.capture_event();
            }
            Event::Keyboard(keyboard::Event::KeyPressed {
                key: Key::Named(key::Named::ArrowRight),
                ..
            }) if self.input_focused && self.input_empty => {
                shell.publish((on_action)(TagsInputAction::HighlightNextTag));
                shell.capture_event();
            }
            Event::Keyboard(keyboard::Event::KeyPressed {
                key: Key::Named(key::Named::Backspace),
                ..
            }) if self.input_focused && self.input_empty => {
                shell.publish((on_action)(TagsInputAction::BackspacePressed));
            }
            Event::Keyboard(keyboard::Event::KeyPressed {
                key: Key::Named(key::Named::Delete),
                ..
            }) if self.input_focused && self.input_empty => {
                shell.publish((on_action)(TagsInputAction::DeletePressed));
            }
            Event::Keyboard(keyboard::Event::KeyPressed {
                key: Key::Named(key::Named::Tab),
                ..
            }) if self.input_focused => {
                shell.publish((on_action)(TagsInputAction::Blur));
            }
            _ => {}
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut iced::Renderer,
        theme: &iced::Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        self.content.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            style,
            layout,
            cursor,
            viewport,
        );
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        self.content.as_widget().mouse_interaction(
            &tree.children[0],
            layout,
            cursor,
            viewport,
            renderer,
        )
    }
}

impl<'a, Message: Clone + 'a> From<TagsInputRoot<'a, Message>> for Element<'a, Message> {
    fn from(widget: TagsInputRoot<'a, Message>) -> Self {
        Element::new(widget)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn reduce(
        state: &mut TagsInputState,
        action: TagsInputAction,
        suggestions: &[String],
    ) -> TagsInputEffects {
        tags_input_reduce(
            state,
            action,
            &TagsInputProps::new().suggestions(suggestions),
        )
    }

    #[test]
    fn default_props_and_builders() {
        let suggestions = vec!["Rust".to_string()];
        let props = TagsInputProps::new()
            .placeholder("Add tag")
            .disabled(true)
            .suggestions(&suggestions)
            .restrict_to_suggestions(true)
            .badge_size(BadgeSize::Size3)
            .badge_variant(BadgeVariant::Outline)
            .active_badge_variant(BadgeVariant::Destructive)
            .suggestions_max_height(120.0);

        assert_eq!(props.placeholder, "Add tag");
        assert!(props.disabled);
        assert_eq!(props.suggestions, suggestions.as_slice());
        assert!(props.restrict_to_suggestions);
        assert_eq!(props.badge_size, BadgeSize::Size3);
        assert_eq!(props.badge_variant, BadgeVariant::Outline);
        assert_eq!(props.active_badge_variant, BadgeVariant::Destructive);
        assert_eq!(props.suggestions_max_height, 120.0);
    }

    #[test]
    fn add_trim_and_reject_empty() {
        let suggestions = vec![];
        let mut state = TagsInputState::default();
        reduce(
            &mut state,
            TagsInputAction::InputChanged("  rust  ".to_string()),
            &suggestions,
        );
        let effects = reduce(&mut state, TagsInputAction::AddRequested, &suggestions);
        assert_eq!(state.tags, vec!["rust".to_string()]);
        assert!(state.input_value.is_empty());
        assert!(!state.invalid);
        assert!(effects.value_changed);

        reduce(
            &mut state,
            TagsInputAction::InputChanged("   ".to_string()),
            &suggestions,
        );
        let effects = reduce(&mut state, TagsInputAction::AddRequested, &suggestions);
        assert_eq!(state.tags, vec!["rust".to_string()]);
        assert!(state.invalid);
        assert!(!effects.value_changed);
    }

    #[test]
    fn duplicate_reject() {
        let suggestions = vec![];
        let mut state = TagsInputState::new(vec!["rust".to_string()]);
        reduce(
            &mut state,
            TagsInputAction::InputChanged("RuSt".to_string()),
            &suggestions,
        );
        let effects = reduce(&mut state, TagsInputAction::AddRequested, &suggestions);
        assert_eq!(state.tags, vec!["rust".to_string()]);
        assert!(state.invalid);
        assert!(!effects.value_changed);
    }

    #[test]
    fn remove_by_index_and_value() {
        let suggestions = vec![];
        let mut state = TagsInputState::new(vec![
            "rust".to_string(),
            "iced".to_string(),
            "svelte".to_string(),
        ]);
        let effects = reduce(&mut state, TagsInputAction::RemoveAt(1), &suggestions);
        assert_eq!(state.tags, vec!["rust".to_string(), "svelte".to_string()]);
        assert!(effects.value_changed);

        let effects = reduce(
            &mut state,
            TagsInputAction::RemoveValue("rust".to_string()),
            &suggestions,
        );
        assert_eq!(state.tags, vec!["svelte".to_string()]);
        assert!(effects.value_changed);
    }

    #[test]
    fn restrict_to_suggestions_accepts_and_rejects() {
        let suggestions = vec!["Rust".to_string(), "Iced".to_string()];
        let props = TagsInputProps::new()
            .suggestions(&suggestions)
            .restrict_to_suggestions(true);
        let mut state = TagsInputState::default();

        let _ = tags_input_reduce(
            &mut state,
            TagsInputAction::InputChanged("rust".to_string()),
            &props,
        );
        let effects = tags_input_reduce(&mut state, TagsInputAction::AddRequested, &props);
        assert_eq!(state.tags, vec!["Rust".to_string()]);
        assert!(!state.invalid);
        assert!(effects.value_changed);

        let _ = tags_input_reduce(
            &mut state,
            TagsInputAction::InputChanged("unknown".to_string()),
            &props,
        );
        let effects = tags_input_reduce(&mut state, TagsInputAction::AddRequested, &props);
        assert_eq!(state.tags, vec!["Rust".to_string()]);
        assert!(state.invalid);
        assert!(!effects.value_changed);
    }

    #[test]
    fn suggestion_filtering_excludes_selected_tags() {
        let suggestions = vec!["Rust".to_string(), "Iced".to_string(), "Svelte".to_string()];
        let mut state = TagsInputState::new(vec!["rust".to_string()]);
        state.input_value = "e".to_string();
        let filtered =
            filtered_suggestions(&state, &TagsInputProps::new().suggestions(&suggestions));
        assert_eq!(filtered, vec!["Iced".to_string(), "Svelte".to_string()]);
    }

    #[test]
    fn keyboard_transitions() {
        let suggestions = vec!["Rust".to_string(), "Iced".to_string(), "Svelte".to_string()];
        let props = TagsInputProps::new().suggestions(&suggestions);
        let mut state = TagsInputState::new(vec!["Tokio".to_string(), "Axum".to_string()]);

        let _ = tags_input_reduce(&mut state, TagsInputAction::Focus, &props);
        assert!(state.input_focused);
        assert_eq!(state.highlighted_suggestion, Some(0));

        let _ = tags_input_reduce(&mut state, TagsInputAction::HighlightNextSuggestion, &props);
        assert_eq!(state.highlighted_suggestion, Some(1));

        let _ = tags_input_reduce(
            &mut state,
            TagsInputAction::InputChanged(String::new()),
            &props,
        );
        let _ = tags_input_reduce(&mut state, TagsInputAction::HighlightPreviousTag, &props);
        assert_eq!(state.active_tag, Some(1));

        let _ = tags_input_reduce(&mut state, TagsInputAction::BackspacePressed, &props);
        assert_eq!(state.tags, vec!["Tokio".to_string()]);
        assert_eq!(state.active_tag, Some(0));

        let _ = tags_input_reduce(&mut state, TagsInputAction::DeletePressed, &props);
        assert!(state.tags.is_empty());

        let _ = tags_input_reduce(
            &mut state,
            TagsInputAction::InputChanged("rust".to_string()),
            &props,
        );
        let _ = tags_input_reduce(&mut state, TagsInputAction::AddRequested, &props);
        assert_eq!(state.tags, vec!["Rust".to_string()]);

        let _ = tags_input_reduce(&mut state, TagsInputAction::EscapePressed, &props);
        assert!(!state.input_focused);
        assert_eq!(state.highlighted_suggestion, None);
        assert_eq!(state.active_tag, None);
    }
}
