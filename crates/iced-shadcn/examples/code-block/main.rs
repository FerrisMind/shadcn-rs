use iced::widget::{column, container, row, text};
use iced::{Element, Length, Task};
use lucide_icons::LUCIDE_FONT_BYTES;

use iced_shadcn::{
    BadgeProps, BadgeSize, BadgeVariant, CodeBlockCodeProps, CodeBlockCopyAction,
    CodeBlockCopyButtonProps, CodeBlockCopyState, CodeBlockGroupProps, CodeBlockProps, Theme,
    badge, code_block, code_block_code, code_block_copy_button, code_block_copy_reduce,
    code_block_copy_task, code_block_group,
};

pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view)
        .title("Code Block with Copy")
        .font(LUCIDE_FONT_BYTES)
        .run()
}

#[derive(Debug, Clone)]
enum Message {
    Pressed,
    WriteFinished(Result<(), String>),
    ResetDue,
}

struct Example {
    theme: Theme,
    code: String,
    copy_state: CodeBlockCopyState,
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::default(),
            code: r#"use std::collections::HashMap;

fn group_by_length(words: &[&str]) -> HashMap<usize, Vec<&str>> {
    let mut map = HashMap::new();
    for word in words {
        map.entry(word.len()).or_insert_with(Vec::new).push(*word);
    }
    map
}

fn main() {
    let words = vec!["iced", "shadcn", "code", "block", "copy"];
    let grouped = group_by_length(&words);
    println!("{grouped:?}");
}"#
            .to_owned(),
            copy_state: CodeBlockCopyState::default(),
        }
    }
}

impl Example {
    fn update(&mut self, message: Message) -> Task<Message> {
        let action = match message {
            Message::Pressed => CodeBlockCopyAction::Pressed {
                text: self.code.clone(),
            },
            Message::WriteFinished(result) => CodeBlockCopyAction::WriteFinished(result),
            Message::ResetDue => CodeBlockCopyAction::ResetDue,
        };

        let update = code_block_copy_reduce(&mut self.copy_state, action, 1200);
        code_block_copy_task(update, Message::WriteFinished, || Message::ResetDue)
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let language_badge = badge(
            "Rust",
            BadgeProps::new()
                .variant(BadgeVariant::Secondary)
                .size(BadgeSize::Size1),
            theme,
        );
        let filename = text("main.rs")
            .size(13)
            .style(|_| iced::widget::text::Style {
                color: Some(theme.palette.muted_foreground),
            });
        let leading = row![language_badge, filename]
            .spacing(8)
            .align_y(iced::Alignment::Center);

        let copy_button = code_block_copy_button(
            self.copy_state.status,
            Some(Message::Pressed),
            CodeBlockCopyButtonProps::new(),
            theme,
        );
        let header = code_block_group(leading, copy_button, CodeBlockGroupProps::new(), theme);
        let code = code_block_code(
            CodeBlockCodeProps::new(&self.code)
                .language("rs")
                .highlighter_theme(iced::highlighter::Theme::InspiredGitHub),
            theme,
        );

        let block = code_block(
            column![header, code].spacing(0),
            CodeBlockProps::new().max_width(880.0),
            theme,
        );

        container(block)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .padding(24)
            .into()
    }
}
