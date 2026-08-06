//! Interactive playground for `iced-shadcn-v2::Item` + `shadcn-common` theme knobs.
//!
//! Mirrors the shadcn-svelte item demos: variants, sizes, media slots,
//! header/footer rows, pressable link-like items, and grouped lists.
//!
//! Run: `cargo run -p iced-shadcn-v2 --example item`

use std::fmt;

use iced::widget::{Space, column, container, pick_list, row, scrollable, text};
use iced::{Alignment, Background, Border, Color, Element, Length, Task};

use iced_shadcn_v2::{
    AccentColor, BaseColor, Button, ButtonSize, ButtonVariant, FontHeading, FontId, FontPack, Item,
    ItemActions, ItemContent, ItemDescription, ItemFooter, ItemGroup, ItemHeader, ItemMedia,
    ItemSize, ItemTitle, ItemVariant, RadiusId, StyleId, Theme, ThemeMode, fonts, iced_font,
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
    last_action: &'static str,
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
    Action(&'static str),
    OpenItem,
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::light(),
            last_action: "none",
            open_count: 0,
        }
    }
}

impl Example {
    fn title(&self) -> String {
        "iced-shadcn-v2 Item".to_owned()
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
            Message::Action(label) => {
                self.last_action = label;
            }
            Message::OpenItem => {
                self.open_count += 1;
            }
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

        let variants = column![
            basic_item(theme, ItemVariant::Default, "Default variant"),
            basic_item(theme, ItemVariant::Outline, "Outline variant"),
            basic_item(theme, ItemVariant::Muted, "Muted variant"),
        ]
        .spacing(12);

        let sizes = column![
            sized_item(theme, ItemSize::Default, "Default size"),
            sized_item(theme, ItemSize::Sm, "Small size"),
            sized_item(theme, ItemSize::Xs, "Extra-small size"),
        ]
        .spacing(12);

        let media = column![
            Item::new(theme)
                .variant(ItemVariant::Outline)
                .media(ItemMedia::icon(glyph("♪", 16, p.foreground), theme))
                .content(
                    ItemContent::new(theme)
                        .title(ItemTitle::text("Icon media", theme))
                        .description(ItemDescription::text(
                            "The icon variant leaves the glyph unboxed.",
                            theme,
                        )),
                )
                .actions(chevron_action(theme, "icon media")),
            Item::new(theme)
                .variant(ItemVariant::Outline)
                .media(ItemMedia::image(thumbnail(p.primary), theme))
                .content(
                    ItemContent::new(theme)
                        .title(ItemTitle::text("Image media", theme))
                        .description(ItemDescription::text(
                            "The image variant clips a density-sized square.",
                            theme,
                        )),
                )
                .actions(chevron_action(theme, "image media")),
            Item::new(theme)
                .variant(ItemVariant::Outline)
                .size(ItemSize::Sm)
                .media(ItemMedia::image(thumbnail(p.accent), theme))
                .content(
                    ItemContent::new(theme)
                        .title(ItemTitle::text("Small image", theme))
                        .description(ItemDescription::text("size-8 thumbnail on sm.", theme)),
                )
                .actions(chevron_action(theme, "small image")),
        ]
        .spacing(12);

        let pressable = column![
            Item::new(theme)
                .variant(ItemVariant::Outline)
                .media(ItemMedia::icon(glyph("↗", 16, p.foreground), theme))
                .content(
                    ItemContent::new(theme)
                        .title(ItemTitle::text("Open the documentation", theme))
                        .description(ItemDescription::text(
                            "Pressable item: hover paints the muted surface.",
                            theme,
                        )),
                )
                .actions(ItemActions::new(theme).push(glyph("›", 16, p.muted_foreground)))
                .on_press(Message::OpenItem),
            text(format!("Opened: {}", self.open_count))
                .size(13)
                .font(iced_font(theme.font_pack().sans))
                .color(p.muted_foreground),
        ]
        .spacing(8);

        let header_footer = Item::new(theme)
            .variant(ItemVariant::Outline)
            .header(
                ItemHeader::new(theme)
                    .push(
                        text("Header slot")
                            .size(13)
                            .font(iced_font(theme.font_pack().sans))
                            .color(p.muted_foreground),
                    )
                    .push(
                        text("Top right")
                            .size(13)
                            .font(iced_font(theme.font_pack().sans))
                            .color(p.muted_foreground),
                    ),
            )
            .content(
                ItemContent::new(theme)
                    .title(ItemTitle::text("Header and footer", theme))
                    .description(ItemDescription::text(
                        "Full-width rows above and below the main row.",
                        theme,
                    )),
            )
            .actions(chevron_action(theme, "header/footer"))
            .footer(
                ItemFooter::new(theme)
                    .push(
                        text("Footer slot")
                            .size(13)
                            .font(iced_font(theme.font_pack().sans))
                            .color(p.muted_foreground),
                    )
                    .push(
                        Button::text("Review", theme)
                            .variant(ButtonVariant::Outline)
                            .size(ButtonSize::Sm)
                            .on_press(Message::Action("review")),
                    ),
            );

        let group = ItemGroup::new(theme)
            .push(
                Item::new(theme)
                    .media(ItemMedia::icon(glyph("✓", 16, p.foreground), theme))
                    .content(
                        ItemContent::new(theme)
                            .title(ItemTitle::text("First entry", theme))
                            .description(ItemDescription::text(
                                "Items in a group share a density-aware gap.",
                                theme,
                            )),
                    )
                    .actions(chevron_action(theme, "group first")),
            )
            .separator()
            .push(
                Item::new(theme)
                    .media(ItemMedia::icon(glyph("✗", 16, p.foreground), theme))
                    .content(
                        ItemContent::new(theme)
                            .title(ItemTitle::text("Second entry", theme))
                            .description(ItemDescription::text(
                                "An ItemSeparator sits between the rows.",
                                theme,
                            )),
                    )
                    .actions(chevron_action(theme, "group second")),
            );

        let two_contents = Item::new(theme)
            .variant(ItemVariant::Muted)
            .content(
                ItemContent::new(theme)
                    .title(ItemTitle::text("Flexible content", theme))
                    .description(ItemDescription::text(
                        "The first content slot fills the row.",
                        theme,
                    )),
            )
            .content(
                ItemContent::new(theme).push(
                    text("flex-none")
                        .size(13)
                        .font(iced_font(theme.font_pack().mono))
                        .color(p.muted_foreground),
                ),
            );

        let content = column![
            text("iced-shadcn-v2 Item")
                .size(32)
                .font(iced_font(theme.font_pack().heading))
                .color(p.foreground),
            text(format!("Last action: {}", self.last_action))
                .size(14)
                .font(iced_font(theme.font_pack().sans))
                .color(p.foreground),
            controls,
            section_label("Variants", p.muted_foreground, theme.font_pack()),
            variants,
            section_label("Sizes", p.muted_foreground, theme.font_pack()),
            sizes,
            section_label(
                "Media (icon / image)",
                p.muted_foreground,
                theme.font_pack()
            ),
            media,
            section_label("Pressable item", p.muted_foreground, theme.font_pack()),
            pressable,
            section_label("Header & footer", p.muted_foreground, theme.font_pack()),
            header_footer,
            section_label(
                "Group with separator",
                p.muted_foreground,
                theme.font_pack()
            ),
            group,
            section_label("Second content slot", p.muted_foreground, theme.font_pack()),
            two_contents,
        ]
        .spacing(16)
        .max_width(760)
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

fn basic_item<'a>(
    theme: &'a Theme,
    variant: ItemVariant,
    title: &'static str,
) -> Item<'a, Message> {
    Item::new(theme)
        .variant(variant)
        .content(
            ItemContent::new(theme)
                .title(ItemTitle::text(title, theme))
                .description(ItemDescription::text(
                    "A simple item with title and description.",
                    theme,
                )),
        )
        .actions(
            ItemActions::new(theme).push(
                Button::text("Action", theme)
                    .variant(ButtonVariant::Outline)
                    .size(ButtonSize::Sm)
                    .on_press(Message::Action(title)),
            ),
        )
}

fn sized_item<'a>(theme: &'a Theme, size: ItemSize, title: &'static str) -> Item<'a, Message> {
    Item::new(theme)
        .variant(ItemVariant::Outline)
        .size(size)
        .media(ItemMedia::icon(
            glyph("◆", 16, theme.palette.foreground),
            theme,
        ))
        .content(
            ItemContent::new(theme)
                .title(ItemTitle::text(title, theme))
                .description(ItemDescription::text(
                    "Gap and padding follow the density.",
                    theme,
                )),
        )
        .actions(chevron_action(theme, title))
}

fn chevron_action<'a>(theme: &'a Theme, label: &'static str) -> ItemActions<'a, Message> {
    ItemActions::new(theme).push(
        Button::icon(glyph("›", 16, theme.palette.foreground), theme)
            .variant(ButtonVariant::Ghost)
            .size(ButtonSize::IconSm)
            .on_press(Message::Action(label)),
    )
}

fn glyph<'a>(symbol: &'static str, size: u32, color: Color) -> Element<'a, Message> {
    text(symbol).size(size).color(color).into()
}

fn thumbnail<'a>(fill: Color) -> Element<'a, Message> {
    container(Space::new())
        .width(Length::Fill)
        .height(Length::Fill)
        .style(move |_| container::Style {
            background: Some(Background::Color(fill)),
            ..container::Style::default()
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
