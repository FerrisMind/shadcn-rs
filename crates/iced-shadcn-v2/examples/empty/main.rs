//! Interactive playground for `iced-shadcn-v2::Empty`.
//!
//! Mirrors the shadcn-svelte empty demo and exposes the same theme controls as
//! the button playground. Run with:
//!
//! `cargo run -p iced-shadcn-v2 --example empty`

use std::fmt;

use iced::widget::{column, container, pick_list, row, scrollable, text};
use iced::{Alignment, Background, Border, Color, Element, Length, Task};

use iced_shadcn_v2::{
    AccentColor, BaseColor, Button, ButtonSize, ButtonVariant, Empty, EmptyBorderStyle,
    EmptyContent, EmptyDescription, EmptyHeader, EmptyMedia, EmptyTitle, FontHeading, FontId,
    FontPack, RadiusId, Spinner, SpinnerSize, StyleId, Theme, ThemeMode, fonts, iced_font,
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
    outlined: bool,
    filled: bool,
    show_spinner: bool,
    pressed_count: u32,
}

#[derive(Debug, Clone)]
enum Message {
    Style(Labelled<StyleId>),
    Base(Labelled<BaseColor>),
    Accent(AccentOpt),
    Mode(Labelled<ThemeMode>),
    Font(Labelled<FontId>),
    Heading(Labelled<FontHeading>),
    Radius(Labelled<RadiusId>),
    ToggleOutline,
    ToggleFilled,
    ToggleSpinner,
    Pressed,
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::light().with_style(StyleId::Nova),
            outlined: false,
            filled: false,
            show_spinner: false,
            pressed_count: 0,
        }
    }
}

impl Example {
    fn title(&self) -> String {
        "iced-shadcn-v2 Empty".to_owned()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Style(style) => self.theme = self.theme.clone().with_style(style.0),
            Message::Base(base) => self.theme = self.theme.clone().with_base(base.0),
            Message::Accent(accent) => {
                self.theme = self.theme.clone().with_accent(accent.into_option())
            }
            Message::Mode(mode) => self.theme = self.theme.clone().with_mode(mode.0),
            Message::Font(font) => self.theme = self.theme.clone().with_font(font.0),
            Message::Heading(heading) => {
                self.theme = self.theme.clone().with_font_heading(heading.0)
            }
            Message::Radius(radius) => self.theme = self.theme.clone().with_radius(radius.0),
            Message::ToggleOutline => self.outlined = !self.outlined,
            Message::ToggleFilled => self.filled = !self.filled,
            Message::ToggleSpinner => self.show_spinner = !self.show_spinner,
            Message::Pressed => self.pressed_count += 1,
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let palette = theme.palette;

        let controls = column![
            section_label(
                "Theme (shadcn-common)",
                palette.muted_foreground,
                theme.font_pack()
            ),
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
            control_select(
                "Heading",
                &HEADINGS,
                Some(Labelled(theme.font_heading())),
                Message::Heading,
                theme,
            ),
            control_select(
                "Font",
                &FONTS,
                Some(Labelled(theme.font_id())),
                Message::Font,
                theme,
            ),
            control_select(
                "Radius",
                &RADII,
                Some(Labelled(theme.radius_id())),
                Message::Radius,
                theme,
            ),
            text(format!(
                "root padding={}px · root gap={}px · section max=384px · pressed={}",
                root_padding(theme),
                16,
                self.pressed_count
            ))
            .size(12)
            .font(iced_font(theme.font_pack().mono))
            .color(palette.muted_foreground),
        ]
        .spacing(8);

        let swatches = row![
            swatch("bg", palette.background, palette.border),
            swatch("fg", palette.foreground, palette.border),
            swatch("muted", palette.muted, palette.border),
            swatch("primary", palette.primary, palette.border),
            swatch("card", palette.card, palette.border),
            swatch("border", palette.border, palette.foreground),
        ]
        .spacing(8)
        .wrap();

        let toggles = row![
            Button::text(
                if self.outlined {
                    "Hide dashed outline"
                } else {
                    "Show dashed outline"
                },
                theme,
            )
            .variant(ButtonVariant::Outline)
            .on_press(Message::ToggleOutline),
            Button::text(
                if self.filled {
                    "Transparent background"
                } else {
                    "Card background"
                },
                theme,
            )
            .variant(ButtonVariant::Outline)
            .on_press(Message::ToggleFilled),
            Button::text(
                if self.show_spinner {
                    "Use folder glyph"
                } else {
                    "Use spinner media"
                },
                theme,
            )
            .variant(ButtonVariant::Ghost)
            .on_press(Message::ToggleSpinner),
        ]
        .spacing(8)
        .wrap();

        let preview = empty_preview(theme, self.outlined, self.filled, self.show_spinner);

        let content =
            column![
            text("iced-shadcn-v2 Empty")
                .size(32)
                .font(iced_font(theme.font_pack().heading))
                .color(palette.foreground),
            text("A composable empty state with typed slots and arbitrary iced content.")
                .size(14)
                .font(iced_font(theme.font_pack().sans))
                .color(palette.muted_foreground),
            controls,
            section_label("Palette", palette.muted_foreground, theme.font_pack()),
            swatches,
            section_label("Source-compatible demo", palette.muted_foreground, theme.font_pack()),
            toggles,
            preview,
            section_label("Composition notes", palette.muted_foreground, theme.font_pack()),
            text(
                "Header, media, title, description, and content are independent builders. "
                    .to_owned()
                    + "Each slot also accepts arbitrary iced Elements, while typed text follows "
                    + "the selected shadcn style pack."
            )
            .size(13)
            .font(iced_font(theme.font_pack().sans))
            .color(palette.muted_foreground),
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

fn empty_preview<'a>(
    theme: &'a Theme,
    outlined: bool,
    filled: bool,
    show_spinner: bool,
) -> Element<'a, Message> {
    let media = if show_spinner {
        EmptyMedia::icon(
            Spinner::new(theme).size(SpinnerSize::Sm).animated(true),
            theme,
        )
    } else {
        EmptyMedia::icon(text("□").size(16), theme)
    };

    let actions = row![
        Button::text("Create Project", theme)
            .variant(ButtonVariant::Default)
            .on_press(Message::Pressed),
        Button::text("Import Project", theme)
            .variant(ButtonVariant::Outline)
            .on_press(Message::Pressed),
    ]
    .spacing(8)
    .align_y(Alignment::Center)
    .wrap();

    let mut empty = Empty::new(theme)
        .width(Length::Fill)
        .header(
            EmptyHeader::new(theme)
                .media(media)
                .title(EmptyTitle::text("No Projects Yet", theme))
                .description(EmptyDescription::text(
                    "You haven't created any projects yet. Get started by creating your first project.",
                    theme,
                )),
        )
        .content(EmptyContent::new(theme).push(actions))
        .push(
            Button::text("Learn More ↗", theme)
                .size(ButtonSize::Sm)
                .variant(ButtonVariant::Link)
                .on_press(Message::Pressed),
        );

    if outlined {
        empty = empty.border(EmptyBorderStyle::Dashed);
    }
    if filled {
        empty = empty.background(theme.palette.card);
    }

    empty.into()
}

fn root_padding(theme: &Theme) -> u32 {
    match theme.style_id() {
        StyleId::Nova | StyleId::Lyra | StyleId::Mira => 24,
        _ => 48,
    }
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
            .width(72)
            .font(font)
            .color(palette.muted_foreground),
        pick_list(options, selected, on_select)
            .text_size(13)
            .font(font)
            .width(Length::Fixed(200.0))
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

fn section_label<'a>(label: &'static str, color: Color, pack: FontPack) -> Element<'a, Message> {
    text(label)
        .size(18)
        .font(iced_font(pack.heading))
        .color(color)
        .into()
}

fn swatch<'a>(label: &'static str, fill: Color, border: Color) -> Element<'a, Message> {
    column![
        container(text(""))
            .width(36)
            .height(36)
            .style(move |_| container::Style {
                background: Some(Background::Color(fill)),
                border: Border {
                    color: border,
                    width: 1.0,
                    radius: 6.0.into(),
                },
                ..container::Style::default()
            }),
        text(label).size(10).color(border),
    ]
    .spacing(4)
    .align_x(Alignment::Center)
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

impl fmt::Display for Labelled<FontId> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.0.title())
    }
}

impl fmt::Display for Labelled<FontHeading> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.0.title())
    }
}

impl fmt::Display for Labelled<RadiusId> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.0.label())
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

const ACCENTS: [AccentOpt; 18] = [
    AccentOpt::None,
    AccentOpt::Color(AccentColor::Amber),
    AccentOpt::Color(AccentColor::Blue),
    AccentOpt::Color(AccentColor::Cyan),
    AccentOpt::Color(AccentColor::Emerald),
    AccentOpt::Color(AccentColor::Fuchsia),
    AccentOpt::Color(AccentColor::Green),
    AccentOpt::Color(AccentColor::Indigo),
    AccentOpt::Color(AccentColor::Lime),
    AccentOpt::Color(AccentColor::Orange),
    AccentOpt::Color(AccentColor::Pink),
    AccentOpt::Color(AccentColor::Purple),
    AccentOpt::Color(AccentColor::Red),
    AccentOpt::Color(AccentColor::Rose),
    AccentOpt::Color(AccentColor::Sky),
    AccentOpt::Color(AccentColor::Teal),
    AccentOpt::Color(AccentColor::Violet),
    AccentOpt::Color(AccentColor::Yellow),
];

const MODES: [Labelled<ThemeMode>; 2] = [Labelled(ThemeMode::Light), Labelled(ThemeMode::Dark)];

const FONTS: [Labelled<FontId>; 5] = [
    Labelled(FontId::Geist),
    Labelled(FontId::Inter),
    Labelled(FontId::InstrumentSerif),
    Labelled(FontId::GeistMono),
    Labelled(FontId::JetBrainsMono),
];

const HEADINGS: [Labelled<FontHeading>; 6] = [
    Labelled(FontHeading::Inherit),
    Labelled(FontHeading::Font(FontId::Geist)),
    Labelled(FontHeading::Font(FontId::Inter)),
    Labelled(FontHeading::Font(FontId::InstrumentSerif)),
    Labelled(FontHeading::Font(FontId::GeistMono)),
    Labelled(FontHeading::Font(FontId::JetBrainsMono)),
];

const RADII: [Labelled<RadiusId>; 5] = [
    Labelled(RadiusId::Default),
    Labelled(RadiusId::None),
    Labelled(RadiusId::Small),
    Labelled(RadiusId::Medium),
    Labelled(RadiusId::Large),
];
