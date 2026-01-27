use iced::border::Border;
use iced::widget::{container, row};
use iced::{Background, Element, Length, Task};
use lucide_icons::LUCIDE_FONT_BYTES;

use iced_shadcn::{
    ButtonProps, ButtonVariant, Theme, Toast, ToastVariant, Toaster, button,
};

pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view)
        .font(LUCIDE_FONT_BYTES)
        .run()
}

#[derive(Debug, Clone)]
enum Message {
    Default,
    Success,
    Info,
    Warning,
    Error,
    Promise,
    PromiseResolved(bool),
}

struct Example {
    theme: Theme,
    toaster: Toaster,
    promise: Option<iced_shadcn::ToastPromise>,
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::default(),
            toaster: Toaster::new(),
            promise: None,
        }
    }
}

impl Example {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Default => {
                self.toaster.show(Toast::new("Event has been created"));
            }
            Message::Success => {
                self.toaster
                    .show(Toast::new("Event has been created").with_variant(ToastVariant::Success));
            }
            Message::Info => {
                self.toaster.show(
                    Toast::new("Be at the area 10 minutes before the event time")
                        .with_variant(ToastVariant::Info),
                );
            }
            Message::Warning => {
                self.toaster.show(
                    Toast::new("Event start time cannot be earlier than 8am")
                        .with_variant(ToastVariant::Warning),
                );
            }
            Message::Error => {
                self.toaster.show(
                    Toast::new("Event has not been created").with_variant(ToastVariant::Error),
                );
            }
            Message::Promise => {
                self.promise = Some(self.toaster.promise(Toast::new("Loading...")));
                return Task::perform(
                    async {
                        std::thread::sleep(std::time::Duration::from_millis(500));
                        true
                    },
                    Message::PromiseResolved,
                );
            }
            Message::PromiseResolved(success) => {
                if let Some(promise) = self.promise.take() {
                    if success {
                        promise.success(&self.toaster, Toast::new("Event has been created"));
                    } else {
                        promise.error(&self.toaster, Toast::new("Error"));
                    }
                }
            }
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;

        let content = row![
            button(
                "Default",
                Some(Message::Default),
                ButtonProps::new().variant(ButtonVariant::Outline),
                theme
            ),
            button(
                "Success",
                Some(Message::Success),
                ButtonProps::new().variant(ButtonVariant::Outline),
                theme
            ),
            button(
                "Info",
                Some(Message::Info),
                ButtonProps::new().variant(ButtonVariant::Outline),
                theme
            ),
            button(
                "Warning",
                Some(Message::Warning),
                ButtonProps::new().variant(ButtonVariant::Outline),
                theme
            ),
            button(
                "Error",
                Some(Message::Error),
                ButtonProps::new().variant(ButtonVariant::Outline),
                theme
            ),
            button(
                "Promise",
                Some(Message::Promise),
                ButtonProps::new().variant(ButtonVariant::Outline),
                theme
            ),
        ]
        .spacing(8);

        let base = app(theme, preview(theme, content).into());
        self.toaster.overlay(base, theme)
    }
}

fn app<'a, Message: 'a>(theme: &Theme, content: Element<'a, Message>) -> Element<'a, Message> {
    let background = theme.palette.background;
    container(content)
        .padding(24)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(move |_theme| iced::widget::container::Style {
            background: Some(Background::Color(background)),
            ..iced::widget::container::Style::default()
        })
        .into()
}

fn preview<'a, Message: 'a>(
    theme: &Theme,
    content: impl Into<Element<'a, Message>>,
) -> iced::widget::Container<'a, Message> {
    let background = theme.palette.card;
    let border = theme.palette.border;
    let radius = theme.radius.md;

    container(content)
        .padding(16)
        .width(Length::Shrink)
        .style(move |_theme| iced::widget::container::Style {
            background: Some(Background::Color(background)),
            border: Border {
                radius: radius.into(),
                width: 1.0,
                color: border,
            },
            ..iced::widget::container::Style::default()
        })
}

