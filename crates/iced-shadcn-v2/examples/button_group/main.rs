//! Interactive playground for `iced-shadcn-v2::ButtonGroup`.
//!
//! The layout mirrors shadcn-svelte's button-group demos: a basic outline
//! group, orientation and size variants, separators, a split button, text
//! cells, an input wrapped by buttons, nested groups, and the same group
//! rendered under every style pack.
//!
//! Run with:
//! `cargo run -p iced-shadcn-v2 --example button_group`

use std::fmt;

use iced::widget::{column, container, pick_list, row, scrollable, text};
use iced::{Alignment, Background, Border, Element, Length, Task};

use iced_shadcn_v2::{
    AccentColor, BaseColor, Button, ButtonGroup, ButtonGroupOrientation, ButtonGroupText,
    ButtonSize, ButtonVariant, FontId, Input, StyleId, Theme, ThemeMode, fonts, iced_font,
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
    pack_themes: Vec<Theme>,
    likes: u32,
    url: String,
    last_action: &'static str,
}

#[derive(Debug, Clone)]
enum Message {
    Action(&'static str),
    Like,
    Url(String),
    Style(Labelled<StyleId>),
    Base(Labelled<BaseColor>),
    Accent(AccentOpt),
    Mode(Labelled<ThemeMode>),
}

impl Default for Example {
    fn default() -> Self {
        let theme = Theme::light();
        Self {
            pack_themes: pack_themes(&theme),
            theme,
            likes: 12,
            url: "https://shadcn-svelte.com".to_owned(),
            last_action: "—",
        }
    }
}

impl Example {
    fn title(&self) -> String {
        "iced-shadcn-v2 ButtonGroup".to_owned()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Action(action) => self.last_action = action,
            Message::Like => self.likes += 1,
            Message::Url(url) => self.url = url,
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
        }

        self.pack_themes = pack_themes(&self.theme);

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
        ]
        .spacing(8);

        // shadcn `button-group-demo`: adjacent outline buttons share borders.
        let basic = ButtonGroup::new(theme)
            .aria_label("Mail actions")
            .push(outline("Archive", theme))
            .push(outline("Report", theme))
            .push(outline("Snooze", theme));

        // shadcn `button-group-orientation-demo`: vertical icon stack.
        let vertical = ButtonGroup::new(theme)
            .orientation(ButtonGroupOrientation::Vertical)
            .push(outline_icon("+", "Increase", theme))
            .push(outline_icon("−", "Decrease", theme))
            .push(outline_icon("×", "Close", theme));

        // Vertical group with an explicit width stretches its children.
        let vertical_stretched = ButtonGroup::new(theme)
            .orientation(ButtonGroupOrientation::Vertical)
            .width(Length::Fixed(160.0))
            .push(outline("Top", theme))
            .push(outline("Middle", theme))
            .push(outline("Bottom", theme));

        // shadcn `button-group-size-demo`: sizes come from the buttons.
        let sizes = row![
            captioned("sm", size_group(ButtonSize::Sm, theme), theme),
            captioned("default", size_group(ButtonSize::Default, theme), theme),
            captioned("lg", size_group(ButtonSize::Lg, theme), theme),
        ]
        .spacing(24)
        .align_y(Alignment::Start)
        .wrap();

        // shadcn `button-group-separator-demo`: non-outline variants need an
        // explicit separator between the segments.
        let separated = ButtonGroup::new(theme)
            .push(
                Button::text("Copy", theme)
                    .variant(ButtonVariant::Secondary)
                    .on_press(Message::Action("Copy")),
            )
            .push_separator()
            .push(
                Button::text("Paste", theme)
                    .variant(ButtonVariant::Secondary)
                    .on_press(Message::Action("Paste")),
            )
            .push_separator()
            .push(
                Button::text("Cut", theme)
                    .variant(ButtonVariant::Secondary)
                    .on_press(Message::Action("Cut")),
            );

        // shadcn `button-group-split-demo`: a primary action plus a chevron.
        let split = ButtonGroup::new(theme)
            .push(Button::text("Save changes", theme).on_press(Message::Action("Save changes")))
            .push_separator()
            .push(
                Button::icon(glyph("▾", theme), theme)
                    .size(ButtonSize::Icon)
                    .on_press(Message::Action("Open menu")),
            );

        // shadcn `button-group-with-text` / `button-group-with-like`.
        let with_text = ButtonGroup::new(theme)
            .push(ButtonGroupText::text("Likes", theme))
            .push(
                Button::text(self.likes.to_string(), theme)
                    .variant(ButtonVariant::Outline)
                    .on_press(Message::Like),
            );

        // shadcn `button-group-with-input`: a text cell, an input, a button.
        let with_input = ButtonGroup::new(theme)
            .push(ButtonGroupText::text("https://", theme))
            .push_element(
                Input::new(theme)
                    .value(self.url.as_str())
                    .placeholder("example.com")
                    .width(Length::Fixed(220.0))
                    .on_input(Message::Url),
            )
            .push(outline("Copy", theme).on_press(Message::Action("Copy URL")));

        // shadcn `button-group-nested-demo`: groups of groups gain `gap-2`.
        let nested = ButtonGroup::new(theme)
            .push(
                ButtonGroup::new(theme)
                    .push(outline("1", theme))
                    .push(outline("2", theme))
                    .push(outline("3", theme)),
            )
            .push(
                ButtonGroup::new(theme)
                    .push(outline_icon("«", "Previous", theme))
                    .push(outline_icon("»", "Next", theme)),
            );

        let style_packs = column(
            self.pack_themes
                .iter()
                .zip(STYLES.iter())
                .map(|(pack_theme, style)| {
                    captioned(style.0.as_str(), pack_preview(pack_theme), theme)
                })
                .collect::<Vec<_>>(),
        )
        .spacing(12);

        let content = column![
            text("iced-shadcn-v2 ButtonGroup")
                .size(32)
                .font(iced_font(theme.font_pack().heading))
                .color(palette.foreground),
            text("shadcn-svelte parity: merged borders, separators, text cells, nesting")
                .size(14)
                .font(iced_font(theme.font_pack().sans))
                .color(palette.muted_foreground),
            controls,
            section_label("Basic", theme),
            basic,
            section_label("Orientation", theme),
            row![vertical, vertical_stretched]
                .spacing(24)
                .align_y(Alignment::Start),
            section_label("Sizes", theme),
            sizes,
            section_label("Separator", theme),
            separated,
            section_label("Split", theme),
            split,
            text(format!("Last action: {}", self.last_action))
                .size(12)
                .font(iced_font(theme.font_pack().mono))
                .color(palette.muted_foreground),
            section_label("With text", theme),
            with_text,
            section_label("With input", theme),
            with_input,
            section_label("Nested", theme),
            nested,
            section_label("All style packs", theme),
            style_packs,
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
}

/// Outline button pre-wired to report its own label as the last action.
fn outline<'a>(label: &'static str, theme: &'a Theme) -> Button<'a, Message> {
    Button::text(label, theme)
        .variant(ButtonVariant::Outline)
        .on_press(Message::Action(label))
}

fn outline_icon<'a>(icon: &'a str, action: &'static str, theme: &'a Theme) -> Button<'a, Message> {
    Button::icon(glyph(icon, theme), theme)
        .variant(ButtonVariant::Outline)
        .size(ButtonSize::Icon)
        .on_press(Message::Action(action))
}

fn size_group(size: ButtonSize, theme: &Theme) -> Element<'_, Message> {
    ButtonGroup::new(theme)
        .push(
            Button::text("Left", theme)
                .variant(ButtonVariant::Outline)
                .size(size)
                .on_press(Message::Action("Left")),
        )
        .push(
            Button::text("Center", theme)
                .variant(ButtonVariant::Outline)
                .size(size)
                .on_press(Message::Action("Center")),
        )
        .push(
            Button::text("Right", theme)
                .variant(ButtonVariant::Outline)
                .size(size)
                .on_press(Message::Action("Right")),
        )
        .into()
}

/// The same group rendered under a specific style pack.
fn pack_preview(theme: &Theme) -> Element<'_, Message> {
    ButtonGroup::new(theme)
        .push(ButtonGroupText::text("v2.4.1", theme))
        .push(
            Button::text("Update", theme)
                .variant(ButtonVariant::Outline)
                .on_press(Message::Action("Update")),
        )
        .push(
            Button::text("Rollback", theme)
                .variant(ButtonVariant::Outline)
                .on_press(Message::Action("Rollback")),
        )
        .into()
}

/// One theme per style pack, sharing the base/accent/mode of `theme`.
fn pack_themes(theme: &Theme) -> Vec<Theme> {
    STYLES
        .iter()
        .map(|style| theme.clone().with_style(style.0))
        .collect()
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
    element: Element<'a, Message>,
    theme: &'a Theme,
) -> Element<'a, Message> {
    column![
        element,
        text(caption)
            .size(11)
            .font(iced_font(theme.font_pack().mono))
            .color(theme.palette.muted_foreground),
    ]
    .spacing(6)
    .align_x(Alignment::Start)
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
