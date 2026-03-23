use std::rc::Rc;

use iced::border::Border;
use iced::widget::{column, container, text};
use iced::{Background, Element, Length, Task};
use lucide_icons::LUCIDE_FONT_BYTES;

use iced_shadcn::{
    BadgeVariant, ButtonProps, ButtonVariant, InputProps, InputVariant, TagsInputAction,
    TagsInputActionHandler, TagsInputProps, TagsInputState, Theme, tags_input, tags_input_reduce,
    tags_input_update_task,
};

pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view)
        .font(LUCIDE_FONT_BYTES)
        .run()
}

struct Example {
    theme: Theme,
    basic_state: TagsInputState,
    lowercase_state: TagsInputState,
    autocomplete_state: TagsInputState,
    restricted_state: TagsInputState,
    autocomplete_suggestions: Vec<String>,
    restricted_suggestions: Vec<String>,
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::default(),
            basic_state: TagsInputState::new(vec!["iced".to_string()]),
            lowercase_state: TagsInputState::default(),
            autocomplete_state: TagsInputState::default(),
            restricted_state: TagsInputState::default(),
            autocomplete_suggestions: vec![
                "Rust".to_string(),
                "Iced".to_string(),
                "Svelte".to_string(),
                "TypeScript".to_string(),
                "WASM".to_string(),
            ],
            restricted_suggestions: vec![
                "Bug".to_string(),
                "Feature".to_string(),
                "Docs".to_string(),
                "Refactor".to_string(),
            ],
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    Basic(TagsInputAction),
    Lowercase(TagsInputAction),
    Autocomplete(TagsInputAction),
    Restricted(TagsInputAction),
}

impl Example {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Basic(action) => {
                let props = Self::basic_props();
                apply_tags_update(action, &mut self.basic_state, &props)
            }
            Message::Lowercase(action) => {
                let props = Self::lowercase_props();
                apply_tags_update(action, &mut self.lowercase_state, &props)
            }
            Message::Autocomplete(action) => {
                let suggestions = self.autocomplete_suggestions.clone();
                let props = TagsInputProps::new()
                    .placeholder("Type to filter suggestions")
                    .suggestions(&suggestions)
                    .suggestion_button_props(
                        ButtonProps::new()
                            .variant(ButtonVariant::Ghost)
                            .high_contrast(true),
                    );
                apply_tags_update(action, &mut self.autocomplete_state, &props)
            }
            Message::Restricted(action) => {
                let suggestions = self.restricted_suggestions.clone();
                let props = TagsInputProps::new()
                    .placeholder("Only predefined values")
                    .suggestions(&suggestions)
                    .restrict_to_suggestions(true)
                    .suggestion_button_props(ButtonProps::new().variant(ButtonVariant::Outline));
                apply_tags_update(action, &mut self.restricted_state, &props)
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;

        let basic_handler: TagsInputActionHandler<'_, Message> = Rc::new(Message::Basic);
        let lowercase_handler: TagsInputActionHandler<'_, Message> = Rc::new(Message::Lowercase);
        let autocomplete_handler: TagsInputActionHandler<'_, Message> =
            Rc::new(Message::Autocomplete);
        let restricted_handler: TagsInputActionHandler<'_, Message> = Rc::new(Message::Restricted);

        let content = column![
            panel(
                theme,
                "Basic",
                tags_input(
                    &self.basic_state,
                    Some(basic_handler),
                    Self::basic_props(),
                    theme
                ),
                format!("Tags: {:?}", self.basic_state.tags)
            ),
            panel(
                theme,
                "Lowercase validate",
                tags_input(
                    &self.lowercase_state,
                    Some(lowercase_handler),
                    Self::lowercase_props(),
                    theme
                ),
                format!("Tags: {:?}", self.lowercase_state.tags)
            ),
            panel(
                theme,
                "Autocomplete",
                tags_input(
                    &self.autocomplete_state,
                    Some(autocomplete_handler),
                    self.autocomplete_props(),
                    theme
                ),
                format!("Tags: {:?}", self.autocomplete_state.tags)
            ),
            panel(
                theme,
                "Restricted suggestions",
                tags_input(
                    &self.restricted_state,
                    Some(restricted_handler),
                    self.restricted_props(),
                    theme
                ),
                format!("Tags: {:?}", self.restricted_state.tags)
            ),
        ]
        .spacing(16)
        .max_width(900);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(24)
            .center_x(Length::Fill)
            .style(move |_| iced::widget::container::Style {
                background: Some(Background::Color(theme.palette.background)),
                text_color: Some(theme.palette.foreground),
                ..iced::widget::container::Style::default()
            })
            .into()
    }

    fn basic_props() -> TagsInputProps<'static> {
        TagsInputProps::new()
            .placeholder("Add tag and press Enter")
            .input_props(InputProps::new().variant(InputVariant::Surface))
            .active_badge_variant(BadgeVariant::Default)
    }

    fn lowercase_props() -> TagsInputProps<'static> {
        TagsInputProps::new()
            .placeholder("Always lowercase")
            .validate(validate_lowercase)
            .input_props(InputProps::new().variant(InputVariant::Soft))
            .active_badge_variant(BadgeVariant::Default)
    }

    fn autocomplete_props(&self) -> TagsInputProps<'_> {
        TagsInputProps::new()
            .placeholder("Type to filter suggestions")
            .suggestions(&self.autocomplete_suggestions)
            .suggestion_button_props(
                ButtonProps::new()
                    .variant(ButtonVariant::Ghost)
                    .high_contrast(true),
            )
    }

    fn restricted_props(&self) -> TagsInputProps<'_> {
        TagsInputProps::new()
            .placeholder("Only predefined values")
            .suggestions(&self.restricted_suggestions)
            .restrict_to_suggestions(true)
            .suggestion_button_props(ButtonProps::new().variant(ButtonVariant::Outline))
    }
}

fn apply_tags_update(
    action: TagsInputAction,
    state: &mut TagsInputState,
    props: &TagsInputProps<'_>,
) -> Task<Message> {
    let effects = tags_input_reduce(state, action, props);
    tags_input_update_task(props, effects)
}

fn validate_lowercase(value: &str, tags: &[String]) -> Option<String> {
    let transformed = value.trim().to_lowercase();
    if transformed.is_empty() {
        return None;
    }
    if tags
        .iter()
        .any(|tag| tag.eq_ignore_ascii_case(transformed.as_str()))
    {
        return None;
    }
    Some(transformed)
}

fn panel<'a, Message: 'a>(
    theme: &'a Theme,
    title: &'a str,
    content: Element<'a, Message>,
    details: String,
) -> Element<'a, Message> {
    container(
        column![
            text(title).size(18),
            content,
            text(details).size(12).style(move |_| {
                iced::widget::text::Style {
                    color: Some(theme.palette.muted_foreground),
                }
            }),
        ]
        .spacing(10),
    )
    .padding(16)
    .style(move |_| iced::widget::container::Style {
        background: Some(Background::Color(theme.palette.card)),
        text_color: Some(theme.palette.card_foreground),
        border: Border {
            radius: theme.radius.md.into(),
            width: 1.0,
            color: theme.palette.border,
        },
        ..iced::widget::container::Style::default()
    })
    .into()
}
