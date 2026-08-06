//! Interactive playground for `iced-shadcn-v2::Switch`.
//!
//! The layout mirrors shadcn-svelte's switch demos: a controlled switch with a
//! label, the `sm` / `default` footprints, disabled and invalid states, and the
//! same control rendered under every style pack.
//!
//! Run with:
//! `cargo run -p iced-shadcn-v2 --example switch`

use std::fmt;

use iced::widget::{column, container, pick_list, row, scrollable, text};
use iced::{Alignment, Background, Border, Element, Length, Task};

use iced_shadcn_v2::{
    AccentColor, BaseColor, Button, ButtonVariant, FontId, Label, StyleId, Switch, SwitchSize,
    Theme, ThemeMode, fonts, iced_font,
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
    airplane_mode: bool,
    marketing_emails: bool,
    security_emails: bool,
    size: SizeOpt,
    animated: bool,
    focused: bool,
    invalid: bool,
}

#[derive(Debug, Clone)]
enum Message {
    AirplaneMode(bool),
    MarketingEmails(bool),
    SecurityEmails(bool),
    Size(SizeOpt),
    Style(Labelled<StyleId>),
    Base(Labelled<BaseColor>),
    Accent(AccentOpt),
    Mode(Labelled<ThemeMode>),
    ToggleAnimated,
    ToggleFocused,
    ToggleInvalid,
    Noop,
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::light(),
            airplane_mode: false,
            marketing_emails: true,
            security_emails: true,
            size: SizeOpt::Default,
            animated: true,
            focused: false,
            invalid: false,
        }
    }
}

impl Example {
    fn title(&self) -> String {
        "iced-shadcn-v2 Switch".to_owned()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::AirplaneMode(checked) => self.airplane_mode = checked,
            Message::MarketingEmails(checked) => self.marketing_emails = checked,
            Message::SecurityEmails(checked) => self.security_emails = checked,
            Message::Size(size) => self.size = size,
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
            Message::ToggleAnimated => self.animated = !self.animated,
            Message::ToggleFocused => self.focused = !self.focused,
            Message::ToggleInvalid => self.invalid = !self.invalid,
            Message::Noop => {}
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let palette = theme.palette;

        let controls = column![
            section_label("Theme", theme),
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
            section_label("Switch knobs", theme),
            control_select("Size", &SIZES, Some(self.size), Message::Size, theme),
            row![
                toggle_button("Animation", self.animated, Message::ToggleAnimated, theme),
                toggle_button("Focus ring", self.focused, Message::ToggleFocused, theme),
                toggle_button("Invalid", self.invalid, Message::ToggleInvalid, theme),
            ]
            .spacing(8)
            .wrap(),
        ]
        .spacing(8);

        let airplane = row![
            self.demo_switch(theme, self.airplane_mode)
                .on_toggle(Message::AirplaneMode),
            Label::text("Airplane mode", theme)
                .on_press(Message::AirplaneMode(!self.airplane_mode)),
        ]
        .spacing(8)
        .align_y(Alignment::Center);

        let email_card = container(
            column![
                text("Email notifications")
                    .size(14)
                    .font(iced_font(theme.font_pack().sans))
                    .color(palette.foreground),
                setting_row(
                    "Marketing emails",
                    "Receive emails about new products and features.",
                    self.demo_switch(theme, self.marketing_emails)
                        .on_toggle(Message::MarketingEmails),
                    theme,
                ),
                setting_row(
                    "Security emails",
                    "Receive emails about your account activity.",
                    self.demo_switch(theme, self.security_emails)
                        .on_toggle(Message::SecurityEmails)
                        .disabled(true),
                    theme,
                ),
            ]
            .spacing(14),
        )
        .padding(16)
        .width(Length::Fill)
        .style(move |_| container::Style {
            background: Some(Background::Color(palette.card)),
            border: Border {
                color: palette.border,
                width: 1.0,
                radius: 10.0.into(),
            },
            ..container::Style::default()
        });

        let states = row![
            captioned("off", self.demo_switch(theme, false), theme),
            captioned("on", self.demo_switch(theme, true), theme),
            captioned(
                "disabled off",
                self.demo_switch(theme, false).disabled(true),
                theme,
            ),
            captioned(
                "disabled on",
                self.demo_switch(theme, true).disabled(true),
                theme,
            ),
            captioned(
                "invalid",
                self.demo_switch(theme, false).invalid(true),
                theme,
            ),
            captioned(
                "focused",
                self.demo_switch(theme, true).focused(true),
                theme,
            ),
        ]
        .spacing(24)
        .align_y(Alignment::Center)
        .wrap();

        let sizes = row![
            captioned(
                "sm",
                self.demo_switch(theme, true).size(SwitchSize::Sm),
                theme,
            ),
            captioned(
                "default",
                self.demo_switch(theme, true).size(SwitchSize::Default),
                theme,
            ),
            captioned(
                "custom (28 px)",
                self.demo_switch(theme, true).size(SwitchSize::Custom(28.0)),
                theme,
            ),
        ]
        .spacing(24)
        .align_y(Alignment::Center)
        .wrap();

        let accents = row![
            captioned("theme", self.demo_switch(theme, true), theme),
            captioned(
                "blue",
                self.demo_switch(theme, true).color(AccentColor::Blue),
                theme,
            ),
            captioned(
                "emerald",
                self.demo_switch(theme, true).color(AccentColor::Emerald),
                theme,
            ),
            captioned(
                "rose",
                self.demo_switch(theme, true).color(AccentColor::Rose),
                theme,
            ),
        ]
        .spacing(24)
        .align_y(Alignment::Center)
        .wrap();

        let style_buttons = row![
            style_button(StyleId::Vega, theme),
            style_button(StyleId::Nova, theme),
            style_button(StyleId::Maia, theme),
            style_button(StyleId::Lyra, theme),
            style_button(StyleId::Mira, theme),
            style_button(StyleId::Luma, theme),
            style_button(StyleId::Sera, theme),
            style_button(StyleId::Rhea, theme),
        ]
        .spacing(8)
        .wrap();

        let content = column![
            text("iced-shadcn-v2 Switch")
                .size(32)
                .font(iced_font(theme.font_pack().heading))
                .color(palette.foreground),
            text("shadcn-svelte parity: controlled state, sm/default footprints, disabled")
                .size(14)
                .font(iced_font(theme.font_pack().sans))
                .color(palette.muted_foreground),
            controls,
            section_label("Preview", theme),
            airplane,
            email_card,
            section_label("States", theme),
            states,
            section_label("Sizes", theme),
            sizes,
            section_label("Accents", theme),
            accents,
            section_label("All style packs", theme),
            style_buttons,
        ]
        .spacing(16)
        .max_width(960)
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
            background: Some(Background::Color(palette.background)),
            text_color: Some(palette.foreground),
            ..container::Style::default()
        })
        .into()
    }

    /// Switch pre-wired with the playground knobs; callers add the callback.
    fn demo_switch<'a>(&self, theme: &'a Theme, checked: bool) -> Switch<'a, Message> {
        Switch::new(theme)
            .checked(checked)
            .size(self.size.into())
            .animated(self.animated)
            .focused(self.focused)
            .invalid(self.invalid)
            .on_press(Message::Noop)
    }
}

fn setting_row<'a>(
    title: &'static str,
    description: &'static str,
    switch: Switch<'a, Message>,
    theme: &'a Theme,
) -> Element<'a, Message> {
    row![
        column![
            text(title)
                .size(14)
                .font(iced_font(theme.font_pack().sans))
                .color(theme.palette.foreground),
            text(description)
                .size(13)
                .color(theme.palette.muted_foreground),
        ]
        .spacing(4)
        .width(Length::Fill),
        switch,
    ]
    .spacing(12)
    .align_y(Alignment::Center)
    .into()
}

fn captioned<'a>(
    caption: &'static str,
    switch: Switch<'a, Message>,
    theme: &'a Theme,
) -> Element<'a, Message> {
    column![
        switch,
        text(caption)
            .size(11)
            .font(iced_font(theme.font_pack().mono))
            .color(theme.palette.muted_foreground),
    ]
    .spacing(6)
    .align_x(Alignment::Center)
    .into()
}

fn toggle_button<'a>(
    label: &'static str,
    active: bool,
    message: Message,
    theme: &'a Theme,
) -> Element<'a, Message> {
    let state = if active { "on" } else { "off" };

    Button::text(format!("{label}: {state}"), theme)
        .variant(ButtonVariant::Outline)
        .on_press(message)
        .into()
}

fn style_button(style: StyleId, theme: &Theme) -> Element<'_, Message> {
    Button::text(style.as_str(), theme)
        .variant(ButtonVariant::Outline)
        .on_press(Message::Style(Labelled(style)))
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
    let palette = theme.palette;
    let font = iced_font(theme.font_pack().sans);

    row![
        text(label)
            .size(13)
            .width(80)
            .font(font)
            .color(palette.muted_foreground),
        pick_list(options, selected, on_select)
            .text_size(13)
            .font(font)
            .width(Length::Fixed(220.0))
            .style(move |_theme, _status| pick_list::Style {
                background: Background::Color(palette.background),
                text_color: palette.foreground,
                placeholder_color: palette.muted_foreground,
                handle_color: palette.muted_foreground,
                border: Border {
                    color: palette.input,
                    width: 1.0,
                    radius: 6.0.into(),
                },
            }),
    ]
    .spacing(8)
    .align_y(Alignment::Center)
    .into()
}

fn section_label<'a>(label: &'static str, theme: &'a Theme) -> Element<'a, Message> {
    text(label)
        .size(18)
        .font(iced_font(theme.font_pack().heading))
        .color(theme.palette.muted_foreground)
        .into()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Labelled<T>(T);

impl fmt::Display for Labelled<StyleId> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.0.as_str())
    }
}

impl fmt::Display for Labelled<BaseColor> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.0.as_str())
    }
}

impl fmt::Display for Labelled<ThemeMode> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.0.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum SizeOpt {
    Sm,
    Default,
    Custom,
}

impl From<SizeOpt> for SwitchSize {
    fn from(size: SizeOpt) -> Self {
        match size {
            SizeOpt::Sm => Self::Sm,
            SizeOpt::Default => Self::Default,
            SizeOpt::Custom => Self::Custom(28.0),
        }
    }
}

impl fmt::Display for SizeOpt {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            SizeOpt::Sm => "sm",
            SizeOpt::Default => "default",
            SizeOpt::Custom => "custom (28 px)",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AccentOpt {
    None,
    Color(AccentColor),
}

impl AccentOpt {
    const fn from_option(accent: Option<AccentColor>) -> Self {
        match accent {
            None => Self::None,
            Some(color) => Self::Color(color),
        }
    }

    const fn into_option(self) -> Option<AccentColor> {
        match self {
            Self::None => None,
            Self::Color(color) => Some(color),
        }
    }
}

impl fmt::Display for AccentOpt {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => formatter.write_str("none"),
            Self::Color(color) => formatter.write_str(color.as_str()),
        }
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

const ACCENTS: [AccentOpt; 6] = [
    AccentOpt::None,
    AccentOpt::Color(AccentColor::Blue),
    AccentOpt::Color(AccentColor::Amber),
    AccentOpt::Color(AccentColor::Emerald),
    AccentOpt::Color(AccentColor::Rose),
    AccentOpt::Color(AccentColor::Violet),
];

const MODES: [Labelled<ThemeMode>; 2] = [Labelled(ThemeMode::Light), Labelled(ThemeMode::Dark)];

const SIZES: [SizeOpt; 3] = [SizeOpt::Sm, SizeOpt::Default, SizeOpt::Custom];
