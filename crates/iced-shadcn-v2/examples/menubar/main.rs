//! Interactive playground for `iced-shadcn-v2::Menubar` + `shadcn-common`
//! theme knobs.
//!
//! Mirrors the shadcn-svelte menubar docs demos (basic File/Edit, checkboxes,
//! radios, shortcuts, destructive, submenu) plus theme controls like the
//! button example.
//!
//! Run: `cargo run -p iced-shadcn-v2 --example menubar`

use std::fmt;

use iced::widget::{column, container, pick_list, row, scrollable, text};
use iced::{Alignment, Background, Border, Color, Element, Length, Task};

use iced_shadcn_v2::{
    AccentColor, BaseColor, FontHeading, FontId, FontPack, MenuItemVariant, Menubar,
    MenubarCheckboxItem, MenubarItem, MenubarMenu, MenubarRadioItem, MenubarSub, RadiusId, StyleId,
    Theme, ThemeMode, fonts, iced_font, menubar,
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
    last_action: String,
    show_bookmarks: bool,
    show_urls: bool,
    appearance: Appearance,
    open_count: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Appearance {
    Light,
    Dark,
    System,
}

impl Appearance {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Light => "light",
            Self::Dark => "dark",
            Self::System => "system",
        }
    }
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
    ToggleBookmarks,
    ToggleUrls,
    Appearance(Appearance),
    Opened,
    Closed,
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::light(),
            last_action: "—".to_owned(),
            show_bookmarks: true,
            show_urls: false,
            appearance: Appearance::Light,
            open_count: 0,
        }
    }
}

impl Example {
    fn title(&self) -> String {
        "iced-shadcn-v2 Menubar".to_owned()
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
            Message::Action(action) => {
                self.last_action = action.to_owned();
            }
            Message::ToggleBookmarks => {
                self.show_bookmarks = !self.show_bookmarks;
                self.last_action = format!("bookmarks={}", self.show_bookmarks);
            }
            Message::ToggleUrls => {
                self.show_urls = !self.show_urls;
                self.last_action = format!("urls={}", self.show_urls);
            }
            Message::Appearance(appearance) => {
                self.appearance = appearance;
                self.last_action = format!("appearance={}", appearance.as_str());
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

        let basic = menubar(theme)
            .menu(
                MenubarMenu::new("File")
                    .item(
                        MenubarItem::new("New Tab")
                            .shortcut("⌘T")
                            .on_select(Message::Action("New Tab")),
                    )
                    .item(
                        MenubarItem::new("New Window")
                            .shortcut("⌘N")
                            .on_select(Message::Action("New Window")),
                    )
                    .item(MenubarItem::new("New Incognito Window").disabled(true))
                    .separator()
                    .item(
                        MenubarItem::new("Print...")
                            .shortcut("⌘P")
                            .on_select(Message::Action("Print")),
                    ),
            )
            .menu(
                MenubarMenu::new("Edit")
                    .item(
                        MenubarItem::new("Undo")
                            .shortcut("⌘Z")
                            .on_select(Message::Action("Undo")),
                    )
                    .item(
                        MenubarItem::new("Redo")
                            .shortcut("⇧⌘Z")
                            .on_select(Message::Action("Redo")),
                    )
                    .separator()
                    .item(MenubarItem::new("Cut").on_select(Message::Action("Cut")))
                    .item(MenubarItem::new("Copy").on_select(Message::Action("Copy")))
                    .item(MenubarItem::new("Paste").on_select(Message::Action("Paste"))),
            )
            .on_open(Message::Opened)
            .on_close(Message::Closed);

        let rich = Menubar::new(theme)
            .menu(
                MenubarMenu::new("View")
                    .checkbox_item(
                        MenubarCheckboxItem::new("Bookmarks Bar", self.show_bookmarks)
                            .on_toggle(Message::ToggleBookmarks),
                    )
                    .checkbox_item(
                        MenubarCheckboxItem::new("Full URLs", self.show_urls)
                            .on_toggle(Message::ToggleUrls),
                    )
                    .separator()
                    .label("Appearance")
                    .radio_item(
                        MenubarRadioItem::new("Light", self.appearance == Appearance::Light)
                            .on_select(Message::Appearance(Appearance::Light)),
                    )
                    .radio_item(
                        MenubarRadioItem::new("Dark", self.appearance == Appearance::Dark)
                            .on_select(Message::Appearance(Appearance::Dark)),
                    )
                    .radio_item(
                        MenubarRadioItem::new("System", self.appearance == Appearance::System)
                            .on_select(Message::Appearance(Appearance::System)),
                    )
                    .separator()
                    .submenu(
                        MenubarSub::new("More Tools")
                            .item(
                                MenubarItem::new("Name Window...")
                                    .on_select(Message::Action("Name Window")),
                            )
                            .item(
                                MenubarItem::new("Developer Tools")
                                    .shortcut("⌥⌘I")
                                    .on_select(Message::Action("Developer Tools")),
                            )
                            .separator()
                            .item(
                                MenubarItem::new("Task Manager")
                                    .shortcut("⇧⌘Escape")
                                    .on_select(Message::Action("Task Manager")),
                            ),
                    )
                    .separator()
                    .item(
                        MenubarItem::new("Reload")
                            .shortcut("⌘R")
                            .on_select(Message::Action("Reload")),
                    )
                    .item(
                        MenubarItem::new("Force Reload")
                            .shortcut("⇧⌘R")
                            .on_select(Message::Action("Force Reload")),
                    )
                    .separator()
                    .item(
                        MenubarItem::new("Toggle Fullscreen")
                            .on_select(Message::Action("Toggle Fullscreen")),
                    )
                    .separator()
                    .item(
                        MenubarItem::new("Hide Others")
                            .shortcut("⌥⌘H")
                            .on_select(Message::Action("Hide Others")),
                    )
                    .item(
                        MenubarItem::new("Hide Sidebar")
                            .variant(MenuItemVariant::Destructive)
                            .shortcut("⌘\\")
                            .on_select(Message::Action("Hide Sidebar")),
                    ),
            )
            .menu(
                MenubarMenu::new("Profiles")
                    .item(MenubarItem::new("Andy").on_select(Message::Action("Andy")))
                    .item(MenubarItem::new("Benoit").on_select(Message::Action("Benoit")))
                    .item(
                        MenubarItem::new("Luis")
                            .inset(true)
                            .on_select(Message::Action("Luis")),
                    )
                    .separator()
                    .item(MenubarItem::new("Edit...").on_select(Message::Action("Edit Profile")))
                    .separator()
                    .item(
                        MenubarItem::new("Add Profile...")
                            .on_select(Message::Action("Add Profile")),
                    ),
            )
            .on_open(Message::Opened)
            .on_close(Message::Closed);

        let title_px = 32u32;

        let content = column![
            text("iced-shadcn-v2 Menubar")
                .size(title_px)
                .font(iced_font(theme.font_pack().heading))
                .color(p.foreground),
            text(format!(
                "last={} · bookmarks={} · urls={} · appearance={} · opened {} times",
                self.last_action,
                self.show_bookmarks,
                self.show_urls,
                self.appearance.as_str(),
                self.open_count
            ))
            .size(14)
            .font(iced_font(theme.font_pack().sans))
            .color(p.muted_foreground),
            controls,
            section_label("Basic (File / Edit)", p.muted_foreground, theme.font_pack()),
            basic,
            section_label(
                "View + Profiles (checkbox / radio / submenu / destructive / inset)",
                p.muted_foreground,
                theme.font_pack(),
            ),
            rich,
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
