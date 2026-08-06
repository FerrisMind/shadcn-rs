//! Interactive playground for `iced-shadcn-v2::Input`.
//!
//! Mirrors shadcn-svelte input examples: basic, with label, password
//! (`type="password"` → `.secure`), disabled, invalid (`aria-invalid`), with
//! button, and the extra iced size/radius knobs.
//!
//! Run: `cargo run -p iced-shadcn-v2 --example input`

use std::fmt;

use iced::widget::{column, container, pick_list, row, scrollable, text};
use iced::{Alignment, Background, Border, Color, Element, Length, Task};

use iced_shadcn_v2::{
    BaseColor, Button, ButtonVariant, FontId, Input, InputRadius, InputSize, Label, StyleId, Theme,
    ThemeMode, fonts, iced_font,
};

pub fn main() -> iced::Result {
    let mut app = iced::application(Example::default, Example::update, Example::view)
        .title(Example::title)
        .default_font(iced_font(FontId::Geist));

    for face in fonts::ALL_FACES {
        app = app.font(*face);
    }

    app.run()
}

struct Example {
    theme: Theme,
    basic: String,
    username: String,
    password: String,
    email: String,
    subscribe_email: String,
    sized: String,
    status: String,
}

#[derive(Debug, Clone)]
enum Message {
    Style(Labelled<StyleId>),
    Base(Labelled<BaseColor>),
    Mode(Labelled<ThemeMode>),
    Basic(String),
    Username(String),
    UsernameSubmitted,
    Password(String),
    Email(String),
    SubscribeEmail(String),
    Subscribe,
    Sized(String),
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::light(),
            basic: String::new(),
            username: String::new(),
            password: String::new(),
            email: "not-an-email".to_owned(),
            subscribe_email: String::new(),
            sized: String::new(),
            status: "Type into any field.".to_owned(),
        }
    }
}

impl Example {
    fn title(&self) -> String {
        "iced-shadcn-v2 Input".to_owned()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Style(style) => {
                self.theme = self.theme.clone().with_style(style.0);
            }
            Message::Base(base) => {
                self.theme = self.theme.clone().with_base(base.0);
            }
            Message::Mode(mode) => {
                self.theme = self.theme.clone().with_mode(mode.0);
            }
            Message::Basic(value) => {
                self.basic = value;
            }
            Message::Username(value) => {
                self.username = value;
            }
            Message::UsernameSubmitted => {
                self.status = format!("Submitted username: {:?}", self.username);
            }
            Message::Password(value) => {
                self.password = value;
            }
            Message::Email(value) => {
                self.email = value;
            }
            Message::SubscribeEmail(value) => {
                self.subscribe_email = value;
            }
            Message::Subscribe => {
                self.status = format!("Subscribed {:?}", self.subscribe_email);
            }
            Message::Sized(value) => {
                self.sized = value;
            }
        }

        Task::none()
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
                "Mode",
                &MODES,
                Some(Labelled(theme.mode())),
                Message::Mode,
                theme,
            ),
        ]
        .spacing(8);

        let basic = field_card(
            "Basic",
            Input::new(theme)
                .value(&self.basic)
                .placeholder("Email")
                .on_input(Message::Basic),
            theme,
        );

        let with_label = field_card(
            "With Label (submit with Enter)",
            column![
                Label::text("Username", theme).for_id("username"),
                Input::new(theme)
                    .value(&self.username)
                    .placeholder("Username")
                    .id("username")
                    .on_input(Message::Username)
                    .on_submit(Message::UsernameSubmitted),
            ]
            .spacing(8),
            theme,
        );

        let password = field_card(
            "Password (secure)",
            column![
                Label::text("Password", theme).for_id("password"),
                Input::new(theme)
                    .value(&self.password)
                    .placeholder("Password")
                    .secure(true)
                    .on_input(Message::Password),
            ]
            .spacing(8),
            theme,
        );

        let disabled = field_card(
            "Disabled",
            Input::<Message>::new(theme)
                .value("Read only value")
                .placeholder("Email")
                .disabled(true),
            theme,
        );

        let invalid = field_card(
            "Invalid (aria-invalid)",
            Input::new(theme)
                .value(&self.email)
                .placeholder("Email")
                .invalid(!self.email.contains('@'))
                .on_input(Message::Email),
            theme,
        );

        let with_button = field_card(
            "With Button",
            row![
                Input::new(theme)
                    .value(&self.subscribe_email)
                    .placeholder("Email")
                    .on_input(Message::SubscribeEmail)
                    .on_submit(Message::Subscribe),
                Button::text("Subscribe", theme)
                    .variant(ButtonVariant::Outline)
                    .on_press(Message::Subscribe),
            ]
            .spacing(8)
            .align_y(Alignment::Center),
            theme,
        );

        let sizes = field_card(
            "Sizes & radius (iced extension)",
            column![
                Input::new(theme)
                    .value(&self.sized)
                    .placeholder("Small")
                    .size(InputSize::Sm)
                    .on_input(Message::Sized),
                Input::new(theme)
                    .value(&self.sized)
                    .placeholder("Default")
                    .on_input(Message::Sized),
                Input::new(theme)
                    .value(&self.sized)
                    .placeholder("Large, pill radius")
                    .size(InputSize::Lg)
                    .radius(InputRadius::Full)
                    .on_input(Message::Sized),
            ]
            .spacing(8),
            theme,
        );

        let status = text(&self.status)
            .size(13)
            .font(iced_font(theme.font_pack().sans))
            .color(p.muted_foreground);

        let content = column![
            text("iced-shadcn-v2 Input")
                .size(32)
                .font(iced_font(theme.font_pack().heading))
                .color(p.foreground),
            text("Theme-aware text fields (shadcn-svelte Input)")
                .size(14)
                .font(iced_font(theme.font_pack().sans))
                .color(p.muted_foreground),
            controls,
            status,
            basic,
            with_label,
            password,
            disabled,
            invalid,
            with_button,
            sizes,
        ]
        .spacing(16)
        .max_width(640)
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
        .style(move |_| container::Style {
            background: Some(Background::Color(p.background)),
            text_color: Some(p.foreground),
            ..container::Style::default()
        })
        .into()
    }
}

fn field_card<'a>(
    title: &'static str,
    body: impl Into<Element<'a, Message>>,
    theme: &'a Theme,
) -> Element<'a, Message> {
    let p = &theme.palette;

    column![
        section_label(title, p.muted_foreground, theme),
        container(body.into())
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
            }),
    ]
    .spacing(8)
    .into()
}

fn control_select<'a, T, F>(
    label: &'static str,
    options: &'a [T],
    selected: Option<T>,
    on_select: F,
    theme: &'a Theme,
) -> Element<'a, Message>
where
    T: Clone + PartialEq + fmt::Display + 'a,
    F: Fn(T) -> Message + 'a,
{
    let p = theme.palette;
    let font = iced_font(theme.font_pack().sans);

    row![
        text(label)
            .size(13)
            .width(80)
            .font(font)
            .color(p.muted_foreground),
        pick_list(options, selected, on_select)
            .text_size(13)
            .font(font)
            .width(Length::Fixed(220.0))
            .style(move |_theme, _status| pick_list::Style {
                background: Background::Color(p.background),
                text_color: p.foreground,
                placeholder_color: p.muted_foreground,
                handle_color: p.muted_foreground,
                border: Border {
                    color: p.input,
                    width: 1.0,
                    radius: 6.0.into(),
                },
            }),
    ]
    .spacing(8)
    .align_y(Alignment::Center)
    .into()
}

fn section_label<'a>(label: &'static str, color: Color, theme: &'a Theme) -> Element<'a, Message> {
    text(label)
        .size(18)
        .font(iced_font(theme.font_pack().heading))
        .color(color)
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
