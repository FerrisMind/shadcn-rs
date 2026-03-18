use std::time::Duration;

use iced::widget::{Id, column, container, row, text};
use iced::{Alignment, Element, Length, Subscription, Task};
use lucide_icons::LUCIDE_FONT_BYTES;

use iced_shadcn::{
    AvatarProps, AvatarSize, AvatarVariant, ConversationBubbleProps, ConversationBubbleRole,
    ConversationContentProps, ConversationEmptyStateProps, ConversationProps,
    ConversationScrollAnimation, ConversationScrollAnimator, ConversationScrollButtonProps,
    ScrollAreaScrollbars, Theme, avatar, conversation, conversation_bubble, conversation_content,
    conversation_empty_state, conversation_overlay_scroll_button, conversation_scroll_button,
};

const CONTENT_ID: &str = "conversation-example-content";
const MESSAGE_REVEAL_INTERVAL_MS: u64 = 500;

pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view)
        .font(LUCIDE_FONT_BYTES)
        .subscription(Example::subscription)
        .run()
}

#[derive(Debug, Clone)]
enum Message {
    Tick,
    ScrollToBottom,
    AnimateScroll,
    Scrolled(iced::widget::scrollable::Viewport),
}

#[derive(Debug, Clone)]
struct ChatMessage {
    role: Role,
    name: &'static str,
    content: &'static str,
}

#[derive(Debug, Clone, Copy)]
enum Role {
    User,
    Assistant,
}

struct Example {
    theme: Theme,
    messages: Vec<ChatMessage>,
    visible_count: usize,
    is_at_bottom: bool,
    scroll_animator: ConversationScrollAnimator,
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::default(),
            messages: scripted_messages(),
            visible_count: 0,
            is_at_bottom: true,
            scroll_animator: ConversationScrollAnimator::new(
                ConversationScrollAnimation::new()
                    .enabled(true)
                    .speed_px_per_sec(2400.0)
                    .tick_ms(16),
            ),
        }
    }
}

impl Example {
    fn subscription(&self) -> Subscription<Message> {
        let mut subs = Vec::new();

        if self.visible_count < self.messages.len() {
            subs.push(
                iced::time::every(Duration::from_millis(MESSAGE_REVEAL_INTERVAL_MS))
                    .map(|_| Message::Tick),
            );
        }

        subs.push(self.scroll_animator.subscription(|| Message::AnimateScroll));

        if subs.is_empty() {
            Subscription::none()
        } else {
            Subscription::batch(subs)
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Tick => {
                if self.visible_count < self.messages.len() {
                    self.visible_count += 1;
                    if self.is_at_bottom {
                        return self.scroll_animator.request_to_bottom(Id::new(CONTENT_ID));
                    }
                }
                Task::none()
            }
            Message::ScrollToBottom => self.scroll_animator.request_to_bottom(Id::new(CONTENT_ID)),
            Message::AnimateScroll => self.scroll_animator.tick(),
            Message::Scrolled(viewport) => {
                self.is_at_bottom = self.scroll_animator.on_scrolled(viewport, 0.02);
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;

        let list: Element<'_, Message> = if self.visible_count == 0 {
            conversation_empty_state(
                ConversationEmptyStateProps::new("Start a conversation")
                    .description("Messages will appear here as the conversation progresses.")
                    .icon("◌"),
                theme,
            )
        } else {
            let mut items = column![].spacing(theme.spacing.sm);
            for msg in self.messages.iter().take(self.visible_count) {
                items = items.push(message_row(theme, msg));
            }
            items.into()
        };

        let scrollable = conversation_content(
            list,
            ConversationContentProps::new()
                .id(Id::new(CONTENT_ID))
                .scrollbars(ScrollAreaScrollbars::Vertical),
            theme,
        )
        .on_scroll(Message::Scrolled);

        let scroll_button: Element<'_, Message> = if self.is_at_bottom || self.visible_count == 0 {
            container(text("")).width(Length::Shrink).into()
        } else {
            conversation_scroll_button(
                Some(Message::ScrollToBottom),
                ConversationScrollButtonProps::new(),
                theme,
            )
            .into()
        };
        let overlay: Element<'_, Message> =
            conversation_overlay_scroll_button(scrollable, scroll_button);

        let root = conversation(overlay, ConversationProps::new(), theme)
            .height(Length::Fill)
            .width(Length::Fill);

        container(root)
            .padding(theme.spacing.lg)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

fn message_row<'a>(theme: &'a Theme, msg: &'a ChatMessage) -> Element<'a, Message> {
    let bubble = conversation_bubble(
        text(msg.content).size(14),
        ConversationBubbleProps::new(match msg.role {
            Role::User => ConversationBubbleRole::User,
            Role::Assistant => ConversationBubbleRole::Assistant,
        }),
        theme,
    )
    .max_width(620.0);

    let avatar_widget = avatar(
        AvatarProps::new(msg.name)
            .size(AvatarSize::Size4)
            .variant(AvatarVariant::Soft),
        theme,
    );

    match msg.role {
        Role::User => row![
            container(bubble)
                .width(Length::Fill)
                .align_x(iced::alignment::Horizontal::Right),
            avatar_widget
        ]
        .spacing(theme.spacing.sm)
        .align_y(Alignment::Start)
        .width(Length::Fill)
        .into(),
        Role::Assistant => row![avatar_widget, bubble]
            .spacing(theme.spacing.sm)
            .align_y(Alignment::Start)
            .width(Length::Fill)
            .into(),
    }
}

fn scripted_messages() -> Vec<ChatMessage> {
    vec![
        ChatMessage {
            role: Role::User,
            name: "Alex Johnson",
            content: "Hello, how are you?",
        },
        ChatMessage {
            role: Role::Assistant,
            name: "AI Assistant",
            content: "I'm good, thank you! How can I assist you today?",
        },
        ChatMessage {
            role: Role::User,
            name: "Alex Johnson",
            content: "I'm looking for information about your services.",
        },
        ChatMessage {
            role: Role::Assistant,
            name: "AI Assistant",
            content: "Sure! We offer a variety of AI solutions. What are you interested in?",
        },
        ChatMessage {
            role: Role::User,
            name: "Alex Johnson",
            content: "I'm interested in natural language processing tools.",
        },
        ChatMessage {
            role: Role::Assistant,
            name: "AI Assistant",
            content: "Great choice! We have several NLP APIs. Would you like a demo?",
        },
        ChatMessage {
            role: Role::User,
            name: "Alex Johnson",
            content: "Yes, a demo would be helpful.",
        },
        ChatMessage {
            role: Role::Assistant,
            name: "AI Assistant",
            content: "Alright, I can show you a sentiment analysis example. Ready?",
        },
        ChatMessage {
            role: Role::User,
            name: "Alex Johnson",
            content: "Yes, please proceed.",
        },
        ChatMessage {
            role: Role::Assistant,
            name: "AI Assistant",
            content: "Here is a sample: 'I love this product!' -> Positive sentiment.",
        },
        ChatMessage {
            role: Role::User,
            name: "Alex Johnson",
            content: "Impressive! Can it handle multiple languages?",
        },
        ChatMessage {
            role: Role::Assistant,
            name: "AI Assistant",
            content: "Absolutely, our models support over 20 languages.",
        },
        ChatMessage {
            role: Role::User,
            name: "Alex Johnson",
            content: "How do I get started with the API?",
        },
        ChatMessage {
            role: Role::Assistant,
            name: "AI Assistant",
            content: "You can sign up on our website and get an API key instantly.",
        },
        ChatMessage {
            role: Role::User,
            name: "Alex Johnson",
            content: "Is there a free trial available?",
        },
        ChatMessage {
            role: Role::Assistant,
            name: "AI Assistant",
            content: "Yes, we offer a 14-day free trial with full access.",
        },
        ChatMessage {
            role: Role::User,
            name: "Alex Johnson",
            content: "What kind of support do you provide?",
        },
        ChatMessage {
            role: Role::Assistant,
            name: "AI Assistant",
            content: "We provide 24/7 chat and email support for all users.",
        },
        ChatMessage {
            role: Role::User,
            name: "Alex Johnson",
            content: "Thank you for the information!",
        },
        ChatMessage {
            role: Role::Assistant,
            name: "AI Assistant",
            content: "You're welcome! Let me know if you have any more questions.",
        },
    ]
}
