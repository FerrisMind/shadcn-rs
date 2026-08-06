//! Interactive playground for `iced-shadcn-v2::Toggle`.
//!
//! The layout mirrors shadcn-svelte's toggle demos: an icon-style formatting
//! group, text toggles, both variants, all sizes, disabled and invalid states,
//! and the same control rendered under every style pack.
//!
//! Run with:
//! `cargo run -p iced-shadcn-v2 --example toggle`

use std::fmt;

use iced::widget::{column, container, pick_list, row, scrollable, text};
use iced::{Alignment, Background, Border, Element, Length, Task};

use iced_shadcn_v2::{
    AccentColor, BaseColor, Button, ButtonVariant, FontId, StyleId, Theme, ThemeMode, Toggle,
    ToggleSize, ToggleVariant, fonts, iced_font,
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
    bold: bool,
    italic: bool,
    underline: bool,
    subscribed: bool,
    variant: VariantOpt,
    size: SizeOpt,
    invalid: bool,
}

#[derive(Debug, Clone)]
enum Message {
    Bold(bool),
    Italic(bool),
    Underline(bool),
    Subscribed(bool),
    Variant(VariantOpt),
    Size(SizeOpt),
    Style(Labelled<StyleId>),
    Base(Labelled<BaseColor>),
    Accent(AccentOpt),
    Mode(Labelled<ThemeMode>),
    ToggleInvalid,
    Noop,
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::light(),
            bold: true,
            italic: false,
            underline: false,
            subscribed: false,
            variant: VariantOpt::Default,
            size: SizeOpt::Default,
            invalid: false,
        }
    }
}

impl Example {
    fn title(&self) -> String {
        "iced-shadcn-v2 Toggle".to_owned()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Bold(pressed) => self.bold = pressed,
            Message::Italic(pressed) => self.italic = pressed,
            Message::Underline(pressed) => self.underline = pressed,
            Message::Subscribed(pressed) => self.subscribed = pressed,
            Message::Variant(variant) => self.variant = variant,
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
            section_label("Toggle knobs", theme),
            control_select(
                "Variant",
                &VARIANTS,
                Some(self.variant),
                Message::Variant,
                theme,
            ),
            control_select("Size", &SIZES, Some(self.size), Message::Size, theme),
            Button::text(
                if self.invalid {
                    "Invalid: on"
                } else {
                    "Invalid: off"
                },
                theme,
            )
            .variant(ButtonVariant::Outline)
            .on_press(Message::ToggleInvalid),
        ]
        .spacing(8);

        let formatting = row![
            self.demo_toggle_icon("B", self.bold, theme)
                .on_toggle(Message::Bold),
            self.demo_toggle_icon("I", self.italic, theme)
                .on_toggle(Message::Italic),
            self.demo_toggle_icon("U", self.underline, theme)
                .on_toggle(Message::Underline),
        ]
        .spacing(4)
        .align_y(Alignment::Center);

        let subscribe = self
            .demo_toggle_text(
                if self.subscribed {
                    "Subscribed"
                } else {
                    "Subscribe"
                },
                self.subscribed,
                theme,
            )
            .variant(ToggleVariant::Outline)
            .on_toggle(Message::Subscribed);

        // shadcn `toggle-with-text`: icon slot next to a label.
        let icon_with_text = row![
            self.demo_toggle_text("Bold", self.bold, theme)
                .icon_start(glyph("B", theme))
                .on_toggle(Message::Bold),
            self.demo_toggle_text("Italic", self.italic, theme)
                .variant(ToggleVariant::Outline)
                .icon_start(glyph("I", theme))
                .on_toggle(Message::Italic),
            self.demo_toggle_text("Open", self.underline, theme)
                .variant(ToggleVariant::Outline)
                .icon_end(glyph("↗", theme))
                .on_toggle(Message::Underline),
        ]
        .spacing(8)
        .align_y(Alignment::Center)
        .wrap();

        // Icon-only footprints stay square at every size (`min-w-*`).
        let icon_only = row![
            captioned(
                "sm",
                self.demo_toggle_icon("B", true, theme)
                    .size(ToggleSize::Sm)
                    .on_press(Message::Noop),
                theme,
            ),
            captioned(
                "default",
                self.demo_toggle_icon("B", true, theme)
                    .size(ToggleSize::Default)
                    .on_press(Message::Noop),
                theme,
            ),
            captioned(
                "lg",
                self.demo_toggle_icon("B", true, theme)
                    .size(ToggleSize::Lg)
                    .on_press(Message::Noop),
                theme,
            ),
            captioned(
                "outline",
                self.demo_toggle_icon("I", false, theme)
                    .variant(ToggleVariant::Outline)
                    .on_press(Message::Noop),
                theme,
            ),
            captioned(
                "disabled",
                self.demo_toggle_icon("U", true, theme).disabled(true),
                theme,
            ),
        ]
        .spacing(24)
        .align_y(Alignment::Center)
        .wrap();

        let states = row![
            captioned(
                "off",
                self.demo_toggle_text("Bold", false, theme)
                    .on_press(Message::Noop),
                theme,
            ),
            captioned(
                "on",
                self.demo_toggle_text("Bold", true, theme)
                    .on_press(Message::Noop),
                theme,
            ),
            captioned(
                "disabled off",
                self.demo_toggle_text("Bold", false, theme).disabled(true),
                theme,
            ),
            captioned(
                "disabled on",
                self.demo_toggle_text("Bold", true, theme).disabled(true),
                theme,
            ),
            captioned(
                "invalid",
                self.demo_toggle_text("Bold", false, theme)
                    .invalid(true)
                    .on_press(Message::Noop),
                theme,
            ),
        ]
        .spacing(24)
        .align_y(Alignment::Center)
        .wrap();

        let variants = row![
            captioned(
                "default",
                self.demo_toggle_text("Bold", true, theme)
                    .variant(ToggleVariant::Default)
                    .on_press(Message::Noop),
                theme,
            ),
            captioned(
                "outline",
                self.demo_toggle_text("Bold", true, theme)
                    .variant(ToggleVariant::Outline)
                    .on_press(Message::Noop),
                theme,
            ),
        ]
        .spacing(24)
        .align_y(Alignment::Center)
        .wrap();

        let sizes = row![
            captioned(
                "sm",
                self.demo_toggle_text("Bold", true, theme)
                    .size(ToggleSize::Sm)
                    .on_press(Message::Noop),
                theme,
            ),
            captioned(
                "default",
                self.demo_toggle_text("Bold", true, theme)
                    .size(ToggleSize::Default)
                    .on_press(Message::Noop),
                theme,
            ),
            captioned(
                "lg",
                self.demo_toggle_text("Bold", true, theme)
                    .size(ToggleSize::Lg)
                    .on_press(Message::Noop),
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
            text("iced-shadcn-v2 Toggle")
                .size(32)
                .font(iced_font(theme.font_pack().heading))
                .color(palette.foreground),
            text("shadcn-svelte parity: controlled pressed state, default/outline, sm/default/lg")
                .size(14)
                .font(iced_font(theme.font_pack().sans))
                .color(palette.muted_foreground),
            controls,
            section_label("Preview", theme),
            formatting,
            subscribe,
            section_label("Icon only", theme),
            icon_only,
            section_label("Icon with text", theme),
            icon_with_text,
            section_label("States", theme),
            states,
            section_label("Variants", theme),
            variants,
            section_label("Sizes", theme),
            sizes,
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

    /// Text toggle pre-wired with the playground knobs.
    fn demo_toggle_text<'a>(
        &self,
        label: &'a str,
        pressed: bool,
        theme: &'a Theme,
    ) -> Toggle<'a, Message> {
        Toggle::text(label, theme)
            .variant(self.variant.into())
            .size(self.size.into())
            .pressed(pressed)
            .invalid(self.invalid)
    }

    /// Icon-style toggle (single glyph, square footprint).
    fn demo_toggle_icon<'a>(
        &self,
        icon: &'a str,
        pressed: bool,
        theme: &'a Theme,
    ) -> Toggle<'a, Message> {
        Toggle::icon(glyph(icon, theme), theme)
            .variant(self.variant.into())
            .size(self.size.into())
            .pressed(pressed)
            .invalid(self.invalid)
    }
}

/// Stand-in for a Lucide glyph: the examples avoid an icon-font dependency.
fn glyph<'a, Message: 'a>(icon: &'a str, theme: &Theme) -> Element<'a, Message> {
    text(icon)
        .size(14)
        .font(iced_font(theme.font_pack().sans))
        .into()
}

fn captioned<'a>(
    caption: &'static str,
    toggle: Toggle<'a, Message>,
    theme: &'a Theme,
) -> Element<'a, Message> {
    column![
        toggle,
        text(caption)
            .size(11)
            .font(iced_font(theme.font_pack().mono))
            .color(theme.palette.muted_foreground),
    ]
    .spacing(6)
    .align_x(Alignment::Center)
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum VariantOpt {
    Default,
    Outline,
}

impl From<VariantOpt> for ToggleVariant {
    fn from(variant: VariantOpt) -> Self {
        match variant {
            VariantOpt::Default => Self::Default,
            VariantOpt::Outline => Self::Outline,
        }
    }
}

impl fmt::Display for VariantOpt {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            VariantOpt::Default => "default",
            VariantOpt::Outline => "outline",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SizeOpt {
    Sm,
    Default,
    Lg,
}

impl From<SizeOpt> for ToggleSize {
    fn from(size: SizeOpt) -> Self {
        match size {
            SizeOpt::Sm => Self::Sm,
            SizeOpt::Default => Self::Default,
            SizeOpt::Lg => Self::Lg,
        }
    }
}

impl fmt::Display for SizeOpt {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            SizeOpt::Sm => "sm",
            SizeOpt::Default => "default",
            SizeOpt::Lg => "lg",
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

const VARIANTS: [VariantOpt; 2] = [VariantOpt::Default, VariantOpt::Outline];

const SIZES: [SizeOpt; 3] = [SizeOpt::Sm, SizeOpt::Default, SizeOpt::Lg];
