use std::time::{Duration, Instant};

use iced::time;
use iced::widget::{column, container, row, text};
use iced::{Alignment, Element, Length, Subscription, Task};
use lucide_icons::LUCIDE_FONT_BYTES;

use iced_shadcn::{
    ButtonProps, ButtonSize, ButtonVariant, ReasoningContentProps, ReasoningProps, ReasoningState,
    ReasoningTextProps, ReasoningTriggerProps, ReasoningUpdate, Theme, button_content, reasoning,
    reasoning_reduce, reasoning_text, reasoning_trigger_default,
};

const STREAM_TICK_MS: u64 = 350;
const STREAM_PARTS: &[&str] = &[
    "Let me think about this problem step by step.",
    "First, I need to understand what the user is asking for.",
    "They want a reasoning component that opens automatically when streaming begins and closes when it finishes.",
];

pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view)
        .font(LUCIDE_FONT_BYTES)
        .subscription(Example::subscription)
        .run()
}

#[derive(Debug, Clone)]
enum Message {
    Tick(Instant),
    StartStreaming,
    StopStreaming,
    ToggleOpen,
}

struct Example {
    theme: Theme,
    reasoning_state: ReasoningState,
    reasoning_text: String,
    stream_index: usize,
}

impl Default for Example {
    fn default() -> Self {
        let props = ReasoningProps::new();
        Self {
            theme: Theme::default(),
            reasoning_state: ReasoningState {
                open: true,
                is_streaming: false,
                duration_seconds: 4,
                ..ReasoningState::from_props(props)
            },
            reasoning_text: STREAM_PARTS.join("\n"),
            stream_index: STREAM_PARTS.len(),
        }
    }
}

impl Example {
    fn subscription(&self) -> Subscription<Message> {
        time::every(Duration::from_millis(STREAM_TICK_MS)).map(Message::Tick)
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        let props = ReasoningProps::new();
        match message {
            Message::Tick(now) => {
                let _ = reasoning_reduce(
                    &mut self.reasoning_state,
                    ReasoningUpdate::Tick { now },
                    props,
                );

                if self.reasoning_state.is_streaming {
                    if let Some(next) = STREAM_PARTS.get(self.stream_index) {
                        if !self.reasoning_text.is_empty() {
                            self.reasoning_text.push('\n');
                        }
                        self.reasoning_text.push_str(next);
                        self.stream_index += 1;
                    } else {
                        let _ = reasoning_reduce(
                            &mut self.reasoning_state,
                            ReasoningUpdate::StreamingChanged {
                                is_streaming: false,
                                now,
                            },
                            props,
                        );
                    }
                }
            }
            Message::StartStreaming => {
                self.reasoning_text.clear();
                self.stream_index = 0;
                self.reasoning_state = ReasoningState::from_props(props);
                let _ = reasoning_reduce(
                    &mut self.reasoning_state,
                    ReasoningUpdate::StreamingChanged {
                        is_streaming: true,
                        now: Instant::now(),
                    },
                    props,
                );
            }
            Message::StopStreaming => {
                let _ = reasoning_reduce(
                    &mut self.reasoning_state,
                    ReasoningUpdate::StreamingChanged {
                        is_streaming: false,
                        now: Instant::now(),
                    },
                    props,
                );
            }
            Message::ToggleOpen => {
                let next_open = !self.reasoning_state.open;
                let _ = reasoning_reduce(
                    &mut self.reasoning_state,
                    ReasoningUpdate::OpenChanged(next_open),
                    props,
                );
            }
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let props = ReasoningProps::new();

        let trigger = reasoning_trigger_default(
            self.reasoning_state.open,
            self.reasoning_state.is_streaming,
            Some(self.reasoning_state.duration_seconds),
            ReasoningTriggerProps::new(),
            theme,
        );

        let content = reasoning_text(
            &self.reasoning_text,
            ReasoningTextProps::new().paragraph_spacing(6.0),
            theme,
        );

        let reasoning_block = reasoning(
            self.reasoning_state.open,
            trigger,
            content,
            Some(|_| Message::ToggleOpen),
            props,
            ReasoningContentProps::new().muted(false),
            theme,
        );

        let controls = row![
            button_content(
                text("Start"),
                Some(Message::StartStreaming),
                ButtonProps::new()
                    .size(ButtonSize::Size1)
                    .variant(ButtonVariant::Default),
                theme
            ),
            button_content(
                text("Stop"),
                Some(Message::StopStreaming),
                ButtonProps::new()
                    .size(ButtonSize::Size1)
                    .variant(ButtonVariant::Outline),
                theme
            ),
            button_content(
                text(if self.reasoning_state.open {
                    "Collapse"
                } else {
                    "Expand"
                }),
                Some(Message::ToggleOpen),
                ButtonProps::new()
                    .size(ButtonSize::Size1)
                    .variant(ButtonVariant::Ghost),
                theme
            ),
        ]
        .spacing(8)
        .align_y(Alignment::Center);

        container(column![controls, reasoning_block].spacing(16))
            .padding(16)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
