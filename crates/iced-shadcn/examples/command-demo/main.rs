use iced::border::Border;
use iced::event::{self, Event};
use iced::keyboard;
use iced::widget::Id;
use iced::widget::{column, container, row, scrollable, text};
use iced::{Alignment, Background, Element, Length, Subscription, Task};

use iced_shadcn::{
    ButtonProps, ButtonVariant, CommandDialogProps, CommandEmptyProps, CommandGroupProps,
    CommandInputProps, CommandItemProps, CommandLinkItemProps, CommandListEntry, CommandListProps,
    CommandLoadingProps, CommandProps, CommandSeparatorProps, DialogProps, KbdProps, Theme, button,
    command, command_dialog, kbd_shortcut,
};
use lucide_icons::{Icon, LUCIDE_FONT_BYTES};

pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view)
        .subscription(Example::subscription)
        .font(LUCIDE_FONT_BYTES)
        .run()
}

#[derive(Debug, Clone)]
enum Message {
    ToggleDialog,
    CloseDialog,
    InlineQueryChanged(String),
    DialogQueryChanged(String),
    RunAction(String),
    Event(Event),
}

struct Example {
    theme: Theme,
    inline_query: String,
    dialog_query: String,
    dialog_open: bool,
    last_action: String,
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::default(),
            inline_query: String::new(),
            dialog_query: String::new(),
            dialog_open: false,
            last_action: "Last action: none".to_string(),
        }
    }
}

impl Example {
    fn subscription(&self) -> Subscription<Message> {
        event::listen().map(Message::Event)
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ToggleDialog => {
                self.dialog_open = !self.dialog_open;
                Task::none()
            }
            Message::CloseDialog => {
                self.dialog_open = false;
                Task::none()
            }
            Message::InlineQueryChanged(value) => {
                self.inline_query = value;
                Task::none()
            }
            Message::DialogQueryChanged(value) => {
                self.dialog_query = value;
                Task::none()
            }
            Message::RunAction(action) => {
                self.last_action = format!("Last action: {action}");
                self.dialog_open = false;
                Task::none()
            }
            Message::Event(event) => {
                if let Event::Keyboard(keyboard::Event::KeyPressed { key, modifiers, .. }) = event
                    && let keyboard::Key::Character(ch) = key
                    && ch.eq_ignore_ascii_case("j")
                    && (modifiers.control() || modifiers.command())
                {
                    self.dialog_open = !self.dialog_open;
                }
                Task::none()
            }
        }
    }

    fn command_entries(&self, include_loading: bool) -> Vec<CommandListEntry<'_, Message>> {
        let mut entries = vec![
            CommandListEntry::Group(
                CommandGroupProps::new(vec![
                    CommandListEntry::Item(
                        CommandItemProps::new("calendar", "Calendar")
                            .icon(icon(Icon::Calendar))
                            .keywords(["date", "schedule"])
                            .on_select(Message::RunAction("Calendar".to_string())),
                    ),
                    CommandListEntry::Item(
                        CommandItemProps::new("search-emoji", "Search Emoji")
                            .icon(icon(Icon::Smile))
                            .keywords(["emoji", "smile"])
                            .on_select(Message::RunAction("Search Emoji".to_string())),
                    ),
                    CommandListEntry::Item(
                        CommandItemProps::new("calculator", "Calculator")
                            .icon(icon(Icon::Calculator))
                            .disabled(true),
                    ),
                ])
                .heading("Suggestions"),
            ),
            CommandListEntry::Separator(CommandSeparatorProps::default()),
            CommandListEntry::Group(
                CommandGroupProps::new(vec![
                    CommandListEntry::Item(
                        CommandItemProps::new("profile", "Profile")
                            .icon(icon(Icon::User))
                            .shortcut("⌘P")
                            .keywords(["account", "user"])
                            .on_select(Message::RunAction("Profile".to_string())),
                    ),
                    CommandListEntry::Item(
                        CommandItemProps::new("billing", "Billing")
                            .icon(icon(Icon::CreditCard))
                            .shortcut("⌘B")
                            .keywords(["payments", "invoice"])
                            .on_select(Message::RunAction("Billing".to_string())),
                    ),
                    CommandListEntry::Item(
                        CommandItemProps::new("settings", "Settings")
                            .icon(icon(Icon::Settings))
                            .shortcut("⌘S")
                            .keywords(["preferences"])
                            .on_select(Message::RunAction("Settings".to_string())),
                    ),
                    CommandListEntry::LinkItem(
                        CommandLinkItemProps::new(
                            "docs",
                            "Command Docs",
                            "https://bits-ui.com/docs/components/command",
                        )
                        .icon(icon(Icon::BookOpen))
                        .keywords(["documentation", "help"])
                        .on_select(Message::RunAction("Open docs".to_string())),
                    ),
                ])
                .heading("Settings"),
            ),
        ];

        if include_loading {
            entries.push(CommandListEntry::Separator(
                CommandSeparatorProps::default().force_mount(true),
            ));
            entries.push(CommandListEntry::Loading(
                CommandLoadingProps::new("Loading more commands...").progress(0.35),
            ));
        }

        entries
    }

    fn inline_command(&self) -> Element<'_, Message> {
        command(
            CommandProps::new(
                Id::new("command-inline"),
                &self.inline_query,
                CommandListProps::new(self.command_entries(false)).max_height(300.0),
            )
            .input(CommandInputProps::new("Type a command or search..."))
            .on_query_change(Message::InlineQueryChanged)
            .empty(CommandEmptyProps::new("No results found."))
            .min_width(460.0),
            &self.theme,
        )
    }

    fn dialog_command_props(&self) -> CommandProps<'_, Message> {
        let show_loading = self.dialog_query.trim().eq_ignore_ascii_case("loading");
        CommandProps::new(
            Id::new("command-dialog"),
            &self.dialog_query,
            CommandListProps::new(self.command_entries(show_loading)).max_height(300.0),
        )
        .input(CommandInputProps::new("Type a command or search..."))
        .on_query_change(Message::DialogQueryChanged)
        .empty(CommandEmptyProps::new("No results found."))
        .min_width(460.0)
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;

        let shortcut_hint = row![
            text("Press")
                .size(13)
                .style(|_t| iced::widget::text::Style {
                    color: Some(theme.palette.muted_foreground)
                }),
            kbd_shortcut(vec!["Ctrl", "J"], KbdProps::new(), theme),
            text("to toggle dialog command palette")
                .size(13)
                .style(|_t| iced::widget::text::Style {
                    color: Some(theme.palette.muted_foreground)
                }),
        ]
        .spacing(8)
        .align_y(Alignment::Center);

        let content = column![
            text("Command Demo (shadcn-svelte parity)")
                .size(18)
                .style(|_t| iced::widget::text::Style {
                    color: Some(theme.palette.foreground)
                }),
            shortcut_hint,
            button(
                if self.dialog_open {
                    "Close Command Dialog"
                } else {
                    "Open Command Dialog"
                },
                Some(Message::ToggleDialog),
                ButtonProps::new().variant(ButtonVariant::Outline),
                theme,
            ),
            container(self.last_action.as_str()).style(|_t| iced::widget::container::Style {
                text_color: Some(theme.palette.muted_foreground),
                ..Default::default()
            }),
            text("Inline Command (reference: command-demo.svelte)")
                .size(14)
                .style(|_t| iced::widget::text::Style {
                    color: Some(theme.palette.foreground)
                }),
            self.inline_command(),
            text("Dialog Command (reference: command-dialog.svelte)")
                .size(14)
                .style(|_t| iced::widget::text::Style {
                    color: Some(theme.palette.foreground)
                }),
            text("Tip: type \"loading\" in dialog input to see Loading state.")
                .size(12)
                .style(|_t| iced::widget::text::Style {
                    color: Some(theme.palette.muted_foreground)
                }),
        ]
        .spacing(12);

        let base = app(theme, preview(theme, scrollable(content)).into());

        if self.dialog_open {
            command_dialog(
                base,
                CommandDialogProps::new(
                    self.dialog_open,
                    Message::CloseDialog,
                    self.dialog_command_props(),
                )
                .dialog_props(
                    DialogProps::new()
                        .padding(0)
                        .overlay_opacity(0.0)
                        .draggable(true),
                )
                .title("Command Palette")
                .description("Search for a command to run..."),
                theme,
            )
        } else {
            base
        }
    }
}

fn icon(icon: Icon) -> String {
    char::from(icon).to_string()
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
        .width(Length::Fill)
        .height(Length::Fill)
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
