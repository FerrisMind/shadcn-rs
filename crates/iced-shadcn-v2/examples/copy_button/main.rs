//! Interactive playground for `iced-shadcn-v2::CopyButton`.
//!
//! Run with `cargo run -p iced-shadcn-v2 --example copy_button`.
//!
//! The example owns the controlled state, performs the iced clipboard command,
//! confirms success, and resets the transient feedback after 500 ms. A
//! separate action demonstrates the failure state without pretending that the
//! fire-and-forget iced clipboard command returns an error.

use std::time::{Duration, Instant};

use iced::widget::{column, container, row, scrollable, text};
use iced::{Alignment, Background, Color, Element, Length, Subscription, Task};

use iced_shadcn_v2::{
    Button, ButtonSize, ButtonVariant, CopyButton, CopyButtonAction, CopyButtonState,
    CopyButtonStatus, Theme, fonts, iced_font,
};

const COPY_TEXT: &str = "Hello from iced-shadcn-v2";
const COMMAND_TEXT: &str = "jsrepo add ui/copy-button";
const FEEDBACK_DELAY: Duration = Duration::from_millis(500);

pub fn main() -> iced::Result {
    let mut app = iced::application(Example::default, Example::update, Example::view)
        .title(Example::title)
        .subscription(Example::subscription)
        .default_font(iced_font(iced_shadcn_v2::FontId::Geist));

    for face in fonts::ALL_FACES {
        app = app.font(*face);
    }

    app.run()
}

struct Example {
    theme: Theme,
    states: [CopyButtonState; CopyButtonId::COUNT],
    reset_at: [Option<Instant>; CopyButtonId::COUNT],
    variant: ButtonVariant,
    size: ButtonSize,
    with_label: bool,
    press_count: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CopyButtonId {
    Basic,
    Success,
    Failure,
    CustomIcon,
    CustomContent,
}

impl CopyButtonId {
    const COUNT: usize = 5;
    const ALL: [Self; Self::COUNT] = [
        Self::Basic,
        Self::Success,
        Self::Failure,
        Self::CustomIcon,
        Self::CustomContent,
    ];

    const fn index(self) -> usize {
        match self {
            Self::Basic => 0,
            Self::Success => 1,
            Self::Failure => 2,
            Self::CustomIcon => 3,
            Self::CustomContent => 4,
        }
    }

    const fn text(self) -> &'static str {
        match self {
            Self::CustomContent => COMMAND_TEXT,
            Self::Basic | Self::Success | Self::Failure | Self::CustomIcon => COPY_TEXT,
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    Copy(CopyButtonId, CopyButtonAction),
    SimulateFailure,
    Tick(Instant),
    Variant(ButtonVariant),
    Size(ButtonSize),
    ToggleLabel,
}

impl Default for Example {
    fn default() -> Self {
        let mut states = [CopyButtonState::new(); CopyButtonId::COUNT];
        states[CopyButtonId::Success.index()] =
            CopyButtonState::new().with_status(CopyButtonStatus::Success);
        states[CopyButtonId::Failure.index()] =
            CopyButtonState::new().with_status(CopyButtonStatus::Failure);

        Self {
            theme: Theme::light(),
            states,
            reset_at: [None; CopyButtonId::COUNT],
            variant: ButtonVariant::Ghost,
            size: ButtonSize::Icon,
            with_label: false,
            press_count: 0,
        }
    }
}

impl Example {
    fn title(&self) -> String {
        "iced-shadcn-v2 Copy Button".to_owned()
    }

    fn subscription(&self) -> Subscription<Message> {
        if self.reset_at.iter().any(Option::is_some) {
            iced::time::every(Duration::from_millis(16)).map(Message::Tick)
        } else {
            Subscription::none()
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Copy(id, CopyButtonAction::Pressed) => {
                self.apply_action(id, CopyButtonAction::Pressed);
                self.press_count += 1;

                Task::batch([
                    iced::clipboard::write::<Message>(id.text().to_owned()),
                    Task::done(Message::Copy(id, CopyButtonAction::Success)),
                ])
            }
            Message::Copy(id, action) => {
                self.apply_action(id, action);
                Task::none()
            }
            Message::SimulateFailure => {
                self.apply_action(CopyButtonId::Basic, CopyButtonAction::Failure);
                Task::none()
            }
            Message::Tick(now) => {
                for id in CopyButtonId::ALL {
                    let index = id.index();
                    if self.reset_at[index].is_some_and(|deadline| now >= deadline) {
                        self.apply_action(id, CopyButtonAction::Reset);
                    }
                }
                Task::none()
            }
            Message::Variant(variant) => {
                self.variant = variant;
                Task::none()
            }
            Message::Size(size) => {
                self.size = size;
                Task::none()
            }
            Message::ToggleLabel => {
                self.with_label = !self.with_label;
                Task::none()
            }
        }
    }

    fn apply_action(&mut self, id: CopyButtonId, action: CopyButtonAction) {
        let index = id.index();
        let update = iced_shadcn_v2::copy_button_reduce(self.states[index], action);
        self.states[index] = update.state();

        if update.should_reset() {
            self.reset_at[index] = Some(Instant::now() + FEEDBACK_DELAY);
        } else if self.states[index].status().is_idle() {
            self.reset_at[index] = None;
        }
    }

    fn status(&self, id: CopyButtonId) -> CopyButtonStatus {
        self.states[id.index()].status()
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let palette = &theme.palette;

        let mut primary = CopyButton::new(COPY_TEXT, theme)
            .variant(self.variant)
            .size(self.size)
            .status(self.status(CopyButtonId::Basic))
            .on_copy(Message::Copy(
                CopyButtonId::Basic,
                CopyButtonAction::Pressed,
            ));
        if self.with_label {
            primary = primary.label("Copy");
        }

        let showcase = row![
            primary,
            CopyButton::new(COPY_TEXT, theme)
                .label("Copied")
                .status(self.status(CopyButtonId::Success))
                .on_copy(Message::Copy(
                    CopyButtonId::Success,
                    CopyButtonAction::Pressed
                )),
            CopyButton::new(COPY_TEXT, theme)
                .label("Failed")
                .status(self.status(CopyButtonId::Failure))
                .on_copy(Message::Copy(
                    CopyButtonId::Failure,
                    CopyButtonAction::Pressed
                )),
            CopyButton::new(COPY_TEXT, theme)
                .icon(text("⌘").size(16).color(palette.foreground))
                .label("Custom icon")
                .status(self.status(CopyButtonId::CustomIcon))
                .on_copy(Message::Copy(
                    CopyButtonId::CustomIcon,
                    CopyButtonAction::Pressed
                )),
            CopyButton::new(COMMAND_TEXT, theme)
                .size(ButtonSize::Sm)
                .variant(ButtonVariant::Outline)
                .icon(text(">_").size(14).color(palette.foreground))
                .content(
                    text(COMMAND_TEXT)
                        .size(14)
                        .font(iced_font(theme.font_pack().mono))
                        .color(palette.foreground),
                )
                .status(self.status(CopyButtonId::CustomContent))
                .on_copy(Message::Copy(
                    CopyButtonId::CustomContent,
                    CopyButtonAction::Pressed,
                )),
        ]
        .spacing(12)
        .align_y(Alignment::Center)
        .wrap();

        let variants = row![
            variant_button("Ghost", ButtonVariant::Ghost, self.variant, theme),
            variant_button("Outline", ButtonVariant::Outline, self.variant, theme),
            variant_button("Default", ButtonVariant::Default, self.variant, theme),
            variant_button("Secondary", ButtonVariant::Secondary, self.variant, theme),
            variant_button("Soft", ButtonVariant::Soft, self.variant, theme),
            variant_button("Surface", ButtonVariant::Surface, self.variant, theme),
        ]
        .spacing(8)
        .align_y(Alignment::Center)
        .wrap();

        let sizes = row![
            size_button("xs", ButtonSize::IconXs, self.size, theme),
            size_button("sm", ButtonSize::IconSm, self.size, theme),
            size_button("default", ButtonSize::Icon, self.size, theme),
            size_button("lg", ButtonSize::IconLg, self.size, theme),
        ]
        .spacing(8)
        .align_y(Alignment::Center);

        let controls = row![
            Button::text("Toggle label", theme)
                .variant(ButtonVariant::Outline)
                .on_press(Message::ToggleLabel),
            Button::text("Simulate failure", theme)
                .variant(ButtonVariant::Destructive)
                .on_press(Message::SimulateFailure),
        ]
        .spacing(8)
        .align_y(Alignment::Center);

        let status_text = match self.status(CopyButtonId::Basic) {
            CopyButtonStatus::Idle => "idle",
            CopyButtonStatus::Success => "success (resets after 500 ms)",
            CopyButtonStatus::Failure => "failure (resets after 500 ms)",
            _ => "unknown",
        };

        let content = column![
            text("iced-shadcn-v2 Copy Button")
                .size(32)
                .font(iced_font(theme.font_pack().heading))
                .color(palette.foreground),
            text("Copy/Check/X feedback, custom content, and controlled clipboard state")
                .size(14)
                .font(iced_font(theme.font_pack().sans))
                .color(palette.muted_foreground),
            section_label("Interactive", palette.muted_foreground, theme),
            showcase,
            controls,
            text(format!("status: {status_text} · presses: {}", self.press_count))
                .size(13)
                .font(iced_font(theme.font_pack().mono))
                .color(palette.muted_foreground),
            section_label("Variants", palette.muted_foreground, theme),
            variants,
            section_label("Icon sizes", palette.muted_foreground, theme),
            sizes,
            text("The default is Ghost + Icon. Adding a label promotes Icon to Default, just like the Svelte component.")
                .size(13)
                .font(iced_font(theme.font_pack().sans))
                .color(palette.muted_foreground),
        ]
        .spacing(16)
        .max_width(980)
        .padding(8);

        container(
            scrollable(
                container(content)
                    .width(Length::Fill)
                    .center_x(Length::Fill)
                    .padding(24),
            )
            .width(Length::Fill)
            .height(Length::Fill),
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

fn variant_button<'a>(
    label: &'static str,
    variant: ButtonVariant,
    selected: ButtonVariant,
    theme: &'a Theme,
) -> Element<'a, Message> {
    Button::text(label, theme)
        .variant(if variant == selected {
            ButtonVariant::Default
        } else {
            ButtonVariant::Outline
        })
        .on_press(Message::Variant(variant))
        .into()
}

fn size_button<'a>(
    label: &'static str,
    size: ButtonSize,
    selected: ButtonSize,
    theme: &'a Theme,
) -> Element<'a, Message> {
    Button::text(label, theme)
        .variant(if size == selected {
            ButtonVariant::Default
        } else {
            ButtonVariant::Outline
        })
        .on_press(Message::Size(size))
        .into()
}

fn section_label<'a>(label: &'static str, color: Color, theme: &'a Theme) -> Element<'a, Message> {
    text(label)
        .size(18)
        .font(iced_font(theme.font_pack().heading))
        .color(color)
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn feedback_state_is_scoped_to_the_pressed_button() {
        let mut example = Example::default();

        example.apply_action(CopyButtonId::CustomIcon, CopyButtonAction::Success);

        assert_eq!(
            example.status(CopyButtonId::CustomIcon),
            CopyButtonStatus::Success
        );
        assert_eq!(example.status(CopyButtonId::Basic), CopyButtonStatus::Idle);
        assert_eq!(
            example.status(CopyButtonId::CustomContent),
            CopyButtonStatus::Idle
        );

        example.apply_action(CopyButtonId::CustomContent, CopyButtonAction::Failure);

        assert_eq!(
            example.status(CopyButtonId::CustomContent),
            CopyButtonStatus::Failure
        );
        assert_eq!(
            example.status(CopyButtonId::CustomIcon),
            CopyButtonStatus::Success
        );
    }
}
