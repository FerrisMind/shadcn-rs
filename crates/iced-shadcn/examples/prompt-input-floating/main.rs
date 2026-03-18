use iced::widget::{column, container, text, text_editor};
use iced::{Element, Length, Subscription, Task};
use lucide_icons::LUCIDE_FONT_BYTES;
use std::time::Duration;

use iced_shadcn::{
    PromptInputFloatingActions, PromptInputFloatingProps, Theme, prompt_input_floating,
};

pub fn main() -> iced::Result {
    Example::run()
}

#[derive(Debug, Clone)]
enum Message {
    EditorAction(text_editor::Action),
    Tick,
    AddAttachment,
    Search,
    More,
    Mic,
    Submit,
    Stop,
}

struct Example {
    theme: Theme,
    editor: text_editor::Content,
    is_loading: bool,
    last_submit: String,
    clear_after_ms: Option<u16>,
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::default(),
            editor: text_editor::Content::new(),
            is_loading: false,
            last_submit: String::new(),
            clear_after_ms: None,
        }
    }
}

impl Example {
    fn subscription(&self) -> Subscription<Message> {
        if self.clear_after_ms.is_some() {
            iced::time::every(Duration::from_millis(100)).map(|_| Message::Tick)
        } else {
            Subscription::none()
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::EditorAction(action) => {
                self.editor.perform(action);
            }
            Message::Tick => {
                if let Some(remaining) = self.clear_after_ms {
                    if remaining <= 100 {
                        self.is_loading = false;
                        self.clear_after_ms = None;
                    } else {
                        self.clear_after_ms = Some(remaining - 100);
                    }
                }
            }
            Message::AddAttachment => {}
            Message::Search => {}
            Message::More => {}
            Message::Mic => {}
            Message::Submit => {
                let text = self.editor.text().trim().to_string();
                if text.is_empty() {
                    return Task::none();
                }
                self.last_submit = text;
                self.editor = text_editor::Content::new();
                self.is_loading = true;
                self.clear_after_ms = Some(1500);
            }
            Message::Stop => {
                self.is_loading = false;
                self.clear_after_ms = None;
            }
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let content = self.editor.text();
        let can_submit = !content.trim().is_empty();
        let prompt = prompt_input_floating(
            &self.editor,
            Some(Message::EditorAction),
            PromptInputFloatingActions::new()
                .add_action(Some(Message::AddAttachment))
                .search_action(Some(Message::Search))
                .more_action(Some(Message::More))
                .mic_action(Some(Message::Mic))
                .submit_action(Some(Message::Submit))
                .stop_action(Some(Message::Stop)),
            PromptInputFloatingProps::new()
                .placeholder("Ask anything")
                .can_submit(can_submit)
                .is_loading(self.is_loading)
                .max_width(640.0)
                .horizontal_inset(0.0)
                .bottom_inset(12.0)
                .root_radius(24.0),
            &self.theme,
        );

        let body = column![
            container(
                text("Reference: ai-elements/prompt-kit block `prompt-input-with-actions`")
                    .size(14),
            )
            .padding(16)
            .width(Length::Fill),
            container(text(format!("Last submit: {}", self.last_submit)).size(13))
                .padding([0, 16])
                .width(Length::Fill),
            container(prompt)
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(iced::alignment::Horizontal::Center)
                .align_y(iced::alignment::Vertical::Center),
        ]
        .spacing(12)
        .height(Length::Fill)
        .width(Length::Fill);

        container(body)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

impl Example {
    fn run() -> iced::Result {
        iced::application(Example::default, Example::update, Example::view)
            .title("Prompt Input (Floating)")
            .font(LUCIDE_FONT_BYTES)
            .subscription(Example::subscription)
            .run()
    }
}
