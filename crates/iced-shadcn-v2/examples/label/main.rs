//! Interactive playground for `iced-shadcn-v2::Label`.
//!
//! Mirrors shadcn-svelte label examples: with input, disabled, adjacent to a
//! checkbox stand-in, icons, and style-pack typography (Sera uppercase, etc.).
//!
//! Run: `cargo run -p iced-shadcn-v2 --example label`

use std::fmt;

use iced::widget::{column, container, pick_list, row, scrollable, text, text_input};
use iced::{Alignment, Background, Border, Color, Element, Length, Task};

use iced_shadcn_v2::{
    BaseColor, FontId, Label, LabelContext, StyleId, Theme, ThemeMode, fonts, iced_font,
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
    username: String,
    disabled_value: String,
    terms_accepted: bool,
    focus_log: String,
}

#[derive(Debug, Clone)]
enum Message {
    Style(Labelled<StyleId>),
    Base(Labelled<BaseColor>),
    Mode(Labelled<ThemeMode>),
    Username(String),
    DisabledValue(String),
    FocusUsername,
    ToggleTerms,
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::light(),
            username: String::new(),
            disabled_value: String::new(),
            terms_accepted: false,
            focus_log: "Click a label to emit focus / toggle.".to_owned(),
        }
    }
}

impl Example {
    fn title(&self) -> String {
        "iced-shadcn-v2 Label".to_owned()
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
            Message::Username(value) => {
                self.username = value;
            }
            Message::DisabledValue(value) => {
                self.disabled_value = value;
            }
            Message::FocusUsername => {
                self.focus_log = "Focused username (for_id=\"username\")".to_owned();
            }
            Message::ToggleTerms => {
                self.terms_accepted = !self.terms_accepted;
                self.focus_log = format!(
                    "Terms {}",
                    if self.terms_accepted {
                        "accepted"
                    } else {
                        "cleared"
                    }
                );
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

        let with_input = field_card(
            "With Input",
            column![
                Label::text("Username", theme)
                    .for_id("username")
                    .on_press(Message::FocusUsername),
                fake_input(
                    "username",
                    &self.username,
                    "Username",
                    false,
                    Message::Username,
                    theme,
                ),
            ]
            .spacing(8),
            theme,
        );

        let disabled = field_card(
            "Disabled",
            column![
                Label::text("Disabled", theme)
                    .for_id("disabled")
                    .disabled(true),
                fake_input(
                    "disabled",
                    &self.disabled_value,
                    "Disabled",
                    true,
                    Message::DisabledValue,
                    theme,
                ),
            ]
            .spacing(8),
            theme,
        );

        let with_checkbox = field_card(
            "With Checkbox",
            row![
                fake_checkbox(self.terms_accepted, Message::ToggleTerms, theme),
                Label::text("Accept terms and conditions", theme)
                    .context(LabelContext::AdjacentControl)
                    .for_id("terms")
                    .on_press(Message::ToggleTerms),
            ]
            .spacing(8)
            .align_y(Alignment::Center),
            theme,
        );

        let with_icons = field_card(
            "With icons",
            Label::text("Billing email", theme)
                .icon_start(text("@").size(14).color(p.muted_foreground))
                .icon_end(text("*").size(14).color(p.destructive)),
            theme,
        );

        let focus_log = text(&self.focus_log)
            .size(13)
            .font(iced_font(theme.font_pack().sans))
            .color(p.muted_foreground);

        let content = column![
            text("iced-shadcn-v2 Label")
                .size(32)
                .font(iced_font(theme.font_pack().heading))
                .color(p.foreground),
            text("Theme-aware form labels (shadcn-svelte Label)")
                .size(14)
                .font(iced_font(theme.font_pack().sans))
                .color(p.muted_foreground),
            controls,
            focus_log,
            with_input,
            disabled,
            with_checkbox,
            with_icons,
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

fn fake_input<'a>(
    _id: &'static str,
    value: &str,
    placeholder: &'static str,
    disabled: bool,
    on_input: impl Fn(String) -> Message + 'a,
    theme: &'a Theme,
) -> Element<'a, Message> {
    let p = theme.palette;
    let font = iced_font(theme.font_pack().sans);

    let mut input = text_input(placeholder, value)
        .size(14)
        .font(font)
        .padding(10)
        .width(Length::Fill)
        .style(move |_theme, _status| text_input::Style {
            background: Background::Color(p.background),
            border: Border {
                color: p.input,
                width: 1.0,
                radius: 6.0.into(),
            },
            icon: p.muted_foreground,
            placeholder: p.muted_foreground,
            value: if disabled {
                Color {
                    a: p.foreground.a * 0.5,
                    ..p.foreground
                }
            } else {
                p.foreground
            },
            selection: p.primary,
        });

    if !disabled {
        input = input.on_input(on_input);
    }

    input.into()
}

fn fake_checkbox<'a>(checked: bool, on_toggle: Message, theme: &'a Theme) -> Element<'a, Message> {
    let p = theme.palette;
    let mark = if checked { "✓" } else { " " };

    iced::widget::button(
        container(text(mark).size(12).color(p.primary_foreground))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill),
    )
    .width(Length::Fixed(16.0))
    .height(Length::Fixed(16.0))
    .padding(0)
    .on_press(on_toggle)
    .style(move |_theme, _status| iced::widget::button::Style {
        background: Some(Background::Color(if checked {
            p.primary
        } else {
            p.background
        })),
        text_color: p.primary_foreground,
        border: Border {
            color: if checked { p.primary } else { p.input },
            width: 1.0,
            radius: 4.0.into(),
        },
        shadow: iced::Shadow::default(),
        snap: true,
    })
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
