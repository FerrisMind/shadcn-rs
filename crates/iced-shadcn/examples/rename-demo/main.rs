use std::rc::Rc;

use iced::border::Border;
use iced::widget::{column, container, row, text};
use iced::{Background, Element, Length, Task};

use iced_shadcn::{
    ButtonProps, ButtonSize, RenameAction, RenameActionHandler, RenameBlurBehavior,
    RenameButtonProps, RenameFallbackSelectionBehavior, RenameInputTag, RenameProviderProps,
    RenameRootProps, RenameState, TextareaProps, Theme, rename_apply_action, rename_cancel,
    rename_edit, rename_provider, rename_root, rename_save, rename_update_task,
};

pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view).run()
}

struct Example {
    theme: Theme,
    provider_state: RenameState,
    provider_props: RenameRootProps,
    standalone_state: RenameState,
    standalone_props: RenameRootProps,
    textarea_state: RenameState,
    textarea_props: RenameRootProps,
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::default(),
            provider_state: RenameState::new("Project Apollo"),
            provider_props: RenameRootProps::new()
                .click_to_edit(false)
                .fallback_selection_behavior(RenameFallbackSelectionBehavior::End)
                .blur_behavior(RenameBlurBehavior::None),
            standalone_state: RenameState::new("quick-note"),
            standalone_props: RenameRootProps::new()
                .click_to_edit(true)
                .fallback_selection_behavior(RenameFallbackSelectionBehavior::All)
                .blur_behavior(RenameBlurBehavior::Exit),
            textarea_state: RenameState::new("Multiline title"),
            textarea_props: RenameRootProps::new()
                .input_tag(RenameInputTag::Textarea)
                .textarea_props(TextareaProps::new().rows(4))
                .fallback_selection_behavior(RenameFallbackSelectionBehavior::End)
                .blur_behavior(RenameBlurBehavior::Exit),
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    Provider(RenameAction),
    Standalone(RenameAction),
    Textarea(RenameAction),
}

impl Example {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Provider(action) => apply_rename_update(
                action,
                &mut self.provider_state,
                &self.provider_props,
                validate_provider_name,
            ),
            Message::Standalone(action) => apply_rename_update(
                action,
                &mut self.standalone_state,
                &self.standalone_props,
                validate_standalone_name,
            ),
            Message::Textarea(action) => apply_rename_update(
                action,
                &mut self.textarea_state,
                &self.textarea_props,
                validate_textarea_value,
            ),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;

        let provider_handler: RenameActionHandler<'_, Message> = Rc::new(Message::Provider);
        let provider_block = rename_provider(
            &self.provider_state,
            Some(Rc::clone(&provider_handler)),
            RenameProviderProps::new(),
            |ctx| {
                let root = rename_root(
                    &self.provider_state,
                    Some(Rc::clone(&provider_handler)),
                    self.provider_props.clone(),
                    theme,
                );

                let controls = row![
                    rename_edit(
                        ctx,
                        theme,
                        RenameButtonProps::new("Edit").button_props(
                            ButtonProps::new()
                                .size(ButtonSize::Size1)
                                .disabled(matches!(ctx.mode, iced_shadcn::RenameMode::Edit))
                        )
                    ),
                    rename_save(
                        ctx,
                        theme,
                        RenameButtonProps::new("Save")
                            .button_props(ButtonProps::new().size(ButtonSize::Size1))
                    ),
                    rename_cancel(
                        ctx,
                        theme,
                        RenameButtonProps::new("Cancel")
                            .button_props(ButtonProps::new().size(ButtonSize::Size1))
                    ),
                ]
                .spacing(8);

                column![
                    text("Provider + Root + Edit/Save/Cancel").size(16),
                    root,
                    controls,
                    text(format!("Current value: {}", self.provider_state.value)).size(12),
                ]
                .spacing(10)
                .into()
            },
        );

        let standalone_handler: RenameActionHandler<'_, Message> = Rc::new(Message::Standalone);
        let standalone_block = column![
            text("Standalone Root (click text to edit)").size(16),
            rename_root(
                &self.standalone_state,
                Some(standalone_handler),
                self.standalone_props.clone(),
                theme
            ),
            text(format!("Current value: {}", self.standalone_state.value)).size(12),
        ]
        .spacing(10);

        let textarea_handler: RenameActionHandler<'_, Message> = Rc::new(Message::Textarea);
        let textarea_block = column![
            text("Textarea mode (Enter=Save, Escape=Cancel, blur=Exit)").size(16),
            rename_root(
                &self.textarea_state,
                Some(textarea_handler),
                self.textarea_props.clone(),
                theme
            ),
            text(format!("Current value: {}", self.textarea_state.value)).size(12),
        ]
        .spacing(10);

        let content = column![
            panel(theme, provider_block),
            panel(theme, standalone_block.into()),
            panel(theme, textarea_block.into()),
        ]
        .spacing(16)
        .max_width(860);

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
}

fn panel<'a, Message: 'a>(theme: &'a Theme, content: Element<'a, Message>) -> Element<'a, Message> {
    container(content)
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

fn apply_rename_update(
    action: RenameAction,
    state: &mut RenameState,
    props: &RenameRootProps,
    validate: fn(&str) -> bool,
) -> Task<Message> {
    let update = rename_apply_action(
        state,
        action,
        props.input_tag,
        props.fallback_selection_behavior,
        props.blur_behavior,
        validate,
    );

    rename_update_task(props, update)
}

fn validate_provider_name(value: &str) -> bool {
    let trimmed = value.trim();
    !trimmed.is_empty() && trimmed.chars().count() <= 32
}

fn validate_standalone_name(value: &str) -> bool {
    let trimmed = value.trim();
    !trimmed.is_empty()
        && trimmed.chars().count() <= 24
        && trimmed
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || ch == '-' || ch == '_')
}

fn validate_textarea_value(value: &str) -> bool {
    let trimmed = value.trim();
    !trimmed.is_empty() && trimmed.chars().count() <= 120
}
