//! Interactive playground for `iced-shadcn-v2::Command` + theme knobs.
//!
//! Mirrors the shadcn-svelte command demos (inline + dialog) plus the same
//! theme controls as the button example.
//!
//! Command has pack-specific `.cn-command*` recipes. The Style picker also
//! restyles composed Dialog / Button / Spinner / Separator through the shared
//! [`Theme`] (same composite rule as Form when a host has no pack deltas).
//!
//! Run: `cargo run -p iced-shadcn-v2 --example command`

use std::fmt;

use iced::widget::{column, container, pick_list, row, scrollable, text};
use iced::{Alignment, Background, Color, Element, Length, Task};

use iced_shadcn_v2::{
    AccentColor, BaseColor, Button, ButtonVariant, Card, Command, CommandDialog, CommandGlyph,
    CommandGroup, CommandItem, CommandLoading, FontHeading, FontId, FontPack, RadiusId, StyleId,
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
    inline_query: String,
    dialog_query: String,
    dialog_open: bool,
    highlight: Option<usize>,
    last_action: String,
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
    InlineQuery(String),
    DialogQuery(String),
    Highlight(usize),
    OpenChange(bool),
    Run(&'static str),
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::light(),
            inline_query: String::new(),
            dialog_query: String::new(),
            dialog_open: false,
            highlight: None,
            last_action: "Last action: none".to_owned(),
        }
    }
}

impl Example {
    fn title(&self) -> String {
        "iced-shadcn-v2 Command".to_owned()
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
            Message::Font(font) => {
                self.theme = self.theme.clone().with_font(font.0);
            }
            Message::Heading(heading) => {
                self.theme = self.theme.clone().with_font_heading(heading.0);
            }
            Message::Radius(radius) => {
                self.theme = self.theme.clone().with_radius(radius.0);
            }
            Message::InlineQuery(value) => {
                self.inline_query = value;
                self.highlight = None;
            }
            Message::DialogQuery(value) => {
                self.dialog_query = value;
                self.highlight = None;
            }
            Message::Highlight(index) => {
                self.highlight = Some(index);
            }
            Message::OpenChange(open) => {
                self.dialog_open = open;
                if !open {
                    self.dialog_query.clear();
                }
            }
            Message::Run(action) => {
                self.last_action = format!("Last action: {action}");
                self.dialog_open = false;
            }
        }
        Task::none()
    }

    fn palette_entries(
        include_loading: bool,
    ) -> (
        CommandGroup<&'static str>,
        CommandGroup<&'static str>,
        Option<CommandLoading>,
    ) {
        let suggestions = CommandGroup::new("Suggestions")
            .item(
                CommandItem::new("calendar", "Calendar")
                    .icon(CommandGlyph::Calendar)
                    .keywords(["date", "schedule"]),
            )
            .item(
                CommandItem::new("search-emoji", "Search Emoji")
                    .icon(CommandGlyph::Smile)
                    .keywords(["emoji", "smile"]),
            )
            .item(
                CommandItem::new("calculator", "Calculator")
                    .icon(CommandGlyph::Calculator)
                    .disabled(true),
            );

        let settings = CommandGroup::new("Settings")
            .item(
                CommandItem::new("profile", "Profile")
                    .icon(CommandGlyph::User)
                    .shortcut("⌘P")
                    .keywords(["account", "user"]),
            )
            .item(
                CommandItem::new("billing", "Billing")
                    .icon(CommandGlyph::CreditCard)
                    .shortcut("⌘B")
                    .keywords(["payments", "invoice"]),
            )
            .item(
                CommandItem::new("settings", "Settings")
                    .icon(CommandGlyph::Settings)
                    .shortcut("⌘S")
                    .keywords(["preferences"]),
            );

        let loading =
            include_loading.then(|| CommandLoading::new("Loading more commands...").progress(0.35));

        (suggestions, settings, loading)
    }

    fn inline_command(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let (suggestions, settings, _) = Self::palette_entries(false);

        // Docs demo: `<Card.Root class="w-full p-0"><Card.Content class="p-0">`
        // — Card owns `ring-1` + pack radius; `.cn-command` has no border.
        let mut command = Command::new(theme)
            .query(&self.inline_query)
            .on_query_change(Message::InlineQuery)
            .placeholder("Type a command or search...")
            .empty("No results found.")
            .group(suggestions)
            .separator()
            .group(settings)
            .width(Length::Fill)
            .on_select(Message::Run);

        if let Some(index) = self.highlight {
            command = command.highlighted(index);
        }
        command = command.on_highlight_change(Message::Highlight);

        Card::new(theme)
            .width(Length::Fixed(460.0))
            .spacing(0.0)
            .top_padding(0.0)
            .bottom_padding(0.0)
            .push(command)
            .into()
    }

    fn dialog_command(&self) -> Command<'_, &'static str, Message> {
        let theme = &self.theme;
        let show_loading = self.dialog_query.trim().eq_ignore_ascii_case("loading");
        let (suggestions, settings, loading) = Self::palette_entries(show_loading);

        let mut command = Command::new(theme)
            .query(&self.dialog_query)
            .on_query_change(Message::DialogQuery)
            .placeholder("Type a command or search...")
            .empty("No results found.")
            .group(suggestions)
            .separator()
            .group(settings)
            .width(Length::Fill)
            .on_select(Message::Run)
            .on_highlight_change(Message::Highlight);

        if let Some(loading) = loading {
            command = command.separator_force_mount().loading(loading);
        }
        if let Some(index) = self.highlight {
            command = command.highlighted(index);
        }
        command
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let p = &theme.palette;

        let controls = column![
            section_label(
                "Theme (shadcn-common)",
                p.muted_foreground,
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
        ]
        .spacing(8);

        let dialog = CommandDialog::new(
            Button::text(
                if self.dialog_open {
                    "Close Command Dialog"
                } else {
                    "Open Command Dialog"
                },
                theme,
            )
            .variant(ButtonVariant::Outline)
            .on_press(Message::OpenChange(!self.dialog_open)),
            self.dialog_command(),
            theme,
        )
        .open(self.dialog_open)
        .on_open_change(Message::OpenChange);

        let body = column![
            text("Command")
                .size(22)
                .font(iced_font(theme.font_pack().sans))
                .color(p.foreground),
            text("Inline palette + dialog command menu (shadcn-svelte parity).")
                .size(13)
                .color(p.muted_foreground),
            controls,
            section_label("Inline", p.muted_foreground, theme.font_pack()),
            self.inline_command(),
            section_label("Dialog", p.muted_foreground, theme.font_pack()),
            dialog,
            text(&self.last_action).size(13).color(p.muted_foreground),
            text("Tip: type \"loading\" in the dialog search to show Command.Loading.")
                .size(12)
                .color(p.muted_foreground),
        ]
        .spacing(16)
        .padding(24)
        .width(Length::Fill);

        container(scrollable(body).height(Length::Fill))
            .width(Length::Fill)
            .height(Length::Fill)
            .style(move |_| container::Style {
                background: Some(Background::Color(p.background)),
                ..container::Style::default()
            })
            .into()
    }
}

fn section_label<'a>(label: &'a str, color: Color, fonts: FontPack) -> Element<'a, Message> {
    text(label)
        .size(12)
        .font(iced_font(fonts.mono))
        .color(color)
        .into()
}

fn control_select<'a, T>(
    label: &'a str,
    options: &'a [T],
    selected: Option<T>,
    on_select: impl Fn(T) -> Message + 'a,
    theme: &'a Theme,
) -> Element<'a, Message>
where
    T: Clone + fmt::Display + PartialEq + 'static,
{
    let p = &theme.palette;
    row![
        text(label)
            .size(12)
            .width(Length::Fixed(72.0))
            .color(p.muted_foreground),
        pick_list(options, selected, on_select).width(Length::Fixed(180.0)),
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

impl fmt::Display for Labelled<FontId> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0.title())
    }
}

impl fmt::Display for Labelled<FontHeading> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0.title())
    }
}

impl fmt::Display for Labelled<RadiusId> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0.label())
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => f.write_str("none"),
            Self::Color(color) => f.write_str(color.as_str()),
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

const ACCENTS: [AccentOpt; 5] = [
    AccentOpt::None,
    AccentOpt::Color(AccentColor::Blue),
    AccentOpt::Color(AccentColor::Amber),
    AccentOpt::Color(AccentColor::Emerald),
    AccentOpt::Color(AccentColor::Rose),
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
