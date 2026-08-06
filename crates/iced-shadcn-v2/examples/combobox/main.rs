//! Interactive playground for the composed `Combobox` component.
//!
//! Run with:
//!
//! ```text
//! cargo run -p iced-shadcn-v2 --example combobox
//! ```

use std::fmt;

use iced::widget::{column, container, pick_list, row, scrollable, text};
use iced::{Alignment, Background, Border, Color, Element, Length, Task};

use iced_shadcn_v2::{
    AccentColor, BaseColor, Button, ButtonVariant, Combobox, ComboboxGroup, ComboboxItem,
    ComboboxSelection, FontHeading, FontId, FontPack, RadiusId, StyleId, Theme, ThemeMode, fonts,
    iced_font,
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
    single_query: String,
    single_selection: ComboboxSelection<&'static str>,
    single_open: bool,
    multiple_query: String,
    multiple_selection: ComboboxSelection<&'static str>,
    multiple_open: bool,
    disabled: bool,
    invalid: bool,
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
    SingleQuery(String),
    SingleSelection(ComboboxSelection<&'static str>),
    SingleOpen(bool),
    MultipleQuery(String),
    MultipleSelection(ComboboxSelection<&'static str>),
    MultipleOpen(bool),
    ToggleDisabled,
    ToggleInvalid,
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::light(),
            single_query: String::new(),
            single_selection: ComboboxSelection::single(None),
            single_open: false,
            multiple_query: String::new(),
            multiple_selection: ComboboxSelection::multiple([]),
            multiple_open: false,
            disabled: false,
            invalid: false,
        }
    }
}

impl Example {
    fn title(&self) -> String {
        "iced-shadcn-v2 Combobox".to_owned()
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
            Message::SingleQuery(query) => self.single_query = query,
            Message::SingleSelection(selection) => {
                self.single_selection = selection;
                // The Svelte reference closes a single combobox from its
                // `onSelect` handler and refocuses the trigger.
                self.single_open = false;
            }
            Message::SingleOpen(open) => self.single_open = open,
            Message::MultipleQuery(query) => self.multiple_query = query,
            Message::MultipleSelection(selection) => self.multiple_selection = selection,
            Message::MultipleOpen(open) => self.multiple_open = open,
            Message::ToggleDisabled => self.disabled = !self.disabled,
            Message::ToggleInvalid => self.invalid = !self.invalid,
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let palette = &theme.palette;

        let controls = column![
            section_label(
                "Theme (shadcn-common)",
                palette.muted_foreground,
                theme.font_pack(),
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

        let state_controls = row![
            Button::text(
                if self.disabled {
                    "Enable trigger"
                } else {
                    "Disable trigger"
                },
                theme,
            )
            .variant(ButtonVariant::Outline)
            .on_press(Message::ToggleDisabled),
            Button::text(
                if self.invalid {
                    "Clear invalid"
                } else {
                    "Mark invalid"
                },
                theme,
            )
            .variant(ButtonVariant::Outline)
            .on_press(Message::ToggleInvalid),
        ]
        .spacing(12)
        .align_y(Alignment::Center);

        let single = Combobox::<&'static str, Message>::new(theme)
            .width(Length::Fixed(200.0))
            .content_width(200.0)
            .placeholder("Select a framework...")
            .search_placeholder("Search framework...")
            .group(
                ComboboxGroup::new("Frameworks")
                    .item(
                        ComboboxItem::new("sveltekit", "SvelteKit")
                            .description("Full-stack Svelte framework"),
                    )
                    .item(("next", "Next.js"))
                    .item(ComboboxItem::new("react", "React").disabled(true))
                    .item(("astro", "Astro")),
            )
            .query(&self.single_query)
            .selection(self.single_selection.clone())
            .open(self.single_open)
            .disabled(self.disabled)
            .invalid(self.invalid)
            .on_query_change(Message::SingleQuery)
            .on_selection_change(Message::SingleSelection)
            .on_open_change(Message::SingleOpen);

        let multiple = Combobox::<&'static str, Message>::new(theme)
            .width(Length::Fixed(256.0))
            .content_width(256.0)
            .placeholder("Select technologies...")
            .search_placeholder("Search technologies...")
            .group(
                ComboboxGroup::new("Frontend")
                    .item(("svelte", "Svelte"))
                    .item(("react", "React"))
                    .item(("vue", "Vue")),
            )
            .separator()
            .group(
                ComboboxGroup::new("Backend")
                    .item(("rust", "Rust"))
                    .item(("go", "Go"))
                    .item(("python", "Python")),
            )
            .query(&self.multiple_query)
            .selection(self.multiple_selection.clone())
            .open(self.multiple_open)
            .on_query_change(Message::MultipleQuery)
            .on_selection_change(Message::MultipleSelection)
            .on_open_change(Message::MultipleOpen);

        let examples = column![
            section_label(
                "Single selection",
                palette.muted_foreground,
                theme.font_pack()
            ),
            text("Search, arrow keys, Enter, disabled rows, and empty state.")
                .size(13)
                .color(palette.muted_foreground),
            single,
            section_label(
                "Multiple selection",
                palette.muted_foreground,
                theme.font_pack()
            ),
            text("Selected values stay checked and the controlled popover remains open.")
                .size(13)
                .color(palette.muted_foreground),
            multiple,
            state_controls,
            text(format!(
                "single={:?} · multiple={:?} · query='{}'",
                self.single_selection, self.multiple_selection, self.single_query
            ))
            .size(12)
            .font(iced_font(theme.font_pack().mono))
            .color(palette.muted_foreground),
        ]
        .spacing(12)
        .width(Length::Fill);

        let content = column![
            text("iced-shadcn-v2 Combobox")
                .size(32)
                .font(iced_font(theme.font_pack().heading))
                .color(palette.foreground),
            text("Button + Popover + Command — style changes flow through all three primitives.")
                .size(14)
                .color(palette.muted_foreground),
            controls,
            examples,
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

fn section_label<'a>(label: &'static str, color: Color, pack: FontPack) -> Element<'a, Message> {
    text(label)
        .size(18)
        .font(iced_font(pack.heading))
        .color(color)
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
