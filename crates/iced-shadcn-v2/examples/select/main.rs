//! Interactive playground for `iced-shadcn-v2::Select` + `shadcn-common`
//! theme knobs.
//!
//! Mirrors the shadcn-svelte select docs demos (basic, groups, scrollable,
//! sizes, multiple, disabled, invalid) plus theme controls like the button
//! example.
//!
//! Run: `cargo run -p iced-shadcn-v2 --example select`

use std::fmt;

use iced::widget::{column, container, pick_list, row, scrollable, text};
use iced::{Alignment, Background, Border, Color, Element, Length, Task};

use iced_shadcn_v2::{
    AccentColor, BaseColor, FontHeading, FontId, FontPack, RadiusId, Select, SelectGroup,
    SelectItem, SelectSelection, SelectSize, SelectType, StyleId, Theme, ThemeMode, fonts,
    iced_font, select,
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
    fruit: Option<&'static str>,
    food: Option<&'static str>,
    size_sm: Option<&'static str>,
    size_default: Option<&'static str>,
    multi: SelectSelection<&'static str>,
    role: Option<&'static str>,
    timezone: Option<&'static str>,
    open_count: u32,
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
    FruitPicked(&'static str),
    FoodChanged(SelectSelection<&'static str>),
    SizeSm(&'static str),
    SizeDefault(&'static str),
    MultiChanged(SelectSelection<&'static str>),
    RolePicked(&'static str),
    TimezonePicked(&'static str),
    Opened,
    Closed,
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::light(),
            fruit: None,
            food: None,
            size_sm: None,
            size_default: None,
            multi: SelectSelection::Multiple(Vec::new()),
            role: None,
            timezone: None,
            open_count: 0,
        }
    }
}

impl Example {
    fn title(&self) -> String {
        "iced-shadcn-v2 Select".to_owned()
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
            Message::FruitPicked(fruit) => {
                self.fruit = Some(fruit);
            }
            Message::FoodChanged(selection) => {
                self.food = selection.as_single().copied();
            }
            Message::SizeSm(value) => {
                self.size_sm = Some(value);
            }
            Message::SizeDefault(value) => {
                self.size_default = Some(value);
            }
            Message::MultiChanged(selection) => {
                self.multi = selection;
            }
            Message::RolePicked(role) => {
                self.role = Some(role);
            }
            Message::TimezonePicked(timezone) => {
                self.timezone = Some(timezone);
            }
            Message::Opened => {
                self.open_count += 1;
            }
            Message::Closed => {}
        }

        Task::none()
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

        let basic = select("Select a fruit", theme)
            .item(("apple", "Apple"))
            .item(("banana", "Banana"))
            .item(("blueberry", "Blueberry"))
            .item(SelectItem::new("grapes", "Grapes").disabled(true))
            .item(("pineapple", "Pineapple"))
            .selected_maybe(self.fruit)
            .on_select(Message::FruitPicked)
            .on_open(Message::Opened)
            .on_close(Message::Closed);

        let groups = Select::new(theme)
            .placeholder("Select a food")
            .group(
                SelectGroup::new("Fruits")
                    .item(("apple", "Apple"))
                    .item(("banana", "Banana"))
                    .item(("blueberry", "Blueberry")),
            )
            .separator()
            .group(
                SelectGroup::new("Vegetables")
                    .item(("carrot", "Carrot"))
                    .item(("broccoli", "Broccoli"))
                    .item(("spinach", "Spinach")),
            )
            .selected_maybe(self.food)
            .on_selection_change(Message::FoodChanged);

        let sizes = column![
            Select::new(theme)
                .placeholder("Small size")
                .items(FRUITS_SMALL)
                .size(SelectSize::Sm)
                .selected_maybe(self.size_sm)
                .on_select(Message::SizeSm),
            Select::new(theme)
                .placeholder("Default size")
                .items(FRUITS_SMALL)
                .selected_maybe(self.size_default)
                .on_select(Message::SizeDefault),
        ]
        .spacing(12);

        let multiple = Select::new(theme)
            .select_type(SelectType::Multiple)
            .placeholder("Select fruits")
            .width(Length::Fixed(288.0))
            .items([
                ("apple", "Apple"),
                ("banana", "Banana"),
                ("blueberry", "Blueberry"),
                ("grapes", "Grapes"),
                ("pineapple", "Pineapple"),
                ("strawberry", "Strawberry"),
                ("watermelon", "Watermelon"),
            ])
            .selection(self.multi.clone())
            .on_selection_change(Message::MultiChanged);

        let disabled = select("Select priority", theme)
            .items([
                ("low", "Low"),
                ("medium", "Medium"),
                ("high", "High"),
                ("critical", "Critical"),
            ])
            .disabled(true)
            .on_select(Message::RolePicked);

        let invalid = select("Select role", theme)
            .items([
                ("admin", "Admin"),
                ("editor", "Editor"),
                ("viewer", "Viewer"),
                ("guest", "Guest"),
            ])
            .selected_maybe(self.role)
            .invalid(true)
            .on_select(Message::RolePicked);

        let scrollable_select = Select::new(theme)
            .placeholder("Select a timezone")
            .width(Length::Fixed(280.0))
            .max_height(300.0)
            .group(
                SelectGroup::new("North America")
                    .item(("est", "Eastern Standard Time (EST)"))
                    .item(("cst", "Central Standard Time (CST)"))
                    .item(("mst", "Mountain Standard Time (MST)"))
                    .item(("pst", "Pacific Standard Time (PST)"))
                    .item(("akst", "Alaska Standard Time (AKST)"))
                    .item(("hst", "Hawaii Standard Time (HST)")),
            )
            .group(
                SelectGroup::new("Europe & Africa")
                    .item(("gmt", "Greenwich Mean Time (GMT)"))
                    .item(("cet", "Central European Time (CET)"))
                    .item(("eet", "Eastern European Time (EET)"))
                    .item(("west", "Western European Summer Time (WEST)"))
                    .item(("cat", "Central Africa Time (CAT)"))
                    .item(("eat", "East Africa Time (EAT)")),
            )
            .group(
                SelectGroup::new("Asia")
                    .item(("msk", "Moscow Time (MSK)"))
                    .item(("ist", "India Standard Time (IST)"))
                    .item(("cst_china", "China Standard Time (CST)"))
                    .item(("jst", "Japan Standard Time (JST)"))
                    .item(("kst", "Korea Standard Time (KST)"))
                    .item(("ist_indonesia", "Indonesia Central Standard Time (WITA)")),
            )
            .group(
                SelectGroup::new("Australia & Pacific")
                    .item(("awst", "Australian Western Standard Time (AWST)"))
                    .item(("acst", "Australian Central Standard Time (ACST)"))
                    .item(("aest", "Australian Eastern Standard Time (AEST)"))
                    .item(("nzst", "New Zealand Standard Time (NZST)"))
                    .item(("fjt", "Fiji Time (FJT)")),
            )
            .group(
                SelectGroup::new("South America")
                    .item(("art", "Argentina Time (ART)"))
                    .item(("bot", "Bolivia Time (BOT)"))
                    .item(("brt", "Brasilia Time (BRT)"))
                    .item(("clt", "Chile Standard Time (CLT)")),
            )
            .selected_maybe(self.timezone)
            .on_select(Message::TimezonePicked);

        let title_px = 32u32;

        let content = column![
            text("iced-shadcn-v2 Select")
                .size(title_px)
                .font(iced_font(theme.font_pack().heading))
                .color(p.foreground),
            text(format!(
                "selected: fruit={:?} · food={:?} · multi={:?} · tz={:?} · opened {} times",
                self.fruit, self.food, self.multi, self.timezone, self.open_count
            ))
            .size(14)
            .font(iced_font(theme.font_pack().sans))
            .color(p.muted_foreground),
            controls,
            section_label("Basic (docs demo)", p.muted_foreground, theme.font_pack()),
            basic,
            section_label(
                "With groups, labels & separator",
                p.muted_foreground,
                theme.font_pack()
            ),
            groups,
            section_label(
                "Scrollable (max-h-[300px] + scroll buttons)",
                p.muted_foreground,
                theme.font_pack()
            ),
            scrollable_select,
            section_label(
                "Sizes (sm · default)",
                p.muted_foreground,
                theme.font_pack()
            ),
            sizes,
            section_label("Multiple selection", p.muted_foreground, theme.font_pack()),
            multiple,
            section_label("Disabled", p.muted_foreground, theme.font_pack()),
            disabled,
            section_label(
                "Invalid (aria-invalid)",
                p.muted_foreground,
                theme.font_pack()
            ),
            invalid,
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
            background: Some(Background::Color(p.background)),
            text_color: Some(p.foreground),
            ..container::Style::default()
        })
        .into()
    }
}

const FRUITS_SMALL: [(&str, &str); 3] = [
    ("apple", "Apple"),
    ("banana", "Banana"),
    ("blueberry", "Blueberry"),
];

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
            .width(72)
            .font(font)
            .color(p.muted_foreground),
        pick_list(options, selected, on_select)
            .text_size(13)
            .font(font)
            .width(Length::Fixed(200.0))
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
