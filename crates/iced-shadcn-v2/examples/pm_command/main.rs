//! Interactive playground for `iced-shadcn-v2::PmCommand`.
//!
//! Run with `cargo run -p iced-shadcn-v2 --example pm_command`.
//! The example owns clipboard feedback just like the button and copy-button
//! examples: the component emits an action, the application performs the
//! clipboard command, then feeds the controlled status back into the view.

use std::time::{Duration, Instant};

use iced::widget::{column, container, row, scrollable, text};
use iced::{Background, Element, Length, Subscription, Task};

use iced_shadcn_v2::{
    Button, ButtonSize, ButtonVariant, CopyButtonAction, CopyButtonState, CopyButtonStatus,
    PmCommand, PmCommandAgent, PmCommandVariant, PmCommandVerb, StyleId, Theme, ThemeMode, fonts,
    iced_font, resolve_pm_command,
};

const FEEDBACK_DELAY: Duration = Duration::from_millis(500);
const COMMAND_ARGS: [&str; 3] = ["jsrepo", "add", "ui/pm-command"];

pub fn main() -> iced::Result {
    let mut app = iced::application(Example::default, Example::update, Example::view)
        .title(Example::title)
        .subscription(Example::subscription)
        .window_size(iced::Size::new(1180.0, 760.0))
        .default_font(iced_font(iced_shadcn_v2::FontId::Geist));

    for face in fonts::ALL_FACES {
        app = app.font(*face);
    }

    app.run()
}

#[derive(Debug, Clone)]
enum Message {
    Agent(PmCommandAgent),
    Copy(CopyButtonAction),
    Style(StyleId),
    Mode(ThemeMode),
    Variant(PmCommandVariant),
    Tick(Instant),
}

struct Example {
    theme: Theme,
    agent: PmCommandAgent,
    variant: PmCommandVariant,
    state: CopyButtonState,
    reset_at: Option<Instant>,
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::light(),
            agent: PmCommandAgent::Npm,
            variant: PmCommandVariant::Default,
            state: CopyButtonState::new(),
            reset_at: None,
        }
    }
}

impl Example {
    fn title(&self) -> String {
        "iced-shadcn-v2 PMCommand".to_owned()
    }

    fn subscription(&self) -> Subscription<Message> {
        if self.reset_at.is_some() {
            iced::time::every(Duration::from_millis(16)).map(Message::Tick)
        } else {
            Subscription::none()
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Agent(agent) => {
                self.agent = agent;
                Task::none()
            }
            Message::Copy(CopyButtonAction::Pressed) => {
                self.apply_copy(CopyButtonAction::Pressed);
                let command = self.command_text();
                Task::batch([
                    iced::clipboard::write::<Message>(command),
                    Task::done(Message::Copy(CopyButtonAction::Success)),
                ])
            }
            Message::Copy(action) => {
                self.apply_copy(action);
                Task::none()
            }
            Message::Style(style) => {
                self.theme = self.theme.clone().with_style(style);
                Task::none()
            }
            Message::Mode(mode) => {
                self.theme = self.theme.clone().with_mode(mode);
                Task::none()
            }
            Message::Variant(variant) => {
                self.variant = variant;
                Task::none()
            }
            Message::Tick(now) => {
                if self.reset_at.is_some_and(|deadline| now >= deadline) {
                    self.apply_copy(CopyButtonAction::Reset);
                }
                Task::none()
            }
        }
    }

    fn apply_copy(&mut self, action: CopyButtonAction) {
        let update = iced_shadcn_v2::copy_button_reduce(self.state, action);
        self.state = update.state();
        if update.should_reset() {
            self.reset_at = Some(Instant::now() + FEEDBACK_DELAY);
        } else if self.state.status().is_idle() {
            self.reset_at = None;
        }
    }

    fn command_text(&self) -> String {
        let args = COMMAND_ARGS
            .iter()
            .map(|arg| (*arg).to_owned())
            .collect::<Vec<_>>();
        resolve_pm_command(&self.agent, &PmCommandVerb::Execute, &args).command_text()
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let palette = &theme.palette;

        let style_controls = row(StyleId::ALL.into_iter().map(|style| {
            let variant = if style == theme.style_id() {
                ButtonVariant::Default
            } else {
                ButtonVariant::Ghost
            };
            Button::text(style.as_str(), theme)
                .variant(variant)
                .size(ButtonSize::Sm)
                .on_press(Message::Style(style))
                .into()
        }))
        .spacing(4)
        .wrap();

        let mode_controls = row![
            Button::text("Light", theme)
                .variant(if theme.mode() == ThemeMode::Light {
                    ButtonVariant::Default
                } else {
                    ButtonVariant::Ghost
                })
                .size(ButtonSize::Sm)
                .on_press(Message::Mode(ThemeMode::Light)),
            Button::text("Dark", theme)
                .variant(if theme.mode() == ThemeMode::Dark {
                    ButtonVariant::Default
                } else {
                    ButtonVariant::Ghost
                })
                .size(ButtonSize::Sm)
                .on_press(Message::Mode(ThemeMode::Dark)),
        ]
        .spacing(4);

        let variant_controls = row![
            Button::text("Default", theme)
                .variant(if self.variant == PmCommandVariant::Default {
                    ButtonVariant::Default
                } else {
                    ButtonVariant::Ghost
                })
                .size(ButtonSize::Sm)
                .on_press(Message::Variant(PmCommandVariant::Default)),
            Button::text("Secondary", theme)
                .variant(if self.variant == PmCommandVariant::Secondary {
                    ButtonVariant::Default
                } else {
                    ButtonVariant::Ghost
                })
                .size(ButtonSize::Sm)
                .on_press(Message::Variant(PmCommandVariant::Secondary)),
        ]
        .spacing(4);

        let main_command = PmCommand::new(PmCommandVerb::Execute, COMMAND_ARGS, theme)
            .agent(self.agent.clone())
            .variant(self.variant)
            .copy_status(self.state.status())
            .on_agent_change(Message::Agent)
            .on_copy_action(Message::Copy)
            .max_width(600.0);

        let custom_command = PmCommand::new(PmCommandVerb::Add, ["bits-ui", "-D"], theme)
            .agents([
                PmCommandAgent::Pnpm,
                PmCommandAgent::Npm,
                PmCommandAgent::Bun,
            ])
            .agent(PmCommandAgent::Pnpm)
            .on_agent_change(Message::Agent)
            .on_copy_action(Message::Copy)
            .max_width(600.0);

        let overflow_command = PmCommand::new(
            PmCommandVerb::Execute,
            [
                "jsrepo",
                "build",
                "--preview",
                "--include-blocks",
                "pm-command",
                "button",
                "copy-button",
                "utils",
            ],
            theme,
        )
        .agent(self.agent.clone())
        .copy_status(CopyButtonStatus::Idle)
        .on_copy(Message::Copy(CopyButtonAction::Pressed))
        .max_width(600.0);

        let controls = container(
            column![
                text("PMCommand playground")
                    .size(22)
                    .color(palette.foreground),
                text("Style pack"),
                style_controls,
                text("Mode"),
                mode_controls,
                text("Root variant"),
                variant_controls,
                text("Active agent: ").color(palette.muted_foreground),
                text(self.agent.as_str().to_owned())
                    .font(iced_shadcn_v2::iced_font(theme.style.font_pack.mono,)),
            ]
            .spacing(10)
            .padding(20),
        )
        .width(Length::Fixed(330.0));

        let demos = column![
            text("Execute command").size(16),
            main_command,
            text("Add command").size(16),
            custom_command,
            text("Long command / hidden horizontal scrollbar").size(16),
            overflow_command,
        ]
        .spacing(12)
        .padding(24)
        .width(Length::Fill);

        container(
            row![
                controls,
                scrollable(demos).width(Length::Fill).height(Length::Fill),
            ]
            // Keep the same 16px page gutter as the other v2 examples. It
            // also prevents a narrow window from centering the row outside
            // the viewport and hiding the controls completely.
            .padding(16),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .style(move |_| iced::widget::container::Style {
            background: Some(Background::Color(palette.background)),
            text_color: Some(palette.foreground),
            ..iced::widget::container::Style::default()
        })
        .into()
    }
}
