//! Interactive playground for `iced-shadcn-v2` Password + `shadcn-common` state.
//!
//! Password has no pack-specific tokens in shadcn-svelte-extras (only shared
//! Tailwind utilities). Choosing a style pack on the shared [`Theme`] therefore
//! styles every composed part — Input, Toggle, CopyButton — through that pack's
//! own recipes (e.g. Rhea input/toggle/button).
//!
//! Mirrors the extras demos: basic (toggle + strength), toggle only, copy only,
//! both actions, and strength with score label.
//!
//! Run: `cargo run -p iced-shadcn-v2 --example password`

use std::fmt;
use std::time::{Duration, Instant};

use iced::widget::{column, container, pick_list, row, scrollable, text};
use iced::{Alignment, Background, Border, Color, Element, Length, Subscription, Task};

use iced_shadcn_v2::{
    AccentColor, BaseColor, CopyButtonAction, CopyButtonStatus, FontId, Password, PasswordAction,
    PasswordCopy, PasswordInput, PasswordScore, PasswordState, PasswordStrength,
    PasswordToggleVisibility, StyleId, Theme, ThemeMode, Toggle, fonts, iced_font, password_reduce,
};

const FEEDBACK_DELAY: Duration = Duration::from_millis(500);

pub fn main() -> iced::Result {
    let mut app = iced::application(Example::default, Example::update, Example::view)
        .title(Example::title)
        .subscription(Example::subscription)
        .default_font(iced_font(FontId::Geist));

    for face in fonts::ALL_FACES {
        app = app.font(*face);
    }

    app.run()
}

struct Example {
    theme: Theme,
    basic: PasswordState,
    toggle_only: PasswordState,
    copy_only: PasswordState,
    both: PasswordState,
    strength: PasswordState,
    show_visibility: bool,
    show_copy: bool,
    copy_status: CopyButtonStatus,
    copy_reset_at: Option<Instant>,
}

#[derive(Debug, Clone)]
enum Message {
    Style(Labelled<StyleId>),
    Base(Labelled<BaseColor>),
    Accent(AccentOpt),
    Mode(Labelled<ThemeMode>),
    Basic(PasswordAction),
    ToggleOnly(PasswordAction),
    CopyOnly(PasswordAction),
    Both(PasswordAction),
    Strength(PasswordAction),
    ToggleShowVisibility(bool),
    ToggleShowCopy(bool),
    Copy(CopyButtonAction),
    Tick(Instant),
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::light(),
            basic: PasswordState::new()
                .with_toggle_mounted(true)
                .with_strength_mounted(true),
            toggle_only: PasswordState::new()
                .with_toggle_mounted(true)
                .with_value("$ecretpa$$word"),
            copy_only: PasswordState::new()
                .with_copy_mounted(true)
                .with_value("c5xZTsVUs8HoLpBAajKGfbtG8SSbQAC6"),
            both: PasswordState::new()
                .with_toggle_mounted(true)
                .with_copy_mounted(true)
                .with_value(
                    "thisIsASuperLongSecretPasswordThatShouldBeUsedForTestingPurposesAndIsDefinitelyLongerThanMostTypicalPasswords1234567890",
                ),
            strength: PasswordState::new()
                .with_toggle_mounted(true)
                .with_strength_mounted(true)
                .with_min_score(PasswordScore::Two)
                .with_value("$ecretpa$$word"),
            show_visibility: true,
            show_copy: true,
            copy_status: CopyButtonStatus::Idle,
            copy_reset_at: None,
        }
    }
}

impl Example {
    fn title(&self) -> String {
        "iced-shadcn-v2 Password".to_owned()
    }

    fn subscription(&self) -> Subscription<Message> {
        if self.copy_reset_at.is_some() {
            iced::time::every(Duration::from_millis(50)).map(Message::Tick)
        } else {
            Subscription::none()
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Style(style) => {
                self.theme = self.theme.clone().with_style(style.0);
            }
            Message::Base(base) => {
                self.theme = self.theme.clone().with_base(base.0);
            }
            Message::Accent(accent) => {
                self.theme = self.theme.clone().with_accent(accent.into_option());
            }
            Message::Mode(mode) => {
                self.theme = self.theme.clone().with_mode(mode.0);
            }
            Message::Basic(action) => {
                self.basic = password_reduce(self.basic.clone(), action);
            }
            Message::ToggleOnly(action) => {
                self.toggle_only = password_reduce(self.toggle_only.clone(), action);
            }
            Message::CopyOnly(action) => {
                self.copy_only = password_reduce(self.copy_only.clone(), action);
            }
            Message::Both(action) => {
                self.both = password_reduce(self.both.clone(), action);
                self.sync_both_mounts();
            }
            Message::Strength(action) => {
                self.strength = password_reduce(self.strength.clone(), action);
            }
            Message::ToggleShowVisibility(show) => {
                self.show_visibility = show;
                self.sync_both_mounts();
            }
            Message::ToggleShowCopy(show) => {
                self.show_copy = show;
                self.sync_both_mounts();
            }
            Message::Copy(CopyButtonAction::Pressed) => {
                let text = self.both.value().to_owned();
                let fallback = self.copy_only.value().to_owned();
                let payload = if !text.is_empty() { text } else { fallback };
                return iced::clipboard::write(payload)
                    .chain(Task::done(Message::Copy(CopyButtonAction::Success)));
            }
            Message::Copy(CopyButtonAction::Success) => {
                self.copy_status = CopyButtonStatus::Success;
                self.copy_reset_at = Some(Instant::now() + FEEDBACK_DELAY);
            }
            Message::Copy(CopyButtonAction::Failure) => {
                self.copy_status = CopyButtonStatus::Failure;
                self.copy_reset_at = Some(Instant::now() + FEEDBACK_DELAY);
            }
            Message::Copy(CopyButtonAction::Reset) => {
                self.copy_status = CopyButtonStatus::Idle;
                self.copy_reset_at = None;
            }
            Message::Tick(now) => {
                if self.copy_reset_at.is_some_and(|deadline| now >= deadline) {
                    self.copy_status = CopyButtonStatus::Idle;
                    self.copy_reset_at = None;
                }
            }
            Message::Copy(_) => {}
        }

        Task::none()
    }

    fn sync_both_mounts(&mut self) {
        self.both = password_reduce(
            self.both.clone(),
            PasswordAction::MountToggle(self.show_visibility),
        );
        self.both = password_reduce(self.both.clone(), PasswordAction::MountCopy(self.show_copy));
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let p = &theme.palette;

        let controls = column![
            section_label("Theme (shadcn-common)", p.muted_foreground, theme),
            control_select(
                "Style",
                &STYLES,
                Some(Labelled(theme.style_id())),
                Message::Style,
                theme,
            ),
            control_select(
                "Base",
                &BASES,
                Some(Labelled(theme.base())),
                Message::Base,
                theme,
            ),
            control_select(
                "Accent",
                &ACCENTS,
                Some(AccentOpt::from_option(theme.accent())),
                Message::Accent,
                theme,
            ),
            control_select(
                "Mode",
                &MODES,
                Some(Labelled(theme.mode())),
                Message::Mode,
                theme,
            ),
            text(format!(
                "style={} · Password extras geometry shared · Input/Toggle/Copy use this pack",
                theme.style_id().as_str(),
            ))
            .size(12)
            .font(iced_font(theme.font_pack().mono))
            .color(p.muted_foreground),
        ]
        .spacing(8);

        let demos = column![
            demo_card(
                "Basic",
                "Toggle visibility + strength meter (extras default).",
                build_password(PasswordDemo {
                    theme,
                    state: &self.basic,
                    show_toggle: true,
                    show_copy: false,
                    show_strength: true,
                    map: Message::Basic,
                    on_copy: None,
                    copy_status: CopyButtonStatus::Idle,
                }),
                theme,
            ),
            demo_card(
                "Toggle visibility",
                "Show / hide the secret.",
                build_password(PasswordDemo {
                    theme,
                    state: &self.toggle_only,
                    show_toggle: true,
                    show_copy: false,
                    show_strength: false,
                    map: Message::ToggleOnly,
                    on_copy: None,
                    copy_status: CopyButtonStatus::Idle,
                }),
                theme,
            ),
            demo_card(
                "Copy",
                "Copy the secret to the clipboard.",
                build_password(PasswordDemo {
                    theme,
                    state: &self.copy_only,
                    show_toggle: false,
                    show_copy: true,
                    show_strength: false,
                    map: Message::CopyOnly,
                    on_copy: Some(Message::Copy),
                    copy_status: self.copy_status,
                }),
                theme,
            ),
            demo_card(
                "Both",
                "Toggle and copy share the trailing slot.",
                column![
                    build_password(PasswordDemo {
                        theme,
                        state: &self.both,
                        show_toggle: self.show_visibility,
                        show_copy: self.show_copy,
                        show_strength: false,
                        map: Message::Both,
                        on_copy: Some(Message::Copy),
                        copy_status: self.copy_status,
                    }),
                    row![
                        Toggle::text("Show Visibility", theme)
                            .pressed(self.show_visibility)
                            .on_toggle(Message::ToggleShowVisibility),
                        Toggle::text("Show Copy", theme)
                            .pressed(self.show_copy)
                            .on_toggle(Message::ToggleShowCopy),
                    ]
                    .spacing(8)
                    .align_y(Alignment::Center)
                ]
                .spacing(8)
                .into(),
                theme,
            ),
            demo_card(
                "Strength",
                "zxcvbn score with minScore=2; weak values mark the input invalid.",
                column![
                    build_password(PasswordDemo {
                        theme,
                        state: &self.strength,
                        show_toggle: true,
                        show_copy: false,
                        show_strength: true,
                        map: Message::Strength,
                        on_copy: None,
                        copy_status: CopyButtonStatus::Idle,
                    }),
                    text(self.strength.score().label())
                        .size(14)
                        .color(p.muted_foreground),
                ]
                .spacing(4)
                .into(),
                theme,
            ),
        ]
        .spacing(24)
        .width(Length::Fixed(320.0));

        let content = column![
            text("Password")
                .size(28)
                .font(iced_font(theme.font_pack().sans))
                .color(p.foreground),
            text(
                "No Password style variants in the extras registry — pick Style (e.g. Rhea) and composed Input / Toggle / Copy follow that pack."
            )
            .size(14)
            .color(p.muted_foreground),
            controls,
            demos,
        ]
        .spacing(24)
        .padding(24)
        .width(Length::Fill);

        container(scrollable(content).height(Length::Fill))
            .width(Length::Fill)
            .height(Length::Fill)
            .style(move |_| container::Style {
                background: Some(Background::Color(p.background)),
                ..container::Style::default()
            })
            .into()
    }
}

struct PasswordDemo<'a> {
    theme: &'a Theme,
    state: &'a PasswordState,
    show_toggle: bool,
    show_copy: bool,
    show_strength: bool,
    map: fn(PasswordAction) -> Message,
    on_copy: Option<fn(CopyButtonAction) -> Message>,
    copy_status: CopyButtonStatus,
}

fn build_password(demo: PasswordDemo<'_>) -> Element<'_, Message> {
    let PasswordDemo {
        theme,
        state,
        show_toggle,
        show_copy,
        show_strength,
        map,
        on_copy,
        copy_status,
    } = demo;

    let mut input = PasswordInput::new(theme)
        .value(state.value())
        .hidden(state.hidden())
        .invalid(state.is_invalid())
        .placeholder("Password")
        .on_input(move |value| map(PasswordAction::SetValue(value)));

    if show_toggle {
        input = input.toggle(
            PasswordToggleVisibility::new(theme)
                .hidden(state.hidden())
                .on_toggle(move |hidden| map(PasswordAction::SetHidden(hidden))),
        );
    }

    if show_copy {
        let mut copy = PasswordCopy::new(state.value(), theme).status(copy_status);
        if let Some(on_copy) = on_copy {
            copy = copy.on_copy(on_copy);
        }
        input = input.copy(copy);
    }

    let mut root = Password::new(theme).push(input);
    if show_strength {
        root = root.push(PasswordStrength::new(theme).score(state.score()));
    }
    root.into()
}

fn demo_card<'a>(
    title: &'a str,
    description: &'a str,
    body: Element<'a, Message>,
    theme: &'a Theme,
) -> Element<'a, Message> {
    let p = &theme.palette;
    container(
        column![
            text(title).size(16).color(p.foreground),
            text(description).size(13).color(p.muted_foreground),
            body,
        ]
        .spacing(12),
    )
    .padding(16)
    .width(Length::Fill)
    .style(move |_| container::Style {
        background: Some(Background::Color(p.card)),
        border: Border {
            color: p.border,
            width: 1.0,
            radius: 8.0.into(),
        },
        ..container::Style::default()
    })
    .into()
}

fn section_label<'a>(label: &'a str, color: Color, theme: &'a Theme) -> Element<'a, Message> {
    text(label)
        .size(12)
        .font(iced_font(theme.font_pack().mono))
        .color(color)
        .into()
}

fn control_select<'a, T>(
    label: &'a str,
    options: &'static [T],
    selected: Option<T>,
    on_select: fn(T) -> Message,
    theme: &'a Theme,
) -> Element<'a, Message>
where
    T: Clone + PartialEq + fmt::Display + 'static,
{
    let p = &theme.palette;
    row![
        text(label)
            .size(13)
            .width(Length::Fixed(64.0))
            .color(p.muted_foreground),
        pick_list(options, selected, on_select).width(Length::Fixed(160.0)),
    ]
    .spacing(8)
    .align_y(Alignment::Center)
    .into()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Labelled<T>(T);

impl fmt::Display for Labelled<StyleId> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0.as_str())
    }
}

impl fmt::Display for Labelled<BaseColor> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0.as_str())
    }
}

impl fmt::Display for Labelled<ThemeMode> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0.as_str())
    }
}

const STYLES: [Labelled<StyleId>; 8] = [
    Labelled(StyleId::Vega),
    Labelled(StyleId::Nova),
    Labelled(StyleId::Maia),
    Labelled(StyleId::Lyra),
    Labelled(StyleId::Mira),
    Labelled(StyleId::Luma),
    Labelled(StyleId::Sera),
    Labelled(StyleId::Rhea),
];

const BASES: [Labelled<BaseColor>; 7] = [
    Labelled(BaseColor::Neutral),
    Labelled(BaseColor::Zinc),
    Labelled(BaseColor::Stone),
    Labelled(BaseColor::Mauve),
    Labelled(BaseColor::Mist),
    Labelled(BaseColor::Olive),
    Labelled(BaseColor::Taupe),
];

const MODES: [Labelled<ThemeMode>; 2] = [Labelled(ThemeMode::Light), Labelled(ThemeMode::Dark)];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AccentOpt {
    None,
    Blue,
    Amber,
    Emerald,
    Rose,
}

impl AccentOpt {
    fn from_option(accent: Option<AccentColor>) -> Self {
        match accent {
            None => Self::None,
            Some(AccentColor::Blue) => Self::Blue,
            Some(AccentColor::Amber) => Self::Amber,
            Some(AccentColor::Emerald) => Self::Emerald,
            Some(AccentColor::Rose) => Self::Rose,
            Some(_) => Self::None,
        }
    }

    fn into_option(self) -> Option<AccentColor> {
        match self {
            Self::None => None,
            Self::Blue => Some(AccentColor::Blue),
            Self::Amber => Some(AccentColor::Amber),
            Self::Emerald => Some(AccentColor::Emerald),
            Self::Rose => Some(AccentColor::Rose),
        }
    }
}

impl fmt::Display for AccentOpt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::None => "Theme primary",
            Self::Blue => "Blue",
            Self::Amber => "Amber",
            Self::Emerald => "Emerald",
            Self::Rose => "Rose",
        })
    }
}

const ACCENTS: [AccentOpt; 5] = [
    AccentOpt::None,
    AccentOpt::Blue,
    AccentOpt::Amber,
    AccentOpt::Emerald,
    AccentOpt::Rose,
];
